//! Defines the [PdfPageObjects] struct, exposing functionality related to the
//! page objects contained within a single `PdfPage`.

use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::font::PdfFont;
use crate::page::{PdfPage, PdfPoints};
use crate::page_object::{PdfPageObject, PdfPageObjectCommon};
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_object_text::PdfPageTextObject;
use std::ops::{Range, RangeInclusive};
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

    /// Returns the total number of page objects within the containing [PdfPage].
    #[inline]
    pub fn len(&self) -> PdfPageObjectIndex {
        self.bindings.FPDFPage_CountObjects(self.page_handle) as PdfPageObjectIndex
    }

    /// Returns true if this [PdfPageObjects] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of objects)` for this [PdfPageObjects] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageObjectIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of objects - 1)` for this [PdfPageObjects] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfPageObjectIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfPageObject] from this [PdfPageObjects] collection.
    pub fn get(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject, PdfiumError> {
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
                self.page_handle,
                self.bindings,
            ))
        }
    }

    /// Returns an iterator over all the page objects in this [PdfPageObjects] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageObjectsIterator {
        PdfPageObjectsIterator::new(self)
    }

    /// Adds the given [PdfPageObject] to this [PdfPageObjects] collection. The object's
    /// memory ownership will be transferred to the [PdfPage] containing this [PdfPageObjects]
    /// collection, and the updated page object will be returned.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn add_object(
        &mut self,
        mut object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        self.bindings
            .FPDFPage_InsertObject(self.page_handle, *object.get_handle());

        if let Some(error) = self.bindings.get_pdfium_last_error() {
            Err(PdfiumError::PdfiumLibraryInternalError(error))
        } else {
            // Update the object's ownership.

            object.set_object_memory_owned_by_page(self.page_handle);

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
        }
    }

    /// Adds the given [PdfPageObjectText] to this [PdfPageObjects] collection,
    /// returning the text object wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn add_text_object(
        &mut self,
        object: PdfPageTextObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        self.add_object(PdfPageObject::Text(object))
    }

    /// Creates a new [PdfPageObjectText] at the given x and y page co-ordinates
    /// from the given arguments and adds it to this [PdfPageObjects] collection,
    /// returning the text object wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    pub fn create_text_object(
        &mut self,
        x: PdfPoints,
        y: PdfPoints,
        text: impl ToString,
        font: &PdfFont,
        font_size: PdfPoints,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let mut object = PdfPageTextObject::new_from_handles(
            self.document_handle,
            text,
            *font.get_handle(),
            font_size,
            self.bindings,
        )?;

        object.translate(x, y);

        self.add_text_object(object)
    }

    /// Deletes the given [PdfPageObject] from this [PdfPageObjects] collection. The object's
    /// memory ownership will be removed from the [PdfPage] containing this [PdfPageObjects]
    /// collection, and the updated page object will be returned. It can be added back to a
    /// page objects collection or dropped, at which point the memory owned by the object will
    /// be freed.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn delete_object(
        &mut self,
        mut object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        if self.bindings.is_true(
            self.bindings
                .FPDFPage_RemoveObject(self.page_handle, *object.get_handle()),
        ) {
            // Update the object's ownership.

            object.set_object_memory_released_by_page();

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
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
            ))
        }
    }

    /// Deletes the [PdfPageObject] at the given index from this [PdfPageObjects] collection.
    /// The object's memory ownership will be removed from the [PdfPage] containing this [PdfPageObjects]
    /// collection, and the updated page object will be returned. It can be added back to a
    /// page objects collection or discarded, at which point the memory owned by the object will
    /// be dropped.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn delete_object_at_index(
        &mut self,
        index: PdfPageObjectIndex,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
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
            self.delete_object(PdfPageObject::from_pdfium(
                object_handle,
                self.page_handle,
                self.bindings,
            ))
        }
    }

    /// Copies a single page object with the given source page object index from the given
    /// source [PdfPage], adding the object to the end of this [PdfPageObjectsMut] collection.
    ///
    /// Note that Pdfium does not support or recognize all PDF page object types. For instance,
    /// Pdfium does not currently support or recognize the External Object ("XObject") page object
    /// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. If the page object is
    /// of a type not supported by Pdfium, it will be silently ignored and not copied.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn copy_object_from_page(
        &mut self,
        source: &'a PdfPage<'a>,
        source_page_object_index: PdfPageObjectIndex,
    ) -> Result<(), PdfiumError> {
        self.copy_object_range_from_page(
            source,
            source_page_object_index..=source_page_object_index,
        )
    }

    /// Copies one or more page objects with the given range of indices from the given
    /// source [PdfPage], adding the objects sequentially to the end of this
    /// [PdfPageObjectsMut] collection.
    ///
    /// Note that Pdfium does not support or recognize all PDF page object types. For instance,
    /// Pdfium does not currently support or recognize the External Object ("XObject") page object
    /// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. Page objects not supported
    /// by Pdfium will be silently ignored by this function and will not copied.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn copy_object_range_from_page(
        &mut self,
        source: &'a PdfPage<'a>,
        source_page_object_range: RangeInclusive<PdfPageObjectIndex>,
    ) -> Result<(), PdfiumError> {
        for index in source_page_object_range {
            let object = source.objects().get(index)?;

            let clone = match object {
                PdfPageObject::Text(ref object) => Some(self.create_text_object(
                    object.bounds()?.left,
                    object.bounds()?.bottom,
                    object.text(),
                    &object.font(),
                    object.font_size(),
                )?),
                // TODO: AJRC - 30/5/22 - inline cloning of all supported page object types
                // PdfPageObject::Path(_) => {}
                // PdfPageObject::Image(_) => {}
                // PdfPageObject::Shading(_) => {}
                // PdfPageObject::FormFragment(_) => {}
                PdfPageObject::Unsupported(_) => None,
                _ => unimplemented!(),
            };

            if let Some(clone) = clone {
                self.add_object(clone)?;
            }
        }

        Ok(())
    }

    /// Copies all page objects in the given [PdfPage] into this [PdfPageObjectsMut] collection,
    /// appending them to the end of this [PdfPageObjectsMut] collection.
    ///
    /// For finer control over which page objects are imported, use one of the
    /// [PdfPageObjectsMut::import_object_from_page()] or
    /// [PdfPageObjectsMut::import_object_range_from_page()] functions.
    ///
    /// Note that Pdfium does not support or recognize all PDF page object types. For instance,
    /// Pdfium does not currently support or recognize the External Object ("XObject") page object
    /// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. Page objects not supported
    /// by Pdfium will be silently ignored by this function and will not copied.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    ///
    /// Calling this function is equivalent to
    ///
    /// ```
    /// self.import_object_range_from_page(
    ///     page, // Source
    ///     page.objects().as_range_inclusive(), // Select all page objects
    /// );
    /// ```
    pub fn copy_all(&mut self, page: &'a PdfPage<'a>) -> Result<(), PdfiumError> {
        self.copy_object_range_from_page(page, page.objects().as_range_inclusive())
    }

    /// Removes a single page object with the given source page object index from the given
    /// source [PdfPage], adding the object to the end of this [PdfPageObjectsMut] collection.
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
        source: &'a mut PdfPage<'a>,
        source_page_object_index: PdfPageObjectIndex,
    ) -> Result<(), PdfiumError> {
        self.take_object_range_from_page(
            source,
            source_page_object_index..=source_page_object_index,
        )
    }

    /// Removes one or more page objects with the given range of indices from the given
    /// source [PdfPage], adding the objects sequentially to the end of this
    /// [PdfPageObjectsMut] collection.
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
        source: &mut PdfPage<'a>,
        source_page_object_range: RangeInclusive<PdfPageObjectIndex>,
    ) -> Result<(), PdfiumError> {
        for index in source_page_object_range {
            let mut object = source.objects_mut().delete_object_at_index(index)?;

            // Update the object's ownership.

            object.set_object_memory_owned_by_page(self.page_handle);

            // Avoid a lifetime ownership problem when transferring the object from one collection
            // to another by dropping the object and creating a new one from the same handle.

            self.add_object(PdfPageObject::from_pdfium(
                *object.get_handle(),
                self.page_handle,
                self.bindings,
            ))?;
        }

        source.set_content_regeneration_required();

        Ok(())
    }

    /// Removes all page objects in the given [PdfPage] into this [PdfPageObjectsMut] collection,
    /// appending them to the end of this [PdfPageObjectsMut] collection. The given [PdfPage]
    /// will be drained of all page objects once this operation is completed.
    ///
    /// For finer control over which page objects are imported, use one of the
    /// [PdfPageObjectsMut::import_object_from_page()] or
    /// [PdfPageObjectsMut::import_object_range_from_page()] functions.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    ///
    /// Likewise, if the given source [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the source page.
    ///
    /// Calling this function is equivalent to
    ///
    /// ```
    /// self.take_object_range_from_page(
    ///     page, // Source
    ///     page.objects().as_range_inclusive(), // Select all page objects
    /// );
    /// ```
    pub fn take_all(&mut self, page: &'a mut PdfPage<'a>) -> Result<(), PdfiumError> {
        self.take_object_range_from_page(page, page.objects().as_range_inclusive())
    }
}

/// An iterator over all the [PdfPageObject] objects in a [PdfPageObjects] collection.
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
