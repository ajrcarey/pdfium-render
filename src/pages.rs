//! Defines the [PdfPages] struct, a collection of all the `PdfPage` objects in a
//! `PdfDocument`.

use crate::bindgen::{
    PAGEMODE_FULLSCREEN, PAGEMODE_UNKNOWN, PAGEMODE_USEATTACHMENTS, PAGEMODE_USENONE,
    PAGEMODE_USEOC, PAGEMODE_USEOUTLINES, PAGEMODE_USETHUMBS,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPage;
use std::ops::Range;
use std::os::raw::c_int;

/// The zero-based unique index of a single [PdfPage] inside its containing [PdfPages] collection.
pub type PdfPageIndex = u16;

/// A hint to a PDF document reader (such as Adobe Acrobat) as to how the creator intended
/// the [PdfPage] objects in a [PdfDocument] to be displayed to the viewer when the document is opened.
#[derive(Debug, Copy, Clone)]
pub enum PdfPageMode {
    /// No known page mode is set for this [PdfDocument].
    UnsetOrUnknown = PAGEMODE_UNKNOWN as isize,

    /// No page mode, i.e. neither the document outline nor thumbnail images should be visible,
    /// no side panels should be visible, and the document should not be displayed in full screen mode.
    None = PAGEMODE_USENONE as isize,

    /// Outline page mode: the document outline should be visible.
    ShowDocumentOutline = PAGEMODE_USEOUTLINES as isize,

    /// Thumbnail page mode: page thumbnails should be visible.
    ShowPageThumbnails = PAGEMODE_USETHUMBS as isize,

    /// Fullscreen page mode: no menu bar, window controls, or other windows should be visible.
    Fullscreen = PAGEMODE_FULLSCREEN as isize,

    /// The optional content group panel should be visible.
    ShowContentGroupPanel = PAGEMODE_USEOC as isize,

    /// The attachments panel should be visible.
    ShowAttachmentsPanel = PAGEMODE_USEATTACHMENTS as isize,
}

impl PdfPageMode {
    #[inline]
    pub(crate) fn from_pdfium(page_mode: i32) -> Option<Self> {
        // The PAGEMODE_* enum constants are a mixture of i32 and u32 values :/

        if page_mode == PAGEMODE_UNKNOWN {
            return Some(PdfPageMode::UnsetOrUnknown);
        }

        match page_mode as u32 {
            PAGEMODE_USENONE => Some(PdfPageMode::None),
            PAGEMODE_USEOUTLINES => Some(PdfPageMode::ShowDocumentOutline),
            PAGEMODE_USETHUMBS => Some(PdfPageMode::ShowPageThumbnails),
            PAGEMODE_FULLSCREEN => Some(PdfPageMode::Fullscreen),
            PAGEMODE_USEOC => Some(PdfPageMode::ShowContentGroupPanel),
            PAGEMODE_USEATTACHMENTS => Some(PdfPageMode::ShowAttachmentsPanel),
            _ => None,
        }
    }
}

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

    /// Returns the [PdfPageMode] setting embedded in the containing [PdfDocument].
    pub fn page_mode(&self) -> PdfPageMode {
        PdfPageMode::from_pdfium(
            self.bindings
                .FPDFDoc_GetPageMode(*self.document.get_handle()),
        )
        .unwrap_or(PdfPageMode::UnsetOrUnknown)
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
