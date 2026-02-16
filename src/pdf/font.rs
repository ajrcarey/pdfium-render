//! Defines the [PdfFont] struct, exposing functionality related to a single font used to
//! render text in a `PdfDocument`.

pub mod glyph;
pub mod glyphs;

use crate::bindgen::FPDF_FONT;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::fonts::PdfFontBuiltin;
use crate::pdf::font::glyphs::PdfFontGlyphs;
use crate::pdf::points::PdfPoints;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use crate::utils::mem::create_byte_buffer;
use bitflags::bitflags;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};

// The following dummy declaration is used only when running cargo doc.
// It allows documentation of WASM-specific functionality to be included
// in documentation generated on non-WASM targets.

#[cfg(doc)]
struct Blob;

bitflags! {
    pub(crate) struct FpdfFontDescriptorFlags: u32 {
        const FIXED_PITCH_BIT_1 =  0b00000000000000000000000000000001;
        const SERIF_BIT_2 =        0b00000000000000000000000000000010;
        const SYMBOLIC_BIT_3 =     0b00000000000000000000000000000100;
        const SCRIPT_BIT_4 =       0b00000000000000000000000000001000;
        const NON_SYMBOLIC_BIT_6 = 0b00000000000000000000000000100000;
        const ITALIC_BIT_7 =       0b00000000000000000000000001000000;
        const ALL_CAP_BIT_17 =     0b00000000000000010000000000000000;
        const SMALL_CAP_BIT_18 =   0b00000000000000100000000000000000;
        const FORCE_BOLD_BIT_19 =  0b00000000000001000000000000000000;
    }
}

/// The weight of a [PdfFont]. Typical values are 400 (normal) and 700 (bold).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfFontWeight {
    Weight100,
    Weight200,
    Weight300,
    Weight400Normal,
    Weight500,
    Weight600,
    Weight700Bold,
    Weight800,
    Weight900,

    /// Any font weight value that falls outside the typical 100 - 900 value range.
    Custom(u32),
}

impl PdfFontWeight {
    pub(crate) fn from_pdfium(value: c_int) -> Option<PdfFontWeight> {
        match value {
            -1 => None,
            100 => Some(PdfFontWeight::Weight100),
            200 => Some(PdfFontWeight::Weight200),
            300 => Some(PdfFontWeight::Weight300),
            400 => Some(PdfFontWeight::Weight400Normal),
            500 => Some(PdfFontWeight::Weight500),
            600 => Some(PdfFontWeight::Weight600),
            700 => Some(PdfFontWeight::Weight700Bold),
            800 => Some(PdfFontWeight::Weight800),
            900 => Some(PdfFontWeight::Weight900),
            other => Some(PdfFontWeight::Custom(other as u32)),
        }
    }
}

/// A single font used to render text in a [PdfDocument].
///
/// The PDF specification defines 14 built-in fonts that can be used in any PDF file without
/// font embedding. Additionally, custom fonts can be directly embedded into any PDF file as
/// a data stream.
pub struct PdfFont<'a> {
    built_in: Option<PdfFontBuiltin>,
    handle: FPDF_FONT,
    glyphs: PdfFontGlyphs<'a>,
    is_font_memory_loaded: bool,
    lifetime: PhantomData<&'a FPDF_FONT>,
}

impl<'a> PdfFont<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_FONT,
        built_in: Option<PdfFontBuiltin>,
        is_font_memory_loaded: bool,
    ) -> Self {
        PdfFont {
            built_in,
            handle,
            glyphs: PdfFontGlyphs::from_pdfium(handle),
            is_font_memory_loaded,
            lifetime: PhantomData,
        }
    }

    /// Returns the internal `FPDF_FONT` handle for this [PdfFont].
    #[inline]
    pub(crate) fn handle(&self) -> FPDF_FONT {
        self.handle
    }

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7543",
        feature = "pdfium_7350",
        feature = "pdfium_7215",
        feature = "pdfium_7123",
        feature = "pdfium_6996",
        feature = "pdfium_6721",
        feature = "pdfium_6666"
    ))]
    /// Returns the name of this [PdfFont].
    pub fn name(&self) -> String {
        // Retrieving the font name from Pdfium is a two-step operation. First, we call
        // FPDFFont_GetBaseFontName() with a null buffer; this will retrieve the length of
        // the font name in bytes. If the length is zero, then there is no font name.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFFont_GetBaseFontName() again with a pointer to the buffer;
        // this will write the font name into the buffer. Unlike most text handling in
        // Pdfium, font names are returned in UTF-8 format.

        let buffer_length =
            self.bindings()
                .FPDFFont_GetBaseFontName(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // The font name is not present.

            return String::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings().FPDFFont_GetBaseFontName(
            self.handle,
            buffer.as_mut_ptr() as *mut c_char,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        String::from_utf8(buffer)
            // Trim any trailing nulls. All strings returned from Pdfium are generally terminated
            // by one null byte.
            .map(|str| str.trim_end_matches(char::from(0)).to_owned())
            .unwrap_or_else(|_| String::new())
    }

    /// Returns the family of this [PdfFont].
    pub fn family(&self) -> String {
        // Retrieving the family name from Pdfium is a two-step operation. First, we call
        // FPDFFont_GetFamilyName() with a null buffer; this will retrieve the length of
        // the font name in bytes. If the length is zero, then there is no font name.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFFont_GetFamilyName() again with a pointer to the buffer;
        // this will write the font name into the buffer. Unlike most text handling in
        // Pdfium, font names are returned in UTF-8 format.

        #[cfg(any(
            feature = "pdfium_future",
            feature = "pdfium_7543",
            feature = "pdfium_7350",
            feature = "pdfium_7215",
            feature = "pdfium_7123",
            feature = "pdfium_6996",
            feature = "pdfium_6721",
            feature = "pdfium_6666",
            feature = "pdfium_6611"
        ))]
        let buffer_length =
            self.bindings()
                .FPDFFont_GetFamilyName(self.handle, std::ptr::null_mut(), 0);

        #[cfg(any(
            feature = "pdfium_6569",
            feature = "pdfium_6555",
            feature = "pdfium_6490",
            feature = "pdfium_6406",
            feature = "pdfium_6337",
            feature = "pdfium_6295",
            feature = "pdfium_6259",
            feature = "pdfium_6164",
            feature = "pdfium_6124",
            feature = "pdfium_6110",
            feature = "pdfium_6084",
            feature = "pdfium_6043",
            feature = "pdfium_6015",
            feature = "pdfium_5961"
        ))]
        let buffer_length =
            self.bindings()
                .FPDFFont_GetFontName(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // The font name is not present.

            return String::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        #[cfg(any(
            feature = "pdfium_future",
            feature = "pdfium_7543",
            feature = "pdfium_7350",
            feature = "pdfium_7215",
            feature = "pdfium_7123",
            feature = "pdfium_6996",
            feature = "pdfium_6721",
            feature = "pdfium_6666",
            feature = "pdfium_6611"
        ))]
        let result = self.bindings().FPDFFont_GetFamilyName(
            self.handle,
            buffer.as_mut_ptr() as *mut c_char,
            buffer_length,
        );

        #[cfg(any(
            feature = "pdfium_6569",
            feature = "pdfium_6555",
            feature = "pdfium_6490",
            feature = "pdfium_6406",
            feature = "pdfium_6337",
            feature = "pdfium_6295",
            feature = "pdfium_6259",
            feature = "pdfium_6164",
            feature = "pdfium_6124",
            feature = "pdfium_6110",
            feature = "pdfium_6084",
            feature = "pdfium_6043",
            feature = "pdfium_6015",
            feature = "pdfium_5961"
        ))]
        let result = self.bindings().FPDFFont_GetFontName(
            self.handle,
            buffer.as_mut_ptr() as *mut c_char,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        String::from_utf8(buffer)
            // Trim any trailing nulls. All strings returned from Pdfium are generally terminated
            // by one null byte.
            .map(|str| str.trim_end_matches(char::from(0)).to_owned())
            .unwrap_or_else(|_| String::new())
    }

    /// Returns the weight of this [PdfFont].
    ///
    /// Pdfium may not reliably return the correct value of this property for built-in fonts.
    pub fn weight(&self) -> Result<PdfFontWeight, PdfiumError> {
        PdfFontWeight::from_pdfium(self.bindings().FPDFFont_GetWeight(self.handle)).ok_or(
            PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::Unknown),
        )
    }

    /// Returns the italic angle of this [PdfFont]. The italic angle is the angle,
    /// expressed in degrees counter-clockwise from the vertical, of the dominant vertical
    /// strokes of the font. The value is zero for non-italic fonts, and negative for fonts
    /// that slope to the right (as almost all italic fonts do).
    ///
    /// Pdfium may not reliably return the correct value of this property for built-in fonts.
    pub fn italic_angle(&self) -> Result<i32, PdfiumError> {
        let mut angle = 0;

        if self.bindings().is_true(
            self.bindings()
                .FPDFFont_GetItalicAngle(self.handle, &mut angle),
        ) {
            Ok(angle)
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns the ascent of this [PdfFont] for the given font size. The ascent is the maximum
    /// height above the baseline reached by glyphs in this font, excluding the height of glyphs
    /// for accented characters.
    pub fn ascent(&self, font_size: PdfPoints) -> Result<PdfPoints, PdfiumError> {
        let mut ascent = 0.0;

        if self.bindings().is_true(self.bindings().FPDFFont_GetAscent(
            self.handle,
            font_size.value,
            &mut ascent,
        )) {
            Ok(PdfPoints::new(ascent))
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns the descent of this [PdfFont] for the given font size. The descent is the
    /// maximum distance below the baseline reached by glyphs in this font, expressed as a
    /// negative points value.
    pub fn descent(&self, font_size: PdfPoints) -> Result<PdfPoints, PdfiumError> {
        let mut descent = 0.0;

        if self.bindings().is_true(self.bindings().FPDFFont_GetDescent(
            self.handle,
            font_size.value,
            &mut descent,
        )) {
            Ok(PdfPoints::new(descent))
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns the raw font descriptor bitflags for the containing [PdfFont].
    #[inline]
    fn get_flags_bits(&self) -> FpdfFontDescriptorFlags {
        FpdfFontDescriptorFlags::from_bits_truncate(
            self.bindings().FPDFFont_GetFlags(self.handle) as u32
        )
    }

    /// Returns `true` if all the glyphs in this [PdfFont] have the same width.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_fixed_pitch(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::FIXED_PITCH_BIT_1)
    }

    /// Returns `true` if the glyphs in this [PdfFont] have variable widths.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    #[inline]
    pub fn is_proportional_pitch(&self) -> bool {
        !self.is_fixed_pitch()
    }

    /// Returns `true` if one or more glyphs in this [PdfFont] have serifs - short strokes
    /// drawn at an angle on the top or bottom of glyph stems to decorate the glyphs.
    /// For example, Times New Roman is a serif font.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_serif(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::SERIF_BIT_2)
    }

    /// Returns `true` if no glyphs in this [PdfFont] have serifs - short strokes
    /// drawn at an angle on the top or bottom of glyph stems to decorate the glyphs.
    /// For example, Helvetica is a sans-serif font.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    #[inline]
    pub fn is_sans_serif(&self) -> bool {
        !self.is_serif()
    }

    /// Returns `true` if this [PdfFont] contains glyphs outside the Adobe standard Latin
    /// character set.
    ///
    /// This classification of non-symbolic and symbolic fonts is peculiar to PDF. A font may
    /// contain additional characters that are used in Latin writing systems but are outside the
    /// Adobe standard Latin character set; PDF considers such a font to be symbolic.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_symbolic(&self) -> bool {
        // This flag bit and the non-symbolic flag bit cannot both be set or both be clear.

        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::SYMBOLIC_BIT_3)
    }

    /// Returns `true` if this [PdfFont] does not contain glyphs outside the Adobe standard
    /// Latin character set.
    ///
    /// This classification of non-symbolic and symbolic fonts is peculiar to PDF. A font may
    /// contain additional characters that are used in Latin writing systems but are outside the
    /// Adobe standard Latin character set; PDF considers such a font to be symbolic.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_non_symbolic(&self) -> bool {
        // This flag bit and the symbolic flag bit cannot both be set or both be clear.

        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::NON_SYMBOLIC_BIT_6)
    }

    /// Returns `true` if the glyphs in this [PdfFont] are designed to resemble cursive handwriting.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_cursive(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::SCRIPT_BIT_4)
    }

    /// Returns `true` if the glyphs in this [PdfFont] include dominant vertical strokes
    /// that are slanted.
    ///
    /// The designed vertical stroke angle can be retrieved using the [PdfFont::italic_angle()] function.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_italic(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::ITALIC_BIT_7)
    }

    /// Returns `true` if this [PdfFont] contains no lowercase letters by design.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_all_caps(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::ALL_CAP_BIT_17)
    }

    /// Returns `true` if the lowercase letters in this [PdfFont] have the same shapes as the
    /// corresponding uppercase letters but are sized proportionally so they have the same size
    /// and stroke weight as lowercase glyphs in the same typeface family.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_small_caps(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::SMALL_CAP_BIT_18)
    }

    /// Returns `true` if bold glyphs in this [PdfFont] are painted with extra pixels
    /// at very small font sizes.
    ///
    /// Typically when glyphs are painted at small sizes on low-resolution devices, individual strokes
    /// of bold glyphs may appear only one pixel wide. Because this is the minimum width of a pixel
    /// based device, individual strokes of non-bold glyphs may also appear as one pixel wide
    /// and therefore cannot be distinguished from bold glyphs. If this flag is set, individual
    /// strokes of bold glyphs may be thickened at small font sizes.
    ///
    /// Pdfium may not reliably return the correct value of this flag for built-in fonts.
    pub fn is_bold_reenforced(&self) -> bool {
        self.get_flags_bits()
            .contains(FpdfFontDescriptorFlags::FORCE_BOLD_BIT_19)
    }

    /// Returns `true` if this [PdfFont] is an instance of one of the 14 built-in fonts
    /// provided as part of the PDF specification.
    #[inline]
    pub fn is_built_in(&self) -> bool {
        self.built_in.is_some()
    }

    /// Returns the [PdfFontBuiltin] type of this built-in font, or `None` if this font is
    /// not one of the 14 built-in fonts provided as part of the PDF specification.
    #[inline]
    pub fn built_in(&self) -> Option<PdfFontBuiltin> {
        self.built_in
    }

    /// Returns `true` if the data for this [PdfFont] is embedded in the containing [PdfDocument].
    pub fn is_embedded(&self) -> Result<bool, PdfiumError> {
        let result = self.bindings().FPDFFont_GetIsEmbedded(self.handle);

        match result {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            )),
        }
    }

    /// Writes this [PdfFont] to a new byte buffer, returning the byte buffer.
    ///
    /// If this [PdfFont] is not embedded in the containing [PdfDocument], then the data
    /// returned will be for the substitution font instead.
    pub fn data(&self) -> Result<Vec<u8>, PdfiumError> {
        // Retrieving the font data from Pdfium is a two-step operation. First, we call
        // FPDFFont_GetFontData() with a null buffer; this will retrieve the length of
        // the data in bytes. If the length is zero, then there is no data associated
        // with this font.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFFont_GetFontData() again with a pointer to the buffer;
        // this will write the font data to the buffer.

        let mut out_buflen: usize = 0;

        if self
            .bindings()
            .is_true(self.bindings().FPDFFont_GetFontData(
                self.handle,
                std::ptr::null_mut(),
                0,
                &mut out_buflen,
            ))
        {
            // out_buflen now contains the length of the font data.

            let buffer_length = out_buflen;

            let mut buffer = create_byte_buffer(buffer_length);

            let result = self.bindings().FPDFFont_GetFontData(
                self.handle,
                buffer.as_mut_ptr(),
                buffer_length,
                &mut out_buflen,
            );

            assert!(self.bindings().is_true(result));
            assert_eq!(buffer_length, out_buflen);

            Ok(buffer)
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns a collection of all the [PdfFontGlyphs] defined for this [PdfFont] in the containing
    /// `PdfDocument`.
    ///
    /// Note that documents typically include only the specific glyphs they need from any given font,
    /// not the entire font glyphset. This is a PDF feature known as font subsetting. The collection
    /// of glyphs returned by this function may therefore not cover the entire font glyphset.
    #[inline]
    pub fn glyphs(&self) -> &PdfFontGlyphs<'_> {
        self.glyphs.initialize_len();
        &self.glyphs
    }
}

impl<'a> Drop for PdfFont<'a> {
    /// Closes this [PdfFont], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        // The documentation for FPDFText_LoadFont() and FPDFText_LoadStandardFont() both state
        // that the font loaded by the function can be closed by calling FPDFFont_Close().
        // I had taken this to mean that _any_ FPDF_Font handle returned from a Pdfium function
        // should be closed via FPDFFont_Close(), but testing suggests this is not the case;
        // rather, it is only fonts specifically loaded by calling FPDFText_LoadFont() or
        // FPDFText_LoadStandardFont() that need to be actively closed.

        // In other words, retrieving a handle to a font that already exists in a document evidently
        // does not allocate any additional resources, so we don't need to free anything.
        // (Indeed, if we try to, Pdfium segfaults.)

        if self.is_font_memory_loaded {
            self.bindings().FPDFFont_Close(self.handle);
        }
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfFont<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfFont<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfFont<'a> {}
