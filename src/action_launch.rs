//! Defines the [PdfActionLaunch] struct, exposing functionality related to a single
//! action of type `PdfActionType::Launch`.

use crate::action_private::internal::PdfActionPrivate;
use crate::bindgen::FPDF_ACTION;
use crate::bindings::PdfiumLibraryBindings;

pub struct PdfActionLaunch<'a> {
    handle: FPDF_ACTION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfActionLaunch<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ACTION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfActionLaunch { handle, bindings }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionLaunch<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
