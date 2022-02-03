use crate::bindgen::{
    FPDF_BITMAP, FPDF_BOOL, FPDF_DOCUMENT, FPDF_DWORD, FPDF_FORMFILLINFO, FPDF_FORMHANDLE,
    FPDF_PAGE, FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use js_sys::{Array, Function, Object, Reflect, Uint8Array};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::{c_void, CString};
use std::mem::size_of;
use std::os::raw::{c_float, c_int, c_uchar, c_ulong};
use std::sync::{Mutex, MutexGuard};
use wasm_bindgen::intern;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref PDFIUM_RENDER_WASM_STATE: Mutex<PdfiumRenderWasmState> =
        Mutex::new(PdfiumRenderWasmState::default());
}

#[derive(Debug, Copy, Clone)]
enum JsFunctionArgumentType {
    Void,
    Number,
    String,
    Pointer,
}

/// A global singleton used to store bindings to an external Pdfium WASM module.
///
/// This singleton is constructed when the WASM module containing pdfium-render is first loaded
/// into the browser. Binding to an external Pdfium WASM module is established by calling
/// the exported [initialize_pdfium_render()] function from Javascript. It is essential that this
/// function is called _before_ initializing `pdfium-render` from within Rust code. For an example, see:
/// <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
#[derive(Debug)]
pub struct PdfiumRenderWasmState {
    pdfium_wasm_module: Option<Object>,
    malloc_js_fn: Option<Function>,
    free_js_fn: Option<Function>,
    call_js_fn: Option<Function>,
    debug: bool,
    state: HashMap<String, JsValue>,
}

impl PdfiumRenderWasmState {
    const BYTES_PER_PIXEL: i32 = 4;

    /// Returns exclusive access to the global [PdfiumRenderWasmState] singleton.
    #[inline]
    pub fn lock() -> MutexGuard<'static, PdfiumRenderWasmState> {
        match PDFIUM_RENDER_WASM_STATE.try_lock() {
            Ok(mutex) => mutex,
            Err(err) => {
                log::error!(
                    "PdfiumRenderWasmState::lock(): unable to acquire singleton lock: {:#?}",
                    err
                );
                log::error!("This may indicate a programming error in pdfium-render. Please file an issue: https://github.com/ajrcarey/pdfium-render/issues");

                panic!()
            }
        }
    }

    /// Returns `true` if this [PdfiumRenderWasmState] has been successfully bound to a valid
    /// external Pdfium WASM module.
    pub fn is_ready(&self) -> bool {
        self.pdfium_wasm_module.is_some()
    }

    /// Configures this [PdfiumRenderWasmState] with bindings to emscripten-exposed Pdfium functions
    /// in the given Javascript Object.
    fn bind_to_pdfium(&mut self, module: Object, debug: bool) -> Result<(), &str> {
        self.malloc_js_fn = Some(Function::from(
            Reflect::get(&module, &JsValue::from("_malloc"))
                .map_err(|_| "Module._malloc() not defined")?,
        ));

        self.free_js_fn = Some(Function::from(
            Reflect::get(&module, &JsValue::from("_free"))
                .map_err(|_| "Module._free() not defined")?,
        ));

        self.call_js_fn = Some(Function::from(
            Reflect::get(&module, &JsValue::from("ccall"))
                .map_err(|_| "Module.ccall() not defined")?,
        ));

        // We don't define a fixed binding to it, but check now that the Module.HEAPU8 accessor works.

        if Reflect::get(&module, &JsValue::from("HEAPU8")).is_err() {
            return Err("Module.HEAPU8[] not defined");
        }

        self.pdfium_wasm_module = Some(module);
        self.debug = debug;

        Ok(())
    }

    /// Allocates the given number of bytes in Pdfium's WASM memory heap, returning a pointer
    /// to the allocation address.
    fn malloc(&self, len: usize) -> usize {
        match self
            .malloc_js_fn
            .as_ref()
            .unwrap()
            .call1(&JsValue::null(), &JsValue::from_f64(len as f64))
        {
            Ok(result) => match result.as_f64() {
                Some(result) => result as usize,
                None => {
                    log::error!("pdfium-render::PdfiumRenderWasmState::malloc(): return value from Module._malloc() is not a Javascript number");

                    panic!();
                }
            },
            Err(err) => {
                log::error!(
                    "pdfium-render::PdfiumRenderWasmState::malloc(): call to Module._malloc() failed for allocation length {}: {:#?}",
                    len,
                    err
                );

                panic!();
            }
        }
    }

    /// Frees a previously-made memory allocation in Pdfium's WASM memory heap.
    fn free(&self, ptr: usize) {
        let result = self
            .free_js_fn
            .as_ref()
            .unwrap()
            .call1(&JsValue::null(), &JsValue::from_f64(ptr as f64));

        if let Some(err) = result.err() {
            log::error!(
                "pdfium-render::PdfiumRenderWasmState::free(): call to Module._free() failed: {:#?}",
                err
            );

            panic!()
        }
    }

    /// Calls an emscripten-wrapped Pdfium function.
    fn call(
        &self,
        fn_name: &str,
        return_type: JsFunctionArgumentType,
        arg_types: Option<Vec<JsFunctionArgumentType>>,
        args: Option<&JsValue>,
    ) -> JsValue {
        fn js_value_from_argument_type(arg_type: JsFunctionArgumentType) -> JsValue {
            match arg_type {
                JsFunctionArgumentType::Void => JsValue::undefined(),
                JsFunctionArgumentType::Number | JsFunctionArgumentType::Pointer => {
                    JsValue::from(intern("number"))
                }
                JsFunctionArgumentType::String => JsValue::from(intern("string")),
            }
        }

        log::debug!(
            "pdfium-render::PdfiumRenderWasmState::call(): performing call to function: {:#?}",
            fn_name
        );

        let js_fn_name = JsValue::from(intern(fn_name));

        let js_return_type = js_value_from_argument_type(return_type);

        let js_arg_types = match arg_types {
            Some(arg_types) => {
                if self.debug {
                    // Type-check arguments.

                    if let Some(args) = args {
                        if !Array::is_array(args) {
                            log::warn!("pdfium-render::PdfiumRenderWasmState::call(): argument type list given, but arguments is not an Array");
                        }

                        let args = Array::from(args);

                        if arg_types.len() != args.length() as usize {
                            log::warn!("pdfium-render::PdfiumRenderWasmState::call(): length of argument type and argument lists does not match");
                        }

                        for (index, (arg_type, arg)) in arg_types
                            .iter()
                            .zip(args.iter().collect::<Vec<_>>())
                            .enumerate()
                        {
                            if !match arg_type {
                                JsFunctionArgumentType::Void => arg.is_undefined(),
                                JsFunctionArgumentType::Number
                                | JsFunctionArgumentType::Pointer => arg.as_f64().is_some(),
                                JsFunctionArgumentType::String => arg.as_string().is_some(),
                            } {
                                log::warn!("pdfium-render::PdfiumRenderWasmState::call(): type-checking of argument {} failed: expected {:#?}, received {:#?}",
                            index,
                            arg_type,
                            arg);
                            }
                        }
                    }
                }

                JsValue::from(
                    arg_types
                        .into_iter()
                        .map(|arg_type| js_value_from_argument_type(arg_type))
                        .collect::<Array>(),
                )
            }
            None => JsValue::undefined(),
        };

        log::debug!(
            "pdfium-render::PdfiumRenderWasmState::call(): arguments: {:#?}",
            args
        );

        match self.call_js_fn.as_ref().unwrap().apply(
            &JsValue::null(),
            &Array::of4(
                &js_fn_name,
                &js_return_type,
                &js_arg_types,
                args.unwrap_or(&JsValue::undefined()),
            ),
        ) {
            Ok(result) => {
                log::debug!(
                    "pdfium-render::PdfiumRenderWasmState::call(): call returned result: {:#?}",
                    result
                );

                if self.debug {
                    // Type-check result.

                    if !match return_type {
                        JsFunctionArgumentType::Void => result.is_undefined(),
                        JsFunctionArgumentType::Number | JsFunctionArgumentType::Pointer => {
                            result.as_f64().is_some()
                        }
                        JsFunctionArgumentType::String => result.as_string().is_some(),
                    } {
                        log::warn!("pdfium-render::PdfiumRenderWasmState::call(): result data type does not match expected return type {:#?}", return_type);
                    }
                }

                result
            }
            Err(err) => {
                log::error!(
                    "pdfium-render::PdfiumRenderWasmState::call(): call to {:#?} failed: {:#?}",
                    fn_name,
                    err,
                );

                panic!();
            }
        }
    }

    /// Returns a live view of Pdfium's WASM memory heap.
    fn heap_u8(&self) -> Uint8Array {
        match Reflect::get(
            self.pdfium_wasm_module.as_ref().unwrap(),
            &JsValue::from("HEAPU8"),
        ) {
            Ok(result) => Uint8Array::from(result),
            Err(err) => {
                log::error!(
                    "pdfium-render::PdfiumRenderWasmState::heap_u8(): Module.HEAPU8[] not defined: {:#?}",
                    err
                );

                panic!();
            }
        }
    }

    /// Copies the given bytes into Pdfium's WASM memory heap, returning a pointer to the
    /// destination location.
    ///
    /// WASM modules are isolated from one another and cannot directly share memory. We must
    /// therefore copy buffers from our own memory heap across into Pdfium's memory heap, and vice versa.
    fn copy_bytes_to_pdfium(&self, bytes: &[u8]) -> usize {
        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_to_pdfium(): entering");

        let remote_ptr = self.malloc(bytes.len());

        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_to_pdfium(): got pointer offset back from Module._malloc(): {}", remote_ptr);

        self.heap_u8()
            .set(unsafe { &Uint8Array::view(bytes) }, remote_ptr as u32);

        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_to_pdfium(): copied {} bytes into allocated buffer", bytes.len());

        if self.debug {
            // Compare memory after copying to ensure it is the same.

            let mut differences_count = 0;

            let pdfium_heap = self.heap_u8();

            for (index, byte) in bytes.iter().enumerate() {
                let dest = pdfium_heap.get_index((remote_ptr + index) as u32);

                if *byte != dest {
                    differences_count += 1;

                    log::warn!(
                        "pdfium-render::PdfiumRenderWasmState::copy_bytes_to_pdfium(): byte index {} differs between source and destination: source data = {}, destination data = {}",
                        index,
                        byte,
                        dest,
                    );
                }
            }

            log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_to_pdfium(): {} bytes differ between source and destination byte buffers", differences_count);
        }

        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_to_pdfium(): leaving");

        remote_ptr
    }

    /// Copies the raw bytes at the given pointer into Pdfium's WASM memory heap, returning a
    /// pointer to the copied struct at the destination location.
    ///
    /// WASM modules are isolated from one another and cannot directly share memory. We must
    /// therefore copy buffers from our own memory heap across into Pdfium's memory heap, and vice versa.
    #[inline]
    fn copy_struct_to_pdfium<T>(&self, ptr: *const T) -> usize {
        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_struct_to_pdfium(): entering");

        let len = size_of::<T>();

        log::debug!(
            "pdfium-render::PdfiumRenderWasmState::copy_struct_to_pdfium(): struct is at heap offset {}, data length {} bytes",
            ptr as usize as u32,
            len
        );

        self.copy_bytes_to_pdfium(
            unsafe { Uint8Array::view_mut_raw(ptr as *mut u8, len) }
                .to_vec()
                .as_slice(),
        )
    }

    /// Copies the raw bytes at the given pointer into Pdfium's WASM memory heap, returning a
    /// pointer to the copied struct at the destination location.
    ///
    /// WASM modules are isolated from one another and cannot directly share memory. We must
    /// therefore copy buffers from our own memory heap across into Pdfium's memory heap, and vice versa.
    #[inline]
    fn copy_struct_to_pdfium_mut<T>(&self, ptr: *mut T) -> usize {
        self.copy_struct_to_pdfium(ptr as *const T)
    }

    /// Copies bytes from the given pointer address in Pdfium's memory heap into our memory
    /// heap, returning an address to the location in our memory heap where the first copied
    /// byte was placed.
    ///
    /// WASM modules are isolated from one another and cannot directly share memory. We must
    /// therefore copy buffers from Pdfium's memory heap across into our own, and vice versa.
    fn copy_bytes_from_pdfium(&self, ptr: usize, len: usize) -> Vec<u8> {
        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_from_pdfium(): entering");

        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_from_pdfium(): copying {} bytes from pointer offset {}", len, ptr);

        let copy = self
            .heap_u8()
            .slice(ptr as u32, (ptr + len) as u32)
            .to_vec();

        if self.debug {
            let mut differences_count = 0;

            let pdfium_heap = self.heap_u8();

            for (index, byte) in copy.iter().enumerate() {
                let src = pdfium_heap.get_index((ptr + index) as u32);

                if *byte != src {
                    differences_count += 1;

                    log::warn!(
                        "pdfium-render::PdfiumRenderWasmState::copy_bytes_from_pdfium(): byte index {} differs between source and destination: source data = {}, destination data = {}",
                        index,
                        src,
                        byte,
                    );
                }
            }

            log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_from_pdfium(): {} bytes differ between source and destination byte buffers", differences_count);
        }

        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_bytes_from_pdfium(): leaving");

        copy
    }

    /// Calls FPDF_GetLastError(), returning the result.
    ///
    /// We make this available as a separate function so that it can be used as part of
    /// a Mutex locking transaction.
    fn get_last_error(&self) -> c_ulong {
        self.call(
            "FPDF_GetLastError",
            JsFunctionArgumentType::Number,
            None,
            None,
        )
        .as_f64()
        .unwrap() as c_ulong
    }

    /// Stores the given key / value pair.
    fn set(&mut self, key: &str, value: JsValue) {
        log::debug!(
            "pdfium-render::PdfiumRenderWasmState::set(): setting key: {}, value: {:#?}",
            key,
            value
        );

        self.state.insert(String::from(key), value);
    }

    /// Retrieves the value associated with the given key, if any.
    fn get(&self, key: &str) -> Option<&JsValue> {
        let value = self.state.get(key);

        log::debug!(
            "pdfium-render::PdfiumRenderWasmState::get(): getting value for key: {}, value: {:#?}",
            key,
            value
        );

        value
    }
}

impl Default for PdfiumRenderWasmState {
    fn default() -> Self {
        PdfiumRenderWasmState {
            pdfium_wasm_module: None,
            malloc_js_fn: None,
            free_js_fn: None,
            call_js_fn: None,
            debug: false,
            state: HashMap::new(),
        }
    }
}

unsafe impl Send for PdfiumRenderWasmState {}

unsafe impl Sync for PdfiumRenderWasmState {}

/// Establishes a binding from `pdfium-render` to an external Pdfium WASM module.
/// This function should be called from Javascript once the external Pdfium WASM module has been loaded
/// into the browser. It is essential that this function is called _before_ initializing
/// `pdfium-render` from within Rust code. For an example, see:
/// <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
#[wasm_bindgen]
pub fn initialize_pdfium_render(pdfium: JsValue, debug: bool) -> bool {
    if console_log::init_with_level(if debug {
        log::Level::Trace
    } else {
        log::Level::Info
    })
    .is_err()
    {
        log::error!(
            "pdfium-render::initialize_pdfium_render(): Error initializing console-based logging"
        );
    }

    if debug {
        // Output full Rust stack traces to Javascript console.

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    if !pdfium.is_object() {
        log::error!("pdfium-render::initialize_pdfium_render(): provided module is not a valid Javascript Object");

        return false;
    }

    if pdfium.is_object() {
        match PdfiumRenderWasmState::lock().bind_to_pdfium(Object::from(pdfium), debug) {
            Ok(()) => true,
            Err(msg) => {
                log::error!("pdfium-render::initialize_pdfium_render(): {}", msg);

                false
            }
        }
    } else {
        log::error!("pdfium-render:initialize_pdfium_render(): value passed to initialize_pdfium_render() is not a Javascript Object");

        false
    }
}

pub(crate) struct WasmPdfiumBindings {}

impl WasmPdfiumBindings {
    #[inline]
    pub(crate) fn new() -> Self {
        WasmPdfiumBindings {}
    }

    /// Converts a pointer to an FPDF_DOCUMENT struct to a JsValue.
    #[inline]
    fn js_value_from_document(document: FPDF_DOCUMENT) -> JsValue {
        Self::js_value_from_offset(document as usize)
    }

    /// Converts a pointer to an FPDF_PAGE struct to a JsValue.
    #[inline]
    fn js_value_from_page(page: FPDF_PAGE) -> JsValue {
        Self::js_value_from_offset(page as usize)
    }

    /// Converts a pointer to an FPDF_FORMHANDLE struct to a JsValue.
    #[inline]
    fn js_value_from_form(form: FPDF_FORMHANDLE) -> JsValue {
        Self::js_value_from_offset(form as usize)
    }

    /// Converts a pointer to an FPDF_BITMAP struct to a JsValue.
    #[inline]
    fn js_value_from_bitmap(bitmap: FPDF_BITMAP) -> JsValue {
        Self::js_value_from_offset(bitmap as usize)
    }

    /// Converts a WASM memory heap offset to a JsValue.
    #[inline]
    fn js_value_from_offset(offset: usize) -> JsValue {
        JsValue::from_f64(offset as f64)
    }

    /// Converts a Vec<JsValue> to a Javascript Array.
    #[inline]
    fn js_array_from_vec(vec: Vec<JsValue>) -> Array {
        let array = Array::new_with_length(vec.len() as u32);

        for (index, value) in vec.into_iter().enumerate() {
            array.set(index as u32, value);
        }

        array
    }

    /// Calls an FPDF_Get*Box() function. Since all of these functions share the same
    /// signature, we abstract out the function call into this separate function so it can
    /// be re-used.
    #[inline]
    fn call_pdfium_get_page_box_fn(
        fn_name: &str,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        // Generic function used to retrieve bounding box co-ordinates for the
        // FPDFPage_Get*Box() functions.

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // empty buffers corresponding to four pointers into Pdfium's heap, call the given
        // Pdfium function, and copy the buffers back from Pdfium's heap into the given
        // pointer locations.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_float>();

        let ptr_left = state.malloc(len);

        let ptr_bottom = state.malloc(len);

        let ptr_right = state.malloc(len);

        let ptr_top = state.malloc(len);

        let result = state.call(
            fn_name,
            JsFunctionArgumentType::Number,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
            ]),
            Some(&JsValue::from(Array::of5(
                &Self::js_value_from_page(page),
                &Self::js_value_from_offset(ptr_left),
                &Self::js_value_from_offset(ptr_bottom),
                &Self::js_value_from_offset(ptr_right),
                &Self::js_value_from_offset(ptr_top),
            ))),
        );

        let result = result.as_f64().unwrap() as FPDF_BOOL;

        if result != 0 {
            unsafe {
                *left = state
                    .copy_bytes_from_pdfium(ptr_left, len)
                    .try_into()
                    .map(f32::from_ne_bytes)
                    .unwrap_or(0_f32);

                *bottom = state
                    .copy_bytes_from_pdfium(ptr_bottom, len)
                    .try_into()
                    .map(f32::from_ne_bytes)
                    .unwrap_or(0_f32);

                *right = state
                    .copy_bytes_from_pdfium(ptr_right, len)
                    .try_into()
                    .map(f32::from_ne_bytes)
                    .unwrap_or(0_f32);

                *top = state
                    .copy_bytes_from_pdfium(ptr_top, len)
                    .try_into()
                    .map(f32::from_ne_bytes)
                    .unwrap_or(0_f32);
            }
        }

        state.free(ptr_left);
        state.free(ptr_bottom);
        state.free(ptr_right);
        state.free(ptr_top);

        result
    }

    /// Calls an FPDF_Set*Box() function. Since all of these functions share the same
    /// signature, we abstract out the function call into this separate function so it can
    /// be re-used.
    #[inline]
    fn call_pdfium_set_page_box_fn(
        fn_name: &str,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        PdfiumRenderWasmState::lock().call(
            fn_name,
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Array::of5(
                &Self::js_value_from_page(page),
                &JsValue::from(left),
                &JsValue::from(bottom),
                &JsValue::from(right),
                &JsValue::from(top),
            ))),
        );
    }
}

impl Default for WasmPdfiumBindings {
    #[inline]
    fn default() -> Self {
        WasmPdfiumBindings::new()
    }
}

impl PdfiumLibraryBindings for WasmPdfiumBindings {
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_InitLibrary()");

        PdfiumRenderWasmState::lock().call("PDFium_Init", JsFunctionArgumentType::Void, None, None);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_DestroyLibrary()");

        PdfiumRenderWasmState::lock().call(
            "FPDF_DestroyLibrary",
            JsFunctionArgumentType::Void,
            None,
            None,
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetLastError()");

        PdfiumRenderWasmState::lock().get_last_error()
    }

    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, _file_path: &str, _password: Option<&str>) -> FPDF_DOCUMENT {
        // FPDF_LoadDocument() is not available on WASM. When compiling to WASM,
        // this function definition in the PdfiumLibraryBindings trait will be
        // entirely omitted, so calling code that attempts to call FPDF_LoadDocument()
        // will fail at compile-time, not run-time.

        unimplemented!()
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_LoadMemDocument(): entering");

        let mut state = PdfiumRenderWasmState::lock();

        let ptr = state.copy_bytes_to_pdfium(bytes);

        let result = state
            .call(
                "FPDF_LoadMemDocument",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::String,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_offset(ptr),
                    &Self::js_value_from_offset(bytes.len()),
                    &JsValue::from(password.unwrap_or("")),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_DOCUMENT;

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDF_LoadMemDocument(): FPDF_DOCUMENT = {:#?}",
            result
        );

        state.set(
            format!("document_ptr_{:#?}", result).as_str(),
            JsValue::from_f64(ptr as f64),
        );

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_LoadMemDocument(): leaving");

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_CloseDocument(): entering");

        let state = PdfiumRenderWasmState::lock();

        state.call(
            "FPDF_CloseDocument",
            JsFunctionArgumentType::Void,
            Some(vec![JsFunctionArgumentType::Pointer]),
            Some(&JsValue::from(Array::of1(&Self::js_value_from_document(
                document,
            )))),
        );

        // The bytes representing the document were previously copied across to a separate
        // buffer in Pdfium's WASM module using state.copy_bytes_to_pdfium().
        // Free that buffer now.

        let key = format!("document_ptr_{:#?}", document);

        if let Some(ptr) = state.get(key.as_str()) {
            state.free(ptr.as_f64().unwrap() as usize);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_CloseDocument(): leaving");
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetFileVersion()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer relating to the pointer to Pdfium's heap, call FPDF_GetFileVersion(),
        // and copy the buffer back from Pdfium's heap into the given pointer location.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_int>();

        let ptr = state.malloc(len);

        let result = state.call(
            "FPDF_GetFileVersion",
            JsFunctionArgumentType::Number,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
            ]),
            Some(&JsValue::from(Array::of2(
                &Self::js_value_from_document(doc),
                &Self::js_value_from_offset(ptr),
            ))),
        );

        unsafe {
            fileVersion.copy_from(
                state.copy_bytes_from_pdfium(ptr, len).as_ptr() as *mut c_int,
                len,
            );
        }

        result.as_f64().unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetFormType()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDF_GetFormType",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_document(
                    document,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): entering");

        let buflen = buflen as usize;

        let state = PdfiumRenderWasmState::lock();

        let c_tag = CString::new(tag).unwrap();

        let tag_ptr = state.copy_bytes_to_pdfium(&c_tag.into_bytes_with_nul());

        let buffer_ptr = if buflen > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): allocating buffer of {} bytes in Pdfium's WASM heap", buflen);

            state.malloc(buflen)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): calling FPDF_GetMetaText()"
        );

        let result = state.call(
            "FPDF_GetMetaText",
            JsFunctionArgumentType::Number,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Array::of4(
                &Self::js_value_from_document(document),
                &Self::js_value_from_offset(tag_ptr),
                &Self::js_value_from_offset(buffer_ptr),
                &Self::js_value_from_offset(buflen),
            ))),
        );

        if buflen > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buflen,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state.copy_bytes_from_pdfium(buffer_ptr, buflen).as_ptr() as *mut c_void,
                    buflen,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): leaving");

        result.as_f64().unwrap() as c_ulong
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageCount()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDF_GetPageCount",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_document(
                    document,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_LoadPage()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDF_LoadPage",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &JsValue::from(page_index),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGE
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_ClosePage()");

        PdfiumRenderWasmState::lock().call(
            "FPDF_ClosePage",
            JsFunctionArgumentType::Void,
            Some(vec![JsFunctionArgumentType::Pointer]),
            Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> c_float {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageWidthF()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDF_GetPageWidthF",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
            )
            .as_f64()
            .unwrap() as c_float
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> c_float {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageHeightF()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDF_GetPageHeightF",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
            )
            .as_f64()
            .unwrap() as c_float
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): entering");

        let buflen = buflen as usize;

        let state = PdfiumRenderWasmState::lock();

        let buffer_ptr = if buflen > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): allocating buffer of {} bytes in Pdfium's WASM heap", buflen);

            state.malloc(buflen)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): calling FPDF_GetPageLabel()"
        );

        let result = state.call(
            "FPDF_GetPageLabel",
            JsFunctionArgumentType::Number,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Array::of4(
                &Self::js_value_from_document(document),
                &JsValue::from(page_index),
                &Self::js_value_from_offset(buffer_ptr),
                &Self::js_value_from_offset(buflen),
            ))),
        );

        if buflen > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buflen,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state.copy_bytes_from_pdfium(buffer_ptr, buflen).as_ptr() as *mut c_void,
                    buflen,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): leaving");

        result.as_f64().unwrap() as c_ulong
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetRotation()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPage_GetRotation",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_SetRotation()");

        PdfiumRenderWasmState::lock().call(
            "FPDFPage_SetRotation",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Array::of2(
                &Self::js_value_from_page(page),
                &JsValue::from(rotate),
            ))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetPageBoundingBox()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer relating to the pointer to Pdfium's heap, call FPDF_GetPageBoundingBox(),
        // and copy the buffer back from Pdfium's heap into the given pointer location.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<FS_RECTF>();

        let ptr = state.malloc(len);

        let result = state.call(
            "FPDF_GetPageBoundingBox",
            JsFunctionArgumentType::Number,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
            ]),
            Some(&JsValue::from(Array::of2(
                &Self::js_value_from_page(page),
                &Self::js_value_from_offset(ptr),
            ))),
        );

        unsafe {
            rect.copy_from(
                state.copy_bytes_from_pdfium(ptr, len).as_ptr() as *mut FS_RECTF,
                len,
            );
        }

        state.free(ptr);

        result.as_f64().unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetMediaBox()");

        Self::call_pdfium_get_page_box_fn("FPDFPage_GetMediaBox", page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetCropBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetCropBox()");

        Self::call_pdfium_get_page_box_fn("FPDFPage_GetCropBox", page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetBleedBox()");

        Self::call_pdfium_get_page_box_fn("FPDFPage_GetBleedBox", page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetTrimBox()");

        Self::call_pdfium_get_page_box_fn("FPDFPage_GetTrimBox", page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetArtBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetArtBox()");

        Self::call_pdfium_get_page_box_fn("FPDFPage_GetArtBox", page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_SetMediaBox()");

        Self::call_pdfium_set_page_box_fn("FPDFPage_SetMediaBox", page, left, bottom, right, top);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetCropBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_SetCropBox()");

        Self::call_pdfium_set_page_box_fn("FPDFPage_SetCropBox", page, left, bottom, right, top);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_SetBleedBox()");

        Self::call_pdfium_set_page_box_fn("FPDFPage_SetBleedBox", page, left, bottom, right, top);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_SetTrimBox()");

        Self::call_pdfium_set_page_box_fn("FPDFPage_SetTrimBox", page, left, bottom, right, top);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetArtBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_SetArtBox()");

        Self::call_pdfium_set_page_box_fn("FPDFPage_SetArtBox", page, left, bottom, right, top);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_CreateEx(
        &self,
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_CreateEx()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBitmap_CreateEx",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of5(
                    &JsValue::from(width),
                    &JsValue::from(height),
                    &JsValue::from(format),
                    &Self::js_value_from_offset(first_scan as usize),
                    &JsValue::from(stride),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_BITMAP
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_Destroy()");

        PdfiumRenderWasmState::lock().call(
            "FPDFBitmap_Destroy",
            JsFunctionArgumentType::Void,
            Some(vec![JsFunctionArgumentType::Pointer]),
            Some(&JsValue::from(Array::of1(&Self::js_value_from_bitmap(
                bitmap,
            )))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_FillRect()");

        PdfiumRenderWasmState::lock().call(
            "FPDFBitmap_FillRect",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Self::js_array_from_vec(vec![
                Self::js_value_from_bitmap(bitmap),
                JsValue::from(left),
                JsValue::from(top),
                JsValue::from(width),
                JsValue::from(height),
                JsValue::from(color),
            ]))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_GetBuffer()");

        let width = self.FPDFBitmap_GetWidth(bitmap);

        let height = self.FPDFBitmap_GetHeight(bitmap);

        let state = PdfiumRenderWasmState::lock();

        let buffer_ptr = state
            .call(
                "FPDFBitmap_GetBuffer",
                JsFunctionArgumentType::Pointer,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_bitmap(
                    bitmap,
                )))),
            )
            .as_f64()
            .unwrap() as usize;

        let buffer = state.copy_bytes_from_pdfium(
            buffer_ptr,
            (width * height * PdfiumRenderWasmState::BYTES_PER_PIXEL) as usize,
        );

        buffer.as_ptr() as *mut c_void
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_GetWidth()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBitmap_GetWidth",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_bitmap(
                    bitmap,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_GetHeight()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBitmap_GetHeight",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_bitmap(
                    bitmap,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBitmap_GetStride()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBitmap_GetStride",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_bitmap(
                    bitmap,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmap(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_RenderPageBitmap()");

        PdfiumRenderWasmState::lock().call(
            "FPDF_RenderPageBitmap",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Self::js_array_from_vec(vec![
                Self::js_value_from_bitmap(bitmap),
                Self::js_value_from_page(page),
                JsValue::from(start_x),
                JsValue::from(start_y),
                JsValue::from(size_x),
                JsValue::from(size_y),
                JsValue::from(rotate),
                JsValue::from(flags),
            ]))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer for an FPDF_FORMFILLINFO struct to Pdfium's heap,
        // copy the form_info struct we have been given to that buffer, and call
        // FPDFDOC_InitFormFillEnvironment() using a pointer to the buffer in Pdfium's WASM
        // heap rather than the pointer we have been given.

        let state = PdfiumRenderWasmState::lock();

        let ptr = state.copy_struct_to_pdfium_mut(form_info);

        state
            .call(
                "FPDFDOC_InitFormFillEnvironment",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_offset(ptr),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_FORMHANDLE
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFDOC_ExitFormFillEnvironment()");

        PdfiumRenderWasmState::lock().call(
            "FPDFDOC_ExitFormFillEnvironment",
            JsFunctionArgumentType::Void,
            Some(vec![JsFunctionArgumentType::Pointer]),
            Some(&JsValue::from(Array::of1(&Self::js_value_from_form(
                handle,
            )))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFDoc_GetPageMode()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFDoc_GetPageMode",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_document(
                    document,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_SetFormFieldHighlightColor()");

        PdfiumRenderWasmState::lock().call(
            "FPDF_SetFormFieldHighlightColor",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Array::of3(
                &Self::js_value_from_form(handle),
                &JsValue::from(field_type),
                &JsValue::from(color),
            ))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_SetFormFieldHighlightAlpha()");

        PdfiumRenderWasmState::lock().call(
            "FPDF_SetFormFieldHighlightAlpha",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Array::of2(
                &Self::js_value_from_form(handle),
                &JsValue::from(alpha),
            ))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_FFLDraw(
        &self,
        handle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_FFLDraw()");

        PdfiumRenderWasmState::lock().call(
            "FPDF_FFLDraw",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Self::js_array_from_vec(vec![
                Self::js_value_from_form(handle),
                Self::js_value_from_bitmap(bitmap),
                Self::js_value_from_page(page),
                JsValue::from_f64(start_x as f64),
                JsValue::from_f64(start_y as f64),
                JsValue::from_f64(size_x as f64),
                JsValue::from_f64(size_y as f64),
                JsValue::from_f64(rotate as f64),
                JsValue::from_f64(flags as f64),
            ]))),
        );
    }
}
