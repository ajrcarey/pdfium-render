//! Defines the [PdfPageWidgetAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Widget`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_FORMHANDLE, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::document::page::annotation::attachment_points::PdfPageAnnotationAttachmentPoints;
use crate::pdf::document::page::annotation::objects::PdfPageAnnotationObjects;
use crate::pdf::document::page::annotation::private::internal::PdfPageAnnotationPrivate;
use crate::pdf::document::page::field::PdfFormField;

/// A single `PdfPageAnnotation` of type `PdfPageAnnotationType::Widget`.
///
/// Widget annotation types can wrap form fields. To access the form field, use the
/// [PdfPageWidgetAnnotation::form_field()] function.
pub struct PdfPageWidgetAnnotation<'a> {
    annotation_handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    attachment_points: PdfPageAnnotationAttachmentPoints<'a>,
    form_field: Option<PdfFormField<'a>>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageWidgetAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        form_handle: Option<FPDF_FORMHANDLE>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageWidgetAnnotation {
            annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
                bindings,
            ),
            attachment_points: PdfPageAnnotationAttachmentPoints::from_pdfium(
                annotation_handle,
                bindings,
            ),
            form_field: form_handle.and_then(|form_handle| {
                PdfFormField::from_pdfium(form_handle, annotation_handle, bindings)
            }),
            bindings,
        }
    }

    /// Returns an immutable reference to the [PdfFormField] wrapped by this [PdfPageWidgetAnnotation],
    /// if any.
    #[inline]
    pub fn form_field(&self) -> Option<&PdfFormField> {
        self.form_field.as_ref()
    }

    /// Returns a mutable reference to the [PdfFormField] wrapped by this [PdfPageWidgetAnnotation],
    /// if any.
    #[inline]
    pub fn form_field_mut(&mut self) -> Option<&mut PdfFormField<'a>> {
        self.form_field.as_mut()
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageWidgetAnnotation<'a> {
    #[inline]
    fn handle(&self) -> FPDF_ANNOTATION {
        self.annotation_handle
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

    #[inline]
    fn attachment_points_impl(&self) -> &PdfPageAnnotationAttachmentPoints {
        &self.attachment_points
    }

    #[inline]
    fn attachment_points_mut_impl(&mut self) -> &mut PdfPageAnnotationAttachmentPoints<'a> {
        &mut self.attachment_points
    }
}
