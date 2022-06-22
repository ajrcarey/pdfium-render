//! Defines the [PdfFont] struct, exposing functionality related to a single font used to
//! render text in a `PdfDocument`.

use crate::bindgen::{FPDF_FONT, FPDF_FONT_TRUETYPE, FPDF_FONT_TYPE1};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPoints;
use crate::utils::mem::create_byte_buffer;
use bitflags::bitflags;
use std::os::raw::{c_char, c_int, c_uint};

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

/// The 14 built-in fonts provided as part of the PDF specification.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfFontBuiltin {
    TimesRoman,
    TimesBold,
    TimesItalic,
    TimesBoldItalic,
    Helvetica,
    HelveticaBold,
    HelveticaOblique,
    HelveticaBoldOblique,
    Courier,
    CourierBold,
    CourierOblique,
    CourierBoldOblique,
    Symbol,
    ZapfDingbats,
}

impl PdfFontBuiltin {
    /// Returns the PostScript name of this built-in PDF font, as listed on page 416
    /// of the PDF 1.7 specification.
    pub fn to_pdf_font_name(&self) -> &str {
        match self {
            PdfFontBuiltin::TimesRoman => "Times-Roman",
            PdfFontBuiltin::TimesBold => "Times-Bold",
            PdfFontBuiltin::TimesItalic => "Times-Italic",
            PdfFontBuiltin::TimesBoldItalic => "Times-BoldItalic",
            PdfFontBuiltin::Helvetica => "Helvetica",
            PdfFontBuiltin::HelveticaBold => "Helvetica-Bold",
            PdfFontBuiltin::HelveticaOblique => "Helvetica-Oblique",
            PdfFontBuiltin::HelveticaBoldOblique => "Helvetica-BoldOblique",
            PdfFontBuiltin::Courier => "Courier",
            PdfFontBuiltin::CourierBold => "Courier-Bold",
            PdfFontBuiltin::CourierOblique => "Courier-Oblique",
            PdfFontBuiltin::CourierBoldOblique => "Courier-BoldOblique",
            PdfFontBuiltin::Symbol => "Symbol",
            PdfFontBuiltin::ZapfDingbats => "ZapfDingbats",
        }
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
    bindings: &'a dyn PdfiumLibraryBindings,
    is_font_memory_loaded: bool,
}

impl<'a> PdfFont<'a> {
    #[inline]
    pub(crate) fn from_pdfium(handle: FPDF_FONT, bindings: &'a dyn PdfiumLibraryBindings) -> Self {
        PdfFont {
            built_in: None,
            handle,
            bindings,
            is_font_memory_loaded: false,
        }
    }

    /// Creates a new [PdfFont] from the given given built-in font argument.
    #[inline]
    pub fn new_built_in(document: &'a PdfDocument<'a>, font: PdfFontBuiltin) -> Self {
        let mut result = Self::from_pdfium(
            document
                .get_bindings()
                .FPDFText_LoadStandardFont(*document.get_handle(), font.to_pdf_font_name()),
            document.get_bindings(),
        );

        result.built_in = Some(font);
        result.is_font_memory_loaded = true;

        result
    }

    /// Creates a new [PdfFont] for the built-in "Times-Roman" font.
    #[inline]
    pub fn times_roman(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::TimesRoman)
    }

    /// Creates a new [PdfFont] for the built-in "Times-Bold" font.
    #[inline]
    pub fn times_bold(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::TimesBold)
    }

    /// Creates a new [PdfFont] for the built-in "Times-Italic" font.
    #[inline]
    pub fn times_italic(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::TimesItalic)
    }

    /// Creates a new [PdfFont] for the built-in "Times-BoldItalic" font.
    #[inline]
    pub fn times_bold_italic(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::TimesBoldItalic)
    }

    /// Creates a new [PdfFont] for the built-in "Helvetica" font.
    #[inline]
    pub fn helvetica(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::Helvetica)
    }

    /// Creates a new [PdfFont] for the built-in "Helvetica-Bold" font.
    #[inline]
    pub fn helvetica_bold(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::HelveticaBold)
    }

    /// Creates a new [PdfFont] for the built-in "Helvetica-Oblique" font.
    #[inline]
    pub fn helvetica_oblique(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::HelveticaOblique)
    }

    /// Creates a new [PdfFont] for the built-in "Helvetica-BoldOblique" font.
    #[inline]
    pub fn helvetica_bold_oblique(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::HelveticaBoldOblique)
    }

    /// Creates a new [PdfFont] for the built-in "Courier" font.
    #[inline]
    pub fn courier(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::Courier)
    }

    /// Creates a new [PdfFont] for the built-in "Courier-Bold" font.
    #[inline]
    pub fn courier_bold(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::CourierBold)
    }

    /// Creates a new [PdfFont] for the built-in "Courier-Oblique" font.
    #[inline]
    pub fn courier_oblique(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::CourierOblique)
    }

    /// Creates a new [PdfFont] for the built-in "Courier-BoldOblique" font.
    #[inline]
    pub fn courier_bold_oblique(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::CourierBoldOblique)
    }

    /// Creates a new [PdfFont] for the built-in "Symbol" font.
    #[inline]
    pub fn symbol(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::Symbol)
    }

    /// Creates a new [PdfFont] for the built-in "ZapfDingbats" font.
    #[inline]
    pub fn zapf_dingbats(document: &'a PdfDocument<'a>) -> Self {
        Self::new_built_in(document, PdfFontBuiltin::ZapfDingbats)
    }

    /// Attempts to load the given byte data as a Type 1 font file.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn new_type1_from_bytes(
        document: &'a PdfDocument<'a>,
        font_data: &[u8],
        is_cid_font: bool,
    ) -> Result<Self, PdfiumError> {
        Self::new_font_from_bytes(document, font_data, FPDF_FONT_TYPE1, is_cid_font)
    }

    /// Attempts to load the given byte data as a TrueType font file.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn new_true_type_from_bytes(
        document: &'a PdfDocument<'a>,
        font_data: &[u8],
        is_cid_font: bool,
    ) -> Result<Self, PdfiumError> {
        Self::new_font_from_bytes(document, font_data, FPDF_FONT_TRUETYPE, is_cid_font)
    }

    #[inline]
    pub(crate) fn new_font_from_bytes(
        document: &'a PdfDocument<'a>,
        font_data: &[u8],
        font_type: c_uint,
        is_cid_font: bool,
    ) -> Result<Self, PdfiumError> {
        let bindings = document.get_bindings();

        let handle = bindings.FPDFText_LoadFont(
            *document.get_handle(),
            font_data.as_ptr(),
            font_data.len() as c_uint,
            font_type as c_int,
            bindings.bool_to_pdfium(is_cid_font),
        );

        if handle.is_null() {
            if let Some(error) = bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            let mut result = PdfFont::from_pdfium(handle, bindings);

            result.is_font_memory_loaded = true;

            Ok(result)
        }
    }

    /// Returns the internal FPDF_FONT handle for this [PdfFont].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_FONT {
        &self.handle
    }

    /// Returns the name of this [PdfFont].
    pub fn name(&self) -> String {
        // Retrieving the font name from Pdfium is a two-step operation. First, we call
        // FPDFFont_GetFontName() with a null buffer; this will retrieve the length of
        // the font name in bytes. If the length is zero, then there is no font name.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFFont_GetFontName() again with a pointer to the buffer;
        // this will write the font name into the buffer. Unlike most text handling in
        // Pdfium, font names are returned in UTF-8 format.

        let buffer_length =
            self.bindings
                .FPDFFont_GetFontName(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // The font name is not present.

            return String::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFFont_GetFontName(
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
        PdfFontWeight::from_pdfium(self.bindings.FPDFFont_GetWeight(self.handle)).ok_or_else(|| {
            PdfiumError::PdfiumLibraryInternalError(
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            )
        })
    }

    /// Returns the italic angle of this [PdfFont]. The italic angle is the angle,
    /// expressed in degrees counter-clock-wise from the vertical, of the dominant vertical
    /// strokes of the font. The value is zero for non-italic fonts, and negative for fonts
    /// that slope to the right (as almost all italic fonts do).
    ///
    /// Pdfium may not reliably return the correct value of this property for built-in fonts.
    pub fn italic_angle(&self) -> Result<i32, PdfiumError> {
        let mut angle = 0;

        if self.bindings.is_true(
            self.bindings
                .FPDFFont_GetItalicAngle(self.handle, &mut angle),
        ) {
            Ok(angle)
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            ))
        }
    }

    /// Returns the ascent of this [PdfFont] for the given font size. The ascent is the maximum
    /// height above the baseline reached by glyphs in this font, excluding the height of glyphs
    /// for accented characters.
    pub fn ascent(&self, font_size: PdfPoints) -> Result<PdfPoints, PdfiumError> {
        let mut ascent = 0.0;

        if self.bindings.is_true(self.bindings.FPDFFont_GetAscent(
            self.handle,
            font_size.value,
            &mut ascent,
        )) {
            Ok(PdfPoints::new(ascent))
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            ))
        }
    }

    /// Returns the descent of this [PdfFont] for the given font size. The descent is the
    /// maximum distance below the baseline reached by glyphs in this font, expressed as a
    /// negative number.
    pub fn descent(&self, font_size: PdfPoints) -> Result<PdfPoints, PdfiumError> {
        let mut descent = 0.0;

        if self.bindings.is_true(self.bindings.FPDFFont_GetDescent(
            self.handle,
            font_size.value,
            &mut descent,
        )) {
            Ok(PdfPoints::new(descent))
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            ))
        }
    }

    /// Returns the raw font descriptor bitflags for the containing [PdfFont].
    #[inline]
    fn get_flags_bits(&self) -> FpdfFontDescriptorFlags {
        FpdfFontDescriptorFlags::from_bits_truncate(
            self.bindings.FPDFFont_GetFlags(self.handle) as u32
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
            self.bindings.FPDFFont_Close(self.handle);
        }
    }
}
