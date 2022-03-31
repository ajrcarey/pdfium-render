//! Defines the [PdfAnnotation] struct, exposing functionality related to a single annotation.

use crate::bindgen::{
    FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE, FPDF_ANNOT_CARET, FPDF_ANNOT_CIRCLE,
    FPDF_ANNOT_FILEATTACHMENT, FPDF_ANNOT_FREETEXT, FPDF_ANNOT_HIGHLIGHT, FPDF_ANNOT_INK,
    FPDF_ANNOT_LINE, FPDF_ANNOT_LINK, FPDF_ANNOT_MOVIE, FPDF_ANNOT_POLYGON, FPDF_ANNOT_POLYLINE,
    FPDF_ANNOT_POPUP, FPDF_ANNOT_PRINTERMARK, FPDF_ANNOT_REDACT, FPDF_ANNOT_RICHMEDIA,
    FPDF_ANNOT_SCREEN, FPDF_ANNOT_SOUND, FPDF_ANNOT_SQUARE, FPDF_ANNOT_SQUIGGLY, FPDF_ANNOT_STAMP,
    FPDF_ANNOT_STRIKEOUT, FPDF_ANNOT_TEXT, FPDF_ANNOT_THREED, FPDF_ANNOT_TRAPNET,
    FPDF_ANNOT_UNDERLINE, FPDF_ANNOT_UNKNOWN, FPDF_ANNOT_WATERMARK, FPDF_ANNOT_WIDGET,
    FPDF_ANNOT_XFAWIDGET, FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::page::PdfRect;
use crate::page_annotations::PdfPageAnnotationIndex;

/// The type of a single [PdfAnnotation], as defined in table 8.20 of the PDF Reference,
/// version 1.7, on page 615.
///
/// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
/// currently support embedded sound or movie files, embedded 3D animations, or embedded
/// file attachments generally.
///
/// Pdfium currently supports creating, editing, and rendering the following types of annotations:
///
/// * [PdfAnnotationType::Circle]
/// * [PdfAnnotationType::FreeText]
/// * [PdfAnnotationType::Highlight]
/// * [PdfAnnotationType::Ink]
/// * [PdfAnnotationType::Link]
/// * [PdfAnnotationType::Popup]
/// * [PdfAnnotationType::Square]
/// * [PdfAnnotationType::Squiggly]
/// * [PdfAnnotationType::Stamp]
/// * [PdfAnnotationType::Strikeout]
/// * [PdfAnnotationType::Text]
/// * [PdfAnnotationType::Underline]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfAnnotationType {
    Unknown = FPDF_ANNOT_UNKNOWN as isize,
    Text = FPDF_ANNOT_TEXT as isize,
    Link = FPDF_ANNOT_LINK as isize,
    FreeText = FPDF_ANNOT_FREETEXT as isize,
    Line = FPDF_ANNOT_LINE as isize,
    Square = FPDF_ANNOT_SQUARE as isize,
    Circle = FPDF_ANNOT_CIRCLE as isize,
    Polygon = FPDF_ANNOT_POLYGON as isize,
    Polyline = FPDF_ANNOT_POLYLINE as isize,
    Highlight = FPDF_ANNOT_HIGHLIGHT as isize,
    Underline = FPDF_ANNOT_UNDERLINE as isize,
    Squiggly = FPDF_ANNOT_SQUIGGLY as isize,
    Strikeout = FPDF_ANNOT_STRIKEOUT as isize,
    Stamp = FPDF_ANNOT_STAMP as isize,
    Caret = FPDF_ANNOT_CARET as isize,
    Ink = FPDF_ANNOT_INK as isize,
    Popup = FPDF_ANNOT_POPUP as isize,
    FileAttachment = FPDF_ANNOT_FILEATTACHMENT as isize,
    Sound = FPDF_ANNOT_SOUND as isize,
    Movie = FPDF_ANNOT_MOVIE as isize,
    Widget = FPDF_ANNOT_WIDGET as isize,
    Screen = FPDF_ANNOT_SCREEN as isize,
    PrinterMark = FPDF_ANNOT_PRINTERMARK as isize,
    TrapNet = FPDF_ANNOT_TRAPNET as isize,
    Watermark = FPDF_ANNOT_WATERMARK as isize,
    ThreeD = FPDF_ANNOT_THREED as isize,
    RichMedia = FPDF_ANNOT_RICHMEDIA as isize,
    XfaWidget = FPDF_ANNOT_XFAWIDGET as isize,
    Redact = FPDF_ANNOT_REDACT as isize,
}

impl PdfAnnotationType {
    pub(crate) fn from_pdfium(
        value: FPDF_ANNOTATION_SUBTYPE,
    ) -> Result<PdfAnnotationType, PdfiumError> {
        match value as u32 {
            FPDF_ANNOT_UNKNOWN => Ok(PdfAnnotationType::Unknown),
            FPDF_ANNOT_TEXT => Ok(PdfAnnotationType::Text),
            FPDF_ANNOT_LINK => Ok(PdfAnnotationType::Link),
            FPDF_ANNOT_FREETEXT => Ok(PdfAnnotationType::FreeText),
            FPDF_ANNOT_LINE => Ok(PdfAnnotationType::Line),
            FPDF_ANNOT_SQUARE => Ok(PdfAnnotationType::Square),
            FPDF_ANNOT_CIRCLE => Ok(PdfAnnotationType::Circle),
            FPDF_ANNOT_POLYGON => Ok(PdfAnnotationType::Polygon),
            FPDF_ANNOT_POLYLINE => Ok(PdfAnnotationType::Polyline),
            FPDF_ANNOT_HIGHLIGHT => Ok(PdfAnnotationType::Highlight),
            FPDF_ANNOT_UNDERLINE => Ok(PdfAnnotationType::Underline),
            FPDF_ANNOT_SQUIGGLY => Ok(PdfAnnotationType::Squiggly),
            FPDF_ANNOT_STRIKEOUT => Ok(PdfAnnotationType::Strikeout),
            FPDF_ANNOT_STAMP => Ok(PdfAnnotationType::Stamp),
            FPDF_ANNOT_CARET => Ok(PdfAnnotationType::Caret),
            FPDF_ANNOT_INK => Ok(PdfAnnotationType::Ink),
            FPDF_ANNOT_POPUP => Ok(PdfAnnotationType::Popup),
            FPDF_ANNOT_FILEATTACHMENT => Ok(PdfAnnotationType::FileAttachment),
            FPDF_ANNOT_SOUND => Ok(PdfAnnotationType::Sound),
            FPDF_ANNOT_MOVIE => Ok(PdfAnnotationType::Movie),
            FPDF_ANNOT_WIDGET => Ok(PdfAnnotationType::Widget),
            FPDF_ANNOT_SCREEN => Ok(PdfAnnotationType::Screen),
            FPDF_ANNOT_PRINTERMARK => Ok(PdfAnnotationType::PrinterMark),
            FPDF_ANNOT_TRAPNET => Ok(PdfAnnotationType::TrapNet),
            FPDF_ANNOT_WATERMARK => Ok(PdfAnnotationType::Watermark),
            FPDF_ANNOT_THREED => Ok(PdfAnnotationType::ThreeD),
            FPDF_ANNOT_RICHMEDIA => Ok(PdfAnnotationType::RichMedia),
            FPDF_ANNOT_XFAWIDGET => Ok(PdfAnnotationType::XfaWidget),
            FPDF_ANNOT_REDACT => Ok(PdfAnnotationType::Redact),
            _ => Err(PdfiumError::UnknownPdfAnnotationType),
        }
    }

    pub(crate) fn as_pdfium(&self) -> FPDF_ANNOTATION_SUBTYPE {
        (match self {
            PdfAnnotationType::Unknown => FPDF_ANNOT_UNKNOWN,
            PdfAnnotationType::Text => FPDF_ANNOT_TEXT,
            PdfAnnotationType::Link => FPDF_ANNOT_LINK,
            PdfAnnotationType::FreeText => FPDF_ANNOT_FREETEXT,
            PdfAnnotationType::Line => FPDF_ANNOT_LINE,
            PdfAnnotationType::Square => FPDF_ANNOT_SQUARE,
            PdfAnnotationType::Circle => FPDF_ANNOT_CIRCLE,
            PdfAnnotationType::Polygon => FPDF_ANNOT_POLYGON,
            PdfAnnotationType::Polyline => FPDF_ANNOT_POLYLINE,
            PdfAnnotationType::Highlight => FPDF_ANNOT_HIGHLIGHT,
            PdfAnnotationType::Underline => FPDF_ANNOT_UNDERLINE,
            PdfAnnotationType::Squiggly => FPDF_ANNOT_SQUIGGLY,
            PdfAnnotationType::Strikeout => FPDF_ANNOT_STRIKEOUT,
            PdfAnnotationType::Stamp => FPDF_ANNOT_STAMP,
            PdfAnnotationType::Caret => FPDF_ANNOT_CARET,
            PdfAnnotationType::Ink => FPDF_ANNOT_INK,
            PdfAnnotationType::Popup => FPDF_ANNOT_POPUP,
            PdfAnnotationType::FileAttachment => FPDF_ANNOT_FILEATTACHMENT,
            PdfAnnotationType::Sound => FPDF_ANNOT_SOUND,
            PdfAnnotationType::Movie => FPDF_ANNOT_MOVIE,
            PdfAnnotationType::Widget => FPDF_ANNOT_WIDGET,
            PdfAnnotationType::Screen => FPDF_ANNOT_SCREEN,
            PdfAnnotationType::PrinterMark => FPDF_ANNOT_PRINTERMARK,
            PdfAnnotationType::TrapNet => FPDF_ANNOT_TRAPNET,
            PdfAnnotationType::Watermark => FPDF_ANNOT_WATERMARK,
            PdfAnnotationType::ThreeD => FPDF_ANNOT_THREED,
            PdfAnnotationType::RichMedia => FPDF_ANNOT_RICHMEDIA,
            PdfAnnotationType::XfaWidget => FPDF_ANNOT_XFAWIDGET,
            PdfAnnotationType::Redact => FPDF_ANNOT_REDACT,
        }) as FPDF_ANNOTATION_SUBTYPE
    }
}

pub struct PdfAnnotation<'a> {
    index: PdfPageAnnotationIndex,
    handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfAnnotation<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        index: PdfPageAnnotationIndex,
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfAnnotation {
            index,
            handle,
            bindings,
        }
    }

    /// Returns the internal FPDF_ANNOTATION handle for this [PdfAnnotation].
    #[inline]
    pub(crate) fn get_handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }
}

/// Functionality common to all [PdfAnnotation] objects, regardless of their [PdfAnnotationType].
pub trait PdfAnnotationCommon {
    /// Returns the zero-based page index of this [PdfAnnotation] in its containing
    /// `PdfPageAnnotations` collection.
    fn index(&self) -> PdfPageAnnotationIndex;

    /// Returns the [PdfAnnotationType] of this [PdfAnnotation].
    fn annotation_type(&self) -> PdfAnnotationType;

    /// Returns `true` if Pdfium supports creating, editing, and rendering this type of
    /// [PdfAnnotation].
    ///
    /// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
    /// currently support embedded sound or movie files, embedded 3D animations, or embedded
    /// file attachments generally.
    ///
    /// Pdfium currently supports creating, editing, and rendering the following types of annotations:
    ///
    /// * [PdfAnnotationType::Circle]
    /// * [PdfAnnotationType::FreeText]
    /// * [PdfAnnotationType::Highlight]
    /// * [PdfAnnotationType::Ink]
    /// * [PdfAnnotationType::Link]
    /// * [PdfAnnotationType::Popup]
    /// * [PdfAnnotationType::Square]
    /// * [PdfAnnotationType::Squiggly]
    /// * [PdfAnnotationType::Stamp]
    /// * [PdfAnnotationType::Strikeout]
    /// * [PdfAnnotationType::Text]
    /// * [PdfAnnotationType::Underline]
    fn is_supported(&self) -> bool;

    /// Returns `true` if Pdfium does _not_ support creating, editing, and rendering this type of
    /// [PdfAnnotation].
    ///
    /// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
    /// currently support embedded sound or movie files, embedded 3D animations, or embedded
    /// file attachments generally.
    ///
    /// Pdfium currently supports creating, editing, and rendering the following types of annotations:
    ///
    /// * [PdfAnnotationType::Circle]
    /// * [PdfAnnotationType::FreeText]
    /// * [PdfAnnotationType::Highlight]
    /// * [PdfAnnotationType::Ink]
    /// * [PdfAnnotationType::Link]
    /// * [PdfAnnotationType::Popup]
    /// * [PdfAnnotationType::Square]
    /// * [PdfAnnotationType::Squiggly]
    /// * [PdfAnnotationType::Stamp]
    /// * [PdfAnnotationType::Strikeout]
    /// * [PdfAnnotationType::Text]
    /// * [PdfAnnotationType::Underline]
    #[inline]
    fn is_unsupported(&self) -> bool {
        !self.is_supported()
    }

    /// Returns the bounding box of this [PdfAnnotation].
    fn bounds(&self) -> Result<PdfRect, PdfiumError>;
}

impl<'a> PdfAnnotationCommon for PdfAnnotation<'a> {
    #[inline]
    fn index(&self) -> PdfPageAnnotationIndex {
        self.index
    }

    #[inline]
    fn annotation_type(&self) -> PdfAnnotationType {
        PdfAnnotationType::from_pdfium(self.bindings.FPDFAnnot_GetSubtype(self.handle))
            .unwrap_or(PdfAnnotationType::Unknown)
    }

    #[inline]
    fn is_supported(&self) -> bool {
        self.bindings.is_true(
            self.bindings
                .FPDFAnnot_IsSupportedSubtype(self.annotation_type().as_pdfium()),
        )
    }

    fn bounds(&self) -> Result<PdfRect, PdfiumError> {
        let mut rect = FS_RECTF {
            left: 0_f32,
            bottom: 0_f32,
            right: 0_f32,
            top: 0_f32,
        };

        let result = self
            .bindings
            .FPDFAnnot_GetRect(*self.get_handle(), &mut rect);

        PdfRect::from_pdfium_as_result(result, rect, self.bindings)
    }
}

impl<'a> Drop for PdfAnnotation<'a> {
    /// Closes the [PdfAnnotation], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.bindings.FPDFPage_CloseAnnot(self.handle);
    }
}
