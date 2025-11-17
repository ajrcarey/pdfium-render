//! Defines the [PdfFormComboBoxField] struct, exposing functionality related to a single
//! form field of type [PdfFormFieldType::ComboBox].

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::document::page::field::options::PdfFormFieldOptions;
use crate::pdf::document::page::field::private::internal::{
    PdfFormFieldFlags, PdfFormFieldPrivate,
};

#[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
use crate::error::PdfiumError;

#[cfg(doc)]
use {
    crate::pdf::document::form::PdfForm,
    crate::pdf::document::page::annotation::PdfPageAnnotationType,
    crate::pdf::document::page::field::{PdfFormField, PdfFormFieldType},
};

/// A single [PdfFormField] of type [PdfFormFieldType::ComboBox]. The form field object defines
/// an interactive drop-down list widget that allows the user to either select a value
/// from a list of options or type a value into a text field.
///
/// Form fields in Pdfium are wrapped inside page annotations of type [PdfPageAnnotationType::Widget]
/// or [PdfPageAnnotationType::XfaWidget]. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// [PdfForm::field_values()] function.
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
    pub fn options(&self) -> &PdfFormFieldOptions<'_> {
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

    /// Returns `true` if this [PdfFormComboBoxField] also includes an editable text box.
    /// If `false`, this combo box field only includes a drop-down list.
    #[inline]
    pub fn has_editable_text_box(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::ChoiceEdit)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether or not this [PdfFormComboBoxField] includes an editable text box
    /// in addition to a drop-down list.
    #[inline]
    pub fn set_has_editable_text_box(
        &mut self,
        has_editable_text_box: bool,
    ) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::ChoiceEdit, has_editable_text_box)
    }

    /// Returns `true` if the option items of this [PdfFormComboBoxField] should be sorted
    /// alphabetically.
    ///
    /// This flag is intended for use by form authoring tools, not by PDF viewer applications.
    #[inline]
    pub fn is_sorted(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::ChoiceSort)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether or not the option items of this [PdfFormComboBoxField] should be
    /// sorted alphabetically.
    ///
    /// This flag is intended for use by form authoring tools, not by PDF viewer applications.
    #[inline]
    pub fn set_is_sorted(&mut self, is_sorted: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::ChoiceSort, is_sorted)
    }

    /// Returns `true` if more than one of the option items in this [PdfFormComboBoxField]
    /// may be selected simultaneously. If `false`, only one item at a time may be selected.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn is_multiselect(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::ChoiceMultiSelect)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether more than one of the option items in this [PdfFormComboBoxField]
    /// may be selected simultaneously.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn set_is_multiselect(&mut self, is_multiselect: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::ChoiceMultiSelect, is_multiselect)
    }

    /// Returns `true` if text entered into the editable text box included in this
    /// [PdfFormComboBoxField] should be spell checked.
    ///
    /// This flag is meaningful only if the [PdfFormComboBoxField::has_editable_text_box()]
    /// flag is also `true`.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn is_spell_checked(&self) -> bool {
        !self
            .get_flags_impl()
            .contains(PdfFormFieldFlags::TextDoNotSpellCheck)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether or not text entered into the editable text box included in this
    /// [PdfFormComboBoxField] should be spell checked.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn set_is_spell_checked(&mut self, is_spell_checked: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextDoNotSpellCheck, !is_spell_checked)
    }

    /// Returns `true` if any new value is committed to this [PdfFormComboBoxField]
    /// as soon as a selection is made with the pointing device. This option enables
    /// applications to perform an action once a selection is made, without requiring
    /// the user to exit the field. If `false`, any new value is not committed until the
    /// user exits the field.
    ///
    /// This flag was added in PDF version 1.5.
    pub fn is_commit_on_selection_change(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::ChoiceCommitOnSelectionChange)
    }

    #[cfg(any(feature = "pdfium_future", feature = "pdfium_7350"))]
    /// Controls whether or not any new value is committed to this [PdfFormComboBoxField]
    /// as soon as a selection is made with the pointing device.
    ///
    /// This flag was added in PDF version 1.5.
    pub fn set_is_commit_on_selection_change(
        &mut self,
        is_commit_on_selection_change: bool,
    ) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(
            PdfFormFieldFlags::ChoiceCommitOnSelectionChange,
            is_commit_on_selection_change,
        )
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormComboBoxField<'a> {
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
