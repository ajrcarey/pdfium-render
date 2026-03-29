//! Defines the [PdfFormSignatureField] struct, exposing functionality related to a single
//! form field of type [PdfFormFieldType::Signature].

use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::pdf::document::page::field::private::internal::PdfFormFieldPrivate;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

#[cfg(doc)]
use {
    crate::pdf::document::form::PdfForm,
    crate::pdf::document::page::annotation::PdfPageAnnotationType,
    crate::pdf::document::page::field::{PdfFormField, PdfFormFieldType},
};

/// A single [PdfFormField] of type [PdfFormFieldType::Signature]. The form field object defines
/// an interactive data entry widget that allows the user to draw a signature.
///
/// Form fields in Pdfium are wrapped inside page annotations of type [PdfPageAnnotationType::Widget]
/// or [PdfPageAnnotationType::XfaWidget]. User-specified values can be retrieved directly from
/// each form field object by unwrapping the form field from the annotation, or in bulk from the
/// [PdfForm::field_values()] function.
pub struct PdfFormSignatureField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    lifetime: PhantomData<&'a FPDF_ANNOTATION>,
}

impl<'a> PdfFormSignatureField<'a> {
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
    ) -> Self {
        PdfFormSignatureField {
            form_handle,
            annotation_handle,
            lifetime: PhantomData,
        }
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormSignatureField<'a> {
    #[inline]
    fn form_handle(&self) -> FPDF_FORMHANDLE {
        self.form_handle
    }

    #[inline]
    fn annotation_handle(&self) -> FPDF_ANNOTATION {
        self.annotation_handle
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfFormSignatureField<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfFormSignatureField<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfFormSignatureField<'a> {}
