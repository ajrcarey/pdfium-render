//! Defines the [PdfPageImageObject] struct, exposing functionality related to a single page
//! object defining a bitmapped image.

use crate::bindgen::{
    fpdf_page_t__, FPDF_BITMAP, FPDF_DOCUMENT, FPDF_IMAGEOBJ_METADATA, FPDF_PAGE, FPDF_PAGEOBJECT,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::bitmap::{PdfBitmap, PdfBitmapFormat};
use crate::color_space::PdfColorSpace;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page::PdfPoints;
use crate::page_object::PdfPageObjectCommon;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::utils::mem::create_byte_buffer;
use crate::utils::pixels::{bgr_to_rgba, bgra_to_rgba, rgba_to_bgra};
use image::{DynamicImage, EncodableLayout, RgbaImage};
use std::convert::TryInto;
use std::ops::{Range, RangeInclusive};
use std::os::raw::{c_int, c_void};

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
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageImageObject<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageImageObject {
            object_handle,
            page_handle: Some(page_handle),
            bindings,
        }
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The returned page object
    /// will not be rendered until it is added to a `PdfPage` using the
    /// `PdfPageObjects::add_image_object()` function.
    ///
    /// The returned page object will have its width and height both set to 1.0 points.
    /// Use the [PdfPageObjectCommon::scale()] function to apply a horizontal and vertical scale
    /// to the object after it is created, or use one of the [PdfPageImageObject::new_with_width()],
    /// [PdfPageImageObject::new_with_height()], or [PdfPageImageObject::new_with_size()] functions
    /// to scale the page object to a specific width and/or height at the time the object is created.
    #[inline]
    pub fn new(document: &PdfDocument<'a>, image: DynamicImage) -> Result<Self, PdfiumError> {
        Self::new_from_handle(*document.handle(), image, document.bindings())
    }

    // Takes a raw FPDF_DOCUMENT handle to avoid cascading lifetime problems
    // associated with borrowing PdfDocument<'a>.
    pub(crate) fn new_from_handle(
        document: FPDF_DOCUMENT,
        image: DynamicImage,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Result<Self, PdfiumError> {
        let handle = bindings.FPDFPageObj_NewImageObj(document);

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
            let mut result = PdfPageImageObject {
                object_handle: handle,
                page_handle: None,
                bindings,
            };

            result.set_image(image)?;

            Ok(result)
        }
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The page object will be scaled
    /// horizontally to match the given width; its height will be adjusted to maintain the aspect
    /// ratio of the given image. The returned page object will not be rendered until it is
    /// added to a `PdfPage` using the `PdfPageObjects::add_image_object()` function.
    pub fn new_with_width(
        document: &PdfDocument<'a>,
        image: DynamicImage,
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
    pub fn new_with_height(
        document: &PdfDocument<'a>,
        image: DynamicImage,
        height: PdfPoints,
    ) -> Result<Self, PdfiumError> {
        let aspect_ratio = image.height() as f32 / image.width() as f32;

        let width = height / aspect_ratio;

        Self::new_with_size(document, image, width, height)
    }

    /// Creates a new [PdfPageImageObject] from the given arguments. The page object will be scaled to
    /// match the given width and height. The returned page object will not be rendered until it is
    /// added to a `PdfPage` using the `PdfPageObjects::add_image_object()` function.
    #[inline]
    pub fn new_with_size(
        document: &PdfDocument<'a>,
        image: DynamicImage,
        width: PdfPoints,
        height: PdfPoints,
    ) -> Result<Self, PdfiumError> {
        let mut result = Self::new_from_handle(*document.handle(), image, document.bindings())?;

        result.scale(width.value as f64, height.value as f64)?;

        Ok(result)
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], ignoring any image filters, image mask, or object
    /// transforms applied to this page object.
    pub fn get_raw_image(&self) -> Result<DynamicImage, PdfiumError> {
        self.get_image_from_bitmap_handle(self.bindings.FPDFImageObj_GetBitmap(self.object_handle))
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account any image filters, image mask, and
    /// object transforms applied to this page object.
    pub fn get_processed_image(&self, document: &PdfDocument) -> Result<DynamicImage, PdfiumError> {
        // TODO: AJRC - 1/10/22 - Pdfium's FPDFImageObj_GetRenderedBitmap() function fails,
        // returning a null FPDF_BITMAP handle, if the image object's transformation matrix
        // includes negative values for either the matrix.a or matrix.d values.
        // See https://groups.google.com/g/pdfium/c/V-H9LpuHpPY
        // Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/52
        // Attempt to work-around this by flipping the matrix values from negative to positive.

        let mut matrix = self.matrix()?;

        let is_matrix_a_flipped = if matrix.a < 0f32 {
            matrix.a = -matrix.a;
            self.set_matrix(matrix)?;

            true
        } else {
            false
        };

        let is_matrix_d_flipped = if matrix.d < 0f32 {
            matrix.d = -matrix.d;
            self.set_matrix(matrix)?;

            true
        } else {
            false
        };

        let result = self.get_image_from_bitmap_handle(match self.page_handle {
            Some(page_handle) => self.bindings.FPDFImageObj_GetRenderedBitmap(
                *document.handle(),
                page_handle,
                self.object_handle,
            ),
            None => self.bindings.FPDFImageObj_GetRenderedBitmap(
                *document.handle(),
                std::ptr::null_mut::<fpdf_page_t__>(),
                self.object_handle,
            ),
        });

        // Restore the previous transformation matrix values, if necessary.

        if is_matrix_a_flipped {
            matrix.a = -matrix.a;
            self.set_matrix(matrix)?;
        }

        if is_matrix_d_flipped {
            matrix.d = -matrix.d;
            self.set_matrix(matrix)?;
        }

        result
    }

    pub(crate) fn get_image_from_bitmap_handle(
        &self,
        bitmap: FPDF_BITMAP,
    ) -> Result<DynamicImage, PdfiumError> {
        if bitmap.is_null() {
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
            let width = self.bindings.FPDFBitmap_GetWidth(bitmap);

            let height = self.bindings.FPDFBitmap_GetHeight(bitmap);

            let buffer_length = self.bindings.FPDFBitmap_GetStride(bitmap) * height;

            let buffer_start = self.bindings.FPDFBitmap_GetBuffer(bitmap);

            let format =
                PdfBitmapFormat::from_pdfium(self.bindings.FPDFBitmap_GetFormat(bitmap) as u32)?;

            let buffer = unsafe {
                std::slice::from_raw_parts(buffer_start as *const u8, buffer_length as usize)
            };

            let result = match format {
                PdfBitmapFormat::BGRA | PdfBitmapFormat::BRGx => {
                    RgbaImage::from_raw(width as u32, height as u32, bgra_to_rgba(buffer))
                        .map(DynamicImage::ImageRgba8)
                }
                PdfBitmapFormat::BGR => {
                    RgbaImage::from_raw(width as u32, height as u32, bgr_to_rgba(buffer))
                        .map(DynamicImage::ImageRgba8)
                }
                _ => None,
            }
            .ok_or(PdfiumError::ImageError);

            self.bindings.FPDFBitmap_Destroy(bitmap);

            result
        }
    }

    /// Applies the byte data in the given `Image::DynamicImage` to this [PdfPageImageObject].
    pub fn set_image(&mut self, image: DynamicImage) -> Result<(), PdfiumError> {
        if let Some(image) = image.as_rgba8() {
            // The given image is already in RGBA format.

            let width: u16 = image
                .width()
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?;

            let height: u16 = image
                .height()
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?;

            let bitmap = PdfBitmap::empty(width, height, PdfBitmapFormat::BGRA, self.bindings)?;

            if !self
                .bindings
                .FPDFBitmap_SetBuffer(*bitmap.handle(), rgba_to_bgra(image.as_bytes()).as_slice())
            {
                return Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ));
            }

            self.set_bitmap(&bitmap)
        } else {
            // The image must be converted to RGBA first.

            let image = image.to_rgba8();

            let width: u16 = image
                .width()
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?;

            let height: u16 = image
                .height()
                .try_into()
                .map_err(|_| PdfiumError::ImageSizeOutOfBounds)?;

            let bitmap = PdfBitmap::empty(width, height, PdfBitmapFormat::BGRA, self.bindings)?;

            if !self
                .bindings
                .FPDFBitmap_SetBuffer(*bitmap.handle(), rgba_to_bgra(image.as_bytes()).as_slice())
            {
                return Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ));
            }

            self.set_bitmap(&bitmap)
        }
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
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
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
                self.bindings
                    .get_pdfium_last_error()
                    .unwrap_or(PdfiumInternalError::Unknown),
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
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageImageObject<'a> {
    #[inline]
    fn get_object_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.object_handle
    }

    #[inline]
    fn get_page_handle(&self) -> &Option<FPDF_PAGE> {
        &self.page_handle
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
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
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
            .bindings()
            .FPDFImageObj_GetImageFilterCount(*self.object.get_object_handle()) as usize
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

        let buffer_length = self.object.bindings().FPDFImageObj_GetImageFilter(
            *self.object.get_object_handle(),
            index as c_int,
            std::ptr::null_mut(),
            0,
        );

        if buffer_length == 0 {
            // The image filter name is not present.

            return Err(PdfiumError::ImageObjectFilterIndexInBoundsButFilterUndefined);
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.object.bindings().FPDFImageObj_GetImageFilter(
            *self.object.get_object_handle(),
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
pub mod tests {
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

        let document = pdfium.create_new_pdf()?;

        let mut page = document
            .pages()
            .create_page_at_end(PdfPagePaperSize::a4())?;

        let object = page.objects_mut().create_image_object(
            PdfPoints::new(100.0),
            PdfPoints::new(100.0),
            image.clone(),
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
