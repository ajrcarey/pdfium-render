//! Defines the [Pdfium] struct, a high-level idiomatic Rust wrapper around Pdfium.

use crate::bindings::PdfiumLibraryBindings;
use crate::document::{PdfDocument, PdfDocumentVersion};
use crate::error::{PdfiumError, PdfiumInternalError};

#[cfg(all(not(target_arch = "wasm32"), not(feature = "static")))]
use std::ffi::OsString;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "static")))]
use libloading::Library;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "static")))]
use crate::native::DynamicPdfiumBindings;

#[cfg(all(not(target_arch = "wasm32"), feature = "static"))]
use crate::linked::StaticPdfiumBindings;

#[cfg(not(target_arch = "wasm32"))]
use crate::utils::files::get_pdfium_file_accessor_from_reader;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;

#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Seek};

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use crate::wasm::{PdfiumRenderWasmState, WasmPdfiumBindings};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
use js_sys::{ArrayBuffer, Uint8Array};

#[cfg(target_arch = "wasm32")]
use web_sys::{window, Blob, Response};

#[cfg(feature = "thread_safe")]
use crate::thread_safe::ThreadSafePdfiumBindings;

/// A high-level idiomatic Rust wrapper around Pdfium, the C++ PDF library used by
/// the Google Chromium project.
pub struct Pdfium {
    bindings: Box<dyn PdfiumLibraryBindings>,
}

impl Pdfium {
    /// Binds to a Pdfium library that was statically linked into the currently running
    /// executable, returning a new [PdfiumLibraryBindings] object that contains bindings to the
    /// functions exposed by the library. The application will immediately crash if Pdfium
    /// was not correctly statically linked into the executable at compile time.
    ///
    /// This function is only available when this crate's `static` feature is enabled.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "static")]
    #[inline]
    pub fn bind_to_statically_linked_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>
    {
        let bindings = StaticPdfiumBindings::new();

        #[cfg(feature = "thread_safe")]
        let bindings = ThreadSafePdfiumBindings::new(bindings);

        Ok(Box::new(bindings))
    }

    /// Initializes the external Pdfium library, loading it from the system libraries.
    /// Returns a new [PdfiumLibraryBindings] object that contains bindings to the functions exposed
    /// by the library, or an error if the library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    #[inline]
    pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        let bindings = DynamicPdfiumBindings::new(
            unsafe { Library::new(Self::pdfium_platform_library_name()) }
                .map_err(PdfiumError::LoadLibraryError)?,
        )
        .map_err(PdfiumError::LoadLibraryError)?;

        #[cfg(feature = "thread_safe")]
        let bindings = ThreadSafePdfiumBindings::new(bindings);

        Ok(Box::new(bindings))
    }

    /// Initializes the external Pdfium library, binding to an external WASM module.
    /// Returns a new [PdfiumLibraryBindings] object that contains bindings to the functions exposed
    /// by the library, or an error if the library is not available.
    ///
    /// It is essential that the exported `initialize_pdfium_render()` function be called
    /// from Javascript _before_ calling this function from within your Rust code. For an example, see:
    /// <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        if PdfiumRenderWasmState::lock().is_ready() {
            let bindings = WasmPdfiumBindings::new();

            #[cfg(feature = "thread_safe")]
            let bindings = ThreadSafePdfiumBindings::new(bindings);

            Ok(Box::new(bindings))
        } else {
            Err(PdfiumError::PdfiumWASMModuleNotConfigured)
        }
    }

    /// Initializes the external pdfium library, loading it from the given path.
    /// Returns a new [PdfiumLibraryBindings] object that contains bindings to the functions
    /// exposed by the library, or an error if the library could not be loaded.
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    #[inline]
    pub fn bind_to_library(
        path: impl ToString,
    ) -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        let bindings = DynamicPdfiumBindings::new(
            unsafe { Library::new(OsString::from(path.to_string())) }
                .map_err(PdfiumError::LoadLibraryError)?,
        )
        .map_err(PdfiumError::LoadLibraryError)?;

        #[cfg(feature = "thread_safe")]
        let bindings = ThreadSafePdfiumBindings::new(bindings);

        Ok(Box::new(bindings))
    }

    /// Returns the name of the external Pdfium library on the currently running platform.
    /// On Linux and Android, this will be `libpdfium.so` or similar; on Windows, this will
    /// be `pdfium.dll` or similar; on MacOS, this will be `libpdfium.dylib` or similar.
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

    /// Creates a new [Pdfium] instance from the given external Pdfium library bindings.
    #[inline]
    pub fn new(bindings: Box<dyn PdfiumLibraryBindings>) -> Self {
        bindings.FPDF_InitLibrary();

        Self { bindings }
    }

    // TODO: AJRC - 17/9/22 - remove deprecated Pdfium::get_bindings() function in 0.9.0
    // as part of tracking issue https://github.com/ajrcarey/pdfium-render/issues/36
    /// Returns the [PdfiumLibraryBindings] wrapped by this instance of [Pdfium].
    #[deprecated(
        since = "0.7.18",
        note = "This function has been renamed. Use the Pdfium::bindings() function instead."
    )]
    #[doc(hidden)]
    #[inline]
    pub fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings.as_ref()
    }

    /// Returns the [PdfiumLibraryBindings] wrapped by this instance of [Pdfium].
    #[inline]
    pub fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings.as_ref()
    }

    /// Attempts to open a [PdfDocument] from the given byte buffer.
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    pub fn load_pdf_from_bytes(
        &self,
        bytes: &[u8],
        password: Option<&str>,
    ) -> Result<PdfDocument, PdfiumError> {
        self.pdfium_document_handle_to_result(self.bindings.FPDF_LoadMemDocument64(bytes, password))
    }

    /// Attempts to open a [PdfDocument] from the given file path.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading your PDF document data in WASM:
    /// * Use the `Pdfium::load_pdf_from_fetch()` function to download document data from a
    /// URL using the browser's built-in `fetch()` API. This function is only available when
    /// compiling to WASM.
    /// * Use the `Pdfium::load_pdf_from_blob()` function to load document data from a
    /// Javascript File or Blob object (such as a File object returned from an HTML
    /// `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use another method to retrieve the bytes of the target document over the network,
    /// then load those bytes into Pdfium using the [Pdfium::load_pdf_from_bytes()] function.
    /// * Embed the bytes of the target document directly into the compiled WASM module
    /// using the `include_bytes!()` macro.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_pdf_from_file(
        &self,
        path: &(impl AsRef<Path> + ?Sized),
        password: Option<&str>,
    ) -> Result<PdfDocument, PdfiumError> {
        self.load_pdf_from_reader(File::open(path).map_err(PdfiumError::IoError)?, password)
    }

    /// Attempts to open a [PdfDocument] from the given reader.
    ///
    /// Pdfium will only load the portions of the document it actually needs into memory.
    /// This is more efficient than loading the entire document into memory, especially when
    /// working with large documents, and allows for working with documents larger than the
    /// amount of available memory.
    ///
    /// Because Pdfium must know the total content length in advance prior to loading
    /// any portion of it, the given reader must implement the `Seek` trait as well as
    /// the `Read` trait.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading your PDF document data in WASM:
    /// * Use the `Pdfium::load_pdf_from_fetch()` function to download document data from a
    /// URL using the browser's built-in `fetch()` API. This function is only available when
    /// compiling to WASM.
    /// * Use the `Pdfium::load_pdf_from_blob()` function to load document data from a
    /// Javascript File or Blob object (such as a File object returned from an HTML
    /// `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use another method to retrieve the bytes of the target document over the network,
    /// then load those bytes into Pdfium using the [Pdfium::load_pdf_from_bytes()] function.
    /// * Embed the bytes of the target document directly into the compiled WASM module
    /// using the `include_bytes!()` macro.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_pdf_from_reader<R: Read + Seek + 'static>(
        &self,
        reader: R,
        password: Option<&str>,
    ) -> Result<PdfDocument, PdfiumError> {
        let mut reader = get_pdfium_file_accessor_from_reader(reader);

        self.pdfium_document_handle_to_result(
            self.bindings
                .FPDF_LoadCustomDocument(reader.as_fpdf_file_access_mut_ptr(), password),
        )
        .map(|mut document| {
            // Give the newly-created document ownership of the reader, so that Pdfium can continue
            // to read from it on an as-needed basis throughout the lifetime of the document.

            document.set_file_access_reader(reader);

            document
        })
    }

    /// Attempts to open a [PdfDocument] by loading document data from the given URL.
    /// The Javascript `fetch()` API is used to download data over the network.
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(target_arch = "wasm32")]
    pub async fn load_pdf_from_fetch<'a>(
        &'a self,
        url: impl ToString,
        password: Option<&str>,
    ) -> Result<PdfDocument<'a>, PdfiumError> {
        if let Some(window) = window() {
            let fetch_result = JsFuture::from(window.fetch_with_str(url.to_string().as_str()))
                .await
                .map_err(PdfiumError::WebSysFetchError)?;

            debug_assert!(fetch_result.is_instance_of::<Response>());

            let response: Response = fetch_result
                .dyn_into()
                .map_err(|_| PdfiumError::WebSysInvalidResponseError)?;

            let blob: Blob =
                JsFuture::from(response.blob().map_err(PdfiumError::WebSysFetchError)?)
                    .await
                    .map_err(PdfiumError::WebSysFetchError)?
                    .into();

            self.load_pdf_from_blob(blob, password).await
        } else {
            Err(PdfiumError::WebSysWindowObjectNotAvailable)
        }
    }

    /// Attempts to open a [PdfDocument] by loading document data from the given Blob.
    /// A File object returned from a FileList is a suitable Blob:
    ///
    /// ```
    /// <input id="filePicker" type="file">
    ///
    /// const file = document.getElementById('filePicker').files[0];
    /// ```
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(target_arch = "wasm32")]
    pub async fn load_pdf_from_blob<'a>(
        &'a self,
        blob: Blob,
        password: Option<&str>,
    ) -> Result<PdfDocument<'a>, PdfiumError> {
        let array_buffer: ArrayBuffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(PdfiumError::WebSysFetchError)?
            .into();

        let u8_array: Uint8Array = Uint8Array::new(&array_buffer);

        let bytes: Vec<u8> = u8_array.to_vec();

        self.load_pdf_from_bytes(bytes.as_slice(), password)
    }

    /// Creates a new, empty [PdfDocument] in memory.
    pub fn create_new_pdf(&self) -> Result<PdfDocument, PdfiumError> {
        self.pdfium_document_handle_to_result(self.bindings.FPDF_CreateNewDocument())
            .map(|mut document| {
                document.set_version(PdfDocumentVersion::DEFAULT_VERSION);

                document
            })
    }

    /// Returns a [PdfDocument] from the given `FPDF_DOCUMENT` handle, if possible.
    fn pdfium_document_handle_to_result(
        &self,
        handle: crate::bindgen::FPDF_DOCUMENT,
    ) -> Result<PdfDocument, PdfiumError> {
        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

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
    /// Closes the external Pdfium library, releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_DestroyLibrary();
    }
}

impl Default for Pdfium {
    /// Binds to a Pdfium library that was statically linked into the currently running
    /// executable by calling [Pdfium::bind_to_statically_linked_library()]. This function
    /// will panic if no statically linked Pdfium functions can be located.
    #[cfg(feature = "static")]
    #[inline]
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap())
    }

    /// Binds to an external Pdfium library, loading it from the system libraries,
    /// by calling [Pdfium::bind_to_system_library()]. This function will panic if no
    /// suitable system library can be loaded.
    #[cfg(not(feature = "static"))]
    #[inline]
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_system_library().unwrap())
    }
}
