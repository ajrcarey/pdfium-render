//! Defines the [PdfiumCustomFontProvider] trait, used to define and configure custom
//! font providers for a Pdfium instance to use in place of the default system font provider.

use crate::bindgen::{
    FPDF_BOOL, FPDF_SYSFONTINFO, FXFONT_FF_FIXEDPITCH, FXFONT_FF_ROMAN, FXFONT_FF_SCRIPT,
};
use crate::pdf::font::{PdfFontCharacterSet, PdfFontWeight};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use std::panic;
use std::str::FromStr;

/// A single custom font lookup request from Pdfium.
pub struct PdfiumCustomFontProviderRequest {
    /// The font face of the requested custom font.
    pub font_face: String,

    /// The character set of the requested custom font.
    pub character_set: PdfFontCharacterSet,

    /// The weight of the requested custom font.
    pub weight: PdfFontWeight,

    /// `true` if the glyphs in the requested custom font should include dominant vertical
    /// strokes that are slanted.
    pub is_italic: bool,

    /// `true` if all the glyphs in the requested custom font should have the same width.
    pub is_fixed_pitch: bool,

    /// `true` if the glyphs in the requested custom font should have serifs - short strokes
    /// drawn at an angle on the top or bottom of glyph stems to decorate the glyphs.
    /// For example, Times New Roman is a serif font.
    pub is_serif: bool,

    /// `true` if the glyphs in the requested custom font should be designed to resemble
    /// cursive handwriting.
    pub is_cursive: bool,
}

#[cfg(feature = "thread_safe")]
unsafe impl Sync for PdfiumCustomFontProviderRequest {}

#[cfg(feature = "thread_safe")]
unsafe impl Send for PdfiumCustomFontProviderRequest {}

/// The unique ID of a single custom font lookup result.
pub type PdfiumCustomFontHandle = u64;

/// The response to a single custom font lookup request from Pdfium.
pub struct PdfiumCustomFontProviderResponse {
    /// A unique ID for the custom font provided in this response. Pdfium will use this
    /// value as a font handle in all subsequent calls related to this font.
    pub id: PdfiumCustomFontHandle,

    /// The font face of the custom font provided in this response.
    pub font_face: String,

    /// The character set of the custom font provided in this response.
    pub character_set: PdfFontCharacterSet,

    /// The raw font byte data for the custom font provided in this response, in either
    /// OpenType or TrueType format.
    pub data: Vec<u8>,
}

#[cfg(feature = "thread_safe")]
unsafe impl Sync for PdfiumCustomFontProviderResponse {}

#[cfg(feature = "thread_safe")]
unsafe impl Send for PdfiumCustomFontProviderResponse {}

/// At trait that responds to a single custom font lookup request from Pdfium.
pub trait PdfiumCustomFontProvider: Send + Sync {
    /// Responds to a single custom font lookup request from Pdfium, returning either a valid
    /// response if the font is available or `None` if the font is not available to this
    /// provider implementation.
    fn provide(
        &mut self,
        request: PdfiumCustomFontProviderRequest,
    ) -> Option<PdfiumCustomFontProviderResponse>;
}

#[repr(C)]
#[allow(non_snake_case)]
pub(crate) struct PdfiumCustomFontProviderExt {
    // An extension of Pdfium's FPDF_SYSFONTINFO struct that adds extra fields to carry the
    // user-provided PdfiumCustomFontProvider trait implementation and a cache of responses
    // from the PdfiumCustomFontProvider implementation.
    version: c_int,
    Release: Option<unsafe extern "C" fn(pThis: *mut FPDF_SYSFONTINFO)>,
    EnumFonts: Option<unsafe extern "C" fn(pThis: *mut FPDF_SYSFONTINFO, pMapper: *mut c_void)>,
    MapFont: Option<
        unsafe extern "C" fn(
            pThis: *mut FPDF_SYSFONTINFO,
            weight: c_int,
            bItalic: FPDF_BOOL,
            charset: c_int,
            pitch_family: c_int,
            face: *const c_char,
            bExact: *mut FPDF_BOOL,
        ) -> *mut c_void,
    >,
    GetFont: Option<
        unsafe extern "C" fn(pThis: *mut FPDF_SYSFONTINFO, face: *const c_char) -> *mut c_void,
    >,
    GetFontData: Option<
        unsafe extern "C" fn(
            pThis: *mut FPDF_SYSFONTINFO,
            hFont: *mut c_void,
            table: c_uint,
            buffer: *mut c_uchar,
            buf_size: c_ulong,
        ) -> c_ulong,
    >,
    GetFaceName: Option<
        unsafe extern "C" fn(
            pThis: *mut FPDF_SYSFONTINFO,
            hFont: *mut c_void,
            buffer: *mut c_char,
            buf_size: c_ulong,
        ) -> c_ulong,
    >,
    GetFontCharset:
        Option<unsafe extern "C" fn(pThis: *mut FPDF_SYSFONTINFO, hFont: *mut c_void) -> c_int>,
    DeleteFont: Option<unsafe extern "C" fn(pThis: *mut FPDF_SYSFONTINFO, hFont: *mut c_void)>,
    provider: Box<dyn PdfiumCustomFontProvider>,
    cache: HashMap<PdfiumCustomFontHandle, PdfiumCustomFontProviderResponse>,
}

impl PdfiumCustomFontProviderExt {
    pub(crate) fn new(provider: Box<dyn PdfiumCustomFontProvider>) -> Self {
        PdfiumCustomFontProviderExt {
            version: 2,
            EnumFonts: None, // not used in interface version 2
            Release: Some(fpdf_sys_font_info_release),
            MapFont: Some(fpdf_sys_font_info_map_font),
            GetFont: None,
            GetFontData: Some(fpdf_sys_font_info_get_font_data),
            GetFaceName: Some(fpdf_sys_font_info_get_face_name),
            GetFontCharset: Some(fpdf_sys_font_info_get_font_charset),
            DeleteFont: Some(fpdf_sys_font_info_delete_font),
            provider,
            cache: HashMap::new(),
        }
    }

    /// Returns an `FPDF_SYSFONTINFO` pointer suitable for passing to `FPDF_SetSystemFontInfo()`.
    #[inline]
    pub(crate) fn as_fpdf_sys_font_info_mut_ptr(&mut self) -> &mut FPDF_SYSFONTINFO {
        unsafe { &mut *(self as *mut PdfiumCustomFontProviderExt as *mut FPDF_SYSFONTINFO) }
    }
}

/// Unwraps a mutable reference to the underlying [PdfiumCustomFontProvider] from a `FPDF_SYSFONTINFO` pointer.
#[allow(non_snake_case)]
unsafe fn fpdf_sys_font_info_to_custom_font_provider<'a>(
    pThis: *mut FPDF_SYSFONTINFO,
) -> &'a mut PdfiumCustomFontProviderExt {
    &mut *(pThis as *mut PdfiumCustomFontProviderExt)
}

// The `FPDF_SYSFONTINFO::Release` callback function invoked by Pdfium.
#[allow(non_snake_case)]
unsafe extern "C" fn fpdf_sys_font_info_release(pThis: *mut FPDF_SYSFONTINFO) {
    fpdf_sys_font_info_to_custom_font_provider(pThis)
        .cache
        .clear();
}

// The `FPDF_SYSFONTINFO::MapFont` callback function invoked by Pdfium.
#[allow(non_snake_case)]
unsafe extern "C" fn fpdf_sys_font_info_map_font(
    pThis: *mut FPDF_SYSFONTINFO,
    weight: c_int,
    bItalic: FPDF_BOOL,
    charset: c_int,
    pitch_family: c_int,
    face: *const c_char,
    _bExact: *mut FPDF_BOOL, // unused field
) -> *mut c_void {
    if pThis.is_null() || face.is_null() {
        return std::ptr::null_mut();
    }

    let provider = fpdf_sys_font_info_to_custom_font_provider(pThis);

    let result = provider.provider.provide(PdfiumCustomFontProviderRequest {
        font_face: match CStr::from_ptr(face).to_str() {
            Ok(font_face) => font_face.to_owned(),
            Err(_) => return std::ptr::null_mut(),
        },
        character_set: match PdfFontCharacterSet::from_pdfium(charset) {
            Some(character_set) => character_set,
            None => return std::ptr::null_mut(),
        },
        weight: match PdfFontWeight::from_pdfium(weight) {
            Some(weight) => weight,
            None => return std::ptr::null_mut(),
        },
        is_italic: bItalic != 0,
        is_fixed_pitch: pitch_family & (FXFONT_FF_FIXEDPITCH as i32) == 1,
        is_serif: pitch_family & (FXFONT_FF_ROMAN as i32) == 1,
        is_cursive: pitch_family & (FXFONT_FF_SCRIPT as i32) == 1,
    });

    match result {
        Some(response) => {
            let id = response.id;

            provider.cache.insert(id, response);

            id as *mut c_void
        }
        None => std::ptr::null_mut(),
    }
}

// The `FPDF_SYSFONTINFO::GetFontData` callback function invoked by Pdfium.
#[allow(non_snake_case)]
unsafe extern "C" fn fpdf_sys_font_info_get_font_data(
    pThis: *mut FPDF_SYSFONTINFO,
    hFont: *mut c_void,
    table: c_uint,
    buffer: *mut c_uchar,
    buf_size: c_ulong,
) -> c_ulong {
    if pThis.is_null() || hFont.is_null() {
        return 0;
    }

    if table != 0 {
        // We only support table == 0, i.e. returning the full font file.

        return 0;
    }

    if let Some(response) = fpdf_sys_font_info_to_custom_font_provider(pThis)
        .cache
        .get(&(hFont as PdfiumCustomFontHandle))
    {
        let font_data = &response.data;

        if !buffer.is_null() && buf_size as usize >= font_data.len() {
            buffer.copy_from_nonoverlapping(font_data.as_ptr(), font_data.len());
        }

        font_data.len() as c_ulong
    } else {
        // Undefined behaviour: Pdfium called us with an opaque font handle that doesn't
        // correspond to any cached response. This should never happen; we cannot directly
        // communicate the failure to Pdfium, but we can at least safely return no data.

        0
    }
}

// The `FPDF_SYSFONTINFO::GetFaceName` callback function invoked by Pdfium.
#[allow(non_snake_case)]
unsafe extern "C" fn fpdf_sys_font_info_get_face_name(
    pThis: *mut FPDF_SYSFONTINFO,
    hFont: *mut c_void,
    buffer: *mut c_char,
    buf_size: c_ulong,
) -> c_ulong {
    if let Some(response) = fpdf_sys_font_info_to_custom_font_provider(pThis)
        .cache
        .get(&(hFont as PdfiumCustomFontHandle))
    {
        if let Ok(face_name) = CString::from_str(&response.font_face) {
            let chars = face_name.as_bytes_with_nul();

            if !buffer.is_null() && buf_size as usize >= chars.len() {
                buffer.copy_from_nonoverlapping(chars.as_ptr() as *const i8, chars.len());
            }

            chars.len() as c_ulong
        } else {
            // Undefined behaviour: font face name cannot be converted into a C string.
            // There is no mechanism for reporting the failure back to Pdfium, so we must abort.

            panic!(
                "Unable to convert face name to C string in fpdf_sys_font_info_get_face_name: {:?}",
                &response.font_face
            );
        }
    } else {
        // Undefined behaviour: Pdfium called us with an opaque font handle that doesn't
        // correspond to any cached response. This should never happen, there is no mechanism
        // for reporting the failure back to Pdfium, and so we must abort.

        panic!(
            "Unknown font handle received from Pdfium in fpdf_sys_font_info_get_face_name: {:?}",
            hFont
        );
    }
}

// The `FPDF_SYSFONTINFO::GetFontCharset` callback function invoked by Pdfium.
#[allow(non_snake_case)]
unsafe extern "C" fn fpdf_sys_font_info_get_font_charset(
    pThis: *mut FPDF_SYSFONTINFO,
    hFont: *mut c_void,
) -> c_int {
    if let Some(response) = fpdf_sys_font_info_to_custom_font_provider(pThis)
        .cache
        .get(&(hFont as PdfiumCustomFontHandle))
    {
        response.character_set.as_pdfium()
    } else {
        // Undefined behaviour: Pdfium called us with an opaque font handle that doesn't
        // correspond to any cached response. This should never happen, there is no mechanism
        // for reporting the failure back to Pdfium, and so we must abort.

        panic!(
            "Unknown font handle received from Pdfium in fpdf_sys_font_info_get_font_charset: {:?}",
            hFont
        );
    }
}

// The `FPDF_SYSFONTINFO::DeleteFont` callback function invoked by Pdfium.
#[allow(non_snake_case)]
unsafe extern "C" fn fpdf_sys_font_info_delete_font(
    pThis: *mut FPDF_SYSFONTINFO,
    hFont: *mut c_void,
) {
    fpdf_sys_font_info_to_custom_font_provider(pThis)
        .cache
        .remove(&(hFont as PdfiumCustomFontHandle));
}
