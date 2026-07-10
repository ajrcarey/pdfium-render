//! Defines the [PdfCatalog] struct, exposing internal properties related to the
//! document catalog for a single [PdfDocument].

use crate::bindgen::FPDF_DOCUMENT;
use crate::error::PdfiumError;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use crate::utils::{mem::create_byte_buffer, utf16le::get_string_from_pdfium_utf16le_bytes};
use std::marker::PhantomData;

#[cfg(any(
    feature = "pdfium_future",
    feature = "pdfium_7881",
    feature = "pdfium_7763"
))]
use std::ffi::c_ushort;

#[cfg(doc)]
use crate::pdf::document::PdfDocument;

/// The internal catalog properties for a single [PdfDocument].
pub struct PdfCatalog<'a> {
    document_handle: FPDF_DOCUMENT,
    lifetime: PhantomData<&'a FPDF_DOCUMENT>,
}

impl<'a> PdfCatalog<'a> {
    #[inline]
    pub(crate) fn from_pdfium(document_handle: FPDF_DOCUMENT) -> Self {
        Self {
            document_handle,
            lifetime: PhantomData,
        }
    }

    /// Returns the internal `FPDF_DOCUMENT` handle of the `PdfDocument` containing
    /// this [PdfCatalog] instance.
    #[inline]
    pub(crate) fn document_handle(&self) -> FPDF_DOCUMENT {
        self.document_handle
    }

    /// Returns `true` if the containing [PdfDocument] is a tagged PDF.
    ///
    /// A PDF is considered "tagged" if it includes structural elements and metadata
    /// that can be used to facilitate content extraction and processing by tooling;
    /// in other words, the PDF contains data above and beyond that required merely for
    /// rendering.
    ///
    /// For more information on tagged PDFs, see The PDF Reference, Sixth Edition,
    /// section 10.7, starting on page 883.
    #[inline]
    pub fn is_tagged(&self) -> bool {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        self.bindings()
            .is_true(unsafe { self.bindings().FPDFCatalog_IsTagged(self.document_handle()) })
    }

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7881",
        feature = "pdfium_7763"
    ))]
    /// Returns the language set in the catalog of the containing [PdfDocument], if any.
    pub fn get_language(&self) -> Result<String, PdfiumError> {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        // Retrieving the bookmark title from Pdfium is a two-step operation. First, we call
        // FPDFBookmark_GetTitle() with a null buffer; this will retrieve the length of
        // the bookmark title in bytes. If the length is zero, then there is no title.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFBookmark_GetTitle() again with a pointer to the buffer;
        // this will write the bookmark title to the buffer in UTF16-LE format.

        let buffer_length = unsafe {
            self.bindings()
                .FPDFCatalog_GetLanguage(self.document_handle(), std::ptr::null_mut(), 0)
        };

        if buffer_length == 0 {
            // An error occurred.

            return Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure);
        }

        if buffer_length == 2 {
            // No language is set.

            return Err(PdfiumError::NoLanguageSetInDocumentCatalog);
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = unsafe {
            self.bindings().FPDFCatalog_GetLanguage(
                self.document_handle(),
                buffer.as_mut_ptr() as *mut c_ushort,
                buffer_length,
            )
        };

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer)
            .ok_or(PdfiumError::NoLanguageSetInDocumentCatalog)
    }

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7881",
        feature = "pdfium_7763",
        feature = "pdfium_7543",
        feature = "pdfium_7350",
        feature = "pdfium_7215",
        feature = "pdfium_7123",
        feature = "pdfium_6996",
        feature = "pdfium_6721",
        feature = "pdfium_6666"
    ))]
    /// Sets the language of the containing [PdfDocument] to the given value.
    pub fn set_language(&mut self, language: impl ToString) -> Result<(), PdfiumError> {
        #[cfg(feature = "thread_safe")]
        let _ffi = crate::pdfium::FfiLock::acquire();

        if self.bindings().is_true(unsafe {
            self.bindings()
                .FPDFCatalog_SetLanguage_str(self.document_handle(), language.to_string().as_str())
        }) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfCatalog<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfCatalog<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfCatalog<'a> {}
