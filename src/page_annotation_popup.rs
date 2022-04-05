//! Defines the [PdfPagePopupAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Popup`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation::internal::PdfPageAnnotationPrivate;
use crate::page_annotations::PdfPageAnnotationIndex;

pub struct PdfPagePopupAnnotation<'a> {
    index: PdfPageAnnotationIndex,
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPagePopupAnnotation<'a> {
    pub(crate) fn from_pdfium(
        index: PdfPageAnnotationIndex,
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPagePopupAnnotation {
            index,
            handle,
            bindings,
        }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPagePopupAnnotation<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn index_impl(&self) -> PdfPageAnnotationIndex {
        self.index
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
