pub(crate) mod internal {
    // We want to make the PdfPageObjectPrivate trait private while providing a blanket
    // implementation of PdfPageObjectCommon for any type T where T: PdfPageObjectPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageObjectPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_PAGE, FPDF_PAGEOBJECT, FS_MATRIX, FS_RECTF};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::{PdfiumError, PdfiumInternalError};
    use crate::page::{PdfPoints, PdfRect};
    use crate::page_object::PdfPageObjectCommon;
    use crate::page_objects::PdfPageObjects;

    /// Internal crate-specific functionality common to all [PdfPageObject] objects.
    pub(crate) trait PdfPageObjectPrivate<'a>: PdfPageObjectCommon<'a> {
        /// Returns the internal `FPDF_PAGEOBJECT` handle for this [PdfPageObject].
        fn get_object_handle(&self) -> &FPDF_PAGEOBJECT;

        /// Returns the internal `FPDF_PAGE` handle for the page containing this [PdfPageObject], if any.
        fn get_page_handle(&self) -> &Option<FPDF_PAGE>;

        /// Sets the internal `FPDF_PAGE` handle for the page containing this [PdfPageObject].
        fn set_page_handle(&mut self, page: FPDF_PAGE);

        /// Clears the internal `FPDF_PAGE` handle for the page containing this [PdfPageObject].
        /// This [PdfPageObject] is detached from any containing page.
        fn clear_page_handle(&mut self);

        fn get_bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Returns `true` if the memory allocated to this [PdfPageObject] is owned by a containing
        /// [PdfPage]. Page objects that are contained within a [PdfPage] do not require their
        /// data buffers to be de-allocated when references to them are dropped. Returns `false`
        /// for a [PdfPageObject] that has been created programmatically but not yet added to an
        /// existing [PdfPage].
        #[inline]
        fn is_object_memory_owned_by_page(&self) -> bool {
            self.get_page_handle().is_some()
        }

        /// Adds this [PdfPageObject] to the given [PdfPageObjects] collection.
        // We use inversion of control here so that PdfPageObjects doesn't need to care whether
        // the page object being added is a single object or a group.
        #[inline]
        fn add_object_to_page(&mut self, page_objects: &PdfPageObjects) -> Result<(), PdfiumError> {
            self.add_object_to_page_handle(*page_objects.get_page_handle())
        }

        fn add_object_to_page_handle(&mut self, page_handle: FPDF_PAGE) -> Result<(), PdfiumError> {
            self.get_bindings()
                .FPDFPage_InsertObject(page_handle, *self.get_object_handle());

            if let Some(error) = self.get_bindings().get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                self.set_page_handle(page_handle);

                Ok(())
            }
        }

        /// Removes this [PdfPageObject] from the [PdfPageObjects] collection that contains it.
        // We use inversion of control here so that PdfPageObjects doesn't need to care whether
        // the page object being removed is a single object or a group.
        fn remove_object_from_page(&mut self) -> Result<(), PdfiumError> {
            if let Some(page_handle) = self.get_page_handle() {
                if self.get_bindings().is_true(
                    self.get_bindings()
                        .FPDFPage_RemoveObject(*page_handle, *self.get_object_handle()),
                ) {
                    self.clear_page_handle();

                    Ok(())
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        self.get_bindings()
                            .get_pdfium_last_error()
                            .unwrap_or(PdfiumInternalError::Unknown),
                    ))
                }
            } else {
                Err(PdfiumError::PageObjectNotAttachedToPage)
            }
        }

        /// Internal implementation of [PdfPageObjectCommon::has_transparency()].
        #[inline]
        fn has_transparency_impl(&self) -> bool {
            let bindings = self.get_bindings();

            bindings.is_true(bindings.FPDFPageObj_HasTransparency(*self.get_object_handle()))
        }

        /// Internal implementation of [PdfPageObjectCommon::bounds()].
        #[inline]
        fn bounds_impl(&self) -> Result<PdfRect, PdfiumError> {
            let mut left = 0.0;

            let mut bottom = 0.0;

            let mut right = 0.0;

            let mut top = 0.0;

            let result = self.get_bindings().FPDFPageObj_GetBounds(
                *self.get_object_handle(),
                &mut left,
                &mut bottom,
                &mut right,
                &mut top,
            );

            PdfRect::from_pdfium_as_result(
                result,
                FS_RECTF {
                    left,
                    top,
                    right,
                    bottom,
                },
                self.get_bindings(),
            )
        }

        /// Internal implementation of [PdfPageObjectCommon::transform()].
        #[inline]
        fn transform_impl(
            &mut self,
            a: f64,
            b: f64,
            c: f64,
            d: f64,
            e: f64,
            f: f64,
        ) -> Result<(), PdfiumError> {
            self.get_bindings()
                .FPDFPageObj_Transform(*self.get_object_handle(), a, b, c, d, e, f);

            match self.get_bindings().get_pdfium_last_error() {
                Some(err) => Err(PdfiumError::PdfiumLibraryInternalError(err)),
                None => Ok(()),
            }
        }

        /// Returns the current raw transformation matrix for this page object.
        fn matrix(&self) -> Result<FS_MATRIX, PdfiumError> {
            let mut matrix = FS_MATRIX {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
                e: 0.0,
                f: 0.0,
            };

            if self.get_bindings().is_true(
                self.get_bindings()
                    .FPDFPageObj_GetMatrix(*self.get_object_handle(), &mut matrix),
            ) {
                Ok(matrix)
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    self.get_bindings()
                        .get_pdfium_last_error()
                        .unwrap_or(PdfiumInternalError::Unknown),
                ))
            }
        }

        /// Sets the raw transformation matrix for this page object.
        fn set_matrix(&self, matrix: FS_MATRIX) -> Result<(), PdfiumError> {
            if self.get_bindings().is_true(
                self.get_bindings()
                    .FPDFPageObj_SetMatrix(*self.get_object_handle(), &matrix),
            ) {
                Ok(())
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    self.get_bindings()
                        .get_pdfium_last_error()
                        .unwrap_or(PdfiumInternalError::Unknown),
                ))
            }
        }

        /// Internal implementation of [PdfPageObjectCommon::get_horizontal_translation()].
        #[inline]
        fn get_horizontal_translation_impl(&self) -> PdfPoints {
            self.matrix()
                .map(|matrix| PdfPoints::new(matrix.e))
                .unwrap_or(PdfPoints::ZERO)
        }

        /// Internal implementation of [PdfPageObjectCommon::get_vertical_translation()].
        #[inline]
        fn get_vertical_translation_impl(&self) -> PdfPoints {
            self.matrix()
                .map(|matrix| PdfPoints::new(matrix.f))
                .unwrap_or(PdfPoints::ZERO)
        }

        /// Internal implementation of [PdfPageObjectCommon::get_horizontal_scale()].
        #[inline]
        fn get_horizontal_scale_impl(&self) -> f64 {
            self.matrix().map(|matrix| matrix.a).unwrap_or(0.0) as f64
        }

        /// Internal implementation of [PdfPageObjectCommon::get_vertical_scale()].
        #[inline]
        fn get_vertical_scale_impl(&self) -> f64 {
            self.matrix().map(|matrix| matrix.d).unwrap_or(0.0) as f64
        }

        /// Internal implementation of [PdfPageObjectCommon::get_x_axis_skew_radians()].
        #[inline]
        fn get_x_axis_skew_radians_impl(&self) -> f32 {
            self.matrix().map(|matrix| matrix.b.atan()).unwrap_or(0.0)
        }

        /// Internal implementation of [PdfPageObjectCommon::get_y_axis_skew_radians()].
        #[inline]
        fn get_y_axis_skew_radians_impl(&self) -> f32 {
            self.matrix().map(|matrix| matrix.c.atan()).unwrap_or(0.0)
        }

        /// Internal implementation of [PdfPageObjectCommon::get_rotation_counter_clockwise_radians()].
        #[inline]
        fn get_rotation_counter_clockwise_radians_impl(&self) -> f32 {
            self.matrix()
                .map(|matrix| matrix.b.atan2(matrix.a))
                .unwrap_or(0.0)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::prelude::*;
    use crate::utils::tests::tests_bind_to_pdfium;

    #[test]
    fn test_object_get_translation() -> Result<(), PdfiumError> {
        // Tests to make sure we can retrieve the correct horizontal and vertical translation deltas
        // from an object after applying a translation transformation.

        let pdfium = tests_bind_to_pdfium();

        let document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::SOLID_RED),
            Some(PdfPoints::new(1.0)),
            None,
        )?;

        object.translate(PdfPoints::new(250.0), PdfPoints::new(350.0))?;

        let object = page.objects_mut().add_path_object(object)?;

        assert_eq!(object.get_horizontal_translation().value, 250.0);
        assert_eq!(object.get_vertical_translation().value, 350.0);
        assert_eq!(object.get_horizontal_scale(), 1.0);
        assert_eq!(object.get_vertical_scale(), 1.0);
        assert_eq!(object.get_rotation_clockwise_degrees(), 0.0);

        Ok(())
    }

    #[test]
    fn test_object_get_scale() -> Result<(), PdfiumError> {
        // Tests to make sure we can retrieve the correct horizontal and vertical scale factors
        // from an object after applying a scale transformation.

        let pdfium = tests_bind_to_pdfium();

        let document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::SOLID_RED),
            Some(PdfPoints::new(1.0)),
            None,
        )?;

        object.scale(1.75, 2.25)?;

        let object = page.objects_mut().add_path_object(object)?;

        assert_eq!(object.get_horizontal_scale(), 1.75);
        assert_eq!(object.get_vertical_scale(), 2.25);
        assert_eq!(object.get_horizontal_translation().value, 0.0);
        assert_eq!(object.get_vertical_translation().value, 0.0);
        assert_eq!(object.get_rotation_clockwise_degrees(), 0.0);

        Ok(())
    }

    #[test]
    fn test_object_get_rotation() -> Result<(), PdfiumError> {
        // Tests to make sure we can retrieve the correct clockwise rotation angle from an object
        // after applying a rotation transformation.

        let pdfium = tests_bind_to_pdfium();

        let document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::SOLID_RED),
            Some(PdfPoints::new(1.0)),
            None,
        )?;

        object.rotate_clockwise_degrees(35.0)?;

        let object = page.objects_mut().add_path_object(object)?;

        assert_eq!(object.get_rotation_clockwise_degrees(), 35.0);
        assert_eq!(object.get_horizontal_translation().value, 0.0);
        assert_eq!(object.get_vertical_translation().value, 0.0);
        assert_eq!(object.get_horizontal_scale(), 0.8191520571708679); // Rotating affects the scale factors
        assert_eq!(object.get_vertical_scale(), 0.8191520571708679); // Rotating affects the scale factors

        Ok(())
    }

    #[test]
    fn test_object_get_skew() -> Result<(), PdfiumError> {
        // Tests to make sure we can retrieve the correct skew axes values from an object
        // after applying a skew transformation.

        let pdfium = tests_bind_to_pdfium();

        let document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::SOLID_RED),
            Some(PdfPoints::new(1.0)),
            None,
        )?;

        object.skew_degrees(15.5, 25.5)?;

        let object = page.objects_mut().add_path_object(object)?;

        assert_eq!(
            (object.get_x_axis_skew_degrees() * 10.0).round() / 10.0,
            15.5
        ); // Handles the returned value being a tiny bit off, e.g. 15.4999 instead of 15.5
        assert_eq!(
            (object.get_y_axis_skew_degrees() * 10.0).round() / 10.0,
            25.5
        ); // Handles the returned value being a tiny bit off, e.g. 25.4999 instead of 25.5
        assert_eq!(object.get_horizontal_translation().value, 0.0);
        assert_eq!(object.get_vertical_translation().value, 0.0);
        assert_eq!(object.get_horizontal_scale(), 1.0);
        assert_eq!(object.get_vertical_scale(), 1.0);
        assert_eq!(
            (object.get_rotation_counter_clockwise_degrees() * 10.0).round() / 10.0,
            15.5
        ); // Rotation angle will be the same as the x axis skew angle.

        Ok(())
    }
}
