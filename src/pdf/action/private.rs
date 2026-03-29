pub(crate) mod internal {
    // We want to make the PdfActionPrivate trait private while providing a blanket
    // implementation of PdfActionCommon for any type T where T: PdfActionPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfActionPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::FPDF_ACTION;
    use crate::pdf::action::PdfActionCommon;
    use crate::pdfium::PdfiumLibraryBindingsAccessor;

    /// Internal crate-specific functionality common to all [PdfAction] actions.
    pub(crate) trait PdfActionPrivate<'a>:
        PdfActionCommon<'a> + PdfiumLibraryBindingsAccessor<'a>
    {
        /// Returns the internal `FPDF_ACTION` handle for this [PdfAction].
        #[allow(dead_code)] // This function is not currently used, but we expect it to be in future
        fn handle(&self) -> &FPDF_ACTION;
    }
}
