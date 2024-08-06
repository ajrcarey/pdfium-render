//! Defines the [PdfFormField] enum, exposing functionality related to a single interactive
//! form field in a [PdfForm].

pub mod button;
pub mod checkbox;
pub mod combo;
pub mod list;
pub mod option;
pub mod options;
pub(crate) mod private; // Keep private so that the PdfFormFieldPrivate trait is not exposed.
pub mod radio;
pub mod signature;
pub mod text;
pub mod unknown;

use crate::bindgen::{
    FPDF_ANNOTATION, FPDF_FORMFIELD_CHECKBOX, FPDF_FORMFIELD_COMBOBOX, FPDF_FORMFIELD_LISTBOX,
    FPDF_FORMFIELD_PUSHBUTTON, FPDF_FORMFIELD_RADIOBUTTON, FPDF_FORMFIELD_SIGNATURE,
    FPDF_FORMFIELD_TEXTFIELD, FPDF_FORMFIELD_UNKNOWN, FPDF_FORMHANDLE,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::appearance_mode::PdfAppearanceMode;
use crate::pdf::document::page::field::button::PdfFormPushButtonField;
use crate::pdf::document::page::field::checkbox::PdfFormCheckboxField;
use crate::pdf::document::page::field::combo::PdfFormComboBoxField;
use crate::pdf::document::page::field::list::PdfFormListBoxField;
use crate::pdf::document::page::field::private::internal::PdfFormFieldPrivate;
use crate::pdf::document::page::field::radio::PdfFormRadioButtonField;
use crate::pdf::document::page::field::signature::PdfFormSignatureField;
use crate::pdf::document::page::field::text::PdfFormTextField;
use crate::pdf::document::page::field::unknown::PdfFormUnknownField;
use std::os::raw::c_int;

#[cfg(doc)]
use crate::pdf::document::form::PdfForm;

/// The widget display type of a single interactive form field in a [PdfForm].
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfFormFieldType {
    // The FPDF_FORMFIELD_COUNT constant simply specifies the number of form field
    // widget types supported by Pdfium; we do not need to expose it.
    Unknown = FPDF_FORMFIELD_UNKNOWN as isize,
    PushButton = FPDF_FORMFIELD_PUSHBUTTON as isize,
    Checkbox = FPDF_FORMFIELD_CHECKBOX as isize,
    RadioButton = FPDF_FORMFIELD_RADIOBUTTON as isize,
    ComboBox = FPDF_FORMFIELD_COMBOBOX as isize,
    ListBox = FPDF_FORMFIELD_LISTBOX as isize,
    Text = FPDF_FORMFIELD_TEXTFIELD as isize,
    Signature = FPDF_FORMFIELD_SIGNATURE as isize,
}

impl PdfFormFieldType {
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn from_pdfium(value: c_int) -> Result<PdfFormFieldType, PdfiumError> {
        match value as u32 {
            FPDF_FORMFIELD_UNKNOWN => Ok(PdfFormFieldType::Unknown),
            FPDF_FORMFIELD_PUSHBUTTON => Ok(PdfFormFieldType::PushButton),
            FPDF_FORMFIELD_CHECKBOX => Ok(PdfFormFieldType::Checkbox),
            FPDF_FORMFIELD_RADIOBUTTON => Ok(PdfFormFieldType::RadioButton),
            FPDF_FORMFIELD_COMBOBOX => Ok(PdfFormFieldType::ComboBox),
            FPDF_FORMFIELD_LISTBOX => Ok(PdfFormFieldType::ListBox),
            FPDF_FORMFIELD_TEXTFIELD => Ok(PdfFormFieldType::Text),
            FPDF_FORMFIELD_SIGNATURE => Ok(PdfFormFieldType::Signature),
            _ => Err(PdfiumError::UnknownFormFieldType),
        }
    }

    #[inline]
    #[allow(dead_code)]
    // The as_pdfium() function is not currently used, but we expect it to be in future
    pub(crate) fn as_pdfium(&self) -> u32 {
        match self {
            PdfFormFieldType::Unknown => FPDF_FORMFIELD_UNKNOWN,
            PdfFormFieldType::PushButton => FPDF_FORMFIELD_PUSHBUTTON,
            PdfFormFieldType::Checkbox => FPDF_FORMFIELD_CHECKBOX,
            PdfFormFieldType::RadioButton => FPDF_FORMFIELD_RADIOBUTTON,
            PdfFormFieldType::ComboBox => FPDF_FORMFIELD_COMBOBOX,
            PdfFormFieldType::ListBox => FPDF_FORMFIELD_LISTBOX,
            PdfFormFieldType::Text => FPDF_FORMFIELD_TEXTFIELD,
            PdfFormFieldType::Signature => FPDF_FORMFIELD_SIGNATURE,
        }
    }
}

/// A single interactive form field in a [PdfForm].
pub enum PdfFormField<'a> {
    PushButton(PdfFormPushButtonField<'a>),
    Checkbox(PdfFormCheckboxField<'a>),
    RadioButton(PdfFormRadioButtonField<'a>),
    ComboBox(PdfFormComboBoxField<'a>),
    ListBox(PdfFormListBoxField<'a>),
    Signature(PdfFormSignatureField<'a>),
    Text(PdfFormTextField<'a>),
    Unknown(PdfFormUnknownField<'a>),
}

impl<'a> PdfFormField<'a> {
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Option<Self> {
        let result = bindings.FPDFAnnot_GetFormFieldType(form_handle, annotation_handle);

        if result == -1 {
            return None;
        }

        let form_field_type =
            PdfFormFieldType::from_pdfium(result).unwrap_or(PdfFormFieldType::Unknown);

        Some(match form_field_type {
            PdfFormFieldType::PushButton => PdfFormField::PushButton(
                PdfFormPushButtonField::from_pdfium(form_handle, annotation_handle, bindings),
            ),
            PdfFormFieldType::Checkbox => PdfFormField::Checkbox(
                PdfFormCheckboxField::from_pdfium(form_handle, annotation_handle, bindings),
            ),
            PdfFormFieldType::RadioButton => PdfFormField::RadioButton(
                PdfFormRadioButtonField::from_pdfium(form_handle, annotation_handle, bindings),
            ),
            PdfFormFieldType::ComboBox => PdfFormField::ComboBox(
                PdfFormComboBoxField::from_pdfium(form_handle, annotation_handle, bindings),
            ),
            PdfFormFieldType::ListBox => PdfFormField::ListBox(PdfFormListBoxField::from_pdfium(
                form_handle,
                annotation_handle,
                bindings,
            )),
            PdfFormFieldType::Text => PdfFormField::Text(PdfFormTextField::from_pdfium(
                form_handle,
                annotation_handle,
                bindings,
            )),
            PdfFormFieldType::Signature => PdfFormField::Signature(
                PdfFormSignatureField::from_pdfium(form_handle, annotation_handle, bindings),
            ),
            _ => PdfFormField::Unknown(PdfFormUnknownField::from_pdfium(
                form_handle,
                annotation_handle,
                bindings,
            )),
        })
    }

    #[inline]
    pub(crate) fn unwrap_as_trait(&self) -> &dyn PdfFormFieldPrivate<'a> {
        match self {
            PdfFormField::PushButton(field) => field,
            PdfFormField::Checkbox(field) => field,
            PdfFormField::RadioButton(field) => field,
            PdfFormField::ComboBox(field) => field,
            PdfFormField::ListBox(field) => field,
            PdfFormField::Signature(field) => field,
            PdfFormField::Text(field) => field,
            PdfFormField::Unknown(field) => field,
        }
    }

    /// The type of this [PdfFormField].
    #[inline]
    pub fn field_type(&self) -> PdfFormFieldType {
        match self {
            PdfFormField::PushButton(_) => PdfFormFieldType::PushButton,
            PdfFormField::Checkbox(_) => PdfFormFieldType::Checkbox,
            PdfFormField::RadioButton(_) => PdfFormFieldType::RadioButton,
            PdfFormField::ComboBox(_) => PdfFormFieldType::ComboBox,
            PdfFormField::ListBox(_) => PdfFormFieldType::ListBox,
            PdfFormField::Signature(_) => PdfFormFieldType::Signature,
            PdfFormField::Text(_) => PdfFormFieldType::Text,
            PdfFormField::Unknown(_) => PdfFormFieldType::Unknown,
        }
    }

    /// Returns a reference to the underlying [PdfFormPushButtonField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::PushButton].
    #[inline]
    pub fn as_push_button_field(&self) -> Option<&PdfFormPushButtonField> {
        match self {
            PdfFormField::PushButton(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormCheckboxField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::Checkbox].
    #[inline]
    pub fn as_checkbox_field(&self) -> Option<&PdfFormCheckboxField> {
        match self {
            PdfFormField::Checkbox(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a mutable reference to the underlying [PdfFormCheckboxField]
    /// for this [PdfFormField], if this form field has a field type of [PdfFormField::Checkbox].
    #[inline]
    pub fn as_checkbox_field_mut(&mut self) -> Option<&mut PdfFormCheckboxField<'a>> {
        match self {
            PdfFormField::Checkbox(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormRadioButtonField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::RadioButton].
    #[inline]
    pub fn as_radio_button_field(&self) -> Option<&PdfFormRadioButtonField> {
        match self {
            PdfFormField::RadioButton(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a mutable reference to the underlying [PdfFormRadioButtonField]
    /// for this [PdfFormField], if this form field has a field type of [PdfFormField::RadioButton].
    #[inline]
    pub fn as_radio_button_field_mut(&mut self) -> Option<&mut PdfFormRadioButtonField<'a>> {
        match self {
            PdfFormField::RadioButton(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormComboBoxField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::ComboBox].
    #[inline]
    pub fn as_combo_box_field(&self) -> Option<&PdfFormComboBoxField> {
        match self {
            PdfFormField::ComboBox(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormListBoxField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::ListBox].
    #[inline]
    pub fn as_list_box_field(&self) -> Option<&PdfFormListBoxField> {
        match self {
            PdfFormField::ListBox(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormSignatureField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::Signature].
    #[inline]
    pub fn as_signature_field(&self) -> Option<&PdfFormSignatureField> {
        match self {
            PdfFormField::Signature(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormTextField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::Text].
    #[inline]
    pub fn as_text_field(&self) -> Option<&PdfFormTextField> {
        match self {
            PdfFormField::Text(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a mutable reference to the underlying [PdfFormTextField] for this
    /// [PdfFormField], if this form field has a field type of [PdfFormField::Text].
    pub fn as_text_field_mut(&mut self) -> Option<&mut PdfFormTextField<'a>> {
        match self {
            PdfFormField::Text(field) => Some(field),
            _ => None,
        }
    }

    /// Returns a reference to the underlying [PdfFormUnknownField] for this [PdfFormField],
    /// if this form field has a field type of [PdfFormField::Unknown].
    #[inline]
    pub fn as_unknown_field(&self) -> Option<&PdfFormUnknownField> {
        match self {
            PdfFormField::Unknown(field) => Some(field),
            _ => None,
        }
    }
}

/// Functionality common to all [PdfFormField] objects, regardless of their [PdfFormFieldType].
pub trait PdfFormFieldCommon {
    /// Returns the name of this [PdfFormField], if any.
    fn name(&self) -> Option<String>;

    /// Returns the name of the currently set appearance stream for this [PdfFormField], if any.
    fn appearance_stream(&self) -> Option<String>;

    /// Returns the value currently set for the given appearance mode for this [PdfFormField],
    /// if any.
    fn appearance_mode_value(&self, appearance_mode: PdfAppearanceMode) -> Option<String>;
}

// Blanket implementation for all PdfFormFieldCommon types.

impl<'a, T> PdfFormFieldCommon for T
where
    T: PdfFormFieldPrivate<'a>,
{
    #[inline]
    fn name(&self) -> Option<String> {
        self.name_impl()
    }

    #[inline]
    fn appearance_stream(&self) -> Option<String> {
        self.appearance_stream_impl()
    }

    #[inline]
    fn appearance_mode_value(&self, appearance_mode: PdfAppearanceMode) -> Option<String> {
        self.appearance_mode_value_impl(appearance_mode)
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormField<'a> {
    #[inline]
    fn form_handle(&self) -> &FPDF_FORMHANDLE {
        self.unwrap_as_trait().form_handle()
    }

    #[inline]
    fn annotation_handle(&self) -> &FPDF_ANNOTATION {
        self.unwrap_as_trait().annotation_handle()
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.unwrap_as_trait().bindings()
    }
}

impl<'a> From<PdfFormPushButtonField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormPushButtonField<'a>) -> Self {
        Self::PushButton(field)
    }
}

impl<'a> From<PdfFormCheckboxField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormCheckboxField<'a>) -> Self {
        Self::Checkbox(field)
    }
}

impl<'a> From<PdfFormRadioButtonField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormRadioButtonField<'a>) -> Self {
        Self::RadioButton(field)
    }
}

impl<'a> From<PdfFormComboBoxField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormComboBoxField<'a>) -> Self {
        Self::ComboBox(field)
    }
}

impl<'a> From<PdfFormListBoxField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormListBoxField<'a>) -> Self {
        Self::ListBox(field)
    }
}

impl<'a> From<PdfFormTextField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormTextField<'a>) -> Self {
        Self::Text(field)
    }
}

impl<'a> From<PdfFormSignatureField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormSignatureField<'a>) -> Self {
        Self::Signature(field)
    }
}

impl<'a> From<PdfFormUnknownField<'a>> for PdfFormField<'a> {
    #[inline]
    fn from(field: PdfFormUnknownField<'a>) -> Self {
        Self::Unknown(field)
    }
}
