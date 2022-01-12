use crate::bindgen::{FPDF_BITMAP, FPDF_DOCUMENT, FPDF_DWORD, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Bind to Javascript functions in the FPDF object namespace that themselves
    // wrap exported WASM functions from a separate WASM module containing the compiled
    // PDFium library.

    // The Javascript function signatures are broadly similar to the native C
    // function signatures, with one important exception: we cannot pass pointers to
    // shared memory across WASM modules. We must pass our byte buffers directly;
    // these are exposed to Javascript as ArrayBuffer objects. This affects the
    // LoadMemDocument() function.

    #[wasm_bindgen(js_namespace = FPDF)]
    fn InitLibrary();

    #[wasm_bindgen(js_namespace = FPDF)]
    fn DestroyLibrary();

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetLastError() -> ::std::os::raw::c_ulong;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn LoadDocument(file_path: &str, password: &str) -> FPDF_DOCUMENT;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn LoadMemDocument(bytes: &[u8], password: &str) -> FPDF_DOCUMENT;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn CloseDocument(document: FPDF_DOCUMENT);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageCount(document: FPDF_DOCUMENT) -> ::std::os::raw::c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn LoadPage(document: FPDF_DOCUMENT, page_index: ::std::os::raw::c_int) -> FPDF_PAGE;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn ClosePage(page: FPDF_PAGE);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageWidthF(page: FPDF_PAGE) -> f32;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn GetPageHeightF(page: FPDF_PAGE) -> f32;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_CreateEx(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        first_scan: *mut ::std::os::raw::c_void,
        stride: ::std::os::raw::c_int,
    ) -> FPDF_BITMAP;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_Destroy(bitmap: FPDF_BITMAP);

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_FillRect(
        bitmap: FPDF_BITMAP,
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: FPDF_DWORD,
    );

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetBuffer(bitmap: FPDF_BITMAP) -> Vec<u8>;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetWidth(bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetHeight(bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn Bitmap_GetStride(bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int;

    #[wasm_bindgen(js_namespace = FPDF)]
    fn RenderPageBitmap(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::std::os::raw::c_int,
        start_y: ::std::os::raw::c_int,
        size_x: ::std::os::raw::c_int,
        size_y: ::std::os::raw::c_int,
        rotate: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
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
        // entirely omitted, so calling code that attempts to call FPDF_LoadDocument
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
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> ::std::os::raw::c_int {
        GetPageCount(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::std::os::raw::c_int,
    ) -> FPDF_PAGE {
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
    fn FPDFBitmap_CreateEx(
        &self,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        first_scan: *mut std::os::raw::c_void,
        stride: ::std::os::raw::c_int,
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
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: FPDF_DWORD,
    ) {
        Bitmap_FillRect(bitmap, left, top, width, height, color);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut ::std::os::raw::c_void {
        Bitmap_GetBuffer(bitmap).as_ptr() as *mut ::std::os::raw::c_void
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int {
        Bitmap_GetWidth(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int {
        Bitmap_GetHeight(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int {
        Bitmap_GetStride(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmap(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::std::os::raw::c_int,
        start_y: ::std::os::raw::c_int,
        size_x: ::std::os::raw::c_int,
        size_y: ::std::os::raw::c_int,
        rotate: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) {
        RenderPageBitmap(
            bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        );
    }
}

impl Default for WasmPdfiumBindings {
    #[inline]
    fn default() -> Self {
        WasmPdfiumBindings::new()
    }
}
