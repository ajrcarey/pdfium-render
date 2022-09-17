//! Defines the [PdfPageStrikeoutAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Strikeout`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageStrikeoutAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageStrikeoutAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageStrikeoutAnnotation { handle, bindings }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageStrikeoutAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
