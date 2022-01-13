use crate::bindgen::{
    FPDFBitmap_BGR, FPDFBitmap_BGRA, FPDFBitmap_BGRx, FPDFBitmap_Gray, FPDFBitmap_Unknown,
    FPDF_BITMAP, FPDF_PAGE,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::PdfiumError;
use image::{DynamicImage, ImageBuffer};

/// The pixel format of the rendered image data in the backing buffer of a PdfBitmap.
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
    pub(crate) fn from_pdfium(format: u32) -> Result<PdfBitmapFormat, PdfiumError> {
        match format {
            FPDFBitmap_Unknown => Err(PdfiumError::UnknownBitmapFormat),
            FPDFBitmap_Gray => Ok(PdfBitmapFormat::Gray),
            FPDFBitmap_BGR => Ok(PdfBitmapFormat::BGR),
            FPDFBitmap_BGRx => Ok(PdfBitmapFormat::BRGx),
            FPDFBitmap_BGRA => Ok(PdfBitmapFormat::BGRA),
            _ => Err(PdfiumError::UnknownBitmapFormat),
        }
    }
}

/// A rotation transformation that should be applied to a PdfPage when it is rendered
/// into a PdfBitmap.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfBitmapRotation {
    None,
    Degrees90,
    Degrees180,
    Degrees270,
}

/// A rendered image of a single PdfPage at a specific width and height.
///
/// By default, PdfBitmap is lazy; it will not render its page into a bitmap until
/// it is required to do so in order to return a byte buffer or a DynamicImage.
/// If preferred, rendering can be initiated manually by calling the [PdfBitmap::render()] function.
/// Once rendered, the page will not be re-rendered.
pub struct PdfBitmap<'a> {
    width: u16,
    height: u16,
    format: PdfBitmapFormat,
    rotation: PdfBitmapRotation,
    bitmap_handle: FPDF_BITMAP,
    page_handle: &'a FPDF_PAGE,
    is_rendered: bool,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBitmap<'a> {
    pub(crate) fn from_pdfium(
        width: u16,
        height: u16,
        format: PdfBitmapFormat,
        rotation: PdfBitmapRotation,
        handle: FPDF_BITMAP,
        page: &'a FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfBitmap {
            width,
            height,
            format,
            rotation,
            bitmap_handle: handle,
            page_handle: page,
            is_rendered: false,
            bindings,
        }
    }

    /// Returns the width of the image in this PdfBitmap, in pixels.
    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height of the image in this PdfBitmap, in pixels.
    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns the pixel format of the image in this PdfBitmap.
    #[inline]
    pub fn format(&self) -> PdfBitmapFormat {
        self.format
    }

    /// Returns a new DynamicImage created from the bitmap buffer backing
    /// this PdfBitmap, rendering the referenced page if necessary.
    pub fn as_image(&mut self) -> DynamicImage {
        ImageBuffer::from_raw(
            self.width as u32,
            self.height as u32,
            self.as_bytes().to_owned(),
        )
        .map(DynamicImage::ImageBgra8)
        .unwrap()
    }

    /// Returns an immutable reference to the bitmap buffer backing this PdfBitmap,
    /// rendering the referenced page if necessary.
    pub fn as_bytes(&mut self) -> &'a [u8] {
        self.render();

        let buffer_length = self.bindings.FPDFBitmap_GetStride(self.bitmap_handle)
            * self.bindings.FPDFBitmap_GetHeight(self.bitmap_handle);

        let buffer_start = self.bindings.FPDFBitmap_GetBuffer(self.bitmap_handle);

        unsafe { std::slice::from_raw_parts(buffer_start as *const u8, buffer_length as usize) }
    }

    /// Renders this page into a bitmap buffer. Once rendered, the page will not be
    /// re-rendered.
    ///
    /// It is generally unnecessary to call this function directly, since PdfBitmap
    /// will automatically initiate rendering itself on the first call to either
    /// the [PdfBitmap::as_bytes()] function or the [PdfBitmap::as_image()] function.
    pub fn render(&mut self) {
        if self.is_rendered {
            // The page has already been rendered.

            return;
        }

        let width = self.width as i32;

        let height = self.height as i32;

        // Clear the bitmap buffer by setting every pixel to white.

        self.bindings
            .FPDFBitmap_FillRect(self.bitmap_handle, 0, 0, width, height, 0xFFFFFFFF);

        // Render the PDF page into the bitmap buffer.

        self.bindings.FPDF_RenderPageBitmap(
            self.bitmap_handle,
            *self.page_handle,
            0,
            0,
            width,
            height,
            match self.rotation {
                PdfBitmapRotation::None => 0,
                PdfBitmapRotation::Degrees90 => 1,
                PdfBitmapRotation::Degrees180 => 2,
                PdfBitmapRotation::Degrees270 => 3,
            },
            0,
        );

        self.is_rendered = true;
    }
}

impl<'a> Drop for PdfBitmap<'a> {
    /// Closes the PdfPage, releasing the memory held by the bitmap buffer.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDFBitmap_Destroy(self.bitmap_handle);
    }
}
