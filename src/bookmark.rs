//! Defines the [PdfBookmark] struct, exposing functionality related to a single bookmark
//! in a `PdfBookmarks` collection.

use crate::action::PdfAction;
use crate::bindgen::{FPDF_BOOKMARK, FPDF_DOCUMENT};
use crate::bindings::PdfiumLibraryBindings;
use crate::bookmarks::PdfBookmarksIterator;
use crate::destination::PdfDestination;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::os::raw::c_void;

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

    /// Returns the internal `FPDF_DOCUMENT` handle of the `PdfDocument` containing this [PdfBookmark].
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
    /// of type `PdfActionType::GoToDestinationInSameDocument`, but the PDF file format supports
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

    /// Returns the first child [PdfBookmark] of this [PdfBookmark] in the containing
    /// `PdfDocument`, if any.
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

    /// Returns the next [PdfBookmark] at the same tree level as this [PdfBookmark] in
    /// the containing `PdfDocument`, if any.
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
                    Some(PdfBookmark::from_pdfium(
                        parent_handle,
                        None,
                        self.document_handle,
                        self.bindings,
                    )),
                    false,
                    true,
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
                    true,
                    false,
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
    /// grandchildren, great-grandchildren and other descendant nodes will be ignored.
    /// To visit all child nodes, including children of children, use [PdfBookmark::iter_all_descendants()].
    #[inline]
    pub fn iter_direct_children(&self) -> PdfBookmarksIterator<'a> {
        PdfBookmarksIterator::new(
            Some(self.clone()),
            false,
            true,
            false,
            None,
            self.document_handle(),
            self.bindings(),
        )
    }

    /// Returns an iterator over all [PdfBookmark] child nodes of this [PdfBookmark],
    /// including any children of those nodes. To visit only direct children of this [PdfBookmark],
    /// use [PdfBookmark::iter_direct_children()].
    #[inline]
    pub fn iter_all_descendants(&self) -> PdfBookmarksIterator<'a> {
        PdfBookmarksIterator::new(
            Some(self.clone()),
            false,
            true,
            true,
            None,
            self.document_handle(),
            self.bindings(),
        )
    }
}
