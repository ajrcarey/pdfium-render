//! Defines the [PdfActionLocalDestination] struct, exposing functionality related to a single
//! action of type `PdfActionType::GoToDestinationInSameDocument`.

use crate::bindgen::{FPDF_ACTION, FPDF_DOCUMENT};
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::action::private::internal::PdfActionPrivate;
use crate::pdf::destination::PdfDestination;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

pub struct PdfActionLocalDestination<'a> {
    handle: FPDF_ACTION,
    document: FPDF_DOCUMENT,
    lifetime: PhantomData<&'a FPDF_ACTION>,
}

impl<'a> PdfActionLocalDestination<'a> {
    #[inline]
    pub(crate) fn from_pdfium(handle: FPDF_ACTION, document: FPDF_DOCUMENT) -> Self {
        PdfActionLocalDestination {
            handle,
            document,
            lifetime: PhantomData,
        }
    }

    /// Returns the target [PdfDestination] for this [PdfActionLocalDestination].
    pub fn destination(&self) -> Result<PdfDestination<'_>, PdfiumError> {
        let handle = unsafe {
            self.bindings()
                .FPDFAction_GetDest(self.document, self.handle)
        };

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfDestination::from_pdfium(self.document, handle))
        }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionLocalDestination<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfActionLocalDestination<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfActionLocalDestination<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfActionLocalDestination<'a> {}
