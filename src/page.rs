//! Defines the [PdfPage] struct, exposing functionality related to a single page in a
//! `PdfPages` collection.

use crate::bindgen::{FPDFBitmap_BGRA, FPDF_ANNOT, FPDF_BITMAP, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::bitmap::{PdfBitmap, PdfBitmapRotation};
use crate::bitmap_config::{PdfBitmapConfig, PdfBitmapRenderSettings};
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pages::PdfPageIndex;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::ffi::c_void;
use std::os::raw::c_int;

pub type PdfPoints = f32;

/// The orientation of a [PdfPage].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfPageOrientation {
    Portrait,
    Landscape,
}

impl PdfPageOrientation {
    #[inline]
    pub(crate) fn from_width_and_height(width: PdfPoints, height: PdfPoints) -> Self {
        if width > height {
            PdfPageOrientation::Landscape
        } else {
            PdfPageOrientation::Portrait
        }
    }
}

/// A single page in a [PdfDocument].
pub struct PdfPage<'a> {
    index: PdfPageIndex,
    page_handle: FPDF_PAGE,
    document: &'a PdfDocument<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPage<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        index: PdfPageIndex,
        page_handle: FPDF_PAGE,
        document: &'a PdfDocument<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPage {
            index,
            page_handle,
            document,
            bindings,
        }
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

    /// Returns the width of this [PdfPage] in points.
    pub fn width(&self) -> PdfPoints {
        self.bindings.FPDF_GetPageWidthF(self.page_handle)
    }

    /// Returns the height of this [PdfPage] in points.
    pub fn height(&self) -> PdfPoints {
        self.bindings.FPDF_GetPageHeightF(self.page_handle)
    }

    /// Returns [PdfPageOrientation::Landscape] if the width of this [PdfPage]
    /// is greater than its height; otherwise returns [PdfPageOrientation::Portrait].
    #[inline]
    pub fn orientation(&self) -> PdfPageOrientation {
        PdfPageOrientation::from_width_and_height(self.width(), self.height())
    }

    /// Returns true if this [PdfPage] has orientation [PdfPageOrientation::Portrait].
    #[inline]
    pub fn is_portrait(&self) -> bool {
        self.orientation() == PdfPageOrientation::Portrait
    }

    /// Returns true if this [PdfPage] has orientation [PdfPageOrientation::Landscape].
    #[inline]
    pub fn is_landscape(&self) -> bool {
        self.orientation() == PdfPageOrientation::Landscape
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

        let bitmap_handle =
            self.create_empty_bitmap_handle(config.width, config.height, config.format)?;

        Ok(PdfBitmap::from_pdfium(
            config,
            bitmap_handle,
            self.document.form().map(|form| form.get_handle()),
            &self.page_handle,
            self.bindings,
        ))
    }

    /// Returns a PdfBitmap with the given pixel dimensions and render-time rotation
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
        let bitmap_handle =
            self.create_empty_bitmap_handle(width as i32, height as i32, FPDFBitmap_BGRA as i32)?;

        Ok(PdfBitmap::from_pdfium(
            PdfBitmapRenderSettings {
                width: width as i32,
                height: height as i32,
                format: FPDFBitmap_BGRA as i32,
                rotate: rotation.unwrap_or(PdfBitmapRotation::None).as_pdfium(),
                do_render_form_data: true,
                form_field_highlight: vec![],
                render_flags: FPDF_ANNOT as i32,
            },
            bitmap_handle,
            self.document.form().map(|form| form.get_handle()),
            &self.page_handle,
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
            std::ptr::null_mut(),
            0, // Not relevant because pdfium will create the buffer itself.
        );

        if handle.is_null() {
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
            Ok(handle)
        }
    }
}

impl<'a> Drop for PdfPage<'a> {
    /// Closes the [PdfPage], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_ClosePage(self.page_handle);
    }
}
