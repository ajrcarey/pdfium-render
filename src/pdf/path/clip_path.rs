//! Defines the [PdfClipPath] struct, exposing functionality related to a clip path.

use crate::bindgen::FPDF_CLIPPATH;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::page::object::ownership::PdfPageObjectOwnership;
use crate::pdf::path::segment::PdfPathSegment;
use crate::pdf::path::segments::{PdfPathSegmentIndex, PdfPathSegments, PdfPathSegmentsIterator};
use std::convert::TryInto;
use std::os::raw::c_int;

/// A single clip path, containing zero or more path objects.
pub struct PdfClipPath<'a> {
    handle: FPDF_CLIPPATH,
    ownership: PdfPageObjectOwnership,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfClipPath<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_CLIPPATH,
        ownership: PdfPageObjectOwnership,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            handle,
            ownership,
            bindings,
        }
    }

    /// Returns the internal `FPDF_CLIPPATH` handle for this [PdfPathSegment].
    #[inline]
    pub(crate) fn handle(&self) -> FPDF_CLIPPATH {
        self.handle
    }

    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the number of path objects inside this [PdfClipPath] instance.
    pub fn len(&self) -> i32 {
        self.bindings().FPDFClipPath_CountPaths(self.handle()) as i32
    }

    /// Returns the path objects inside this [PdfClipPath] instance.
    pub fn get_segments(&self, path_index: i32) -> PdfClipPathSegments<'a> {
        PdfClipPathSegments::from_pdfium(self.handle(), path_index, self.bindings())
    }
}

impl<'a> Drop for PdfClipPath<'a> {
    /// Closes this [PdfClipPath], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        if !self.ownership.is_owned() {
            // Responsibility for de-allocation lies with us, not Pdfium, since
            // the clip path is not attached to a page, a page object, or an annotation.

            self.bindings.FPDF_DestroyClipPath(self.handle)
        }
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
