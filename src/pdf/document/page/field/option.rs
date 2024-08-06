//! Defines the [PdfFormFieldOption] struct, exposing functionality related to a single selectable
//! option in a `PdfFormFieldOptions` collection.

use crate::pdf::document::page::field::options::PdfFormFieldOptionIndex;

/// A single selectable option in a list box or check box form field widget.
pub struct PdfFormFieldOption {
    index: PdfFormFieldOptionIndex,
    is_set: bool,
    label: Option<String>,
}

impl PdfFormFieldOption {
    #[inline]
    pub(crate) fn new(index: PdfFormFieldOptionIndex, is_set: bool, label: Option<String>) -> Self {
        PdfFormFieldOption {
            index,
            is_set,
            label,
        }
    }

    /// Returns the zero-based index of this option in its containing form field.
    #[inline]
    pub fn index(&self) -> PdfFormFieldOptionIndex {
        self.index
    }

    /// Returns `true` if this option is selected.
    #[inline]
    pub fn is_set(&self) -> bool {
        self.is_set
    }

    /// Returns the displayed label for this option, if any.
    #[inline]
    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
}
