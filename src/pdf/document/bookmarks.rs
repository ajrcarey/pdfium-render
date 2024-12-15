//! Defines the [PdfBookmarks] struct, exposing functionality related to the
//! bookmarks contained within a single `PdfDocument`.

use crate::bindgen::{FPDF_BOOKMARK, FPDF_DOCUMENT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::bookmark::PdfBookmark;
use std::collections::HashSet;
use std::ptr::null_mut;

/// The bookmarks contained within a single `PdfDocument`.
///
/// Bookmarks in PDF files form a tree structure, branching out from a top-level root bookmark.
/// The [PdfBookmarks::root()] returns the root bookmark in the containing `PdfDocument`, if any;
/// use the root's [PdfBookmark::first_child()] and [PdfBookmark::next_sibling()] functions to
/// traverse the bookmark tree.
///
/// To search the tree for a bookmark with a specific title, use the [PdfBookmarks::find_first_by_title()]
/// and [PdfBookmarks::find_all_by_title()] functions. To traverse the tree breadth-first, visiting
/// every bookmark in the tree, create an iterator using the [PdfBookmarks::iter()] function.
pub struct PdfBookmarks<'a> {
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBookmarks<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            document_handle,
            bindings,
        }
    }

    /// Returns the internal `FPDF_DOCUMENT` handle of the `PdfDocument` containing
    /// this [PdfBookmarks] collection.
    #[inline]
    pub(crate) fn document_handle(&self) -> FPDF_DOCUMENT {
        self.document_handle
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfBookmarks] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the root [PdfBookmark] in the containing `PdfDocument`, if any.
    pub fn root(&self) -> Option<PdfBookmark> {
        let bookmark_handle = self
            .bindings
            .FPDFBookmark_GetFirstChild(self.document_handle, null_mut());

        if bookmark_handle.is_null() {
            None
        } else {
            Some(PdfBookmark::from_pdfium(
                bookmark_handle,
                None,
                self.document_handle,
                self.bindings,
            ))
        }
    }

    /// Returns the first [PdfBookmark] in the containing `PdfDocument` that has a title matching
    /// the given string.
    ///
    /// Note that bookmarks are not required to have unique titles, so in theory any number of
    /// bookmarks could match a given title. This function only ever returns the first. To return
    /// all matches, use [PdfBookmarks::find_all_by_title()].
    pub fn find_first_by_title(&self, title: &str) -> Result<PdfBookmark, PdfiumError> {
        let handle = self
            .bindings
            .FPDFBookmark_Find_str(self.document_handle, title);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfBookmark::from_pdfium(
                handle,
                None,
                self.document_handle,
                self.bindings,
            ))
        }
    }

    /// Returns all [PdfBookmark] objects in the containing `PdfDocument` that have a title
    /// matching the given string.
    ///
    /// Note that bookmarks are not required to have unique titles, so in theory any number of
    /// bookmarks could match a given title. This function returns all matches by performing
    /// a complete breadth-first traversal of the entire bookmark tree. To return just the first
    /// match, use [PdfBookmarks::find_first_by_title()].
    pub fn find_all_by_title(&self, title: &str) -> Vec<PdfBookmark> {
        self.iter()
            .filter(|bookmark| match bookmark.title() {
                Some(bookmark_title) => bookmark_title == title,
                None => false,
            })
            .collect()
    }

    /// Returns a depth-first prefix-order iterator over all the [PdfBookmark]
    /// objects in the containing `PdfDocument`, starting from the top-level
    /// root bookmark.
    #[inline]
    pub fn iter(&self) -> PdfBookmarksIterator {
        PdfBookmarksIterator::new(
            self.root(),
            true,
            None,
            self.document_handle(),
            self.bindings(),
        )
    }
}

/// An iterator over all the [PdfBookmark] objects in a [PdfBookmarks] collection.
pub struct PdfBookmarksIterator<'a> {
    // If true, recurse into descendants.
    include_descendants: bool,
    // Stack of pairs of (Bookmark Node, Node's Parent). The parent may be NULL
    // if its a root node or the parent is unknown.
    pending_stack: Vec<(FPDF_BOOKMARK, FPDF_BOOKMARK)>,
    // Set of nodes already visitied. This ensures we terminate if the PDF's
    // bookmark graph is cyclic.
    visited: HashSet<FPDF_BOOKMARK>,
    // This bookmark will not be returned by the iterator (but its siblings and
    // descendants will be explored). May be NULL.
    skip_sibling: FPDF_BOOKMARK,
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBookmarksIterator<'a> {
    pub(crate) fn new(
        start_node: Option<PdfBookmark<'a>>,
        include_descendants: bool,
        skip_sibling: Option<PdfBookmark<'a>>,
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        let mut result = PdfBookmarksIterator {
            document_handle,
            include_descendants,
            pending_stack: Vec::with_capacity(20),
            visited: HashSet::new(),
            skip_sibling: null_mut(),
            bindings,
        };

        // If we have a skip-sibling, record its handle.
        if let Some(skip_sibling) = skip_sibling {
            result.skip_sibling = skip_sibling.bookmark_handle();
        }

        // Push the start node onto the stack to initiate graph traversal.
        if let Some(start_node) = start_node {
            result.pending_stack.push((
                start_node.bookmark_handle(),
                start_node
                    .parent()
                    .map(|parent| parent.bookmark_handle())
                    .unwrap_or(null_mut()),
            ));
        }

        result
    }
}

impl<'a> Iterator for PdfBookmarksIterator<'a> {
    type Item = PdfBookmark<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // A straightforward tail-recursive function to walk the bookmarks might
        // look about like this:
        //
        // pub fn walk(node: Option<PdfBookmark<'a>>) {
        //     if let Some(node) = node) {
        //         visit(&node);
        //         walk(node.first_child());
        //         walk(node.next_sibling());
        //     }
        // }
        //
        // This iterator implements that algorithm with the following additional
        // complexities:
        //
        // - Iterators, of course, can't take advantage of recursion. So the
        //   call stack which is implicit in the recursive version becomes an
        //   explicit stack retained in PdfIterator::pending_stack.
        // - For efficiency, the iterator internally operates with FPDF_BOOKMARK
        //   handles, and only constructs PdfBookmark objects right before
        //   they're returned.
        // - PdfIterator::visited keeps a HashSet of visited nodes, to ensure
        //   termination even if the PDF's bookmark graph is cyclic.
        // - PdfIterator::skip_sibling keeps a FPDF_BOOKMARK that will not be
        //   returned by the iterator (but, importantly, it's siblings will
        //   still be explored).

        while let Some((node, parent)) = self.pending_stack.pop() {
            if node.is_null() || self.visited.contains(&node) {
                continue;
            }
            self.visited.insert(node);

            // Add our next sibling to the stack first, so we'll come back to it
            // after having addressed our descendants. It's okay if it's NULL,
            // we'll handle that when it comes off the stack.
            self.pending_stack.push((
                self.bindings
                    .FPDFBookmark_GetNextSibling(self.document_handle, node),
                parent,
            ));

            // Add our first descendant to the stack if we should include them.
            // Again, its okay if it's NULL.
            if self.include_descendants {
                self.pending_stack.push((
                    self.bindings
                        .FPDFBookmark_GetFirstChild(self.document_handle, node),
                    node,
                ));
            }

            // If the present node isn't the one we're meant to skip, return it.
            if node != self.skip_sibling {
                let parent = if parent.is_null() { None } else { Some(parent) };
                return Some(PdfBookmark::from_pdfium(
                    node,
                    parent,
                    self.document_handle,
                    self.bindings,
                ));
            }
        }

        // If we got here, then the stack is empty and we're done.
        None
    }
}
