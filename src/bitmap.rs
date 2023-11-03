//! Defines the [PdfBitmap] struct, a bitmap image with a specific width and height.

use crate::bindgen::{
    FPDFBitmap_BGR, FPDFBitmap_BGRA, FPDFBitmap_BGRx, FPDFBitmap_Gray, FPDFBitmap_Unknown,
    FPDF_BITMAP,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
#[cfg(feature = "image")]
use crate::utils::pixels::{aligned_bgr_to_rgba, bgra_to_rgba};
use std::os::raw::c_int;

#[cfg(not(target_arch = "wasm32"))]
use std::os::raw::c_void;

#[cfg(feature = "image")]
use image::{DynamicImage, GrayImage, RgbaImage};

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
pub type Pixels = i32;

/// The pixel format of the rendered image data in the backing buffer of a [PdfBitmap].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfBitmapFormat {
    Gray = FPDFBitmap_Gray as isize,
    BGR = FPDFBitmap_BGR as isize,
    BGRx = FPDFBitmap_BGRx as isize,
    BGRA = FPDFBitmap_BGRA as isize,

    // TODO: AJRC - 22/7/23 - remove deprecated variant in 0.9.0
    // as part of tracking issue https://github.com/ajrcarey/pdfium-render/issues/36
    #[deprecated(
        since = "0.8.7",
        note = "This variant has been renamed to correct a misspelling. Use the BGRx variant instead."
    )]
    #[doc(hidden)]
    BRGx = 999,
}

impl PdfBitmapFormat {
    #[inline]
    #[allow(non_upper_case_globals)]
    pub(crate) fn from_pdfium(format: u32) -> Result<Self, PdfiumError> {
        match format {
            FPDFBitmap_Unknown => Err(PdfiumError::UnknownBitmapFormat),
            FPDFBitmap_Gray => Ok(PdfBitmapFormat::Gray),
            FPDFBitmap_BGR => Ok(PdfBitmapFormat::BGR),
            FPDFBitmap_BGRx => Ok(PdfBitmapFormat::BGRx),
            FPDFBitmap_BGRA => Ok(PdfBitmapFormat::BGRA),
            _ => Err(PdfiumError::UnknownBitmapFormat),
        }
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> u32 {
        match self {
            PdfBitmapFormat::Gray => FPDFBitmap_Gray,
            PdfBitmapFormat::BGR => FPDFBitmap_BGR,
            #[allow(deprecated)]
            PdfBitmapFormat::BRGx | PdfBitmapFormat::BGRx => FPDFBitmap_BGRx,
            PdfBitmapFormat::BGRA => FPDFBitmap_BGRA,
        }
    }
}

// Deriving Default for enums is experimental. We implement the trait ourselves
// to provide better compatibility with older Rust versions.
#[allow(clippy::derivable_impls)]
impl Default for PdfBitmapFormat {
    #[inline]
    fn default() -> Self {
        PdfBitmapFormat::BGRA
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
        let format = self.format().unwrap_or(PdfBitmapFormat::default());

        match format {
            #[allow(deprecated)]
            PdfBitmapFormat::BGRA | PdfBitmapFormat::BRGx | PdfBitmapFormat::BGRx => {
                RgbaImage::from_raw(
                    self.width() as u32,
                    self.height() as u32,
                    bgra_to_rgba(self.as_bytes()),
                )
                .map(DynamicImage::ImageRgba8)
            }
            PdfBitmapFormat::BGR => RgbaImage::from_raw(
                self.width() as u32,
                self.height() as u32,
                aligned_bgr_to_rgba(
                    self.as_bytes(),
                    self.width() as usize,
                    self.as_bytes().len() / self.height() as usize,
                ),
            )
            .map(DynamicImage::ImageRgba8),
            PdfBitmapFormat::Gray => GrayImage::from_raw(
                self.width() as u32,
                self.height() as u32,
                self.as_bytes().to_vec(),
            )
            .map(DynamicImage::ImageLuma8),
        }
        // TODO: AJRC - 3/11/23 - change function signature to return Result<DynamicImage, PdfiumError>
        // in 0.9.0 so we can account for any image conversion failure here. Tracked
        // as part of https://github.com/ajrcarey/pdfium-render/issues/36
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

#[cfg(test)]
mod tests {
    use crate::bitmap::{PdfBitmap, PdfBitmapFormat};
    use crate::error::PdfiumError;
    use crate::utils::mem::create_sized_buffer;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_from_bytes() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        let test_width = 2000;
        let test_height = 4000;

        let buffer_size_required = PdfBitmap::bytes_required_for_size(test_width, test_height);

        let mut buffer = create_sized_buffer(buffer_size_required);

        let buffer_ptr = buffer.as_ptr();

        let bitmap = unsafe {
            PdfBitmap::from_bytes(
                test_width,
                test_height,
                PdfBitmapFormat::BGRx,
                buffer.as_mut_slice(),
                pdfium.bindings(),
            )?
        };

        assert_eq!(bitmap.width(), test_width);
        assert_eq!(bitmap.height(), test_height);
        assert_eq!(
            pdfium.bindings().FPDFBitmap_GetBuffer(bitmap.handle) as usize,
            buffer_ptr as usize
        );
        assert_eq!(
            pdfium.bindings().FPDFBitmap_GetStride(bitmap.handle),
            // The stride length is always a multiple of four bytes; for image formats
            // that require less than four bytes per pixel, the extra bytes serve as
            // alignment padding. For this test, we use the PdfBitmapFormat::BGRx which
            // consumes four bytes per pixel, so test_width * 4 should indeed match
            // the returned stride length.
            test_width * 4
        );

        Ok(())
    }
}
