//! Defines the [PdfPages] struct, a collection of all the `PdfPage` objects in a
//! `PdfDocument`.

use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPage;
use std::ops::Range;
use std::os::raw::c_int;

pub type PdfPageIndex = u16;

/// The collection of [PdfPage] objects inside a [PdfDocument].
pub struct PdfPages<'a> {
    document: &'a PdfDocument<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPages<'a> {
    /// Creates a new [PdfPages] collection from the given [PdfDocument] and library bindings.
    pub(crate) fn new(
        document: &'a PdfDocument<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPages { document, bindings }
    }

    /// Returns the number of pages in this [PdfPages] collection.
    pub fn len(&self) -> PdfPageIndex {
        self.bindings.FPDF_GetPageCount(*self.document.get_handle()) as PdfPageIndex
    }

    /// Returns true if this [PdfPages] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from 0..(number of pages) for this [PdfPages] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageIndex> {
        0..self.len()
    }

    /// Returns a single [PdfPage] from this [PdfPages] collection.
    pub fn get(&self, index: PdfPageIndex) -> Result<PdfPage, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageIndexOutOfBounds);
        }

        let page_handle = self
            .bindings
            .FPDF_LoadPage(*self.document.get_handle(), index as c_int);

        if page_handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfPage::from_pdfium(
                index,
                page_handle,
                self.document,
                self.bindings,
            ))
        }
    }

    /// Returns an iterator over all the pages in this [PdfPages] collection.
    #[inline]
    pub fn iter(&self) -> PdfDocumentPdfPageIterator {
        PdfDocumentPdfPageIterator::new(self)
    }
}

pub struct PdfDocumentPdfPageIterator<'a> {
    pages: &'a PdfPages<'a>,
    page_count: PdfPageIndex,
    next_index: PdfPageIndex,
}

impl<'a> PdfDocumentPdfPageIterator<'a> {
    #[inline]
    pub(crate) fn new(pages: &'a PdfPages<'a>) -> Self {
        PdfDocumentPdfPageIterator {
            pages,
            page_count: pages.len(),
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfDocumentPdfPageIterator<'a> {
    type Item = PdfPage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.page_count {
            return None;
        }

        let next = self.pages.get(self.next_index);

        self.next_index += 1;

        match next {
            Ok(next) => Some(next),
            Err(_) => None,
        }
    }
}
