//! Defines the [PdfPageUnsupportedAnnotation] struct, exposing functionality related to any
//! single annotation object of a type not supported by Pdfium.

use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation::PdfPageAnnotationType;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageUnsupportedAnnotation<'a> {
    annotation_type: PdfPageAnnotationType,
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageUnsupportedAnnotation<'a> {
    pub(crate) fn from_pdfium(
        annotation_type: PdfPageAnnotationType,
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageUnsupportedAnnotation {
            annotation_type,
            handle,
            bindings,
        }
    }

    /// Returns the annotation type of this annotation recognized by Pdfium, but unsupported
    /// for creation, editing, or rendering.
    #[inline]
    pub fn get_type(&self) -> PdfPageAnnotationType {
        self.annotation_type
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageUnsupportedAnnotation<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
