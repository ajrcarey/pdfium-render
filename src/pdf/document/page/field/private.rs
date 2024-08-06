pub(crate) mod internal {
    // We want to make the PdfFormFieldPrivate trait private while providing a blanket
    // implementation of PdfFormFieldCommon for any type T where T: PdfFormFieldPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfFormFieldPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{
        FPDF_ANNOTATION, FPDF_ANNOT_FLAG_HIDDEN, FPDF_ANNOT_FLAG_INVISIBLE, FPDF_ANNOT_FLAG_LOCKED,
        FPDF_ANNOT_FLAG_NONE, FPDF_ANNOT_FLAG_NOROTATE, FPDF_ANNOT_FLAG_NOVIEW,
        FPDF_ANNOT_FLAG_NOZOOM, FPDF_ANNOT_FLAG_PRINT, FPDF_ANNOT_FLAG_READONLY,
        FPDF_ANNOT_FLAG_TOGGLENOVIEW, FPDF_FORMHANDLE, FPDF_WCHAR,
    };
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::PdfiumError;
    use crate::pdf::appearance_mode::PdfAppearanceMode;
    use crate::pdf::document::page::field::PdfFormFieldCommon;
    use crate::utils::dates::date_time_to_pdf_string;
    use crate::utils::mem::create_byte_buffer;
    use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
    use bitflags::bitflags;
    use chrono::Utc;
    use std::os::raw::c_int;

    bitflags! {
        pub struct FpdfAnnotationFlags: u32 {
             const None = FPDF_ANNOT_FLAG_NONE;
             const Invisible = FPDF_ANNOT_FLAG_INVISIBLE;
             const Hidden = FPDF_ANNOT_FLAG_HIDDEN;
             const Print = FPDF_ANNOT_FLAG_PRINT;
             const NoZoom = FPDF_ANNOT_FLAG_NOZOOM;
             const NoRotate = FPDF_ANNOT_FLAG_NOROTATE;
             const NoView = FPDF_ANNOT_FLAG_NOVIEW;
             const ReadOnly = FPDF_ANNOT_FLAG_READONLY;
             const Locked = FPDF_ANNOT_FLAG_LOCKED;
             const ToggleNoView = FPDF_ANNOT_FLAG_TOGGLENOVIEW;
        }
    }

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

        /// Internal implementation of `set_value()` function shared by value-carrying form
        /// field widgets such as text fields. Not exposed directly by [PdfFormFieldCommon].
        #[inline]
        fn set_value_impl(&mut self, value: &str) -> Result<(), PdfiumError> {
            self.bindings()
                .to_result(self.bindings().FPDFAnnot_SetStringValue_str(
                    *self.annotation_handle(),
                    "M",
                    &date_time_to_pdf_string(Utc::now()),
                ))
                .and_then(|_| {
                    self.bindings().to_result(self.bindings().FPDFAnnot_SetAP(
                        *self.annotation_handle(),
                        PdfAppearanceMode::Normal as i32,
                        std::ptr::null(),
                    ))
                })
                .and_then(|_| {
                    self.bindings()
                        .to_result(self.bindings().FPDFAnnot_SetStringValue_str(
                            *self.annotation_handle(),
                            "V",
                            value,
                        ))
                })
        }

        /// Internal implementation of `export_value()` function shared by on/off form field widgets
        /// such as checkbox and radio button fields. Not exposed directly by [PdfFormFieldCommon].
        fn export_value_impl(&self) -> Option<String> {
            // Retrieving the export value from Pdfium is a two-step operation. First, we call
            // FPDFAnnot_GetFormFieldExportValue() with a null buffer; this will retrieve the length of
            // the export value text in bytes. If the length is zero, then the export value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetFormFieldExportValue() again with a pointer to the buffer;
            // this will write the export value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetFormFieldExportValue(
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

                let result = self.bindings().FPDFAnnot_GetFormFieldExportValue(
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

        /// Internal implementation of `index_in_group()` function shared by checkable form field
        /// widgets such as radio buttons and checkboxes. Not exposed directly by [PdfFormFieldCommon].
        fn index_in_group_impl(&self) -> u32 {
            let result = self
                .bindings()
                .FPDFAnnot_GetFormControlIndex(*self.form_handle(), *self.annotation_handle());

            if result < 0 {
                // Pdfium uses a -1 value to signal an error.

                0
            } else {
                result as u32
            }
        }
        /// Internal implementation of [PdfFormFieldCommon::appearance_mode_value()].
        fn appearance_mode_value_impl(&self, appearance_mode: PdfAppearanceMode) -> Option<String> {
            // Retrieving the appearance mode value from Pdfium is a two-step operation.
            // First, we call FPDFAnnot_GetAP() with a null buffer; this will retrieve the length of
            // the appearance mode value text in bytes. If the length is zero, then the
            // appearance mode value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetAP() again with a pointer to the buffer;
            // this will write the appearance mode value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetAP(
                *self.annotation_handle(),
                appearance_mode.as_pdfium(),
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The appearance mode value is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetAP(
                    *self.annotation_handle(),
                    appearance_mode.as_pdfium(),
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        /// Returns the currently set appearance stream for this form field, if any.
        fn appearance_stream_impl(&self) -> Option<String> {
            // Retrieving the appearance stream value from Pdfium is a two-step operation.
            // First, we call FPDFAnnot_GetStringValue() with a null buffer; this will retrieve
            // the length of the appearance stream value text in bytes. If the length is zero,
            // then the appearance stream value is not set.

            // If the length is non-zero, then we reserve a byte buffer of the given
            // length and call FPDFAnnot_GetStringValue() again with a pointer to the buffer;
            // this will write the appearance stream value to the buffer in UTF16LE format.

            let buffer_length = self.bindings().FPDFAnnot_GetStringValue(
                *self.annotation_handle(),
                "AS",
                std::ptr::null_mut(),
                0,
            );

            if buffer_length == 0 {
                // The appearance mode value is not present.

                None
            } else {
                let mut buffer = create_byte_buffer(buffer_length as usize);

                let result = self.bindings().FPDFAnnot_GetStringValue(
                    *self.annotation_handle(),
                    "AS",
                    buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                    buffer_length,
                );

                debug_assert_eq!(result, buffer_length);

                get_string_from_pdfium_utf16le_bytes(buffer)
            }
        }

        #[inline]
        fn form_field_flags_impl(&self) -> FpdfAnnotationFlags {
            FpdfAnnotationFlags::from_bits_truncate(
                self.bindings()
                    .FPDFAnnot_GetFormFieldFlags(*self.form_handle(), *self.annotation_handle())
                    as u32,
            )
        }

        #[inline]
        fn flags_impl(&self) -> FpdfAnnotationFlags {
            FpdfAnnotationFlags::from_bits_truncate(
                self.bindings()
                    .FPDFAnnot_GetFlags(*self.annotation_handle()) as u32,
            )
        }

        #[inline]
        fn set_flags_impl(&mut self, flags: FpdfAnnotationFlags) -> bool {
            self.bindings().is_true(
                self.bindings()
                    .FPDFAnnot_SetFlags(*self.annotation_handle(), flags.bits() as c_int),
            )
        }
    }
}
