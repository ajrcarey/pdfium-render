//! Defines the [PdfActionRemoteDestination] struct, exposing functionality related to a single
//! action of type `PdfActionType::GoToDestinationInRemoteDocument`.

use crate::action_private::internal::PdfActionPrivate;
use crate::bindgen::FPDF_ACTION;
use crate::bindings::PdfiumLibraryBindings;

pub struct PdfActionRemoteDestination<'a> {
    handle: FPDF_ACTION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfActionRemoteDestination<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ACTION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfActionRemoteDestination { handle, bindings }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionRemoteDestination<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
