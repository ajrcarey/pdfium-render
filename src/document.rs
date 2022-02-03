//! Defines the [PdfDocument] struct, the entry point to all Pdfium functionality
//! related to a single PDF file.

use crate::bindgen::FPDF_DOCUMENT;
use crate::bindings::PdfiumLibraryBindings;
use crate::form::PdfForm;
use crate::metadata::PdfMetadata;
use crate::pages::PdfPages;
use std::os::raw::c_int;

/// The file version of a [PdfDocument].
///
/// A list of PDF file versions is available at <https://en.wikipedia.org/wiki/History_of_PDF>.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PdfDocumentVersion {
    /// No version information is available. This is the case if the [PdfDocument]
    /// was created via a call to [PdfDocument::new()] rather than loaded from a file.
    Unset,

    /// PDF 1.0, first published in 1993, supported by Acrobat Reader Carousel (1.0) onwards.
    Pdf1_0,

    /// PDF 1.1, first published in 1994, supported by Acrobat Reader 2.0 onwards.
    Pdf1_1,

    /// PDF 1.2, first published in 1996, supported by Acrobat Reader 3.0 onwards.
    Pdf1_2,

    /// PDF 1.3, first published in 2000, supported by Acrobat Reader 4.0 onwards.
    Pdf1_3,

    /// PDF 1.4, first published in 2001, supported by Acrobat Reader 5.0 onwards.
    Pdf1_4,

    /// PDF 1.5, first published in 2003, supported by Acrobat Reader 6.0 onwards.
    Pdf1_5,

    /// PDF 1.6, first published in 2004, supported by Acrobat Reader 7.0 onwards.
    Pdf1_6,

    /// PDF 1.7, first published in 2006, supported by Acrobat Reader 8.0 onwards,
    /// adopted as ISO open standard 32000-1 in 2008. Certain proprietary Adobe
    /// extensions to PDF 1.7 are only fully supported in Acrobat Reader X (10.0)
    /// and later.
    Pdf1_7,

    /// PDF 2.0, first published in 2017, ISO open standard 32000-2.
    Pdf2_0,

    /// A two-digit raw file version number. For instance, a value of 21 would indicate
    /// PDF version 2.1, a value of 34 would indicate PDF version 3.4, and so on.
    /// Only used when the file version number is not directly recognized by
    /// pdfium-render.
    Other(i32),
}

/// An entry point to all the various object collections contained in a single PDF file.
/// These collections include:
/// * [PdfDocument::pages()], all the [PdfPages] in the document
/// * [PdfDocument::metadata()], all the [PdfMetadata] tags in the document
/// * [PdfDocument::form()], the [PdfForm] embedded in the document, if any
pub struct PdfDocument<'a> {
    handle: FPDF_DOCUMENT,
    form: Option<PdfForm<'a>>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfDocument<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            handle,
            form: PdfForm::from_pdfium(handle, bindings),
            bindings,
        }
    }

    /// Returns the internal FPDF_DOCUMENT handle for this [PdfDocument].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_DOCUMENT {
        &self.handle
    }

    /// Returns the file version of this [PdfDocument].
    pub fn version(&self) -> PdfDocumentVersion {
        let mut version: c_int = 0;

        if self.bindings.FPDF_GetFileVersion(self.handle, &mut version) != 0 {
            match version {
                10 => PdfDocumentVersion::Pdf1_0,
                11 => PdfDocumentVersion::Pdf1_1,
                12 => PdfDocumentVersion::Pdf1_2,
                13 => PdfDocumentVersion::Pdf1_3,
                14 => PdfDocumentVersion::Pdf1_4,
                15 => PdfDocumentVersion::Pdf1_5,
                16 => PdfDocumentVersion::Pdf1_6,
                17 => PdfDocumentVersion::Pdf1_7,
                20 => PdfDocumentVersion::Pdf2_0,
                _ => PdfDocumentVersion::Other(version),
            }
        } else {
            PdfDocumentVersion::Unset
        }
    }

    /// Returns the collection of [PdfPages] in this [PdfDocument].
    #[inline]
    pub fn pages(&self) -> PdfPages {
        PdfPages::new(self, self.bindings)
    }

    /// Returns the collection of [PdfMetadata] tags in this [PdfDocument].
    #[inline]
    pub fn metadata(&self) -> PdfMetadata {
        PdfMetadata::new(self, self.bindings)
    }

    /// Returns a reference to the [PdfForm] embedded in this [PdfDocument], if any.
    #[inline]
    pub fn form(&self) -> Option<&PdfForm> {
        self.form.as_ref()
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
