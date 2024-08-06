//! Defines the [PdfPageUnsupportedObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Unsupported`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::PdfPageObject;

pub struct PdfPageUnsupportedObject<'a> {
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    annotation_handle: Option<FPDF_ANNOTATION>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageUnsupportedObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: Option<FPDF_PAGE>,
        annotation_handle: Option<FPDF_ANNOTATION>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageUnsupportedObject {
            object_handle,
            page_handle,
            annotation_handle,
            bindings,
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageUnsupportedObject<'a> {
    #[inline]
    fn get_object_handle(&self) -> FPDF_PAGEOBJECT {
        self.object_handle
    }

    #[inline]
    fn get_page_handle(&self) -> Option<FPDF_PAGE> {
        self.page_handle
    }

    #[inline]
    fn set_page_handle(&mut self, page: FPDF_PAGE) {
        self.page_handle = Some(page);
    }

    #[inline]
    fn clear_page_handle(&mut self) {
        self.page_handle = None;
    }

    #[inline]
    fn get_annotation_handle(&self) -> Option<FPDF_ANNOTATION> {
        self.annotation_handle
    }

    #[inline]
    fn set_annotation_handle(&mut self, annotation: FPDF_ANNOTATION) {
        self.annotation_handle = Some(annotation);
    }

    #[inline]
    fn clear_annotation_handle(&mut self) {
        self.annotation_handle = None;
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn is_copyable_impl(&self) -> bool {
        false
    }

    #[inline]
    fn try_copy_impl<'b>(
        &self,
        _: FPDF_DOCUMENT,
        _: &'b dyn PdfiumLibraryBindings,
    ) -> Result<PdfPageObject<'b>, PdfiumError> {
        Err(PdfiumError::UnsupportedPdfPageObjectType)
    }
}
