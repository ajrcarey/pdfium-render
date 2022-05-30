//! Defines the [PdfPageFormFragmentObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::FormFragment`.

use crate::bindgen::{FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::page_object_private::internal::PdfPageObjectPrivate;

pub struct PdfPageFormFragmentObject<'a> {
    is_object_memory_owned_by_page: bool,
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
            is_object_memory_owned_by_page: true,
            object_handle,
            page_handle: Some(page_handle),
            bindings,
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageFormFragmentObject<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.object_handle
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn is_object_memory_owned_by_page(&self) -> bool {
        self.is_object_memory_owned_by_page
    }

    #[inline]
    fn set_object_memory_owned_by_page(&mut self, page: FPDF_PAGE) {
        self.page_handle = Some(page);
        self.is_object_memory_owned_by_page = true;
    }

    #[inline]
    fn set_object_memory_released_by_page(&mut self) {
        self.page_handle = None;
        self.is_object_memory_owned_by_page = false;
    }
}
