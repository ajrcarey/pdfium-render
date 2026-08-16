//! Defines the [PdfiumLibraryConfig] struct, used to pass custom initialization configuration
//! to a new Pdfium library instance.

use crate::bindgen::{
    FPDF_LIBRARY_CONFIG, FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG,
    FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,
};
use crate::error::PdfiumError;
use std::ffi::{CString, NulError};
use std::os::raw::{c_char, c_uint};
use std::pin::Pin;
use std::ptr::null_mut;
use std::str::FromStr;

#[cfg(not(target_arch = "wasm32"))]
use std::os::raw::c_void;

#[cfg(any(
    feature = "pdfium_future",
    feature = "pdfium_7881",
    feature = "pdfium_7763",
))]
use crate::bindgen::{
    FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FONTATIONS,
    FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FREETYPE,
};

#[derive(Clone)]
pub struct PdfiumLibraryConfig {
    user_font_paths: Vec<CString>,
    user_font_paths_ptrs: Pin<Box<Vec<*const c_char>>>,

    #[cfg(not(target_arch = "wasm32"))]
    v8_isolate_ptr: *mut c_void,
    #[cfg(not(target_arch = "wasm32"))]
    v8_embedder_slot_idx: c_uint,
    #[cfg(not(target_arch = "wasm32"))]
    v8_platform_ptr: *mut c_void,

    renderer_type: c_uint,

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7881",
        feature = "pdfium_7763",
    ))]
    font_library_type: c_uint,
}

impl PdfiumLibraryConfig {
    /// Creates a new [PdfiumLibraryConfig] object with all settings initialized to
    /// their default values.
    pub fn new() -> Self {
        PdfiumLibraryConfig {
            user_font_paths: vec![],
            user_font_paths_ptrs: Box::pin(vec![]),

            #[cfg(not(target_arch = "wasm32"))]
            v8_isolate_ptr: null_mut(),
            #[cfg(not(target_arch = "wasm32"))]
            v8_embedder_slot_idx: 0,
            #[cfg(not(target_arch = "wasm32"))]
            v8_platform_ptr: null_mut(),

            renderer_type: FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA,

            #[cfg(any(
                feature = "pdfium_future",
                feature = "pdfium_7881",
                feature = "pdfium_7763",
            ))]
            font_library_type: FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FREETYPE,
        }
    }

    /// Clears any user-specified paths that should be interrogated by Pdfium when
    /// attempting to load any custom fonts referenced in a PDF document.
    #[inline]
    pub fn clear_user_font_paths(self) -> Self {
        self.set_user_font_paths(&[]).unwrap()
    }

    #[cfg(target_arch = "wasm32")]
    /// Sets the list of user-specified paths that should be interrogated by Pdfium when
    /// attempting to load any custom fonts referenced in a PDF document.
    ///
    /// Since the browser does not provide a font loading mechanism, this list of font paths
    /// is empty when compiling to WASM.
    #[inline]
    pub fn set_platform_default_user_font_paths(self) -> Self {
        self.clear_user_font_paths()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(target_os = "linux")]
    /// Sets the list of user-specified paths that should be interrogated by Pdfium when
    /// attempting to load any custom fonts referenced in a PDF document.
    ///
    /// On Linux systems, the platform default font paths are `/usr/share/fonts/truetype/`
    /// and `/usr/local/share/fonts/`.
    #[inline]
    pub fn set_platform_default_user_font_paths(self) -> Self {
        self.set_user_font_paths(&["/usr/share/fonts/truetype/", "/usr/local/share/fonts/"])
            .unwrap()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(target_os = "macos")]
    /// Sets the list of user-specified paths that should be interrogated by Pdfium when
    /// attempting to load any custom fonts referenced in a PDF document.
    ///
    /// On macOS systems, the platform default font paths are `/Library/Fonts/` and
    /// `/System/Library/Fonts/`.
    #[inline]
    pub fn set_platform_default_user_font_paths(self) -> Self {
        self.set_user_font_paths(&["/Library/Fonts/", "/System/Library/Fonts/"])
            .unwrap()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(target_os = "windows")]
    /// Sets the list of user-specified paths that should be interrogated by Pdfium when
    /// attempting to load any custom fonts referenced in a PDF document.
    ///
    /// On Windows systems, the platform default font path is `C:\Windows\Fonts\`.
    #[inline]
    pub fn set_platform_default_user_font_paths(self) -> Self {
        self.set_user_font_paths(&["C:\\Windows\\Fonts\\"]).unwrap()
    }

    /// Sets the list of user-specified paths that should be interrogated by Pdfium when
    /// attempting to load any custom fonts referenced in a PDF document.
    pub fn set_user_font_paths(mut self, paths: &[&str]) -> Result<Self, PdfiumError> {
        let user_font_paths = paths
            .iter()
            .map(|path| CString::from_str(path))
            .collect::<Result<Vec<CString>, NulError>>();

        match user_font_paths {
            Ok(paths) => {
                let mut ptrs = paths.iter().map(|path| path.as_ptr()).collect::<Vec<_>>();

                if !ptrs.is_empty() {
                    ptrs.push(std::ptr::null());
                }

                self.user_font_paths = paths;
                self.user_font_paths_ptrs = Box::pin(ptrs);

                Ok(self)
            }
            Err(e) => Err(PdfiumError::InvalidUserFontPath(e)),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Sets the pointer to the `v8::Isolate` to use. If `NULL`, Pdfium will create one.
    #[inline]
    pub unsafe fn set_v8_isolate_ptr(mut self, ptr: *mut c_void) -> Self {
        self.v8_isolate_ptr = ptr;
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Sets the embedder data slot to use in the `v8::Isolate` to store Pdfium's per-isolate
    /// data. The value needs to be in the range `[0, v8::Internals::kNumIsolateDataLots)`.
    /// Note that `0` is fine for most embedders.
    #[inline]
    pub unsafe fn set_v8_embedder_slot(mut self, idx: c_uint) -> Self {
        self.v8_embedder_slot_idx = idx;
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Sets the pointer to the `v8::Platform` to use.
    #[inline]
    pub unsafe fn set_v8_platform_ptr(mut self, ptr: &mut c_void) -> Self {
        self.v8_platform_ptr = ptr;
        self
    }

    /// Sets Pdfium's graphics renderer to the Anti-Grain Geometry library, <https://sourceforge.net/projects/agg/>.
    #[inline]
    pub fn set_renderer_anti_grain_geometry(mut self) -> Self {
        self.renderer_type = FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_AGG;
        self
    }

    /// Sets Pdfium's graphics renderer to Skia, <https://skia.org/>.
    #[inline]
    pub fn set_renderer_skia(mut self) -> Self {
        self.renderer_type = FPDF_RENDERER_TYPE_FPDF_RENDERERTYPE_SKIA;
        self
    }

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7881",
        feature = "pdfium_7763",
    ))]
    /// Sets Pdfium's font handler to FreeType, <https://freetype.org/>.
    #[inline]
    pub fn set_font_backend_freetype(mut self) -> Self {
        self.font_library_type = FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FREETYPE;
        self
    }

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7881",
        feature = "pdfium_7763",
    ))]
    /// Sets Pdfium's font handler to Fontations, <https://github.com/googlefonts/fontations/>.
    #[inline]
    pub fn set_font_backend_fontations(mut self) -> Self {
        self.font_library_type = FPDF_FONT_BACKEND_TYPE_FPDF_FONTBACKENDTYPE_FONTATIONS;
        self
    }

    /// Returns a `FPDF_LIBRARY_CONFIG` instance from this [PdfiumLibraryConfig] instance
    /// that can be passed to Pdfium's `FPDF_FPDF_InitLibraryWithConfig()` function.
    pub(crate) fn as_pdfium(&mut self) -> FPDF_LIBRARY_CONFIG {
        let config = FPDF_LIBRARY_CONFIG {
            version: 2,
            m_pUserFontPaths: if self.user_font_paths_ptrs.is_empty() {
                std::ptr::null_mut()
            } else {
                self.user_font_paths_ptrs.as_mut_ptr()
            },

            #[cfg(not(target_arch = "wasm32"))]
            m_pIsolate: self.v8_isolate_ptr,
            #[cfg(not(target_arch = "wasm32"))]
            m_v8EmbedderSlot: self.v8_embedder_slot_idx,
            #[cfg(not(target_arch = "wasm32"))]
            m_pPlatform: self.v8_platform_ptr,

            #[cfg(target_arch = "wasm32")]
            m_pIsolate: null_mut(),
            #[cfg(target_arch = "wasm32")]
            m_v8EmbedderSlot: 0,
            #[cfg(target_arch = "wasm32")]
            m_pPlatform: null_mut(),

            m_RendererType: self.renderer_type,

            #[cfg(any(
                feature = "pdfium_future",
                feature = "pdfium_7881",
                feature = "pdfium_7763",
            ))]
            m_FontLibraryType: self.font_library_type,
        };

        config
    }
}

#[cfg(feature = "thread_safe")]
unsafe impl Sync for PdfiumLibraryConfig {}

#[cfg(feature = "thread_safe")]
unsafe impl Send for PdfiumLibraryConfig {}

#[cfg(test)]
mod tests {
    use super::PdfiumLibraryConfig;
    use std::ffi::CStr;

    #[test]
    fn user_font_paths_are_kept_alive_and_null_terminated() {
        let mut config = PdfiumLibraryConfig::new()
            .set_user_font_paths(&["/first/font/path", "/second/font/path"])
            .unwrap();

        assert_eq!(
            config.as_pdfium().m_pUserFontPaths,
            config.user_font_paths_ptrs.as_ptr().cast_mut()
        );
        assert_eq!(config.user_font_paths.len(), 2); // The number of paths submitted by the user
        assert_eq!(config.user_font_paths_ptrs.len(), 3); // Always one extra for the trailing null entry
        assert_eq!(
            unsafe { CStr::from_ptr(config.user_font_paths_ptrs[0]) }
                .to_str()
                .unwrap(),
            "/first/font/path"
        );
        assert_eq!(
            unsafe { CStr::from_ptr(config.user_font_paths_ptrs[1]) }
                .to_str()
                .unwrap(),
            "/second/font/path"
        );
        assert!(config.user_font_paths_ptrs[2].is_null());
    }

    #[test]
    fn empty_user_font_paths_use_a_null_pointer() {
        let mut config = PdfiumLibraryConfig::new();

        assert!(config.as_pdfium().m_pUserFontPaths.is_null());
        assert!(config.user_font_paths_ptrs.is_empty());
    }
}
