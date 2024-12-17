//! Defines the [PdfPageGroupObject] struct, exposing functionality related to a group of
//! page objects contained in the same `PdfPageObjects` collection.

use crate::bindgen::{FPDF_DOCUMENT, FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::create_transform_setters;
use crate::error::PdfiumError;
use crate::pdf::color::PdfColor;
use crate::pdf::document::page::index_cache::PdfPageIndexCache;
use crate::pdf::document::page::object::path::PdfPathFillMode;
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::{
    PdfPageObject, PdfPageObjectBlendMode, PdfPageObjectCommon, PdfPageObjectLineCap,
    PdfPageObjectLineJoin,
};
use crate::pdf::document::page::objects::common::{PdfPageObjectIndex, PdfPageObjectsCommon};
use crate::pdf::document::page::{PdfPage, PdfPageContentRegenerationStrategy};
use crate::pdf::document::pages::{PdfPageIndex, PdfPages};
use crate::pdf::document::PdfDocument;
use crate::pdf::matrix::PdfMatrix;
use crate::pdf::matrix::PdfMatrixValue;
use crate::pdf::points::PdfPoints;
use crate::pdf::quad_points::PdfQuadPoints;
use crate::pdf::rect::PdfRect;
use crate::pdfium::Pdfium;
use std::collections::HashMap;

/// A group of [PdfPageObject] objects contained in the same `PdfPageObjects` collection.
/// The page objects contained in the group can be manipulated and transformed together
/// as if they were a single object.
///
/// Groups are bound to specific pages in the document. To create an empty group, use either the
/// `PdfPageObjects::create_new_group()` function or the [PdfPageGroupObject::empty()] function.
/// To create a populated group, use one of the [PdfPageGroupObject::new()],
/// [PdfPageGroupObject::from_vec()], or [PdfPageGroupObject::from_slice()] functions.
pub struct PdfPageGroupObject<'a> {
    document_handle: FPDF_DOCUMENT,
    page_handle: FPDF_PAGE,
    object_handles: Vec<FPDF_PAGEOBJECT>,
    bindings: &'a dyn PdfiumLibraryBindings,
    do_regenerate_page_content_after_each_change: bool,
}

impl<'a> PdfPageGroupObject<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
        do_regenerate_page_content_after_each_change: bool,
    ) -> Self {
        PdfPageGroupObject {
            page_handle,
            document_handle,
            object_handles: Vec::new(),
            bindings,
            do_regenerate_page_content_after_each_change,
        }
    }

    /// Creates a new, empty [PdfPageGroupObject] that can be used to hold any page objects
    /// on the given [PdfPage].
    pub fn empty(page: &'a PdfPage) -> Self {
        Self::from_pdfium(
            page.document_handle(),
            page.page_handle(),
            page.bindings(),
            page.content_regeneration_strategy()
                == PdfPageContentRegenerationStrategy::AutomaticOnEveryChange,
        )
    }

    /// Creates a new [PdfPageGroupObject] that includes any page objects on the given [PdfPage]
    /// matching the given predicate function.
    pub fn new<F>(page: &'a PdfPage, predicate: F) -> Result<Self, PdfiumError>
    where
        F: FnMut(&PdfPageObject) -> bool,
    {
        let mut result = Self::from_pdfium(
            page.document_handle(),
            page.page_handle(),
            page.bindings(),
            page.content_regeneration_strategy()
                == PdfPageContentRegenerationStrategy::AutomaticOnEveryChange,
        );

        for mut object in page.objects().iter().filter(predicate) {
            result.push(&mut object)?;
        }

        Ok(result)
    }

    /// Creates a new [PdfPageGroupObject] that includes the given page objects on the
    /// given [PdfPage].
    #[inline]
    pub fn from_vec(
        page: &PdfPage<'a>,
        mut objects: Vec<PdfPageObject<'a>>,
    ) -> Result<Self, PdfiumError> {
        Self::from_slice(page, objects.as_mut_slice())
    }

    /// Creates a new [PdfPageGroupObject] that includes the given page objects on the
    /// given [PdfPage].
    pub fn from_slice(
        page: &PdfPage<'a>,
        objects: &mut [PdfPageObject<'a>],
    ) -> Result<Self, PdfiumError> {
        let mut result = Self::from_pdfium(
            page.document_handle(),
            page.page_handle(),
            page.bindings(),
            page.content_regeneration_strategy()
                == PdfPageContentRegenerationStrategy::AutomaticOnEveryChange,
        );

        for object in objects.iter_mut() {
            result.push(object)?;
        }

        Ok(result)
    }

    /// Returns the number of page objects in this group.
    #[inline]
    pub fn len(&self) -> usize {
        self.object_handles.len()
    }

    /// Returns `true` if this group contains no page objects.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if this group already contains the given page object.
    #[inline]
    pub fn contains(&self, object: &PdfPageObject) -> bool {
        self.object_handles.contains(&object.get_object_handle())
    }

    /// Adds a single [PdfPageObject] to this group.
    pub fn push(&mut self, object: &mut PdfPageObject<'a>) -> Result<(), PdfiumError> {
        let was_object_already_attached_to_group_page =
            if let Some(page_handle) = object.get_page_handle() {
                if page_handle != self.page_handle {
                    // The object is attached to a different page.

                    // In theory, transferring ownership of the page object from its current
                    // page to the page referenced by this group should be possible:

                    // object.remove_object_from_page()?;
                    // object.add_object_to_page_handle(self.page)?;

                    // But in practice, as per https://github.com/ajrcarey/pdfium-render/issues/18,
                    // transferring memory ownership of a page object from one page to another
                    // generally segfaults Pdfium. Instead, return an error.

                    return Err(PdfiumError::PageObjectAlreadyAttachedToDifferentPage);
                } else {
                    // The object is already attached to this group's parent page.

                    true
                }
            } else {
                // The object isn't attached to a page.

                object.add_object_to_page_handle(self.page_handle)?;

                false
            };

        self.object_handles.push(object.get_object_handle());

        if !was_object_already_attached_to_group_page
            && self.do_regenerate_page_content_after_each_change
        {
            PdfPage::regenerate_content_immut_for_handle(self.page_handle, self.bindings)?;
        }

        Ok(())
    }

    /// Adds all the given [PdfPageObject] objects to this group.
    pub fn append(&mut self, objects: &mut [PdfPageObject<'a>]) -> Result<(), PdfiumError> {
        // Hold off regenerating page content until all objects have been processed.

        let do_regenerate_page_content_after_each_change =
            self.do_regenerate_page_content_after_each_change;

        self.do_regenerate_page_content_after_each_change = false;

        for object in objects.iter_mut() {
            self.push(object)?;
        }

        // Regenerate page content now, if necessary.

        self.do_regenerate_page_content_after_each_change =
            do_regenerate_page_content_after_each_change;

        if self.do_regenerate_page_content_after_each_change {
            PdfPage::regenerate_content_immut_for_handle(self.page_handle, self.bindings)?;
        }

        Ok(())
    }

    /// Removes every [PdfPageObject] in this group from the group's containing [PdfPage]
    /// and from this group, consuming the group.
    ///
    /// Each object's memory ownership will be removed from the `PdfPageObjects` collection for
    /// this group's containing [PdfPage]. The objects will also be removed from this group,
    /// and the memory owned by each object will be freed.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn remove_objects_from_page(mut self) -> Result<(), PdfiumError> {
        // Hold off regenerating page content until all objects have been processed.

        let do_regenerate_page_content_after_each_change =
            self.do_regenerate_page_content_after_each_change;

        self.do_regenerate_page_content_after_each_change = false;

        // Remove the selected objects from the source page.

        self.apply_to_each(|object| object.remove_object_from_page())?;
        self.object_handles.clear();

        // A curious upstream bug in Pdfium means that any objects _not_ removed from the page
        // may be vertically reflected and translated. Attempt to mitigate this.
        // For more details, see: https://github.com/ajrcarey/pdfium-render/issues/60

        let page_height = PdfPoints::new(self.bindings.FPDF_GetPageHeightF(self.page_handle));

        for index in 0..self.bindings.FPDFPage_CountObjects(self.page_handle) {
            let mut object = PdfPageObject::from_pdfium(
                self.bindings.FPDFPage_GetObject(self.page_handle, index),
                Some(self.page_handle),
                None,
                self.bindings,
            );

            // Undo the reflection effect.
            // TODO: AJRC - 28/1/23 - it is not clear that _all_ objects need to be unreflected.
            // The challenge here is detecting which objects, if any, have been affected by
            // the Pdfium reflection bug. Testing suggests that comparing object transformation matrices
            // before and after object removal doesn't result in any detectable change to the matrices,
            // so that approach doesn't work.

            object.flip_vertically()?;
            object.translate(PdfPoints::ZERO, page_height)?;
        }

        // Regenerate page content now, if necessary.

        self.do_regenerate_page_content_after_each_change =
            do_regenerate_page_content_after_each_change;

        if self.do_regenerate_page_content_after_each_change {
            PdfPage::regenerate_content_immut_for_handle(self.page_handle, self.bindings)?;
        }

        Ok(())
    }

    /// Returns a single [PdfPageObject] from this group.
    #[inline]
    pub fn get(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject, PdfiumError> {
        if let Some(handle) = self.object_handles.get(index) {
            Ok(self.get_object_from_handle(handle))
        } else {
            Err(PdfiumError::PageObjectIndexOutOfBounds)
        }
    }

    /// Retains only the [PdfPageObject] objects in this group specified by the given predicate function.
    ///
    /// Unretained objects are only removed from this group. They remain on the source [PdfPage] that
    /// currently contains them.
    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&PdfPageObject) -> bool,
    {
        // The naive approach of using self.object_handles.retain() directly like so:

        // self.object_handles.retain(|handle| f(&self.get_object_from_handle(handle)));

        // does not work, due to self being borrowed both mutably and immutably simultaneously.
        // Instead, we build a separate list indicating whether each object should be retained
        // or discarded ...

        let mut do_retain = vec![false; self.object_handles.len()];

        for (index, handle) in self.object_handles.iter().enumerate() {
            do_retain[index] = f(&self.get_object_from_handle(handle));
        }

        // ... and then we use that marker list in our call to self.object_handles.retain().

        let mut index = 0;

        self.object_handles.retain(|_| {
            // Should the object at index position |index| be retained?

            let do_retain = do_retain[index];

            index += 1;

            do_retain
        });
    }

    /// Retains only the [PdfPageObject] objects in this group that can be copied.
    ///
    /// Objects that cannot be copied are only removed from this group. They remain on the source
    /// [PdfPage] that currently contains them.
    #[inline]
    pub fn retain_if_copyable(&mut self) {
        self.retain(|object| object.is_copyable());
    }

    /// Returns `true` if all the [PdfPageObject] objects in this group can be copied.
    #[inline]
    pub fn is_copyable(&self) -> bool {
        self.iter().all(|object| object.is_copyable())
    }

    /// Attempts to copy all the [PdfPageObject] objects in this group, placing the copied objects
    /// onto the given existing destination [PdfPage].
    ///
    /// This function can only copy page objects supported by the [PdfPageObjectCommon::try_copy()]
    /// function. For a different approach that supports more page object types but is more limited
    /// in where the copied objects can be placed, see the [PdfPageGroupObject::copy_onto_new_page_at_start()],
    /// [PdfPageGroupObject::copy_onto_new_page_at_end()], and
    /// [PdfPageGroupObject::copy_onto_new_page_at_index()] functions.
    ///
    /// If all objects were copied successfully, then a new [PdfPageGroupObject] containing the clones
    /// is returned, allowing the new objects to be manipulated as a group.
    pub fn try_copy_onto_existing_page<'b>(
        &self,
        destination: &mut PdfPage<'b>,
    ) -> Result<PdfPageGroupObject<'b>, PdfiumError> {
        if !self.is_copyable() {
            return Err(PdfiumError::GroupContainsNonCopyablePageObjects);
        }

        let mut group = destination.objects_mut().create_empty_group();

        for handle in self.object_handles.iter() {
            let source = self.get_object_from_handle(handle);

            let clone =
                source.try_copy_impl(destination.document_handle(), destination.bindings())?;

            group.push(&mut destination.objects_mut().add_object(clone)?)?;
        }

        Ok(group)
    }

    /// Copies all the [PdfPageObject] objects in this group by copying the page containing the
    /// objects in this group into a new page at the start of the given destination [PdfDocument]
    /// then removing all objects from the new page _not_ in this group.
    ///
    /// This function differs internally from [PdfPageGroupObject::try_copy_onto_existing_page()]
    /// in that it uses `Pdfium` to copy page objects instead of the [PdfPageObjectCommon::try_copy()]
    /// method provided by `pdfium-render`. As a result, this function can copy some objects that
    /// [PdfPageGroupObject::try_copy_onto_existing_page()] cannot; for example, it can copy
    /// path objects containing Bézier curves. However, it can only copy objects onto a new page,
    /// not an existing page, and it cannot return a new [PdfPageGroupObject] containing the
    /// newly created objects.
    ///
    /// The new page will have the same size and bounding box configuration as the page containing
    /// the objects in this group.
    #[inline]
    pub fn copy_onto_new_page_at_start(
        &self,
        destination: &PdfDocument,
    ) -> Result<(), PdfiumError> {
        self.copy_onto_new_page_at_index(0, destination)
    }

    /// Copies all the [PdfPageObject] objects in this group by copying the page containing the
    /// objects in this group into a new page at the end of the given destination [PdfDocument]
    /// then removing all objects from the new page _not_ in this group.
    ///
    /// This function differs internally from [PdfPageGroupObject::try_copy_onto_existing_page()]
    /// in that it uses `Pdfium` to copy page objects instead of the [PdfPageObjectCommon::try_copy()]
    /// method provided by `pdfium-render`. As a result, this function can copy some objects that
    /// [PdfPageGroupObject::try_copy_onto_existing_page()] cannot; for example, it can copy
    /// path objects containing Bézier curves. However, it can only copy objects onto a new page,
    /// not an existing page, and it cannot return a new [PdfPageGroupObject] containing the
    /// newly created objects.
    ///
    /// The new page will have the same size and bounding box configuration as the page containing
    /// the objects in this group.
    #[inline]
    pub fn copy_onto_new_page_at_end(&self, destination: &PdfDocument) -> Result<(), PdfiumError> {
        self.copy_onto_new_page_at_index(destination.pages().len(), destination)
    }

    /// Copies all the [PdfPageObject] objects in this group by copying the page containing the
    /// objects in this group into a new page in the given destination [PdfDocument] at the given
    /// page index, then removing all objects from the new page _not_ in this group.
    ///
    /// This function differs internally from [PdfPageGroupObject::try_copy_onto_existing_page()]
    /// in that it uses `Pdfium` to copy page objects instead of the [PdfPageObjectCommon::try_copy()]
    /// method provided by `pdfium-render`. As a result, this function can copy some objects that
    /// [PdfPageGroupObject::try_copy_onto_existing_page()] cannot; for example, it can copy
    /// path objects containing Bézier curves. However, it can only copy objects onto a new page,
    /// not an existing page, and it cannot return a new [PdfPageGroupObject] containing the
    /// newly created objects.
    ///
    /// The new page will have the same size and bounding box configuration as the page containing
    /// the objects in this group.
    pub fn copy_onto_new_page_at_index(
        &self,
        index: PdfPageIndex,
        destination: &PdfDocument,
    ) -> Result<(), PdfiumError> {
        // Pdfium provides the FPDF_ImportPages() function for copying one or more pages
        // from one document into another. Using this function as a substitute for true
        // page object cloning allows us to copy some objects (such as path objects containing
        // Bézier curves) that PdfPageObject::try_copy() cannot.

        // To use FPDF_ImportPages() as a cloning substitute, we take the following approach:

        // First, we create a new in-memory document and import the source page for this
        // page object group into that new document.

        let cache = Pdfium::pdfium_document_handle_to_result(
            self.bindings.FPDF_CreateNewDocument(),
            self.bindings,
        )?;

        if let Some(source_page_index) =
            PdfPageIndexCache::get_index_for_page(self.document_handle, self.page_handle)
        {
            PdfPages::copy_page_range_between_documents(
                self.document_handle,
                source_page_index..=source_page_index,
                cache.handle(),
                0,
                self.bindings,
            )?;
        } else {
            return Err(PdfiumError::SourcePageIndexNotInCache);
        }

        // Next, we remove all page objects from the in-memory document _except_ the ones in this group.

        // We cannot compare object references across documents. Instead, we build a map of
        // the types of objects, their positions, their bounds, and their transformation matrices,
        // and use this map to determine which objects should be removed from the in-memory page.

        let mut objects_to_discard = HashMap::new();

        for index in 0..self.bindings.FPDFPage_CountObjects(self.page_handle) {
            let object = PdfPageObject::from_pdfium(
                self.bindings.FPDFPage_GetObject(self.page_handle, index),
                Some(self.page_handle),
                None,
                self.bindings,
            );

            if !self.contains(&object) {
                objects_to_discard.insert(
                    (object.bounds()?, object.matrix()?, object.object_type()),
                    true,
                );
            }
        }

        // We now have a map of objects that should be removed from the in-memory page; after
        // we remove them, only the copies of the objects in this group will remain on the page.

        cache
            .pages()
            .get(0)?
            .objects()
            .create_group(|object| {
                objects_to_discard.contains_key(&(
                    object.bounds().unwrap_or(PdfQuadPoints::ZERO),
                    object.matrix().unwrap_or(PdfMatrix::IDENTITY),
                    object.object_type(),
                ))
            })?
            .remove_objects_from_page()?;

        // Finally, with only the copies of the objects in this group left on the in-memory page,
        // we now copy the page back into the given destination.

        PdfPages::copy_page_range_between_documents(
            cache.handle(),
            0..=0,
            destination.handle(),
            index,
            self.bindings,
        )?;

        Ok(())
    }

    /// Returns an iterator over all the [PdfPageObject] objects in this group.
    #[inline]
    pub fn iter(&'a self) -> PdfPageGroupObjectIterator<'a> {
        PdfPageGroupObjectIterator::new(self)
    }

    /// Returns the text contained within all `PdfPageTextObject` objects in this group.
    #[inline]
    pub fn text(&self) -> String {
        self.text_separated("")
    }

    /// Returns the text contained within all `PdfPageTextObject` objects in this group,
    /// separating each text fragment with the given separator.
    pub fn text_separated(&self, separator: &str) -> String {
        let mut strings = Vec::with_capacity(self.len());

        self.for_each(|object| {
            if let Some(object) = object.as_text_object() {
                strings.push(object.text());
            }
        });

        strings.join(separator)
    }

    /// Returns `true` if any [PdfPageObject] in this group contains transparency.
    #[inline]
    pub fn has_transparency(&self) -> bool {
        self.object_handles.iter().any(|object_handle| {
            PdfPageObject::from_pdfium(*object_handle, Some(self.page_handle), None, self.bindings)
                .has_transparency()
        })
    }

    /// Returns the bounding box of this group of objects. Since the bounds of every object in the
    /// group must be considered, this function has runtime complexity of O(n).
    pub fn bounds(&self) -> Result<PdfRect, PdfiumError> {
        let mut bounds: Option<PdfRect> = None;

        self.object_handles.iter().for_each(|object_handle| {
            if let Ok(object_bounds) = PdfPageObject::from_pdfium(
                *object_handle,
                Some(self.page_handle),
                None,
                self.bindings,
            )
            .bounds()
            {
                if let Some(bounds) = bounds.as_mut() {
                    if object_bounds.bottom() < bounds.bottom {
                        bounds.bottom = object_bounds.bottom();
                    }

                    if object_bounds.left() < bounds.left {
                        bounds.left = object_bounds.left();
                    }

                    if object_bounds.top() > bounds.top {
                        bounds.top = object_bounds.top();
                    }

                    if object_bounds.right() > bounds.right {
                        bounds.right = object_bounds.right();
                    }
                } else {
                    bounds = Some(object_bounds.to_rect());
                }
            }
        });

        bounds.ok_or(PdfiumError::EmptyPageObjectGroup)
    }

    /// Sets the blend mode that will be applied when painting every [PdfPageObject] in this group.
    #[inline]
    pub fn set_blend_mode(
        &mut self,
        blend_mode: PdfPageObjectBlendMode,
    ) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.set_blend_mode(blend_mode))
    }

    /// Sets the color of any filled paths in every [PdfPageObject] in this group.
    #[inline]
    pub fn set_fill_color(&mut self, fill_color: PdfColor) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.set_fill_color(fill_color))
    }

    /// Sets the color of any stroked lines in every [PdfPageObject] in this group.
    ///
    /// Even if an object's path is set with a visible color and a non-zero stroke width,
    /// the object's stroke mode must be set in order for strokes to actually be visible.
    #[inline]
    pub fn set_stroke_color(&mut self, stroke_color: PdfColor) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.set_stroke_color(stroke_color))
    }

    /// Sets the width of any stroked lines in every [PdfPageObject] in this group.
    ///
    /// A line width of 0 denotes the thinnest line that can be rendered at device resolution:
    /// 1 device pixel wide. However, some devices cannot reproduce 1-pixel lines,
    /// and on high-resolution devices, they are nearly invisible. Since the results of rendering
    /// such zero-width lines are device-dependent, their use is not recommended.
    ///
    /// Even if an object's path is set with a visible color and a non-zero stroke width,
    /// the object's stroke mode must be set in order for strokes to actually be visible.
    #[inline]
    pub fn set_stroke_width(&mut self, stroke_width: PdfPoints) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.set_stroke_width(stroke_width))
    }

    /// Sets the line join style that will be used when painting stroked path segments
    /// in every [PdfPageObject] in this group.
    #[inline]
    pub fn set_line_join(&mut self, line_join: PdfPageObjectLineJoin) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.set_line_join(line_join))
    }

    /// Sets the line cap style that will be used when painting stroked path segments
    /// in every [PdfPageObject] in this group.
    #[inline]
    pub fn set_line_cap(&mut self, line_cap: PdfPageObjectLineCap) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.set_line_cap(line_cap))
    }

    /// Sets the method used to determine which sub-paths of any path in a [PdfPageObject]
    /// should be filled, and whether or not any path in a [PdfPageObject] should be stroked,
    /// for every [PdfPageObject] in this group.
    ///
    /// Even if an object's path is set to be stroked, the stroke must be configured with
    /// a visible color and a non-zero width in order to actually be visible.
    #[inline]
    pub fn set_fill_and_stroke_mode(
        &mut self,
        fill_mode: PdfPathFillMode,
        do_stroke: bool,
    ) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| {
            if let Some(object) = object.as_path_object_mut() {
                object.set_fill_and_stroke_mode(fill_mode, do_stroke)
            } else {
                Ok(())
            }
        })
    }

    /// Applies the given closure to each [PdfPageObject] in this group.
    #[inline]
    pub(crate) fn apply_to_each<F, T>(&mut self, f: F) -> Result<(), PdfiumError>
    where
        F: Fn(&mut PdfPageObject<'a>) -> Result<T, PdfiumError>,
    {
        let mut error = None;

        self.object_handles.iter().for_each(|handle| {
            if let Err(err) = f(&mut self.get_object_from_handle(handle)) {
                error = Some(err)
            }
        });

        match error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    /// Calls the given closure on each [PdfPageObject] in this group.
    #[inline]
    pub(crate) fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&mut PdfPageObject<'a>),
    {
        self.object_handles.iter().for_each(|handle| {
            f(&mut self.get_object_from_handle(handle));
        });
    }

    /// Inflates an internal `FPDF_PAGEOBJECT` handle into a [PdfPageObject].
    #[inline]
    pub(crate) fn get_object_from_handle(&self, handle: &FPDF_PAGEOBJECT) -> PdfPageObject<'a> {
        PdfPageObject::from_pdfium(*handle, Some(self.page_handle), None, self.bindings)
    }

    create_transform_setters!(
        &mut Self,
        Result<(), PdfiumError>,
        "every [PdfPageObject] in this group",
        "every [PdfPageObject] in this group.",
        "every [PdfPageObject] in this group,"
    );

    // The internal implementation of the transform() function used by the create_transform_setters!() macro.
    fn transform_impl(
        &mut self,
        a: PdfMatrixValue,
        b: PdfMatrixValue,
        c: PdfMatrixValue,
        d: PdfMatrixValue,
        e: PdfMatrixValue,
        f: PdfMatrixValue,
    ) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.transform(a, b, c, d, e, f))
    }

    // The internal implementation of the reset_matrix() function used by the create_transform_setters!() macro.
    fn reset_matrix_impl(&mut self, matrix: PdfMatrix) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.reset_matrix_impl(matrix))
    }
}

/// An iterator over all the [PdfPageObject] objects in a [PdfPageGroupObject] group.
pub struct PdfPageGroupObjectIterator<'a> {
    group: &'a PdfPageGroupObject<'a>,
    next_index: PdfPageObjectIndex,
}

impl<'a> PdfPageGroupObjectIterator<'a> {
    #[inline]
    pub(crate) fn new(group: &'a PdfPageGroupObject<'a>) -> Self {
        PdfPageGroupObjectIterator {
            group,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfPageGroupObjectIterator<'a> {
    type Item = PdfPageObject<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.group.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_group_bounds() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        let document = pdfium.load_pdf_from_file("./test/export-test.pdf", None)?;

        // Form a group of all text objects in the top half of the first page of music ...

        let page = document.pages().get(2)?;

        let mut group = page.objects().create_empty_group();

        group.append(
            page.objects()
                .iter()
                .filter(|object| {
                    object.object_type() == PdfPageObjectType::Text
                        && object.bounds().unwrap().bottom() > page.height() / 2.0
                })
                .collect::<Vec<_>>()
                .as_mut_slice(),
        )?;

        // ... and confirm the group's bounds are restricted to the top half of the page.

        let bounds = group.bounds()?;

        assert_eq!(bounds.bottom.value, 428.3103);
        assert_eq!(bounds.left.value, 62.60526);
        assert_eq!(bounds.top.value, 807.8812);
        assert_eq!(bounds.right.value, 544.4809);

        Ok(())
    }

    #[test]
    fn test_group_text() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        let document = pdfium.load_pdf_from_file("./test/export-test.pdf", None)?;

        // Form a group of all text objects in the bottom half of the last page of music ...

        let page = document.pages().get(5)?;

        let mut group = page.objects().create_empty_group();

        group.append(
            page.objects()
                .iter()
                .filter(|object| {
                    object.object_type() == PdfPageObjectType::Text
                        && object.bounds().unwrap().bottom() < page.height() / 2.0
                })
                .collect::<Vec<_>>()
                .as_mut_slice(),
        )?;

        // ... and extract the text from the group.

        assert_eq!(group.text_separated(" "), "Cento Concerti Ecclesiastici a Una, a Due, a Tre, e   a Quattro voci Giacomo Vincenti, Venice, 1605 Edited by Alastair Carey Source is the 1605 reprint of the original 1602 publication.  Item #2 in the source. Folio pages f5r (binding B1) in both Can to and Basso partbooks. The Basso partbook is barred; the Canto par tbook is not. The piece is marked ™Canto solo, Û Tenoreº in the  Basso partbook, indicating it can be sung either by a Soprano or by a  Tenor down an octave. V.  Quem vidistis, pastores, dicite, annuntiate nobis: in terris quis apparuit? R.  Natum vidimus, et choros angelorum collaudantes Dominum. Alleluia. What did you see, shepherds, speak, tell us: who has appeared on earth? We saw the new-born, and choirs of angels praising the Lord. Alleluia. Third responsory at Matins on Christmas Day 2  Basso, bar 47: one tone lower in source.");

        Ok(())
    }

    #[test]
    fn test_group_apply() -> Result<(), PdfiumError> {
        // Measure the bounds of a group of objects, translate the group, and confirm the
        // bounds have changed.

        let pdfium = test_bind_to_pdfium();

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        page.objects_mut().create_path_object_rect(
            PdfRect::new_from_values(100.0, 100.0, 200.0, 200.0),
            None,
            None,
            Some(PdfColor::RED),
        )?;

        page.objects_mut().create_path_object_rect(
            PdfRect::new_from_values(150.0, 150.0, 250.0, 250.0),
            None,
            None,
            Some(PdfColor::GREEN),
        )?;

        page.objects_mut().create_path_object_rect(
            PdfRect::new_from_values(200.0, 200.0, 300.0, 300.0),
            None,
            None,
            Some(PdfColor::BLUE),
        )?;

        let mut group = PdfPageGroupObject::new(&page, |_| true)?;

        let bounds = group.bounds()?;

        assert_eq!(bounds.bottom.value, 100.0);
        assert_eq!(bounds.left.value, 100.0);
        assert_eq!(bounds.top.value, 300.0);
        assert_eq!(bounds.right.value, 300.0);

        group.translate(PdfPoints::new(150.0), PdfPoints::new(200.0))?;

        let bounds = group.bounds()?;

        assert_eq!(bounds.bottom.value, 300.0);
        assert_eq!(bounds.left.value, 250.0);
        assert_eq!(bounds.top.value, 500.0);
        assert_eq!(bounds.right.value, 450.0);

        Ok(())
    }
}
