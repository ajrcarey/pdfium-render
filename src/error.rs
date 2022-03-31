//! Defines the [PdfiumError] enum, used to wrap Pdfium errors as `Err` values.

use crate::bindgen::{
    FPDF_ERR_FILE, FPDF_ERR_FORMAT, FPDF_ERR_PAGE, FPDF_ERR_PASSWORD, FPDF_ERR_SECURITY,
    FPDF_ERR_UNKNOWN,
};

/// A wrapped internal library error from Pdfium's FPDF_ERR_* constant values.
#[derive(Debug)]
pub enum PdfiumInternalError {
    Unknown = FPDF_ERR_UNKNOWN as isize,
    FileError = FPDF_ERR_FILE as isize,
    FormatError = FPDF_ERR_FORMAT as isize,
    PasswordError = FPDF_ERR_PASSWORD as isize,
    SecurityError = FPDF_ERR_SECURITY as isize,
    PageError = FPDF_ERR_PAGE as isize,
}

#[derive(Debug)]
pub enum PdfiumError {
    /// The Pdfium WASM module has not been configured.
    /// It is essential that the exported `initialize_pdfium_render()` function be called
    /// from Javascript _before_ calling any `pdfium-render` function from within your Rust code.
    /// See: <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
    #[cfg(target_arch = "wasm32")]
    PdfiumWASMModuleNotConfigured,

    /// The external Pdfium library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    LoadLibraryError(libloading::Error),

    PageIndexOutOfBounds,
    UnknownBitmapFormat,
    UnknownBitmapRotation,
    UnknownFormType,
    UnknownFormFieldType,
    UnknownActionType,
    PageObjectIndexOutOfBounds,
    PageAnnotationIndexOutOfBounds,
    UnknownPdfPageObjectType,
    UnknownPdfPageTextRenderMode,
    UnknownPdfAnnotationType,

    /// A wrapped internal library error from Pdfium's FPDF_ERR_* constant values.
    PdfiumLibraryInternalError(PdfiumInternalError),
}
