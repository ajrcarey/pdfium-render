use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE};
use crate::pages::PdfPageIndex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

/// A cache of [PdfPageIndex] indices for all open [PdfPage] objects.
/// We keep track of these so that we can return accurate [PdfPageIndex] values to
/// the object copying functions in [PdfPageObjectGroup], some of which depend upon
/// accurate source page indices.
static PAGE_INDEX_CACHE: Lazy<Mutex<PdfPageIndexCache>> =
    Lazy::new(|| Mutex::new(PdfPageIndexCache::new()));

pub(crate) struct PdfPageIndexCache {
    pages_by_index: HashMap<(FPDF_DOCUMENT, FPDF_PAGE), PdfPageIndex>,
    indices_by_page: HashMap<(FPDF_DOCUMENT, PdfPageIndex), FPDF_PAGE>,
    documents_by_maximum_index: HashMap<FPDF_DOCUMENT, PdfPageIndex>,
}

impl PdfPageIndexCache {
    #[inline]
    fn new() -> Self {
        Self {
            pages_by_index: HashMap::new(),
            indices_by_page: HashMap::new(),
            documents_by_maximum_index: HashMap::new(),
        }
    }

    /// Returns the current [PdfPageIndex] value for the given raw document and page handles, if any.
    #[inline]
    fn get(&self, document: FPDF_DOCUMENT, page: FPDF_PAGE) -> Option<PdfPageIndex> {
        self.pages_by_index.get(&(document, page)).copied()
    }

    /// Sets the current [PdfPageIndex] value for the given raw document and page handles.
    #[inline]
    fn set(&mut self, document: FPDF_DOCUMENT, page: FPDF_PAGE, index: PdfPageIndex) {
        self.pages_by_index.insert((document, page), index);
        self.indices_by_page.insert((document, index), page);

        // Keep track of the maximum page index for this document. We'll need to know this
        // if we have to shuffle indices to accommodate page insertions or deletions.

        match self.documents_by_maximum_index.get(&document).copied() {
            Some(maximum) => {
                if index > maximum {
                    self.documents_by_maximum_index.insert(document, index);
                }
            }
            None => {
                self.documents_by_maximum_index.insert(document, index);
            }
        }
    }

    /// Removes the cached [PdfPageIndex] value for the given raw document and page handles.
    #[inline]
    fn remove(&mut self, document: FPDF_DOCUMENT, page: FPDF_PAGE) {
        if let Some(index) = self.pages_by_index.remove(&(document, page)) {
            self.indices_by_page.remove(&(document, index));

            if self.documents_by_maximum_index.get(&document).copied() == Some(index) {
                // This page had the maximum page index for this document. Now that it's been removed
                // from the cache, we need to find the new maximum page index for this document.

                let mut maximum = 0;

                for (key, index) in self.indices_by_page.keys() {
                    if key == &document {
                        let index = *index;

                        maximum = index.max(maximum);
                    }
                }

                self.documents_by_maximum_index.insert(document, maximum);
            }
        }
    }

    /// Adjusts all cached [PdfPageIndex] values for the given document as necessary to accommodate
    /// an insertion of the given number of pages at the given index position.
    #[inline]
    fn insert(&mut self, document: FPDF_DOCUMENT, index: PdfPageIndex, count: PdfPageIndex) {
        let mut maximum_index_for_document = self
            .documents_by_maximum_index
            .get(&document)
            .copied()
            .unwrap_or(0);

        if maximum_index_for_document > index {
            // Shuffle down all page indices in the document after the given index position.

            for index in (index..=maximum_index_for_document).rev() {
                if let Some(page) = self.indices_by_page.get(&(document, index)).copied() {
                    // Update the indices of this page.

                    self.remove(document, page);
                    self.set(document, page, index + count);
                }
            }
        } else {
            maximum_index_for_document = index;
        }

        // Update the maximum index position for this document.

        self.documents_by_maximum_index
            .insert(document, maximum_index_for_document + count);
    }

    /// Adjusts all cached [PdfPageIndex] values for the given document as necessary to accommodate
    /// a deletion of the given number of pages at the given index position.
    #[inline]
    fn delete(&mut self, document: FPDF_DOCUMENT, index: PdfPageIndex, count: PdfPageIndex) {
        // Shuffle up all page indices in the document after the given index position.

        let mut maximum_index_for_document = self
            .documents_by_maximum_index
            .get(&document)
            .copied()
            .unwrap_or(0);

        // Remove the deleted pages from the cache.

        for index in index..index + count {
            if let Some(page) = self.indices_by_page.get(&(document, index)) {
                let page = *page;

                self.remove(document, page);
            }
        }

        if maximum_index_for_document > index {
            // Shuffle down all page indices in the document after the given index position.

            for index in (index..=maximum_index_for_document).rev() {
                if let Some(page) = self.indices_by_page.get(&(document, index)).copied() {
                    // Update the indices of this page.

                    self.remove(document, page);
                    self.set(document, page, index - count);
                }
            }
        } else {
            maximum_index_for_document = index;
        }

        // Update the maximum index position for this document.

        self.documents_by_maximum_index
            .insert(document, maximum_index_for_document - count);
    }

    #[inline]
    fn lock() -> MutexGuard<'static, PdfPageIndexCache> {
        PAGE_INDEX_CACHE.lock().unwrap()
    }

    // The remaining methods in this implementation take care of thread-safe locking.
    // These methods form the public API of the cache.

    /// Sets the current [PdfPageIndex] value for the given raw document and page handles.
    #[inline]
    pub(crate) fn set_index_for_page(
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        index: PdfPageIndex,
    ) {
        Self::lock().set(document, page, index)
    }

    /// Returns the current [PdfPageIndex] value for the given raw document and page handles, if any.
    #[inline]
    pub(crate) fn get_index_for_page(
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
    ) -> Option<PdfPageIndex> {
        Self::lock().get(document, page)
    }

    /// Removes the cached [PdfPageIndex] value for the given raw document and page handles.
    #[inline]
    pub(crate) fn remove_index_for_page(document: FPDF_DOCUMENT, page: FPDF_PAGE) {
        Self::lock().remove(document, page)
    }

    /// Adjusts all cached [PdfPageIndex] values for the given document as necessary to accommodate
    /// an insertion of the given number of pages at the given index position.
    #[inline]
    pub(crate) fn insert_pages_at_index(
        document: FPDF_DOCUMENT,
        index: PdfPageIndex,
        count: PdfPageIndex,
    ) {
        Self::lock().insert(document, index, count)
    }

    /// Adjusts all cached [PdfPageIndex] values for the given document as necessary to accommodate
    /// a deletion of the given number of pages at the given index position.
    #[inline]
    pub(crate) fn delete_pages_at_index(
        document: FPDF_DOCUMENT,
        index: PdfPageIndex,
        count: PdfPageIndex,
    ) {
        Self::lock().delete(document, index, count)
    }

    /// Returns the number of [PdfPageIndex] values cached in this [PdfPageIndexCache].
    #[inline]
    pub(crate) fn len() -> usize {
        Self::lock().pages_by_index.len()
    }
}

unsafe impl Send for PdfPageIndexCache {}

unsafe impl Sync for PdfPageIndexCache {}

#[cfg(test)]
mod test {
    use crate::page_index_cache::PdfPageIndexCache;
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_cache_index_for_page() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        let document = pdfium.create_new_pdf()?;

        assert_eq!(PdfPageIndexCache::len(), 0);

        {
            // Now let's create a blank page and get a handle to it...

            let _page = document
                .pages()
                .create_page_at_start(PdfPagePaperSize::a4())?;

            // ... and confirm the cache updated.

            assert_eq!(PdfPageIndexCache::len(), 1);
        }

        // The page has dropped out of scope. Confirm the cache got cleaned up.

        assert_eq!(PdfPageIndexCache::len(), 0);

        // Get a new handle to the page...

        let _page = document.pages().first();

        // ... and confirm the cache updated.

        assert_eq!(PdfPageIndexCache::len(), 1);

        Ok(())
    }
}
