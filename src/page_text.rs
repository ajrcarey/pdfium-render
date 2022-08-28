//! Defines the [PdfPageText] struct, exposing functionality related to the
//! collection of Unicode characters visible in a single `PdfPage`.

use crate::bindgen::{FPDF_TEXTPAGE, FPDF_WCHAR};
use crate::bindings::PdfiumLibraryBindings;
use crate::page::{PdfPage, PdfRect};
use crate::page_annotation::PdfPageAnnotation;
use crate::page_annotation::PdfPageAnnotationCommon;
use crate::page_object::PdfPageObjectCommon;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_object_text::PdfPageTextObject;
use crate::page_text_chars::PdfPageTextChars;
use crate::page_text_segments::PdfPageTextSegments;
use crate::prelude::PdfiumError;
use crate::utils::mem::{create_byte_buffer, create_sized_buffer};
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use image::EncodableLayout;
use std::fmt::{Display, Formatter};
use std::ptr::null_mut;

/// The collection of Unicode characters visible in a single [PdfPage].
///
/// Use the [PdfPageText::all()] function to easily return all characters in the containing
/// [PdfPage] in the order in which they are defined in the PDF file.
///
/// In complex custom layouts, the order in which characters are defined in the document
/// and the order in which they appear visually during rendering (and thus the order in
/// which they are read by a user) may not necessarily match.
///
/// [PdfPageText] implements both the [ToString] and the [Display] traits.
pub struct PdfPageText<'a> {
    handle: FPDF_TEXTPAGE,
    page: &'a PdfPage<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageText<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_TEXTPAGE,
        page: &'a PdfPage<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageText {
            handle,
            page,
            bindings,
        }
    }

    /// Returns the internal `FPDF_TEXTPAGE` handle for this [PdfPageText].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_TEXTPAGE {
        &self.handle
    }

    /// Returns the total number of characters in all text segments in the containing [PdfPage].
    ///
    /// The character count includes whitespace and newlines, and so may differ slightly
    /// from the result of calling `PdfPageText::all().len()`.
    #[inline]
    pub fn len(&self) -> i32 {
        self.bindings.FPDFText_CountChars(self.handle)
    }

    /// Returns `true` if there are no characters in any text box collection in the containing [PdfPage].
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a collection of all the `PdfPageTextSegment` text segments in the containing [PdfPage].
    #[inline]
    pub fn segments(&self) -> PdfPageTextSegments {
        PdfPageTextSegments::new(self, self.len(), self.bindings)
    }

    /// Returns a collection of all the `PdfPageTextChar` characters in the containing [PdfPage].
    #[inline]
    pub fn chars(&self) -> PdfPageTextChars {
        PdfPageTextChars::new(self, 0, self.len(), self.bindings)
    }

    /// Returns a collection of all the `PdfPageTextChar` characters in the given [PdfPageTextObject].
    ///
    /// The return result will be empty if the given [PdfPageTextObject] is not attached to the
    /// containing [PdfPage].
    #[inline]
    pub fn chars_for_object(
        &self,
        object: &PdfPageTextObject,
    ) -> Result<PdfPageTextChars, PdfiumError> {
        self.chars_inside_rect(object.bounds()?)
            .map_err(|_| PdfiumError::NoCharsInPageObject)
    }

    /// Returns a collection of all the `PdfPageTextChar` characters in the given [PdfPageAnnotation].
    ///
    /// The return result will be empty if the given [PdfPageAnnotation] is not attached to the
    /// containing [PdfPage].
    #[inline]
    pub fn chars_for_annotation(
        &self,
        annotation: &PdfPageAnnotation,
    ) -> Result<PdfPageTextChars, PdfiumError> {
        self.chars_inside_rect(annotation.bounds()?)
            .map_err(|_| PdfiumError::NoCharsInAnnotation)
    }

    /// Returns a collection of all the `PdfPageTextChar` characters that lie within the bounds of
    /// the given [PdfRect] in the containing [PdfPage].
    pub fn chars_inside_rect(&self, rect: PdfRect) -> Result<PdfPageTextChars, PdfiumError> {
        let tolerance_x = rect.width() / 2.0;
        let tolerance_y = rect.height() / 2.0;
        let center_height = rect.bottom + tolerance_y;

        let chars = self.chars();

        match (
            chars.get_char_near_point(rect.left, tolerance_x, center_height, tolerance_y),
            chars.get_char_near_point(rect.right, tolerance_x, center_height, tolerance_y),
        ) {
            (Some(start), Some(end)) => Ok(PdfPageTextChars::new(
                self,
                start.index() as i32,
                (end.index() - start.index() + 1) as i32,
                self.bindings,
            )),
            _ => Err(PdfiumError::NoCharsInRect),
        }
    }

    /// Returns all characters that lie within the containing [PdfPage], in the order in which
    /// they are defined in the document, concatenated into a single string.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    pub fn all(&self) -> String {
        self.inside_rect(self.page.page_size())
    }

    /// Returns all characters that lie within the bounds of the given [PdfRect] in the
    /// containing [PdfPage], in the order in which they are defined in the document,
    /// concatenated into a single string.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    pub fn inside_rect(&self, rect: PdfRect) -> String {
        // Retrieving the bounded text from Pdfium is a two-step operation. First, we call
        // FPDFText_GetBoundedText() with a null buffer; this will retrieve the length of
        // the bounded text in _characters_ (not _bytes_!). If the length is zero, then there is
        // no text within the given rectangle's boundaries.

        // If the length is non-zero, then we reserve a buffer (sized in words rather than bytes,
        // to allow for two bytes per character) and call FPDFText_GetBoundedText() again with a
        // pointer to the buffer; this will write the bounded text to the buffer in UTF16-LE format.

        let left = rect.left.value as f64;

        let top = rect.top.value as f64;

        let right = rect.right.value as f64;

        let bottom = rect.bottom.value as f64;

        let chars_count = self.bindings.FPDFText_GetBoundedText(
            self.handle,
            left,
            top,
            right,
            bottom,
            null_mut(),
            0,
        );

        if chars_count == 0 {
            // No text lies within the given rectangle.

            return String::new();
        }

        let mut buffer = create_sized_buffer(chars_count as usize);

        let result = self.bindings.FPDFText_GetBoundedText(
            self.handle,
            left,
            top,
            right,
            bottom,
            buffer.as_mut_ptr(),
            chars_count,
        );

        assert_eq!(result, chars_count);

        get_string_from_pdfium_utf16le_bytes(buffer.as_bytes().to_vec()).unwrap_or_default()
    }

    /// Returns all characters assigned to the given [PdfPageTextObject] in this [PdfPageText] object,
    /// concatenated into a single string.
    pub fn for_object(&self, object: &PdfPageTextObject) -> String {
        // Retrieving the string value from Pdfium is a two-step operation. First, we call
        // FPDFTextObj_GetText() with a null buffer; this will retrieve the length of
        // the text in bytes, assuming the page object exists. If the length is zero,
        // then there is no text.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFTextObj_GetText() again with a pointer to the buffer;
        // this will write the text for the page object into the buffer.

        let buffer_length = self.bindings.FPDFTextObj_GetText(
            *object.get_object_handle(),
            self.handle,
            std::ptr::null_mut(),
            0,
        );

        if buffer_length == 0 {
            // There is no text.

            return String::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFTextObj_GetText(
            *object.get_object_handle(),
            self.handle,
            buffer.as_mut_ptr() as *mut FPDF_WCHAR,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default()
    }

    /// Returns all characters that lie within the bounds of the given [PdfPageAnnotation] in the
    /// containing [PdfPage], in the order in which they are defined in the document,
    /// concatenated into a single string.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    #[inline]
    pub fn for_annotation(&self, annotation: &PdfPageAnnotation) -> Result<String, PdfiumError> {
        let bounds = annotation.bounds()?;

        Ok(self.inside_rect(bounds))
    }
}

impl<'a> Display for PdfPageText<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.all().as_str())
    }
}

impl<'a> Drop for PdfPageText<'a> {
    /// Closes the [PdfPageText] collection, releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDFText_ClosePage(self.handle);
    }
}
