//! Defines the [PdfPageTextObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Text`.

use crate::bindgen::{
    FPDF_DOCUMENT, FPDF_FONT, FPDF_PAGE, FPDF_PAGEOBJECT, FPDF_TEXT_RENDERMODE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_CLIP, FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_FILL_STROKE_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_INVISIBLE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_STROKE_CLIP,
    FPDF_TEXT_RENDERMODE_FPDF_TEXTRENDERMODE_UNKNOWN, FPDF_WCHAR,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::font::PdfFont;
use crate::page::PdfPoints;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;

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

    /// The text will be stroked and added to the path for clipping.
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
    #[allow(dead_code)]
    // The to_pdfium() function is not currently used, but we expect it to be in future
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

/// A single `PdfPageObject` of type `PdfPageObjectType::Text`.
///
/// Page objects can be created either attached to a `PdfPage` (in which case the page object's
/// memory is owned by the containing page) or detached from any page (in which case the page
/// object's memory is owned by the object). Page objects are not rendered until they are
/// attached to a page; page objects that are never attached to a page will be lost when they
/// fall out of scope.
///
/// The simplest way to create a page text object that is immediately attached to a page
/// is to call the `PdfPage::objects_mut().create_text_object()` function.
///
/// To create a detached page text object, use the [PdfPageTextObject::new()] function.
/// The detached page text object can later be attached to a page by calling
/// `PdfPage::objects_mut().add_object()` with the detached page object as the parameter.
pub struct PdfPageTextObject<'a> {
    is_object_memory_owned_by_page: bool,
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageTextObject<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageTextObject {
            is_object_memory_owned_by_page: true,
            object_handle,
            page_handle: Some(page_handle),
            bindings,
        }
    }

    // Take raw FPDF_DOCUMENT and FPDF_FONT handles to avoid cascading lifetime problems
    // associated with borrowing PdfDocument<'a> and/or PdfFont<'a>.
    pub(crate) fn new_from_handles(
        document: FPDF_DOCUMENT,
        text: impl ToString,
        font: FPDF_FONT,
        font_size: PdfPoints,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Result<Self, PdfiumError> {
        let handle = bindings.FPDFPageObj_CreateTextObj(document, font, font_size.value);

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
            let mut result = PdfPageTextObject {
                is_object_memory_owned_by_page: false,
                object_handle: handle,
                page_handle: None,
                bindings,
            };

            result.set_text(text)?;

            Ok(result)
        }
    }

    /// Creates a new [PdfPageTextObject] from the given arguments. The returned page object
    /// will not be rendered until it is added to a [PdfPage] using the
    /// `PdfPageObjects::add_text_object()` function.
    ///
    /// A single space will be used if the given text is empty, in order to avoid
    /// unexpected behaviour from Pdfium when dealing with an empty string.
    // Specifically, FPDFPageObj_SetText() will crash if you try to set an empty string on a
    // text object, and FPDFText_LoadPage() will crash if any text object on the page contains
    // an empty string (so it isn't enough to avoid calling FPDFPageObj_SetText() for an empty
    // text object, you _have_ to set a non-empty string to avoid segfaults).
    #[inline]
    pub fn new(
        document: &PdfDocument<'a>,
        text: impl ToString,
        font: &PdfFont,
        font_size: PdfPoints,
    ) -> Result<Self, PdfiumError> {
        Self::new_from_handles(
            *document.get_handle(),
            text,
            *font.get_handle(),
            font_size,
            document.get_bindings(),
        )
    }

    /// Returns the text contained within this [PdfPageTextObject].
    ///
    /// Text retrieval in Pdfium is handled by the `PdfPageText` object owned by the [PdfPage]
    /// containing this [PdfPageTextObject]. If this text object has not been placed on a page,
    /// or the page has no associated `PdfPageText` object, then text retrieval is not available
    /// and an empty string will be returned.
    pub fn text(&self) -> String {
        // Retrieving the text from Pdfium is a two-step operation. First, we call
        // FPDFTextObj_GetText() with a null buffer; this will retrieve the length of
        // the text in bytes. If the length is zero, then there is no text associated
        // with the page object.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFTextObj_GetText() again with a pointer to the buffer;
        // this will write the text to the buffer in UTF16-LE format.

        if let Some(page_handle) = self.page_handle {
            let text_handle = self.bindings.FPDFText_LoadPage(page_handle);

            if !text_handle.is_null() {
                let buffer_length = self.get_bindings().FPDFTextObj_GetText(
                    self.object_handle,
                    text_handle,
                    std::ptr::null_mut(),
                    0,
                );

                if buffer_length == 0 {
                    // There is no text.

                    return String::new();
                }

                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.get_bindings().FPDFTextObj_GetText(
                    self.object_handle,
                    text_handle,
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                assert_eq!(result, buffer_length);

                self.bindings.FPDFText_ClosePage(text_handle);

                get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default()
            } else {
                // The PdfPage containing this page object does not have an associated
                // FPDF_TEXTPAGE object.

                String::new()
            }
        } else {
            // This page object is not contained by a PdfPage.

            String::new()
        }
    }

    /// Returns the text rendering mode for the text contained within this [PdfPageTextObject].
    pub fn render_mode(&self) -> PdfPageTextRenderMode {
        PdfPageTextRenderMode::from_pdfium(
            self.get_bindings()
                .FPDFTextObj_GetTextRenderMode(self.object_handle) as u32,
        )
        .unwrap_or(PdfPageTextRenderMode::Unknown)
    }

    /// Returns the font size of the text contained within this [PdfPageTextObject],
    /// expressed in [PdfPoints].
    pub fn font_size(&self) -> PdfPoints {
        let mut result = 0.0;

        if self.get_bindings().is_true(
            self.get_bindings()
                .FPDFTextObj_GetFontSize(self.object_handle, &mut result),
        ) {
            PdfPoints::new(result)
        } else {
            PdfPoints::ZERO
        }
    }

    /// Returns the [PdfFont] used to render the text contained within this [PdfPageTextObject].
    pub fn font(&self) -> PdfFont {
        PdfFont::from_pdfium(
            self.get_bindings().FPDFTextObj_GetFont(self.object_handle),
            self.get_bindings(),
        )
    }

    /// Sets the text contained within this [PdfPageTextObject], replacing any existing text.
    ///
    /// A single space will be used if the given text is empty, in order to avoid
    /// unexpected behaviour from Pdfium when dealing with an empty string.
    pub fn set_text(&mut self, text: impl ToString) -> Result<(), PdfiumError> {
        let text = text.to_string();

        let text = if text.is_empty() { " " } else { text.as_str() };

        if self.get_bindings().is_true(
            self.get_bindings()
                .FPDFText_SetText_str(self.object_handle, text),
        ) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                self.get_bindings()
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            ))
        }
    }

    /// Sets the text rendering mode for the text contained within this [PdfPageTextObject].
    pub fn set_render_mode(&mut self, render_mode: PdfPageTextRenderMode) {
        self.get_bindings()
            .FPDFTextObj_SetTextRenderMode(self.object_handle, render_mode.to_pdfium());
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageTextObject<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.object_handle
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn is_object_memory_owned_by_page(&self) -> bool {
        self.is_object_memory_owned_by_page
    }

    #[inline]
    fn set_object_memory_owned_by_page(&mut self, page: FPDF_PAGE) {
        self.page_handle = Some(page);
        self.is_object_memory_owned_by_page = true;
    }

    #[inline]
    fn set_object_memory_released_by_page(&mut self) {
        self.page_handle = None;
        self.is_object_memory_owned_by_page = false;
    }
}
