//! Defines the [PdfActionLaunch] struct, exposing functionality related to a single
//! action of type `PdfActionType::Launch`.

use crate::bindgen::FPDF_ACTION;
use crate::pdf::action::private::internal::PdfActionPrivate;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

pub struct PdfActionLaunch<'a> {
    #[allow(dead_code)] // This field is not currently used, but we expect it to be in future
    handle: FPDF_ACTION,
    lifetime: PhantomData<&'a FPDF_ACTION>,
}

impl<'a> PdfActionLaunch<'a> {
    #[inline]
    pub(crate) fn from_pdfium(handle: FPDF_ACTION) -> Self {
        PdfActionLaunch {
            handle,
            lifetime: PhantomData,
        }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionLaunch<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfActionLaunch<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfActionLaunch<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfActionLaunch<'a> {}
