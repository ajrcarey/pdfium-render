use crate::bindgen::{FPDFBitmap_BGRA, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::bitmap::{PdfBitmap, PdfBitmapFormat, PdfBitmapRotation};
use crate::bitmap_config::PdfBitmapConfig;
use crate::{PdfPageIndex, PdfPoints, PdfiumError, PdfiumInternalError};

/// The orientation of a PdfPage.
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

/// A single page in a PdfDocument.
pub struct PdfPage<'a> {
    index: PdfPageIndex,
    handle: FPDF_PAGE,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPage<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        index: PdfPageIndex,
        handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPage {
            index,
            handle,
            bindings,
        }
    }

    /// Returns the zero-based page index of this PdfPage in its containing PdfDocument.
    #[inline]
    pub fn index(&self) -> PdfPageIndex {
        self.index
    }

    /// Returns the width of this PdfPage in points.
    pub fn width(&self) -> PdfPoints {
        self.bindings.FPDF_GetPageWidthF(self.handle)
    }

    /// Returns the height of this PdfPage in points.
    pub fn height(&self) -> PdfPoints {
        self.bindings.FPDF_GetPageHeightF(self.handle)
    }

    /// Returns PdfPageOrientation::Landscape if the width of this PdfPage
    /// is greater than its height; otherwise returns PdfPageOrientation::Portrait.
    #[inline]
    pub fn orientation(&self) -> PdfPageOrientation {
        PdfPageOrientation::from_width_and_height(self.width(), self.height())
    }

    /// Returns true if this PdfPage has orientation PdfPageOrientation::Portait.
    #[inline]
    pub fn is_portrait(&self) -> bool {
        self.orientation() == PdfPageOrientation::Portrait
    }

    /// Returns true if this PdfPage has orientation PdfPageOrientation::Landscape.
    #[inline]
    pub fn is_landscape(&self) -> bool {
        self.orientation() == PdfPageOrientation::Landscape
    }

    /// Returns a PdfBitmap using sizing and rotation settings configured in the given
    /// PdfBitmapConfig.
    ///
    /// See also [get_bitmap()], which directly creates a PdfBitmap object from this page
    /// using caller-specified pixel dimensions and page rotation settings.
    #[inline]
    pub fn get_bitmap_with_config(
        &self,
        config: &PdfBitmapConfig,
    ) -> Result<PdfBitmap, PdfiumError> {
        let (width, height, rotation) = config.apply_to_page(self);

        self.get_bitmap(width, height, rotation)
    }

    /// Returns a PdfBitmap with the given pixel dimensions and render-time rotation
    /// for this PdfPage containing the rendered content of this PdfPage.
    ///
    /// It is the responsibility of the caller to ensure the given pixel width and height
    /// correctly maintain the page's aspect ratio.
    ///
    /// See also [get_bitmap_with_config()], which calculates the correct pixel dimensions
    /// and rotation settings to use from a PdfBitmapConfig object.
    pub fn get_bitmap(
        &self,
        width: u16,
        height: u16,
        rotation: Option<PdfBitmapRotation>,
    ) -> Result<PdfBitmap, PdfiumError> {
        let handle = self.bindings.FPDFBitmap_CreateEx(
            width as i32,
            height as i32,
            FPDFBitmap_BGRA as i32,
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
            Ok(PdfBitmap::from_pdfium(
                width,
                height,
                PdfBitmapFormat::from_pdfium(FPDFBitmap_BGRA)?,
                rotation.unwrap_or(PdfBitmapRotation::None),
                handle,
                &self.handle,
                self.bindings,
            ))
        }
    }
}

impl<'a> Drop for PdfPage<'a> {
    /// Closes the PdfPage, releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDF_ClosePage(self.handle);
    }
}
