//! Defines the [PdfFont] struct, exposing functionality related to a single font used to
//! render text in a `PdfDocument`.

use crate::bindgen::{FPDF_FONT, FPDF_FONT_TRUETYPE, FPDF_FONT_TYPE1};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::utils::mem::create_byte_buffer;
use std::os::raw::{c_char, c_int, c_uint};

/// The 14 built-in fonts provided as part of the PDF specification.
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

/// A single font used to render text in a `PdfDocument`.
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

    /// Loads the given built-in PDF font.
    #[inline]
    pub fn new_built_in(document: PdfDocument<'a>, font: PdfFontBuiltin) -> Self {
        Self::from_pdfium(
            document
                .get_bindings()
                .FPDFText_LoadStandardFont(*document.get_handle(), font.to_pdf_font_name()),
            document.get_bindings(),
        )
    }

    /// Attempts to load the given byte data as a Type 1 font file.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn new_type1_from_bytes(
        document: PdfDocument<'a>,
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
        document: PdfDocument<'a>,
        font_data: &[u8],
        is_cid_font: bool,
    ) -> Result<Self, PdfiumError> {
        Self::new_font_from_bytes(document, font_data, FPDF_FONT_TRUETYPE, is_cid_font)
    }

    #[inline]
    pub(crate) fn new_font_from_bytes(
        document: PdfDocument<'a>,
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
}
