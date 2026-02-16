//! Defines the [PdfPageUnsupportedAnnotation] struct, exposing functionality related to any
//! single annotation object of a type not supported by Pdfium.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::pdf::document::page::annotation::attachment_points::PdfPageAnnotationAttachmentPoints;
use crate::pdf::document::page::annotation::objects::PdfPageAnnotationObjects;
use crate::pdf::document::page::annotation::private::internal::PdfPageAnnotationPrivate;
use crate::pdf::document::page::annotation::PdfPageAnnotationType;
use crate::pdf::document::page::object::ownership::PdfPageObjectOwnership;
use crate::pdf::document::page::objects::private::internal::PdfPageObjectsPrivate;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

#[cfg(doc)]
use crate::pdf::document::page::annotation::PdfPageAnnotation;

/// A single [PdfPageAnnotation] of any annotation type not supported by Pdfium.
pub struct PdfPageUnsupportedAnnotation<'a> {
    annotation_type: PdfPageAnnotationType,
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    attachment_points: PdfPageAnnotationAttachmentPoints<'a>,
    lifetime: PhantomData<&'a FPDF_ANNOTATION>,
}

impl<'a> PdfPageUnsupportedAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        annotation_type: PdfPageAnnotationType,
    ) -> Self {
        PdfPageUnsupportedAnnotation {
            annotation_type,
            handle: annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
            ),
            attachment_points: PdfPageAnnotationAttachmentPoints::from_pdfium(annotation_handle),
            lifetime: PhantomData,
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
    fn handle(&self) -> FPDF_ANNOTATION {
        self.handle
    }

    #[inline]
    fn ownership(&self) -> &PdfPageObjectOwnership {
        self.objects_impl().ownership()
    }

    #[inline]
    fn objects_impl(&self) -> &PdfPageAnnotationObjects<'_> {
        &self.objects
    }

    #[inline]
    fn attachment_points_impl(&self) -> &PdfPageAnnotationAttachmentPoints<'_> {
        &self.attachment_points
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfPageUnsupportedAnnotation<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfPageUnsupportedAnnotation<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfPageUnsupportedAnnotation<'a> {}
