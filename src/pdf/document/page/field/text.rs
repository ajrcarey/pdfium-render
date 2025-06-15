//! Defines the [PdfFormTextField] struct, exposing functionality related to a single
//! form field of type [PdfFormFieldType::Text].

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE, FPDF_FORMFLAG_TEXT_MULTILINE, FPDF_FORMFLAG_TEXT_PASSWORD};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::field::private::internal::PdfFormFieldPrivate;

#[cfg(doc)]
use {
    crate::pdf::document::form::PdfForm,
    crate::pdf::document::page::annotation::PdfPageAnnotationType,
    crate::pdf::document::page::field::{PdfFormField, PdfFormFieldType},
};

/// A single [PdfFormField] of type [PdfFormFieldType::Text]. The form field object defines
/// an interactive data entry widget that allows the user to enter data by typing.
///
/// Form fields in Pdfium are wrapped inside page annotations of type [PdfPageAnnotationType::Widget]
/// or [PdfPageAnnotationType::XfaWidget]. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// [PdfForm::field_values()] function.
pub struct PdfFormTextField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormTextField<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormTextField {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormTextField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the value assigned to this [PdfFormTextField] object, if any.
    #[inline]
    pub fn value(&self) -> Option<String> {
        self.value_impl()
    }

    /// Sets the value of this [PdfFormTextField] object.
    #[inline]
    pub fn set_value(&mut self, value: &str) -> Result<(), PdfiumError> {
        self.set_value_impl(value)
    }

    /// Returns the raw form field flags for this [PdfFormTextField].
    /// 
    /// This returns the complete set of form field flags as a u32 value.
    /// You can check specific flags by using bitwise operations with the
    /// `FPDF_FORMFLAG_*` constants.
    #[inline]
    pub fn form_field_flags(&self) -> u32 {
        self.bindings().FPDFAnnot_GetFormFieldFlags(self.form_handle, self.annotation_handle) as u32
    }

    /// Returns `true` if this [PdfFormTextField] is configured as a multiline text field.
    #[inline]
    pub fn is_multiline(&self) -> bool {
        (self.form_field_flags() & FPDF_FORMFLAG_TEXT_MULTILINE) != 0
    }

    /// Returns `true` if this [PdfFormTextField] is configured as a password field.
    #[inline]
    pub fn is_password(&self) -> bool {
        (self.form_field_flags() & FPDF_FORMFLAG_TEXT_PASSWORD) != 0
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormTextField<'a> {
    #[inline]
    fn form_handle(&self) -> &FPDF_FORMHANDLE {
        &self.form_handle
    }

    #[inline]
    fn annotation_handle(&self) -> &FPDF_ANNOTATION {
        &self.annotation_handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
