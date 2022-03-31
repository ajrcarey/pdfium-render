//! Defines the [PdfPageTextObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Text`.

use crate::bindgen::{
    FPDF_PAGEOBJECT, FPDF_TEXT_RENDERMODE, FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN, FPDF_WCHAR,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::font::PdfFont;
use crate::page::{PdfPage, PdfPoints};
use crate::page_object::internal::PdfPageObjectPrivate;
use crate::page_objects::PdfPageObjectIndex;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::os::raw::c_float;

/// The text rendering modes supported by the PDF standard, as listed in table 5.3
/// on page 402 in the PDF Reference manual version 1.7.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfPageTextRenderMode {
    /// The text render mode is not recognized by Pdfium.
    Unknown = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN as isize,

    /// The text will be filled, but not stroked.
    FilledUnstroked = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL as isize,

    /// The text will be stroked, but not filled.
    StrokedUnfilled = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE as isize,

    /// The text will be filled, then stroked.
    FilledThenStroked = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE as isize,

    /// The text will be neither filled nor stroked. It will still take up size in the layout, however.
    Invisible = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE as isize,

    /// The text will be filled and added to the path for clipping.
    FilledUnstrokedClipping = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP as isize,

    /// THe text will be stroked and added to the path for clipping.
    StrokedUnfilledClipping = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP as isize,

    /// The text will be filled, then stroked, and added to the path for clipping.
    FilledThenStrokedClipping = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP as isize,

    /// The text will be neither filled nor stroked, only added to the path for clipping.
    InvisibleClipping = FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP as isize,
}

impl PdfPageTextRenderMode {
    #[inline]
    pub(crate) fn from_pdfium(value: u32) -> Result<PdfPageTextRenderMode, PdfiumError> {
        match value as i32 {
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN => Ok(PdfPageTextRenderMode::Unknown),
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL => {
                Ok(PdfPageTextRenderMode::FilledUnstroked)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE => {
                Ok(PdfPageTextRenderMode::StrokedUnfilled)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE => {
                Ok(PdfPageTextRenderMode::FilledThenStroked)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE => {
                Ok(PdfPageTextRenderMode::Invisible)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP => {
                Ok(PdfPageTextRenderMode::FilledUnstrokedClipping)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP => {
                Ok(PdfPageTextRenderMode::StrokedUnfilledClipping)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP => {
                Ok(PdfPageTextRenderMode::FilledThenStrokedClipping)
            }
            FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP => {
                Ok(PdfPageTextRenderMode::InvisibleClipping)
            }
            _ => Err(PdfiumError::UnknownPdfPageTextRenderMode),
        }
    }

    #[inline]
    pub(crate) fn to_pdfium(self) -> FPDF_TEXT_RENDERMODE {
        match self {
            PdfPageTextRenderMode::Unknown => FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN,
            PdfPageTextRenderMode::FilledUnstroked => FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL,
            PdfPageTextRenderMode::StrokedUnfilled => {
                FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE
            }
            PdfPageTextRenderMode::FilledThenStroked => {
                FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE
            }
            PdfPageTextRenderMode::Invisible => FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE,
            PdfPageTextRenderMode::FilledUnstrokedClipping => {
                FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP
            }
            PdfPageTextRenderMode::StrokedUnfilledClipping => {
                FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP
            }
            PdfPageTextRenderMode::FilledThenStrokedClipping => {
                FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP
            }
            PdfPageTextRenderMode::InvisibleClipping => {
                FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP
            }
        }
    }
}

pub struct PdfPageTextObject<'a> {
    index: PdfPageObjectIndex,
    is_object_memory_owned_by_page: bool,
    handle: FPDF_PAGEOBJECT,
    page: &'a PdfPage<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageTextObject<'a> {
    pub(crate) fn from_pdfium(
        index: PdfPageObjectIndex,
        handle: FPDF_PAGEOBJECT,
        page: &'a PdfPage<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageTextObject {
            index,
            is_object_memory_owned_by_page: true,
            handle,
            page,
            bindings,
        }
    }

    /// Returns the text contained within this [PdfPageTextObject].
    pub fn text(&self) -> String {
        // Retrieving the text from Pdfium is a two-step operation. First, we call
        // FPDFTextObj_GetText() with a null buffer; this will retrieve the length of
        // the text in bytes. If the length is zero, then there is no text associated
        // with the page object.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFTextObj_GetText() again with a pointer to the buffer;
        // this will write the text to the buffer in UTF16-LE format.

        if let Ok(page_text) = self.page.text() {
            let buffer_length = self.bindings.FPDFTextObj_GetText(
                self.handle,
                *page_text.get_handle(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // There is no text.

                return String::new();
            }

            let mut buffer = create_byte_buffer(buffer_length as usize);

            let result = self.bindings.FPDFTextObj_GetText(
                self.handle,
                *page_text.get_handle(),
                buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                buffer_length,
            );

            assert_eq!(result, buffer_length);

            get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default()
        } else {
            // The containing PdfPage does not have an FPDF_TEXTPAGE associated with it.

            String::new()
        }
    }

    /// Returns the text rendering mode for the text contained within this [PdfPageTextObject].
    pub fn render_mode(&self) -> PdfPageTextRenderMode {
        PdfPageTextRenderMode::from_pdfium(
            self.bindings.FPDFTextObj_GetTextRenderMode(self.handle) as u32
        )
        .unwrap_or(PdfPageTextRenderMode::Unknown)
    }

    /// Sets the text rendering mode for the text contained within this [PdfPageTextObject].
    pub fn set_render_mode(&mut self, render_mode: PdfPageTextRenderMode) {
        self.bindings
            .FPDFTextObj_SetTextRenderMode(self.handle, render_mode.to_pdfium());
    }

    /// Returns the font size of the text contained within this [PdfPageTextObject],
    /// expressed in [PdfPoints].
    pub fn font_size(&self) -> PdfPoints {
        // Clippy doesn't want us to cast to c_float because c_float == f32 in the
        // development environment, but we don't want to assume that will be the case
        // on every target architecture.
        #[allow(clippy::unnecessary_cast)]
        let mut result = 0.0 as c_float;

        if self.bindings.is_true(
            self.bindings
                .FPDFTextObj_GetFontSize(self.handle, &mut result),
        ) {
            PdfPoints::new(result)
        } else {
            PdfPoints::ZERO
        }
    }

    /// Returns the [PdfFont] used to render the text contained within this [PdfPageTextObject].
    pub fn font(&self) -> PdfFont {
        PdfFont::from_pdfium(
            self.bindings.FPDFTextObj_GetFont(self.handle),
            self.bindings,
        )
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageTextObject<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.handle
    }

    #[inline]
    fn index_impl(&self) -> PdfPageObjectIndex {
        self.index
    }

    #[inline]
    fn is_object_memory_owned_by_page(&self) -> bool {
        self.is_object_memory_owned_by_page
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
