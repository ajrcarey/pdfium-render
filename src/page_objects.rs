//! Defines the [PdfPageObjects] struct, exposing functionality related to the
//! page objects contained within a single `PdfPage`.

use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPage;
use crate::page_object::PdfPageObject;
use std::ops::Range;
use std::os::raw::c_int;

pub type PdfPageObjectIndex = usize;

/// The page objects contained within a single [PdfPage].
///
/// Content in a PDF page is structured as a stream of [PdfPageObject] objects of different types:
/// text objects, image objects, path objects, and so on. Note that Pdfium does not support or
/// recognize all PDF page object types. For instance, Pdfium does not currently support or
/// recognize the External Object ("XObject") page object type supported by Adobe Acrobat and
/// Foxit's commercial PDF SDK. In these cases, Pdfium will return `PdfPageObjectType::Unsupported`.
pub struct PdfPageObjects<'a> {
    page: &'a PdfPage<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageObjects<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        page: &'a PdfPage<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self { page, bindings }
    }

    /// Returns the total number of page objects within the containing [PdfPage].
    #[inline]
    pub fn len(&self) -> PdfPageObjectIndex {
        self.bindings.FPDFPage_CountObjects(*self.page.get_handle()) as PdfPageObjectIndex
    }

    /// Returns true if this [PdfPageObjects] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from 0..(number of objects) for this [PdfPageObjects] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageObjectIndex> {
        0..self.len()
    }

    /// Returns a single [PdfPageObject] from this [PdfPageObjects] collection.
    pub fn get(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageObjectIndexOutOfBounds);
        }

        let object_handle = self
            .bindings
            .FPDFPage_GetObject(*self.page.get_handle(), index as c_int);

        if object_handle.is_null() {
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
            Ok(PdfPageObject::from_pdfium(
                index,
                object_handle,
                self.page,
                self.bindings,
            ))
        }
    }

    /// Returns an iterator over all the page objects in this [PdfPageObjects] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageObjectsIterator {
        PdfPageObjectsIterator::new(self)
    }
}

/// An iterator over all the [PdfPage] objects in a [PdfPages] collection.
pub struct PdfPageObjectsIterator<'a> {
    objects: &'a PdfPageObjects<'a>,
    next_index: PdfPageObjectIndex,
}

impl<'a> PdfPageObjectsIterator<'a> {
    #[inline]
    pub(crate) fn new(objects: &'a PdfPageObjects<'a>) -> Self {
        PdfPageObjectsIterator {
            objects,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfPageObjectsIterator<'a> {
    type Item = PdfPageObject<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.objects.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
