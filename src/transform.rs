use crate::bindgen::FS_MATRIX;
use crate::error::PdfiumError;
use crate::page::PdfPoints;
use std::hash::{Hash, Hasher};

pub type PdfMatrixValue = f32;

/// Six floating-point values that represent the six configurable elements of a nine-element
/// 3x3 PDF transformation matrix.
///
/// Applying the matrix to a page, individual page object, or page object group effects a transformation
/// to that object. Depending on the values specified in the matrix, the object can be moved,
/// scaled, rotated, or skewed.
///
/// **It is rare that a matrix needs to be used directly.** The functions in the [Transformable] trait
/// provide convenient and expressive access to the most commonly used transformation operations
/// without requiring a matrix.
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
    /// Consider using the compile-time constant value [PdfMatrix::ZERO]
    /// rather than calling this function directly.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Creates a new [PdfMatrix] object with all matrix values set to 0.0.
    ///
    /// Consider using the compile-time constant value [PdfMatrix::ZERO]
    /// rather than calling this function directly.
    #[inline]
    pub const fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }

    /// Applies the values in this [PdfMatrix] to the given transformable object.
    #[inline]
    pub fn apply(&self, transformable: &mut impl Transformable) -> Result<(), PdfiumError> {
        transformable.transform(self.a, self.b, self.c, self.d, self.e, self.f)
    }

    #[inline]
    pub(crate) fn to_pdfium(&self) -> FS_MATRIX {
        FS_MATRIX {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
        }
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

/// Common transformation setter operations that can be applied to a variety of PDF objects,
/// including pages, individual page objects, and groups of page objects.
pub trait Transformable {
    // TODO: AJRC - 28/1/23 - apply this trait to PdfPage, PdfPageObject, and PdfPageObjectGroup.

    /// Applies the given transformation, expressed as six values representing the six configurable
    /// elements of a nine-element 3x3 PDF transformation matrix, to this page or page object.
    ///
    /// To move, scale, rotate, or skew this object, consider using one or more of the following
    /// functions. Internally they all use [Transformable::transform()], but are probably easier
    /// to use (and certainly clearer in their intent) in most situations.
    ///
    /// * [Transformable::translate()]: changes the position of this object.
    /// * [Transformable::scale()]: changes the size of this object.
    /// * [Transformable::rotate_clockwise_degrees()], [Transformable::rotate_counter_clockwise_degrees()],
    /// [Transformable::rotate_clockwise_radians()], [Transformable::rotate_counter_clockwise_radians()]:
    /// rotates this object around its origin.
    /// * [Transformable::skew_degrees()], [Transformable::skew_radians()]: skews this object
    /// relative to its axes.
    ///
    /// **The order in which transformations are applied is significant.**
    /// For example, the result of rotating _then_ translating an object may be vastly different
    /// from translating _then_ rotating the same object.
    ///
    /// An overview of PDF transformation matrices can be found in the PDF Reference Manual
    /// version 1.7 on page 204; a detailed description can be founded in section 4.2.3 on page 207.
    fn transform(
        &mut self,
        a: PdfMatrixValue,
        b: PdfMatrixValue,
        c: PdfMatrixValue,
        d: PdfMatrixValue,
        e: PdfMatrixValue,
        f: PdfMatrixValue,
    ) -> Result<(), PdfiumError>;

    /// Applies the values in the given [PdfMatrix] to this transformable object.
    #[inline]
    fn set_matrix(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError> {
        self.transform(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f)
    }

    /// Moves the origin of this object by the given horizontal and vertical delta distances.
    #[inline]
    fn translate(&mut self, delta_x: PdfPoints, delta_y: PdfPoints) -> Result<(), PdfiumError> {
        self.transform(1.0, 0.0, 0.0, 1.0, delta_x.value, delta_y.value)
    }

    /// Changes the size of this object, scaling it by the given horizontal and vertical scale factors.
    #[inline]
    fn scale(
        &mut self,
        horizontal_scale_factor: PdfMatrixValue,
        vertical_scale_factor: PdfMatrixValue,
    ) -> Result<(), PdfiumError> {
        self.transform(
            horizontal_scale_factor,
            0.0,
            0.0,
            vertical_scale_factor,
            0.0,
            0.0,
        )
    }

    /// Flips this object horizontally around its origin by applying a horizontal scale factor of -1.
    #[inline]
    fn flip_horizontally(&mut self) -> Result<(), PdfiumError> {
        self.scale(-1.0, 1.0)
    }

    /// Flips this object vertically around its origin by applying a vertical scale factor of -1.
    #[inline]
    fn flip_vertically(&mut self) -> Result<(), PdfiumError> {
        self.scale(1.0, -1.0)
    }

    /// Reflects this object by flipping it both horizontally and vertically around its origin.
    #[inline]
    fn reflect(&mut self) -> Result<(), PdfiumError> {
        self.scale(-1.0, -1.0)
    }

    /// Rotates this object counter-clockwise by the given number of degrees.
    #[inline]
    fn rotate_counter_clockwise_degrees(
        &mut self,
        degrees: PdfMatrixValue,
    ) -> Result<(), PdfiumError> {
        self.rotate_counter_clockwise_radians(degrees.to_radians())
    }

    /// Rotates this object clockwise by the given number of degrees.
    #[inline]
    fn rotate_clockwise_degrees(&mut self, degrees: PdfMatrixValue) -> Result<(), PdfiumError> {
        self.rotate_counter_clockwise_degrees(-degrees)
    }

    /// Rotates this object counter-clockwise by the given number of radians.
    #[inline]
    fn rotate_counter_clockwise_radians(
        &mut self,
        radians: PdfMatrixValue,
    ) -> Result<(), PdfiumError> {
        let cos_theta = radians.cos();

        let sin_theta = radians.sin();

        self.transform(cos_theta, sin_theta, -sin_theta, cos_theta, 0.0, 0.0)
    }

    /// Rotates this object clockwise by the given number of radians.
    #[inline]
    fn rotate_clockwise_radians(&mut self, radians: PdfMatrixValue) -> Result<(), PdfiumError> {
        self.rotate_counter_clockwise_radians(-radians)
    }

    /// Skews the axes of this object by the given angles in degrees.
    #[inline]
    fn skew_degrees(
        &mut self,
        x_axis_skew: PdfMatrixValue,
        y_axis_skew: PdfMatrixValue,
    ) -> Result<(), PdfiumError> {
        self.skew_radians(x_axis_skew.to_radians(), y_axis_skew.to_radians())
    }

    /// Skews the axes of this object by the given angles in radians.
    #[inline]
    fn skew_radians(
        &mut self,
        x_axis_skew: PdfMatrixValue,
        y_axis_skew: PdfMatrixValue,
    ) -> Result<(), PdfiumError> {
        let tan_alpha = x_axis_skew.tan();

        let tan_beta = y_axis_skew.tan();

        self.transform(1.0, tan_alpha, tan_beta, 1.0, 0.0, 0.0)
    }
}

/// Common transformation getter operations that can be applied to a variety of PDF objects,
/// including pages, individual page objects, and groups of page objects.
pub trait TransformConfiguration {
    // TODO: AJRC - 28/1/23 - apply this trait to PdfPage, PdfPageObject, and PdfPageObjectGroup,
    // and come up with a better name for the trait.

    /// Returns the transformation matrix currently applied to this transformable object.
    fn matrix(&self) -> Result<PdfMatrix, PdfiumError>;

    /// Returns the current horizontal and vertical translation of the origin of this object.
    #[inline]
    fn get_translation(&self) -> (PdfPoints, PdfPoints) {
        (
            self.get_horizontal_translation(),
            self.get_vertical_translation(),
        )
    }

    /// Returns the current horizontal translation of the origin of this object.
    #[inline]
    fn get_horizontal_translation(&self) -> PdfPoints {
        self.matrix()
            .map(|matrix| PdfPoints::new(matrix.e))
            .unwrap_or(PdfPoints::ZERO)
    }

    /// Returns the current vertical translation of the origin of this object.
    #[inline]
    fn get_vertical_translation(&self) -> PdfPoints {
        self.matrix()
            .map(|matrix| PdfPoints::new(matrix.f))
            .unwrap_or(PdfPoints::ZERO)
    }

    /// Returns the current horizontal and vertical scale factors applied to this object.
    #[inline]
    fn get_scale(&self) -> (PdfMatrixValue, PdfMatrixValue) {
        (self.get_horizontal_scale(), self.get_vertical_scale())
    }

    /// Returns the current horizontal scale factor applied to this object.
    #[inline]
    fn get_horizontal_scale(&self) -> PdfMatrixValue {
        self.matrix().map(|matrix| matrix.a).unwrap_or(0.0)
    }

    /// Returns the current vertical scale factor applied to this object.
    #[inline]
    fn get_vertical_scale(&self) -> PdfMatrixValue {
        self.matrix().map(|matrix| matrix.d).unwrap_or(0.0)
    }

    /// Returns the counter-clockwise rotation applied to this object, in degrees.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_rotation_counter_clockwise_degrees(&self) -> PdfMatrixValue {
        self.get_rotation_counter_clockwise_radians().to_degrees()
    }

    /// Returns the clockwise rotation applied to this object, in degrees.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_rotation_clockwise_degrees(&self) -> PdfMatrixValue {
        -self.get_rotation_counter_clockwise_degrees()
    }

    /// Returns the counter-clockwise rotation applied to this object, in radians.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_rotation_counter_clockwise_radians(&self) -> PdfMatrixValue {
        self.matrix()
            .map(|matrix| matrix.b.atan2(matrix.a))
            .unwrap_or(0.0)
    }

    /// Returns the clockwise rotation applied to this object, in radians.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_rotation_clockwise_radians(&self) -> PdfMatrixValue {
        -self.get_rotation_counter_clockwise_radians()
    }

    /// Returns the current x axis and y axis skew angles applied to this object, in degrees.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_skew_degrees(&self) -> (PdfMatrixValue, PdfMatrixValue) {
        (
            self.get_x_axis_skew_degrees(),
            self.get_y_axis_skew_degrees(),
        )
    }

    /// Returns the current x axis skew angle applied to this object, in degrees.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_x_axis_skew_degrees(&self) -> PdfMatrixValue {
        self.get_x_axis_skew_radians().to_degrees()
    }

    /// Returns the current y axis skew applied to this object, in degrees.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_y_axis_skew_degrees(&self) -> PdfMatrixValue {
        self.get_y_axis_skew_radians().to_degrees()
    }

    /// Returns the current x axis and y axis skew angles applied to this object, in radians.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_skew_radians(&self) -> (PdfMatrixValue, PdfMatrixValue) {
        (
            self.get_x_axis_skew_radians(),
            self.get_y_axis_skew_radians(),
        )
    }

    /// Returns the current x axis skew applied to this object, in radians.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_x_axis_skew_radians(&self) -> PdfMatrixValue {
        self.matrix().map(|matrix| matrix.b.atan()).unwrap_or(0.0)
    }

    /// Returns the current y axis skew applied to this object, in radians.
    ///
    /// If the object is both rotated and skewed, the return value of this function will reflect
    /// the combined operation.
    #[inline]
    fn get_y_axis_skew_radians(&self) -> PdfMatrixValue {
        self.matrix().map(|matrix| matrix.c.atan()).unwrap_or(0.0)
    }
}
