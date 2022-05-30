pub(crate) mod internal {
    // We want to make the PdfPageAnnotationPrivate trait private while providing a blanket
    // implementation of PdfPageAnnotationCommon for any type T where T: PdfPageAnnotationPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageAnnotationPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_ANNOTATION, FS_RECTF};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::PdfiumError;
    use crate::page::PdfRect;
    use crate::page_annotation::PdfPageAnnotationCommon;

    /// Internal crate-specific functionality common to all [PdfPageAnnotation] objects.
    pub trait PdfPageAnnotationPrivate: PdfPageAnnotationCommon {
        /// Returns the internal FPDF_ANNOTATION handle for this [PdfPageANnotation].
        fn get_handle(&self) -> &FPDF_ANNOTATION;

        fn get_bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Internal implementation of [PdfPageObjectCommon::bounding()].
        #[inline]
        fn bounds_impl(&self) -> Result<PdfRect, PdfiumError> {
            let mut rect = FS_RECTF {
                left: 0_f32,
                bottom: 0_f32,
                right: 0_f32,
                top: 0_f32,
            };

            let result = self
                .get_bindings()
                .FPDFAnnot_GetRect(*self.get_handle(), &mut rect);

            PdfRect::from_pdfium_as_result(result, rect, self.get_bindings())
        }
    }
}
