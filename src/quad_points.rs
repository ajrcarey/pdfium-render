//! Defines the [PdfQuadPoints] struct, a set of four coordinates expressed in [PdfPoints]
//! that outline the bounds of a four-sided quadrilateral.

use crate::bindgen::FS_QUADPOINTSF;
use crate::points::PdfPoints;
use crate::rect::PdfRect;

/// A set of four coordinates expressed in [PdfPoints] that outline the bounds of a
/// four-sided quadrilateral.
#[derive(Debug, Copy, Clone)]
pub struct PdfQuadPoints {
    pub x1: PdfPoints,
    pub y1: PdfPoints,
    pub x2: PdfPoints,
    pub y2: PdfPoints,
    pub x3: PdfPoints,
    pub y3: PdfPoints,
    pub x4: PdfPoints,
    pub y4: PdfPoints,
}

impl PdfQuadPoints {
    #[inline]
    pub(crate) fn from_pdfium(points: FS_QUADPOINTSF) -> Self {
        PdfQuadPoints::new_from_values(
            points.x1, points.y1, points.x2, points.y2, points.x3, points.y3, points.x4, points.y4,
        )
    }

    /// Creates a new [PdfQuadPoints] from the given [PdfPoints] coordinate pairs.
    ///
    /// The coordinate space of a `PdfPage` has its origin (0,0) at the bottom left of the page,
    /// with x values increasing as coordinates move horizontally to the right and
    /// y values increasing as coordinates move vertically up.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x1: PdfPoints,
        y1: PdfPoints,
        x2: PdfPoints,
        y2: PdfPoints,
        x3: PdfPoints,
        y3: PdfPoints,
        x4: PdfPoints,
        y4: PdfPoints,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            x4,
            y4,
        }
    }

    /// Creates a new [PdfQuadPoints] from the given raw points values.
    ///
    /// The coordinate space of a `PdfPage` has its origin (0,0) at the bottom left of the page,
    /// with x values increasing as coordinates move horizontally to the right and
    /// y values increasing as coordinates move vertically up.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_values(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
    ) -> Self {
        Self {
            x1: PdfPoints::new(x1),
            y1: PdfPoints::new(y1),
            x2: PdfPoints::new(x2),
            y2: PdfPoints::new(y2),
            x3: PdfPoints::new(x3),
            y3: PdfPoints::new(y3),
            x4: PdfPoints::new(x4),
            y4: PdfPoints::new(y4),
        }
    }

    /// Creates a new [PdfQuadPoints] from the given [PdfRect].
    #[inline]
    pub fn from_rect(rect: PdfRect) -> Self {
        PdfQuadPoints::new(
            rect.left,
            rect.bottom,
            rect.left,
            rect.top,
            rect.right,
            rect.top,
            rect.right,
            rect.bottom,
        )
    }

    #[inline]
    pub(crate) fn to_pdfium(&self) -> FS_QUADPOINTSF {
        FS_QUADPOINTSF {
            x1: self.x1.value,
            y1: self.y1.value,
            x2: self.x2.value,
            y2: self.y2.value,
            x3: self.x3.value,
            y3: self.y3.value,
            x4: self.x4.value,
            y4: self.y4.value,
        }
    }
}
