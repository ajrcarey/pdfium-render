//! Defines the [PdfDestination] struct, exposing functionality related to the target destination
//! of a link contained within a single `PdfPage`.

use crate::bindgen::{FPDF_DEST, FPDF_DOCUMENT, FS_FLOAT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::pages::PdfPageIndex;
use crate::points::PdfPoints;

/// The page and region, if any, that will be the target of any behaviour that will occur
/// when the user interacts with a link in a PDF viewer.
pub struct PdfDestination<'a> {
    document_handle: FPDF_DOCUMENT,
    destination_handle: FPDF_DEST,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfDestination<'a> {
    // TODO: AJRC - 18/2/23 - as the PdfDestination struct is fleshed out, the example at
    // examples/links.rs should be expanded to demonstrate the new functionality.

    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        destination_handle: FPDF_DEST,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfDestination {
            document_handle,
            destination_handle,
            bindings,
        }
    }

    /// Returns the internal `FPDF_DEST` handle for this [PdfDestination].
    #[inline]
    #[allow(unused)]
    pub(crate) fn destination_handle(&self) -> FPDF_DEST {
        self.destination_handle
    }

    /// Returns the internal `FPDF_DOCUMENT` handle for this [PdfDestination].
    #[inline]
    #[allow(unused)]
    pub(crate) fn document_handle(&self) -> FPDF_DOCUMENT {
        self.document_handle
    }

    /// Returns the zero-based index of the `PdfPage` containing this [PdfDestination].
    pub fn page_index(&self) -> Result<PdfPageIndex, PdfiumError> {
        match self
            .bindings
            .FPDFDest_GetDestPageIndex(self.document_handle, self.destination_handle)
        {
            -1 => Err(PdfiumError::DestinationPageIndexNotAvailable),
            index => Ok(index as PdfPageIndex),
        }
    }

    /// Returns the [PdfPoints] coordinates of the target page location of this [PdfDestination],
    /// if any.
    pub fn page_location(&self) -> Result<(Option<PdfPoints>, Option<PdfPoints>), PdfiumError> {
        let mut has_x_value = self.bindings.FALSE();

        let mut has_y_value = self.bindings.FALSE();

        let mut _has_zoom_value = self.bindings.FALSE();

        let mut x_value: FS_FLOAT = 0.0;

        let mut y_value: FS_FLOAT = 0.0;

        let mut _zoom_value: FS_FLOAT = 0.0;

        if self
            .bindings
            .is_true(self.bindings.FPDFDest_GetLocationInPage(
                self.destination_handle,
                &mut has_x_value,
                &mut has_y_value,
                &mut _has_zoom_value,
                &mut x_value,
                &mut y_value,
                &mut _zoom_value,
            ))
        {
            let x = if self.bindings.is_true(has_x_value) {
                Some(PdfPoints::new(x_value))
            } else {
                None
            };

            let y = if self.bindings.is_true(has_y_value) {
                Some(PdfPoints::new(y_value))
            } else {
                None
            };

            Ok((x, y))
        } else {
            Err(PdfiumError::DestinationPageLocationNotAvailable)
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfDestination].
    #[inline]
    pub fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
