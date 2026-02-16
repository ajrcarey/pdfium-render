//! Defines the [PdfPageHighlightAnnotation] struct, exposing functionality related to a single
//! user annotation of type [PdfPageAnnotationType::Highlight].

use std::marker::PhantomData;

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::pdf::document::page::annotation::attachment_points::PdfPageAnnotationAttachmentPoints;
use crate::pdf::document::page::annotation::objects::PdfPageAnnotationObjects;
use crate::pdf::document::page::annotation::private::internal::PdfPageAnnotationPrivate;
use crate::pdf::document::page::object::ownership::PdfPageObjectOwnership;
use crate::pdf::document::page::objects::private::internal::PdfPageObjectsPrivate;
use crate::pdfium::PdfiumLibraryBindingsAccessor;

#[cfg(doc)]
use {
    crate::pdf::document::page::annotation::PdfPageAnnotation,
    crate::pdf::document::page::annotation::PdfPageAnnotationType,
};

/// A single [PdfPageAnnotation] of type [PdfPageAnnotationType::Highlight].
pub struct PdfPageHighlightAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    attachment_points: PdfPageAnnotationAttachmentPoints<'a>,
    lifetime: PhantomData<&'a FPDF_ANNOTATION>,
}

impl<'a> PdfPageHighlightAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
    ) -> Self {
        PdfPageHighlightAnnotation {
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

    /// Returns a mutable collection of all the attachment points in this [PdfPageHighlightAnnotation].
    #[inline]
    pub fn attachment_points_mut(&mut self) -> &mut PdfPageAnnotationAttachmentPoints<'a> {
        &mut self.attachment_points
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageHighlightAnnotation<'a> {
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

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfPageHighlightAnnotation<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfPageHighlightAnnotation<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfPageHighlightAnnotation<'a> {}
