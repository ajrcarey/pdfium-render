use crate::bindgen::{
    size_t, FPDF_ACTION, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL, FPDF_BYTESTRING, FPDF_DEST,
    FPDF_DOCUMENT, FPDF_DWORD, FPDF_FILEACCESS, FPDF_FONT, FPDF_FORMFILLINFO, FPDF_FORMHANDLE,
    FPDF_IMAGEOBJ_METADATA, FPDF_OBJECT_TYPE, FPDF_PAGE, FPDF_PAGEOBJECT, FPDF_PAGEOBJECTMARK,
    FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR, FPDF_WIDESTRING, FS_MATRIX, FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use js_sys::{Array, Function, Object, Reflect, Uint8Array};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::{c_void, CString};
use std::mem::size_of;
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort};
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

    /// Copies a given number of bytes starting at the given pointer into Pdfium's WASM memory heap,
    /// returning a pointer to the copied bytes at the destination location.
    ///
    /// WASM modules are isolated from one another and cannot directly share memory. We must
    /// therefore copy buffers from our own memory heap across into Pdfium's memory heap, and vice versa.
    #[inline]
    fn copy_ptr_with_len_to_pdfium<T>(&self, ptr: *const T, len: usize) -> usize {
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
    fn copy_struct_to_pdfium<T>(&self, ptr: *const T) -> usize {
        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_struct_to_pdfium(): entering");

        let len = size_of::<T>();

        log::debug!(
            "pdfium-render::PdfiumRenderWasmState::copy_struct_to_pdfium(): struct is at heap offset {}, data length {} bytes",
            ptr as usize as u32,
            len
        );

        let result = self.copy_bytes_to_pdfium(
            unsafe { Uint8Array::view_mut_raw(ptr as *mut u8, len) }
                .to_vec()
                .as_slice(),
        );

        log::debug!("pdfium-render::PdfiumRenderWasmState::copy_struct_to_pdfium(): leaving");

        result
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

    /// Copies the give FPDF_FILEACCESS struct into Pdfium's WASM heap, returning a
    /// pointer to the copied struct at the destination location.
    ///
    /// The FPDF_FILEACCESS struct contains a pointer to a callback function that Pdfium
    /// repeatedly calls to load (or save) blocks of data from (or to) a file. That function
    /// pointer points to a location in our local WASM memory heap, but Pdfium needs a pointer to
    /// a function in its own WASM heap. Simply copying the function to Pdfium's WASM heap
    /// is not sufficient, since the function will no longer have access to any data it expects
    /// to find in our local WASM heap.
    ///
    /// We instead create a new FPDF_FILEACCESS struct in Pdfium's WASM heap that contains
    /// a wrapper function that serves as a conduit between Pdfium's memory heap and the
    /// callback function in our own local WASM heap.
    fn copy_file_access_to_pdfium(&self, _file_access: *mut FPDF_FILEACCESS) -> usize {
        // TODO: AJRC - 10/2/22 - https://github.com/ajrcarey/pdfium-render/issues/8
        todo!()
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

        console_error_panic_hook::set_once();
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
        log::error!("pdfium-render::initialize_pdfium_render(): provided module is not a valid Javascript Object");

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

    /// Converts a mutable pointer to an FPDF_PAGE struct to a JsValue.
    #[inline]
    fn js_value_from_page_mut(page: *mut FPDF_PAGE) -> JsValue {
        Self::js_value_from_offset(page as usize)
    }

    /// Converts a pointer to an FPDF_TEXTPAGE struct to a JsValue.
    #[inline]
    fn js_value_from_text_page(text_page: FPDF_TEXTPAGE) -> JsValue {
        Self::js_value_from_offset(text_page as usize)
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

    /// Converts a pointer to an FPDF_ACTION struct to a JsValue.
    #[inline]
    fn js_value_from_action(action: FPDF_ACTION) -> JsValue {
        Self::js_value_from_offset(action as usize)
    }

    /// Converts a pointer to an FPDF_DEST struct to a JsValue.
    #[inline]
    fn js_value_from_destination(dest: FPDF_DEST) -> JsValue {
        Self::js_value_from_offset(dest as usize)
    }

    /// Converts a pointer to an FPDF_PAGEOBJECT struct to a JsValue.
    #[inline]
    fn js_value_from_object(object: FPDF_PAGEOBJECT) -> JsValue {
        Self::js_value_from_offset(object as usize)
    }

    /// Converts a pointer to an FPDF_FONT struct to a JsValue.
    #[inline]
    fn js_value_from_font(font: FPDF_FONT) -> JsValue {
        Self::js_value_from_offset(font as usize)
    }

    /// Converts a pointer to an FPDF_BOOKMARK struct to a JsValue.
    #[inline]
    fn js_value_from_bookmark(bookmark: FPDF_BOOKMARK) -> JsValue {
        Self::js_value_from_offset(bookmark as usize)
    }

    /// Converts a pointer to an FPDF_PAGEOBJECTMARK struct to a JsValue.
    #[inline]
    fn js_value_from_mark(mark: FPDF_PAGEOBJECTMARK) -> JsValue {
        Self::js_value_from_offset(mark as usize)
    }

    /// Converts a pointer to an FPDF_WIDESTRING struct to a JsValue.
    #[inline]
    fn js_value_from_wstr(str: FPDF_WIDESTRING) -> JsValue {
        Self::js_value_from_offset(str as usize)
    }

    /// Converts a pointer to an FPDF_BYTESTRING struct to a JsValue.
    #[inline]
    fn js_value_from_bstr(str: FPDF_BYTESTRING) -> JsValue {
        Self::js_value_from_offset(str as usize)
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
        &self,
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

        let result = state
            .call(
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
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
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
        &self,
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

        let buffer_length = buflen as usize;

        let state = PdfiumRenderWasmState::lock();

        let c_tag = CString::new(tag).unwrap();

        let tag_ptr = state.copy_bytes_to_pdfium(&c_tag.into_bytes_with_nul());

        let buffer_ptr = if buffer_length > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_length);

            state.malloc(buffer_length)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): calling FPDF_GetMetaText()"
        );

        let result = state
            .call(
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
                    &Self::js_value_from_offset(buffer_length),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_length,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_void,
                    buffer_length,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetMetaText(): leaving");

        result
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
                    &JsValue::from_f64(page_index as f64),
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

        let buffer_length = buflen as usize;

        let state = PdfiumRenderWasmState::lock();

        let buffer_ptr = if buffer_length > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_length);

            state.malloc(buffer_length)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): calling FPDF_GetPageLabel()"
        );

        let result = state
            .call(
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
                    &Self::js_value_from_offset(buffer_length),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_length,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_void,
                    buffer_length,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDF_GetPageLabel(): leaving");

        result
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

        self.call_pdfium_get_page_box_fn("FPDFPage_GetMediaBox", page, left, bottom, right, top)
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

        self.call_pdfium_get_page_box_fn("FPDFPage_GetCropBox", page, left, bottom, right, top)
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

        self.call_pdfium_get_page_box_fn("FPDFPage_GetBleedBox", page, left, bottom, right, top)
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

        self.call_pdfium_get_page_box_fn("FPDFPage_GetTrimBox", page, left, bottom, right, top)
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

        self.call_pdfium_get_page_box_fn("FPDFPage_GetArtBox", page, left, bottom, right, top)
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

        self.call_pdfium_set_page_box_fn("FPDFPage_SetMediaBox", page, left, bottom, right, top);
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

        self.call_pdfium_set_page_box_fn("FPDFPage_SetCropBox", page, left, bottom, right, top);
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

        self.call_pdfium_set_page_box_fn("FPDFPage_SetBleedBox", page, left, bottom, right, top);
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

        self.call_pdfium_set_page_box_fn("FPDFPage_SetTrimBox", page, left, bottom, right, top);
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

        self.call_pdfium_set_page_box_fn("FPDFPage_SetArtBox", page, left, bottom, right, top);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_HasTransparency()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPage_HasTransparency",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
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

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetFirstChild()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBookmark_GetFirstChild",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_bookmark(bookmark),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_BOOKMARK
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetNextSibling()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBookmark_GetNextSibling",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_bookmark(bookmark),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_BOOKMARK
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetTitle(): entering");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_len = buflen as usize;

        let buffer_ptr = if buffer_len > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetTitle(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_len);

            state.malloc(buffer_len)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetTitle(): calling FPDFBookmark_GetTitle()"
        );

        let result = state
            .call(
                "FPDFBookmark_GetTitle",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_bookmark(bookmark),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_len as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetTitle(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_len,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_len)
                        .as_ptr() as *mut c_void,
                    buffer_len,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetTitle(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetTitle(): leaving");

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_Find()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBookmark_Find",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_wstr(title),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_BOOKMARK
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetDest()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBookmark_GetDest",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_bookmark(bookmark),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_DEST
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFBookmark_GetAction()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFBookmark_GetAction",
                JsFunctionArgumentType::Pointer,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_bookmark(
                    bookmark,
                )))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_ACTION
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetType()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFAction_GetType",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_action(
                    action,
                )))),
            )
            .as_f64()
            .unwrap() as c_ulong
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetDest()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFAction_GetDest",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_action(action),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_DEST
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetFilePath(): entering");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_len = buflen as usize;

        let buffer_ptr = if buffer_len > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetFilePath(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_len);

            state.malloc(buffer_len)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFAction_GetFilePath(): calling FPDFAction_GetFilePath()"
        );

        let result = state
            .call(
                "FPDFAction_GetFilePath",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_action(action),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_len as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFAction_GetFilePath(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_len,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_len)
                        .as_ptr() as *mut c_void,
                    buffer_len,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetFilePath(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetFilePath(): leaving");

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetURIPath(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetURIPath(): entering");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_len = buflen as usize;

        let buffer_ptr = if buffer_len > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetURIPath(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_len);

            state.malloc(buffer_len)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFAction_GetURIPath(): calling FPDFAction_GetURIPath()"
        );

        let result = state
            .call(
                "FPDFAction_GetURIPath",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_action(action),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_len as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFAction_GetURIPath(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_len,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_len)
                        .as_ptr() as *mut c_void,
                    buffer_len,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetURIPath(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFAction_GetURIPath(): leaving");

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFDest_GetDestPageIndex()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFDest_GetDestPageIndex",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_destination(dest),
                ))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_LoadPage()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFText_LoadPage",
                JsFunctionArgumentType::Pointer,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_TEXTPAGE
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_ClosePage()");

        PdfiumRenderWasmState::lock().call(
            "FPDFText_ClosePage",
            JsFunctionArgumentType::Void,
            Some(vec![JsFunctionArgumentType::Pointer]),
            Some(&JsValue::from(Array::of1(&Self::js_value_from_text_page(
                text_page,
            )))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_CountChars()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFText_CountChars",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_text_page(
                    text_page,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetBoundedText(
        &self,
        text_page: FPDF_TEXTPAGE,
        left: c_double,
        top: c_double,
        right: c_double,
        bottom: c_double,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_GetBoundedText(): entering");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize * size_of::<c_ushort>();

        let buffer_ptr = if buffer_length > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_GetBoundedText(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_length);

            state.malloc(buffer_length)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFText_GetBoundedText(): calling FPDFText_GetBoundedText()"
        );

        let result = state
            .call(
                "FPDFText_GetBoundedText",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Self::js_array_from_vec(vec![
                    Self::js_value_from_text_page(text_page),
                    JsValue::from(left),
                    JsValue::from(top),
                    JsValue::from(right),
                    JsValue::from(bottom),
                    Self::js_value_from_offset(buffer_ptr),
                    JsValue::from_f64(buflen as f64),
                ]))),
            )
            .as_f64()
            .unwrap() as c_int;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFText_GetBoundedText(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_length,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_ushort,
                    // It is critical we use buflen here rather than buffer_length,
                    // since buflen refers to the number of _c_ushorts_ to copy, but
                    // buffer_length refers to the number of _bytes_ in the buffer.
                    buflen as usize,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_GetBoundedText(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_GetBoundedText(): leaving");

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFFormObj_CountObjects()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFFormObj_CountObjects",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    form_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFFormObj_GetObject()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFFormObj_GetObject",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(form_object),
                    &JsValue::from_f64(index as f64),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_CreateTextObj()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_CreateTextObj",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_font(font),
                    &JsValue::from(font_size),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetTextRenderMode()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFTextObj_GetTextRenderMode",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    text,
                )))),
            )
            .as_f64()
            .unwrap() as FPDF_TEXT_RENDERMODE
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_SetTextRenderMode()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFTextObj_SetTextRenderMode",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(text),
                    &JsValue::from_f64(render_mode as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetText(
        &self,
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetText(): entering");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_len = length as usize;

        let buffer_ptr = if buffer_len > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetText(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_len);

            state.malloc(buffer_len)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetText(): calling FPDFTextObj_GetText()"
        );

        let result = state
            .call(
                "FPDFTextObj_GetText",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_object(text_object),
                    &Self::js_value_from_text_page(text_page),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_len as f64),
                ))),
            )
            .as_f64()
            .unwrap() as usize;

        if result <= buffer_len && result > 0 && buffer_len > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetText(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                result,
                buffer as usize
            );

            unsafe {
                // result is expressed in bytes, but buffer.copy_from() expects the _count_
                // of pointer-sized objects to copy, i.e. the number of FPDF_WCHARs; this is
                // not the same as the number of bytes. Giving buffer.copy_from() the raw result
                // corrupts the WASM memory heap, leading to strange errors later on. Tracking
                // down this bug led me on quite the annoying wild goose chase; see:
                // https://github.com/ajrcarey/pdfium-render/issues/11

                let char_count = result / size_of::<FPDF_WCHAR>();

                buffer.copy_from(
                    state.copy_bytes_from_pdfium(buffer_ptr, result).as_ptr() as *mut FPDF_WCHAR,
                    char_count,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetText(): freeing buffer in Pdfium's WASM heap");
        }

        if buffer_len > 0 {
            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetText(): leaving");

        result as c_ulong
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetFont()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFTextObj_GetFont",
                JsFunctionArgumentType::Pointer,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    text,
                )))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_FONT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFTextObj_GetFontSize()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_float>();

        let ptr_size = state.malloc(len);

        let result = state
            .call(
                "FPDFTextObj_GetFontSize",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(text),
                    &Self::js_value_from_offset(ptr_size),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *size = state
                    .copy_bytes_from_pdfium(ptr_size, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);
            }
        }

        state.free(ptr_size);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_BYTESTRING,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_NewTextObj()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_NewTextObj",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_bstr(font),
                    &JsValue::from(font_size),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_SetText()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFText_SetText",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(text_object),
                    &Self::js_value_from_wstr(text),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFText_SetCharcodes()");

        let state = PdfiumRenderWasmState::lock();

        let ptr_charcodes =
            state.copy_ptr_with_len_to_pdfium(charcodes, size_of::<c_uint>() * count as usize);

        let result = state
            .call(
                "FPDFText_SetCharcodes",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_object(text_object),
                    &Self::js_value_from_offset(ptr_charcodes),
                    &JsValue::from_f64(count as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        state.free(ptr_charcodes);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_InsertObject()");

        PdfiumRenderWasmState::lock().call(
            "FPDFPage_InsertObject",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
            ]),
            Some(&JsValue::from(Array::of2(
                &Self::js_value_from_page(page),
                &Self::js_value_from_object(page_obj),
            ))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_RemoveObject()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPage_RemoveObject",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_page(page),
                    &Self::js_value_from_object(page_obj),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_CountObjects()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPage_CountObjects",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_page(page)))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPage_GetObject()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPage_GetObject",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_page(page),
                    &JsValue::from_f64(index as f64),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Destroy(&self, page_obj: FPDF_PAGEOBJECT) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_Destroy()");

        PdfiumRenderWasmState::lock().call(
            "FPDFPageObj_Destroy",
            JsFunctionArgumentType::Void,
            Some(vec![JsFunctionArgumentType::Pointer]),
            Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                page_obj,
            )))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_HasTransparency()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_HasTransparency",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    page_object,
                )))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetType()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_GetType",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    page_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Transform(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_Transform()");

        PdfiumRenderWasmState::lock().call(
            "FPDFPageObj_Transform",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
                JsFunctionArgumentType::Number,
            ]),
            Some(&JsValue::from(Self::js_array_from_vec(vec![
                Self::js_value_from_object(page_object),
                JsValue::from(a),
                JsValue::from(b),
                JsValue::from(c),
                JsValue::from(d),
                JsValue::from(e),
                JsValue::from(f),
            ]))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetMatrix()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = size_of::<FS_MATRIX>();

        let ptr_matrix = state.copy_struct_to_pdfium_mut(matrix);

        let result = state
            .call(
                "FPDFPageObj_GetMatrix",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(ptr_matrix),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                matrix.copy_from(
                    state
                        .copy_bytes_from_pdfium(ptr_matrix, buffer_length)
                        .as_ptr() as *mut FS_MATRIX,
                    buffer_length,
                );
            }
        }

        state.free(ptr_matrix);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetMatrix(&self, path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetMatrix()");

        let state = PdfiumRenderWasmState::lock();

        let ptr_matrix = state.copy_struct_to_pdfium(matrix);

        let result = state
            .call(
                "FPDFPageObj_SetMatrix",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(path),
                    &Self::js_value_from_offset(ptr_matrix),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        state.free(ptr_matrix);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_NewImageObj()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_NewImageObj",
                JsFunctionArgumentType::Pointer,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_document(
                    document,
                )))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_CountMarks()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_CountMarks",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    page_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECTMARK {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetMark()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_GetMark",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from_f64(index as f64),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECTMARK
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_AddMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        name: FPDF_BYTESTRING,
    ) -> FPDF_PAGEOBJECTMARK {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_AddMark()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_AddMark",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_bstr(name),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECTMARK
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_RemoveMark()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_RemoveMark",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_mark(mark),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetName(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_GetName()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = if buffer_length > 0 {
            state.malloc(buffer_length)
        } else {
            0
        };

        let out_buflen_length = size_of::<c_ulong>();

        let out_buflen_ptr = state.malloc(out_buflen_length);

        let result = state
            .call(
                "FPDFPageObjMark_GetName",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                    &Self::js_value_from_offset(out_buflen_ptr),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *out_buflen = state
                    .copy_bytes_from_pdfium(out_buflen_ptr, out_buflen_length)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);
            }

            if buffer_length > 0 {
                unsafe {
                    buffer.copy_from(
                        state
                            .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                            .as_ptr() as *mut c_void,
                        buffer_length,
                    )
                }
            }
        }

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_CountParams()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObjMark_CountParams",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_mark(mark)))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamKey(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        index: c_ulong,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_GetParamKey()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = if buffer_length > 0 {
            state.malloc(buffer_length)
        } else {
            0
        };

        let out_buflen_length = size_of::<c_ulong>();

        let out_buflen_ptr = state.malloc(out_buflen_length);

        let result = state
            .call(
                "FPDFPageObjMark_GetParamKey",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_mark(mark),
                    &JsValue::from_f64(index as f64),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                    &Self::js_value_from_offset(out_buflen_ptr),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *out_buflen = state
                    .copy_bytes_from_pdfium(out_buflen_ptr, out_buflen_length)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);
            }

            if buffer_length > 0 {
                unsafe {
                    buffer.copy_from(
                        state
                            .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                            .as_ptr() as *mut c_void,
                        buffer_length,
                    )
                }
            }
        }

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_OBJECT_TYPE {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_GetParamValueType()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObjMark_GetParamValueType",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_OBJECT_TYPE
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        out_value: *mut c_int,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_GetParamIntValue()");

        let state = PdfiumRenderWasmState::lock();

        let out_value_length = size_of::<c_ulong>();

        let out_value_ptr = state.malloc(out_value_length);

        let result = state
            .call(
                "FPDFPageObjMark_GetParamIntValue",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                    &Self::js_value_from_offset(out_value_ptr),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *out_value = state
                    .copy_bytes_from_pdfium(out_value_ptr, out_value_length)
                    .try_into()
                    .map(i32::from_le_bytes)
                    .unwrap_or(0);
            }
        }

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_GetParamStringValue()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = if buffer_length > 0 {
            state.malloc(buffer_length)
        } else {
            0
        };

        let out_buflen_length = size_of::<c_ulong>();

        let out_buflen_ptr = state.malloc(out_buflen_length);

        let result = state
            .call(
                "FPDFPageObjMark_GetParamStringValue",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                    &Self::js_value_from_offset(out_buflen_ptr),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *out_buflen = state
                    .copy_bytes_from_pdfium(out_buflen_ptr, out_buflen_length)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);
            }

            if buffer_length > 0 {
                unsafe {
                    buffer.copy_from(
                        state
                            .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                            .as_ptr() as *mut c_void,
                        buffer_length,
                    )
                }
            }
        }

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_GetParamBlobValue()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = if buffer_length > 0 {
            state.malloc(buffer_length)
        } else {
            0
        };

        let out_buflen_length = size_of::<c_ulong>();

        let out_buflen_ptr = state.malloc(out_buflen_length);

        let result = state
            .call(
                "FPDFPageObjMark_GetParamKey",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                    &Self::js_value_from_offset(out_buflen_ptr),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *out_buflen = state
                    .copy_bytes_from_pdfium(out_buflen_ptr, out_buflen_length)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);
            }

            if buffer_length > 0 {
                unsafe {
                    buffer.copy_from(
                        state
                            .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                            .as_ptr() as *mut c_void,
                        buffer_length,
                    )
                }
            }
        }

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetIntParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: c_int,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_SetIntParam()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObjMark_SetIntParam",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                    &JsValue::from_f64(value as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetStringParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_SetStringParam()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObjMark_SetStringParam",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                    &Self::js_value_from_bstr(value),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_SetBlobParam()");

        let state = PdfiumRenderWasmState::lock();

        let ptr_value = state.copy_ptr_with_len_to_pdfium(value, value_len as usize);

        state
            .call(
                "FPDFPageObjMark_SetBlobParam",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Self::js_array_from_vec(vec![
                    Self::js_value_from_document(document),
                    Self::js_value_from_object(page_object),
                    Self::js_value_from_mark(mark),
                    Self::js_value_from_bstr(key),
                    Self::js_value_from_offset(ptr_value),
                    JsValue::from_f64(value_len as f64),
                ]))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObjMark_RemoveParam()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObjMark_RemoveParam",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_mark(mark),
                    &Self::js_value_from_bstr(key),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFile(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_LoadJpegFile()");

        let state = PdfiumRenderWasmState::lock();

        let ptr_file_access = state.copy_file_access_to_pdfium(file_access);

        state
            .call(
                "FPDFImageObj_LoadJpegFile",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_page_mut(pages),
                    &JsValue::from_f64(count as f64),
                    &Self::js_value_from_object(image_object),
                    &Self::js_value_from_offset(ptr_file_access),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFileInline(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_LoadJpegFileInline()");

        let state = PdfiumRenderWasmState::lock();

        let ptr_file_access = state.copy_file_access_to_pdfium(file_access);

        state
            .call(
                "FPDFImageObj_LoadJpegFile",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_page_mut(pages),
                    &JsValue::from_f64(count as f64),
                    &Self::js_value_from_object(image_object),
                    &Self::js_value_from_offset(ptr_file_access),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_SetMatrix(
        &self,
        image_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_SetMatrix()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFImageObj_SetMatrix",
                JsFunctionArgumentType::Void,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Self::js_array_from_vec(vec![
                    Self::js_value_from_object(image_object),
                    JsValue::from(a),
                    JsValue::from(b),
                    JsValue::from(c),
                    JsValue::from(d),
                    JsValue::from(e),
                    JsValue::from(f),
                ]))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_SetBitmap(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_SetBitmap()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFImageObj_SetBitmap",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_page_mut(pages),
                    &JsValue::from_f64(count as f64),
                    &Self::js_value_from_object(image_object),
                    &Self::js_value_from_bitmap(bitmap),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetBitmap()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFImageObj_GetBitmap",
                JsFunctionArgumentType::Pointer,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    image_object,
                )))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_BITMAP
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetRenderedBitmap()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFImageObj_GetRenderedBitmap",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_document(document),
                    &Self::js_value_from_page(page),
                    &Self::js_value_from_object(image_object),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_BITMAP
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataDecoded()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = if buffer_length > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataDecoded(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_length);

            state.malloc(buffer_length)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataDecoded(): calling FPDFImageObj_GetImageDataDecoded()"
        );

        let result = state
            .call(
                "FPDFImageObj_GetImageDataDecoded",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_object(image_object),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataDecoded(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_length,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_void,
                    buffer_length,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataDecoded(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataDecoded(): leaving"
        );

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataRaw()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = if buffer_length > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataRaw(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_length);

            state.malloc(buffer_length)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataRaw(): calling FPDFImageObj_GetImageDataRaw()"
        );

        let result = state
            .call(
                "FPDFImageObj_GetImageDataRaw",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_object(image_object),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataRaw(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_length,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_void,
                    buffer_length,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataRaw(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageDataRaw(): leaving"
        );

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageFilterCount()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFImageObj_GetImageFilterCount",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    image_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilter(
        &self,
        image_object: FPDF_PAGEOBJECT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageFilter()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = buflen as usize;

        let buffer_ptr = state.malloc(buffer_length);

        let result = state
            .call(
                "FPDFImageObj_GetImageFilter",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_object(image_object),
                    &JsValue::from_f64(index as f64),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= buflen && result > 0 {
            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_void,
                    buffer_length,
                );
            }
        }

        state.free(buffer_ptr);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFImageObj_GetImageMetadata()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = size_of::<FPDF_IMAGEOBJ_METADATA>();

        let buffer_ptr = state.malloc(buffer_length);

        let result = state
            .call(
                "FPDFImageObj_GetImageMetadata",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_object(image_object),
                    &Self::js_value_from_page(page),
                    &Self::js_value_from_offset(buffer_ptr),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                metadata.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut FPDF_IMAGEOBJ_METADATA,
                    buffer_length,
                );
            }
        }

        state.free(buffer_ptr);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewPath(&self, x: c_float, y: c_float) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_CreateNewPath()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_CreateNewPath",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &JsValue::from(x),
                    &JsValue::from(y),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewRect(
        &self,
        x: c_float,
        y: c_float,
        w: c_float,
        h: c_float,
    ) -> FPDF_PAGEOBJECT {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_CreateNewRect()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_CreateNewRect",
                JsFunctionArgumentType::Pointer,
                Some(vec![
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of4(
                    &JsValue::from(x),
                    &JsValue::from(y),
                    &JsValue::from(w),
                    &JsValue::from(h),
                ))),
            )
            .as_f64()
            .unwrap() as usize as FPDF_PAGEOBJECT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetBounds(
        &self,
        page_object: FPDF_PAGEOBJECT,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetBounds()");

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

        let result = state
            .call(
                "FPDFPageObj_GetBounds",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(ptr_left),
                    &Self::js_value_from_offset(ptr_bottom),
                    &Self::js_value_from_offset(ptr_right),
                    &Self::js_value_from_offset(ptr_top),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *left = state
                    .copy_bytes_from_pdfium(ptr_left, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);

                *bottom = state
                    .copy_bytes_from_pdfium(ptr_bottom, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);

                *right = state
                    .copy_bytes_from_pdfium(ptr_right, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);

                *top = state
                    .copy_bytes_from_pdfium(ptr_top, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);
            }
        }

        state.free(ptr_left);
        state.free(ptr_bottom);
        state.free(ptr_right);
        state.free(ptr_top);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING) {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetBlendMode()");

        PdfiumRenderWasmState::lock().call(
            "FPDFPageObj_SetBlendMode",
            JsFunctionArgumentType::Void,
            Some(vec![
                JsFunctionArgumentType::Pointer,
                JsFunctionArgumentType::Pointer,
            ]),
            Some(&JsValue::from(Array::of2(
                &Self::js_value_from_object(page_object),
                &Self::js_value_from_bstr(blend_mode),
            ))),
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetStrokeColor()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_SetStrokeColor",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from_f64(R as f64),
                    &JsValue::from_f64(G as f64),
                    &JsValue::from_f64(B as f64),
                    &JsValue::from_f64(A as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetStrokeColor()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // empty buffers corresponding to four pointers into Pdfium's heap, call the given
        // Pdfium function, and copy the buffers back from Pdfium's heap into the given
        // pointer locations.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_uint>();

        let ptr_r = state.malloc(len);

        let ptr_g = state.malloc(len);

        let ptr_b = state.malloc(len);

        let ptr_a = state.malloc(len);

        let result = state
            .call(
                "FPDFPageObj_GetStrokeColor",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(ptr_r),
                    &Self::js_value_from_offset(ptr_g),
                    &Self::js_value_from_offset(ptr_b),
                    &Self::js_value_from_offset(ptr_a),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *R = state
                    .copy_bytes_from_pdfium(ptr_r, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);

                *G = state
                    .copy_bytes_from_pdfium(ptr_g, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);

                *B = state
                    .copy_bytes_from_pdfium(ptr_b, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);

                *A = state
                    .copy_bytes_from_pdfium(ptr_a, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);
            }
        }

        state.free(ptr_r);
        state.free(ptr_g);
        state.free(ptr_b);
        state.free(ptr_a);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetStrokeWidth()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_SetStrokeWidth",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from(width),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetStrokeWidth()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_float>();

        let ptr_width = state.malloc(len);

        let result = state
            .call(
                "FPDFPageObj_GetStrokeWidth",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(ptr_width),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *width = state
                    .copy_bytes_from_pdfium(ptr_width, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);
            }
        }

        state.free(ptr_width);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetLineJoin()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_GetLineJoin",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    page_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetLineJoin()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_SetLineJoin",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from_f64(line_join as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetLineCap()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_GetLineCap",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    page_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetLineCap()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_SetLineCap",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from_f64(line_cap as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetFillColor()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_SetFillColor",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from_f64(R as f64),
                    &JsValue::from_f64(G as f64),
                    &JsValue::from_f64(B as f64),
                    &JsValue::from_f64(A as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetFillColor()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // empty buffers corresponding to four pointers into Pdfium's heap, call the given
        // Pdfium function, and copy the buffers back from Pdfium's heap into the given
        // pointer locations.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_uint>();

        let ptr_r = state.malloc(len);

        let ptr_g = state.malloc(len);

        let ptr_b = state.malloc(len);

        let ptr_a = state.malloc(len);

        let result = state
            .call(
                "FPDFPageObj_GetFillColor",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of5(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(ptr_r),
                    &Self::js_value_from_offset(ptr_g),
                    &Self::js_value_from_offset(ptr_b),
                    &Self::js_value_from_offset(ptr_a),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *R = state
                    .copy_bytes_from_pdfium(ptr_r, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);

                *G = state
                    .copy_bytes_from_pdfium(ptr_g, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);

                *B = state
                    .copy_bytes_from_pdfium(ptr_b, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);

                *A = state
                    .copy_bytes_from_pdfium(ptr_a, len)
                    .try_into()
                    .map(u32::from_le_bytes)
                    .unwrap_or(0);
            }
        }

        state.free(ptr_r);
        state.free(ptr_g);
        state.free(ptr_b);
        state.free(ptr_a);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashPhase()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let len = size_of::<c_float>();

        let ptr_phase = state.malloc(len);

        let result = state
            .call(
                "FPDFPageObj_GetDashPhase",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(ptr_phase),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            unsafe {
                *phase = state
                    .copy_bytes_from_pdfium(ptr_phase, len)
                    .try_into()
                    .map(f32::from_le_bytes)
                    .unwrap_or(0_f32);
            }
        }

        state.free(ptr_phase);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetDashPhase()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_SetDashPhase",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of2(
                    &Self::js_value_from_object(page_object),
                    &JsValue::from(phase),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashCount()");

        PdfiumRenderWasmState::lock()
            .call(
                "FPDFPageObj_GetDashCount",
                JsFunctionArgumentType::Number,
                Some(vec![JsFunctionArgumentType::Pointer]),
                Some(&JsValue::from(Array::of1(&Self::js_value_from_object(
                    page_object,
                )))),
            )
            .as_f64()
            .unwrap() as c_int
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashArray()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_len = size_of::<c_float>() * dash_count as usize;

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashArray(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_len);

        let buffer_ptr = state.malloc(buffer_len);

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashArray(): calling FPDFPageObj_GetDashArray()"
        );

        let result = state
            .call(
                "FPDFPageObj_GetDashArray",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(dash_count as f64),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        if self.is_true(result) {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashArray(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_len,
                buffer_ptr as usize
            );

            unsafe {
                dash_array.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_len)
                        .as_ptr() as *mut c_float,
                    buffer_len,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashArray(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_GetDashArray(): leaving");

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const c_float,
        dash_count: size_t,
        phase: c_float,
    ) -> FPDF_BOOL {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFPageObj_SetDashArray()");

        let state = PdfiumRenderWasmState::lock();

        let buffer_ptr = state
            .copy_ptr_with_len_to_pdfium(dash_array, size_of::<c_float>() * dash_count as usize);

        let result = state
            .call(
                "FPDFPageObj_SetDashArray",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of4(
                    &Self::js_value_from_object(page_object),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(dash_count as f64),
                    &JsValue::from(phase),
                ))),
            )
            .as_f64()
            .unwrap() as FPDF_BOOL;

        state.free(buffer_ptr);

        result
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFFont_GetFontName()");

        // Pdfium cannot access a pointer location in our own WASM heap. Instead, create
        // an empty buffer corresponding to the phase pointer in Pdfium's heap, call the given
        // Pdfium function, and copy the buffer back from Pdfium's heap into the given pointer.

        let state = PdfiumRenderWasmState::lock();

        let buffer_length = length as usize;

        let buffer_ptr = if buffer_length > 0 {
            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFFont_GetFontName(): allocating buffer of {} bytes in Pdfium's WASM heap", buffer_length);

            state.malloc(buffer_length)
        } else {
            0
        };

        log::debug!(
            "pdfium-render::PdfiumLibraryBindings::FPDFFont_GetFontName(): calling FPDFFont_GetFontName()"
        );

        let result = state
            .call(
                "FPDFFont_GetFontName",
                JsFunctionArgumentType::Number,
                Some(vec![
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Pointer,
                    JsFunctionArgumentType::Number,
                ]),
                Some(&JsValue::from(Array::of3(
                    &Self::js_value_from_font(font),
                    &Self::js_value_from_offset(buffer_ptr),
                    &JsValue::from_f64(buffer_length as f64),
                ))),
            )
            .as_f64()
            .unwrap() as c_ulong;

        if result <= length && result > 0 {
            log::debug!(
                "pdfium-render::PdfiumLibraryBindings::FPDFFont_GetFontName(): copying {} bytes from Pdfium's WASM heap into local buffer at offset {}",
                buffer_length,
                buffer as usize
            );

            unsafe {
                buffer.copy_from(
                    state
                        .copy_bytes_from_pdfium(buffer_ptr, buffer_length)
                        .as_ptr() as *mut c_char,
                    buffer_length,
                );
            }

            log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFFont_GetFontName(): freeing buffer in Pdfium's WASM heap");

            state.free(buffer_ptr);
        }

        log::debug!("pdfium-render::PdfiumLibraryBindings::FPDFFont_GetFontName(): leaving");

        result
    }
}
