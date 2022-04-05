//! Defines the [PdfPageInkAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Ink`.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation::internal::PdfPageAnnotationPrivate;
use crate::page_annotations::PdfPageAnnotationIndex;

pub struct PdfPageInkAnnotation<'a> {
    index: PdfPageAnnotationIndex,
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageInkAnnotation<'a> {
    pub(crate) fn from_pdfium(
        index: PdfPageAnnotationIndex,
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageInkAnnotation {
            index,
            handle,
            bindings,
        }
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageInkAnnotation<'a> {
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
