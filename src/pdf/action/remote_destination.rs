//! Defines the [PdfActionRemoteDestination] struct, exposing functionality related to a single
//! action of type `PdfActionType::GoToDestinationInRemoteDocument`.

use crate::bindgen::FPDF_ACTION;
use crate::pdf::action::private::internal::PdfActionPrivate;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

pub struct PdfActionRemoteDestination<'a> {
    #[allow(dead_code)] // This field is not currently used, but we expect it to be in future
    handle: FPDF_ACTION,
    lifetime: PhantomData<&'a FPDF_ACTION>,
}

impl<'a> PdfActionRemoteDestination<'a> {
    #[inline]
    pub(crate) fn from_pdfium(handle: FPDF_ACTION) -> Self {
        PdfActionRemoteDestination {
            handle,
            lifetime: PhantomData,
        }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionRemoteDestination<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfActionRemoteDestination<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfActionRemoteDestination<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfActionRemoteDestination<'a> {}
