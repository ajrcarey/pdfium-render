use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::{PdfiumError, PdfiumInternalError};

#[cfg(not(target_arch = "wasm32"))]
use std::ffi::OsString;

#[cfg(not(target_arch = "wasm32"))]
use libloading::Library;

#[cfg(not(target_arch = "wasm32"))]
use crate::native::NativePdfiumBindings;

#[cfg(target_arch = "wasm32")]
use crate::wasm::WasmPdfiumBindings;

/// A high-level idiomatic Rust wrapper around Pdfium, the C++ PDF library used by
/// the Google Chromium project.
pub struct Pdfium {
    bindings: Box<dyn PdfiumLibraryBindings>,
}

impl Pdfium {
    /// Initializes the external pdfium library, loading it from the system libraries.
    /// Returns a new PdfiumLibraryBindings object that contains bindings to the functions exposed
    /// by the library, or an error if the library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn bind_to_system_library() -> Result<impl PdfiumLibraryBindings, PdfiumError> {
        let bindings = NativePdfiumBindings::new(
            unsafe { Library::new(Self::pdfium_platform_library_name()) }
                .map_err(PdfiumError::LoadLibraryError)?,
        )
        .map_err(PdfiumError::LoadLibraryError)?;

        Ok(bindings)
    }

    /// Binds to the external pdfium WASM module. The pdfium module must already be
    /// loaded and present in the browser context for binding to be successful.
    /// Returns a new PdfiumLibraryBindings object that contains bindings to the
    /// functions exposed by the pdfium module, or an error if the library could not be loaded.
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn bind_to_system_library() -> Result<impl PdfiumLibraryBindings, PdfiumError> {
        Ok(WasmPdfiumBindings::new())
    }

    /// Initializes the external pdfium library, loading it from the given path.
    /// Returns a new PdfiumLibraryBindings object that contains bindings to the functions
    /// exposed by the library, or an error if the library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn bind_to_library(path: impl ToString) -> Result<impl PdfiumLibraryBindings, PdfiumError> {
        let bindings = NativePdfiumBindings::new(
            unsafe { Library::new(OsString::from(path.to_string())) }
                .map_err(PdfiumError::LoadLibraryError)?,
        )
        .map_err(PdfiumError::LoadLibraryError)?;

        Ok(bindings)
    }

    /// Returns the name of the external Pdfium library on the currently running platform.
    /// On Linux and Android, this will be libpdfium.so or similar; on Windows, this will
    /// be pdfium.dll or similar; on MacOS, this will be libpdfium.dylib or similar.
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn pdfium_platform_library_name() -> OsString {
        libloading::library_filename("pdfium")
    }

    /// Creates a new Pdfium object from the given external pdfium library bindings.
    #[inline]
    pub fn new(bindings: impl PdfiumLibraryBindings + 'static) -> Self {
        bindings.FPDF_InitLibrary();

        Self {
            bindings: Box::new(bindings),
        }
    }

    /// Attempts to open a PdfDocument from the given file path.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    pub fn load_pdf_from_file(
        &self,
        path: &str,
        password: Option<&str>,
    ) -> Result<PdfDocument, PdfiumError> {
        self.pdfium_load_document_to_result(self.bindings.FPDF_LoadDocument(path, password))
    }

    /// Attempts to open a PdfDocument from the given byte buffer.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    pub fn load_pdf_from_bytes(
        &self,
        bytes: &[u8],
        password: Option<&str>,
    ) -> Result<PdfDocument, PdfiumError> {
        self.pdfium_load_document_to_result(self.bindings.FPDF_LoadMemDocument(bytes, password))
    }

    /// Returns a PdfDocument from the given FPDF_DOCUMENT handle, if possible.
    fn pdfium_load_document_to_result(
        &self,
        handle: crate::bindgen::FPDF_DOCUMENT,
    ) -> Result<PdfDocument, PdfiumError> {
        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfDocument::from_pdfium(handle, self.bindings.as_ref()))
        }
    }
}

impl Drop for Pdfium {
    /// Closes the external pdfium library, releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_DestroyLibrary();
    }
}

impl Default for Pdfium {
    #[inline]
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_system_library().unwrap())
    }
}
