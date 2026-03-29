//! Defines the [PdfActionUnsupported] struct, exposing functionality related to a single
//! action of type `PdfActionType::Unsupported`.

use crate::bindgen::FPDF_ACTION;
use crate::pdf::action::private::internal::PdfActionPrivate;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

pub struct PdfActionUnsupported<'a> {
    #[allow(dead_code)] // This field is not currently used, but we expect it to be in future
    handle: FPDF_ACTION,
    lifetime: PhantomData<&'a FPDF_ACTION>,
}

impl<'a> PdfActionUnsupported<'a> {
    #[inline]
    pub(crate) fn from_pdfium(handle: FPDF_ACTION) -> Self {
        PdfActionUnsupported {
            handle,
            lifetime: PhantomData,
        }
    }
}

impl<'a> PdfActionPrivate<'a> for PdfActionUnsupported<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ACTION {
        &self.handle
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfActionUnsupported<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfActionUnsupported<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfActionUnsupported<'a> {}
