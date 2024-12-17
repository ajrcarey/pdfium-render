pub(crate) mod internal {
    // We want to make the PdfPageObjectPrivate trait private while providing a blanket
    // implementation of PdfPageObjectCommon for any type T where T: PdfPageObjectPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageObjectPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{
        FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE, FPDF_PAGEOBJECT, FS_MATRIX, FS_RECTF,
    };
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::{PdfiumError, PdfiumInternalError};
    use crate::pdf::document::page::annotation::objects::PdfPageAnnotationObjects;
    use crate::pdf::document::page::object::{
        PdfPageObject, PdfPageObjectCommon, PdfPageObjectType,
    };
    use crate::pdf::document::page::objects::PdfPageObjects;
    use crate::pdf::document::page::PdfPageIndexCache;
    use crate::pdf::matrix::{PdfMatrix, PdfMatrixValue};
    use crate::pdf::quad_points::PdfQuadPoints;
    use crate::pdf::rect::PdfRect;
    use std::os::raw::c_double;

    /// Internal crate-specific functionality common to all [PdfPageObject] objects.
    pub(crate) trait PdfPageObjectPrivate<'a>: PdfPageObjectCommon<'a> {
        /// Returns the internal `FPDF_PAGEOBJECT` handle for this [PdfPageObject].
        fn get_object_handle(&self) -> FPDF_PAGEOBJECT;

        // // Returns the internal `FPDF_DOCUMENT` handle for the document containing this
        // // [PdfPageObject], if any.
        // fn get_document_handle(&self) -> Option<FPDF_DOCUMENT>;

        // /// Sets the internal `FPDF_DOCUMENT` handle for the document containing this [PdfPageObject].
        // fn set_document_handle(&mut self, document: FPDF_DOCUMENT);

        /// Returns the internal `FPDF_PAGE` handle for the page containing this
        /// [PdfPageObject], if any.
        fn get_page_handle(&self) -> Option<FPDF_PAGE>;

        /// Sets the internal `FPDF_PAGE` handle for the page containing this [PdfPageObject].
        fn set_page_handle(&mut self, page: FPDF_PAGE);

        /// Clears the internal `FPDF_PAGE` handle for the page containing this [PdfPageObject].
        /// This [PdfPageObject] is detached from any containing page.
        fn clear_page_handle(&mut self);

        /// Returns the internal `FPDF_ANNOTATION` handle for the annotation containing
        /// this [PdfPageObject], if any.
        fn get_annotation_handle(&self) -> Option<FPDF_ANNOTATION>;

        /// Sets the internal `FPDF_ANNOTATION` handle for the annotation containing this [PdfPageObject].
        fn set_annotation_handle(&mut self, annotation: FPDF_ANNOTATION);

        /// Clears the internal `FPDF_ANNOTATION` handle for the annotation containing
        /// this [PdfPageObject]. This [PdfPageObject] is detached from any containing annotation.
        #[allow(dead_code)] // TODO: AJRC - 13/6/24 - remove once clear_annotation_handle() function is in use.
        fn clear_annotation_handle(&mut self);

        /// Returns the [PdfiumLibraryBindings] used by this [PdfPageObject].
        fn bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Returns `true` if the memory allocated to this [PdfPageObject] is owned by either
        /// a containing [PdfPage] or a containing [PdfPageAnnotation].
        ///
        /// Page objects that are contained within another object do not require their
        /// data buffers to be de-allocated when references to them are dropped.
        ///
        /// Returns `false` for a [PdfPageObject] that has been created programmatically but not
        /// yet added to either an existing [PdfPage] or an existing [PdfPageAnnotation].
        #[inline]
        fn is_object_memory_owned_by_container(&self) -> bool {
            self.get_page_handle().is_some() || self.get_annotation_handle().is_some()
        }

        /// Adds this [PdfPageObject] to the given [PdfPageObjects] collection.
        // We use inversion of control here so that PdfPageObjects doesn't need to care whether
        // the page object being added is a single object or a group.
        #[inline]
        fn add_object_to_page(&mut self, page_objects: &PdfPageObjects) -> Result<(), PdfiumError> {
            self.add_object_to_page_handle(page_objects.get_page_handle())
        }

        fn add_object_to_page_handle(&mut self, page_handle: FPDF_PAGE) -> Result<(), PdfiumError> {
            self.bindings()
                .FPDFPage_InsertObject(page_handle, self.get_object_handle());

            self.set_page_handle(page_handle);

            Ok(())
        }

        /// Removes this [PdfPageObject] from the [PdfPageObjects] collection that contains it.
        // We use inversion of control here so that PdfPageObjects doesn't need to care whether
        // the page object being removed is a single object or a group.
        fn remove_object_from_page(&mut self) -> Result<(), PdfiumError> {
            if let Some(page_handle) = self.get_page_handle() {
                if self.bindings().is_true(
                    self.bindings()
                        .FPDFPage_RemoveObject(page_handle, self.get_object_handle()),
                ) {
                    self.clear_page_handle();

                    Ok(())
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            } else {
                Err(PdfiumError::PageObjectNotAttachedToPage)
            }
        }

        /// Adds this [PdfPageObject] to the given [PdfPageAnnotationObjects] collection.
        // We use inversion of control here so that PdfPageAnnotationObjects doesn't need to care
        // whether the page object being added is a single object or a group.
        #[inline]
        fn add_object_to_annotation(
            &mut self,
            annotation_objects: &PdfPageAnnotationObjects,
        ) -> Result<(), PdfiumError> {
            self.add_object_to_annotation_handle(*annotation_objects.get_annotation_handle())
        }

        fn add_object_to_annotation_handle(
            &mut self,
            annotation_handle: FPDF_ANNOTATION,
        ) -> Result<(), PdfiumError> {
            if self.bindings().is_true(
                self.bindings()
                    .FPDFAnnot_AppendObject(annotation_handle, self.get_object_handle()),
            ) {
                self.set_annotation_handle(annotation_handle);

                Ok(())
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        }

        /// Removes this [PdfPageObject] from the [PdfPageAnnotationsObjects] collection that contains it.
        // We use inversion of control here so that PdfPageAnnotationsObjects doesn't need to care
        // whether the page object being removed is a single object or a group.
        fn remove_object_from_annotation(&mut self) -> Result<(), PdfiumError> {
            if let Some(annotation_handle) = self.get_annotation_handle() {
                // Pdfium only allows removing objects from annotations by index. We must
                // perform a linear scan over the annotation's page objects.

                let index = {
                    let mut result = None;

                    for i in 0..self.bindings().FPDFAnnot_GetObjectCount(annotation_handle) {
                        if self.get_object_handle()
                            == self.bindings().FPDFAnnot_GetObject(annotation_handle, i)
                        {
                            result = Some(i);

                            break;
                        }
                    }

                    result
                };

                if let Some(index) = index {
                    if self.bindings().is_true(
                        self.bindings()
                            .FPDFAnnot_RemoveObject(annotation_handle, index),
                    ) {
                        self.clear_page_handle();

                        Ok(())
                    } else {
                        Err(PdfiumError::PdfiumLibraryInternalError(
                            PdfiumInternalError::Unknown,
                        ))
                    }
                } else {
                    Err(PdfiumError::PageObjectNotAttachedToAnnotation)
                }
            } else {
                Err(PdfiumError::PageObjectNotAttachedToAnnotation)
            }
        }

        /// Internal implementation of [PdfPageObjectCommon::has_transparency()].
        #[inline]
        fn has_transparency_impl(&self) -> bool {
            let bindings = self.bindings();

            bindings.is_true(bindings.FPDFPageObj_HasTransparency(self.get_object_handle()))
        }

        /// Internal implementation of [PdfPageObjectCommon::bounds()].
        #[inline]
        fn bounds_impl(&self) -> Result<PdfQuadPoints, PdfiumError> {
            match PdfPageObjectType::from_pdfium(
                self.bindings()
                    .FPDFPageObj_GetType(self.get_object_handle()) as u32,
            ) {
                Ok(PdfPageObjectType::Text) | Ok(PdfPageObjectType::Image) => {
                    // Text and image page objects support tight fitting bounds via the
                    // FPDFPageObject_GetRotatedBounds() function.

                    let mut points = PdfQuadPoints::ZERO.as_pdfium();

                    let result = self
                        .bindings()
                        .FPDFPageObj_GetRotatedBounds(self.get_object_handle(), &mut points);

                    PdfQuadPoints::from_pdfium_as_result(result, points, self.bindings())
                }
                _ => {
                    // All other page objects support the FPDFPageObj_GetBounds() function.

                    let mut left = 0.0;

                    let mut bottom = 0.0;

                    let mut right = 0.0;

                    let mut top = 0.0;

                    let result = self.bindings().FPDFPageObj_GetBounds(
                        self.get_object_handle(),
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
                        self.bindings(),
                    )
                    .map(|r| r.to_quad_points())
                }
            }
        }

        /// Internal implementation of [PdfPageObjectCommon::transform()].
        #[inline]
        fn transform_impl(
            &self,
            a: PdfMatrixValue,
            b: PdfMatrixValue,
            c: PdfMatrixValue,
            d: PdfMatrixValue,
            e: PdfMatrixValue,
            f: PdfMatrixValue,
        ) -> Result<(), PdfiumError> {
            self.bindings().FPDFPageObj_Transform(
                self.get_object_handle(),
                a as c_double,
                b as c_double,
                c as c_double,
                d as c_double,
                e as c_double,
                f as c_double,
            );

            // if let (Some(document_handle), Some(page_handle)) =
            //     (self.get_document_handle(), self.get_page_handle())
            // {
            //     PdfPageIndexCache::set_page_requires_content_regeneration(
            //         document_handle,
            //         page_handle,
            //     );
            // }

            Ok(())
        }

        /// Internal implementation of [PdfPageObjectCommon::matrix()].
        fn get_matrix_impl(&self) -> Result<PdfMatrix, PdfiumError> {
            let mut matrix = FS_MATRIX {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
                e: 0.0,
                f: 0.0,
            };

            if self.bindings().is_true(
                self.bindings()
                    .FPDFPageObj_GetMatrix(self.get_object_handle(), &mut matrix),
            ) {
                Ok(PdfMatrix::from_pdfium(matrix))
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        }

        /// Resets the raw transformation matrix for this page object, overwriting
        /// the existing transformation matrix.
        fn reset_matrix_impl(&self, matrix: PdfMatrix) -> Result<(), PdfiumError> {
            if self.bindings().is_true(
                self.bindings()
                    .FPDFPageObj_SetMatrix(self.get_object_handle(), &matrix.as_pdfium()),
            ) {
                // if let (Some(document_handle), Some(page_handle)) =
                //     (self.get_document_handle(), self.get_page_handle())
                // {
                //     PdfPageIndexCache::set_page_requires_content_regeneration(
                //         document_handle,
                //         page_handle,
                //     );
                // }

                Ok(())
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        }

        /// Returns `true` if this [PdfPageObject] can be successfully cloned by calling its
        /// `try_clone()` function.
        fn is_copyable_impl(&self) -> bool;

        /// Attempts to clone this [PdfPageObject] by creating a new page object and copying across
        /// all the properties of this [PdfPageObject] to the new page object.
        fn try_copy_impl<'b>(
            &self,
            document_handle: FPDF_DOCUMENT,
            bindings: &'b dyn PdfiumLibraryBindings,
        ) -> Result<PdfPageObject<'b>, PdfiumError>;
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_object_get_translation() -> Result<(), PdfiumError> {
        // Tests to make sure we can retrieve the correct horizontal and vertical translation deltas
        // from an object after applying a translation transformation.

        let pdfium = test_bind_to_pdfium();

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::RED),
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

        let pdfium = test_bind_to_pdfium();

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::RED),
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

        let pdfium = test_bind_to_pdfium();

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::RED),
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

        let pdfium = test_bind_to_pdfium();

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        let mut object = PdfPagePathObject::new_rect(
            &document,
            PdfRect::new_from_values(100.0, 100.0, 400.0, 400.0),
            Some(PdfColor::RED),
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
