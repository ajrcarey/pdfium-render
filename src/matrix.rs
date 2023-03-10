//! Defines the [PdfMatrix] struct, a container for six floating-point values that represent
//! the six configurable elements of a nine-element 3x3 PDF transformation matrix.

use crate::bindgen::FS_MATRIX;
use crate::error::PdfiumError;
use crate::page::PdfPoints;
use crate::{create_transform_getters, create_transform_setters};
use std::hash::{Hash, Hasher};
use vecmath::{mat3_det, row_mat3_mul};

pub type PdfMatrixValue = f32;

/// Six floating-point values that represent the six configurable elements of a nine-element
/// 3x3 PDF transformation matrix.
///
/// Applying the matrix to any transformable object containing a `set_matrix()` function - such as
/// a page, clip path, individual page object, or page object group - will result in a
/// transformation of that object. Depending on the values specified in the matrix, the object
/// can be moved, scaled, rotated, or skewed.
///
/// **It is rare that a matrix needs to be used directly.** All transformable objects provide
/// convenient and expressive access to the most commonly used transformation operations without
/// requiring a matrix.
///
/// However, a matrix can be convenient when the same transformation values need to be applied
/// to a large set of transformable objects.
///
/// An overview of PDF transformation matrices can be found in the PDF Reference Manual
/// version 1.7 on page 204; a detailed description can be founded in section 4.2.3 on page 207.
#[derive(Debug, Copy, Clone)]
pub struct PdfMatrix {
    pub a: PdfMatrixValue,
    pub b: PdfMatrixValue,
    pub c: PdfMatrixValue,
    pub d: PdfMatrixValue,
    pub e: PdfMatrixValue,
    pub f: PdfMatrixValue,
}

impl PdfMatrix {
    /// A [PdfMatrix] object with all matrix values set to 0.0.
    pub const ZERO: PdfMatrix = Self::zero();

    /// A [PdfMatrix] object with matrix values a and d set to 1.0
    /// and all other values set to 0.0.
    pub const IDENTITY: PdfMatrix = Self::identity();

    #[inline]
    pub(crate) fn from_pdfium(matrix: FS_MATRIX) -> Self {
        Self::new(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f)
    }

    /// Creates a new [PdfMatrix] with the given matrix values.
    #[inline]
    pub const fn new(
        a: PdfMatrixValue,
        b: PdfMatrixValue,
        c: PdfMatrixValue,
        d: PdfMatrixValue,
        e: PdfMatrixValue,
        f: PdfMatrixValue,
    ) -> Self {
        Self { a, b, c, d, e, f }
    }

    /// Creates a new [PdfMatrix] object with all matrix values set to 0.0.
    ///
    /// The return value of this function is identical to the constant [PdfMatrix::ZERO].
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Creates a new [PdfMatrix] object with matrix values a and d set to 1.0
    /// and all other values set to 0.0.
    ///
    /// The return value of this function is identical to the constant [PdfMatrix::IDENTITY].
    #[inline]
    pub const fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> FS_MATRIX {
        FS_MATRIX {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
        }
    }

    create_transform_setters!(
        Self,
        Result<Self, PdfiumError>,
        "this [PdfMatrix]",
        "this [PdfMatrix].",
        "this [PdfMatrix],"
    );

    // The internal implementation of the transform() function used by the create_transform_setters!() macro.
    fn transform_impl(
        mut self,
        a: PdfMatrixValue,
        b: PdfMatrixValue,
        c: PdfMatrixValue,
        d: PdfMatrixValue,
        e: PdfMatrixValue,
        f: PdfMatrixValue,
    ) -> Result<Self, PdfiumError> {
        let result = row_mat3_mul(
            [
                [self.a, self.b, 0.0],
                [self.c, self.d, 0.0],
                [self.e, self.f, 1.0],
            ],
            [[a, b, 0.0], [c, d, 0.0], [e, f, 1.0]],
        );

        if mat3_det(result) == 0.0 {
            Err(PdfiumError::InvalidTransformationMatrix)
        } else {
            self.a = result[0][0];
            self.b = result[0][1];
            self.c = result[1][0];
            self.d = result[1][1];
            self.e = result[2][0];
            self.f = result[2][1];

            Ok(self)
        }
    }

    create_transform_getters!("this [PdfMatrix]", "this [PdfMatrix].", "this [PdfMatrix],");

    // The internal implementation of the get_matrix_impl() function used by the create_transform_getters!() macro.
    #[inline]
    fn get_matrix_impl(&self) -> Result<PdfMatrix, PdfiumError> {
        Ok(*self)
    }
}

// We could derive PartialEq automatically, but it's good practice to implement PartialEq
// by hand when implementing Hash.

impl PartialEq for PdfMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
            && self.b == other.b
            && self.c == other.c
            && self.d == other.d
            && self.e == other.e
            && self.f == other.f
    }
}

// The PdfMatrixValue values inside PdfMatrix will never be NaN or Infinity, so these implementations
// of Eq and Hash are safe.

impl Eq for PdfMatrix {}

impl Hash for PdfMatrix {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.a.to_bits());
        state.write_u32(self.b.to_bits());
        state.write_u32(self.c.to_bits());
        state.write_u32(self.d.to_bits());
        state.write_u32(self.e.to_bits());
        state.write_u32(self.f.to_bits());
    }
}
