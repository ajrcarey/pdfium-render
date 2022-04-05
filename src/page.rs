//! Defines the [PdfPage] struct, exposing functionality related to a single page in a
//! `PdfPages` collection.

use crate::bindgen::{FPDFBitmap_BGRA, FPDF_ANNOT, FPDF_BITMAP, FPDF_BOOL, FPDF_PAGE, FS_RECTF};
use crate::bindings::PdfiumLibraryBindings;
use crate::bitmap::{PdfBitmap, PdfBitmapRotation};
use crate::bitmap_config::{PdfBitmapConfig, PdfBitmapRenderSettings};
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page_boundaries::PdfPageBoundaries;
use crate::page_objects::PdfPageObjects;
use crate::page_size::PdfPagePaperSize;
use crate::page_text::PdfPageText;
use crate::pages::PdfPageIndex;
use crate::prelude::PdfPageAnnotations;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::ptr::null_mut;

/// The internal coordinate system inside a [PdfDocument] is measured in Points, a
/// device-independent unit equal to 1/72 inches, roughly 0.358 mm. Points are converted to pixels
/// when a [PdfPage] is rendered to a [PdfBitmap].
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PdfPoints {
    pub value: f32,
}

impl PdfPoints {
    pub const ZERO: PdfPoints = PdfPoints::new(0.0);

    /// Creates a new [PdfPoints] object with the given value.
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    /// Creates a new [PdfPoints] object from the given measurement in inches.
    #[inline]
    pub fn from_inches(inches: f32) -> Self {
        Self::new(inches * 72.0)
    }

    /// Creates a new [PdfPoints] object from the given measurement in centimeters.
    #[inline]
    pub fn from_cm(cm: f32) -> Self {
        Self::from_inches(cm / 2.54)
    }

    /// Creates a new [PdfPoints] object from the given measurement in millimeters.
    #[inline]
    pub fn from_mm(mm: f32) -> Self {
        Self::from_cm(mm / 10.0)
    }

    /// Converts the value of this [PdfPoints] object to inches.
    #[inline]
    pub fn to_inches(&self) -> f32 {
        self.value / 72.0
    }

    /// Converts the value of this [PdfPoints] object to centimeters.
    #[inline]
    pub fn to_cm(&self) -> f32 {
        self.to_inches() * 2.54
    }

    /// Converts the value of this [PdfPoints] object to millimeters.
    #[inline]
    pub fn to_mm(self) -> f32 {
        self.to_cm() * 10.0
    }
}

/// A rectangle measured in [PdfPoints].
///
/// The coordinate space of a [PdfPage] has its origin (0,0) at the bottom left of the page,
/// with x values increasing as coordinates move horizontally to the right and
/// y values increasing as coordinates move vertically up.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PdfRect {
    pub bottom: PdfPoints,
    pub left: PdfPoints,
    pub top: PdfPoints,
    pub right: PdfPoints,
}

impl PdfRect {
    #[inline]
    pub(crate) fn from_pdfium(rect: FS_RECTF) -> Self {
        Self {
            bottom: PdfPoints::new(rect.bottom),
            left: PdfPoints::new(rect.left),
            top: PdfPoints::new(rect.top),
            right: PdfPoints::new(rect.right),
        }
    }

    #[inline]
    pub(crate) fn from_pdfium_as_result(
        result: FPDF_BOOL,
        rect: FS_RECTF,
        bindings: &dyn PdfiumLibraryBindings,
    ) -> Result<PdfRect, PdfiumError> {
        if result == 0 {
            if let Some(error) = bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(PdfRect::from_pdfium(rect))
        }
    }

    /// Creates a new [PdfRect] from the given [PdfPoints] measurements.
    ///
    /// The coordinate space of a [PdfPage] has its origin (0,0) at the bottom left of the page,
    /// with x values increasing as coordinates move horizontally to the right and
    /// y values increasing as coordinates move vertically up.
    #[inline]
    pub fn new(bottom: PdfPoints, left: PdfPoints, top: PdfPoints, right: PdfPoints) -> Self {
        Self {
            bottom,
            left,
            top,
            right,
        }
    }

    /// Creates a new [PdfRect] from the given raw points values.
    ///
    /// The coordinate space of a [PdfPage] has its origin (0,0) at the bottom left of the page,
    /// with x values increasing as coordinates move horizontally to the right and
    /// y values increasing as coordinates move vertically up.
    #[inline]
    pub fn new_from_values(bottom: f32, left: f32, top: f32, right: f32) -> Self {
        Self::new(
            PdfPoints::new(bottom),
            PdfPoints::new(left),
            PdfPoints::new(top),
            PdfPoints::new(right),
        )
    }
}

/// The orientation of a [PdfPage].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfPageOrientation {
    Portrait,
    Landscape,
}

impl PdfPageOrientation {
    #[inline]
    pub(crate) fn from_width_and_height(width: PdfPoints, height: PdfPoints) -> Self {
        if width.value > height.value {
            PdfPageOrientation::Landscape
        } else {
            PdfPageOrientation::Portrait
        }
    }
}

/// A single page in a [PdfDocument].
///
/// In addition to its own intrinsic properties, a [PdfPage] serves as the entry point
/// to all object collections related to a single page in a PDF file.
/// These collections include:
/// * [PdfPage::annotations()], all the user annotations attached to the [PdfPage].
/// * [PdfPage::boundaries()], all the boundary boxes relating to the [PdfPage].
/// * [PdfPage::objects()], all the displayable objects on the [PdfPage].
pub struct PdfPage<'a> {
    index: PdfPageIndex,
    handle: FPDF_PAGE,
    document: &'a PdfDocument<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPage<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        index: PdfPageIndex,
        handle: FPDF_PAGE,
        document: &'a PdfDocument<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPage {
            index,
            handle,
            document,
            bindings,
        }
    }

    /// Returns the internal FPDF_PAGE handle for this [PdfPage].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_PAGE {
        &self.handle
    }

    /// Returns the zero-based page index of this [PdfPage] in its containing [PdfDocument].
    #[inline]
    pub fn index(&self) -> PdfPageIndex {
        self.index
    }

    /// Returns the label assigned to this [PdfPage], if any.
    #[inline]
    pub fn label(&self) -> Option<String> {
        // Retrieving the label text from Pdfium is a two-step operation. First, we call
        // FPDF_GetPageLabel() with a null buffer; this will retrieve the length of
        // the label text in bytes. If the length is zero, then there is no such tag.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDF_GetPageLabel() again with a pointer to the buffer;
        // this will write the label text to the buffer in UTF16LE format.

        let buffer_length = self.bindings.FPDF_GetPageLabel(
            *self.document.get_handle(),
            self.index as c_int,
            std::ptr::null_mut(),
            0,
        );

        if buffer_length == 0 {
            // The label is not present.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDF_GetPageLabel(
            *self.document.get_handle(),
            self.index as c_int,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer)
    }

    /// Returns the width of this [PdfPage] in device-independent points.
    /// One point is 1/72 inches, roughly 0.358 mm.
    pub fn width(&self) -> PdfPoints {
        PdfPoints::new(self.bindings.FPDF_GetPageWidthF(self.handle))
    }

    /// Returns the height of this [PdfPage] in device-independent points.
    /// One point is 1/72 inches, roughly 0.358 mm.
    pub fn height(&self) -> PdfPoints {
        PdfPoints::new(self.bindings.FPDF_GetPageHeightF(self.handle))
    }

    /// Returns the width and height of this [PdfPage] expressed as a [PdfRect].
    #[inline]
    pub fn page_size(&self) -> PdfRect {
        PdfRect::new(
            PdfPoints::ZERO,
            PdfPoints::ZERO,
            self.height(),
            self.width(),
        )
    }

    /// Returns [PdfPageOrientation::Landscape] if the width of this [PdfPage]
    /// is greater than its height; otherwise returns [PdfPageOrientation::Portrait].
    #[inline]
    pub fn orientation(&self) -> PdfPageOrientation {
        PdfPageOrientation::from_width_and_height(self.width(), self.height())
    }

    /// Returns `true` if this [PdfPage] has orientation [PdfPageOrientation::Portrait].
    #[inline]
    pub fn is_portrait(&self) -> bool {
        self.orientation() == PdfPageOrientation::Portrait
    }

    /// Returns `true` if this [PdfPage] has orientation [PdfPageOrientation::Landscape].
    #[inline]
    pub fn is_landscape(&self) -> bool {
        self.orientation() == PdfPageOrientation::Landscape
    }

    /// Returns any intrinsic rotation encoded into this document indicating a rotation
    /// should be applied to this [PdfPage] during rendering.
    #[inline]
    pub fn rotation(&self) -> Result<PdfBitmapRotation, PdfiumError> {
        PdfBitmapRotation::from_pdfium(self.bindings.FPDFPage_GetRotation(self.handle))
    }

    /// Sets the intrinsic rotation that should be applied to this [PdfPage] during rendering.
    #[inline]
    pub fn set_rotation(&mut self, rotation: PdfBitmapRotation) {
        self.bindings
            .FPDFPage_SetRotation(self.handle, rotation.as_pdfium());
    }

    /// Returns `true` if any object on the page contains transparency.
    #[inline]
    pub fn has_transparency(&self) -> bool {
        self.bindings
            .is_true(self.bindings.FPDFPage_HasTransparency(self.handle))
    }

    /// Returns the collection of bounding boxes defining the extents of this [PdfPage].
    #[inline]
    pub fn boundaries(&self) -> PdfPageBoundaries {
        PdfPageBoundaries::from_pdfium(self, self.bindings)
    }

    /// Returns the paper size of this [PdfPage].
    #[inline]
    pub fn paper_size(&self) -> PdfPagePaperSize {
        PdfPagePaperSize::from_points(self.width(), self.height())
    }

    /// Returns the collection of text boxes contained within this [PdfPage].
    pub fn text(&self) -> Result<PdfPageText, PdfiumError> {
        let text_handle = self.bindings.FPDFText_LoadPage(self.handle);

        if text_handle.is_null() {
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
            Ok(PdfPageText::from_pdfium(text_handle, self, self.bindings))
        }
    }

    /// Returns the collection of page objects contained within this [PdfPage].
    #[inline]
    pub fn objects(&self) -> PdfPageObjects {
        PdfPageObjects::from_pdfium(self, self.bindings)
    }

    /// Returns the collection of annotations that have been added to this [PdfPage].
    #[inline]
    pub fn annotations(&self) -> PdfPageAnnotations {
        PdfPageAnnotations::from_pdfium(self, self.bindings)
    }

    /// Returns a [PdfBitmap] using pixel dimensions, rotation settings, and rendering options
    /// configured in the given [PdfBitmapConfig].
    ///
    /// See also [PdfPage::get_bitmap()], which directly creates a [PdfBitmap] object from this page
    /// using caller-specified pixel dimensions and page rotation settings.
    #[inline]
    pub fn get_bitmap_with_config(
        &self,
        config: &PdfBitmapConfig,
    ) -> Result<PdfBitmap, PdfiumError> {
        let config = config.apply_to_page(self);

        let handle = self.create_empty_bitmap_handle(config.width, config.height, config.format)?;

        Ok(PdfBitmap::from_pdfium(
            handle,
            config,
            self,
            self.document,
            self.bindings,
        ))
    }

    /// Returns a [PdfBitmap] with the given pixel dimensions and render-time rotation
    /// for this PdfPage containing the rendered content of this [PdfPage].
    ///
    /// It is the responsibility of the caller to ensure the given pixel width and height
    /// correctly maintain the page's aspect ratio.
    ///
    /// See also [PdfPage::get_bitmap_with_config()], which calculates the correct pixel dimensions,
    /// rotation settings, and rendering options to apply from a [PdfBitmapConfig] object.
    pub fn get_bitmap(
        &self,
        width: u16,
        height: u16,
        rotation: Option<PdfBitmapRotation>,
    ) -> Result<PdfBitmap, PdfiumError> {
        let handle =
            self.create_empty_bitmap_handle(width as i32, height as i32, FPDFBitmap_BGRA as i32)?;

        Ok(PdfBitmap::from_pdfium(
            handle,
            PdfBitmapRenderSettings {
                width: width as i32,
                height: height as i32,
                format: FPDFBitmap_BGRA as i32,
                rotate: rotation.unwrap_or(PdfBitmapRotation::None).as_pdfium(),
                do_render_form_data: true,
                form_field_highlight: vec![],
                render_flags: FPDF_ANNOT as i32,
            },
            self,
            self.document,
            self.bindings,
        ))
    }

    /// Returns a raw FPDF_BITMAP handle to an empty bitmap with the given width and height.
    fn create_empty_bitmap_handle(
        &self,
        width: i32,
        height: i32,
        format: i32,
    ) -> Result<FPDF_BITMAP, PdfiumError> {
        let handle = self.bindings.FPDFBitmap_CreateEx(
            width,
            height,
            format,
            null_mut(),
            0, // Not relevant because Pdfium will create the buffer itself.
        );

        if handle.is_null() {
            if let Some(error) = self.bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(handle)
        }
    }
}

impl<'a> Drop for PdfPage<'a> {
    /// Closes the [PdfPage], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_ClosePage(self.handle);
    }
}
