use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::form::PdfForm;
use crate::pdf::document::page::annotation::private::internal::PdfPageAnnotationPrivate;
use crate::pdf::points::PdfPoints;
use std::ffi::c_float;

#[cfg(any(
    feature = "pdfium_future",
    feature = "pdfium_7215",
    feature = "pdfium_7123",
    feature = "pdfium_6996",
    feature = "pdfium_6721",
    feature = "pdfium_6666",
    feature = "pdfium_6611",
    feature = "pdfium_6569",
    feature = "pdfium_6555",
))]
use {crate::pdf::color::PdfColor, std::ffi::c_uint};

#[cfg(doc)]
use crate::pdf::document::page::annotation::PdfPageAnnotation;

/// The form of justification that should be used when displaying the text assigned
/// to a [PdfPageAnnotation] that supports variable text.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfPageAnnotationVariableTextJustification {
    LeftJustified,
    Centered,
    RightJustified,
}

impl PdfPageAnnotationVariableTextJustification {
    #[inline]
    pub(crate) fn from_pdfium(value: i32) -> Result<Self, PdfiumError> {
        match value {
            0 => Ok(PdfPageAnnotationVariableTextJustification::LeftJustified),
            1 => Ok(PdfPageAnnotationVariableTextJustification::Centered),
            2 => Ok(PdfPageAnnotationVariableTextJustification::RightJustified),
            _ => Err(PdfiumError::UnknownPageAnnotationVariableTextJustificationType),
        }
    }
}

/// Text-handling functions common to all [PdfPageAnnotation] types that
/// support custom text.
pub trait PdfPageAnnotationVariableText<'a> {
    /// Returns the size of the text in this annotation. A value of [PdfPoints::ZERO]
    /// indicates that the font size is determined automatically from the annotation height.
    /// See also the [PdfPageAnnotationVariableText::is_font_auto_sized()] function.
    fn font_size(&self, form: &PdfForm) -> Result<PdfPoints, PdfiumError>;

    /// Returns `true` if the font size for this annotation is determined automatically
    /// from the annotation height.
    fn is_font_auto_sized(&self, form: &PdfForm) -> bool;

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7215",
        feature = "pdfium_7123",
        feature = "pdfium_6996",
        feature = "pdfium_6721",
        feature = "pdfium_6666",
        feature = "pdfium_6611",
        feature = "pdfium_6569",
        feature = "pdfium_6555",
    ))]
    /// Returns the color of the text in this annotation.
    fn font_color(&self, form: &PdfForm) -> Result<PdfColor, PdfiumError>;

    #[cfg(feature = "pdfium_future")]
    /// Sets the color of the text in this annotation.
    fn set_font_color(&mut self, form: &PdfForm, color: PdfColor) -> Result<(), PdfiumError>;

    /// Returns the form of justification that should be used when displaying the text
    /// assigned to this annotation.
    fn justification(&self) -> Result<PdfPageAnnotationVariableTextJustification, PdfiumError>;

    /// Returns the rich text string assigned to this annotation, if any.
    ///
    /// Rich text support was added in PDF version 1.5.
    fn rich_text(&self) -> Option<String>;
}

impl<'a, T> PdfPageAnnotationVariableText<'a> for T
where
    T: PdfPageAnnotationPrivate<'a>,
{
    fn font_size(&self, form: &PdfForm) -> Result<PdfPoints, PdfiumError> {
        let mut value: c_float = 0.0;

        if self
            .bindings()
            .is_true(self.bindings().FPDFAnnot_GetFontSize(
                form.handle(),
                self.handle(),
                &mut value,
            ))
        {
            Ok(PdfPoints::new(value))
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    #[inline]
    fn is_font_auto_sized(&self, form: &PdfForm) -> bool {
        match self.font_size(form) {
            Ok(size) => size.value == 0.0,
            _ => false,
        }
    }

    #[cfg(any(
        feature = "pdfium_future",
        feature = "pdfium_7215",
        feature = "pdfium_7123",
        feature = "pdfium_6996",
        feature = "pdfium_6721",
        feature = "pdfium_6666",
        feature = "pdfium_6611",
        feature = "pdfium_6569",
        feature = "pdfium_6555",
    ))]
    fn font_color(&self, form: &PdfForm) -> Result<PdfColor, PdfiumError> {
        let mut red: c_uint = 0;
        let mut green: c_uint = 0;
        let mut blue: c_uint = 0;

        if self
            .bindings()
            .is_true(self.bindings().FPDFAnnot_GetFontColor(
                form.handle(),
                self.handle(),
                &mut red,
                &mut green,
                &mut blue,
            ))
        {
            Ok(PdfColor::new(red as u8, green as u8, blue as u8, 255))
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    #[cfg(feature = "pdfium_future")]
    fn set_font_color(&mut self, form: &PdfForm, color: PdfColor) -> Result<(), PdfiumError> {
        if self
            .bindings()
            .is_true(self.bindings().FPDFAnnot_SetFontColor(
                form.handle(),
                self.handle(),
                color.red() as c_uint,
                color.green() as c_uint,
                color.blue() as c_uint,
            ))
        {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    fn justification(&self) -> Result<PdfPageAnnotationVariableTextJustification, PdfiumError> {
        let mut value: c_float = 0.0;

        if self
            .bindings()
            .is_true(
                self.bindings()
                    .FPDFAnnot_GetNumberValue(self.handle(), "Q", &mut value),
            )
        {
            PdfPageAnnotationVariableTextJustification::from_pdfium(value as i32)
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    #[inline]
    fn rich_text(&self) -> Option<String> {
        self.get_string_value("RV")
    }
}
