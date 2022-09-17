//! Defines the [PdfPageFreeTextAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::FreeText`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageFreeTextAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageFreeTextAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageFreeTextAnnotation { handle, bindings }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageFreeTextAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
