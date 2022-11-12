//! Defines the [PdfPageSquigglyAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Squiggly`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageSquigglyAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
    objects: PdfPageAnnotationObjects<'a>,
}

impl<'a> PdfPageSquigglyAnnotation<'a> {
    pub(crate) fn from_pdfium(
        annotation_handle: FPDF_ANNOTATION,
        page_handle: FPDF_PAGE,
        document: &'a PdfDocument<'a>,
    ) -> Self {
        PdfPageSquigglyAnnotation {
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
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageSquigglyAnnotation<'a> {
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
