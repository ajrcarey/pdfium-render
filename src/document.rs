use crate::bindgen::FPDF_DOCUMENT;
use crate::bindings::PdfiumLibraryBindings;
use crate::page::PdfPage;
use crate::{PdfPageIndex, PdfiumError, PdfiumInternalError};
use std::ops::Range;

/// A collection of PdfPages contained in a single file.
pub struct PdfDocument<'a> {
    handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfDocument<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self { handle, bindings }
    }

    /// Returns the number of pages in this PdfDocument.
    pub fn page_count(&self) -> PdfPageIndex {
        self.bindings.FPDF_GetPageCount(self.handle) as PdfPageIndex
    }

    /// Returns a Range from 0..(number of pages) for this PdfDocument.
    #[inline]
    pub fn page_range(&self) -> Range<PdfPageIndex> {
        0..self.page_count()
    }

    /// Returns a single page from this PdfDocument.
    pub fn get_page(&self, index: PdfPageIndex) -> Result<PdfPage, PdfiumError> {
        if index >= self.page_count() {
            return Err(PdfiumError::PageIndexOutOfBounds);
        }

        let handle = self.bindings.FPDF_LoadPage(self.handle, index as i32);

        if handle.is_null() {
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
            Ok(PdfPage::from_pdfium(index, handle, self.bindings))
        }
    }

    /// Returns an iterator over all the pages in this PdfDocument.
    #[inline]
    pub fn pages(&self) -> PdfDocumentPdfPageIterator {
        PdfDocumentPdfPageIterator::new(self)
    }
}

impl<'a> Drop for PdfDocument<'a> {
    /// Closes this PdfDocument, releasing held memory and, if the document was loaded
    /// from a file, the file handle on the document.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_CloseDocument(self.handle);
    }
}

pub struct PdfDocumentPdfPageIterator<'a> {
    document: &'a PdfDocument<'a>,
    page_count: PdfPageIndex,
    next_index: PdfPageIndex,
}

impl<'a> PdfDocumentPdfPageIterator<'a> {
    #[inline]
    fn new(document: &'a PdfDocument<'a>) -> Self {
        PdfDocumentPdfPageIterator {
            document,
            page_count: document.page_count(),
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

        let next = self.document.get_page(self.next_index);

        self.next_index += 1;

        match next {
            Ok(next) => Some(next),
            Err(_) => None,
        }
    }
}
