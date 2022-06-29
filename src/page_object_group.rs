//! Defines the [PdfPageGroupObject] struct, exposing functionality related to a group of
//! page objects contained in the same `PdfPageObjects` collection.

use crate::bindgen::{FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::color::PdfColor;
use crate::error::PdfiumError;
use crate::page::{PdfPage, PdfPageContentRegenerationStrategy, PdfPoints, PdfRect};
use crate::page_object::{
    PdfPageObject, PdfPageObjectBlendMode, PdfPageObjectCommon, PdfPageObjectLineCap,
    PdfPageObjectLineJoin,
};
use crate::page_object_path::PdfPathFillMode;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_objects::PdfPageObjectIndex;

/// A group of [PdfPageObject] objects contained in the same `PdfPageObjects` collection.
/// The page objects contained in the group can be manipulated and transformed together
/// as if they were a single object.
///
/// Groups are bound to specific pages in the document. To create an empty group, use either the
/// `PdfPageObjects::create_new_group()` function or the [PdfPageGroupObject::empty()] function.
/// To create a populated group, use one of the [PdfPageGroupObject::new()],
/// [PdfPageGroupObject::from_vec()], or [PdfPageGroupObject::from_slice()] functions.
pub struct PdfPageGroupObject<'a> {
    object_handles: Vec<FPDF_PAGEOBJECT>,
    page: FPDF_PAGE,
    bindings: &'a dyn PdfiumLibraryBindings,
    do_regenerate_page_content_after_each_change: bool,
}

impl<'a> PdfPageGroupObject<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        page: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
        do_regenerate_page_content_after_each_change: bool,
    ) -> Self {
        PdfPageGroupObject {
            object_handles: Vec::new(),
            page,
            bindings,
            do_regenerate_page_content_after_each_change,
        }
    }

    /// Creates a new, empty [PdfPageGroupObject] that can be used to hold any page objects
    /// on the given [PdfPage].
    pub fn empty(page: &'a PdfPage<'a>) -> Self {
        Self::from_pdfium(
            *page.get_handle(),
            page.get_bindings(),
            page.content_regeneration_strategy()
                == PdfPageContentRegenerationStrategy::AutomaticOnEveryChange,
        )
    }

    /// Creates a new [PdfPageGroupObject] that includes any page objects on the given [PdfPage]
    /// matching the given predicate function.
    pub fn new<F>(page: &'a PdfPage<'a>, predicate: F) -> Result<Self, PdfiumError>
    where
        F: FnMut(&PdfPageObject) -> bool,
    {
        let mut result = Self::from_pdfium(
            *page.get_handle(),
            page.get_bindings(),
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
        page: &'a PdfPage<'a>,
        mut objects: Vec<PdfPageObject<'a>>,
    ) -> Result<Self, PdfiumError> {
        Self::from_slice(page, objects.as_mut_slice())
    }

    /// Creates a new [PdfPageGroupObject] that includes the given page objects on the
    /// given [PdfPage].
    pub fn from_slice(
        page: &'a PdfPage<'a>,
        objects: &mut [PdfPageObject<'a>],
    ) -> Result<Self, PdfiumError> {
        let mut result = Self::from_pdfium(
            *page.get_handle(),
            page.get_bindings(),
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
        self.object_handles.contains(object.get_object_handle())
    }

    /// Adds a single [PdfPageObject] to this group.
    pub fn push(&mut self, object: &mut PdfPageObject<'a>) -> Result<(), PdfiumError> {
        let was_object_already_attached_to_group_page =
            if let Some(page_handle) = object.get_page_handle() {
                if *page_handle != self.page {
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

                object.add_object_to_page_handle(self.page)?;

                false
            };

        self.object_handles.push(*object.get_object_handle());

        if !was_object_already_attached_to_group_page
            && self.do_regenerate_page_content_after_each_change
        {
            PdfPage::regenerate_content_immut_for_handle(self.page, self.bindings)?;
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
            PdfPage::regenerate_content_immut_for_handle(self.page, self.bindings)?;
        }

        Ok(())
    }

    /// Removes every [PdfPageObject] in this group from the group's containing [PdfPage].
    ///
    /// Each object's memory ownership will be removed from the `PdfPageObjects` collection for
    /// this group's containing [PdfPage]. The objects will also be removed from this group,
    /// and the memory owned by each object will be freed. The group will be empty at the end of
    /// this operation.
    ///
    /// If the containing [PdfPage] has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    #[inline]
    pub fn remove_objects_from_page(&mut self) -> Result<(), PdfiumError> {
        // Hold off regenerating page content until all objects have been processed.

        let do_regenerate_page_content_after_each_change =
            self.do_regenerate_page_content_after_each_change;

        self.do_regenerate_page_content_after_each_change = false;

        self.apply_to_each(|object| object.remove_object_from_page())
            .map(|_| self.object_handles.clear())?;

        // Regenerate page content now, if necessary.

        self.do_regenerate_page_content_after_each_change =
            do_regenerate_page_content_after_each_change;

        if self.do_regenerate_page_content_after_each_change {
            PdfPage::regenerate_content_immut_for_handle(self.page, self.bindings)?;
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

    /// Returns an iterator over all the [PdfPageObject] objects in this group.
    #[inline]
    pub fn iter(&self) -> PdfPageGroupObjectIterator {
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
            PdfPageObject::from_pdfium(*object_handle, self.page, self.bindings).has_transparency()
        })
    }

    /// Returns the bounding box of this group of objects. Since the bounds of every object in the
    /// group must be considered, this function has runtime complexity of O(n).
    pub fn bounds(&self) -> Result<PdfRect, PdfiumError> {
        let mut bounds: Option<PdfRect> = None;

        self.object_handles.iter().for_each(|object_handle| {
            if let Ok(object_bounds) =
                PdfPageObject::from_pdfium(*object_handle, self.page, self.bindings).bounds()
            {
                if let Some(bounds) = bounds.as_mut() {
                    if object_bounds.bottom < bounds.bottom {
                        bounds.bottom = object_bounds.bottom;
                    }

                    if object_bounds.left < bounds.left {
                        bounds.left = object_bounds.left;
                    }

                    if object_bounds.top > bounds.top {
                        bounds.top = object_bounds.top;
                    }

                    if object_bounds.right > bounds.right {
                        bounds.right = object_bounds.right;
                    }
                } else {
                    bounds = Some(object_bounds);
                }
            }
        });

        bounds.ok_or(PdfiumError::EmptyPageObjectGroup)
    }

    /// Applies the given transformation, expressed as six values representing the six configurable
    /// elements of a nine-element 3x3 PDF transformation matrix, to every [PdfPageObject] in this group.
    ///
    /// To move, scale, rotate, or skew the page objects in this group, consider using one or more of the
    /// following functions. Internally they all use [PdfPageGroupObject::transform()], but are
    /// probably easier to use (and certainly clearer in their intent) in most situations.
    ///
    /// * [PdfPageGroupObject::translate()]: changes the position of every [PdfPageObject] in this group.
    /// * [PdfPageGroupObject::scale()]: changes the size of every [PdfPageObject] in this group.
    /// * [PdfPageGroupObject::rotate_clockwise_degrees()], [PdfPageGroupObject::rotate_counter_clockwise_degrees()],
    /// [PdfPageGroupObject::rotate_clockwise_radians()], [PdfPageGroupObject::rotate_counter_clockwise_radians()]:
    /// rotates every [PdfPageObject] in this group around its origin.
    /// * [PdfPageGroupObject::skew_degrees()], [PdfPageGroupObject::skew_radians()]: skews every
    /// [PdfPageObject] in this group relative to its axes.
    ///
    /// **The order in which transformations are applied to a page object is significant.**
    /// For example, the result of rotating _then_ translating a page object may be vastly different
    /// from translating _then_ rotating the same page object.
    ///
    /// An overview of PDF transformation matrices can be found in the PDF Reference Manual
    /// version 1.7 on page 204; a detailed description can be founded in section 4.2.3 on page 207.
    #[inline]
    pub fn transform(
        &mut self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.transform(a, b, c, d, e, f))
    }

    /// Moves the origin of every [PdfPageObject] in this group by the given horizontal and vertical
    /// delta distances.
    #[inline]
    pub fn translate(&mut self, delta_x: PdfPoints, delta_y: PdfPoints) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.translate(delta_x, delta_y))
    }

    /// Changes the size of every [PdfPageObject] in this group, scaling them by the given
    /// horizontal and vertical scale factors.
    #[inline]
    pub fn scale(
        &mut self,
        horizontal_scale_factor: f64,
        vertical_scale_factor: f64,
    ) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.scale(horizontal_scale_factor, vertical_scale_factor))
    }

    /// Rotates every [PdfPageObject] in this group counter-clockwise by the given number of degrees.
    #[inline]
    pub fn rotate_counter_clockwise_degrees(&mut self, degrees: f32) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.rotate_counter_clockwise_degrees(degrees))
    }

    /// Rotates every [PdfPageObject] in this group clockwise by the given number of degrees.
    #[inline]
    pub fn rotate_clockwise_degrees(&mut self, degrees: f32) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.rotate_clockwise_degrees(degrees))
    }

    /// Rotates every [PdfPageObject] in this group counter-clockwise by the given number of radians.
    #[inline]
    pub fn rotate_counter_clockwise_radians(&mut self, radians: f32) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.rotate_counter_clockwise_radians(radians))
    }

    /// Rotates every [PdfPageObject] in this group clockwise by the given number of radians.
    #[inline]
    pub fn rotate_clockwise_radians(&mut self, radians: f32) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.rotate_clockwise_radians(radians))
    }

    /// Skews the axes of every [PdfPageObject] in this group by the given angles in degrees.
    #[inline]
    pub fn skew_degrees(&mut self, x_axis_skew: f32, y_axis_skew: f32) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.skew_degrees(x_axis_skew, y_axis_skew))
    }

    /// Skews the axes of every [PdfPageObject] in this group by the given angles in radians.
    #[inline]
    pub fn skew_radians(&mut self, x_axis_skew: f32, y_axis_skew: f32) -> Result<(), PdfiumError> {
        self.apply_to_each(|object| object.skew_radians(x_axis_skew, y_axis_skew))
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
        PdfPageObject::from_pdfium(*handle, self.page, self.bindings)
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
    use crate::page_object_group::PdfPageGroupObject;
    use crate::prelude::*;

    #[test]
    fn test_group_bounds() -> Result<(), PdfiumError> {
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        let document = pdfium.load_pdf_from_file("./test/export-test.pdf", None)?;

        // Form a group of all text objects in the top half of the first page of music ...

        let page = document.pages().get(2)?;

        let mut group = page.objects().create_empty_group();

        group.append(
            page.objects()
                .iter()
                .filter(|object| {
                    object.object_type() == PdfPageObjectType::Text
                        && object.bounds().unwrap().bottom > page.height() / 2.0
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
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        let document = pdfium.load_pdf_from_file("./test/export-test.pdf", None)?;

        // Form a group of all text objects in the bottom half of the last page of music ...

        let page = document.pages().get(5)?;

        let mut group = page.objects().create_empty_group();

        group.append(
            page.objects()
                .iter()
                .filter(|object| {
                    object.object_type() == PdfPageObjectType::Text
                        && object.bounds().unwrap().bottom < page.height() / 2.0
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

        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        let document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages()
            .create_page_at_start(PdfPagePaperSize::a4())?;

        page.objects_mut().create_path_object_rect(
            PdfRect::new_from_values(100.0, 100.0, 200.0, 200.0),
            None,
            None,
            Some(PdfColor::SOLID_RED),
        )?;

        page.objects_mut().create_path_object_rect(
            PdfRect::new_from_values(150.0, 150.0, 250.0, 250.0),
            None,
            None,
            Some(PdfColor::SOLID_GREEN),
        )?;

        page.objects_mut().create_path_object_rect(
            PdfRect::new_from_values(200.0, 200.0, 300.0, 300.0),
            None,
            None,
            Some(PdfColor::SOLID_BLUE),
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
