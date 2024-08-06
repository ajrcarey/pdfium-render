//! Defines the [PdfPageTextSegment] struct, exposing functionality related to a single rectangular
//! text segment in a `PdfPageTextSegments` collection.

use crate::error::PdfiumError;
use crate::pdf::document::page::text::chars::PdfPageTextChars;
use crate::pdf::document::page::text::PdfPageText;
use crate::pdf::points::PdfPoints;
use crate::pdf::rect::PdfRect;

#[cfg(doc)]
use {
    crate::pdf::document::page::text::char::PdfPageTextChar, crate::pdf::document::page::PdfPage,
};

/// A single rectangular text segment in a `PdfPageTextSegments` collection.
///
/// Pdfium automatically merges smaller text boxes into larger text segments if all
/// enclosed characters share the same baseline and the same font settings. The number of
/// individual `PdfPageTextObject` objects on the page may be much larger than the number of
/// text segments.
pub struct PdfPageTextSegment<'a> {
    text: &'a PdfPageText<'a>,
    bounds: PdfRect,
}

impl<'a> PdfPageTextSegment<'a> {
    pub(crate) fn from_pdfium(text: &'a PdfPageText<'a>, bounds: PdfRect) -> Self {
        PdfPageTextSegment { text, bounds }
    }

    /// Returns the bounding box of this [PdfPageTextSegment].
    #[inline]
    pub fn bounds(&self) -> PdfRect {
        self.bounds
    }

    /// Returns the width of this [PdfPageTextSegment].
    #[inline]
    pub fn width(&self) -> PdfPoints {
        self.bounds.width()
    }

    /// Returns the height of this [PdfPageTextSegment].
    #[inline]
    pub fn height(&self) -> PdfPoints {
        self.bounds.height()
    }

    /// Returns `true` if the bounds of this [PdfPageTextSegment] lie entirely within the given rectangle.
    #[inline]
    pub fn is_inside_rect(&self, rect: &PdfRect) -> bool {
        self.bounds.is_inside(rect)
    }

    /// Returns `true` if the bounds of this [PdfPageTextSegment] lie at least partially within
    /// the given rectangle.
    #[inline]
    pub fn does_overlap_rect(&self, rect: &PdfRect) -> bool {
        self.bounds.does_overlap(rect)
    }

    /// Returns all characters that lie within the bounds of this [PdfPageTextSegment] in the
    /// containing [PdfPage], in the order in which they are defined in the document.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    #[inline]
    pub fn text(&self) -> String {
        self.text.inside_rect(self.bounds)
    }

    /// Returns a collection of all the [PdfPageTextChar] characters that lie within the bounds of
    /// this [PdfPageTextSegment] in the containing [PdfPage], in the order in which they are
    /// defined in the document.
    ///
    /// In complex custom layouts, the order in which characters are defined in the document
    /// and the order in which they appear visually during rendering (and thus the order in
    /// which they are read by a user) may not necessarily match.
    #[inline]
    pub fn chars(&self) -> Result<PdfPageTextChars, PdfiumError> {
        self.text.chars_inside_rect(self.bounds)
    }
}
