//! Defines the [PdfiumLibraryBindings] trait, which exposes the raw FPDF_* functions
//! exported by the Pdfium library.

use crate::bindgen::{
    FPDF_BITMAP, FPDF_DOCUMENT, FPDF_DWORD, FPDF_FORMFILLINFO, FPDF_FORMHANDLE, FPDF_PAGE,
};
use crate::error::PdfiumInternalError;
use std::ffi::c_void;
use std::os::raw::{c_int, c_uchar, c_ulong};

/// Platform-independent function bindings to an external Pdfium library.
/// On most platforms this will be an external shared library loaded dynamically
/// at runtime, either bundled alongside your compiled Rust application or provided as a system
/// library by the platform. On WASM, this will be a set of Javascript functions exposed by a
/// separate WASM module that is imported into the same browser context.
///
/// Note that the [PdfiumLibraryBindings::FPDF_LoadDocument()] function is not available when
/// compiling to WASM. Either embed the target PDF document directly using the [include_bytes!()]
/// macro, or use Javascript's `fetch()` API to retrieve the bytes of the target document over
/// the network, then load those bytes into Pdfium using the [PdfiumLibraryBindings::FPDF_LoadMemDocument()] function.
pub trait PdfiumLibraryBindings {
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong;

    /// This function is not available when compiling to WASM. Either embed the target PDF document
    /// directly using the [include_bytes!()] macro, or use Javascript's `fetch()` API to retrieve
    /// the bytes of the target document over the network, then load those bytes into Pdfium using
    /// the [PdfiumLibraryBindings::FPDF_LoadMemDocument()] function.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT);

    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> bool;

    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE;

    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE);

    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> f32;

    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> f32;

    #[allow(non_snake_case)]
    fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int);

    #[allow(non_snake_case)]
    fn FPDFBitmap_CreateEx(
        &self,
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP;

    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP);

    #[allow(non_snake_case)]
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    );

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
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
    );

    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE;

    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE);

    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    );

    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar);

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
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
    );

    /// Retrieves the error code of the last error, if any, recorded by the external
    /// Pdfium library and maps it to a [PdfiumInternalError] enum value.
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
