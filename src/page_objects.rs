//! Defines the [PdfPageObjects] struct, exposing functionality related to the
//! page objects contained within a single `PdfPage`.

use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPage;
use crate::page_object::PdfPageObject;
use crate::page_object_group::PdfPageGroupObject;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_objects_common::{
    PdfPageObjectIndex, PdfPageObjectsCommon, PdfPageObjectsIterator,
};
use crate::page_objects_private::internal::PdfPageObjectsPrivate;
use std::ops::RangeInclusive;
use std::os::raw::c_int;

/// The page objects contained within a single [PdfPage].
///
/// Content on a page is structured as a stream of [PdfPageObject] objects of different types:
/// text objects, image objects, path objects, and so on.
///
/// Note that Pdfium does not support or recognize all PDF page object types. For instance,
/// Pdfium does not currently support or recognize the External Object ("XObject") page object type
/// supported by Adobe Acrobat and Foxit's commercial PDF SDK. In these cases, Pdfium will return
/// `PdfPageObjectType::Unsupported`.
pub struct PdfPageObjects<'a> {
    document_handle: FPDF_DOCUMENT,
    page_handle: FPDF_PAGE,
    bindings: &'a dyn PdfiumLibraryBindings,
    do_regenerate_page_content_after_each_change: bool,
}

impl<'a> PdfPageObjects<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            document_handle,
            page_handle,
            bindings,
            do_regenerate_page_content_after_each_change: false,
        }
    }

    /// Returns the internal `FPDF_PAGE` handle for the [PdfPage] containing this [PdfPageObjects] collection.
    #[inline]
    pub(crate) fn get_page_handle(&self) -> &FPDF_PAGE {
        &self.page_handle
    }

    /// Sets whether or not this [PdfPageObjects] collection should trigger content regeneration
    /// on its containing [PdfPage] when the collection is mutated.
    #[inline]
    pub(crate) fn do_regenerate_page_content_after_each_change(
        &mut self,
        do_regenerate_page_content_after_each_change: bool,
    ) {
        self.do_regenerate_page_content_after_each_change =
            do_regenerate_page_content_after_each_change;
    }

    /// Removes a single page object with the given source page object index from the given
    /// source [PdfPage], adding the object to the end of this [PdfPageObjects] collection.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    ///
    /// Likewise, if the given source [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the source page.
    pub fn take_object_from_page(
        &mut self,
        source: &mut PdfPage,
        source_page_object_index: PdfPageObjectIndex,
    ) -> Result<(), PdfiumError> {
        self.take_object_range_from_page(
            source,
            source_page_object_index..=source_page_object_index,
        )
    }

    /// Removes one or more page objects with the given range of indices from the given
    /// source [PdfPage], adding the objects sequentially to the end of this
    /// [PdfPageObjects] collection.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    ///
    /// Likewise, if the given source [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the source page.
    pub fn take_object_range_from_page(
        &mut self,
        source: &mut PdfPage,
        source_page_object_range: RangeInclusive<PdfPageObjectIndex>,
    ) -> Result<(), PdfiumError> {
        self.take_object_range_from_handles(
            *source.handle(),
            source.document(),
            source_page_object_range,
        )
    }

    // Take a raw FPDF_PAGE handle to avoid cascading lifetime problems associated with borrowing
    // &'a mut PdfPage<'a>.
    pub(crate) fn take_object_range_from_handles(
        &mut self,
        page: FPDF_PAGE,
        document: &PdfDocument,
        source_page_object_range: RangeInclusive<PdfPageObjectIndex>,
    ) -> Result<(), PdfiumError> {
        let source = PdfPage::from_pdfium(page, None, document);

        // Make sure we iterate over the range backwards. The collection's length will reduce
        // each time we remove an object from it, and we must avoid overflow or Pdfium may segfault.

        for index in source_page_object_range.rev() {
            let mut object = source.objects().get(index)?;

            object.remove_object_from_page()?;
            object.add_object_to_page(self)?;
        }

        Ok(())
    }

    /// Moves all page objects in the given [PdfPage] into this [PdfPageObjects] collection,
    /// appending them to the end of this [PdfPageObjects] collection. The given [PdfPage]
    /// will be drained of all page objects once this operation is completed.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    ///
    /// Likewise, if the given source [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the source page.
    pub fn take_all(&mut self, source: &mut PdfPage) -> Result<(), PdfiumError> {
        self.take_object_range_from_page(source, source.objects().as_range_inclusive())
    }

    /// Creates a new [PdfPageGroupObject] object group that includes any page objects in this
    /// [PdfPageObjects] collection matching the given predicate function.
    pub fn create_group<F>(&'a self, predicate: F) -> Result<PdfPageGroupObject<'a>, PdfiumError>
    where
        F: Fn(&PdfPageObject) -> bool,
    {
        let mut result = self.create_empty_group();

        for mut object in self.iter().filter(predicate) {
            result.push(&mut object)?;
        }

        Ok(result)
    }

    /// Creates a new [PdfPageGroupObject] object group that can accept any [PdfPageObject]
    /// in this [PdfPageObjects]. The newly created group will be empty;
    /// you will need to manually add to it the objects you want to manipulate.
    #[inline]
    pub fn create_empty_group(&self) -> PdfPageGroupObject<'a> {
        PdfPageGroupObject::from_pdfium(
            self.page_handle,
            self.bindings,
            self.do_regenerate_page_content_after_each_change,
        )
    }
}

impl<'a> PdfPageObjectsPrivate<'a> for PdfPageObjects<'a> {
    #[inline]
    fn document_handle(&self) -> FPDF_DOCUMENT {
        self.document_handle
    }

    #[inline]
    fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn len_impl(&self) -> PdfPageObjectIndex {
        self.bindings.FPDFPage_CountObjects(self.page_handle) as PdfPageObjectIndex
    }

    fn get_impl(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageObjectIndexOutOfBounds);
        }

        let object_handle = self
            .bindings
            .FPDFPage_GetObject(self.page_handle, index as c_int);

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
                object_handle,
                Some(self.page_handle),
                None,
                self.bindings,
            ))
        }
    }

    #[inline]
    fn iter_impl(&'a self) -> PdfPageObjectsIterator<'a> {
        PdfPageObjectsIterator::new(self)
    }

    fn add_object_impl(
        &mut self,
        mut object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        object.add_object_to_page(self).and_then(|_| {
            if self.do_regenerate_page_content_after_each_change {
                if !self
                    .bindings
                    .is_true(self.bindings.FPDFPage_GenerateContent(self.page_handle))
                {
                    if let Some(error) = self.bindings.get_pdfium_last_error() {
                        Err(PdfiumError::PdfiumLibraryInternalError(error))
                    } else {
                        // This would be an unusual situation; an FPDF_BOOL result indicating failure,
                        // yet pdfium's error code indicates success.

                        Err(PdfiumError::PdfiumLibraryInternalError(
                            PdfiumInternalError::Unknown,
                        ))
                    }
                } else {
                    Ok(object)
                }
            } else {
                Ok(object)
            }
        })
    }

    fn remove_object_impl(
        &mut self,
        mut object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        object.remove_object_from_page().and_then(|_| {
            if self.do_regenerate_page_content_after_each_change {
                self.bindings.FPDFPage_GenerateContent(self.page_handle);

                if let Some(error) = self.bindings.get_pdfium_last_error() {
                    Err(PdfiumError::PdfiumLibraryInternalError(error))
                } else {
                    Ok(object)
                }
            } else {
                Ok(object)
            }
        })
    }
}
