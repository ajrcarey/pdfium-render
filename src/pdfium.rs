//! Defines the [Pdfium] struct, a high-level idiomatic Rust wrapper around Pdfium.

use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};

#[cfg(all(not(target_arch = "wasm32"), not(feature = "static")))]
use std::ffi::OsString;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "static")))]
use libloading::Library;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "static"))]
use crate::native::NativePdfiumBindings;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "static")]
use crate::linked::StaticPdfiumBindings;

#[cfg(target_arch = "wasm32")]
use crate::wasm::WasmPdfiumBindings;

#[cfg(target_arch = "wasm32")]
use crate::wasm::PdfiumRenderWasmState;

/// A high-level idiomatic Rust wrapper around Pdfium, the C++ PDF library used by
/// the Google Chromium project.
pub struct Pdfium {
    bindings: Box<dyn PdfiumLibraryBindings>,
}

impl Pdfium {
    /// Binds to a Pdfium library that was statically linked into the currently running
    /// executable, returning a new PdfiumLibraryBindings object that contains bindings to the
    /// functions exposed by the library. The application will immediately crash if Pdfium
    /// was not correctly statically linked into the executable at compile time.
    ///
    /// This function is only available when the `static` feature is enabled.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "static")]
    #[inline]
    pub fn bind_to_statically_linked_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>
    {
        Ok(Box::new(StaticPdfiumBindings {}))
    }

    /// Initializes the external Pdfium library, loading it from the system libraries.
    /// Returns a new PdfiumLibraryBindings object that contains bindings to the functions exposed
    /// by the library, or an error if the library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    #[inline]
    pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        let bindings = NativePdfiumBindings::new(
            unsafe { Library::new(Self::pdfium_platform_library_name()) }
                .map_err(PdfiumError::LoadLibraryError)?,
        )
        .map_err(PdfiumError::LoadLibraryError)?;

        Ok(Box::new(bindings))
    }

    /// Initializes the external Pdfium library, binding to an external WASM module.
    /// Returns a new PdfiumLibraryBindings object that contains bindings to the functions exposed
    /// by the library, or an error if the library is not available.
    ///
    /// It is essential that the exported `initialize_pdfium_render()` function be called
    /// from Javascript _before_ calling this function from within your Rust code. For an example, see:
    /// <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        if PdfiumRenderWasmState::lock().is_ready() {
            Ok(Box::new(WasmPdfiumBindings::new()))
        } else {
            Err(PdfiumError::PdfiumWASMModuleNotConfigured)
        }
    }

    /// Initializes the external pdfium library, loading it from the given path.
    /// Returns a new PdfiumLibraryBindings object that contains bindings to the functions
    /// exposed by the library, or an error if the library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    #[inline]
    pub fn bind_to_library(
        path: impl ToString,
    ) -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        let bindings = NativePdfiumBindings::new(
            unsafe { Library::new(OsString::from(path.to_string())) }
                .map_err(PdfiumError::LoadLibraryError)?,
        )
        .map_err(PdfiumError::LoadLibraryError)?;

        Ok(Box::new(bindings))
    }

    /// Returns the name of the external Pdfium library on the currently running platform.
    /// On Linux and Android, this will be libpdfium.so or similar; on Windows, this will
    /// be pdfium.dll or similar; on MacOS, this will be libpdfium.dylib or similar.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    #[inline]
    pub fn pdfium_platform_library_name() -> OsString {
        libloading::library_filename("pdfium")
    }

    /// Returns the name of the external Pdfium library on the currently running platform,
    /// prefixed with the given path string.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    #[inline]
    pub fn pdfium_platform_library_name_at_path(path: impl ToString) -> String {
        let mut path = path.to_string();

        path.push_str(Pdfium::pdfium_platform_library_name().to_str().unwrap());

        path
    }

    /// Creates a new Pdfium object from the given external pdfium library bindings.
    #[inline]
    pub fn new(bindings: Box<dyn PdfiumLibraryBindings>) -> Self {
        bindings.FPDF_InitLibrary();

        Self { bindings }
    }

    /// Attempts to open a PdfDocument from the given file path.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    ///
    /// This function is not available when compiling to WASM. Either embed the bytes of
    /// the target PDF document directly into the compiled WASM module using the
    /// `include_bytes!()` macro, or use Javascript's `fetch()` API to retrieve the bytes
    /// of the target document over the network, then load those bytes into Pdfium using
    /// the `load_pdf_from_bytes()` function.
    #[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(feature = "static")]
    #[inline]
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap())
    }

    #[cfg(not(feature = "static"))]
    #[inline]
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_system_library().unwrap())
    }
}
