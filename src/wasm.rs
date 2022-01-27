use crate::bindgen::{
    FPDF_BITMAP, FPDF_DOCUMENT, FPDF_DWORD, FPDF_FORMFILLINFO, FPDF_FORMHANDLE, FPDF_PAGE,
};
use crate::bindings::PdfiumLibraryBindings;
use std::ffi::{c_void, CString};
use std::mem::size_of;
use std::ops::DerefMut;
use std::os::raw::{c_int, c_uchar, c_ulong};
use wasm_bindgen::memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Bind to Javascript functions in the FPDF object namespace that themselves
    // wrap exported WASM functions from a separate WASM module containing the compiled
    // Pdfium library.

    // The Javascript function signatures are broadly similar to the native C
    // function signatures, with one important exception: we cannot pass pointers to
    // shared memory across WASM modules. We must pass our byte buffers directly;
    // these are exposed to Javascript as ArrayBuffer objects. This affects the
    // LoadMemDocument(), CloseDocument(), and Bitmap_GetBuffer() functions, for instance.

    // The Javascript function definitions matching these signatures can be found in
    // examples/pdfium_render.js.

    #[wasm_bindgen(js_namespace = FPDF)]
    fn InitLibrary();

    #[wasm_bindgen(js_namespace = FPDF)]
    fn DestroyLibrary();

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetLastError() -> c_ulong;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn LoadDocument(file_path: &str, password: &str) -> FPDF_DOCUMENT;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn LoadMemDocument(bytes: &[u8], password: &str) -> FPDF_DOCUMENT;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn CloseDocument(document: FPDF_DOCUMENT);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetFileVersion(document: FPDF_DOCUMENT) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetFormType(document: FPDF_DOCUMENT) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetMetaText(
        memory: JsValue,
        document: FPDF_DOCUMENT,
        tag: Vec<u8>,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageCount(document: FPDF_DOCUMENT) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn LoadPage(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn ClosePage(page: FPDF_PAGE);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageLabel(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn PageGetRotation(page: FPDF_PAGE) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn PageSetRotation(page: FPDF_PAGE, rotate: c_int);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageWidthF(page: FPDF_PAGE) -> f32;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageHeightF(page: FPDF_PAGE) -> f32;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_CreateEx(
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_Destroy(bitmap: FPDF_BITMAP);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_FillRect(
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    );

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetBuffer(bitmap: FPDF_BITMAP) -> Vec<u8>;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetWidth(bitmap: FPDF_BITMAP) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetHeight(bitmap: FPDF_BITMAP) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetStride(bitmap: FPDF_BITMAP) -> c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn RenderPageBitmap(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    );

    #[wasm_bindgen(js_namespace = FPDF)]
    fn DOC_InitFormFillEnvironment(
        memory: JsValue,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
        form_info_length: usize,
    ) -> FPDF_FORMHANDLE;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn DOC_ExitFormFillEnvironment(handle: FPDF_FORMHANDLE);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn SetFormFieldHighlightColor(handle: FPDF_FORMHANDLE, field_type: c_int, color: FPDF_DWORD);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn SetFormFieldHighlightAlpha(handle: FPDF_FORMHANDLE, alpha: c_uchar);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn FFLDraw(
        handle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    );
}

pub(crate) struct WasmPdfiumBindings {}

impl WasmPdfiumBindings {
    #[inline]
    pub(crate) fn new() -> Self {
        WasmPdfiumBindings {}
    }
}

impl PdfiumLibraryBindings for WasmPdfiumBindings {
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self) {
        InitLibrary();
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self) {
        DestroyLibrary();
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> ::std::os::raw::c_ulong {
        GetLastError()
    }

    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, _file_path: &str, _password: Option<&str>) -> FPDF_DOCUMENT {
        // FPDF_LoadDocument is not available on WASM. When compiling to WASM,
        // this function definition in the PdfiumLibraryBindings trait will be
        // entirely omitted, so calling code that attempts to call FPDF_LoadDocument()
        // will fail at compile-time, not run-time.

        unimplemented!()
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        LoadMemDocument(bytes, password.unwrap_or(""))
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        CloseDocument(document);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> bool {
        // The GetFileVersion() Javascript function returns the file version directly,
        // or a -1 if the function failed.

        let result = GetFileVersion(doc);

        if result < 0 {
            false
        } else {
            unsafe {
                *fileVersion = result;
            }

            true
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int {
        GetFormType(document)
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
        let c_tag = CString::new(tag).unwrap();

        // We pass through the CString as a byte array to our Javascript wrapper function.
        // The Javascript function will then copy it into the Pdfium WASM module's heap
        // and send a pointer through to Pdfium as it expects.

        GetMetaText(
            memory(),
            document,
            c_tag.into_bytes_with_nul(),
            buffer,
            buflen,
        )
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int {
        GetPageCount(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE {
        LoadPage(document, page_index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        ClosePage(page);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> f32 {
        GetPageWidthF(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> f32 {
        GetPageHeightF(page)
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
        GetPageLabel(document, page_index, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int {
        PageGetRotation(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int) {
        PageSetRotation(page, rotate);
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
        Bitmap_CreateEx(width, height, format, first_scan, stride)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        Bitmap_Destroy(bitmap);
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
        Bitmap_FillRect(bitmap, left, top, width, height, color);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void {
        Bitmap_GetBuffer(bitmap).as_ptr() as *mut c_void
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int {
        Bitmap_GetWidth(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int {
        Bitmap_GetHeight(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int {
        Bitmap_GetStride(bitmap)
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
        RenderPageBitmap(
            bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        );
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        _form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        let mut form_info = Box::new(FPDF_FORMFILLINFO {
            version: 2,
            Release: None,
            FFI_Invalidate: None,
            FFI_OutputSelectedRect: None,
            FFI_SetCursor: None,
            FFI_SetTimer: None,
            FFI_KillTimer: None,
            FFI_GetLocalTime: None,
            FFI_OnChange: None,
            FFI_GetPage: None,
            FFI_GetCurrentPage: None,
            FFI_GetRotation: None,
            FFI_ExecuteNamedAction: None,
            FFI_SetTextFieldFocus: None,
            FFI_DoURIAction: None,
            FFI_DoGoToAction: None,
            m_pJsPlatform: std::ptr::null_mut(),
            xfa_disabled: 0,
            FFI_DisplayCaret: None,
            FFI_GetCurrentPageIndex: None,
            FFI_SetCurrentPage: None,
            FFI_GotoURL: None,
            FFI_GetPageViewRect: None,
            FFI_PageEvent: None,
            FFI_PopupMenu: None,
            FFI_OpenFile: None,
            FFI_EmailTo: None,
            FFI_UploadTo: None,
            FFI_GetPlatform: None,
            FFI_GetLanguage: None,
            FFI_DownloadFromURL: None,
            FFI_PostRequestURL: None,
            FFI_PutRequestURL: None,
            FFI_OnFocusChange: None,
            FFI_DoURIActionWithKeyboardModifier: None,
        });

        DOC_InitFormFillEnvironment(
            memory(),
            document,
            form_info.deref_mut(),
            size_of::<FPDF_FORMFILLINFO>(),
        )
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE) {
        DOC_ExitFormFillEnvironment(handle)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    ) {
        SetFormFieldHighlightColor(handle, field_type, color)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar) {
        SetFormFieldHighlightAlpha(handle, alpha)
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
        FFLDraw(
            handle, bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        )
    }
}

impl Default for WasmPdfiumBindings {
    #[inline]
    fn default() -> Self {
        WasmPdfiumBindings::new()
    }
}
