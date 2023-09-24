//! Defines the [PdfAppearanceMode] enum, which specifies the type of appearance stream
//! that should apply to a given `PdfPageAnnotation` or `PdfFormField` object.

use crate::bindgen::{
    FPDF_ANNOT_APPEARANCEMODE_DOWN, FPDF_ANNOT_APPEARANCEMODE_NORMAL,
    FPDF_ANNOT_APPEARANCEMODE_ROLLOVER,
};
use crate::error::PdfiumError;
use std::os::raw::c_int;

/// The type of appearance stream that should apply to a given `PdfPageAnnotation` or
/// `PdfFormField` object.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfAppearanceMode {
    Normal = FPDF_ANNOT_APPEARANCEMODE_NORMAL as isize,
    RollOver = FPDF_ANNOT_APPEARANCEMODE_ROLLOVER as isize,
    Down = FPDF_ANNOT_APPEARANCEMODE_DOWN as isize,
}

impl PdfAppearanceMode {
    #[allow(dead_code)] // We don't currently use the from_pdfium() function, but we expect to in future
    #[inline]
    pub(crate) fn from_pdfium(value: u32) -> Result<Self, PdfiumError> {
        match value {
            FPDF_ANNOT_APPEARANCEMODE_NORMAL => Ok(PdfAppearanceMode::Normal),
            FPDF_ANNOT_APPEARANCEMODE_ROLLOVER => Ok(PdfAppearanceMode::RollOver),
            FPDF_ANNOT_APPEARANCEMODE_DOWN => Ok(PdfAppearanceMode::Down),
            _ => Err(PdfiumError::UnknownAppearanceMode),
        }
    }

    #[inline]
    pub(crate) fn as_pdfium(&self) -> c_int {
        (match self {
            PdfAppearanceMode::Normal => FPDF_ANNOT_APPEARANCEMODE_NORMAL,
            PdfAppearanceMode::RollOver => FPDF_ANNOT_APPEARANCEMODE_ROLLOVER,
            PdfAppearanceMode::Down => FPDF_ANNOT_APPEARANCEMODE_DOWN,
        }) as c_int
    }
}
