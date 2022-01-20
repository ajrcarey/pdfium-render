//! Defines the [PdfiumLibraryBindings] trait, which exposes the raw FPDF_* functions
//! exported by the Pdfium library.

use crate::bindgen::{FPDF_BITMAP, FPDF_DOCUMENT, FPDF_DWORD, FPDF_FORMFILLINFO, FPDF_FORMHANDLE, FPDF_PAGE};
use crate::PdfiumInternalError;

/// Platform-independent function bindings to an external Pdfium library.
/// On most platforms this will be an external shared library loaded dynamically
/// at runtime, either bundled alongside the compiled Rust application or provided
/// by the platform; on WASM, this will be a set of Javascript functions exposed by a
/// separate WASM import into the same browser context.
///
/// Note that the `FPDF_LoadDocument()` function is not available when compiling to WASM.
/// Either embed the target PDF document directly using the `include_bytes!()` macro,
/// or use Javascript's `fetch()` API to retrieve the bytes of the target document over
/// the network, then load those bytes into Pdfium using the `FPDF_LoadMemDocument()` function.
pub trait PdfiumLibraryBindings {
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> ::std::os::raw::c_ulong;

    #[cfg(not(target_arch = "wasm32"))]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT);

    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> ::std::os::raw::c_int;

    #[allow(non_snake_case)]
    fn FPDF_LoadPage(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::std::os::raw::c_int,
    ) -> FPDF_PAGE;

    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE);

    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> f32;

    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> f32;

    #[allow(non_snake_case)]
    fn FPDFBitmap_CreateEx(
        &self,
        width: i32,
        height: i32,
        format: i32,
        first_scan: *mut ::std::os::raw::c_void,
        stride: i32,
    ) -> FPDF_BITMAP;

    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP);

    #[allow(non_snake_case)]
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: FPDF_DWORD,
    );

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut ::std::os::raw::c_void;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
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
    );

    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE;

    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: ::std::os::raw::c_int,
        color: ::std::os::raw::c_ulong,
    );

    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(
        &self,
        handle: FPDF_FORMHANDLE,
        alpha: ::std::os::raw::c_uchar,
    );

    #[allow(non_snake_case)]
    fn FPDF_FFLDraw(
        &self,
        handle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::std::os::raw::c_int,
        start_y: ::std::os::raw::c_int,
        size_x: ::std::os::raw::c_int,
        size_y: ::std::os::raw::c_int,
        rotate: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    );

    /// Retrieves the error code of the last error, if any, recorded by the external
    /// libpdfium provider and maps it to a PdfiumInternalError enum value.
    #[inline]
    fn get_pdfium_last_error(&self) -> Option<PdfiumInternalError> {
        let result = self.FPDF_GetLastError() as u32;

        match result {
            crate::bindgen::FPDF_ERR_SUCCESS => None,
            crate::bindgen::FPDF_ERR_UNKNOWN => Some(PdfiumInternalError::Unknown),
            crate::bindgen::FPDF_ERR_FILE => Some(PdfiumInternalError::FileError),
            crate::bindgen::FPDF_ERR_FORMAT => Some(PdfiumInternalError::FormatError),
            crate::bindgen::FPDF_ERR_PASSWORD => Some(PdfiumInternalError::PasswordError),
            crate::bindgen::FPDF_ERR_SECURITY => Some(PdfiumInternalError::SecurityError),
            crate::bindgen::FPDF_ERR_PAGE => Some(PdfiumInternalError::PageError),
            _ => Some(PdfiumInternalError::Unknown),
        }
    }

}
