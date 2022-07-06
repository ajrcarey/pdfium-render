//! Defines the [PdfPageImageObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Image`.

use crate::bindgen::{
    fpdf_page_t__, FPDFBitmap_BGRA, FPDF_BITMAP, FPDF_DOCUMENT, FPDF_PAGE, FPDF_PAGEOBJECT,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::bitmap::PdfBitmap;
use crate::document::PdfDocument;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page_object::PdfPageObjectCommon;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use image::{DynamicImage, EncodableLayout, ImageBuffer};

/// A single `PdfPageObject` of type `PdfPageObjectType::Image`.
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
    /// Use the [PdfPageImageObject::scale()] function to apply a horizontal and vertical scale
    /// to the object after it is created, or use the [PdfPageImageObject::new_with_scale()]
    /// function to apply scaling at the time the object is created.
    #[inline]
    pub fn new(document: &PdfDocument<'a>, image: DynamicImage) -> Result<Self, PdfiumError> {
        Self::new_from_handle(*document.get_handle(), image, document.get_bindings())
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

    /// Creates a new [PdfPageImageObject] from the given arguments. The given horizontal and
    /// vertical scale factors will be applied to the created page object. The returned page object
    /// will not be rendered until it is added to a `PdfPage` using the
    /// `PdfPageObjects::add_image_object()` function.
    #[inline]
    pub fn new_with_scale(
        document: &PdfDocument<'a>,
        image: DynamicImage,
        horizontal_scale_factor: f64,
        vertical_scale_factor: f64,
    ) -> Result<Self, PdfiumError> {
        let mut result =
            Self::new_from_handle(*document.get_handle(), image, document.get_bindings())?;

        result.scale(horizontal_scale_factor, vertical_scale_factor)?;

        Ok(result)
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], ignoring any image filters, image mask, or object
    /// transforms applied to this page object.
    pub fn get_raw_image(&self) -> Result<DynamicImage, PdfiumError> {
        self.get_image_from_bitmap_handle(self.bindings.FPDFImageObj_GetBitmap(self.object_handle))
    }

    /// Returns a new `Image::DynamicImage` created from the bitmap buffer backing
    /// this [PdfPageImageObject], taking into account all image filters, image mask, and
    /// object transforms applied to this page object.
    pub fn get_processed_image(&self, document: &PdfDocument) -> Result<DynamicImage, PdfiumError> {
        self.get_image_from_bitmap_handle(match self.page_handle {
            Some(page_handle) => self.bindings.FPDFImageObj_GetRenderedBitmap(
                *document.get_handle(),
                page_handle,
                self.object_handle,
            ),
            None => self.bindings.FPDFImageObj_GetRenderedBitmap(
                *document.get_handle(),
                std::ptr::null_mut::<fpdf_page_t__>(),
                self.object_handle,
            ),
        })
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

            let buffer = unsafe {
                std::slice::from_raw_parts(buffer_start as *const u8, buffer_length as usize)
            };

            let result = ImageBuffer::from_raw(width as u32, height as u32, buffer.to_owned())
                .map(DynamicImage::ImageRgba8)
                .ok_or(PdfiumError::ImageError);

            self.bindings.FPDFBitmap_Destroy(bitmap);

            result
        }
    }

    /// Applies the byte data in the given `Image::DynamicImage` to this [PdfPageImageObject].
    pub fn set_image(&mut self, image: DynamicImage) -> Result<(), PdfiumError> {
        let image = image.to_rgba8();

        let bitmap = PdfBitmap::create_empty_bitmap_handle(
            image.width() as i32,
            image.height() as i32,
            FPDFBitmap_BGRA as i32,
            self.bindings,
        )?;

        if !self.bindings.FPDFBitmap_SetBuffer(bitmap, image.as_bytes()) {
            return Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ));
        }

        if self.bindings.is_true(self.bindings.FPDFImageObj_SetBitmap(
            std::ptr::null_mut::<FPDF_PAGE>(),
            0,
            self.object_handle,
            bitmap,
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
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
