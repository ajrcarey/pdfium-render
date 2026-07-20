//! Defines the [Pdfium] struct, a high-level idiomatic Rust wrapper around Pdfium.

use crate::bindgen::{
    FPDF_DOCUMENT, FPDF_ERR_FILE, FPDF_ERR_FORMAT, FPDF_ERR_PAGE, FPDF_ERR_PASSWORD,
    FPDF_ERR_SECURITY, FPDF_ERR_SUCCESS, FPDF_ERR_UNKNOWN,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::config::PdfiumLibraryConfig;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::{PdfDocument, PdfDocumentVersion};
use crate::pdf::font::provider::{PdfiumCustomFontProvider, PdfiumCustomFontProviderExt};
use once_cell::sync::OnceCell;
use std::fmt::{Debug, Formatter};
use std::pin::Pin;

#[cfg(feature = "thread_safe")]
use crate::bindings::thread_safe::ThreadSafePdfiumBindings;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "static")))]
use {
    crate::bindings::dynamic_bindings::DynamicPdfiumBindings, libloading::Library,
    std::ffi::OsString, std::path::PathBuf,
};

#[cfg(all(not(target_arch = "wasm32"), feature = "static"))]
use crate::bindings::static_bindings::StaticPdfiumBindings;

#[cfg(not(target_arch = "wasm32"))]
use {
    crate::bindgen::FPDF_SYSFONTINFO,
    crate::utils::files::get_pdfium_file_accessor_from_reader,
    std::fs::File,
    std::io::{Read, Seek},
    std::path::Path,
};

#[cfg(target_arch = "wasm32")]
use {
    crate::bindings::wasm_bindings::{PdfiumRenderWasmState, WasmPdfiumBindings},
    js_sys::{ArrayBuffer, Uint8Array},
    wasm_bindgen::JsCast,
    wasm_bindgen_futures::JsFuture,
    web_sys::{window, Blob, Response},
};

// The following dummy declaration is used only when running cargo doc.
// It allows documentation of WASM-specific functionality to be included
// in documentation generated on non-WASM targets.
#[cfg(doc)]
struct Blob;

// The first instantiation of a Pdfium object will promote a concrete PdfiumLibraryBindings
// trait implementation into a global static OnceCell. This allows for thread-safe,
// lifetime-free access to that PdfiumLibraryBindings instance from any object that
// implements the PdfiumLibraryBindingsAccessor trait.
static BINDINGS: OnceCell<Box<dyn PdfiumLibraryBindings>> = OnceCell::new();

// Pdfium exposes a non-reentrant C API: concurrent calls into the same library
// instance corrupt Pdfium's internal state. When the `thread_safe` feature is
// enabled a `Pdfium` may be shared across threads (it is marked Send + Sync), so
// every call that reaches into the bindings must be serialized process-wide.
//
// Serialization is compartmentalized inside the bindings layer. Under the
// `thread_safe` feature the concrete `PdfiumLibraryBindings` implementation is
// wrapped in a `ThreadSafePdfiumBindings` (see `crate::bindings::thread_safe`)
// before it is promoted into `BINDINGS`. That wrapper acquires a single
// process-wide mutex at the start of every FFI method and releases it as soon as
// the call returns, so no two calls into Pdfium can ever run concurrently. The
// wrapper stores no lock guard of its own, which is what keeps `Pdfium` — and the
// objects derived from it — soundly `Send + Sync`. High-level methods therefore
// hold no lock themselves: they simply call through `bindings()`, and each
// individual FFI call is serialized for them by the wrapper.

/// A trait implemented by every high-level type, giving lifetime-free access to
/// the process-wide [PdfiumLibraryBindings] promoted into a global on the first
/// [Pdfium] instantiation.
///
/// When the `thread_safe` feature is enabled the promoted bindings are a
/// [ThreadSafePdfiumBindings](crate::bindings::thread_safe::ThreadSafePdfiumBindings)
/// wrapper, so every FFI call made through the reference returned by
/// [PdfiumLibraryBindingsAccessor::bindings] is serialized process-wide
/// automatically; callers never have to take a lock themselves.
#[cfg(feature = "thread_safe")]
pub trait PdfiumLibraryBindingsAccessor<'a>: Send + Sync {
    fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        BINDINGS.wait().as_ref()
    }
}

#[cfg(not(feature = "thread_safe"))]
pub trait PdfiumLibraryBindingsAccessor<'a> {
    fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        BINDINGS.get().unwrap().as_ref()
    }
}

/// The trait bound for a reader passed to [Pdfium::load_pdf_from_reader].
///
/// Under the `thread_safe` feature the resulting [PdfDocument] is `Send` and
/// Pdfium may invoke the reader's callback from whichever thread later triggers a
/// lazy read, so the reader must also be `Send`. Without `thread_safe`, only
/// [Read] and [Seek] are required. This is a blanket trait implemented for every
/// type that satisfies the underlying bounds; you never name it directly.
#[cfg(all(not(target_arch = "wasm32"), feature = "thread_safe"))]
pub trait PdfiumReader: Read + Seek + Send {}
#[cfg(all(not(target_arch = "wasm32"), feature = "thread_safe"))]
impl<R: Read + Seek + Send> PdfiumReader for R {}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "thread_safe")))]
pub trait PdfiumReader: Read + Seek {}
#[cfg(all(not(target_arch = "wasm32"), not(feature = "thread_safe")))]
impl<R: Read + Seek> PdfiumReader for R {}

/// A high-level idiomatic Rust wrapper around Pdfium, the C++ PDF library used by
/// the Google Chromium project.
pub struct Pdfium {
    pub(crate) custom_font_provider: Option<Pin<Box<PdfiumCustomFontProviderExt>>>,

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) platform_default_font_provider: Option<*mut FPDF_SYSFONTINFO>,
}

impl Pdfium {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(any(doc, feature = "static"))]
    /// Binds to a Pdfium library that was statically linked into the currently running
    /// executable, returning a new [PdfiumLibraryBindings] object that contains bindings to the
    /// functions exposed by the library. The application will immediately crash if Pdfium
    /// was not correctly statically linked into the executable at compile time.
    ///
    /// This function is only available when this crate's `static` feature is enabled.
    #[inline]
    pub fn bind_to_statically_linked_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>
    {
        if BINDINGS.get().is_none() {
            let bindings = StaticPdfiumBindings::new();

            #[cfg(feature = "thread_safe")]
            let bindings = ThreadSafePdfiumBindings::new(bindings);

            Ok(Box::new(bindings))
        } else {
            Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    /// Initializes the external Pdfium library, loading it from the system libraries.
    /// Returns a new [PdfiumLibraryBindings] object that contains bindings to the functions exposed
    /// by the library, or an error if the library could not be loaded.
    #[inline]
    pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        if BINDINGS.get().is_none() {
            let bindings = DynamicPdfiumBindings::new(
                unsafe { Library::new(Self::pdfium_platform_library_name()) }
                    .map_err(PdfiumError::LoadLibraryError)?,
            )?;

            #[cfg(feature = "thread_safe")]
            let bindings = ThreadSafePdfiumBindings::new(bindings);

            Ok(Box::new(bindings))
        } else {
            Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized)
        }
    }

    #[cfg(target_arch = "wasm32")]
    /// Initializes the external Pdfium library, binding to an external WASM module.
    /// Returns a new [PdfiumLibraryBindings] object that contains bindings to the functions exposed
    /// by the library, or an error if the library is not available.
    ///
    /// It is essential that the exported `initialize_pdfium_render()` function be called
    /// from Javascript _before_ calling this function from within your Rust code. For an example, see:
    /// <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
    #[inline]
    pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        if BINDINGS.get().is_none() {
            if PdfiumRenderWasmState::lock().is_ready() {
                let bindings = WasmPdfiumBindings::new();

                #[cfg(feature = "thread_safe")]
                let bindings = ThreadSafePdfiumBindings::new(bindings);

                Ok(Box::new(bindings))
            } else {
                Err(PdfiumError::PdfiumWasmModuleNotInitialized)
            }
        } else {
            Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    /// Initializes the external pdfium library, loading it from the given path.
    /// Returns a new [PdfiumLibraryBindings] object that contains bindings to the functions
    /// exposed by the library, or an error if the library could not be loaded.
    #[inline]
    pub fn bind_to_library(
        path: impl AsRef<Path>,
    ) -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> {
        if BINDINGS.get().is_none() {
            let bindings = DynamicPdfiumBindings::new(
                unsafe { Library::new(path.as_ref().as_os_str()) }
                    .map_err(PdfiumError::LoadLibraryError)?,
            )?;

            #[cfg(feature = "thread_safe")]
            let bindings = ThreadSafePdfiumBindings::new(bindings);

            Ok(Box::new(bindings))
        } else {
            Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    /// Returns the name of the external Pdfium library on the currently running platform.
    /// On Linux and Android, this will be `libpdfium.so` or similar; on Windows, this will
    /// be `pdfium.dll` or similar; on MacOS, this will be `libpdfium.dylib` or similar.
    #[inline]
    pub fn pdfium_platform_library_name() -> OsString {
        libloading::library_filename("pdfium")
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static"))]
    /// Returns the name of the external Pdfium library on the currently running platform,
    /// prefixed with the given path string.
    #[inline]
    pub fn pdfium_platform_library_name_at_path(path: &(impl AsRef<Path> + ?Sized)) -> PathBuf {
        path.as_ref().join(Pdfium::pdfium_platform_library_name())
    }

    /// Creates a new [Pdfium] instance from the given external Pdfium library bindings.
    #[inline]
    pub fn new(bindings: Box<dyn PdfiumLibraryBindings>) -> Self {
        // Initialize the library and promote the bindings into the process-global
        // BINDINGS exactly once, even if several threads construct a Pdfium
        // concurrently. get_or_init runs its closure on the first caller and blocks
        // the rest until it completes; a losing thread reuses the installed
        // bindings and its own box is dropped without effect (the bindings Drop no
        // longer calls FPDF_DestroyLibrary, so it cannot tear down the shared
        // library the winner just initialized).
        BINDINGS.get_or_init(move || {
            unsafe {
                bindings.FPDF_InitLibrary();
            }

            bindings
        });

        Self {
            custom_font_provider: None,

            #[cfg(not(target_arch = "wasm32"))]
            platform_default_font_provider: None,
        }
    }

    /// Creates a new [Pdfium] instance from the given external Pdfium library bindings,
    /// using the custom library configuration in the given [PdfiumLibraryConfig].
    #[inline]
    pub fn new_with_config(
        bindings: Box<dyn PdfiumLibraryBindings>,
        config: PdfiumLibraryConfig,
    ) -> Self {
        // See Pdfium::new for why this initializes exactly once and why a losing
        // thread's bindings box is safe to drop.
        BINDINGS.get_or_init(move || {
            unsafe {
                bindings.FPDF_InitLibraryWithConfig(&config.as_pdfium());
            }

            bindings
        });

        Self {
            custom_font_provider: None,

            #[cfg(not(target_arch = "wasm32"))]
            platform_default_font_provider: None,
        }
    }

    /// Applies the given custom font provider to this [Pdfium] instance.
    pub fn set_custom_font_provider(&mut self, provider: Box<dyn PdfiumCustomFontProvider>) {
        let mut wrapper = Box::pin(PdfiumCustomFontProviderExt::new(provider));

        unsafe {
            self.bindings()
                .FPDF_SetSystemFontInfo(wrapper.as_fpdf_sys_font_info_mut_ptr());
        }

        self.custom_font_provider = Some(wrapper);
    }

    /// Clears the currently set font provider, including Pdfium's platform default font provider.
    pub fn clear_custom_font_provider(&mut self) {
        unsafe {
            self.bindings().FPDF_SetSystemFontInfo(std::ptr::null_mut());
        }

        self.custom_font_provider = None;
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Applies Pdfium's included default font provider for the current platform, if any,
    /// to this [Pdfium] instance.
    pub fn use_platform_default_font_provider(&mut self) -> Result<(), PdfiumError> {
        self.clear_custom_font_provider();

        let platform_default_font_provider =
            unsafe { self.bindings().FPDF_GetDefaultSystemFontInfo() };

        if !platform_default_font_provider.is_null() {
            unsafe {
                self.bindings()
                    .FPDF_SetSystemFontInfo(platform_default_font_provider);
            }

            self.platform_default_font_provider = Some(platform_default_font_provider);

            Ok(())
        } else {
            Err(PdfiumError::NoPlatformDefaultFontProvider)
        }
    }

    #[cfg(target_arch = "wasm32")]
    /// Applies Pdfium's included default font provider for the current platform, if any,
    /// to this [Pdfium] instance.
    ///
    /// This function will always return a `PdfiumError::NoPlatformDefaultFontProvider` error
    /// when compiling to WASM, because Pdfium does not include a default platform provider
    /// implementation for WASM.
    pub fn use_platform_default_font_provider(&mut self) -> Result<(), PdfiumError> {
        Err(PdfiumError::NoPlatformDefaultFontProvider)
    }

    /// Attempts to open a [PdfDocument] from the given static byte buffer.
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    pub fn load_pdf_from_byte_slice<'a>(
        &'a self,
        bytes: &'a [u8],
        password: Option<&str>,
    ) -> Result<PdfDocument<'a>, PdfiumError> {
        Self::pdfium_document_handle_to_result(
            unsafe { self.bindings().FPDF_LoadMemDocument64(bytes, password) },
            self.bindings(),
        )
    }

    /// Attempts to open a [PdfDocument] from the given owned byte buffer.
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    ///
    /// `pdfium-render` will take ownership of the given byte buffer, ensuring its lifetime lasts
    /// as long as the [PdfDocument] opened from it.
    pub fn load_pdf_from_byte_vec(
        &self,
        bytes: Vec<u8>,
        password: Option<&str>,
    ) -> Result<PdfDocument<'_>, PdfiumError> {
        Self::pdfium_document_handle_to_result(
            unsafe {
                self.bindings()
                    .FPDF_LoadMemDocument64(bytes.as_slice(), password)
            },
            self.bindings(),
        )
        .map(|mut document| {
            // Give the newly-created document ownership of the byte buffer, so that Pdfium can continue
            // to read from it on an as-needed basis throughout the lifetime of the document.

            document.set_source_byte_buffer(bytes);

            document
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Attempts to open a [PdfDocument] from the given file path.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading your PDF document data in WASM:
    /// * Use the [Pdfium::load_pdf_from_fetch()] function to download document data from a
    ///   URL using the browser's built-in `fetch` API. This function is only available when
    ///   compiling to WASM.
    /// * Use the [Pdfium::load_pdf_from_blob()] function to load document data from a
    ///   Javascript `File` or `Blob` object (such as a `File` object returned from an HTML
    ///   `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use another method to retrieve the bytes of the target document over the network,
    ///   then load those bytes into Pdfium using either the [Pdfium::load_pdf_from_byte_slice()]
    ///   function or the [Pdfium::load_pdf_from_byte_vec()] function.
    /// * Embed the bytes of the target document directly into the compiled WASM module
    ///   using the `include_bytes!` macro.
    pub fn load_pdf_from_file<'a>(
        &'a self,
        path: &(impl AsRef<Path> + ?Sized),
        password: Option<&str>,
    ) -> Result<PdfDocument<'a>, PdfiumError> {
        self.load_pdf_from_reader(File::open(path).map_err(PdfiumError::IoError)?, password)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Attempts to open a [PdfDocument] from the given reader.
    ///
    /// Pdfium will only load the portions of the document it actually needs into memory.
    /// This is more efficient than loading the entire document into memory, especially when
    /// working with large documents, and allows for working with documents larger than the
    /// amount of available memory.
    ///
    /// Because Pdfium must know the total content length in advance prior to loading
    /// any portion of it, the given reader must implement the [Seek] trait as well as
    /// the [Read] trait.
    ///
    /// If the document is password protected, the given password will be used
    /// to unlock it.
    ///
    /// Pdfium reads from the given reader lazily, calling back into it while the
    /// process-wide Pdfium lock is held. When the `thread_safe` feature is enabled,
    /// the reader's `Read`/`Seek` methods must therefore not block on another thread
    /// that also uses Pdfium, or the two threads will deadlock; and the reader must
    /// be `Send`, because Pdfium may invoke it from whichever thread later triggers
    /// a lazy read. For a reader that cannot meet these constraints, read the whole
    /// document into memory first and use [Pdfium::load_pdf_from_byte_vec] instead.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading your PDF document data in WASM:
    /// * Use the [Pdfium::load_pdf_from_fetch()] function to download document data from a
    ///   URL using the browser's built-in `fetch` API. This function is only available when
    ///   compiling to WASM.
    /// * Use the [Pdfium::load_pdf_from_blob()] function to load document data from a
    ///   Javascript `File` or `Blob` object (such as a `File` object returned from an HTML
    ///   `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use another method to retrieve the bytes of the target document over the network,
    ///   then load those bytes into Pdfium using either the [Pdfium::load_pdf_from_byte_slice()]
    ///   function or the [Pdfium::load_pdf_from_byte_vec()] function.
    /// * Embed the bytes of the target document directly into the compiled WASM module
    ///   using the `include_bytes!` macro.
    pub fn load_pdf_from_reader<'a, R: PdfiumReader + 'a>(
        &'a self,
        reader: R,
        password: Option<&str>,
    ) -> Result<PdfDocument<'a>, PdfiumError> {
        let mut reader = get_pdfium_file_accessor_from_reader(reader);

        Pdfium::pdfium_document_handle_to_result(
            unsafe {
                self.bindings()
                    .FPDF_LoadCustomDocument(reader.as_fpdf_file_access_mut_ptr(), password)
            },
            self.bindings(),
        )
        .map(|mut document| {
            // Give the newly-created document ownership of the reader, so that Pdfium can continue
            // to read from it on an as-needed basis throughout the lifetime of the document.

            document.set_file_access_reader(reader);

            document
        })
    }

    #[cfg(any(doc, target_arch = "wasm32"))]
    /// Attempts to open a [PdfDocument] by loading document data from the given URL.
    /// The Javascript `fetch` API is used to download data over the network.
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    ///
    /// This function is only available when compiling to WASM.
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

    #[cfg(any(doc, target_arch = "wasm32"))]
    /// Attempts to open a [PdfDocument] by loading document data from the given `Blob`.
    /// A `File` object returned from a `FileList` is a suitable `Blob`:
    ///
    /// ```text
    /// <input id="filePicker" type="file">
    ///
    /// const file = document.getElementById('filePicker').files[0];
    /// ```
    ///
    /// If the document is password protected, the given password will be used to unlock it.
    ///
    /// This function is only available when compiling to WASM.
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

        self.load_pdf_from_byte_vec(bytes, password)
    }

    /// Creates a new, empty [PdfDocument] in memory.
    pub fn create_new_pdf<'a>(&'a self) -> Result<PdfDocument<'a>, PdfiumError> {
        Self::pdfium_document_handle_to_result(
            unsafe { self.bindings().FPDF_CreateNewDocument() },
            self.bindings(),
        )
        .map(|mut document| {
            document.set_version(PdfDocumentVersion::DEFAULT_VERSION);

            document
        })
    }

    /// Returns a [PdfDocument] from the given `FPDF_DOCUMENT` handle, if possible.
    pub(crate) fn pdfium_document_handle_to_result(
        handle: FPDF_DOCUMENT,
        bindings: &dyn PdfiumLibraryBindings,
    ) -> Result<PdfDocument<'_>, PdfiumError> {
        if handle.is_null() {
            // Retrieve the error code of the last error recorded by Pdfium. Under
            // the `thread_safe` feature the bindings reference is a
            // ThreadSafePdfiumBindings wrapper, so this FFI call is serialized
            // process-wide like every other.

            if let Some(error) = match unsafe { bindings.FPDF_GetLastError() } as u32 {
                FPDF_ERR_SUCCESS => None,
                FPDF_ERR_UNKNOWN => Some(PdfiumInternalError::Unknown),
                FPDF_ERR_FILE => Some(PdfiumInternalError::FileError),
                FPDF_ERR_FORMAT => Some(PdfiumInternalError::FormatError),
                FPDF_ERR_PASSWORD => Some(PdfiumInternalError::PasswordError),
                FPDF_ERR_SECURITY => Some(PdfiumInternalError::SecurityError),
                FPDF_ERR_PAGE => Some(PdfiumInternalError::PageError),
                // The Pdfium documentation says "... if the previous SDK call succeeded, [then] the
                // return value of this function is not defined". On Linux, at least, a return value
                // of FPDF_ERR_SUCCESS seems to be consistently returned; on Windows, however, the
                // return values are indeed unpredictable. See https://github.com/ajrcarey/pdfium-render/issues/24.
                // Therefore, if the return value does not match one of the FPDF_ERR_* constants, we must
                // assume success.
                _ => None,
            } {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfDocument::from_pdfium(handle))
        }
    }
}

impl Default for Pdfium {
    #[cfg(feature = "static")]
    /// Binds to a Pdfium library that was statically linked into the currently running
    /// executable by calling [Pdfium::bind_to_statically_linked_library]. This function
    /// will panic if no statically linked Pdfium functions can be located.
    #[inline]
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_statically_linked_library().unwrap())
    }

    #[cfg(not(feature = "static"))]
    #[cfg(not(target_arch = "wasm32"))]
    /// Binds to an external Pdfium library by first attempting to bind to a Pdfium library
    /// in the current working directory; if that fails, then a system-provided library
    /// will be used as a fall back.
    ///
    /// This function will panic if no suitable Pdfium library can be loaded.
    #[inline]
    fn default() -> Self {
        // Attempt to bind to a Pdfium library in the current working directory.

        match Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")) {
            Ok(bindings) => Pdfium::new(bindings), // Create new bindings
            Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized) => Pdfium {
                custom_font_provider: None,
                platform_default_font_provider: None,
            }, // Re-use the existing bindings
            Err(PdfiumError::LoadLibraryError(err)) => {
                match err {
                    libloading::Error::DlOpen { .. } => {
                        // For DlOpen errors specifically, indicating the Pdfium library in the
                        // current working directory does not exist or is corrupted, we attempt
                        // to fall back to a system-provided library.

                        Pdfium::new(Pdfium::bind_to_system_library().unwrap())
                    }
                    _ => Err(PdfiumError::LoadLibraryError(err)).unwrap(), // Explicitly re-throw the error
                }
            }
            Err(err) => Err(err).unwrap(), // Explicitly re-throw the error
        }
    }

    #[cfg(target_arch = "wasm32")]
    /// Binds to an external Pdfium library by attempting to a system-provided library.
    ///
    /// This function will panic if no suitable Pdfium library can be loaded.
    fn default() -> Self {
        Pdfium::new(Pdfium::bind_to_system_library().unwrap())
    }
}

impl Debug for Pdfium {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pdfium").finish()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Pdfium {
    fn drop(&mut self) {
        if let Some(ptr) = self.platform_default_font_provider {
            unsafe {
                self.bindings().FPDF_FreeDefaultSystemFontInfo(ptr);
            }
        }
    }
}

impl PdfiumLibraryBindingsAccessor<'_> for Pdfium {}

// Sharing a `Pdfium` across threads is sound under the `thread_safe` feature: the
// promoted bindings are a `ThreadSafePdfiumBindings` wrapper that serializes every
// call into Pdfium's non-reentrant C API behind a process-wide mutex, so no two
// threads can ever be inside Pdfium at the same time. `Pdfium` itself holds no
// interior mutable state that these impls would expose unsynchronized.
#[cfg(feature = "thread_safe")]
unsafe impl Sync for Pdfium {}

#[cfg(feature = "thread_safe")]
unsafe impl Send for Pdfium {}
