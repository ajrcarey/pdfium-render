//! Defines the [PdfiumError] enum, used to wrap Pdfium errors as `Result` values.

use crate::bindgen::{
    FPDF_ERR_FILE, FPDF_ERR_FORMAT, FPDF_ERR_PAGE, FPDF_ERR_PASSWORD, FPDF_ERR_SECURITY,
    FPDF_ERR_UNKNOWN,
};

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
    DynamicLibraryLoadingNotSupportedOnWASM,
    #[cfg(not(target_arch = "wasm32"))]
    LoadLibraryError(libloading::Error),
    PageIndexOutOfBounds,
    UnknownBitmapFormat,
    UnknownBitmapRotation,
    UnknownFormType,
    UnknownFormFieldType,
    PdfiumLibraryInternalError(PdfiumInternalError),
}
