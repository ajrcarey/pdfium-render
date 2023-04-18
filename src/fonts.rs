use crate::bindgen::{FPDF_DOCUMENT, FPDF_FONT_TRUETYPE, FPDF_FONT_TYPE1};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::font::PdfFont;
use std::fs::File;
use std::io::Read;
use std::os::raw::{c_int, c_uint};
use std::path::Path;

// The following dummy declaration is used only when running cargo doc.
// It allows documentation of WASM-specific functionality to be included
// in documentation generated on non-WASM targets.

#[cfg(doc)]
struct Blob;

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

pub struct PdfFonts<'a> {
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFonts<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFonts {
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFonts] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Creates a new [PdfFont] instance representing the given built-in font.
    #[inline]
    pub fn new_built_in(&self, font: PdfFontBuiltin) -> PdfFont {
        PdfFont::from_pdfium(
            self.bindings
                .FPDFText_LoadStandardFont(self.document_handle, font.to_pdf_font_name()),
            self.bindings,
            Some(font),
            true,
        )
    }

    /// Creates a new [PdfFont] instance representing the built-in "Times-Roman" font.
    #[inline]
    pub fn times_roman(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::TimesRoman)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Times-Bold" font.
    #[inline]
    pub fn times_bold(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::TimesBold)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Times-Italic" font.
    #[inline]
    pub fn times_italic(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::TimesItalic)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Times-BoldItalic" font.
    #[inline]
    pub fn times_bold_italic(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::TimesBoldItalic)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Helvetica" font.
    #[inline]
    pub fn helvetica(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::Helvetica)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Helvetica-Bold" font.
    #[inline]
    pub fn helvetica_bold(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::HelveticaBold)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Helvetica-Oblique" font.
    #[inline]
    pub fn helvetica_oblique(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::HelveticaOblique)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Helvetica-BoldOblique" font.
    #[inline]
    pub fn helvetica_bold_oblique(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::HelveticaBoldOblique)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Courier" font.
    #[inline]
    pub fn courier(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::Courier)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Courier-Bold" font.
    #[inline]
    pub fn courier_bold(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::CourierBold)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Courier-Oblique" font.
    #[inline]
    pub fn courier_oblique(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::CourierOblique)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Courier-BoldOblique" font.
    #[inline]
    pub fn courier_bold_oblique(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::CourierBoldOblique)
    }

    /// Creates a new [PdfFont] instance representing the built-in "Symbol" font.
    #[inline]
    pub fn symbol(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::Symbol)
    }

    /// Creates a new [PdfFont] instance representing the built-in "ZapfDingbats" font.
    #[inline]
    pub fn zapf_dingbats(&self) -> PdfFont {
        self.new_built_in(PdfFontBuiltin::ZapfDingbats)
    }

    /// Attempts to load a Type 1 font file from the given file path.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading font data in WASM:
    /// * Use the [PdfFont::load_type1_from_fetch()] function to download font data from a
    /// URL using the browser's built-in `fetch()` API. This function is only available when
    /// compiling to WASM.
    /// * Use the [PdfFont::load_type1_from_blob()] function to load font data from a
    /// Javascript File or Blob object (such as a File object returned from an HTML
    /// `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use the [PdfFont::load_type1_from_reader()] function to load font data from any
    /// valid Rust reader.
    /// * Use another method to retrieve the bytes of the target font over the network,
    /// then load those bytes into Pdfium using the [PdfFont::new_type1_from_bytes()] function.
    /// * Embed the bytes of the desired font directly into the compiled WASM module
    /// using the `include_bytes!()` macro.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_type1_from_file(
        &self,
        path: &(impl AsRef<Path> + ?Sized),
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        self.load_type1_from_reader(File::open(path).map_err(PdfiumError::IoError)?, is_cid_font)
    }

    /// Attempts to load a Type 1 font file from the given reader.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn load_type1_from_reader(
        &self,
        mut reader: impl Read,
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        let mut bytes = Vec::new();

        reader
            .read_to_end(&mut bytes)
            .map_err(PdfiumError::IoError)?;

        self.load_type1_from_bytes(bytes.as_slice(), is_cid_font)
    }

    /// Attempts to load a Type 1 font file from the given URL.
    /// The Javascript `fetch()` API is used to download data over the network.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub async fn load_type1_from_fetch(
        &self,
        url: impl ToString,
        is_cid_font: bool,
    ) -> Result<PdfFont<'a>, PdfiumError> {
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

            self.load_type1_from_blob(blob, is_cid_font).await
        } else {
            Err(PdfiumError::WebSysWindowObjectNotAvailable)
        }
    }

    /// Attempts to load a Type 1 font from the given Blob.
    /// A File object returned from a FileList is a suitable Blob:
    ///
    /// ```text
    /// <input id="filePicker" type="file">
    ///
    /// const file = document.getElementById('filePicker').files[0];
    /// ```
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub async fn load_type1_from_blob(
        &self,
        blob: Blob,
        is_cid_font: bool,
    ) -> Result<PdfFont<'a>, PdfiumError> {
        let array_buffer: ArrayBuffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(PdfiumError::WebSysFetchError)?
            .into();

        let u8_array: Uint8Array = Uint8Array::new(&array_buffer);

        let bytes: Vec<u8> = u8_array.to_vec();

        self.load_type1_from_bytes(bytes.as_slice(), is_cid_font)
    }

    /// Attempts to load the given byte data as a Type 1 font file.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn load_type1_from_bytes(
        &self,
        font_data: &[u8],
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        self.new_font_from_bytes(font_data, FPDF_FONT_TYPE1, is_cid_font)
    }

    /// Attempts to load a TrueType font file from the given file path.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading font data in WASM:
    /// * Use the [PdfFont::load_true_type_from_fetch()] function to download font data from a
    /// URL using the browser's built-in `fetch()` API. This function is only available when
    /// compiling to WASM.
    /// * Use the [PdfFont::load_true_type_from_blob()] function to load font data from a
    /// Javascript `File` or `Blob` object (such as a `File` object returned from an HTML
    /// `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use the [PdfFont::load_true_type_from_reader()] function to load font data from any
    /// valid Rust reader.
    /// * Use another method to retrieve the bytes of the target font over the network,
    /// then load those bytes into Pdfium using the [PdfFont::new_true_type_from_bytes()] function.
    /// * Embed the bytes of the desired font directly into the compiled WASM module
    /// using the `include_bytes!()` macro.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_true_type_from_file(
        &self,
        path: &(impl AsRef<Path> + ?Sized),
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        self.load_true_type_from_reader(
            File::open(path).map_err(PdfiumError::IoError)?,
            is_cid_font,
        )
    }

    /// Attempts to load a TrueType font file from the given reader.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn load_true_type_from_reader(
        &self,
        mut reader: impl Read,
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        let mut bytes = Vec::new();

        reader
            .read_to_end(&mut bytes)
            .map_err(PdfiumError::IoError)?;

        self.load_true_type_from_bytes(bytes.as_slice(), is_cid_font)
    }

    /// Attempts to load a TrueType font file from the given URL.
    /// The Javascript `fetch()` API is used to download data over the network.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub async fn load_true_type_from_fetch(
        &self,
        url: impl ToString,
        is_cid_font: bool,
    ) -> Result<PdfFont<'a>, PdfiumError> {
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

            self.load_true_type_from_blob(blob, is_cid_font).await
        } else {
            Err(PdfiumError::WebSysWindowObjectNotAvailable)
        }
    }

    /// Attempts to load a TrueType font from the given `Blob`.
    /// A `File` object returned from a `FileList` is a suitable `Blob`:
    ///
    /// ```text
    /// <input id="filePicker" type="file">
    ///
    /// const file = document.getElementById('filePicker').files[0];
    /// ```
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub async fn load_true_type_from_blob(
        &self,
        blob: Blob,
        is_cid_font: bool,
    ) -> Result<PdfFont<'a>, PdfiumError> {
        let array_buffer: ArrayBuffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(PdfiumError::WebSysFetchError)?
            .into();

        let u8_array: Uint8Array = Uint8Array::new(&array_buffer);

        let bytes: Vec<u8> = u8_array.to_vec();

        self.load_true_type_from_bytes(bytes.as_slice(), is_cid_font)
    }

    /// Attempts to load the given byte data as a TrueType font file.
    ///
    /// Set the `is_cid_font` parameter to `true` if the given font is keyed by
    /// 16-bit character ID (CID), indicating that it supports an extended glyphset of
    /// 65,535 glyphs. This is typically the case with fonts that support Asian character sets
    /// or right-to-left languages.
    pub fn load_true_type_from_bytes(
        &self,
        font_data: &[u8],
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        self.new_font_from_bytes(font_data, FPDF_FONT_TRUETYPE, is_cid_font)
    }

    #[inline]
    pub(crate) fn new_font_from_bytes(
        &self,
        font_data: &[u8],
        font_type: c_uint,
        is_cid_font: bool,
    ) -> Result<PdfFont, PdfiumError> {
        let handle = self.bindings.FPDFText_LoadFont(
            self.document_handle,
            font_data.as_ptr(),
            font_data.len() as c_uint,
            font_type as c_int,
            self.bindings.bool_to_pdfium(is_cid_font),
        );

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfFont::from_pdfium(handle, self.bindings, None, true))
        }
    }
}
