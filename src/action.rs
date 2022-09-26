//! Defines the [PdfAction] struct, exposing functionality related to a single action
//! associated with a clickable link or document bookmark.

use crate::bindgen::{
    FPDF_ACTION, FPDF_DOCUMENT, PDFACTION_EMBEDDEDGOTO, PDFACTION_GOTO, PDFACTION_LAUNCH,
    PDFACTION_REMOTEGOTO, PDFACTION_UNSUPPORTED, PDFACTION_URI,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfActionType {
    GoToDestinationInSameDocument = PDFACTION_GOTO as isize,
    GoToDestinationInRemoteDocument = PDFACTION_REMOTEGOTO as isize,
    GoToDestinationInEmbeddedDocument = PDFACTION_EMBEDDEDGOTO as isize,
    Launch = PDFACTION_LAUNCH as isize,
    URI = PDFACTION_URI as isize,
    Unsupported = PDFACTION_UNSUPPORTED as isize,
}

impl PdfActionType {
    pub(crate) fn from_pdfium(action_type: u32) -> Result<PdfActionType, PdfiumError> {
        match action_type {
            PDFACTION_GOTO => Ok(PdfActionType::GoToDestinationInSameDocument),
            PDFACTION_REMOTEGOTO => Ok(PdfActionType::GoToDestinationInRemoteDocument),
            PDFACTION_EMBEDDEDGOTO => Ok(PdfActionType::GoToDestinationInEmbeddedDocument),
            PDFACTION_LAUNCH => Ok(PdfActionType::Launch),
            PDFACTION_URI => Ok(PdfActionType::URI),
            PDFACTION_UNSUPPORTED => Ok(PdfActionType::Unsupported),
            _ => Err(PdfiumError::UnknownActionType),
        }
    }
}

/// The action associated with a clickable link or document bookmark.
pub struct PdfAction<'a> {
    handle: FPDF_ACTION,
    #[allow(dead_code)]
    // The document field is not currently used, but we expect it to be in future
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfAction<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ACTION,
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            handle,
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfActionType] for this [PdfAction].
    ///
    /// Note that Pdfium does not support or recognize all PDF action types. For instance,
    /// Pdfium does not currently support or recognize the interactive Javascript action type
    /// supported by Adobe Acrobat or Foxit's commercial PDF SDK. In these cases,
    /// Pdfium will return `PdfActionType::Unsupported`.
    #[inline]
    pub fn action_type(&self) -> PdfActionType {
        PdfActionType::from_pdfium(self.bindings.FPDFAction_GetType(self.handle) as u32)
            .unwrap_or(PdfActionType::Unsupported)
    }
}
