//! Defines the [PdfPageLinks] struct, exposing functionality related to the
//! links contained within a single `PdfPage`.

use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::link::PdfLink;
use crate::page::PdfPoints;
use std::ops::{Range, RangeInclusive};
use std::os::raw::c_int;
use std::ptr::null_mut;

pub type PdfPageLinkIndex = usize;

pub struct PdfPageLinks<'a> {
    page_handle: FPDF_PAGE,
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageLinks<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        page_handle: FPDF_PAGE,
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageLinks {
            page_handle,
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfPageLinks] collection.
    #[inline]
    pub fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the number of links in this [PdfPageLinks] collection.
    #[inline]
    pub fn len(&self) -> PdfPageLinkIndex {
        // TODO: AJRC - 18/2/23 - since there is no FPDF_* function to return the number of
        // links in a page, we currently iterate over the entire collection. This is the least
        // efficient way possible we could do this; revise this to use a binary search approach.

        self.iter().count() as PdfPageLinkIndex
    }

    /// Returns `true` if this [PdfPageLinks] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of links)` for this [PdfPageLinks] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageLinkIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of links - 1)` for this [PdfPageLinks] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfPageLinkIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfLink] from this [PdfPageLinks] collection.
    pub fn get(&'a self, index: PdfPageLinkIndex) -> Result<PdfLink<'a>, PdfiumError> {
        // TODO: AJRC - 18/2/23 - since we cannot retrieve links in random access order,
        // we iterate over the entire collection. A linear traversal is the least efficient
        // way possible we could do this; revise this to use a binary search approach.

        for (link_index, link) in self.iter().enumerate() {
            if link_index == index {
                return Ok(link);
            }
        }

        Err(PdfiumError::LinkIndexOutOfBounds)
    }

    /// Returns the first [PdfLink] object in this [PdfPageLinks] collection.
    #[inline]
    pub fn first(&'a self) -> Result<PdfLink<'a>, PdfiumError> {
        if !self.is_empty() {
            self.get(0)
        } else {
            Err(PdfiumError::NoPageObjectsInCollection)
        }
    }

    /// Returns the last [PdfLink] object in this [PdfPageLinks] collection.
    #[inline]
    pub fn last(&'a self) -> Result<PdfLink<'a>, PdfiumError> {
        if !self.is_empty() {
            self.get(self.len() - 1)
        } else {
            Err(PdfiumError::NoPageObjectsInCollection)
        }
    }

    /// Returns the [PdfLink] object at the given position on the containing page, if any.
    pub fn link_at_point(&self, x: PdfPoints, y: PdfPoints) -> Option<PdfLink> {
        let handle =
            self.bindings
                .FPDFLink_GetLinkAtPoint(self.page_handle, x.value as f64, y.value as f64);

        if handle.is_null() {
            None
        } else {
            Some(PdfLink::from_pdfium(
                handle,
                self.document_handle,
                self.bindings,
            ))
        }
    }

    /// Returns an iterator over all the [PdfLink] objects in this [PdfPageLinks] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageLinksIterator {
        PdfPageLinksIterator::new(self)
    }
}

/// An iterator over all the [PdfLink] objects in a [PdfPageLinksIterator] collection.
pub struct PdfPageLinksIterator<'a> {
    links: &'a PdfPageLinks<'a>,
    start_pos: c_int,
}

impl<'a> PdfPageLinksIterator<'a> {
    #[inline]
    pub(crate) fn new(links: &'a PdfPageLinks<'a>) -> Self {
        PdfPageLinksIterator {
            links,
            start_pos: 0,
        }
    }
}

impl<'a> Iterator for PdfPageLinksIterator<'a> {
    type Item = PdfLink<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut handle = null_mut();

        if self
            .links
            .bindings
            .is_true(self.links.bindings.FPDFLink_Enumerate(
                self.links.page_handle,
                &mut self.start_pos,
                &mut handle,
            ))
        {
            Some(PdfLink::from_pdfium(
                handle,
                self.links.document_handle,
                self.links.bindings,
            ))
        } else {
            None
        }
    }
}
