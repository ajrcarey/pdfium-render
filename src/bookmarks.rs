//! Defines the [PdfBookmarks] struct, exposing functionality related to the
//! bookmarks contained within a single `PdfDocument`.

use crate::bindings::PdfiumLibraryBindings;
use crate::bookmark::PdfBookmark;
use crate::document::PdfDocument;
use std::ptr::null_mut;

/// The bookmarks contained within a single [PdfDocument].
///
/// Bookmarks in PDF files form a tree structure, branching out from a top-level root bookmark.
/// The [PdfBookmarks::root()] returns the root bookmark in the containing [PdfDocument], if any;
/// use the root's [PdfBookmark::first_child()] and [PdfBookmark::next_sibling()] functions to
/// traverse the bookmark tree.
///
/// To search the tree for a bookmark with a specific title, use the [PdfBookmarks::find_first_by_title()]
/// and [PdfBookmarks::find_all_by_title()] functions. To traverse the tree breadth-first, visiting
/// every bookmark in the tree, create an iterator using the [PdfBookmarks::iter()] function.
pub struct PdfBookmarks<'a> {
    document: &'a PdfDocument<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBookmarks<'a> {
    #[inline]
    pub(crate) fn new(
        document: &'a PdfDocument<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self { document, bindings }
    }

    /// Returns the root [PdfBookmark] in the containing [PdfDocument], if any.
    pub fn root(&self) -> Option<PdfBookmark> {
        let handle = self
            .bindings
            .FPDFBookmark_GetFirstChild(*self.document.get_handle(), null_mut());

        if handle.is_null() {
            None
        } else {
            Some(PdfBookmark::from_pdfium(
                handle,
                None,
                self.document,
                self.bindings,
            ))
        }
    }

    /// Returns the first [PdfBookmark] in the containing [PdfDocument] that has a title matching
    /// the given string.
    ///
    /// Note that bookmarks are not required to have unique titles, so in theory any number of
    /// bookmarks could match a given title. This function only ever returns the first. To return
    /// all matches, use [PdfBookmarks::find_all_by_title()].
    pub fn find_first_by_title(&self, title: &str) -> Option<PdfBookmark> {
        let handle = self
            .bindings
            .FPDFBookmark_Find_str(*self.document.get_handle(), title);

        if handle.is_null() {
            println!(
                "find_by_title() got null: {:#?}",
                self.bindings.get_pdfium_last_error()
            );
            None
        } else {
            Some(PdfBookmark::from_pdfium(
                handle,
                None,
                self.document,
                self.bindings,
            ))
        }
    }

    /// Returns all [PdfBookmark] objects in the containing [PdfDocument] that have a title
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

    /// Returns a breadth-first iterator over all the [PdfBookmark] objects in this [PdfDocument],
    /// starting from the top-level root bookmark.
    #[inline]
    pub fn iter(&self) -> PdfBookmarksIterator {
        PdfBookmarksIterator::new(self.root(), true, true, true, None)
    }
}

pub struct PdfBookmarksIterator<'a> {
    node: Option<PdfBookmark<'a>>,
    include_siblings: bool,
    include_direct_children: bool,
    include_all_descendants: bool,
    skip_sibling: Option<PdfBookmark<'a>>,
}

impl<'a> PdfBookmarksIterator<'a> {
    pub(crate) fn new(
        node: Option<PdfBookmark<'a>>,
        include_siblings: bool,
        include_direct_children: bool,
        include_all_descendants: bool,
        skip_sibling: Option<PdfBookmark<'a>>,
    ) -> Self {
        PdfBookmarksIterator {
            node,
            include_siblings,
            include_direct_children,
            include_all_descendants,
            skip_sibling,
        }
    }
}

impl<'a> Iterator for PdfBookmarksIterator<'a> {
    type Item = PdfBookmark<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.node = match self.node.as_ref() {
            Some(current_node) => {
                let next_sibling = if self.include_siblings {
                    match (self.skip_sibling.as_ref(), current_node.next_sibling()) {
                        (None, next_sibling) => next_sibling,
                        (Some(skip_sibling), Some(next_sibling)) => {
                            // PdfBookmark::iter_siblings() attempts to achieve consistent
                            // iteration irrespective of which sibling is used to initiate
                            // the traversal. It does this by actually iterating over the
                            // direct children of the bookmark's parent, rather than the
                            // immediate siblings of the target node. When we iterate over the
                            // siblings of the target node's parent's children, we want to
                            // skip over the target node itself. Check for this now.

                            if skip_sibling.get_handle() == next_sibling.get_handle() {
                                // This sibling was the target node that initiated iteration.
                                // Skip over it.

                                next_sibling.next_sibling()
                            } else {
                                Some(next_sibling)
                            }
                        }
                        (_, None) => None,
                    }
                } else {
                    None
                };

                if next_sibling.is_some() {
                    next_sibling
                } else if self.include_direct_children {
                    self.include_siblings = true;
                    self.include_direct_children = self.include_all_descendants;

                    current_node.first_child()
                } else {
                    None
                }
            }
            None => None,
        };

        self.node.as_ref().map(|next_node| next_node.clone())
    }
}
