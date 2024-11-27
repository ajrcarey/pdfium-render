//! Defines the [PdfPageImageObject] struct, exposing functionality related to a single page
//! object defining a bitmapped image.

use crate::bindgen::{
    fpdf_page_t__, FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_IMAGEOBJ_METADATA, FPDF_PAGE,
    FPDF_PAGEOBJECT,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::bitmap::PdfBitmap;
use crate::pdf::bitmap::Pixels;
use crate::pdf::color_space::PdfColorSpace;
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::PdfPageObject;
use crate::pdf::document::PdfDocument;
use crate::pdf::matrix::{PdfMatrix, PdfMatrixValue};
use crate::pdf::points::PdfPoints;
use crate::utils::mem::create_byte_buffer;
use crate::{create_transform_getters, create_transform_setters};
use std::convert::TryInto;
use std::ops::{Range, RangeInclusive};
use std::os::raw::{c_int, c_void};

#[cfg(any(feature = "image_latest", feature = "image_025"))]
use {
    crate::pdf::bitmap::PdfBitmapFormat,
    crate::utils::pixels::{aligned_bgr_to_rgba, bgra_to_rgba, rgba_to_bgra},
    image_025::{DynamicImage, EncodableLayout, GrayImage, RgbaImage},
};

#[cfg(feature = "image_024")]
use {
    crate::pdf::bitmap::PdfBitmapFormat,
    crate::utils::pixels::{aligned_bgr_to_rgba, bgra_to_rgba, rgba_to_bgra},
    image_024::{DynamicImage, EncodableLayout, GrayImage, RgbaImage},
};

#[cfg(feature = "image_023")]
use {
    crate::pdf::bitmap::PdfBitmapFormat,
    crate::utils::pixels::{aligned_bgr_to_rgba, bgra_to_rgba, rgba_to_bgra},
    image_023::{DynamicImage, EncodableLayout, GenericImageView, GrayImage, RgbaImage},
};

/// A single `PdfPageObject` of type `PdfPageObjectType::Image`. The page object defines a single
/// bitmapped image.
///
/// Page objects can be created either attached to a `PdfPage` (in which case the page object's
/// memory is owned by the containing page) or detached from any page (in which case the page
/// object's memory is owned by the object). Page objects are not rendered until they are
/// attached to a page; page objects that are never attached to a page will be lost when they
/// fall out of scope.
///
/// The simplest way to create a page image object that is immediately attached to a page
/// is to call the `PdfPageObjects::create_image_object()` function.
///
/// Creating a detached page image object offers more scope for customization, but you must
/// add the object to a containing `PdfPage` manually. To create a detached page image object,
/// use the [PdfPageImageObject::new()] function. The detached page image object can later
/// be attached to a page by using the `PdfPageObjects::add_image_object()` function.
pub struct PdfPageImageObject<'a> {
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    annotation_handle: Option<FPDF_ANNOTATION>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageImageObject<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: Option<FPDF_PAGE>,
        annotation_handle: Option<FPDF_ANNOTATION>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageImageObject {
            object_handle,
            page_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The returned page object
    /// will not be rendered until it is added to a `PdfPage` using the
    /// `PdfPageObjects::add_image_object()` function.
    ///
    /// The returned page object will have its width and height both set to 1.0 points.
    /// Use the [PdfPageImageObject::scale()] function to apply a horizontal and vertical scale
    /// to the object after it is created, or use one of the [PdfPageImageObject::new_with_width()],
    /// [PdfPageImageObject::new_with_height()], or [PdfPageImageObject::new_with_size()] functions
    /// to scale the page object to a specific width and/or height at the time the object is created.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn new(document: &PdfDocument<'a>, image: &DynamicImage) -> Result<Self, PdfiumError> {
        let mut result = Self::new_from_handle(document.handle(), document.bindings());

        if let Ok(result) = result.as_mut() {
            result.set_image(image)?;
        }

        result
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The returned page object
    /// will not be rendered until it is added to a `PdfPage` using the
    /// `PdfPageObjects::add_image_object()` function.
    ///
    /// The returned page object will have its width and height both set to 1.0 points.
    /// Use the [WriteTransforms::scale()] function to apply a horizontal and vertical scale
    /// to the object after it is created.
    #[cfg(not(feature = "image"))]
    pub fn new(document: &PdfDocument<'a>) -> Result<Self, PdfiumError> {
        Self::new_from_handle(document.handle(), document.bindings())
    }

    // Takes a raw FPDF_DOCUMENT handle to avoid cascading lifetime problems
    // associated with borrowing PdfDocument<'a>.
    pub(crate) fn new_from_handle(
        document: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Result<Self, PdfiumError> {
        let handle = bindings.FPDFPageObj_NewImageObj(document);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfPageImageObject {
                object_handle: handle,
                page_handle: None,
                annotation_handle: None,
                bindings,
            })
        }
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The page object will be scaled
    /// horizontally to match the given width; its height will be adjusted to maintain the aspect
    /// ratio of the given image. The returned page object will not be rendered until it is
    /// added to a `PdfPage` using the `PdfPageObjects::add_image_object()` function.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    pub fn new_with_width(
        document: &PdfDocument<'a>,
        image: &DynamicImage,
        width: PdfPoints,
    ) -> Result<Self, PdfiumError> {
        let aspect_ratio = image.height() as f32 / image.width() as f32;

        let height = width * aspect_ratio;

        Self::new_with_size(document, image, width, height)
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The page object will be scaled
    /// vertically to match the given height; its width will be adjusted to maintain the aspect
    /// ratio of the given image. The returned page object will not be rendered until it is
    /// added to a `PdfPage` using the `PdfPageObjects::add_image_object()` function.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    pub fn new_with_height(
        document: &PdfDocument<'a>,
        image: &DynamicImage,
        height: PdfPoints,
    ) -> Result<Self, PdfiumError> {
        let aspect_ratio = image.height() as f32 / image.width() as f32;

        let width = height / aspect_ratio;

        Self::new_with_size(document, image, width, height)
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The page object will be scaled to
    /// match the given width and height. The returned page object will not be rendered until it is
    /// added to a `PdfPage` using the `PdfPageObjects::add_image_object()` function.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn new_with_size(
        document: &PdfDocument<'a>,
        image: &DynamicImage,
        width: PdfPoints,
        height: PdfPoints,
    ) -> Result<Self, PdfiumError> {
        let mut result = Self::new(document, image)?;

        result.scale(width.value, height.value)?;

        Ok(result)
    }

    /// Returns a new [PdfBitmap] created from the bitmap buffer backing
    /// this [PdfPageImageObject], ignoring any image filters, image mask, or object
    /// transforms applied to this page object.
    pub fn get_raw_bitmap(&self) -> Result<PdfBitmap, PdfiumError> {
        Ok(PdfBitmap::from_pdfium(
            self.bindings.FPDFImageObj_GetBitmap(self.object_handle),
            self.bindings,
        ))
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], ignoring any image filters, image mask, or object
    /// transforms applied to this page object.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn get_raw_image(&self) -> Result<DynamicImage, PdfiumError> {
        self.get_image_from_bitmap(&self.get_raw_bitmap()?)
    }

    /// Returns a new [PdfBitmap] created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    #[inline]
    pub fn get_processed_bitmap(&self, document: &PdfDocument) -> Result<PdfBitmap, PdfiumError> {
        let (width, height) = self.get_current_width_and_height_from_metadata()?;

        self.get_processed_bitmap_with_size(document, width, height)
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn get_processed_image(&self, document: &PdfDocument) -> Result<DynamicImage, PdfiumError> {
        let (width, height) = self.get_current_width_and_height_from_metadata()?;

        self.get_processed_image_with_size(document, width, height)
    }

    /// Returns a new [PdfBitmap] created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// The returned bitmap will be scaled during rendering so its width matches the given target width.
    #[inline]
    pub fn get_processed_bitmap_with_width(
        &self,
        document: &PdfDocument,
        width: Pixels,
    ) -> Result<PdfBitmap, PdfiumError> {
        let (current_width, current_height) = self.get_current_width_and_height_from_metadata()?;

        let aspect_ratio = current_width as f32 / current_height as f32;

        self.get_processed_bitmap_with_size(
            document,
            width,
            ((width as f32 * aspect_ratio) as u32)
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?,
        )
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// The returned image will be scaled during rendering so its width matches the given target width.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn get_processed_image_with_width(
        &self,
        document: &PdfDocument,
        width: Pixels,
    ) -> Result<DynamicImage, PdfiumError> {
        let (current_width, current_height) = self.get_current_width_and_height_from_metadata()?;

        let aspect_ratio = current_width as f32 / current_height as f32;

        self.get_processed_image_with_size(
            document,
            width,
            ((width as f32 * aspect_ratio) as u32)
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?,
        )
    }

    /// Returns a new [PdfBitmap] created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// The returned bitmap will be scaled during rendering so its height matches the given target height.
    #[inline]
    pub fn get_processed_bitmap_with_height(
        &self,
        document: &PdfDocument,
        height: Pixels,
    ) -> Result<PdfBitmap, PdfiumError> {
        let (current_width, current_height) = self.get_current_width_and_height_from_metadata()?;

        let aspect_ratio = current_width as f32 / current_height as f32;

        self.get_processed_bitmap_with_size(
            document,
            ((height as f32 / aspect_ratio) as u32)
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?,
            height,
        )
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// The returned image will be scaled during rendering so its height matches the given target height.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn get_processed_image_with_height(
        &self,
        document: &PdfDocument,
        height: Pixels,
    ) -> Result<DynamicImage, PdfiumError> {
        let (current_width, current_height) = self.get_current_width_and_height_from_metadata()?;

        let aspect_ratio = current_width as f32 / current_height as f32;

        self.get_processed_image_with_size(
            document,
            ((height as f32 / aspect_ratio) as u32)
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?,
            height,
        )
    }

    /// Returns a new [PdfBitmap] created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// The returned bitmap will be scaled during rendering so its width and height match
    /// the given target dimensions.
    pub fn get_processed_bitmap_with_size(
        &self,
        document: &PdfDocument,
        width: Pixels,
        height: Pixels,
    ) -> Result<PdfBitmap, PdfiumError> {
        // We attempt to work around two separate problems in Pdfium's
        // FPDFImageObj_GetRenderedBitmap() function.

        // First, the call to FPDFImageObj_GetRenderedBitmap() can fail, returning
        // a null FPDF_BITMAP handle, if the image object's transformation matrix includes
        // negative values for either the matrix.a or matrix.d values. We flip those values
        // in the transformation matrix if they are negative, and we make sure we return them
        // to their original values before we return to the caller.

        // Second, Pdfium seems to often return a rendered bitmap that is much smaller
        // than the image object's metadata suggests. We look at the dimensions of the bitmap
        // returned from FPDFImageObj_GetRenderedBitmap(), and we apply a scale factor to the
        // image object's transformation matrix if the bitmap is not the expected size.

        // For more details, see: https://github.com/ajrcarey/pdfium-render/issues/52

        let mut matrix = self.matrix()?;

        let original_matrix = matrix; // We'll reset the matrix to this before we return.

        // Ensure the matrix.a and matrix.d values are not negative.

        if matrix.a() < 0f32 {
            matrix.set_a(-matrix.a());
            self.reset_matrix_impl(matrix)?;
        }

        if matrix.d() < 0f32 {
            matrix.set_d(-matrix.d());
            self.reset_matrix_impl(matrix)?;
        }

        let handle = match self.page_handle {
            Some(page_handle) => self.bindings.FPDFImageObj_GetRenderedBitmap(
                document.handle(),
                page_handle,
                self.object_handle,
            ),
            None => self.bindings.FPDFImageObj_GetRenderedBitmap(
                document.handle(),
                std::ptr::null_mut::<fpdf_page_t__>(),
                self.object_handle,
            ),
        };

        if handle.is_null() {
            // Restore the original transformation matrix values before we return the error
            // to the caller.

            self.reset_matrix_impl(original_matrix)?;
            return Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ));
        }

        let result = PdfBitmap::from_pdfium(handle, self.bindings);

        if width == result.width() && height == result.height() {
            // The bitmap generated by Pdfium is already at the caller's requested dimensions.
            // Restore the original transformation matrix values before we return to the caller.

            self.reset_matrix_impl(original_matrix)?;

            Ok(result)
        } else {
            // The bitmap generated by Pdfium is not at the caller's requested dimensions.
            // We apply a scale transform to the page object to encourage Pdfium to generate
            // a bitmap matching the caller's requested dimensions.

            self.transform_impl(
                width as PdfMatrixValue / result.width() as PdfMatrixValue,
                0.0,
                0.0,
                height as PdfMatrixValue / result.height() as PdfMatrixValue,
                0.0,
                0.0,
            )?;

            // Generate the bitmap again at the new scale.

            let result = PdfBitmap::from_pdfium(
                match self.page_handle {
                    Some(page_handle) => self.bindings.FPDFImageObj_GetRenderedBitmap(
                        document.handle(),
                        page_handle,
                        self.object_handle,
                    ),
                    None => self.bindings.FPDFImageObj_GetRenderedBitmap(
                        document.handle(),
                        std::ptr::null_mut::<fpdf_page_t__>(),
                        self.object_handle,
                    ),
                },
                self.bindings,
            );

            // Restore the original transformation matrix values before we return to the caller.

            self.reset_matrix_impl(original_matrix)?;

            Ok(result)
        }
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    ///
    /// The returned image will be scaled during rendering so its width and height match
    /// the given target dimensions.
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    #[inline]
    pub fn get_processed_image_with_size(
        &self,
        document: &PdfDocument,
        width: Pixels,
        height: Pixels,
    ) -> Result<DynamicImage, PdfiumError> {
        self.get_processed_bitmap_with_size(document, width, height)
            .and_then(|bitmap| self.get_image_from_bitmap(&bitmap))
    }

    #[cfg(feature = "image")]
    pub(crate) fn get_image_from_bitmap(
        &self,
        bitmap: &PdfBitmap,
    ) -> Result<DynamicImage, PdfiumError> {
        let handle = *bitmap.handle();

        let width = self.bindings.FPDFBitmap_GetWidth(handle);

        let height = self.bindings.FPDFBitmap_GetHeight(handle);

        let stride = self.bindings.FPDFBitmap_GetStride(handle);

        let format =
            PdfBitmapFormat::from_pdfium(self.bindings.FPDFBitmap_GetFormat(handle) as u32)?;

        #[cfg(not(target_arch = "wasm32"))]
        let buffer = self.bindings.FPDFBitmap_GetBuffer_as_slice(handle);

        #[cfg(target_arch = "wasm32")]
        let buffer_vec = self.bindings.FPDFBitmap_GetBuffer_as_vec(handle);
        #[cfg(target_arch = "wasm32")]
        let buffer = buffer_vec.as_slice();

        match format {
            #[allow(deprecated)]
            PdfBitmapFormat::BGRA | PdfBitmapFormat::BRGx | PdfBitmapFormat::BGRx => {
                RgbaImage::from_raw(width as u32, height as u32, bgra_to_rgba(buffer))
                    .map(DynamicImage::ImageRgba8)
            }
            PdfBitmapFormat::BGR => RgbaImage::from_raw(
                width as u32,
                height as u32,
                aligned_bgr_to_rgba(buffer, width as usize, stride as usize),
            )
            .map(DynamicImage::ImageRgba8),
            PdfBitmapFormat::Gray => {
                GrayImage::from_raw(width as u32, height as u32, buffer.to_vec())
                    .map(DynamicImage::ImageLuma8)
            }
        }
        .ok_or(PdfiumError::ImageError)
    }

    /// Return the expected pixel width and height of the processed image from Pdfium's metadata.
    pub(crate) fn get_current_width_and_height_from_metadata(
        &self,
    ) -> Result<(Pixels, Pixels), PdfiumError> {
        let width = self.get_raw_metadata().and_then(|metadata| {
            metadata
                .width
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)
        })?;

        let height = self.get_raw_metadata().and_then(|metadata| {
            metadata
                .height
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)
        })?;

        Ok((width, height))
    }

    /// Applies the byte data in the given `Image::DynamicImage` to this [PdfPageImageObject].
    ///
    /// This function is only available when this crate's `image` feature is enabled.
    #[cfg(feature = "image")]
    pub fn set_image(&mut self, image: &DynamicImage) -> Result<(), PdfiumError> {
        let width: Pixels = image
            .width()
            .try_into()
            .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?;

        let height: Pixels = image
            .height()
            .try_into()
            .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?;

        let bitmap = PdfBitmap::empty(width, height, PdfBitmapFormat::BGRA, self.bindings)?;

        let buffer = if let Some(image) = image.as_rgba8() {
            // The given image is already in RGBA format.

            rgba_to_bgra(image.as_bytes())
        } else {
            // The image must be converted to RGBA first.

            let image = image.to_rgba8();

            rgba_to_bgra(image.as_bytes())
        };

        if !self
            .bindings
            .FPDFBitmap_SetBuffer(*bitmap.handle(), buffer.as_slice())
        {
            return Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ));
        }

        self.set_bitmap(&bitmap)
    }

    /// Applies the byte data in the given [PdfBitmap] to this [PdfPageImageObject].
    pub fn set_bitmap(&mut self, bitmap: &PdfBitmap) -> Result<(), PdfiumError> {
        if self.bindings.is_true(self.bindings.FPDFImageObj_SetBitmap(
            std::ptr::null_mut::<FPDF_PAGE>(),
            0,
            self.object_handle,
            *bitmap.handle(),
        )) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    pub(crate) fn get_raw_metadata(&self) -> Result<FPDF_IMAGEOBJ_METADATA, PdfiumError> {
        let mut metadata = FPDF_IMAGEOBJ_METADATA {
            width: 0,
            height: 0,
            horizontal_dpi: 0.0,
            vertical_dpi: 0.0,
            bits_per_pixel: 0,
            colorspace: 0,
            marked_content_id: 0,
        };

        let result = self.bindings.FPDFImageObj_GetImageMetadata(
            self.object_handle,
            match self.page_handle {
                Some(page_handle) => page_handle,
                None => std::ptr::null_mut::<fpdf_page_t__>(),
            },
            &mut metadata,
        );

        if self.bindings.is_true(result) {
            Ok(metadata)
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns the horizontal dots per inch resolution of the image assigned to this
    /// [PdfPageImageObject], based on the intrinsic resolution of the assigned image
    /// and the dimensions of this object.
    #[inline]
    pub fn horizontal_dpi(&self) -> Result<f32, PdfiumError> {
        self.get_raw_metadata()
            .map(|metadata| metadata.horizontal_dpi)
    }

    /// Returns the vertical dots per inch resolution of the image assigned to this
    /// [PdfPageImageObject], based on the intrinsic resolution of the assigned image
    /// and the dimensions of this object.
    #[inline]
    pub fn vertical_dpi(&self) -> Result<f32, PdfiumError> {
        self.get_raw_metadata()
            .map(|metadata| metadata.vertical_dpi)
    }

    /// Returns the bits per pixel for the image assigned to this [PdfPageImageObject].
    ///
    /// This value is not available if this object has not been attached to a `PdfPage`.
    #[inline]
    pub fn bits_per_pixel(&self) -> Result<u8, PdfiumError> {
        self.get_raw_metadata()
            .map(|metadata| metadata.bits_per_pixel as u8)
    }

    /// Returns the color space for the image assigned to this [PdfPageImageObject].
    ///
    /// This value is not available if this object has not been attached to a `PdfPage`.
    #[inline]
    pub fn color_space(&self) -> Result<PdfColorSpace, PdfiumError> {
        self.get_raw_metadata()
            .and_then(|metadata| PdfColorSpace::from_pdfium(metadata.colorspace as u32))
    }

    /// Returns the collection of image filters currently applied to this [PdfPageImageObject].
    #[inline]
    pub fn filters(&self) -> PdfPageImageObjectFilters {
        PdfPageImageObjectFilters::new(self)
    }

    create_transform_setters!(
        &mut Self,
        Result<(), PdfiumError>,
        "this [PdfPageImageObject]",
        "this [PdfPageImageObject].",
        "this [PdfPageImageObject],"
    );

    // The transform_impl() function required by the create_transform_setters!() macro
    // is provided by the PdfPageObjectPrivate trait.

    create_transform_getters!(
        "this [PdfPageImageObject]",
        "this [PdfPageImageObject].",
        "this [PdfPageImageObject],"
    );

    // The get_matrix_impl() function required by the create_transform_getters!() macro
    // is provided by the PdfPageObjectPrivate trait.
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageImageObject<'a> {
    #[inline]
    fn get_object_handle(&self) -> FPDF_PAGEOBJECT {
        self.object_handle
    }

    #[inline]
    fn get_page_handle(&self) -> Option<FPDF_PAGE> {
        self.page_handle
    }

    #[inline]
    fn set_page_handle(&mut self, page: FPDF_PAGE) {
        self.page_handle = Some(page);
    }

    #[inline]
    fn clear_page_handle(&mut self) {
        self.page_handle = None;
    }

    #[inline]
    fn get_annotation_handle(&self) -> Option<FPDF_ANNOTATION> {
        self.annotation_handle
    }

    #[inline]
    fn set_annotation_handle(&mut self, annotation: FPDF_ANNOTATION) {
        self.annotation_handle = Some(annotation);
    }

    #[inline]
    fn clear_annotation_handle(&mut self) {
        self.annotation_handle = None;
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn is_copyable_impl(&self) -> bool {
        // Image filters cannot be copied.

        self.filters().is_empty()
    }

    #[inline]
    fn try_copy_impl<'b>(
        &self,
        document: FPDF_DOCUMENT,
        bindings: &'b dyn PdfiumLibraryBindings,
    ) -> Result<PdfPageObject<'b>, PdfiumError> {
        if !self.filters().is_empty() {
            // Image filters cannot be copied.

            return Err(PdfiumError::ImageObjectFiltersNotCopyable);
        }

        let mut copy = PdfPageImageObject::new_from_handle(document, bindings)?;

        copy.set_bitmap(&self.get_raw_bitmap()?)?;
        copy.reset_matrix(self.matrix()?)?;

        Ok(PdfPageObject::Image(copy))
    }
}

pub type PdfPageImageObjectFilterIndex = usize;

/// A collection of all the image filters applied to a [PdfPageImageObject].
pub struct PdfPageImageObjectFilters<'a> {
    object: &'a PdfPageImageObject<'a>,
}

impl<'a> PdfPageImageObjectFilters<'a> {
    #[inline]
    pub(crate) fn new(object: &'a PdfPageImageObject<'a>) -> Self {
        PdfPageImageObjectFilters { object }
    }

    /// Returns the number of image filters applied to the parent [PdfPageImageObject].
    pub fn len(&self) -> usize {
        self.object
            .bindings
            .FPDFImageObj_GetImageFilterCount(self.object.get_object_handle()) as usize
    }

    /// Returns true if this [PdfPageImageObjectFilters] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of filters)` for this [PdfPageImageObjectFilters] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageImageObjectFilterIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of filters - 1)` for this [PdfPageImageObjectFilters] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfPageImageObjectFilterIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfPageImageObjectFilter] from this [PdfPageImageObjectFilters] collection.
    pub fn get(
        &self,
        index: PdfPageImageObjectFilterIndex,
    ) -> Result<PdfPageImageObjectFilter, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::ImageObjectFilterIndexOutOfBounds);
        }

        // Retrieving the image filter name from Pdfium is a two-step operation. First, we call
        // FPDFImageObj_GetImageFilter() with a null buffer; this will retrieve the length of
        // the image filter name in bytes. If the length is zero, then there is no image filter name.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFImageObj_GetImageFilter() again with a pointer to the buffer;
        // this will write the font name into the buffer. Unlike most text handling in
        // Pdfium, image filter names are returned in UTF-8 format.

        let buffer_length = self.object.bindings.FPDFImageObj_GetImageFilter(
            self.object.get_object_handle(),
            index as c_int,
            std::ptr::null_mut(),
            0,
        );

        if buffer_length == 0 {
            // The image filter name is not present.

            return Err(PdfiumError::ImageObjectFilterIndexInBoundsButFilterUndefined);
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.object.bindings.FPDFImageObj_GetImageFilter(
            self.object.get_object_handle(),
            index as c_int,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        Ok(PdfPageImageObjectFilter::new(
            String::from_utf8(buffer)
                // Trim any trailing nulls. All strings returned from Pdfium are generally terminated
                // by one null byte.
                .map(|str| str.trim_end_matches(char::from(0)).to_owned())
                .unwrap_or_default(),
        ))
    }

    /// Returns an iterator over all the [PdfPageImageObjectFilter] objects in this
    /// [PdfPageImageObjectFilters] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageImageObjectFiltersIterator {
        PdfPageImageObjectFiltersIterator::new(self)
    }
}

/// A single image filter applied to a [PdfPageImageObject].
pub struct PdfPageImageObjectFilter {
    name: String,
}

impl PdfPageImageObjectFilter {
    #[inline]
    pub(crate) fn new(name: String) -> Self {
        PdfPageImageObjectFilter { name }
    }

    /// Returns the name of this [PdfPageImageObjectFilter].
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

/// An iterator over all the [PdfPageImageObjectFilter] objects in a
/// [PdfPageImageObjectFilters] collection.
pub struct PdfPageImageObjectFiltersIterator<'a> {
    filters: &'a PdfPageImageObjectFilters<'a>,
    next_index: PdfPageImageObjectFilterIndex,
}

impl<'a> PdfPageImageObjectFiltersIterator<'a> {
    #[inline]
    pub(crate) fn new(filters: &'a PdfPageImageObjectFilters<'a>) -> Self {
        PdfPageImageObjectFiltersIterator {
            filters,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfPageImageObjectFiltersIterator<'a> {
    type Item = PdfPageImageObjectFilter;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.filters.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_page_image_object_retains_format() -> Result<(), PdfiumError> {
        // Make sure the format of the image we pass into a new PdfPageImageObject is the
        // same when we later retrieve it.

        let pdfium = test_bind_to_pdfium();

        let image = pdfium
            .load_pdf_from_file("./test/path-test.pdf", None)?
            .pages()
            .get(0)?
            .render_with_config(&PdfRenderConfig::new().set_target_width(1000))?
            .as_image();

        let mut document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages_mut()
            .create_page_at_end(PdfPagePaperSize::a4())?;

        let object = page.objects_mut().create_image_object(
            PdfPoints::new(100.0),
            PdfPoints::new(100.0),
            &image,
            Some(PdfPoints::new(image.width() as f32)),
            Some(PdfPoints::new(image.height() as f32)),
        )?;

        // Since the object has no image filters applied, both the raw and processed images should
        // be identical to the source image we assigned to the object. The processed image will
        // take the object's scale factors into account, but we made sure to set those to the actual
        // pixel dimensions of the source image.

        // A visual inspection can be carried out by uncommenting the PNG save commands below.

        let raw_image = object.as_image_object().unwrap().get_raw_image()?;

        // raw_image
        //     .save_with_format("./test/1.png", ImageFormat::Png)
        //     .unwrap();

        let processed_image = object
            .as_image_object()
            .unwrap()
            .get_processed_image(&document)?;

        // processed_image
        //     .save_with_format("./test/2.png", ImageFormat::Png)
        //     .unwrap();

        assert!(compare_equality_of_byte_arrays(
            image.as_bytes(),
            raw_image.into_rgba8().as_raw().as_slice()
        ));

        assert!(compare_equality_of_byte_arrays(
            image.as_bytes(),
            processed_image.into_rgba8().as_raw().as_slice()
        ));

        Ok(())
    }

    fn compare_equality_of_byte_arrays(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for index in 0..a.len() {
            if a[index] != b[index] {
                return false;
            }
        }

        true
    }
}
