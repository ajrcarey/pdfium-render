//! Defines the [PdfColor] struct, a 32-bit RGB color value with an optional alpha channel.

use crate::bindgen::FPDF_DWORD;

/// A 32-bit RGB color value with an optional alpha channel.
///
/// Certain basic primary colors are available as const values on this struct.
///
/// Note that when used as a form field highlight color, a solid color with no opacity
/// will overprint any user data in the field. Use the [PdfColor::with_alpha()] function
/// to apply an alpha channel value to an existing [PdfColor].
#[derive(Debug, Copy, Clone)]
pub struct PdfColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl PdfColor {
    pub const SOLID_WHITE: PdfColor = PdfColor::new(255, 255, 255, 255);
    pub const SOLID_RED: PdfColor = PdfColor::new(255, 0, 0, 255);
    pub const SOLID_GREEN: PdfColor = PdfColor::new(0, 255, 0, 255);
    pub const SOLID_BLUE: PdfColor = PdfColor::new(0, 0, 255, 255);
    pub const SOLID_CYAN: PdfColor = PdfColor::new(0, 255, 255, 255);
    pub const SOLID_YELLOW: PdfColor = PdfColor::new(255, 255, 0, 255);

    #[inline]
    #[allow(dead_code)]
    // The from_pdfium() function is not currently used, but we expect it to be in future
    pub(crate) const fn from_pdfium(color: u64) -> Self {
        Self::from_pdfium_with_alpha(color, 255)
    }

    #[inline]
    #[allow(dead_code)]
    // The from_pdfium_with_alpha() function is not currently used, but we expect it to be in future
    pub(crate) const fn from_pdfium_with_alpha(color: u64, alpha: u8) -> Self {
        Self::new(
            ((color & 0x00FF0000) >> 16) as u8,
            ((color & 0x0000FF00) >> 8) as u8,
            (color & 0x000000FF) as u8,
            alpha,
        )
    }

    /// Returns this color encoded as a 32-bit value, suitable for passing to Pdfium.
    #[inline]
    pub(crate) fn as_pdfium_color(&self) -> FPDF_DWORD {
        self.color()
    }

    /// Returns a tuplet comprising this color encoded as a 32-bit value and this alpha
    /// encoded as an 8-bit value, suitable for passing to Pdfium.
    #[inline]
    pub(crate) fn as_pdfium_color_with_alpha(&self) -> (FPDF_DWORD, u8) {
        (self.color(), self.alpha() as u8)
    }

    /// Constructs a new [PdfColor] object from the given arguments.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    /// Constructs a new [PdfColor] by copying the red, green, and blue color components
    /// of this color and applying the given alpha value.
    #[inline]
    pub const fn with_alpha(&self, alpha: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    /// Returns the red component of this color.
    #[inline]
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Returns the green component of this color.
    #[inline]
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Returns the blue component of this color.
    #[inline]
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// Returns this color as a decimal value equivalent to 32-bit hexadecimal 0xFFRRGGBB.
    #[inline]
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn color(&self) -> FPDF_DWORD {
        let (f, r, g, b) = self.color_components();

        ((f << 24) | (r << 16) | (g << 8) | b) as FPDF_DWORD
    }

    /// Returns this color as a decimal value equivalent to 32-bit hexadecimal 0xFFRRGGBB.
    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn color(&self) -> FPDF_DWORD {
        let (f, r, g, b) = self.color_components();

        ((f << 24) | (b << 16) | (g << 8) | r) as FPDF_DWORD
    }

    /// Returns the raw color components of this [PdfColor].
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    fn color_components(&self) -> (FPDF_DWORD, FPDF_DWORD, FPDF_DWORD, FPDF_DWORD) {
        (
            0xFF as FPDF_DWORD,
            self.r as FPDF_DWORD,
            self.g as FPDF_DWORD,
            self.b as FPDF_DWORD,
        )
    }

    /// Returns the alpha (opacity) component of this color, with 0 = completely transparent
    /// and 255 = completely opaque (solid).
    #[inline]
    pub fn alpha(&self) -> u8 {
        self.a
    }
}
