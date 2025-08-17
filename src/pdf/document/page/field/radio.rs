//! Defines the [PdfFormRadioButtonField] struct, exposing functionality related to a single
//! form field of type [PdfFormFieldType::RadioButton].

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::field::private::internal::{
    PdfFormFieldFlags, PdfFormFieldPrivate,
};

#[cfg(doc)]
use {
    crate::pdf::document::form::PdfForm,
    crate::pdf::document::page::annotation::PdfPageAnnotationType,
    crate::pdf::document::page::field::{PdfFormField, PdfFormFieldType},
};

/// A single [PdfFormField] of type [PdfFormFieldType::RadioButton]. The form field object defines
/// an interactive radio button widget that can be toggled by the user.
///
/// Form fields in Pdfium are wrapped inside page annotations of type [PdfPageAnnotationType::Widget]
/// or [PdfPageAnnotationType::XfaWidget]. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// [PdfForm::field_values()] function.
pub struct PdfFormRadioButtonField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormRadioButtonField<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormRadioButtonField {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormRadioButtonField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the index of this [PdfFormRadioButtonField] in its control group.
    ///
    /// Control groups are used to group related interactive fields together. Checkboxes and
    /// radio buttons can be grouped such that only a single button can be selected within
    /// the control group. Each field within the group has a unique group index.
    #[inline]
    pub fn index_in_group(&self) -> u32 {
        self.index_in_group_impl()
    }

    /// Returns the value set for the control group containing this [PdfFormRadioButtonField].
    ///
    /// Control groups are used to group related interactive fields together. Checkboxes and
    /// radio buttons can be grouped such that only a single button can be selected within
    /// the control group. In this case, a single value can be shared by the group, indicating
    /// the value of the currently selected field within the group.
    #[inline]
    pub fn group_value(&self) -> Option<String> {
        self.value_impl()
    }

    /// Returns `true` if this [PdfFormRadioButtonField] object has its radio button selected.
    #[inline]
    pub fn is_checked(&self) -> Result<bool, PdfiumError> {
        // The PDF Reference manual, version 1.7, states that a selected radio button can indicate
        // its selected appearance by setting a custom appearance stream; this appearance stream
        // value will then become the group value. Pdfium's FPDFAnnot_IsChecked()
        // function doesn't check for this; so if FPDFAnnot_IsChecked() comes back with
        // anything other than Ok(true), we also check the appearance stream.

        match self.is_checked_impl() {
            Ok(true) => Ok(true),
            Ok(false) => Ok(self.group_value() == self.appearance_stream_impl()),
            Err(err) => match self.group_value() {
                None => Err(err),
                group_value => Ok(group_value == self.appearance_stream_impl()),
            },
        }
    }

    /// Selects the radio button of this [PdfFormRadioButtonField] object.
    #[inline]
    pub fn set_checked(&mut self) -> Result<(), PdfiumError> {
        println!("*** [radio] set_checked()");
        println!("??? ap: {:#?}", self.appearance_stream_impl());

        match self.appearance_stream_impl() {
            Some(appearance_stream) => self.set_value_impl(appearance_stream.as_str()),
            None => Err(PdfiumError::FormFieldAppearanceStreamUndefined),
        }
    }

    /// Returns `true` if exactly one radio button in the control group containing this
    /// [PdfFormRadioButtonField] must be selected at all times. If so, then toggling the
    /// currently selected radio button is not possible. If `false`, then toggling the
    /// currently selected radio button will deselect it, leaving no radio button in the
    /// group selected.
    #[inline]
    pub fn is_group_selection_required(&self) -> bool {
        !self
            .get_flags_impl()
            .contains(PdfFormFieldFlags::ButtonNoToggleToOff)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether or not the control group containing this [PdfFormRadioButtonField]
    /// requires exactly one radio button to be selected at all times.
    #[inline]
    pub fn set_is_group_selection_required(
        &mut self,
        is_group_selection_required: bool,
    ) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(
            PdfFormFieldFlags::ButtonNoToggleToOff,
            !is_group_selection_required,
        )
    }

    /// Returns `true` if all radio buttons in the same control group as this
    /// [PdfFormRadioButtonField] use the same value for the checked state; if so, if one
    /// is checked, then all will be checked, and so all radio buttons will turn on and
    /// off in unison.
    ///
    /// This flag was added in PDF version 1.5.
    #[inline]
    pub fn is_group_in_unison(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::ButtonIsRadiosInUnison)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether or not all radio buttons in the same control group as this
    /// [PdfFormRadioButtonField] use the same value for the checked state; if so, if one
    /// is checked, then all will be checked, and so all radio buttons will turn on and
    /// off in unison.
    ///
    /// This flag was added in PDF version 1.5.
    #[inline]
    pub fn set_is_group_in_unison(&mut self, is_group_in_unison: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(
            PdfFormFieldFlags::ButtonIsRadiosInUnison,
            is_group_in_unison,
        )
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormRadioButtonField<'a> {
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
