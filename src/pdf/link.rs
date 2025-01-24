//! Defines the [PdfLink] struct, exposing functionality related to a single link contained
//! within a [PdfPage], a [PdfPageAnnotation], or a [PdfBookmark].

use crate::bindgen::{FPDF_DOCUMENT, FPDF_LINK, FS_RECTF};
use crate::bindings::PdfiumLibraryBindings;
use crate::pdf::action::PdfAction;
use crate::pdf::destination::PdfDestination;
use crate::pdf::rect::PdfRect;

#[cfg(doc)]
use {
    crate::pdf::action::PdfActionType, crate::pdf::document::bookmark::PdfBookmark,
    crate::pdf::document::page::annotation::PdfPageAnnotation, crate::pdf::document::page::PdfPage,
};

/// A single link contained within a [PdfPage], a [PdfPageAnnotation], or a [PdfBookmark].
///
/// Each link may have a corresponding [PdfAction] that will be triggered when the user
/// interacts with the link, and a [PdfDestination] that indicates the target of any behaviour
/// triggered by the [PdfAction].
pub struct PdfLink<'a> {
    handle: FPDF_LINK,
    document: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

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

    /// Returns the internal `FPDF_LINK` handle for this [PdfLink].
    #[inline]
    pub(crate) fn handle(&self) -> FPDF_LINK {
        self.handle
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfLink].
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the [PdfAction] associated with this [PdfLink], if any.
    ///
    /// The action indicates the behaviour that will occur when the user interacts with the
    /// link in a PDF viewer. For most links, this will be a local navigation action
    /// of type [PdfActionType::GoToDestinationInSameDocument], but the PDF file format supports
    /// a variety of other actions.
    pub fn action(&self) -> Option<PdfAction<'a>> {
        let handle = self.bindings().FPDFLink_GetAction(self.handle());

        if handle.is_null() {
            None
        } else {
            Some(PdfAction::from_pdfium(
                handle,
                self.document,
                self.bindings(),
            ))
        }
    }

    /// Returns the [PdfDestination] associated with this [PdfLink], if any.
    ///
    /// The destination specifies the page and region, if any, that will be the target
    /// of any behaviour that will occur when the user interacts with the link in a PDF viewer.
    pub fn destination(&self) -> Option<PdfDestination<'a>> {
        let handle = self
            .bindings()
            .FPDFLink_GetDest(self.document, self.handle());

        if handle.is_null() {
            None
        } else {
            Some(PdfDestination::from_pdfium(
                self.document,
                handle,
                self.bindings(),
            ))
        }
    }

    /// Returns the [PdfRect] associated with this [PdfLink], if any.
    ///
    /// The PdfRect specifies the area on the page that can be clicked on to activate the link.
    pub fn rect(&self) -> Option<PdfRect> {
        // If the underlying Pdfium function is named FPDFLink_GetAnnotRect(), then why is this
        // function named rect() and not annot_rect() or somesuch?  Because the name of
        // FPDFLink_GetAnnotRect() comes from Pdfium's public API calling links "Link Annotations".
        // For clarity, pdfium-render doesn't follow that convention; in fact, Pdfium doesn't even
        // follow it internally - it's internal function that does the same thing is named
        // CPDF_Link::GetRect().
        let mut the_rect = FS_RECTF {
            left: 0.,
            top: 0.,
            right: 0.,
            bottom: 0.,
        };
        let result = self
            .bindings()
            .FPDFLink_GetAnnotRect(self.handle(), &mut the_rect);
        match result {
            0 => None,
            _ => Some(PdfRect::from_pdfium(the_rect)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_link_rect() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();
        let document = pdfium.load_pdf_from_file("./test/links-test.pdf", None)?;
        // The document contains a single page with a single link
        const EXPECTED: PdfRect = PdfRect::new_from_values(733.3627, 207.85417, 757.6127, 333.1458);
        // Allow a little bit of error, because it's unreasonable to expect floating point
        // calculations to be identical across builds and platforms.
        const ABS_ERR: PdfPoints = PdfPoints::new(f32::EPSILON * 1000.);
        let actual = document
            .pages()
            .iter()
            .next()
            .unwrap()
            .links()
            .iter()
            .next()
            .unwrap()
            .rect()
            .unwrap();
        assert!((actual.top() - EXPECTED.top()).abs() < ABS_ERR);
        assert!((actual.bottom() - EXPECTED.bottom()).abs() < ABS_ERR);
        assert!((actual.left() - EXPECTED.left()).abs() < ABS_ERR);
        assert!((actual.right() - EXPECTED.right()).abs() < ABS_ERR);
        Ok(())
    }
}
