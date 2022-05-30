//! Defines the [PdfFont] struct, exposing functionality related to a single font used to
//! render text in a `PdfDocument`.

use crate::bindgen::{FPDF_FONT, FPDF_FONT_TRUETYPE, FPDF_FONT_TYPE1};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::utils::mem::create_byte_buffer;
use std::os::raw::{c_char, c_int, c_uint};

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
    handle: FPDF_FONT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFont<'a> {
    #[inline]
    pub(crate) fn from_pdfium(handle: FPDF_FONT, bindings: &'a dyn PdfiumLibraryBindings) -> Self {
        PdfFont { handle, bindings }
    }

    /// Creates a new [PdfFont] from the given given built-in font argument.
    #[inline]
    pub fn new_built_in(document: &'a PdfDocument<'a>, font: PdfFontBuiltin) -> Self {
        Self::from_pdfium(
            document
                .get_bindings()
                .FPDFText_LoadStandardFont(*document.get_handle(), font.to_pdf_font_name()),
            document.get_bindings(),
        )
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
            Ok(PdfFont::from_pdfium(handle, bindings))
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
            // Trim any trailing nulls. All non-UTF-16LE strings returned from Pdfium are
            // generally terminated by one null byte.
            .map(|str| str.trim_end_matches(char::from(0)).to_owned())
            .unwrap_or_else(|_| String::new())
    }

    /// Returns the weight of this [PdfFont].
    pub fn weight(&self) -> Result<PdfFontWeight, PdfiumError> {
        PdfFontWeight::from_pdfium(self.bindings.FPDFFont_GetWeight(self.handle)).ok_or_else(|| {
            PdfiumError::PdfiumLibraryInternalError(
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            )
        })
    }
}
