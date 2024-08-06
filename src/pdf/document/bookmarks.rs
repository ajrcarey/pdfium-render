//! Defines the [PdfBookmarks] struct, exposing functionality related to the
//! bookmarks contained within a single `PdfDocument`.

use crate::bindgen::{FPDF_BOOKMARK, FPDF_DOCUMENT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::bookmark::PdfBookmark;
use std::collections::{HashMap, VecDeque};
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

    /// Returns a breadth-first iterator over all the [PdfBookmark] objects in the containing
    /// `PdfDocument`, starting from the top-level root bookmark.
    #[inline]
    pub fn iter(&self) -> PdfBookmarksIterator {
        PdfBookmarksIterator::new(
            self.root(),
            true,
            true,
            true,
            None,
            self.document_handle(),
            self.bindings(),
        )
    }
}

pub struct PdfBookmarksIterator<'a> {
    include_siblings: bool,
    include_direct_children: bool,
    include_all_descendants: bool,
    skip_sibling: Option<PdfBookmark<'a>>,
    pending_stack: VecDeque<(FPDF_BOOKMARK, Option<FPDF_BOOKMARK>)>,
    visited: HashMap<FPDF_BOOKMARK, bool>,
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBookmarksIterator<'a> {
    pub(crate) fn new(
        start_node: Option<PdfBookmark<'a>>,
        include_siblings: bool,
        include_direct_children: bool,
        include_all_descendants: bool,
        skip_sibling: Option<PdfBookmark<'a>>,
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        let mut result = PdfBookmarksIterator {
            document_handle,
            include_siblings,
            include_direct_children,
            include_all_descendants,
            skip_sibling,
            pending_stack: VecDeque::with_capacity(20),
            visited: HashMap::new(),
            bindings,
        };

        // Push the start node onto the stack to initiate graph traversal.

        if let Some(start_node) = start_node {
            result.pending_stack.push_back((
                start_node.bookmark_handle(),
                start_node.parent().map(|parent| parent.bookmark_handle()),
            ));
        }

        result
    }

    /// Returns `true` if the given [PdfBookmark] has already been visited during
    /// graph traversal.
    fn is_already_visited(&self, bookmark: &PdfBookmark) -> bool {
        self.visited.contains_key(&bookmark.bookmark_handle())
    }

    /// Returns `true` if the given [PdfBookmark] matches the bookmark marked as the skip
    /// sibling for this iterator. The skip sibling should be skipped during traversal
    /// of siblings. This avoids the bookmark used to start a sibling traversal being
    /// itself included in the list of siblings.
    fn is_skip_sibling(&self, bookmark: &PdfBookmark) -> bool {
        match self.skip_sibling.as_ref() {
            Some(skip_sibling) => skip_sibling.bookmark_handle() == bookmark.bookmark_handle(),
            None => false,
        }
    }

    /// Pushes the given [PdfBookmark] into the queue of nodes to visit, placing it at
    /// the front of the queue.
    fn push_front(&mut self, bookmark: PdfBookmark) {
        if !self.is_already_visited(&bookmark) {
            self.pending_stack.push_front((
                bookmark.bookmark_handle(),
                bookmark.parent().map(|parent| parent.bookmark_handle()),
            ));
        }
    }

    /// Pushes the given [PdfBookmark] into the queue of nodes to visit, placing it at
    /// the back of the queue.
    fn push_back(&mut self, bookmark: PdfBookmark) {
        if !self.is_already_visited(&bookmark) {
            self.pending_stack.push_back((
                bookmark.bookmark_handle(),
                bookmark.parent().map(|parent| parent.bookmark_handle()),
            ));
        }
    }

    /// Pushes all children of the given [PdfBookmark] into the queue of nodes to visit.
    fn push_children(&mut self, parent: &PdfBookmark) {
        let mut children = Vec::with_capacity(10);

        let first_child = parent.first_child();

        if let Some(first_child) = first_child {
            let mut next_sibling = first_child.next_sibling();

            children.push(first_child);

            while let Some(sibling) = next_sibling {
                next_sibling = sibling.next_sibling();
                children.push(sibling);
            }
        }

        children
            .drain(..)
            .rev()
            .for_each(|child| self.push_front(child));
    }

    /// Pushes all siblings of the given [PdfBookmark] into the queue of nodes to visit.
    fn push_siblings(&mut self, sibling: &PdfBookmark) {
        let mut siblings = Vec::with_capacity(10);

        let mut next_sibling = sibling.next_sibling();

        while let Some(sibling) = next_sibling {
            next_sibling = sibling.next_sibling();

            if !self.is_skip_sibling(&sibling) {
                siblings.push(sibling);
            }
        }

        siblings
            .drain(..)
            .rev()
            .for_each(|sibling| self.push_back(sibling));
    }
}

impl<'a> Iterator for PdfBookmarksIterator<'a> {
    type Item = PdfBookmark<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // We follow a standard depth-first graph traversal method.

        // Pop the next node we haven't yet visited off the stack.

        let next_unvisited_node = loop {
            if let Some((bookmark_handle, parent_bookmark_handle)) = self.pending_stack.pop_front()
            {
                if !self.visited.contains_key(&bookmark_handle) {
                    break Some(PdfBookmark::from_pdfium(
                        bookmark_handle,
                        parent_bookmark_handle,
                        self.document_handle,
                        self.bindings,
                    ));
                }
            } else {
                break None;
            }
        };

        if let Some(bookmark) = next_unvisited_node {
            // Mark the node as visited...

            self.visited.insert(bookmark.bookmark_handle(), true);

            // ... and schedule its children and siblings for visiting, if so configured.

            if self.include_direct_children {
                self.push_children(&bookmark);

                // Only probe child nodes for grandchildren if we have been instructed
                // to include all descendants.

                self.include_direct_children = self.include_all_descendants;
            }

            if self.include_siblings {
                self.push_siblings(&bookmark);

                if self.is_skip_sibling(&bookmark) {
                    // Don't yield the skip sibling as an iteration result.

                    return self.next();
                }
            }

            Some(bookmark)
        } else {
            // All nodes in the graph have been visited.

            None
        }
    }
}
