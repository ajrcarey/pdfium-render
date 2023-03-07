// A macro that creates transformation functions. The created functions require the containing
// impl block to contain a private transform_impl() function. Both the specification of self and the
// data type of the return value of transform_impl() can be passed as parameters into this macro.
// This offers more flexibility than defining a trait; for instance, this macro can create functions
// that operate on either &mut self or self, whereas a trait cannot.
#[doc(hidden)]
#[macro_export]
macro_rules! create_transform_setters {
    ($self_:ty, $ret_:ty) => {
        /// Applies the given transformation, expressed as six values representing the six configurable
        /// elements of a nine-element 3x3 PDF transformation matrix, to this transformable object.
        ///
        /// Transformable objects include pages, clip paths, individual page objects, and groups of
        /// page objects (in which case the transformation is applied to each page object within the group).
        /// Transforms can also be applied to render configurations, in which case they will take effect
        /// during page rendering, and matrices, in which case they will take effect when applied to
        /// any transformable object.
        ///
        /// To move, scale, rotate, or skew this transformable object, consider using one or more of
        /// the following functions. Internally they all use [Self::transform()], but are
        /// probably easier to use (and certainly clearer in their intent) in most situations.
        ///
        /// * [Self::translate()]: changes the position of this object.
        /// * [Self::scale()]: changes the size of this object.
        /// * [Self::rotate_clockwise_degrees()], [Self::rotate_counter_clockwise_degrees()],
        /// [Self::rotate_clockwise_radians()], [Self::rotate_counter_clockwise_radians()]:
        /// rotates this object around its origin.
        /// * [Self::skew_degrees()], [Self::skew_radians()]: skews this object
        /// relative to its axes.
        ///
        /// **The order in which transformations are applied is significant.**
        /// For example, the result of rotating _then_ translating an object may be vastly different
        /// from translating _then_ rotating the same object.
        ///
        /// An overview of PDF transformation matrices can be found in the PDF Reference Manual
        /// version 1.7 on page 204; a detailed description can be founded in section 4.2.3 on page 207.
        #[inline]
        pub fn transform(
            self: $self_,
            a: PdfMatrixValue,
            b: PdfMatrixValue,
            c: PdfMatrixValue,
            d: PdfMatrixValue,
            e: PdfMatrixValue,
            f: PdfMatrixValue,
        ) -> $ret_ {
            self.transform_impl(a, b, c, d, e, f)
        }

        /// Applies the values in the given [PdfMatrix] to this transformable object.
        #[inline]
        pub fn set_matrix(self: $self_, matrix: PdfMatrix) -> $ret_ {
            self.transform(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f)
        }

        /// Moves the origin of this object by the given horizontal and vertical delta distances.
        #[inline]
        pub fn translate(self: $self_, delta_x: PdfPoints, delta_y: PdfPoints) -> $ret_ {
            self.transform(1.0, 0.0, 0.0, 1.0, delta_x.value, delta_y.value)
        }

        /// Changes the size of this object, scaling it by the given horizontal and vertical scale factors.
        #[inline]
        pub fn scale(
            self: $self_,
            horizontal_scale_factor: PdfMatrixValue,
            vertical_scale_factor: PdfMatrixValue,
        ) -> $ret_ {
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
        pub fn flip_horizontally(self: $self_) -> $ret_ {
            self.scale(-1.0, 1.0)
        }

        /// Flips this object vertically around its origin by applying a vertical scale factor of -1.
        #[inline]
        pub fn flip_vertically(self: $self_) -> $ret_ {
            self.scale(1.0, -1.0)
        }

        /// Reflects this object by flipping it both horizontally and vertically around its origin.
        #[inline]
        pub fn reflect(self: $self_) -> $ret_ {
            self.scale(-1.0, -1.0)
        }

        /// Rotates this object counter-clockwise by the given number of degrees.
        #[inline]
        pub fn rotate_counter_clockwise_degrees(self: $self_, degrees: PdfMatrixValue) -> $ret_ {
            self.rotate_counter_clockwise_radians(degrees.to_radians())
        }

        /// Rotates this object clockwise by the given number of degrees.
        #[inline]
        pub fn rotate_clockwise_degrees(self: $self_, degrees: PdfMatrixValue) -> $ret_ {
            self.rotate_counter_clockwise_degrees(-degrees)
        }

        /// Rotates this object counter-clockwise by the given number of radians.
        #[inline]
        pub fn rotate_counter_clockwise_radians(self: $self_, radians: PdfMatrixValue) -> $ret_ {
            let cos_theta = radians.cos();

            let sin_theta = radians.sin();

            self.transform(cos_theta, sin_theta, -sin_theta, cos_theta, 0.0, 0.0)
        }

        /// Rotates this object clockwise by the given number of radians.
        #[inline]
        pub fn rotate_clockwise_radians(self: $self_, radians: PdfMatrixValue) -> $ret_ {
            self.rotate_counter_clockwise_radians(-radians)
        }

        /// Skews the axes of this object by the given angles in degrees.
        #[inline]
        pub fn skew_degrees(
            self: $self_,
            x_axis_skew: PdfMatrixValue,
            y_axis_skew: PdfMatrixValue,
        ) -> $ret_ {
            self.skew_radians(x_axis_skew.to_radians(), y_axis_skew.to_radians())
        }

        /// Skews the axes of this object by the given angles in radians.
        #[inline]
        pub fn skew_radians(
            self: $self_,
            x_axis_skew: PdfMatrixValue,
            y_axis_skew: PdfMatrixValue,
        ) -> $ret_ {
            let tan_alpha = x_axis_skew.tan();

            let tan_beta = y_axis_skew.tan();

            self.transform(1.0, tan_alpha, tan_beta, 1.0, 0.0, 0.0)
        }
    };
}

// A macro that creates functions that read the current transformation matrix. The created functions
// require the containing impl block to contain a private get_matrix_impl() -> Result<PdfMatrix, PdfiumError>
// function. This could be implemented as a trait, but for the sake of consistency with the
// create_transform_setters!() macro (which could _not_ be implemented as a trait), we stick with
// using a macro.
#[doc(hidden)]
#[macro_export]
macro_rules! create_transform_getters {
    () => {
        /// Returns the transformation matrix currently applied to this transformable object.
        #[inline]
        pub fn matrix(&self) -> Result<PdfMatrix, PdfiumError> {
            self.get_matrix_impl()
        }

        /// Returns the current horizontal and vertical translation of the origin of this object.
        #[inline]
        pub fn get_translation(&self) -> (PdfPoints, PdfPoints) {
            (
                self.get_horizontal_translation(),
                self.get_vertical_translation(),
            )
        }

        /// Returns the current horizontal translation of the origin of this object.
        #[inline]
        pub fn get_horizontal_translation(&self) -> PdfPoints {
            self.matrix()
                .map(|matrix| PdfPoints::new(matrix.e))
                .unwrap_or(PdfPoints::ZERO)
        }

        /// Returns the current vertical translation of the origin of this object.
        #[inline]
        pub fn get_vertical_translation(&self) -> PdfPoints {
            self.matrix()
                .map(|matrix| PdfPoints::new(matrix.f))
                .unwrap_or(PdfPoints::ZERO)
        }

        /// Returns the current horizontal and vertical scale factors applied to this object.
        #[inline]
        pub fn get_scale(&self) -> (PdfMatrixValue, PdfMatrixValue) {
            (self.get_horizontal_scale(), self.get_vertical_scale())
        }

        /// Returns the current horizontal scale factor applied to this object.
        #[inline]
        pub fn get_horizontal_scale(&self) -> PdfMatrixValue {
            self.matrix().map(|matrix| matrix.a).unwrap_or(0.0)
        }

        /// Returns the current vertical scale factor applied to this object.
        #[inline]
        pub fn get_vertical_scale(&self) -> PdfMatrixValue {
            self.matrix().map(|matrix| matrix.d).unwrap_or(0.0)
        }

        /// Returns the counter-clockwise rotation applied to this object, in degrees.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_rotation_counter_clockwise_degrees(&self) -> PdfMatrixValue {
            self.get_rotation_counter_clockwise_radians().to_degrees()
        }

        /// Returns the clockwise rotation applied to this object, in degrees.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_rotation_clockwise_degrees(&self) -> PdfMatrixValue {
            -self.get_rotation_counter_clockwise_degrees()
        }

        /// Returns the counter-clockwise rotation applied to this object, in radians.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_rotation_counter_clockwise_radians(&self) -> PdfMatrixValue {
            self.matrix()
                .map(|matrix| matrix.b.atan2(matrix.a))
                .unwrap_or(0.0)
        }

        /// Returns the clockwise rotation applied to this object, in radians.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_rotation_clockwise_radians(&self) -> PdfMatrixValue {
            -self.get_rotation_counter_clockwise_radians()
        }

        /// Returns the current x axis and y axis skew angles applied to this object, in degrees.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_skew_degrees(&self) -> (PdfMatrixValue, PdfMatrixValue) {
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
        pub fn get_x_axis_skew_degrees(&self) -> PdfMatrixValue {
            self.get_x_axis_skew_radians().to_degrees()
        }

        /// Returns the current y axis skew applied to this object, in degrees.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_y_axis_skew_degrees(&self) -> PdfMatrixValue {
            self.get_y_axis_skew_radians().to_degrees()
        }

        /// Returns the current x axis and y axis skew angles applied to this object, in radians.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_skew_radians(&self) -> (PdfMatrixValue, PdfMatrixValue) {
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
        pub fn get_x_axis_skew_radians(&self) -> PdfMatrixValue {
            self.matrix().map(|matrix| matrix.b.atan()).unwrap_or(0.0)
        }

        /// Returns the current y axis skew applied to this object, in radians.
        ///
        /// If the object is both rotated and skewed, the return value of this function will reflect
        /// the combined operation.
        #[inline]
        pub fn get_y_axis_skew_radians(&self) -> PdfMatrixValue {
            self.matrix().map(|matrix| matrix.c.atan()).unwrap_or(0.0)
        }
    };
}
