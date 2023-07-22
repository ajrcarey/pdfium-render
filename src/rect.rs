//! Defines the [PdfRect] struct, a rectangle measured in [PdfPoints].

use crate::bindgen::{FPDF_BOOL, FS_RECTF};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::points::PdfPoints;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

/// A rectangle measured in [PdfPoints].
///
/// The coordinate space of a `PdfPage` has its origin (0,0) at the bottom left of the page,
/// with x values increasing as coordinates move horizontally to the right and
/// y values increasing as coordinates move vertically up.
#[derive(Debug, Copy, Clone)]
pub struct PdfRect {
    pub bottom: PdfPoints,
    pub left: PdfPoints,
    pub top: PdfPoints,
    pub right: PdfPoints,
}

impl PdfRect {
    /// A [PdfRect] object with the identity value (0.0, 0.0, 0.0, 0.0).
    pub const ZERO: PdfRect = PdfRect::zero();

    /// A [PdfRect] object that encloses the entire addressable `PdfPage` coordinate space of
    /// ([-PdfPoints::MAX], [-PdfPoints::MAX], [PdfPoints::MAX], [PdfPoints::MAX]).
    pub const MAX: PdfRect = PdfRect::new(
        PdfPoints::MIN,
        PdfPoints::MIN,
        PdfPoints::MAX,
        PdfPoints::MAX,
    );

    #[inline]
    pub(crate) fn from_pdfium(rect: FS_RECTF) -> Self {
        Self {
            bottom: PdfPoints::new(rect.bottom),
            left: PdfPoints::new(rect.left),
            top: PdfPoints::new(rect.top),
            right: PdfPoints::new(rect.right),
        }
    }

    #[inline]
    pub(crate) fn from_pdfium_as_result(
        result: FPDF_BOOL,
        rect: FS_RECTF,
        bindings: &dyn PdfiumLibraryBindings,
    ) -> Result<PdfRect, PdfiumError> {
        if !bindings.is_true(result) {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfRect::from_pdfium(rect))
        }
    }

    /// Creates a new [PdfRect] from the given [PdfPoints] measurements.
    ///
    /// The coordinate space of a `PdfPage` has its origin (0,0) at the bottom left of the page,
    /// with x values increasing as coordinates move horizontally to the right and
    /// y values increasing as coordinates move vertically up.
    #[inline]
    pub const fn new(bottom: PdfPoints, left: PdfPoints, top: PdfPoints, right: PdfPoints) -> Self {
        Self {
            bottom,
            left,
            top,
            right,
        }
    }

    /// Creates a new [PdfRect] from the given raw points values.
    ///
    /// The coordinate space of a `PdfPage` has its origin (0,0) at the bottom left of the page,
    /// with x values increasing as coordinates move horizontally to the right and
    /// y values increasing as coordinates move vertically up.
    #[inline]
    pub const fn new_from_values(bottom: f32, left: f32, top: f32, right: f32) -> Self {
        Self::new(
            PdfPoints::new(bottom),
            PdfPoints::new(left),
            PdfPoints::new(top),
            PdfPoints::new(right),
        )
    }

    /// Creates a new [PdfRect] object with all values set to 0.0.
    ///
    /// Consider using the compile-time constant value [PdfRect::ZERO]
    /// rather than calling this function directly.
    #[inline]
    pub const fn zero() -> Self {
        Self::new_from_values(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns the width of this [PdfRect].
    #[inline]
    pub fn width(&self) -> PdfPoints {
        self.right - self.left
    }

    /// Returns the height of this [PdfRect].
    #[inline]
    pub fn height(&self) -> PdfPoints {
        self.top - self.bottom
    }

    #[inline]
    /// Returns `true` if the given point lies inside this [PdfRect].
    pub fn contains(&self, x: PdfPoints, y: PdfPoints) -> bool {
        self.contains_x(x) && self.contains_y(y)
    }

    #[inline]
    /// Returns `true` if the given horizontal coordinate lies inside this [PdfRect].
    pub fn contains_x(&self, x: PdfPoints) -> bool {
        self.left <= x && self.right >= x
    }

    #[inline]
    /// Returns `true` if the given vertical coordinate lies inside this [PdfRect].
    pub fn contains_y(&self, y: PdfPoints) -> bool {
        self.bottom <= y && self.top >= y
    }

    /// Returns `true` if the bounds of this [PdfRect] lie entirely within the given rectangle.
    #[inline]
    pub fn is_inside(&self, rect: &PdfRect) -> bool {
        self.left >= rect.left
            && self.right <= rect.right
            && self.top <= rect.top
            && self.bottom >= rect.bottom
    }

    /// Returns `true` if the bounds of this [PdfRect] lie at least partially within
    /// the given rectangle.
    #[inline]
    pub fn does_overlap(&self, rect: &PdfRect) -> bool {
        self.left < rect.right
            && self.right > rect.left
            && self.bottom < rect.top
            && self.top > rect.bottom
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> FS_RECTF {
        FS_RECTF {
            left: self.left.value,
            top: self.top.value,
            right: self.right.value,
            bottom: self.bottom.value,
        }
    }
}

// We could derive PartialEq automatically, but it's good practice to implement PartialEq
// by hand when implementing Hash.

impl PartialEq for PdfRect {
    fn eq(&self, other: &Self) -> bool {
        self.bottom == other.bottom
            && self.left == other.left
            && self.top == other.top
            && self.right == other.right
    }
}

// The f32 values inside PdfRect will never be NaN or Infinity, so these implementations
// of Eq and Hash are safe.

impl Eq for PdfRect {}

impl Hash for PdfRect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.bottom.value.to_bits());
        state.write_u32(self.left.value.to_bits());
        state.write_u32(self.top.value.to_bits());
        state.write_u32(self.right.value.to_bits());
    }
}

impl Display for PdfRect {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "PdfRect(bottom: {}, left: {}, top: {}, right: {}",
            self.bottom.value, self.left.value, self.top.value, self.right.value
        ))
    }
}
