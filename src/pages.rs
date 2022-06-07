//! Defines the [PdfPages] struct, a collection of all the `PdfPage` objects in a
//! `PdfDocument`.

use crate::bindgen::{
    size_t, FPDF_PAGE, PAGEMODE_FULLSCREEN, PAGEMODE_UNKNOWN, PAGEMODE_USEATTACHMENTS,
    PAGEMODE_USENONE, PAGEMODE_USEOC, PAGEMODE_USEOUTLINES, PAGEMODE_USETHUMBS,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::PdfiumError::PdfiumLibraryInternalError;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPage;
use crate::page_size::PdfPagePaperSize;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::ffi::c_void;
use std::ops::{Range, RangeInclusive};
use std::os::raw::{c_double, c_int};

/// The zero-based index of a single [PdfPage] inside its containing [PdfPages] collection.
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
    /// Creates a new [PdfPages] collection from the given [PdfDocument].
    #[inline]
    pub(crate) fn new(
        document: &'a PdfDocument<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPages { document, bindings }
    }

    /// Creates a new, empty [PdfPage] with the given [PdfPagePaperSize] and inserts it
    /// at the start of this [PdfPages] collection, shuffling down all other pages.
    #[inline]
    pub fn create_page_at_start(
        &mut self,
        size: PdfPagePaperSize,
    ) -> Result<PdfPage<'a>, PdfiumError> {
        self.create_page_at_index(size, 0)
    }

    /// Creates a new, empty [PdfPage] with the given [PdfPagePaperSize] and adds it
    /// to the end of this [PdfPages] collection.
    #[inline]
    pub fn create_page_at_end(
        &mut self,
        size: PdfPagePaperSize,
    ) -> Result<PdfPage<'a>, PdfiumError> {
        self.create_page_at_index(size, self.len())
    }

    /// Creates a new, empty [PdfPage] with the given [PdfPagePaperSize] and inserts it
    /// into this [PdfPages] collection at the given page index.
    pub fn create_page_at_index(
        &mut self,
        size: PdfPagePaperSize,
        index: PdfPageIndex,
    ) -> Result<PdfPage<'a>, PdfiumError> {
        self.pdfium_page_handle_to_result(
            index,
            self.bindings.FPDFPage_New(
                *self.document.get_handle(),
                index as c_int,
                size.width().value as c_double,
                size.height().value as c_double,
            ),
        )
    }

    /// Deletes the page at the given index from this [PdfPages] collection.
    pub fn delete_page_at_index(&mut self, index: PdfPageIndex) -> Result<(), PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageIndexOutOfBounds);
        }

        self.bindings
            .FPDFPage_Delete(*self.document.get_handle(), index as c_int);

        if let Some(error) = self.bindings.get_pdfium_last_error() {
            Err(PdfiumLibraryInternalError(error))
        } else {
            Ok(())
        }
    }

    /// Copies a single page with the given source page index from the given
    /// source [PdfDocument], inserting it at the given destination page index
    /// in this [PdfPages] collection.
    pub fn copy_page_from_document(
        &mut self,
        source: &PdfDocument,
        source_page_index: PdfPageIndex,
        destination_page_index: PdfPageIndex,
    ) -> Result<(), PdfiumError> {
        self.copy_page_range_from_document(
            source,
            source_page_index..=source_page_index,
            destination_page_index,
        )
    }

    /// Copies one or more pages, specified using a user-friendly page range string,
    /// from the given source [PdfDocument], inserting the pages sequentially starting at the given
    /// destination page index in this [PdfPages] collection.
    ///
    /// The page range string should be in a comma-separated list of indexes and ranges,
    /// for example \"1,3,5-7\". Pages are indexed starting at one, not zero.
    pub fn copy_pages_from_document(
        &mut self,
        source: &PdfDocument,
        pages: &str,
        destination_page_index: PdfPageIndex,
    ) -> Result<(), PdfiumError> {
        if self.bindings.is_true(self.bindings.FPDF_ImportPages(
            *self.document.get_handle(),
            *source.get_handle(),
            pages,
            destination_page_index as c_int,
        )) {
            Ok(())
        } else if let Some(error) = self.bindings.get_pdfium_last_error() {
            Err(PdfiumError::PdfiumLibraryInternalError(error))
        } else {
            // This would be an unusual situation; a null handle indicating failure,
            // yet Pdfium's error code indicates success.

            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Copies one or more pages with the given range of indices from the given
    /// source [PdfDocument], inserting the pages sequentially starting at the given
    /// destination page index in this [PdfPages] collection.
    pub fn copy_page_range_from_document(
        &mut self,
        source: &PdfDocument,
        source_page_range: RangeInclusive<PdfPageIndex>,
        destination_page_index: PdfPageIndex,
    ) -> Result<(), PdfiumError> {
        if self.bindings.is_true(
            self.bindings.FPDF_ImportPagesByIndex_vec(
                *self.document.get_handle(),
                *source.get_handle(),
                source_page_range
                    .map(|index| index as c_int)
                    .collect::<Vec<_>>(),
                destination_page_index as c_int,
            ),
        ) {
            Ok(())
        } else if let Some(error) = self.bindings.get_pdfium_last_error() {
            Err(PdfiumError::PdfiumLibraryInternalError(error))
        } else {
            // This would be an unusual situation; a null handle indicating failure,
            // yet Pdfium's error code indicates success.

            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Creates a new [PdfDocument] by copying the pages in this [PdfPages] collection
    /// into tiled grids, the size of tile each shrinking or expanding as necessary to fit
    /// the given [PdfPagePaperSize].
    ///
    /// For example, to output all pages in this [PdfPages] collection into a new
    /// A3 landscape document with six source pages tiled on each destination page arranged
    /// into a 2 row x 3 column grid, you would call:
    ///
    /// ```
    /// PdfPages::tile_into_new_document(2, 3, PdfPagePaperSize::a3().to_landscape())
    /// ```
    pub fn tile_into_new_document(
        &self,
        rows_per_page: u8,
        columns_per_row: u8,
        size: PdfPagePaperSize,
    ) -> Result<PdfDocument, PdfiumError> {
        let handle = self.bindings.FPDF_ImportNPagesToOne(
            *self.document.get_handle(),
            size.width().value,
            size.height().value,
            columns_per_row as size_t,
            rows_per_page as size_t,
        );

        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfDocument::from_pdfium(handle, self.bindings))
        }
    }

    /// Returns the number of pages in this [PdfPages] collection.
    pub fn len(&self) -> PdfPageIndex {
        self.bindings.FPDF_GetPageCount(*self.document.get_handle()) as PdfPageIndex
    }

    /// Returns `true` if this [PdfPages] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of pages)` for this [PdfPages] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of pages - 1)` for this [PdfPages] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfPageIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfPage] from this [PdfPages] collection.
    pub fn get(&self, index: PdfPageIndex) -> Result<PdfPage<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageIndexOutOfBounds);
        }

        self.pdfium_page_handle_to_result(
            index,
            self.bindings
                .FPDF_LoadPage(*self.document.get_handle(), index as c_int),
        )
    }

    /// Returns a PdfPage from the given FPDF_PAGE handle, if possible.
    fn pdfium_page_handle_to_result(
        &self,
        index: PdfPageIndex,
        handle: FPDF_PAGE,
    ) -> Result<PdfPage<'a>, PdfiumError> {
        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            // The page's label (if any) is retrieved by index rather than by using the
            // FPDF_PAGE handle. Since the index of any particular page can change
            // (if other pages are inserted or removed), it's better if we don't treat the
            // page index as an immutable property of the PdfPage; instead, we look up the label now.

            // (Pdfium does not currently include an FPDF_SetPageLabel() function, so the label
            // _will_ be an immutable property of the PdfPage for its entire lifetime.)

            let label = {
                // Retrieving the label text from Pdfium is a two-step operation. First, we call
                // FPDF_GetPageLabel() with a null buffer; this will retrieve the length of
                // the label text in bytes. If the length is zero, then there is no such tag.

                // If the length is non-zero, then we reserve a byte buffer of the given
                // length and call FPDF_GetPageLabel() again with a pointer to the buffer;
                // this will write the label text to the buffer in UTF16LE format.

                let buffer_length = self.bindings.FPDF_GetPageLabel(
                    *self.document.get_handle(),
                    index as c_int,
                    std::ptr::null_mut(),
                    0,
                );

                if buffer_length == 0 {
                    // The label is not present.

                    None
                } else {
                    let mut buffer = create_byte_buffer(buffer_length as usize);

                    let result = self.bindings.FPDF_GetPageLabel(
                        *self.document.get_handle(),
                        index as c_int,
                        buffer.as_mut_ptr() as *mut c_void,
                        buffer_length,
                    );

                    debug_assert_eq!(result, buffer_length);

                    get_string_from_pdfium_utf16le_bytes(buffer)
                }
            };

            Ok(PdfPage::from_pdfium(
                handle,
                label,
                self.document,
                self.bindings,
            ))
        }
    }

    /// Returns the [PdfPageMode] setting embedded in the containing [PdfDocument].
    pub fn page_mode(&self) -> PdfPageMode {
        PdfPageMode::from_pdfium(
            self.bindings
                .FPDFDoc_GetPageMode(*self.document.get_handle()),
        )
        .unwrap_or(PdfPageMode::UnsetOrUnknown)
    }

    /// Returns an iterator over all the pages in this [PdfPages] collection.
    #[inline]
    pub fn iter(&self) -> PdfPagesIterator {
        PdfPagesIterator::new(self)
    }
}

/// An iterator over all the [PdfPage] objects in a [PdfPages] collection.
pub struct PdfPagesIterator<'a> {
    pages: &'a PdfPages<'a>,
    next_index: PdfPageIndex,
}

impl<'a> PdfPagesIterator<'a> {
    #[inline]
    pub(crate) fn new(pages: &'a PdfPages<'a>) -> Self {
        PdfPagesIterator {
            pages,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfPagesIterator<'a> {
    type Item = PdfPage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pages.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
