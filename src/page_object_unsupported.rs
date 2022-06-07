//! Defines the [PdfPageUnsupportedObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Unsupported`.

use crate::bindgen::{FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_objects::PdfPageObjects;

pub struct PdfPageUnsupportedObject<'a> {
    is_object_memory_owned_by_page: bool,
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageUnsupportedObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageUnsupportedObject {
            is_object_memory_owned_by_page: true,
            object_handle,
            page_handle: Some(page_handle),
            bindings,
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageUnsupportedObject<'a> {
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
    fn add_object_to_page(&mut self, page_objects: &PdfPageObjects) -> Result<(), PdfiumError> {
        let page_handle = *page_objects.get_page_handle();

        self.bindings
            .FPDFPage_InsertObject(page_handle, self.object_handle);

        if let Some(error) = self.bindings.get_pdfium_last_error() {
            Err(PdfiumError::PdfiumLibraryInternalError(error))
        } else {
            self.page_handle = Some(page_handle);
            self.is_object_memory_owned_by_page = true;

            Ok(())
        }
    }

    #[inline]
    fn remove_object_from_page(&mut self) -> Result<(), PdfiumError> {
        if let Some(page_handle) = self.page_handle {
            if self.bindings.is_true(
                self.bindings
                    .FPDFPage_RemoveObject(page_handle, self.object_handle),
            ) {
                self.page_handle = None;
                self.is_object_memory_owned_by_page = false;

                Ok(())
            } else {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    self.bindings
                        .get_pdfium_last_error()
                        .unwrap_or(PdfiumInternalError::Unknown),
                ))
            }
        } else {
            Err(PdfiumError::PageObjectNotAttachedToPage)
        }
    }
}
