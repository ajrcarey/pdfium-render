//! Defines the [PdfSignatures] struct, a collection of all the `PdfSignature` objects in a
//! `PdfDocument`.

use crate::bindgen::FPDF_DOCUMENT;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::signature::PdfSignature;
use std::ops::{Range, RangeInclusive};
use std::os::raw::c_int;

pub type PdfSignatureIndex = u16;

/// The collection of [PdfSignature] objects inside a `PdfDocument`.
pub struct PdfSignatures<'a> {
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfSignatures<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfSignatures {
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfSignatures] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the number of signatures in this [PdfSignatures] collection.
    pub fn len(&self) -> PdfSignatureIndex {
        self.bindings().FPDF_GetSignatureCount(self.document_handle) as PdfSignatureIndex
    }

    /// Returns `true` if this [PdfSignatures] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of signatures)` for this [PdfSignatures] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfSignatureIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of signatures - 1)`
    /// for this [PdfSignatures] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfSignatureIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfSignature] from this [PdfSignatures] collection.
    pub fn get(&self, index: PdfSignatureIndex) -> Result<PdfSignature<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::SignatureIndexOutOfBounds);
        }

        let handle = self
            .bindings()
            .FPDF_GetSignatureObject(self.document_handle, index as c_int);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfSignature::from_pdfium(handle, self.bindings()))
        }
    }

    /// Returns an iterator over all the signatures in this [PdfSignatures] collection.
    #[inline]
    pub fn iter(&self) -> PdfSignaturesIterator {
        PdfSignaturesIterator::new(self)
    }
}

/// An iterator over all the [PdfSignature] objects in a [PdfSignatures] collection.
pub struct PdfSignaturesIterator<'a> {
    signatures: &'a PdfSignatures<'a>,
    next_index: PdfSignatureIndex,
}

impl<'a> PdfSignaturesIterator<'a> {
    #[inline]
    pub(crate) fn new(signatures: &'a PdfSignatures<'a>) -> Self {
        PdfSignaturesIterator {
            signatures,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfSignaturesIterator<'a> {
    type Item = PdfSignature<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.signatures.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
