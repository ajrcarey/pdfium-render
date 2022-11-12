//! Defines the [PdfPageUnsupportedAnnotation] struct, exposing functionality related to any
//! single annotation object of a type not supported by Pdfium.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::page_annotation::PdfPageAnnotationType;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageUnsupportedAnnotation<'a> {
    annotation_type: PdfPageAnnotationType,
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
    objects: PdfPageAnnotationObjects<'a>,
}

impl<'a> PdfPageUnsupportedAnnotation<'a> {
    pub(crate) fn from_pdfium(
        annotation_type: PdfPageAnnotationType,
        annotation_handle: FPDF_ANNOTATION,
        page_handle: FPDF_PAGE,
        document: &'a PdfDocument<'a>,
    ) -> Self {
        PdfPageUnsupportedAnnotation {
            annotation_type,
            handle: annotation_handle,
            bindings: document.bindings(),
            objects: PdfPageAnnotationObjects::from_pdfium(
                *document.handle(),
                page_handle,
                annotation_handle,
                document.bindings(),
            ),
        }
    }

    /// Returns the annotation type of this annotation recognized by Pdfium, but unsupported
    /// for creation, editing, or rendering.
    #[inline]
    pub fn get_type(&self) -> PdfPageAnnotationType {
        self.annotation_type
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageUnsupportedAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn objects_impl(&self) -> &PdfPageAnnotationObjects {
        &self.objects
    }

    #[inline]
    fn objects_mut_impl(&mut self) -> &mut PdfPageAnnotationObjects<'a> {
        &mut self.objects
    }
}
