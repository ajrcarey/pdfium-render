//! Defines the [PdfPageLinkAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Link`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::link::PdfLink;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;
use crate::page_objects_private::internal::PdfPageObjectsPrivate;

/// A single `PdfPageAnnotation` of type `PdfPageAnnotationType::Link`.
pub struct PdfPageLinkAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageLinkAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageLinkAnnotation {
            handle: annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
                bindings,
            ),
            bindings,
        }
    }

    /// Returns the [PdfLink] associated with this [PdfPageLinkAnnotation], if any.
    pub fn link(&self) -> Result<PdfLink, PdfiumError> {
        let handle = self.bindings.FPDFAnnot_GetLink(self.handle);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfLink::from_pdfium(
                handle,
                self.objects.document_handle(),
                self.bindings,
            ))
        }
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageLinkAnnotation<'a> {
    #[inline]
    fn handle(&self) -> FPDF_ANNOTATION {
        self.handle
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
