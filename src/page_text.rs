//! Defines the [PdfPageText] struct, exposing functionality related to the
//! collection of Unicode characters visible in a single `PdfPage`.

use crate::bindgen::FPDF_TEXTPAGE;
use crate::bindings::PdfiumLibraryBindings;
use crate::page::{PdfPage, PdfRect};
use crate::utils::mem::create_sized_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use image::EncodableLayout;
use std::fmt::{Display, Formatter};
use std::ptr::null_mut;

/// The collection of Unicode characters visible in a single [PdfPage].
///
/// Since [PdfPageText] implements both the [ToString] and the [Display] traits, you can use
/// [PdfPageText::all()] to easily return all characters in the containing [PdfPage]
/// in the order in which they are defined in the PDF file.
///
/// In complex custom layouts, the order in which characters are defined in the PDF file
/// and the order in which they appear visually during rendering (and thus the order in
/// which they are read by a user) may not necessarily match.
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

    /// Returns the internal FPDF_TEXTPAGE handle for this [PdfPageText].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_TEXTPAGE {
        &self.handle
    }

    /// Returns the total number of characters in all text boxes in the containing [PdfPage].
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

    /// Returns all characters that lie within the containing [PdfPage], in the order in which
    /// they are defined in the PDF file.
    ///
    /// In complex custom layouts, the order in which characters are defined in the PDF file
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    pub fn all(&self) -> String {
        self.inside_rect(self.page.page_size())
    }

    /// Returns all characters that lie within the bounds of the given [PdfRect] in the
    /// containing [PdfPage], in the order in which they are defined in the PDF file.
    ///
    /// In complex custom layouts, the order in which characters are defined in the PDF file
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
