//! Defines the [PdfPageTextChars] struct, a collection of all the distinct characters
//! in a bounded rectangular region of a single [PdfPage].

use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE, FPDF_TEXTPAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::index_cache::PdfPageIndexCache;
use crate::pdf::document::page::text::char::PdfPageTextChar;
use crate::pdf::document::page::text::PdfPageText;
use crate::pdf::document::page::PdfPage;
use crate::pdf::document::pages::PdfPageIndex;
use crate::pdf::points::PdfPoints;
use std::ops::Range;
use std::os::raw::c_int;

pub type PdfPageTextCharIndex = usize;

/// A collection of all the distinct character in a bounded rectangular region of
/// a single [PdfPage].
pub struct PdfPageTextChars<'a> {
    page_handle: FPDF_PAGE,
    text_page_handle: FPDF_TEXTPAGE,
    source_page: Option<PdfPage<'a>>,
    start: i32,
    len: i32,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageTextChars<'a> {
    #[inline]
    pub(crate) fn new(
        page_handle: FPDF_PAGE,
        text_page_handle: FPDF_TEXTPAGE,
        start: i32,
        len: i32,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageTextChars {
            page_handle,
            text_page_handle,
            source_page: None,
            start,
            len,
            bindings,
        }
    }

    /// Creates a new [PdfPageTextChars] instance for the given character range
    /// by loading the text page for the given page index in the given document handle.
    /// The newly created [PdfPageTextChars] instance will take ownership of both the page
    /// and its text page, disposing of both when the [PdfPageTextChars] instance leaves scope.
    pub(crate) fn new_with_owned_page(
        document_handle: FPDF_DOCUMENT,
        page_index: c_int,
        start: i32,
        len: i32,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        let page_handle = bindings.FPDF_LoadPage(document_handle, page_index);

        // Add the page to the page cache, so we can delete it later when this PdfPageTextChars
        // instance moves out of scope.

        PdfPageIndexCache::set_index_for_page(
            document_handle,
            page_handle,
            page_index as PdfPageIndex,
        );

        let page = PdfPage::from_pdfium(document_handle, page_handle, None, None, bindings);

        let text_page_handle = bindings.FPDFText_LoadPage(page.page_handle());

        PdfPageTextChars {
            page_handle,
            text_page_handle,
            source_page: Some(page),
            start,
            len,
            bindings,
        }
    }

    /// Returns the index in the containing [PdfPage] of the first character in this
    /// [PdfPageTextChars] collection.
    #[inline]
    pub fn first_char_index(&self) -> PdfPageTextCharIndex {
        self.start as PdfPageTextCharIndex
    }

    /// Returns the number of individual characters in this [PdfPageTextChars] collection.
    #[inline]
    pub fn len(&self) -> PdfPageTextCharIndex {
        self.len as PdfPageTextCharIndex
    }

    /// Returns the index in the containing [PdfPage] of the last character in this
    /// [PdfPageTextChars] collection.
    #[inline]
    pub fn last_char_index(&self) -> PdfPageTextCharIndex {
        (self.start + self.len - 1) as PdfPageTextCharIndex
    }

    /// Returns the valid index range of this [PdfPageTextChars] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageTextCharIndex> {
        self.first_char_index()..self.last_char_index()
    }

    /// Returns `true` if this [PdfPageTextChars] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a single [PdfPageTextChar] from this [PdfPageTextChars] collection.
    #[inline]
    pub fn get(&self, index: PdfPageTextCharIndex) -> Result<PdfPageTextChar, PdfiumError> {
        let index = index as i32;

        if index < self.start || index >= self.start + self.len {
            Err(PdfiumError::CharIndexOutOfBounds)
        } else {
            Ok(PdfPageTextChar::from_pdfium(
                self.page_handle,
                self.text_page_handle,
                index,
                self.bindings,
            ))
        }
    }

    /// Returns the character at the given x and y positions on the containing [PdfPage], if any.
    #[inline]
    pub fn get_char_at_point(&self, x: PdfPoints, y: PdfPoints) -> Option<PdfPageTextChar> {
        self.get_char_near_point(x, PdfPoints::ZERO, y, PdfPoints::ZERO)
    }

    /// Returns the character near to the given x and y positions on the containing [PdfPage],
    /// if any. The returned character will be no further from the given positions than the given
    /// tolerance values.
    #[inline]
    pub fn get_char_near_point(
        &self,
        x: PdfPoints,
        tolerance_x: PdfPoints,
        y: PdfPoints,
        tolerance_y: PdfPoints,
    ) -> Option<PdfPageTextChar> {
        PdfPageText::get_char_index_near_point(
            self.text_page_handle,
            x,
            tolerance_x,
            y,
            tolerance_y,
            self.bindings,
        )
        .ok_or(PdfiumError::CharIndexOutOfBounds)
        .and_then(|index| self.get(index))
        .ok()
    }

    /// Returns an iterator over all the characters in this [PdfPageTextChars] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageTextCharsIterator {
        PdfPageTextCharsIterator::new(self)
    }
}

impl<'a> Drop for PdfPageTextChars<'a> {
    /// Closes this [PdfPageTextChars] object, releasing held memory.
    #[inline]
    fn drop(&mut self) {
        if let Some(page) = self.source_page.take() {
            // This PdfPageTextChars instance had ownership over the page and text page
            // to which it was bound. Release those resources now.

            self.bindings.FPDFText_ClosePage(self.text_page_handle);
            assert!(page.delete().is_ok());
        }
    }
}

/// An iterator over all the [PdfPageTextChar] objects in a [PdfPageTextChars] collection.
pub struct PdfPageTextCharsIterator<'a> {
    chars: &'a PdfPageTextChars<'a>,
    next_index: PdfPageTextCharIndex,
}

impl<'a> PdfPageTextCharsIterator<'a> {
    #[inline]
    pub(crate) fn new(chars: &'a PdfPageTextChars<'a>) -> Self {
        PdfPageTextCharsIterator {
            chars,
            next_index: chars.first_char_index(),
        }
    }
}

impl<'a> Iterator for PdfPageTextCharsIterator<'a> {
    type Item = PdfPageTextChar<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
