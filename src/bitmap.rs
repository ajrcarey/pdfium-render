//! Defines the [PdfBitmap] struct, a lazily-generated bitmap rendering of a single
//! `PdfPage`.

use crate::bindgen::{
    FPDFBitmap_BGR, FPDFBitmap_BGRA, FPDFBitmap_BGRx, FPDFBitmap_Gray, FPDFBitmap_Unknown,
    FPDF_BITMAP,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::bitmap_config::PdfBitmapRenderSettings;
use crate::color::PdfColor;
use crate::document::PdfDocument;
use crate::error::PdfiumError;
use crate::page::PdfPage;
use image::{DynamicImage, ImageBuffer};

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

/// A rotation transformation that should be applied to a [PdfPage] when it is rendered
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
}

/// A rendered image of a single [PdfPage] at a specific width and height.
///
/// By default, [PdfBitmap] is lazy; it will not render its page into a bitmap until
/// it is required to do so in order to return a byte buffer or an `Image::DynamicImage`.
/// If preferred, rendering can be initiated manually by calling the [PdfBitmap::render()] function.
/// Once rendered, the page will not be re-rendered.
pub struct PdfBitmap<'a> {
    handle: FPDF_BITMAP,
    config: PdfBitmapRenderSettings,
    is_rendered: bool,
    page: &'a PdfPage<'a>,
    document: &'a PdfDocument<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfBitmap<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_BITMAP,
        config: PdfBitmapRenderSettings,
        page: &'a PdfPage,
        document: &'a PdfDocument,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfBitmap {
            config,
            handle,
            is_rendered: false,
            page,
            document,
            bindings,
        }
    }

    /// Returns the width of the image in this PdfBitmap, in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.config.width as u32
    }

    /// Returns the height of the image in this PdfBitmap, in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.config.height as u32
    }

    /// Returns the pixel format of the image in this PdfBitmap.
    #[inline]
    pub fn format(&self) -> PdfBitmapFormat {
        PdfBitmapFormat::from_pdfium(self.config.format as u32).unwrap()
    }

    /// Returns the rotation setting that will be applied to this page during rendering.
    #[inline]
    pub fn rotation(&self) -> PdfBitmapRotation {
        PdfBitmapRotation::from_pdfium(self.config.rotate).unwrap()
    }

    /// Returns a new DynamicImage created from the bitmap buffer backing
    /// this PdfBitmap, rendering the referenced page if necessary.
    pub fn as_image(&mut self) -> DynamicImage {
        ImageBuffer::from_raw(
            self.config.width as u32,
            self.config.height as u32,
            self.as_bytes().to_owned(),
        )
        .map(DynamicImage::ImageBgra8)
        .unwrap()
    }

    /// Returns an immutable reference to the bitmap buffer backing this PdfBitmap,
    /// rendering the referenced page if necessary.
    pub fn as_bytes(&mut self) -> &'a [u8] {
        self.render();

        let buffer_length = self.bindings.FPDFBitmap_GetStride(self.handle)
            * self.bindings.FPDFBitmap_GetHeight(self.handle);

        let buffer_start = self.bindings.FPDFBitmap_GetBuffer(self.handle);

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

        // Clear the bitmap buffer by setting every pixel to white.

        self.bindings.FPDFBitmap_FillRect(
            self.handle,
            0,
            0,
            self.config.width,
            self.config.height,
            PdfColor::SOLID_WHITE.as_pdfium_color(),
        );

        // Render the PDF page into the bitmap buffer.

        self.bindings.FPDF_RenderPageBitmap(
            self.handle,
            *self.page.get_handle(),
            0,
            0,
            self.config.width,
            self.config.height,
            self.config.rotate,
            self.config.render_flags,
        );

        if let Some(form) = self.document.form() {
            if self.config.do_render_form_data {
                // Render user-supplied form data, if any, as an overlay on top of the page.

                for (form_field_type, (color, alpha)) in self.config.form_field_highlight.iter() {
                    self.bindings.FPDF_SetFormFieldHighlightColor(
                        *form.get_handle(),
                        *form_field_type,
                        *color,
                    );

                    self.bindings
                        .FPDF_SetFormFieldHighlightAlpha(*form.get_handle(), *alpha);
                }

                self.bindings.FPDF_FFLDraw(
                    *form.get_handle(),
                    self.handle,
                    *self.page.get_handle(),
                    0,
                    0,
                    self.config.width,
                    self.config.height,
                    self.config.rotate,
                    self.config.render_flags,
                );
            }
        }

        self.is_rendered = true;
    }
}

impl<'a> Drop for PdfBitmap<'a> {
    /// Closes the PdfPage, releasing the memory held by the bitmap buffer.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDFBitmap_Destroy(self.handle);
    }
}
