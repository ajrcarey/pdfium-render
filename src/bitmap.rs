//! Defines the [PdfBitmap] struct, a lazily-generated bitmap rendering of a single
//! `PdfPage`.

use crate::bindgen::{
    FPDFBitmap_BGR, FPDFBitmap_BGRA, FPDFBitmap_BGRx, FPDFBitmap_Gray, FPDFBitmap_Unknown,
    FPDF_BITMAP,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use image::{DynamicImage, ImageBuffer};
use std::f32::consts::{FRAC_PI_2, PI};
use std::os::raw::c_int;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{Clamped, JsValue};

#[cfg(target_arch = "wasm32")]
use web_sys::ImageData;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

/// The device coordinate system when rendering or displaying an image.
pub type Pixels = u16;

/// The pixel format of the rendered image data in the backing buffer of a [PdfBitmap].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfBitmapFormat {
    Gray = FPDFBitmap_Gray as isize,
    BGR = FPDFBitmap_BGR as isize,
    BRGx = FPDFBitmap_BGRx as isize,
    BGRA = FPDFBitmap_BGRA as isize,
}

impl PdfBitmapFormat {
    #[inline]
    #[allow(non_upper_case_globals)]
    pub(crate) fn from_pdfium(format: u32) -> Result<Self, PdfiumError> {
        match format {
            FPDFBitmap_Unknown => Err(PdfiumError::UnknownBitmapFormat),
            FPDFBitmap_Gray => Ok(PdfBitmapFormat::Gray),
            FPDFBitmap_BGR => Ok(PdfBitmapFormat::BGR),
            FPDFBitmap_BGRx => Ok(PdfBitmapFormat::BRGx),
            FPDFBitmap_BGRA => Ok(PdfBitmapFormat::BGRA),
            _ => Err(PdfiumError::UnknownBitmapFormat),
        }
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> u32 {
        match self {
            PdfBitmapFormat::Gray => FPDFBitmap_Gray,
            PdfBitmapFormat::BGR => FPDFBitmap_BGR,
            PdfBitmapFormat::BRGx => FPDFBitmap_BGRx,
            PdfBitmapFormat::BGRA => FPDFBitmap_BGRA,
        }
    }
}

impl Default for PdfBitmapFormat {
    #[inline]
    fn default() -> Self {
        PdfBitmapFormat::BGRA
    }
}

/// A rotation transformation that should be applied to a `PdfPage` when it is rendered
/// into a [PdfBitmap].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfBitmapRotation {
    None,
    Degrees90,
    Degrees180,
    Degrees270,
}

impl PdfBitmapRotation {
    #[inline]
    pub(crate) fn from_pdfium(rotate: i32) -> Result<Self, PdfiumError> {
        match rotate {
            0 => Ok(PdfBitmapRotation::None),
            1 => Ok(PdfBitmapRotation::Degrees90),
            2 => Ok(PdfBitmapRotation::Degrees180),
            3 => Ok(PdfBitmapRotation::Degrees270),
            _ => Err(PdfiumError::UnknownBitmapRotation),
        }
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> i32 {
        match self {
            PdfBitmapRotation::None => 0,
            PdfBitmapRotation::Degrees90 => 1,
            PdfBitmapRotation::Degrees180 => 2,
            PdfBitmapRotation::Degrees270 => 3,
        }
    }

    #[inline]
    pub const fn as_degrees(&self) -> f32 {
        match self {
            PdfBitmapRotation::None => 0.0,
            PdfBitmapRotation::Degrees90 => 90.0,
            PdfBitmapRotation::Degrees180 => 180.0,
            PdfBitmapRotation::Degrees270 => 270.0,
        }
    }

    pub const DEGREES_90_AS_RADIANS: f32 = FRAC_PI_2;

    pub const DEGREES_180_AS_RADIANS: f32 = PI;

    pub const DEGREES_270_AS_RADIANS: f32 = FRAC_PI_2 + PI;

    #[inline]
    pub const fn as_radians(&self) -> f32 {
        match self {
            PdfBitmapRotation::None => 0.0,
            PdfBitmapRotation::Degrees90 => Self::DEGREES_90_AS_RADIANS,
            PdfBitmapRotation::Degrees180 => Self::DEGREES_180_AS_RADIANS,
            PdfBitmapRotation::Degrees270 => Self::DEGREES_270_AS_RADIANS,
        }
    }

    #[inline]
    pub const fn as_radians_cos(&self) -> f32 {
        match self {
            PdfBitmapRotation::None => 1.0,
            PdfBitmapRotation::Degrees90 => 0.0,
            PdfBitmapRotation::Degrees180 => -1.0,
            PdfBitmapRotation::Degrees270 => 0.0,
        }
    }

    #[inline]
    pub const fn as_radians_sin(&self) -> f32 {
        match self {
            PdfBitmapRotation::None => 0.0,
            PdfBitmapRotation::Degrees90 => 1.0,
            PdfBitmapRotation::Degrees180 => 0.0,
            PdfBitmapRotation::Degrees270 => -1.0,
        }
    }
}

/// A bitmap image with a specific width and height.
pub struct PdfBitmap<'a> {
    handle: FPDF_BITMAP,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBitmap<'a> {
    /// Wraps an existing `FPDF_BITMAP` handle inside a new [PdfBitmap].
    pub(crate) fn from_pdfium(
        handle: FPDF_BITMAP,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfBitmap { handle, bindings }
    }

    /// Creates an empty [PdfBitmap] with a buffer capable of storing an image of the given
    /// pixel width and height in the given pixel format.
    pub fn empty(
        width: Pixels,
        height: Pixels,
        format: PdfBitmapFormat,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Result<PdfBitmap, PdfiumError> {
        let handle = bindings.FPDFBitmap_CreateEx(
            width as c_int,
            height as c_int,
            format.as_pdfium() as c_int,
            std::ptr::null_mut(),
            0, // Not relevant because Pdfium will create the buffer itself.
        );

        if handle.is_null() {
            if let Some(error) = bindings.get_pdfium_last_error() {
                Err(PdfiumError::PdfiumLibraryInternalError(error))
            } else {
                // This would be an unusual situation; a null handle indicating failure,
                // yet Pdfium's error code indicates success.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        } else {
            Ok(Self::from_pdfium(handle, bindings))
        }
    }

    /// Returns the internal `FPDF_BITMAP` handle for this [PdfBitmap].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_BITMAP {
        &self.handle
    }

    /// Returns the width of the image in the bitmap buffer backing this [PdfBitmap].
    #[inline]
    pub fn width(&self) -> Pixels {
        self.bindings.FPDFBitmap_GetWidth(self.handle) as Pixels
    }

    /// Returns the height of the image in the bitmap buffer backing this [PdfBitmap].
    #[inline]
    pub fn height(&self) -> Pixels {
        self.bindings.FPDFBitmap_GetHeight(self.handle) as Pixels
    }

    /// Returns the pixel format of the image in the bitmap buffer backing this [PdfBitmap].
    #[inline]
    pub fn format(&self) -> Result<PdfBitmapFormat, PdfiumError> {
        PdfBitmapFormat::from_pdfium(self.bindings.FPDFBitmap_GetFormat(self.handle) as u32)
    }

    /// Returns an immutable reference to the bitmap buffer backing this [PdfBitmap].
    pub fn as_bytes(&mut self) -> &'a [u8] {
        let buffer_length = self.bindings.FPDFBitmap_GetStride(self.handle)
            * self.bindings.FPDFBitmap_GetHeight(self.handle);

        let buffer_start = self.bindings.FPDFBitmap_GetBuffer(self.handle);

        unsafe { std::slice::from_raw_parts(buffer_start as *const u8, buffer_length as usize) }
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing this [PdfBitmap].
    pub fn as_image(&mut self) -> DynamicImage {
        ImageBuffer::from_raw(
            self.width() as u32,
            self.height() as u32,
            self.as_bytes().to_owned(),
        )
        .map(DynamicImage::ImageRgba8)
        .unwrap()
    }

    // TODO: AJRC - 29/7/22 - remove deprecated PdfBitmap::render() function in 0.9.0
    // as part of tracking issue https://github.com/ajrcarey/pdfium-render/issues/36
    /// Prior to 0.7.12, this function rendered the referenced page into a bitmap buffer.
    ///
    /// This is no longer necessary since all page rendering operations are now processed eagerly
    /// rather than lazily.
    ///
    /// This function is now deprecated and will be removed in release 0.9.0.
    #[deprecated(
        since = "0.7.12",
        note = "This function is no longer necessary since all page rendering operations are now processed eagerly rather than lazily. Calls to this function can be removed."
    )]
    #[doc(hidden)]
    #[inline]
    pub fn render(&mut self) {}

    /// Returns a Javascript `Uint8Array` object representing the bitmap buffer backing
    /// this [PdfBitmap].
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(target_arch = "wasm32")]
    pub fn as_array(&mut self) -> Uint8Array {
        self.bindings.FPDFBitmap_GetArray(self.handle)
    }

    /// Returns a new Javascript `ImageData` object created from the bitmap buffer backing
    /// this [PdfBitmap]. The resulting `ImageData` can be easily displayed in an
    /// HTML <canvas> element like so:
    ///
    /// `canvas.getContext('2d').putImageData(image_data);`
    ///
    /// This function is slower than calling [PdfBitmap::as_array()] because it must perform
    /// an additional memory allocation in order to create the `ImageData` object. Consider calling
    /// the [PdfBitmap::as_array()] function directly if performance is paramount.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(target_arch = "wasm32")]
    pub fn as_image_data(&mut self) -> Result<ImageData, JsValue> {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(self.as_bytes()),
            self.width() as u32,
            self.height() as u32,
        )
    }
}

impl<'a> Drop for PdfBitmap<'a> {
    /// Closes this [PdfBitmap], releasing the memory held by the bitmap buffer.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDFBitmap_Destroy(self.handle);
    }
}
