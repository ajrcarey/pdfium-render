//! Defines the [PdfFormTextField] struct, exposing functionality related to a single
//! form field of type [PdfFormFieldType::Text].

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
        if self.is_rich_text() {
            self.get_string_value("RV")
        } else {
            self.value_impl()
        }
    }

    /// Sets the value of this [PdfFormTextField] object.
    #[inline]
    pub fn set_value(&mut self, value: &str) -> Result<(), PdfiumError> {
        if self.is_rich_text() {
            self.set_string_value("RV", value)
        } else {
            self.set_value_impl(value)
        }
    }

    /// Returns `true` if this [PdfFormTextField] is configured as a multi-line text field.
    #[inline]
    pub fn is_multiline(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::TextMultiline)
    }

    #[cfg(feature = "pdfium_future")]
    /// Controls whether or not this [PdfFormTextField] is configured as a multi-line text field.
    #[inline]
    pub fn set_is_multiline(&self, is_multiline: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextMultiline, is_multiline)
    }

    /// Returns `true` if this [PdfFormTextField] is configured as a password field.
    #[inline]
    pub fn is_password(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::TextPassword)
    }

    #[cfg(feature = "pdfium_future")]
    /// Controls whether or not this [PdfFormTextField] is configured as a password text field.
    #[inline]
    pub fn set_is_password(&self, is_password: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextPassword, is_password)
    }

    /// Returns `true` if this [PdfFormTextField] represents the path of a file
    /// whose contents are to be submitted as the value of the field.
    ///
    /// This flag was added in PDF version 1.4
    pub fn is_file_select(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::TextFileSelect)
    }

    #[cfg(feature = "pdfium_future")]
    /// Controls whether or not this [PdfFormTextField] represents the path of a file
    /// whose contents are to be submitted as the value of the field.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn set_is_file_select(&mut self, is_file_select: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextFileSelect, is_file_select)
    }

    /// Returns `true` if text entered into this [PdfFormTextField] should be spell checked.
    pub fn is_spell_checked(&self) -> bool {
        !self
            .get_flags_impl()
            .contains(PdfFormFieldFlags::TextDoNotSpellCheck)
    }

    #[cfg(feature = "pdfium_future")]
    /// Controls whether or not text entered into this [PdfFormTextField] should be spell checked.
    pub fn set_is_spell_checked(&mut self, is_spell_checked: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextDoNotSpellCheck, !is_spell_checked)
    }

    /// Returns `true` if the internal area of this [PdfFormTextField] can scroll either
    /// horizontally or vertically to accommodate text entry longer than what can fit
    /// within the field's annotation bounds. If this value is `false`, then once the
    /// field is full, no further text entry will be accepted.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn is_scrollable(&self) -> bool {
        !self
            .get_flags_impl()
            .contains(PdfFormFieldFlags::TextDoNotScroll)
    }

    #[cfg(feature = "pdfium_future")]
    /// Controls whether or not the internal area of this [PdfFormTextField] can scroll
    /// either horizontally or vertically to accommodate text entry longer than what can fit
    /// within the field's annotation bounds. If set to `false`, no further text entry
    /// will be accepted once the field's annotation bounds are full.
    ///
    /// This flag was added in PDF version 1.4.
    pub fn set_is_scrollable(&mut self, is_scrollable: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextDoNotScroll, !is_scrollable)
    }

    /// Returns `true` if this [PdfFormTextField] is "combed", that is, automatically divided
    /// into equally-spaced positions ("combs"), with the text in the field laid out into
    /// those combs.
    ///
    /// For more information on this setting, refer to Table 8.77 of The PDF Reference
    /// (Sixth Edition, PDF Format 1.7), on page 691.
    ///
    /// This flag was added in PDF version 1.5.
    pub fn is_combed(&self) -> bool {
        // This flag only takes effect if the multi-line, password, and file select flags
        // are all unset.

        !self.is_multiline()
            && !self.is_password()
            && !self.is_file_select()
            && self.get_flags_impl().contains(PdfFormFieldFlags::TextComb)
    }

    // TODO: AJRC - 20/06/25 - there is little point providing the matching `set_is_combed()`
    // function, because it makes little sense without being also able to set the `MaxValue`
    // dictionary parameter that controls the number of combs. However, `MaxValue` must be
    // an integer, and Pdfium does not currently provide a `FPDFAnnot_SetNumberValue()`
    // function that could correctly set it.

    /// Returns `true` if the text in this [PdfFormTextField] is a rich text string.
    ///
    /// This flag was added in PDF version 1.5.
    pub fn is_rich_text(&self) -> bool {
        self.get_flags_impl()
            .contains(PdfFormFieldFlags::TextRichText)
    }

    #[cfg(feature = "pdfium_future")]
    /// Controls whether or not the text in this [PdfFormTextField] is a rich text string.
    ///
    /// This flag was added in PDF version 1.5.
    pub fn set_is_rich_text(&mut self, is_rich_text: bool) -> Result<(), PdfiumError> {
        self.update_one_flag_impl(PdfFormFieldFlags::TextRichText, is_rich_text)
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormTextField<'a> {
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
