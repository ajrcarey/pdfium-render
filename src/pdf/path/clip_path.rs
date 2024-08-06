//! Defines the [PdfClipPath] struct, exposing functionality related to a clip path.
#[doc(hidden)]
use crate::bindgen::FPDF_CLIPPATH;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::path::segment::PdfPathSegment;
use crate::pdf::path::segments::{PdfPathSegmentIndex, PdfPathSegments, PdfPathSegmentsIterator};
use std::convert::TryInto;
use std::os::raw::c_int;

// TODO: AJRC - 22/10/22 - "clip path" is a slight misnomer, since a single clip path can actually
// contain zero or more path objects. Each path object can then return a PdfClipPathSegments object
// that implements the PdfPathSegments trait. Want to complete implementation of top-level PdfClipPath
// collection, then add clip path support to pages and page objects.

#[allow(dead_code)]
// The PdfClipPath struct is not currently used, but we expect it to be in future
pub struct PdfClipPath<'a> {
    // TODO: AJRC - 22/10/22 - this will contain a collection of paths
    // each of which can return a PdfClipPathSegments object
    handle: FPDF_CLIPPATH,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfClipPath<'a> {
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }
}

impl<'a> Drop for PdfClipPath<'a> {
    /// Closes this [PdfClipPath], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_DestroyClipPath(self.handle)
    }
}

/// The collection of [PdfPathSegment] objects inside a single path within a clip path.
pub struct PdfClipPathSegments<'a> {
    handle: FPDF_CLIPPATH,
    path_index: c_int,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfClipPathSegments<'a> {
    #[inline]
    #[allow(dead_code)]
    // The from_pdfium() function is not currently used, but we expect it to be in future
    pub(crate) fn from_pdfium(
        handle: FPDF_CLIPPATH,
        path_index: c_int,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            handle,
            path_index,
            bindings,
        }
    }
}

impl<'a> PdfPathSegments<'a> for PdfClipPathSegments<'a> {
    #[inline]
    fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn len(&self) -> PdfPathSegmentIndex {
        self.bindings()
            .FPDFClipPath_CountPathSegments(self.handle, self.path_index)
            .try_into()
            .unwrap_or(0)
    }

    fn get(&self, index: PdfPathSegmentIndex) -> Result<PdfPathSegment<'a>, PdfiumError> {
        let handle = self.bindings().FPDFClipPath_GetPathSegment(
            self.handle,
            self.path_index,
            index as c_int,
        );

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfPathSegment::from_pdfium(handle, None, self.bindings()))
        }
    }

    #[inline]
    fn iter(&'a self) -> PdfPathSegmentsIterator<'a> {
        PdfPathSegmentsIterator::new(self)
    }
}
