pub(crate) mod internal {
    // We want to make the PdfPageAnnotationPrivate trait private while providing a blanket
    // implementation of PdfPageAnnotationCommon for any type T where T: PdfPageAnnotationPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageAnnotationPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{
        FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_Color,
        FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_InteriorColor, FPDF_ANNOTATION, FPDF_OBJECT_STRING,
        FPDF_PAGEOBJECT, FPDF_WCHAR, FS_RECTF,
    };
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::{PdfiumError, PdfiumInternalError};
    use crate::pdf::color::PdfColor;
    use crate::pdf::document::page::annotation::attachment_points::PdfPageAnnotationAttachmentPoints;
    use crate::pdf::document::page::annotation::objects::PdfPageAnnotationObjects;
    use crate::pdf::document::page::annotation::{PdfPageAnnotationCommon, PdfPageAnnotationType};
    use crate::pdf::points::PdfPoints;
    use crate::pdf::rect::PdfRect;
    use crate::utils::dates::date_time_to_pdf_string;
    use crate::utils::mem::create_byte_buffer;
    use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
    use chrono::{DateTime, Utc};
    use std::os::raw::c_uint;

    /// Internal crate-specific functionality common to all [PdfPageAnnotation] objects.
    pub trait PdfPageAnnotationPrivate<'a>: PdfPageAnnotationCommon {
        /// Returns the internal `FPDF_ANNOTATION` handle for this [PdfPageAnnotation].
        fn handle(&self) -> FPDF_ANNOTATION;

        /// Returns the [PdfiumLibraryBindings] used by this [PdfPageAnnotation].
        fn bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Returns the [PdfPageAnnotationType] of this [PdfPageAnnotation].
        fn get_annotation_type(&self) -> PdfPageAnnotationType {
            PdfPageAnnotationType::from_pdfium(self.bindings().FPDFAnnot_GetSubtype(self.handle()))
                .unwrap_or(PdfPageAnnotationType::Unknown)
        }

        /// Returns the string value associated with the given key in the annotation dictionary
        /// of this [PdfPageAnnotation], if any.
        fn get_string_value(&self, key: &str) -> Option<String> {
            if !self
                .bindings()
                .is_true(self.bindings().FPDFAnnot_HasKey(self.handle(), key))
            {
                // The key does not exist.

                return None;
            }

            if self.bindings().FPDFAnnot_GetValueType(self.handle(), key) as u32
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
                self.handle(),
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
                self.handle(),
                key,
                buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                buffer_length,
            );

            assert_eq!(result, buffer_length);

            Some(get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default())
        }

        /// Sets the string value associated with the given key in the annotation dictionary
        /// of this [PdfPageAnnotation].
        fn set_string_value(&mut self, key: &str, value: &str) -> Result<(), PdfiumError> {
            // Attempt to update the modification date first, before we apply the given value update.
            // That way, if updating the date fails, we can fail early.

            #[allow(clippy::collapsible_if)] // Prefer to keep the intent clear
            if key != "M"
            // Don't update the modification date if the key we have been given to update
            // is itself the modification date!
            {
                self.set_string_value("M", &date_time_to_pdf_string(Utc::now()))?;
            }

            // With the modification date updated, we can now update the key and value
            // we were given.

            if self
                .bindings()
                .is_true(
                    self.bindings()
                        .FPDFAnnot_SetStringValue_str(self.handle(), key, value),
                )
            {
                Ok(())
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
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

            let result = self.bindings().FPDFAnnot_GetRect(self.handle(), &mut rect);

            PdfRect::from_pdfium_as_result(result, rect, self.bindings())
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_bounds()].
        #[inline]
        fn set_bounds_impl(&mut self, bounds: PdfRect) -> Result<(), PdfiumError> {
            if self.bindings().is_true(
                self.bindings()
                    .FPDFAnnot_SetRect(self.handle(), &bounds.as_pdfium()),
            ) {
                self.set_string_value("M", &date_time_to_pdf_string(Utc::now()))
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_position()].
        fn set_position_impl(&mut self, x: PdfPoints, y: PdfPoints) -> Result<(), PdfiumError> {
            let bounds = self
                .bounds()
                .unwrap_or(PdfRect::new_from_values(0.0, 0.0, 1.0, 1.0));

            let width = bounds.width();

            let height = bounds.height();

            self.set_bounds(PdfRect::new(y, x, y + height, x + width))
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_width()].
        fn set_width_impl(&mut self, width: PdfPoints) -> Result<(), PdfiumError> {
            let bounds = self
                .bounds()
                .unwrap_or(PdfRect::new_from_values(0.0, 0.0, 1.0, 1.0));

            let height = bounds.height();

            self.set_bounds(PdfRect::new(
                bounds.bottom,
                bounds.left,
                bounds.bottom + height,
                bounds.left + width,
            ))
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_height()].
        fn set_height_impl(&mut self, height: PdfPoints) -> Result<(), PdfiumError> {
            let bounds = self
                .bounds()
                .unwrap_or(PdfRect::new_from_values(0.0, 0.0, 1.0, 1.0));

            let width = bounds.width();

            self.set_bounds(PdfRect::new(
                bounds.bottom,
                bounds.left,
                bounds.bottom + height,
                bounds.left + width,
            ))
        }

        /// Internal implementation of [PdfPageAnnotationCommon::contents()].
        #[inline]
        fn contents_impl(&self) -> Option<String> {
            self.get_string_value("Contents")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_contents()].
        #[inline]
        fn set_contents_impl(&mut self, contents: &str) -> Result<(), PdfiumError> {
            self.set_string_value("Contents", contents)
        }

        /// Internal implementation of [PdfPageAnnotationCommon::creator()].
        #[inline]
        fn creator_impl(&self) -> Option<String> {
            self.get_string_value("T")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_creator()].
        #[inline]
        fn set_creator(&mut self, creator: &str) -> Result<(), PdfiumError> {
            self.set_string_value("T", creator)
        }

        /// Internal implementation of [PdfPageAnnotationCommon::creation_date()].
        #[inline]
        fn creation_date_impl(&self) -> Option<String> {
            self.get_string_value("CreationDate")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_creation_date()].
        #[inline]
        fn set_creation_date_impl(&mut self, date: DateTime<Utc>) -> Result<(), PdfiumError> {
            self.set_string_value("CreationDate", &date_time_to_pdf_string(date))
        }

        /// Internal implementation of [PdfPageAnnotationCommon::modification_date()].
        #[inline]
        fn modification_date_impl(&self) -> Option<String> {
            self.get_string_value("M")
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_modification_date()].
        #[inline]
        fn set_modification_date_impl(&mut self, date: DateTime<Utc>) -> Result<(), PdfiumError> {
            self.set_string_value("M", &date_time_to_pdf_string(date))
        }

        /// Internal implementation of [PdfPageAnnotationCommon::is_markup_annotation()].
        #[inline]
        fn is_markup_annotation_impl(&self) -> bool {
            // We take advantage of the fact that all markup annotations support attachment points,
            // and the only type of annotation (other than markup annotations) that supports
            // attachment points is the Link annotation.

            self.has_attachment_points_impl()
                && self.get_annotation_type() != PdfPageAnnotationType::Link
        }

        /// Internal implementation of [PdfPageAnnotationCommon::has_attachment_points()].
        #[inline]
        fn has_attachment_points_impl(&self) -> bool {
            self.bindings()
                .is_true(self.bindings().FPDFAnnot_HasAttachmentPoints(self.handle()))
        }

        /// Internal implementation of [PdfPageAnnotationCommon::fill_color()].
        #[inline]
        fn fill_color_impl(&self) -> Result<PdfColor, PdfiumError> {
            let mut r: c_uint = 0;

            let mut g: c_uint = 0;

            let mut b: c_uint = 0;

            let mut a: c_uint = 0;

            if self.bindings().is_true(self.bindings().FPDFAnnot_GetColor(
                self.handle(),
                FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_InteriorColor,
                &mut r,
                &mut g,
                &mut b,
                &mut a,
            )) {
                Ok(PdfColor::new(r as u8, g as u8, b as u8, a as u8))
            } else {
                // The FPDFAnnot_GetColor() function returns false if the annotation
                // is using appearance streams. In this case, the Pdfium documentation
                // states that we must use FPDFPath_GetFillColor() instead; that function
                // is deprecated, and says to use FPDFPageObj_GetFillColor().

                if self
                    .bindings()
                    .is_true(self.bindings().FPDFPageObj_GetFillColor(
                        self.handle() as FPDF_PAGEOBJECT,
                        &mut r,
                        &mut g,
                        &mut b,
                        &mut a,
                    ))
                {
                    Ok(PdfColor::new(r as u8, g as u8, b as u8, a as u8))
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            }
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_fill_color()].
        #[inline]
        fn set_fill_color_impl(&mut self, fill_color: PdfColor) -> Result<(), PdfiumError> {
            if self.bindings().is_true(self.bindings().FPDFAnnot_SetColor(
                self.handle(),
                FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_InteriorColor,
                fill_color.red() as c_uint,
                fill_color.green() as c_uint,
                fill_color.blue() as c_uint,
                fill_color.alpha() as c_uint,
            )) {
                Ok(())
            } else {
                // The FPDFAnnot_SetColor() function returns false if the annotation
                // is using appearance streams. In this case, the Pdfium documentation
                // states that we must use FPDFPath_SetFillColor() instead; that function
                // is deprecated, and says to use FPDFPageObj_SetFillColor().

                if self
                    .bindings()
                    .is_true(self.bindings().FPDFPageObj_SetFillColor(
                        self.handle() as FPDF_PAGEOBJECT,
                        fill_color.red() as c_uint,
                        fill_color.green() as c_uint,
                        fill_color.blue() as c_uint,
                        fill_color.alpha() as c_uint,
                    ))
                {
                    Ok(())
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            }
        }

        /// Internal implementation of [PdfPageAnnotationCommon::stroke_color()].
        #[inline]
        fn stroke_color_impl(&self) -> Result<PdfColor, PdfiumError> {
            let mut r: c_uint = 0;

            let mut g: c_uint = 0;

            let mut b: c_uint = 0;

            let mut a: c_uint = 0;

            if self.bindings().is_true(self.bindings().FPDFAnnot_GetColor(
                self.handle(),
                FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_Color,
                &mut r,
                &mut g,
                &mut b,
                &mut a,
            )) {
                Ok(PdfColor::new(r as u8, g as u8, b as u8, a as u8))
            } else {
                // The FPDFAnnot_GetColor() function returns false if the annotation
                // is using appearance streams. In this case, the Pdfium documentation
                // states that we must use FPDFPath_GetStrokeColor() instead; that function
                // is deprecated, and says to use FPDFPageObj_GetStrokeColor().

                if self
                    .bindings()
                    .is_true(self.bindings().FPDFPageObj_GetStrokeColor(
                        self.handle() as FPDF_PAGEOBJECT,
                        &mut r,
                        &mut g,
                        &mut b,
                        &mut a,
                    ))
                {
                    Ok(PdfColor::new(r as u8, g as u8, b as u8, a as u8))
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            }
        }

        /// Internal implementation of [PdfPageAnnotationCommon::set_stroke_color()].
        #[inline]
        fn set_stroke_color_impl(&mut self, stroke_color: PdfColor) -> Result<(), PdfiumError> {
            if self.bindings().is_true(self.bindings().FPDFAnnot_SetColor(
                self.handle(),
                FPDFANNOT_COLORTYPE_FPDFANNOT_COLORTYPE_Color,
                stroke_color.red() as c_uint,
                stroke_color.green() as c_uint,
                stroke_color.blue() as c_uint,
                stroke_color.alpha() as c_uint,
            )) {
                Ok(())
            } else {
                // The FPDFAnnot_SetColor() function returns false if the annotation
                // is using appearance streams. In this case, the Pdfium documentation
                // states that we must use FPDFPath_SetStrokeColor() instead; that function
                // is deprecated, and says to use FPDFPageObj_SetStrokeColor().

                if self
                    .bindings()
                    .is_true(self.bindings().FPDFPageObj_SetStrokeColor(
                        self.handle() as FPDF_PAGEOBJECT,
                        stroke_color.red() as c_uint,
                        stroke_color.green() as c_uint,
                        stroke_color.blue() as c_uint,
                        stroke_color.alpha() as c_uint,
                    ))
                {
                    Ok(())
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            }
        }

        /// Internal implementation of [PdfPageAnnotationCommon::objects()].
        fn objects_impl(&self) -> &PdfPageAnnotationObjects;

        /// Internal mutable accessor available for all [PdfPageAnnotation] types.
        /// This differs from the public interface, which makes mutable page object access
        /// available only for the ink annotation and stamp annotation types, since those
        /// are the only annotation types for which Pdfium itself supports adding or removing
        /// page objects.
        fn objects_mut_impl(&mut self) -> &mut PdfPageAnnotationObjects<'a>;

        /// Internal implementation of [PdfPageAnnotationCommon::attachment_points()].
        fn attachment_points_impl(&self) -> &PdfPageAnnotationAttachmentPoints;

        /// Internal mutable accessor available for all [PdfPageAnnotation] types.
        /// This differs from the public interface, which makes mutable attachment point access
        /// available only for markup annotations and the Link annotation, since those
        /// are the only annotation types for which Pdfium itself supports adding or removing
        /// attachment points.
        fn attachment_points_mut_impl(&mut self) -> &mut PdfPageAnnotationAttachmentPoints<'a>;
    }
}
