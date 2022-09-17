pub(crate) mod internal {
    // We want to make the PdfPageAnnotationPrivate trait private while providing a blanket
    // implementation of PdfPageAnnotationCommon for any type T where T: PdfPageAnnotationPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageAnnotationPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_ANNOTATION, FPDF_OBJECT_STRING, FPDF_WCHAR, FS_RECTF};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::PdfiumError;
    use crate::page::PdfRect;
    use crate::page_annotation::PdfPageAnnotationCommon;
    use crate::utils::mem::create_byte_buffer;
    use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;

    /// Internal crate-specific functionality common to all [PdfPageAnnotation] objects.
    pub trait PdfPageAnnotationPrivate: PdfPageAnnotationCommon {
        /// Returns the internal `FPDF_ANNOTATION` handle for this [PdfPageAnnotation].
        fn handle(&self) -> &FPDF_ANNOTATION;

        /// Returns the [PdfiumLibraryBindings] used by this [PdfPageAnnotation].
        fn bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Returns the string value associated with the given key in the annotation dictionary
        /// of this [PdfPageAnnotation], if any.
        fn get_string_value(&self, key: &str) -> Option<String> {
            if !self
                .bindings()
                .is_true(self.bindings().FPDFAnnot_HasKey(*self.handle(), key))
            {
                // The key does not exist.

                return None;
            }

            if self.bindings().FPDFAnnot_GetValueType(*self.handle(), key) as u32
                != FPDF_OBJECT_STRING
            {
                // The key exists, but the value associated with the key is not a string.

                return None;
            }

            // Retrieving the string value from Pdfium is a two-step operation. First, we call
            // FPDFAnot_GetStringValue() with a null buffer; this will retrieve the length of
            // the value in bytes, assuming the key exists. If the length is zero, then there
            // is no such key, or the key's value is not a string.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnot_GetStringValue() again with a pointer to the buffer;
            // this will write the string value into the buffer.

            let buffer_length = self.bindings().FPDFAnnot_GetStringValue(
                *self.handle(),
                key,
                std::ptr::null_mut(),
                0,
            );

            if buffer_length <= 2 {
                // A buffer length of 2 indicates that the string value for the given key is
                // an empty UTF16-LE string, so there is no point in retrieving it.

                return None;
            }

            let mut buffer = create_byte_buffer(buffer_length as usize);

            let result = self.bindings().FPDFAnnot_GetStringValue(
                *self.handle(),
                key,
                buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                buffer_length,
            );

            assert_eq!(result, buffer_length);

            Some(get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default())
        }

        /// Internal implementation of [PdfPageAnnotationCommon::name()].
        #[inline]
        fn name_impl(&self) -> Option<String> {
            self.get_string_value("NM")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::bounds()].
        #[inline]
        fn bounds_impl(&self) -> Result<PdfRect, PdfiumError> {
            let mut rect = FS_RECTF {
                left: 0_f32,
                bottom: 0_f32,
                right: 0_f32,
                top: 0_f32,
            };

            let result = self.bindings().FPDFAnnot_GetRect(*self.handle(), &mut rect);

            PdfRect::from_pdfium_as_result(result, rect, self.bindings())
        }

        /// Internal implementation of [PdfPageAnnotationCommon::contents()].
        #[inline]
        fn contents_impl(&self) -> Option<String> {
            self.get_string_value("Contents")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::creator()].
        #[inline]
        fn creator_impl(&self) -> Option<String> {
            self.get_string_value("T")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::creation_date()].
        #[inline]
        fn creation_date_impl(&self) -> Option<String> {
            self.get_string_value("CreationDate")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::modification_date()].
        #[inline]
        fn modification_date_impl(&self) -> Option<String> {
            self.get_string_value("M")
        }
    }
}
