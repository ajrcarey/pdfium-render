//! Defines the [PdfFormPushButtonField] struct, exposing functionality related to a single
//! form field of type `PdfFormFieldType::PushButton`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::document::page::field::private::internal::PdfFormFieldPrivate;

/// A single `PdfFormField` of type `PdfFormFieldType::PushButton`. The form field object defines
/// an interactive button widget that can be clicked or tapped by the user.
///
/// Form fields in Pdfium are wrapped inside page annotations of type `PdfPageAnnotationType::Widget`
/// or `PdfPageAnnotationType::XfaWidget`. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// `PdfForm::field_values()` function.
pub struct PdfFormPushButtonField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormPushButtonField<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormPushButtonField {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormPushButtonField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormPushButtonField<'a> {
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
