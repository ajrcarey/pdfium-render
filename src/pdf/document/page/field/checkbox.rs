//! Defines the [PdfFormCheckboxField] struct, exposing functionality related to a single
//! form field of type [PdfFormFieldType::Checkbox].

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::field::private::internal::PdfFormFieldPrivate;

#[cfg(doc)]
use {
    crate::pdf::document::form::PdfForm,
    crate::pdf::document::page::annotation::PdfPageAnnotationType,
    crate::pdf::document::page::field::{PdfFormField, PdfFormFieldType},
};

/// A single [PdfFormField] of type [PdfFormFieldType::Checkbox]. The form field object defines
/// an interactive checkbox widget that can be toggled by the user.
///
/// Form fields in Pdfium are wrapped inside page annotations of type [PdfPageAnnotationType::Widget]
/// or [PdfPageAnnotationType::XfaWidget]. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// [PdfForm::field_values()] function.
pub struct PdfFormCheckboxField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormCheckboxField<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormCheckboxField {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormCheckboxField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the index of this [PdfFormCheckboxField] in its control group.
    ///
    /// Control groups are used to group related interactive fields together. Checkboxes and
    /// radio buttons can be grouped such that only a single button can be selected within
    /// the control group. Each field within the group has a unique group index.
    #[inline]
    pub fn index_in_group(&self) -> u32 {
        self.index_in_group_impl()
    }

    /// Returns the value set for the control group containing this [PdfFormCheckboxField].
    ///
    /// Control groups are used to group related interactive fields together. Checkboxes and
    /// radio buttons can be grouped such that only a single button can be selected within
    /// the control group. In this case, a single value can be shared by the group, indicating
    /// the value of the currently selected field within the group.
    #[inline]
    pub fn group_value(&self) -> Option<String> {
        self.value_impl()
    }

    /// Returns `true` if this [PdfFormCheckboxField] object has its checkbox checked.
    #[inline]
    pub fn is_checked(&self) -> Result<bool, PdfiumError> {
        // The PDF Reference manual, version 1.7, states that an appearance stream of "Yes"
        // can be used to indicate a selected checkbox. Pdfium's FPDFAnnot_IsChecked()
        // function doesn't account for appearance streams, so we check the currently set
        // appearance stream ourselves.

        match self.appearance_stream_impl().as_ref() {
            Some(appearance_stream) => {
                // Appearance streams are in use. An appearance stream of "Yes" indicates
                // a selected checkbox.

                Ok(appearance_stream == "Yes" || appearance_stream == "/Yes")
            }
            None => {
                // Appearance streams are not in use. We can fall back to using Pdfium's
                // FPDFAnnot_IsChecked() implementation.

                match self.is_checked_impl() {
                    Ok(true) => Ok(true),
                    Ok(false) => match self.group_value() {
                        Some(value) => Ok(value == "Yes"),
                        _ => Ok(false),
                    },
                    Err(err) => match self.group_value() {
                        Some(value) => Ok(value == "Yes"),
                        _ => Err(err),
                    },
                }
            }
        }
    }

    /// Checks or clears the checkbox of this [PdfFormCheckboxField] object.
    #[inline]
    pub fn set_checked(&mut self, is_checked: bool) -> Result<(), PdfiumError> {
        // The manner in which we apply the value varies slightly depending on
        // whether or not appearance streams are in use.

        match self.appearance_stream_impl() {
            Some(_) => {
                // Appearance streams are in use. We must set both the value and
                // the in-use appearance stream for the checkbox.

                self.set_string_value("AS", if is_checked { "/Yes" } else { "/Off" })?;
                self.set_value_impl(if is_checked { "/Yes" } else { "/Off" })
            }
            None => {
                // Appearance streams are not in use.

                self.set_value_impl(if is_checked { "Yes" } else { "Off" })
            }
        }
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormCheckboxField<'a> {
    #[inline]
    fn form_handle(&self) -> FPDF_FORMHANDLE {
        self.form_handle
    }

    #[inline]
    fn annotation_handle(&self) -> FPDF_ANNOTATION {
        self.annotation_handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
