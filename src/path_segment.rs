//! Defines the [PdfPathSegment] struct, exposing functionality related to a single
//! path segment in a `PdfPathSegments` collection.

use crate::bindgen::{
    FPDF_PATHSEGMENT, FPDF_SEGMENT_BEZIERTO, FPDF_SEGMENT_LINETO, FPDF_SEGMENT_MOVETO,
    FPDF_SEGMENT_UNKNOWN,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::page::PdfPoints;
use std::os::raw::c_float;

/// The type of a single [PdfPathSegment].
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfPathSegmentType {
    Unknown = FPDF_SEGMENT_UNKNOWN as isize,
    LineTo = FPDF_SEGMENT_LINETO as isize,
    BezierTo = FPDF_SEGMENT_BEZIERTO as isize,
    MoveTo = FPDF_SEGMENT_MOVETO as isize,
}

impl PdfPathSegmentType {
    #[inline]
    pub(crate) fn from_pdfium(segment_type: i32) -> Result<PdfPathSegmentType, PdfiumError> {
        if segment_type == FPDF_SEGMENT_UNKNOWN {
            return Ok(PdfPathSegmentType::Unknown);
        }

        match segment_type as u32 {
            FPDF_SEGMENT_LINETO => Ok(PdfPathSegmentType::LineTo),
            FPDF_SEGMENT_BEZIERTO => Ok(PdfPathSegmentType::BezierTo),
            FPDF_SEGMENT_MOVETO => Ok(PdfPathSegmentType::MoveTo),
            _ => Err(PdfiumError::UnknownPathSegmentType),
        }
    }
}

/// A single [PdfPathSegment] in a `PdfPathSegments` collection.
pub struct PdfPathSegment<'a> {
    handle: FPDF_PATHSEGMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPathSegment<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_PATHSEGMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self { handle, bindings }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfPathSegment].
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the [PdfPathSegmentType] of this [PdfPathSegment].
    #[inline]
    pub fn segment_type(&self) -> PdfPathSegmentType {
        PdfPathSegmentType::from_pdfium(self.bindings().FPDFPathSegment_GetType(self.handle))
            .unwrap_or(PdfPathSegmentType::Unknown)
    }

    /// Returns `true` if this [PdfPathSegment] closes the current sub-path.
    #[inline]
    pub fn is_close(&self) -> bool {
        self.bindings()
            .is_true(self.bindings().FPDFPathSegment_GetClose(self.handle))
    }

    /// Returns the horizontal and vertical destination positions of this [PdfPathSegment].
    pub fn point(&self) -> (PdfPoints, PdfPoints) {
        let mut x: c_float = 0.0;

        let mut y: c_float = 0.0;

        if self
            .bindings()
            .is_true(
                self.bindings()
                    .FPDFPathSegment_GetPoint(self.handle, &mut x, &mut y),
            )
        {
            (PdfPoints::new(x as f32), PdfPoints::new(y as f32))
        } else {
            (PdfPoints::ZERO, PdfPoints::ZERO)
        }
    }

    /// Returns the horizontal destination position of this [PdfPathSegment].
    #[inline]
    pub fn x(&self) -> PdfPoints {
        self.point().0
    }

    /// Returns the vertical destination position of this [PdfPathSegment].
    #[inline]
    pub fn y(&self) -> PdfPoints {
        self.point().1
    }
}
