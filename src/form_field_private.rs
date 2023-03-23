pub(crate) mod internal {
    // We want to make the PdfFormFieldPrivate trait private while providing a blanket
    // implementation of PdfFormFieldCommon for any type T where T: PdfFormFieldPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfFormFieldPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE, FPDF_WCHAR};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::PdfiumError;
    use crate::form_field::PdfFormFieldCommon;
    use crate::utils::mem::create_byte_buffer;
    use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;

    /// Internal crate-specific functionality common to all [PdfFormField] objects.
    pub trait PdfFormFieldPrivate<'a>: PdfFormFieldCommon {
        /// Returns the internal `FPDF_FORMHANDLE` handle for this [PdfFormField].
        fn form_handle(&self) -> &FPDF_FORMHANDLE;

        /// Returns the internal `FPDF_ANNOTATION` handle for this [PdfFormField].
        fn annotation_handle(&self) -> &FPDF_ANNOTATION;

        /// Returns the [PdfiumLibraryBindings] used by this [PdfFormField].
        fn bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Internal implementation of [PdfFormFieldCommon::name()].
        fn name_impl(&self) -> Option<String> {
            // Retrieving the field name from Pdfium is a two-step operation. First, we call
            // FPDFAnnot_GetFormFieldName() with a null buffer; this will retrieve the length of
            // the field name text in bytes. If the length is zero, then the field name is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetFormFieldName() again with a pointer to the buffer;
            // this will write the field name to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetFormFieldName(
                *self.form_handle(),
                *self.annotation_handle(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The field name is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetFormFieldName(
                    *self.form_handle(),
                    *self.annotation_handle(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Internal implementation of `value()` function shared by value-carrying form field widgets
        /// such as text fields. Not exposed directly by [PdfFormFieldCommon].
        fn value_impl(&self) -> Option<String> {
            // Retrieving the field value from Pdfium is a two-step operation. First, we call
            // FPDFAnnot_GetFormFieldValue() with a null buffer; this will retrieve the length of
            // the form value text in bytes. If the length is zero, then the form value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetFormFieldValue() again with a pointer to the buffer;
            // this will write the field value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetFormFieldValue(
                *self.form_handle(),
                *self.annotation_handle(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The field value is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetFormFieldValue(
                    *self.form_handle(),
                    *self.annotation_handle(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Internal implementation of `is_checked()` function shared by checkable form field widgets
        /// such as radio buttons and checkboxes. Not exposed directly by [PdfFormFieldCommon].
        fn is_checked_impl(&self) -> Result<bool, PdfiumError> {
            Ok(self.bindings().is_true(
                self.bindings()
                    .FPDFAnnot_IsChecked(*self.form_handle(), *self.annotation_handle()),
            ))
        }
    }
}
