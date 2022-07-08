//! Defines the [PdfiumError] enum, used to wrap Pdfium errors as `Err` values.

use crate::bindgen::{
    FPDF_ERR_FILE, FPDF_ERR_FORMAT, FPDF_ERR_PAGE, FPDF_ERR_PASSWORD, FPDF_ERR_SECURITY,
    FPDF_ERR_UNKNOWN,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

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

    UnrecognizedPath,
    PageIndexOutOfBounds,
    UnknownBitmapFormat,
    UnknownBitmapRotation,
    UnknownFormType,
    UnknownFormFieldType,
    UnknownActionType,
    PageObjectIndexOutOfBounds,
    PageObjectNotAttachedToPage,
    PageObjectAlreadyAttachedToDifferentPage,
    PageAnnotationIndexOutOfBounds,
    PageFlattenFailure,
    UnknownPdfPageObjectType,
    UnknownPdfPageTextRenderMode,
    UnknownPdfPagePathFillMode,
    UnknownPdfAnnotationType,
    UnknownPdfSecurityHandlerRevision,
    UnsupportedPdfPageObjectType,
    TextSegmentIndexOutOfBounds,
    CharIndexOutOfBounds,
    NoCharsInPageObject,
    ImageObjectFilterIndexOutOfBounds,
    ImageObjectFilterIndexInBoundsButFilterUndefined,
    UnknownPdfColorSpace,

    /// Two data buffers are expected to have the same size, but they do not.
    DataBufferLengthMismatch,

    /// The setting cannot be returned because this `PdfPageGroupObject` is empty.
    EmptyPageObjectGroup,

    /// A call to a internal Pdfium `FPDF_*` function returned a value indicating failure.
    ///
    /// For Pdfium functions that return enumerations, this means the function returned
    /// a value of -1 rather than a valid enumeration constant.
    ///
    /// For Pdfium functions that return C-style boolean integers, this means that the function
    /// returned a value other than `PdfiumLibraryBindings::TRUE`.
    PdfiumFunctionReturnValueIndicatedFailure,

    /// A call to a Pdfium function that returns a standard 8-bit color component value
    /// (for example, `FPDFPageObj_GetStrokeColor()` and `FPDFPageObj_GetStrokeColor()`)
    /// successfully returned a value, but the value could not be converted from a c_int
    /// to a standard Rust u8.
    UnableToConvertPdfiumColorValueToRustu8(std::num::TryFromIntError),

    /// The browser's built-in `Window` object could not be retrieved.
    WebSysWindowObjectNotAvailable,

    /// An error was returned when attempting to use the browser's built-in `fetch()` API.
    #[cfg(target_arch = "wasm32")]
    WebSysFetchError(JsValue),

    /// An invalid Response object was returned when attempting to use the browser's built-in `fetch()` API.
    #[cfg(target_arch = "wasm32")]
    WebSysInvalidResponseError,

    /// An error was returned when attempting to construct a `Blob` object from a byte buffer.
    #[cfg(target_arch = "wasm32")]
    JsSysErrorConstructingBlobFromBytes,

    /// An error occurred when attempting to retrieve the function table for the compiled
    /// Pdfium WASM module.
    #[cfg(target_arch = "wasm32")]
    JsSysErrorRetrievingFunctionTable(JsValue),

    /// An error occurred when attempting to retrieve an exported function from
    /// `pdfium-render`'s WASM module.
    #[cfg(target_arch = "wasm32")]
    JsSysErrorRetrievingFunction(JsValue),

    /// An error occurred when attempting to update an entry in Pdfium's WASM function table.
    #[cfg(target_arch = "wasm32")]
    JsSysErrorPatchingFunctionTable(JsValue),

    /// No previously cached function was available for a WASM function table restore operation.
    ///
    /// This error should never occur; if it does, it indicates a programming error in pdfium-render.
    /// Please file an issue: https://github.com/ajrcarey/pdfium-render/issues
    #[cfg(target_arch = "wasm32")]
    NoPreviouslyCachedFunctionSet,

    /// An error occurred during an image processing operation.
    ImageError,

    /// An I/O error occurred during a Pdfium file operation.
    IoError(std::io::Error),

    /// A wrapped internal library error from Pdfium's `FPDF_ERR_*` constant values.
    PdfiumLibraryInternalError(PdfiumInternalError),
}
