//! Defines the [PdfPageObjects] struct, exposing functionality related to the
//! page objects contained within a single `PdfPage`.

use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::color::PdfColor;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::font::PdfFont;
use crate::page::{PdfPage, PdfPoints, PdfRect};
use crate::page_object::{PdfPageObject, PdfPageObjectCommon};
use crate::page_object_group::PdfPageGroupObject;
use crate::page_object_path::PdfPagePathObject;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_object_text::PdfPageTextObject;
use std::ops::{Range, RangeInclusive};
use std::os::raw::c_int;

pub type PdfPageObjectIndex = usize;

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
    pub fn get(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject<'a>, PdfiumError> {
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

    /// Returns an iterator over all the [PdfPageObject] objects in this [PdfPageObjects] collection.
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
        object.add_object_to_page(self).and_then(|_| {
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

    /// Adds the given [PdfPageTextObject] to this [PdfPageObjects] collection,
    /// returning the text object wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    #[inline]
    pub fn add_text_object(
        &mut self,
        object: PdfPageTextObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        self.add_object(PdfPageObject::Text(object))
    }

    /// Creates a new [PdfPageTextObject] at the given x and y page co-ordinates
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

        object.translate(x, y)?;

        self.add_text_object(object)
    }

    /// Adds the given [PdfPagePathObject] to this [PdfPageObjects] collection,
    /// returning the path object wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    #[inline]
    pub fn add_path_object(
        &mut self,
        object: PdfPagePathObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        self.add_object(PdfPageObject::Path(object))
    }

    /// Creates a new [PdfPagePathObject] for the given line, with the given
    /// stroke settings applied. The new path object will be added to this [PdfPageObjects] collection
    /// and then returned, wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    pub fn create_path_object_line(
        &mut self,
        x1: PdfPoints,
        y1: PdfPoints,
        x2: PdfPoints,
        y2: PdfPoints,
        stroke_color: PdfColor,
        stroke_width: PdfPoints,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let object = PdfPagePathObject::new_line_from_bindings(
            self.bindings,
            x1,
            y1,
            x2,
            y2,
            stroke_color,
            stroke_width,
        )?;

        self.add_path_object(object)
    }

    /// Creates a new [PdfPagePathObject] for the given rectangle, with the given
    /// fill and stroke settings applied. Both the stroke color and the stroke width must be
    /// provided for the rectangle to be stroked. The new path object will be added to
    /// this [PdfPageObjects] collection and then returned, wrapped inside a generic
    /// [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    pub fn create_path_object_rect(
        &mut self,
        rect: PdfRect,
        stroke_color: Option<PdfColor>,
        stroke_width: Option<PdfPoints>,
        fill_color: Option<PdfColor>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let object = PdfPagePathObject::new_rect_from_bindings(
            self.bindings,
            rect,
            stroke_color,
            stroke_width,
            fill_color,
        )?;

        self.add_path_object(object)
    }

    /// Creates a new [PdfPagePathObject]. The new path will be created with a circle that fills
    /// the given rectangle, with the given fill and stroke settings applied. Both the stroke color
    /// and the stroke width must be provided for the circle to be stroked. The new path object
    /// will be added to this [PdfPageObjects] collection and then returned, wrapped inside a generic
    /// [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    pub fn create_path_object_circle(
        &mut self,
        rect: PdfRect,
        stroke_color: Option<PdfColor>,
        stroke_width: Option<PdfPoints>,
        fill_color: Option<PdfColor>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let object = PdfPagePathObject::new_circle_from_bindings(
            self.bindings,
            rect,
            stroke_color,
            stroke_width,
            fill_color,
        )?;

        self.add_path_object(object)
    }

    /// Creates a new [PdfPagePathObject]. The new path will be created with a circle centered
    /// at the given coordinates, with the given radius, and with the given fill and stroke settings
    /// applied. Both the stroke color and the stroke width must be provided for the circle to be
    /// stroked. The new path object will be added to this [PdfPageObjects] collection and then
    /// returned, wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    pub fn create_path_object_circle_at(
        &mut self,
        center_x: PdfPoints,
        center_y: PdfPoints,
        radius: PdfPoints,
        stroke_color: Option<PdfColor>,
        stroke_width: Option<PdfPoints>,
        fill_color: Option<PdfColor>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let object = PdfPagePathObject::new_circle_at_from_bindings(
            self.bindings,
            center_x,
            center_y,
            radius,
            stroke_color,
            stroke_width,
            fill_color,
        )?;

        self.add_path_object(object)
    }

    /// Creates a new [PdfPagePathObject]. The new path will be created with an ellipse that fills
    /// the given rectangle, with the given fill and stroke settings applied. Both the stroke color
    /// and the stroke width must be provided for the ellipse to be stroked. The new path object
    /// will be added to this [PdfPageObjects] collection and then returned, wrapped inside a generic
    /// [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    pub fn create_path_object_ellipse(
        &mut self,
        rect: PdfRect,
        stroke_color: Option<PdfColor>,
        stroke_width: Option<PdfPoints>,
        fill_color: Option<PdfColor>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let object = PdfPagePathObject::new_ellipse_from_bindings(
            self.bindings,
            rect,
            stroke_color,
            stroke_width,
            fill_color,
        )?;

        self.add_path_object(object)
    }

    /// Creates a new [PdfPagePathObject]. The new path will be created with an ellipse centered
    /// at the given coordinates, with the given radii, and with the given fill and stroke settings
    /// applied. Both the stroke color and the stroke width must be provided for the ellipse to be
    /// stroked. The new path object will be added to this [PdfPageObjects] collection and then
    /// returned, wrapped inside a generic [PdfPageObject] wrapper.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then the content regeneration
    /// will be triggered on the page.
    #[allow(clippy::too_many_arguments)]
    pub fn create_path_object_ellipse_at(
        &mut self,
        center_x: PdfPoints,
        center_y: PdfPoints,
        x_radius: PdfPoints,
        y_radius: PdfPoints,
        stroke_color: Option<PdfColor>,
        stroke_width: Option<PdfPoints>,
        fill_color: Option<PdfColor>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        let object = PdfPagePathObject::new_ellipse_at_from_bindings(
            self.bindings,
            center_x,
            center_y,
            x_radius,
            y_radius,
            stroke_color,
            stroke_width,
            fill_color,
        )?;

        self.add_path_object(object)
    }

    /// Creates a new object group that can accept any [PdfPageObject] in this [PdfPageObjects]
    /// collection. The newly created group will be empty; you will need to manually add to it
    /// the objects you want to manipulate.
    ///
    /// To create a populated group, call the [PdfPageGroupObjects::new()] function with
    /// a predicate function that selects the objects on this page to include in the new group.
    pub fn create_empty_group(&self) -> PdfPageGroupObject<'a> {
        PdfPageGroupObject::from_pdfium(
            self.page_handle,
            self.bindings,
            self.do_regenerate_page_content_after_each_change,
        )
    }

    /// Removes the given [PdfPageObject] from this [PdfPageObjects] collection. The object's
    /// memory ownership will be removed from the [PdfPage] containing this [PdfPageObjects]
    /// collection, and the updated page object will be returned. It can be added back to a
    /// page objects collection or dropped, at which point the memory owned by the object will
    /// be freed.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn remove_object(
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

    /// Removes the [PdfPageObject] at the given index from this [PdfPageObjects] collection.
    /// The object's memory ownership will be removed from the [PdfPage] containing this [PdfPageObjects]
    /// collection, and the updated page object will be returned. It can be added back into a
    /// page objects collection or discarded, at which point the memory owned by the object will
    /// be freed.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn remove_object_at_index(
        &mut self,
        index: PdfPageObjectIndex,
    ) -> Result<PdfPageObject, PdfiumError> {
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
            self.remove_object(PdfPageObject::from_pdfium(
                object_handle,
                self.page_handle,
                self.bindings,
            ))
        }
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
            *source.get_handle(),
            source.get_document(),
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
        let source = PdfPage::from_pdfium(page, None, document, self.bindings);

        // Make sure we iterate over the range backwards. The collection's length will reduce
        // each time we remove an object from it, and we must avoid overflow or Pdfium may segfault.

        for index in source_page_object_range.rev() {
            let mut object = source.objects().get(index)?;

            object.remove_object_from_page()?;
            object.add_object_to_page(self)?;
        }

        Ok(())
    }

    /// Removes all page objects in the given [PdfPage] into this [PdfPageObjects] collection,
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
