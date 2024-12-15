//! Defines the [PdfBookmark] struct, exposing functionality related to a single bookmark
//! in a [PdfBookmarks] collection.

use crate::bindgen::{FPDF_BOOKMARK, FPDF_DOCUMENT};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::action::PdfAction;
use crate::pdf::destination::PdfDestination;
use crate::pdf::document::bookmarks::PdfBookmarksIterator;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::os::raw::c_void;

#[cfg(doc)]
use {
    crate::pdf::action::PdfActionType, crate::pdf::document::bookmarks::PdfBookmarks,
    crate::pdf::document::PdfDocument,
};

/// A single bookmark in a [PdfBookmarks] collection.
pub struct PdfBookmark<'a> {
    bookmark_handle: FPDF_BOOKMARK,
    parent: Option<FPDF_BOOKMARK>,
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBookmark<'a> {
    pub(crate) fn from_pdfium(
        bookmark_handle: FPDF_BOOKMARK,
        parent: Option<FPDF_BOOKMARK>,
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfBookmark {
            bookmark_handle,
            parent,
            document_handle,
            bindings,
        }
    }

    /// Returns the internal `FPDF_BOOKMARK` handle for this [PdfBookmark].
    #[inline]
    pub(crate) fn bookmark_handle(&self) -> FPDF_BOOKMARK {
        self.bookmark_handle
    }

    /// Returns the internal `FPDF_DOCUMENT` handle of the [PdfDocument] containing this [PdfBookmark].
    #[inline]
    pub(crate) fn document_handle(&self) -> FPDF_DOCUMENT {
        self.document_handle
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfBookmark].
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Creates a clone of this [PdfBookmark] that points to the same internal `FPDF_BOOKMARK` handle.
    #[inline]
    pub(crate) fn clone(&self) -> PdfBookmark<'a> {
        Self::from_pdfium(
            self.bookmark_handle,
            self.parent,
            self.document_handle,
            self.bindings,
        )
    }

    /// Returns the title of this [PdfBookmark], if any.
    pub fn title(&self) -> Option<String> {
        // Retrieving the bookmark title from Pdfium is a two-step operation. First, we call
        // FPDFBookmark_GetTitle() with a null buffer; this will retrieve the length of
        // the bookmark title in bytes. If the length is zero, then there is no title.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFBookmark_GetTitle() again with a pointer to the buffer;
        // this will write the bookmark title to the buffer in UTF16-LE format.

        let buffer_length =
            self.bindings
                .FPDFBookmark_GetTitle(self.bookmark_handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // No title is defined.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFBookmark_GetTitle(
            self.bookmark_handle,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer)
    }

    /// Returns the [PdfAction] associated with this [PdfBookmark], if any.
    ///
    /// The action indicates the behaviour that will occur when the user interacts with the
    /// bookmark in a PDF viewer. For most bookmarks, this will be a local navigation action
    /// of type [PdfActionType::GoToDestinationInSameDocument], but the PDF file format supports
    /// a variety of other actions.
    pub fn action(&self) -> Option<PdfAction<'a>> {
        let handle = self.bindings.FPDFBookmark_GetAction(self.bookmark_handle);

        if handle.is_null() {
            None
        } else {
            Some(PdfAction::from_pdfium(
                handle,
                self.document_handle,
                self.bindings,
            ))
        }
    }

    /// Returns the [PdfDestination] associated with this [PdfBookmark], if any.
    ///
    /// The destination specifies the page and region, if any, that will be the target
    /// of the action behaviour specified by [PdfBookmark::action()].
    pub fn destination(&self) -> Option<PdfDestination<'a>> {
        let handle = self
            .bindings
            .FPDFBookmark_GetDest(self.document_handle, self.bookmark_handle);

        if handle.is_null() {
            None
        } else {
            Some(PdfDestination::from_pdfium(
                self.document_handle,
                handle,
                self.bindings,
            ))
        }
    }

    /// Returns this [PdfBookmark] object's direct parent, if available.
    #[inline]
    pub fn parent(&self) -> Option<PdfBookmark<'a>> {
        self.parent.map(|parent_handle| {
            PdfBookmark::from_pdfium(parent_handle, None, self.document_handle, self.bindings)
        })
    }

    /// Returns the number of direct children of this [PdfBookmark].
    #[inline]
    pub fn children_len(&self) -> usize {
        // If there are N child bookmarks, then FPDFBookmark_GetCount returns a
        // N if the bookmark tree should be displayed open by default, and -N if
        // the child tree should be displayed closed by deafult.
        self.bindings
            .FPDFBookmark_GetCount(self.bookmark_handle)
            .abs() as usize
    }

    /// Returns the first child [PdfBookmark] of this [PdfBookmark], if any.
    pub fn first_child(&self) -> Option<PdfBookmark<'a>> {
        let handle = self
            .bindings
            .FPDFBookmark_GetFirstChild(self.document_handle, self.bookmark_handle);

        if handle.is_null() {
            None
        } else {
            Some(PdfBookmark::from_pdfium(
                handle,
                Some(self.bookmark_handle),
                self.document_handle,
                self.bindings,
            ))
        }
    }

    /// Returns the next [PdfBookmark] at the same tree level as this [PdfBookmark], if any.
    pub fn next_sibling(&self) -> Option<PdfBookmark<'a>> {
        let handle = self
            .bindings
            .FPDFBookmark_GetNextSibling(self.document_handle, self.bookmark_handle);

        if handle.is_null() {
            None
        } else {
            Some(PdfBookmark::from_pdfium(
                handle,
                self.parent,
                self.document_handle,
                self.bindings,
            ))
        }
    }

    /// Returns an iterator over all [PdfBookmark] sibling nodes of this [PdfBookmark].
    #[inline]
    pub fn iter_siblings(&self) -> PdfBookmarksIterator<'a> {
        match self.parent {
            Some(parent_handle) => {
                // Siblings by definition all share the same parent. We can achieve a more
                // consistent result, irrespective of whether we are the parent's first direct
                // child or not, by iterating over all the parent's children.

                PdfBookmarksIterator::new(
                    PdfBookmark::from_pdfium(
                        parent_handle,
                        None,
                        self.document_handle,
                        self.bindings,
                    )
                    .first_child(),
                    false,
                    // Signal that the iterator should skip over this bookmark when iterating
                    // the parent's direct children.
                    Some(self.clone()),
                    self.document_handle(),
                    self.bindings(),
                )
            }
            None => {
                // Since no handle to the parent is available, the best we can do is create an iterator
                // that repeatedly calls Self::next_sibling(). If we are not the first direct child
                // of a parent node, then this approach may not include all the parent's children.

                PdfBookmarksIterator::new(
                    Some(self.clone()),
                    false,
                    // Signal that the iterator should skip over this bookmark when iterating
                    // the parent's direct children.
                    Some(self.clone()),
                    self.document_handle(),
                    self.bindings(),
                )
            }
        }
    }

    /// Returns an iterator over all [PdfBookmark] child nodes of this [PdfBookmark].
    /// Only direct children of this [PdfBookmark] will be traversed by the iterator;
    /// grandchildren, great-grandchildren, and other descendant nodes will be ignored.
    /// To visit all child nodes, including children of children, use [PdfBookmark::iter_all_descendants()].
    #[inline]
    pub fn iter_direct_children(&self) -> PdfBookmarksIterator<'a> {
        PdfBookmarksIterator::new(
            self.first_child(),
            false,
            None,
            self.document_handle(),
            self.bindings(),
        )
    }

    /// Returns an iterator over all [PdfBookmark] descendant nodes of this [PdfBookmark],
    /// including any children of those nodes. To visit only direct children of this [PdfBookmark],
    /// use [PdfBookmark::iter_direct_children()].
    #[inline]
    pub fn iter_all_descendants(&self) -> PdfBookmarksIterator<'a> {
        PdfBookmarksIterator::new(
            self.first_child(),
            true,
            None,
            self.document_handle(),
            self.bindings(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_bookmarks() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();
        let document = pdfium.load_pdf_from_file("./test/test-toc.pdf", None)?;

        // Should be able to find Sections 3 and 4
        let section3 = document.bookmarks().find_first_by_title("Section 3")?;
        let section4 = document.bookmarks().find_first_by_title("Section 4")?;

        // Section 3 should have five direct children, sections 3.1 through 3.5,
        // each of which should show up exactly once.
        let direct_children: Vec<String> = section3.iter_direct_children().map(title).collect();
        let expected: Vec<String> = (1..6).map(|i| format!("Section 3.{i}")).collect();
        assert_eq!(direct_children, expected);
        assert_eq!(section3.children_len(), 5);

        // Section 4 has no children
        assert_eq!(section4.iter_direct_children().count(), 0);
        assert_eq!(section4.children_len(), 0);

        // Section 3 has many descendents
        let all_children: Vec<String> = section3.iter_all_descendants().map(title).collect();
        let expected = [
            "Section 3.1",
            "Section 3.2",
            "Section 3.2.1",
            "Section 3.2.2",
            "Section 3.2.3",
            "Section 3.2.4",
            "Section 3.2.5",
            "Section 3.2.6",
            "Section 3.2.7",
            "Section 3.2.8",
            "Section 3.2.9",
            "Section 3.2.10",
            "Section 3.2.11",
            "Section 3.2.12",
            "Section 3.3",
            "Section 3.3.1",
            "Section 3.3.2",
            "Section 3.3.2.1",
            "Section 3.3.2.2",
            "Section 3.3.2.3",
            "Section 3.3.3",
            "Section 3.3.4",
            "Section 3.3.5",
            "Section 3.3.6",
            "Section 3.4",
            "Section 3.4.1",
            "Section 3.4.2",
            "Section 3.5",
            "Section 3.5.1",
            "Section 3.5.2",
        ];
        assert_eq!(all_children, expected);

        // Section 4 has no childern
        assert_eq!(section4.iter_all_descendants().count(), 0);

        // Section 3 has no parents, so when iterating siblings, we expect only
        // sections 4 and 5.
        let siblings: Vec<String> = section3.iter_siblings().map(title).collect();
        assert_eq!(siblings, ["Section 4", "Section 5"]);

        // Find section 3.2.6 in a way that we'll know its parent.
        let section3_2_6 = section3
            .iter_all_descendants()
            .find(|bookmark| bookmark.title() == Some("Section 3.2.6".into()))
            .expect("§3.2.6");
        assert!(section3_2_6.parent().is_some());
        // Then we expect the siblings to be sections 3.2.1 .. 3.2.12, excluding
        // ourselves.
        let siblings: Vec<String> = section3_2_6.iter_siblings().map(title).collect();
        let expected = [
            "Section 3.2.1",
            "Section 3.2.2",
            "Section 3.2.3",
            "Section 3.2.4",
            "Section 3.2.5",
            "Section 3.2.7",
            "Section 3.2.8",
            "Section 3.2.9",
            "Section 3.2.10",
            "Section 3.2.11",
            "Section 3.2.12",
        ];
        assert_eq!(siblings, expected);

        // Section 5.3.3.1 is an only child.
        let section5_3_3_1 = document
            .bookmarks()
            .find_first_by_title("Section 5.3.3.1")?;
        assert!(section5_3_3_1.parent().is_none());
        assert_eq!(section5_3_3_1.iter_siblings().count(), 0);
        // Even if we know its parent.
        let section5_3_3_1 = document
            .bookmarks()
            .iter()
            .find(|bookmark| bookmark.title() == Some("Section 5.3.3.1".into()))
            .expect("§5.3.3.1");
        assert!(section5_3_3_1.parent().is_some());
        assert_eq!(section5_3_3_1.iter_siblings().count(), 0);

        // Check that the parent is set right
        for bookmark in document.bookmarks().iter() {
            match bookmark.parent() {
                // If there is no parent, then this should be a top-level
                // section, which doesn't have a . in its name.
                None => assert!(!title(bookmark).contains('.')),
                Some(parent) => {
                    // If you take this section's title, and lop off the last
                    // dot and what comes after it, you should have the parent
                    // section's title.
                    let this_title = title(bookmark);
                    let last_dot = this_title.rfind('.').unwrap();
                    assert_eq!(title(parent), this_title[..last_dot]);
                }
            }
        }

        let all_bookmarks: Vec<_> = document.bookmarks().iter().map(title).collect();
        let expected = [
            "Section 1",
            "Section 2",
            "Section 3",
            "Section 3.1",
            "Section 3.2",
            "Section 3.2.1",
            "Section 3.2.2",
            "Section 3.2.3",
            "Section 3.2.4",
            "Section 3.2.5",
            "Section 3.2.6",
            "Section 3.2.7",
            "Section 3.2.8",
            "Section 3.2.9",
            "Section 3.2.10",
            "Section 3.2.11",
            "Section 3.2.12",
            "Section 3.3",
            "Section 3.3.1",
            "Section 3.3.2",
            "Section 3.3.2.1",
            "Section 3.3.2.2",
            "Section 3.3.2.3",
            "Section 3.3.3",
            "Section 3.3.4",
            "Section 3.3.5",
            "Section 3.3.6",
            "Section 3.4",
            "Section 3.4.1",
            "Section 3.4.2",
            "Section 3.5",
            "Section 3.5.1",
            "Section 3.5.2",
            "Section 4",
            "Section 5",
            "Section 5.1",
            "Section 5.2",
            "Section 5.3",
            "Section 5.3.1",
            "Section 5.3.2",
            "Section 5.3.3",
            "Section 5.3.3.1",
            "Section 5.3.4",
            "Section 5.4",
            "Section 5.4.1",
            "Section 5.4.1.1",
            "Section 5.4.1.2",
            "Section 5.4.2",
            "Section 5.4.2.1",
            "Section 5.4.3",
            "Section 5.4.4",
            "Section 5.4.5",
            "Section 5.4.6",
            "Section 5.4.7",
            "Section 5.4.8",
            "Section 5.4.8.1",
            "Section 5.5",
            "Section 5.5.1",
            "Section 5.5.2",
            "Section 5.5.3",
            "Section 5.5.4",
            "Section 5.5.5",
            "Section 5.5.5.1",
            "Section 5.5.5.2",
            "Section 5.5.5.3",
            "Section 5.5.6",
            "Section 5.5.6.1",
            "Section 5.5.6.2",
            "Section 5.5.6.3",
            "Section 5.5.7",
            "Section 5.5.7.1",
            "Section 5.5.7.2",
            "Section 5.5.7.3",
            "Section 5.5.7.4",
            "Section 5.5.7.5",
            "Section 5.5.7.6",
            "Section 5.5.8",
            "Section 5.5.8.1",
            "Section 5.5.8.2",
            "Section 5.5.8.3",
            "Section 5.5.8.4",
            "Section 5.5.8.5",
            "Section 5.5.8.6",
            "Section 5.5.8.7",
            "Section 5.5.8.8",
            "Section 5.5.8.9",
            "Section 5.5.9",
            "Section 5.5.10",
            "Section 5.6",
            "Section 5.7",
            "Section 5.7.1",
            "Section 5.7.2",
            "Section 5.7.3",
            "Section 5.7.3.1",
            "Section 5.7.3.2",
            "Section 5.7.3.3",
            "Section 5.7.3.4",
            "Section 5.7.4",
            "Section 5.7.4.1",
            "Section 5.7.4.2",
            "Section 5.7.4.3",
        ];
        assert_eq!(all_bookmarks, expected);

        Ok(())
    }

    fn title(bookmark: PdfBookmark) -> String {
        bookmark.title().expect("Bookmark Title")
    }
}
