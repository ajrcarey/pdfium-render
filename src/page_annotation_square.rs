//! Defines the [PdfPageSquareAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Square`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageSquareAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageSquareAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageSquareAnnotation { handle, bindings }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageSquareAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
