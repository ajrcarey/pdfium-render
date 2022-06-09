//! Defines the [PdfPageFormFragmentObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::FormFragment`.

use crate::bindgen::{FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::page_object_private::internal::PdfPageObjectPrivate;

pub struct PdfPageFormFragmentObject<'a> {
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageFormFragmentObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageFormFragmentObject {
            object_handle,
            page_handle: Some(page_handle),
            bindings,
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageFormFragmentObject<'a> {
    #[inline]
    fn get_object_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.object_handle
    }

    #[inline]
    fn get_page_handle(&self) -> &Option<FPDF_PAGE> {
        &self.page_handle
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
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
