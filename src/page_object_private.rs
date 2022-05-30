pub(crate) mod internal {
    // We want to make the PdfPageObjectPrivate trait private while providing a blanket
    // implementation of PdfPageObjectCommon for any type T where T: PdfPageObjectPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageObjectPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_PAGE, FPDF_PAGEOBJECT, FS_RECTF};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::PdfiumError;
    use crate::page::PdfRect;
    use crate::page_object::PdfPageObjectCommon;

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

        /// Updates the memory ownership of this [PdfPageObject].
        fn set_object_memory_owned_by_page(&mut self, page: FPDF_PAGE);

        /// Updates the memory ownership of this [PdfPageObject].
        fn set_object_memory_released_by_page(&mut self);

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
    }
}
