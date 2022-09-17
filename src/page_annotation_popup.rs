//! Defines the [PdfPagePopupAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Popup`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPagePopupAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPagePopupAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPagePopupAnnotation { handle, bindings }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPagePopupAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
