//! Defines the [PdfDestination] struct, exposing functionality related to the target destination
//! of a link contained within a single `PdfPage`.

use crate::bindgen::FPDF_DEST;
use crate::bindings::PdfiumLibraryBindings;

/// The page and region, if any, that will be the target of any behaviour that will occur
/// when the user interacts with a link in a PDF viewer.
pub struct PdfDestination<'a> {
    handle: FPDF_DEST,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfDestination<'a> {
    // TODO: AJRC - 18/2/23 - as the PdfDestination struct is fleshed out, the example at
    // examples/links.rs should be expanded to demonstrate the new functionality.

    pub(crate) fn from_pdfium(handle: FPDF_DEST, bindings: &'a dyn PdfiumLibraryBindings) -> Self {
        PdfDestination { handle, bindings }
    }

    /// Returns the internal `FPDF_DEST` handle for this [PdfDestination].
    #[inline]
    #[allow(dead_code)] // TODO: AJRC - 18/2/23 - we expect this function to be used in the future.
    pub(crate) fn handle(&self) -> &FPDF_DEST {
        &self.handle
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfDestination].
    #[inline]
    pub fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
