pub(crate) mod internal {
    // We want to make the PdfPageObjectPrivate trait private while providing a blanket
    // implementation of PdfPageObjectCommon for any type T where T: PdfPageObjectPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageObjectPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_PAGEOBJECT, FS_MATRIX, FS_RECTF};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::{PdfiumError, PdfiumInternalError};
    use crate::page::PdfRect;
    use crate::page_object::PdfPageObjectCommon;
    use crate::page_objects::PdfPageObjects;

    /// Internal crate-specific functionality common to all [PdfPageObject] objects.
    pub trait PdfPageObjectPrivate<'a>: PdfPageObjectCommon<'a> {
        /// Returns the internal FPDF_PAGEOBJECT handle for this [PdfPageObject].
        fn get_handle(&self) -> &FPDF_PAGEOBJECT;

        fn get_bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Returns `true` if the memory allocated to this [PdfPageObject] is owned by a containing
        /// [PdfPage]. Page objects that are contained within a [PdfPage] do not require their
        /// data buffers to be de-allocated when references to them are dropped. Returns `false`
        /// for a [PdfPageObject] that has been created programmatically but not yet added to an
        /// existing [PdfPage].
        fn is_object_memory_owned_by_page(&self) -> bool;

        /// Adds this [PdfPageObject] to the given [PdfPageObjects] collection.
        // We use inversion of control here so that PdfPageObjects doesn't need to care whether
        // the page object being added is a single object or a group.
        fn add_object_to_page(&mut self, page_objects: &PdfPageObjects) -> Result<(), PdfiumError>;

        /// Removes this [PdfPageObject] from the [PdfPageObjects] collection that contains it.
        // We use inversion of control here so that PdfPageObjects doesn't need to care whether
        // the page object being removed is a single object or a group.
        fn remove_object_from_page(&mut self) -> Result<(), PdfiumError>;

        /// Internal implementation of [PdfPageObjectCommon::has_transparency()].
        #[inline]
        fn has_transparency_impl(&self) -> bool {
            let bindings = self.get_bindings();

            bindings.is_true(bindings.FPDFPageObj_HasTransparency(*self.get_handle()))
        }

        /// Internal implementation of [PdfPageObjectCommon::bounds()].
        #[inline]
        fn bounds_impl(&self) -> Result<PdfRect, PdfiumError> {
            let mut left = 0.0;

            let mut bottom = 0.0;

            let mut right = 0.0;

            let mut top = 0.0;

            let result = self.get_bindings().FPDFPageObj_GetBounds(
                *self.get_handle(),
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
        fn transform_impl(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
            self.get_bindings()
                .FPDFPageObj_Transform(*self.get_handle(), a, b, c, d, e, f)
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
                    .FPDFPageObj_GetMatrix(*self.get_handle(), &mut matrix),
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
                    .FPDFPageObj_SetMatrix(*self.get_handle(), &matrix),
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
    }
}
