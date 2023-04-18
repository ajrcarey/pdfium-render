//! Defines the [PdfBitmap] struct, a bitmap image with a specific width and height.

use crate::bindgen::{
    FPDFBitmap_BGR, FPDFBitmap_BGRA, FPDFBitmap_BGRx, FPDFBitmap_Gray, FPDFBitmap_Unknown,
    FPDF_BITMAP,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use std::f32::consts::{FRAC_PI_2, PI};
use std::os::raw::{c_int, c_void};

#[cfg(feature = "image")]
use image::{DynamicImage, ImageBuffer};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{Clamped, JsValue};

#[cfg(target_arch = "wasm32")]
use web_sys::ImageData;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

// The following dummy declarations are used only when running cargo doc.
// They allow documentation of WASM-specific functionality to be included
// in documentation generated on non-WASM targets.

#[cfg(doc)]
struct Uint8Array;

#[cfg(doc)]
struct ImageData;

#[cfg(doc)]
struct JsValue;

/// The device coordinate system when rendering or displaying an image.
///
/// While Pdfium will accept pixel sizes in either dimension up to the limits of [i32],
/// in practice the maximum size of a bitmap image is limited to approximately 2,320,723,080 bytes
/// (a little over 2 Gb). You can use the [PdfBitmap::bytes_required_for_size()] function
/// to estimate the maximum size of a bitmap image for a given target pixel width and height.
pub type Pixels = c_int;

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
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(Self::from_pdfium(handle, bindings))
        }
    }

    /// Creates a new [PdfBitmap] that wraps the given byte buffer. The buffer must be capable
    /// of storing an image of the given pixel width and height in the given pixel format,
    /// otherwise a buffer overflow may occur during rendering.
    ///
    /// This function is not available when compiling to WASM.
    ///
    /// # Safety
    ///
    /// This function is unsafe because a buffer overflow may occur during rendering if the buffer
    /// is too small to store a rendered image of the given pixel dimensions.
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn from_bytes(
        width: Pixels,
        height: Pixels,
        format: PdfBitmapFormat,
        buffer: &'a mut [u8],
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Result<PdfBitmap<'a>, PdfiumError> {
        let handle = bindings.FPDFBitmap_CreateEx(
            width as c_int,
            height as c_int,
            format.as_pdfium() as c_int,
            buffer.as_mut_ptr() as *mut c_void,
            0, // Not relevant because Pdfium will compute the stride value itself.
        );

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(Self::from_pdfium(handle, bindings))
        }
    }

    /// Returns the internal `FPDF_BITMAP` handle for this [PdfBitmap].
    #[inline]
    pub(crate) fn handle(&self) -> &FPDF_BITMAP {
        &self.handle
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfBitmap].
    #[inline]
    pub fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
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
    pub fn as_bytes(&self) -> &'a [u8] {
        let buffer_length = self.bindings.FPDFBitmap_GetStride(self.handle)
            * self.bindings.FPDFBitmap_GetHeight(self.handle);

        let buffer_start = self.bindings.FPDFBitmap_GetBuffer(self.handle);

        unsafe { std::slice::from_raw_parts(buffer_start as *const u8, buffer_length as usize) }
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing this [PdfBitmap].
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    pub fn as_image(&self) -> DynamicImage {
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
    pub fn render(&self) {}

    /// Returns a Javascript `Uint8Array` object representing the bitmap buffer backing
    /// this [PdfBitmap].
    ///
    /// This function avoids a memory allocation and copy required by both
    /// [PdfBitmap::as_bytes()] and [PdfBitmap::as_image_data()], making it preferable for
    /// situations where performance is paramount.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub fn as_array(&self) -> Uint8Array {
        self.bindings.FPDFBitmap_GetArray(self.handle)
    }

    /// Returns a new Javascript `ImageData` object created from the bitmap buffer backing
    /// this [PdfBitmap]. The resulting `ImageData` can be easily displayed in an
    /// HTML `<canvas>` element like so:
    ///
    /// `canvas.getContext('2d').putImageData(image_data);`
    ///
    /// This function is slower than calling [PdfBitmap::as_array()] because it must perform
    /// an additional memory allocation in order to create the `ImageData` object. Consider calling
    /// the [PdfBitmap::as_array()] function directly if performance is paramount.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub fn as_image_data(&self) -> Result<ImageData, JsValue> {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(self.as_bytes()),
            self.width() as u32,
            self.height() as u32,
        )
    }

    /// Estimates the maximum memory buffer size required for a [PdfBitmap] of the given dimensions.
    ///
    /// Certain platforms, architectures, and operating systems may limit the maximum size of a
    /// bitmap buffer that can be created by Pdfium.
    ///
    /// The returned value assumes four bytes of memory will be consumed for each rendered pixel.
    #[inline]
    pub fn bytes_required_for_size(width: Pixels, height: Pixels) -> usize {
        4 * width as usize * height as usize
    }
}

impl<'a> Drop for PdfBitmap<'a> {
    /// Closes this [PdfBitmap], releasing the memory held by the bitmap buffer.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDFBitmap_Destroy(self.handle);
    }
}
