//! Defines the [PdfFormComboBoxField] struct, exposing functionality related to a single
//! form field of type `PdfFormFieldType::ComboBox`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::document::page::field::options::PdfFormFieldOptions;
use crate::pdf::document::page::field::private::internal::PdfFormFieldPrivate;

/// A single `PdfFormField` of type `PdfFormFieldType::ComboBox`. The form field object defines
/// an interactive drop-down list widget that allows the user to either select a value
/// from a list of options or type a value into a text field.
///
/// Form fields in Pdfium are wrapped inside page annotations of type `PdfPageAnnotationType::Widget`
/// or `PdfPageAnnotationType::XfaWidget`. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// `PdfForm::field_values()` function.
pub struct PdfFormComboBoxField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    options: PdfFormFieldOptions<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormComboBoxField<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormComboBoxField {
            form_handle,
            annotation_handle,
            options: PdfFormFieldOptions::from_pdfium(form_handle, annotation_handle, bindings),
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormComboBoxField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the collection of selectable options in this [PdfFormComboBoxField].
    pub fn options(&self) -> &PdfFormFieldOptions {
        &self.options
    }

    /// Returns the displayed label for the currently selected option in this [PdfFormComboBoxField] object, if any.
    #[inline]
    pub fn value(&self) -> Option<String> {
        self.options()
            .iter()
            .find(|option| option.is_set())
            .and_then(|option| option.label().cloned())
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormComboBoxField<'a> {
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
