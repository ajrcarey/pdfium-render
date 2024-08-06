//! Defines the [PdfQuadPoints] struct, a set of four coordinates expressed in [PdfPoints]
//! that outline the bounds of a four-sided quadrilateral.

use crate::bindgen::FS_QUADPOINTSF;
use crate::pdf::points::PdfPoints;
use crate::pdf::rect::PdfRect;
use std::fmt::{Display, Formatter};

/// A set of four coordinates expressed in [PdfPoints] that outline the bounds of a
/// four-sided quadrilateral. The coordinates specify the quadrilateral's four vertices
/// in counter-clockwise order:
/// ```
/// (x4, y4)                     (x3, y3)
///        ._____________________.
///        |                     |
///        |                     |
///        !_____________________!
/// (x1, y1)                     (x2, y2)
/// ```
/// More information on quad points can be found in Section 8.30 of the PDF Reference Manual,
/// version 1.7, on page 634.
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
            rect.top,
            rect.right,
            rect.top,
            rect.left,
            rect.bottom,
            rect.right,
            rect.bottom,
        )
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> FS_QUADPOINTSF {
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

impl Display for PdfQuadPoints {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "PdfQuadPoints(x1: {}, y1: {}, x2: {}, y2: {}, x3: {}, y3: {}, x4: {}, y4: {}",
            self.x1.value,
            self.y1.value,
            self.x2.value,
            self.y2.value,
            self.x3.value,
            self.y3.value,
            self.x4.value,
            self.y4.value
        ))
    }
}
