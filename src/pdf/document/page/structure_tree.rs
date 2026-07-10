//! Defines the [PdfStructureTree] struct, the root element of a tree of elements that
//! together describe the logical hierarchy of a single [PdfPage].

use crate::bindgen::{FPDF_PAGE, FPDF_STRUCTTREE};
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use std::marker::PhantomData;

#[cfg(doc)]
use crate::pdf::document::page::PdfPage;

/// The root element of a tree of elements that together describe the logical hierarchy
/// of a single [PdfPage].
///
/// More information on the structure tree can be found in The PDF Reference, Sixth Edition,
/// in Section 10.6.1, beginning on page 856.
pub struct PdfPageStructureTree<'a> {
    handle: FPDF_STRUCTTREE,
    lifetime: PhantomData<&'a FPDF_STRUCTTREE>,
}

impl<'a> PdfPageStructureTree<'a> {
    pub(crate) fn from_pdfium(page_handle: FPDF_PAGE) -> Self {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        let handle = unsafe {
            PdfPageStructureTree {
                handle: 0 as FPDF_STRUCTTREE,
                lifetime: PhantomData,
            }
            .bindings()
            .FPDF_StructTree_GetForPage(page_handle)
        };

        PdfPageStructureTree {
            handle,
            lifetime: PhantomData,
        }
    }

    /// Returns the internal `FPDF_STRUCTTREE` handle for this [PdfStructureTree].
    #[inline]
    pub(crate) fn handle(&self) -> FPDF_STRUCTTREE {
        self.handle
    }
}

impl<'a> Drop for PdfPageStructureTree<'a> {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        unsafe {
            self.bindings().FPDF_StructTree_Close(self.handle());
        }
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfPageStructureTree<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfPageStructureTree<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfPageStructureTree<'a> {}
