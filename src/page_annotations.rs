//! Defines the [PdfPageAnnotations] struct, exposing functionality related to the
//! annotations that have been added to a single `PdfPage`.

use crate::bindgen::FPDF_PAGE;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page_annotation::PdfPageAnnotation;
use std::ops::Range;
use std::os::raw::c_int;

pub type PdfPageAnnotationIndex = usize;

/// The annotations that have been added to a single `PdfPage`.
pub struct PdfPageAnnotations<'a> {
    page_handle: FPDF_PAGE,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageAnnotations<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            page_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfPageAnnotations] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the total number of annotations that have been added to the containing `PdfPage`.
    #[inline]
    pub fn len(&self) -> PdfPageAnnotationIndex {
        self.bindings.FPDFPage_GetAnnotCount(self.page_handle) as PdfPageAnnotationIndex
    }

    /// Returns true if this [PdfPageAnnotations] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from 0..(number of annotations) for this [PdfPageAnnotations] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageAnnotationIndex> {
        0..self.len()
    }

    /// Returns a single [PdfPageAnnotation] from this [PdfPageAnnotations] collection.
    pub fn get(&self, index: PdfPageAnnotationIndex) -> Result<PdfPageAnnotation, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageAnnotationIndexOutOfBounds);
        }

        let annotation_handle = self
            .bindings
            .FPDFPage_GetAnnot(self.page_handle, index as c_int);

        if annotation_handle.is_null() {
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
            Ok(PdfPageAnnotation::from_pdfium(
                annotation_handle,
                self.bindings,
            ))
        }
    }

    /// Returns an iterator over all the annotations in this [PdfPageAnnotations] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageAnnotationsIterator {
        PdfPageAnnotationsIterator::new(self)
    }
}

/// An iterator over all the [PdfPageAnnotation] objects in a [PdfPageAnnotations] collection.
pub struct PdfPageAnnotationsIterator<'a> {
    annotations: &'a PdfPageAnnotations<'a>,
    next_index: PdfPageAnnotationIndex,
}

impl<'a> PdfPageAnnotationsIterator<'a> {
    #[inline]
    pub(crate) fn new(annotations: &'a PdfPageAnnotations<'a>) -> Self {
        PdfPageAnnotationsIterator {
            annotations,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfPageAnnotationsIterator<'a> {
    type Item = PdfPageAnnotation<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.annotations.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
