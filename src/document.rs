use crate::bindgen::{_FPDF_FORMFILLINFO, FPDF_DOCUMENT, FPDF_FORMFILLINFO, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::page::PdfPage;
use crate::{PdfPageIndex, PdfiumError, PdfiumInternalError};
use std::ops::Range;

/// A collection of PdfPages contained in a single file.
pub struct PdfDocument<'a> {
    handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfDocument<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self { handle, bindings}
    }

    /// Returns the number of pages in this PdfDocument.
    pub fn page_count(&self) -> PdfPageIndex {
        self.bindings.FPDF_GetPageCount(self.handle) as PdfPageIndex
    }

    pub fn load_form_data(&self) -> FPDF_FORMHANDLE {
        let mut x = _FPDF_FORMFILLINFO{
            version: 1,
            Release: None,
            FFI_Invalidate: None,
            FFI_OutputSelectedRect: None,
            FFI_SetCursor: None,
            FFI_SetTimer: None,
            FFI_KillTimer: None,
            FFI_GetLocalTime: None,
            FFI_OnChange: None,
            FFI_GetPage: None,
            FFI_GetCurrentPage: None,
            FFI_GetRotation: None,
            FFI_ExecuteNamedAction: None,
            FFI_SetTextFieldFocus: None,
            FFI_DoURIAction: None,
            FFI_DoGoToAction: None,
            m_pJsPlatform: std::ptr::null_mut(),
            xfa_disabled: 0,
            FFI_DisplayCaret: None,
            FFI_GetCurrentPageIndex: None,
            FFI_SetCurrentPage: None,
            FFI_GotoURL: None,
            FFI_GetPageViewRect: None,
            FFI_PageEvent: None,
            FFI_PopupMenu: None,
            FFI_OpenFile: None,
            FFI_EmailTo: None,
            FFI_UploadTo: None,
            FFI_GetPlatform: None,
            FFI_GetLanguage: None,
            FFI_DownloadFromURL: None,
            FFI_PostRequestURL: None,
            FFI_PutRequestURL: None,
            FFI_OnFocusChange: None,
            FFI_DoURIActionWithKeyboardModifier: None
        };

        unsafe {
            let handle = self.bindings.FPDFDOC_InitFormFillEnvironment(self.handle, &mut x);

          //   self.bindings.FPDF_SetFormFieldHighlightColor(handle, 0,  0xFF0000);
          //   self.bindings.FPDF_SetFormFieldHighlightAlpha(handle, 100);
            handle
        }



        //handle
    }

    /// Returns a Range from 0..(number of pages) for this PdfDocument.
    #[inline]
    pub fn page_range(&self) -> Range<PdfPageIndex> {
        0..self.page_count()
    }

    /// Returns a single page from this PdfDocument.
    pub fn get_page(&self, index: PdfPageIndex) -> Result<PdfPage, PdfiumError> {
        if index >= self.page_count() {
            return Err(PdfiumError::PageIndexOutOfBounds);
        }

        let handle = self.bindings.FPDF_LoadPage(self.handle, index as i32);

        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfPage::from_pdfium(index, handle, self.bindings))
        }
    }

    /// Returns an iterator over all the pages in this PdfDocument.
    #[inline]
    pub fn pages(&self) -> PdfDocumentPdfPageIterator {
        PdfDocumentPdfPageIterator::new(self)
    }
}

impl<'a> Drop for PdfDocument<'a> {
    /// Closes this PdfDocument, releasing held memory and, if the document was loaded
    /// from a file, the file handle on the document.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_CloseDocument(self.handle);
    }
}

pub struct PdfDocumentPdfPageIterator<'a> {
    document: &'a PdfDocument<'a>,
    page_count: PdfPageIndex,
    next_index: PdfPageIndex,
}

impl<'a> PdfDocumentPdfPageIterator<'a> {
    #[inline]
    fn new(document: &'a PdfDocument<'a>) -> Self {
        PdfDocumentPdfPageIterator {
            document,
            page_count: document.page_count(),
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfDocumentPdfPageIterator<'a> {
    type Item = PdfPage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.page_count {
            return None;
        }

        let next = self.document.get_page(self.next_index);

        self.next_index += 1;

        match next {
            Ok(next) => Some(next),
            Err(_) => None,
        }
    }
}
