//! Defines the [PdfFormFieldOptions] struct, a collection of all the selectable options
//! displayed in a combo box or list box form field.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE, FPDF_WCHAR};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::field::option::PdfFormFieldOption;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::ops::{Range, RangeInclusive};
use std::os::raw::c_int;

pub type PdfFormFieldOptionIndex = usize;

/// A collection of all selectable options in a list box or check box form field widget.
pub struct PdfFormFieldOptions<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormFieldOptions<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormFieldOptions {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormFieldOptions] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the number of options in this [PdfFormFieldOptions] collection.
    pub fn len(&self) -> PdfFormFieldOptionIndex {
        let result = self
            .bindings
            .FPDFAnnot_GetOptionCount(self.form_handle, self.annotation_handle);

        if result == -1 {
            0
        } else {
            result as PdfFormFieldOptionIndex
        }
    }

    /// Returns `true` if this [PdfFormFieldOptions] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of options)` for this [PdfFormFieldOptions] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfFormFieldOptionIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of options - 1)` for this
    /// [PdfFormFieldOptions] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfFormFieldOptionIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfFormFieldOption] from this [PdfFormFieldOptions] collection.
    pub fn get(&self, index: PdfFormFieldOptionIndex) -> Result<PdfFormFieldOption, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::FormFieldOptionIndexOutOfBounds);
        }

        // Retrieving the option label from Pdfium is a two-step operation. First, we call
        // FPDFAnnot_GetOptionLabel() with a null buffer; this will retrieve the length of
        // the option label text in bytes. If the length is zero, then the option has no label.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFAnnot_GetOptionLabel() again with a pointer to the buffer;
        // this will write the option label to the buffer in UTF16LE format.

        let buffer_length = self.bindings().FPDFAnnot_GetOptionLabel(
            self.form_handle,
            self.annotation_handle,
            index as c_int,
            std::ptr::null_mut(),
            0,
        );

        let option_label = if buffer_length == 0 {
            // The field value is not present.

            None
        } else {
            let mut buffer = create_byte_buffer(buffer_length as usize);

            let result = self.bindings().FPDFAnnot_GetOptionLabel(
                self.form_handle,
                self.annotation_handle,
                index as c_int,
                buffer.as_mut_ptr() as *mut FPDF_WCHAR,
                buffer_length,
            );

            debug_assert_eq!(result, buffer_length);

            get_string_from_pdfium_utf16le_bytes(buffer)
        };

        let option_is_set = self
            .bindings
            .is_true(self.bindings.FPDFAnnot_IsOptionSelected(
                self.form_handle,
                self.annotation_handle,
                index as c_int,
            ));

        Ok(PdfFormFieldOption::new(index, option_is_set, option_label))
    }

    /// Returns an iterator over all the options in this [PdfFormFieldOptions] collection.
    #[inline]
    pub fn iter(&self) -> PdfFormFieldOptionsIterator {
        PdfFormFieldOptionsIterator::new(self)
    }
}

/// An iterator over all the [PdfFormFieldOption] objects in a [PdfFormFieldOptions] collection.
pub struct PdfFormFieldOptionsIterator<'a> {
    options: &'a PdfFormFieldOptions<'a>,
    next_index: PdfFormFieldOptionIndex,
}

impl<'a> PdfFormFieldOptionsIterator<'a> {
    #[inline]
    pub(crate) fn new(options: &'a PdfFormFieldOptions<'a>) -> Self {
        PdfFormFieldOptionsIterator {
            options,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfFormFieldOptionsIterator<'a> {
    type Item = PdfFormFieldOption;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.options.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
