//! Defines the [PdfLink] struct, exposing functionality related to a single link contained
//! within a `PdfPage`, a `PdfPageAnnotation`, or a `PdfBookmark`.

use crate::bindgen::{FPDF_DOCUMENT, FPDF_LINK};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::action::PdfAction;
use crate::pdf::destination::PdfDestination;

pub struct PdfLink<'a> {
    handle: FPDF_LINK,
    document: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

/// A single link contained within a `PdfPage`, a `PdfPageAnnotation`, or a `PdfBookmark`.
///
/// Each link may have a corresponding [PdfAction] that will be triggered when the user
/// interacts with the link, and a [PdfDestination] that indicates the target of any behaviour
/// triggered by the [PdfAction].
impl<'a> PdfLink<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_LINK,
        document: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfLink {
            handle,
            document,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfLink].
    #[inline]
    pub fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the [PdfAction] associated with this [PdfLink], if any.
    ///
    /// The action indicates the behaviour that will occur when the user interacts with the
    /// link in a PDF viewer. For most links, this will be a local navigation action
    /// of type `PdfActionType::GoToDestinationInSameDocument`, but the PDF file format supports
    /// a variety of other actions.
    pub fn action(&self) -> Option<PdfAction<'a>> {
        let handle = self.bindings.FPDFLink_GetAction(self.handle);

        if handle.is_null() {
            None
        } else {
            Some(PdfAction::from_pdfium(handle, self.document, self.bindings))
        }
    }

    /// Returns the [PdfDestination] associated with this [PdfLink], if any.
    ///
    /// The destination specifies the page and region, if any, that will be the target
    /// of any behaviour that will occur when the user interacts with the link in a PDF viewer.
    pub fn destination(&self) -> Option<PdfDestination<'a>> {
        let handle = self.bindings.FPDFLink_GetDest(self.document, self.handle);

        if handle.is_null() {
            None
        } else {
            Some(PdfDestination::from_pdfium(
                self.document,
                handle,
                self.bindings,
            ))
        }
    }
}
