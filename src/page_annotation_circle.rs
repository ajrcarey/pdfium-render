//! Defines the [PdfPageCircleAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Circle`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageCircleAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageCircleAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageCircleAnnotation { handle, bindings }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageCircleAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
