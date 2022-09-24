//! Defines the [PdfAttachments] struct, a collection of all the `PdfAttachment` objects in a
//! `PdfDocument`.

use crate::attachment::PdfAttachment;
use crate::bindgen::FPDF_DOCUMENT;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use std::ops::{Range, RangeInclusive};
use std::os::raw::c_int;

pub type PdfAttachmentIndex = u16;

/// The collection of [PdfAttachment] objects embedded in a `PdfDocument`.
pub struct PdfAttachments<'a> {
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfAttachments<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfAttachments {
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfAttachments] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the number of attachments in this [PdfAttachments] collection.
    pub fn len(&self) -> PdfAttachmentIndex {
        self.bindings()
            .FPDFDoc_GetAttachmentCount(self.document_handle) as PdfAttachmentIndex
    }

    /// Returns `true` if this [PdfAttachments] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of attachments)` for this [PdfAttachments] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfAttachmentIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of attachments - 1)`
    /// for this [PdfAttachments] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfAttachmentIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfAttachment] from this [PdfAttachments] collection.
    pub fn get(&self, index: PdfAttachmentIndex) -> Result<PdfAttachment<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::AttachmentIndexOutOfBounds);
        }

        let handle = self
            .bindings()
            .FPDFDoc_GetAttachment(self.document_handle, index as c_int);

        if handle.is_null() {
            if let Some(error) = self.bindings().get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfAttachment::from_pdfium(
                handle,
                self.document_handle,
                self.bindings(),
            ))
        }
    }

    /// Returns an iterator over all the attachments in this [PdfAttachments] collection.
    #[inline]
    pub fn iter(&self) -> PdfAttachmentsIterator {
        PdfAttachmentsIterator::new(self)
    }
}

/// An iterator over all the [PdfAttachment] objects in a [PdfAttachments] collection.
pub struct PdfAttachmentsIterator<'a> {
    attachments: &'a PdfAttachments<'a>,
    next_index: PdfAttachmentIndex,
}

impl<'a> PdfAttachmentsIterator<'a> {
    #[inline]
    pub(crate) fn new(signatures: &'a PdfAttachments<'a>) -> Self {
        PdfAttachmentsIterator {
            attachments: signatures,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfAttachmentsIterator<'a> {
    type Item = PdfAttachment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.attachments.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
