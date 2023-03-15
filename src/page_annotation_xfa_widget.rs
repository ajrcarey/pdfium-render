//! Defines the [PdfPageXfaWidgetAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::XfaWidget`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::form_field::PdfFormField;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageXfaWidgetAnnotation<'a> {
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
    objects: PdfPageAnnotationObjects<'a>,
    form_field: Option<PdfFormField<'a>>,
}

impl<'a> PdfPageXfaWidgetAnnotation<'a> {
    pub(crate) fn from_pdfium(
        annotation_handle: FPDF_ANNOTATION,
        page_handle: FPDF_PAGE,
        document: &'a PdfDocument<'a>,
    ) -> Self {
        PdfPageXfaWidgetAnnotation {
            annotation_handle,
            bindings: document.bindings(),
            objects: PdfPageAnnotationObjects::from_pdfium(
                *document.handle(),
                page_handle,
                annotation_handle,
                document.bindings(),
            ),
            form_field: document.form().and_then(|form| {
                PdfFormField::from_pdfium(*form.handle(), annotation_handle, document.bindings())
            }),
        }
    }

    /// Returns the [PdfFormField] wrapped by this [PdfPageXfaWidgetAnnotation], if any.
    #[inline]
    pub fn form_field(&self) -> Option<&PdfFormField> {
        self.form_field.as_ref()
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageXfaWidgetAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.annotation_handle
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
