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
use once_cell::sync::{Lazy, OnceCell};
use std::fmt::{Debug, Formatter};
use std::pin::Pin;

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
// Serialization is provided by a reentrant, process-wide lock. Every method that
// calls into the bindings acquires an FfiLock as its first statement and holds it
// for the whole operation, so a logical operation made of several FFI calls runs
// atomically. Because the lock is reentrant, a method never has to reason about
// whether its callers already hold it: the first acquisition on a thread takes
// the global mutex, and nested acquisitions on the same thread do not re-lock, so
// composing locked methods cannot deadlock. Threads other than the current owner
// block until the owning thread releases its outermost acquisition.
//
// A `std::sync::Mutex` backs the lock. It is deliberately NOT a fairer/reentrant
// third-party mutex: full serialization is inherent (pdfium is not thread-safe at
// all, so there is no cross-thread parallelism to be had regardless), and under
// equal demand `std::sync::Mutex` is already fair on the platforms measured while
// keeping noticeably higher throughput than a reentrant mutex. The apparent
// "starvation" of one thread by another arises only when a thread genuinely wants
// the lock ~100% of the time via back-to-back acquisitions (e.g. a tight render
// loop); no mutex can help that, and it is an application-level concern.
//
// `Lazy` rather than a `const`-initialized `Mutex`: `Mutex::new` only became a
// `const fn` in Rust 1.63, and this crate's MSRV is 1.61.
#[cfg(feature = "thread_safe")]
static FFI_MUTEX: Lazy<std::sync::Mutex<()>> = Lazy::new(|| std::sync::Mutex::new(()));

#[cfg(feature = "thread_safe")]
thread_local! {
    // The current thread's reentrancy depth ONLY. This is a `Cell<usize>`, which
    // has no destructor, so it registers no thread-local destructor and stays
    // accessible for the entire life of the thread — including while the thread is
    // being torn down. The outermost mutex guard is NOT kept here; it lives in the
    // FfiLock value (see below). A guard stored in a thread-local would register a
    // destructor, and a Pdfium object held in a user `thread_local!` and dropped
    // during thread teardown could then call the lock after that destructor had
    // run, tripping a "TLS accessed during/after destruction" panic (an abort in a
    // Drop). Keeping only the depth here avoids that hazard entirely.
    static FFI_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A reentrant, process-wide lock serializing calls into Pdfium's non-reentrant
/// C API.
///
/// Acquire one as the first statement of any method that calls into the bindings
/// and hold it for the whole operation:
///
/// ```ignore
/// #[cfg(feature = "thread_safe")]
/// let _ffi = crate::pdfium::FfiLock::acquire();
/// ```
///
/// Bind the guard to a named variable (`_ffi`) as shown, so it is held to the end
/// of the scope. A `let _ = ...` binding would drop it immediately, unlocking
/// before the FFI call it is meant to protect; the ffi_lock_gate test rejects
/// that form.
///
/// The first acquisition on a thread takes the global mutex; nested acquisitions
/// on the same thread do not re-lock, so composing locked methods cannot
/// deadlock. The mutex is released only when the outermost acquisition on the
/// thread is dropped, that is, when the recursion depth returns to zero.
#[cfg(feature = "thread_safe")]
pub(crate) struct FfiLock {
    // The outermost acquisition on a thread holds Some(guard); nested acquisitions
    // hold None. Keeping the guard in the FfiLock value — rather than in a
    // thread-local with a destructor — is what avoids the thread-teardown hazard
    // described on FFI_DEPTH.
    //
    // Correctness rests on FfiLock values dropping in reverse acquisition order
    // (LIFO) on any given thread, so the outermost — the guard holder — drops last,
    // exactly when the depth returns to zero. That holds because FfiLock is
    // crate-private and only ever bound to a local (`let _ffi = ...`); it is never
    // stored in a field or moved into a longer-lived place, so nested acquisitions
    // always drop before their enclosing one.
    _guard: Option<std::sync::MutexGuard<'static, ()>>,

    // Keeps FfiLock neither Send nor Sync even when `_guard` is None, so a lock
    // cannot be moved to another thread or held across an await point on a
    // multi-threaded executor.
    _not_send: std::marker::PhantomData<*const ()>,
}

#[cfg(feature = "thread_safe")]
impl FfiLock {
    #[inline]
    pub(crate) fn acquire() -> Self {
        let outermost = FFI_DEPTH.with(|depth| depth.get() == 0);

        let guard = if outermost {
            // Outermost acquisition on this thread: take the global mutex. A
            // poisoned mutex still yields its guard; recovering here keeps one
            // panicking thread from wedging every other thread on a permanently
            // poisoned lock, and the lock protects only Pdfium's own state, not
            // any Rust invariant that a panic could break. (Locking may block; the
            // depth is only bumped afterwards, but no nested acquisition can run on
            // this thread while it is parked here anyway.)
            Some(
                Lazy::force(&FFI_MUTEX)
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner),
            )
        } else {
            None
        };

        FFI_DEPTH.with(|depth| depth.set(depth.get() + 1));

        Self {
            _guard: guard,
            _not_send: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "thread_safe")]
impl Drop for FfiLock {
    #[inline]
    fn drop(&mut self) {
        // saturating_sub rather than `current - 1`: an underflow (impossible, since
        // FfiLock is !Send so acquire and drop are balanced per thread) would
        // otherwise wrap to usize::MAX in release and never release the mutex.
        FFI_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));

        // `_guard` is dropped immediately after this body: for the outermost
        // FfiLock it holds Some(guard) and releases the mutex; for nested ones it
        // is None. Because FfiLock values drop LIFO, the guard holder drops last,
        // i.e. exactly when the depth has returned to zero.
    }
}

/// A trait implemented by every high-level type, giving lifetime-free access to
/// the process-wide [PdfiumLibraryBindings] promoted into a global on the first
/// [Pdfium] instantiation.
///
/// When the `thread_safe` feature is enabled, any FFI call made through the
/// reference returned by [PdfiumLibraryBindingsAccessor::bindings] must be
/// serialized: acquire an [FfiLock] as the first statement of the enclosing
/// method and hold it for the whole operation. The lock is reentrant, so a
/// method never has to know whether its callers already hold it.
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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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

    /// Runs the given closure while holding the process-wide Pdfium FFI lock,
    /// giving it serialized access to the raw [PdfiumLibraryBindings].
    ///
    /// Use this when calling Pdfium's C API directly through the reference returned
    /// by [PdfiumLibraryBindingsAccessor::bindings]: Pdfium is not reentrant, so
    /// every direct FFI call must happen inside this closure to be serialized
    /// against pdfium-render's own calls. The lock is reentrant, so pdfium-render
    /// methods called from within the closure do not deadlock.
    ///
    /// The closure must not block on another thread that also uses Pdfium: it runs
    /// while the global lock is held, so a thread it waits on can never acquire the
    /// lock.
    ///
    /// When the `thread_safe` feature is disabled there is no lock and the closure
    /// simply runs with the bindings.
    #[cfg(feature = "thread_safe")]
    #[inline]
    pub fn with_ffi_lock<R>(&self, f: impl FnOnce(&dyn PdfiumLibraryBindings) -> R) -> R {
        let _ffi = crate::pdfium::FfiLock::acquire();

        f(self.bindings())
    }

    /// Runs the given closure with serialized access to the raw
    /// [PdfiumLibraryBindings]. See the `thread_safe` variant for details; without
    /// that feature there is no lock and the closure simply runs with the bindings.
    #[cfg(not(feature = "thread_safe"))]
    #[inline]
    pub fn with_ffi_lock<R>(&self, f: impl FnOnce(&dyn PdfiumLibraryBindings) -> R) -> R {
        f(self.bindings())
    }

    /// Applies the given custom font provider to this [Pdfium] instance.
    pub fn set_custom_font_provider(&mut self, provider: Box<dyn PdfiumCustomFontProvider>) {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        let mut wrapper = Box::pin(PdfiumCustomFontProviderExt::new(provider));

        unsafe {
            self.bindings()
                .FPDF_SetSystemFontInfo(wrapper.as_fpdf_sys_font_info_mut_ptr());
        }

        self.custom_font_provider = Some(wrapper);
    }

    /// Clears the currently set font provider, including Pdfium's platform default font provider.
    pub fn clear_custom_font_provider(&mut self) {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        unsafe {
            self.bindings().FPDF_SetSystemFontInfo(std::ptr::null_mut());
        }

        self.custom_font_provider = None;
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Applies Pdfium's included default font provider for the current platform, if any,
    /// to this [Pdfium] instance.
    pub fn use_platform_default_font_provider(&mut self) -> Result<(), PdfiumError> {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

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
            // Retrieve the error code of the last error recorded by Pdfium.
            // This function receives a raw bindings reference rather than going
            // through the locked accessor, so serialize the call explicitly.
            #[cfg(feature = "thread_safe")]
            let _ffi = FfiLock::acquire();

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
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        if let Some(ptr) = self.platform_default_font_provider {
            unsafe {
                self.bindings().FPDF_FreeDefaultSystemFontInfo(ptr);
            }
        }
    }
}

impl PdfiumLibraryBindingsAccessor<'_> for Pdfium {}

#[cfg(feature = "thread_safe")]
unsafe impl Sync for Pdfium {}

#[cfg(feature = "thread_safe")]
unsafe impl Send for Pdfium {}
