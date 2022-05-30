//! Defines the [PdfPageTextAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Text`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageTextAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageTextAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageTextAnnotation { handle, bindings }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageTextAnnotation<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
