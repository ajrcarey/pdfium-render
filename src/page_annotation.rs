//! Defines the [PdfPageAnnotation] struct, exposing functionality related to a single annotation.

use crate::bindgen::{
    FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE, FPDF_ANNOT_CARET, FPDF_ANNOT_CIRCLE,
    FPDF_ANNOT_FILEATTACHMENT, FPDF_ANNOT_FREETEXT, FPDF_ANNOT_HIGHLIGHT, FPDF_ANNOT_INK,
    FPDF_ANNOT_LINE, FPDF_ANNOT_LINK, FPDF_ANNOT_MOVIE, FPDF_ANNOT_POLYGON, FPDF_ANNOT_POLYLINE,
    FPDF_ANNOT_POPUP, FPDF_ANNOT_PRINTERMARK, FPDF_ANNOT_REDACT, FPDF_ANNOT_RICHMEDIA,
    FPDF_ANNOT_SCREEN, FPDF_ANNOT_SOUND, FPDF_ANNOT_SQUARE, FPDF_ANNOT_SQUIGGLY, FPDF_ANNOT_STAMP,
    FPDF_ANNOT_STRIKEOUT, FPDF_ANNOT_TEXT, FPDF_ANNOT_THREED, FPDF_ANNOT_TRAPNET,
    FPDF_ANNOT_UNDERLINE, FPDF_ANNOT_UNKNOWN, FPDF_ANNOT_WATERMARK, FPDF_ANNOT_WIDGET,
    FPDF_ANNOT_XFAWIDGET,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::page::PdfRect;
use crate::page_annotation_circle::PdfPageCircleAnnotation;
use crate::page_annotation_free_text::PdfPageFreeTextAnnotation;
use crate::page_annotation_highlight::PdfPageHighlightAnnotation;
use crate::page_annotation_ink::PdfPageInkAnnotation;
use crate::page_annotation_link::PdfPageLinkAnnotation;
use crate::page_annotation_popup::PdfPagePopupAnnotation;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;
use crate::page_annotation_square::PdfPageSquareAnnotation;
use crate::page_annotation_squiggly::PdfPageSquigglyAnnotation;
use crate::page_annotation_stamp::PdfPageStampAnnotation;
use crate::page_annotation_strikeout::PdfPageStrikeoutAnnotation;
use crate::page_annotation_text::PdfPageTextAnnotation;
use crate::page_annotation_underline::PdfPageUnderlineAnnotation;
use crate::page_annotation_unsupported::PdfPageUnsupportedAnnotation;

/// The type of a single [PdfPageAnnotation], as defined in table 8.20 of the PDF Reference,
/// version 1.7, on page 615.
///
/// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
/// currently support embedded sound or movie files, embedded 3D animations, or embedded
/// file attachments generally.
///
/// Pdfium currently supports creating, editing, and rendering the following types of annotations:
///
/// * [PdfPageAnnotationType::Circle]
/// * [PdfPageAnnotationType::FreeText]
/// * [PdfPageAnnotationType::Highlight]
/// * [PdfPageAnnotationType::Ink]
/// * [PdfPageAnnotationType::Link]
/// * [PdfPageAnnotationType::Popup]
/// * [PdfPageAnnotationType::Square]
/// * [PdfPageAnnotationType::Squiggly]
/// * [PdfPageAnnotationType::Stamp]
/// * [PdfPageAnnotationType::Strikeout]
/// * [PdfPageAnnotationType::Text]
/// * [PdfPageAnnotationType::Underline]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfPageAnnotationType {
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

impl PdfPageAnnotationType {
    pub(crate) fn from_pdfium(
        value: FPDF_ANNOTATION_SUBTYPE,
    ) -> Result<PdfPageAnnotationType, PdfiumError> {
        match value as u32 {
            FPDF_ANNOT_UNKNOWN => Ok(PdfPageAnnotationType::Unknown),
            FPDF_ANNOT_TEXT => Ok(PdfPageAnnotationType::Text),
            FPDF_ANNOT_LINK => Ok(PdfPageAnnotationType::Link),
            FPDF_ANNOT_FREETEXT => Ok(PdfPageAnnotationType::FreeText),
            FPDF_ANNOT_LINE => Ok(PdfPageAnnotationType::Line),
            FPDF_ANNOT_SQUARE => Ok(PdfPageAnnotationType::Square),
            FPDF_ANNOT_CIRCLE => Ok(PdfPageAnnotationType::Circle),
            FPDF_ANNOT_POLYGON => Ok(PdfPageAnnotationType::Polygon),
            FPDF_ANNOT_POLYLINE => Ok(PdfPageAnnotationType::Polyline),
            FPDF_ANNOT_HIGHLIGHT => Ok(PdfPageAnnotationType::Highlight),
            FPDF_ANNOT_UNDERLINE => Ok(PdfPageAnnotationType::Underline),
            FPDF_ANNOT_SQUIGGLY => Ok(PdfPageAnnotationType::Squiggly),
            FPDF_ANNOT_STRIKEOUT => Ok(PdfPageAnnotationType::Strikeout),
            FPDF_ANNOT_STAMP => Ok(PdfPageAnnotationType::Stamp),
            FPDF_ANNOT_CARET => Ok(PdfPageAnnotationType::Caret),
            FPDF_ANNOT_INK => Ok(PdfPageAnnotationType::Ink),
            FPDF_ANNOT_POPUP => Ok(PdfPageAnnotationType::Popup),
            FPDF_ANNOT_FILEATTACHMENT => Ok(PdfPageAnnotationType::FileAttachment),
            FPDF_ANNOT_SOUND => Ok(PdfPageAnnotationType::Sound),
            FPDF_ANNOT_MOVIE => Ok(PdfPageAnnotationType::Movie),
            FPDF_ANNOT_WIDGET => Ok(PdfPageAnnotationType::Widget),
            FPDF_ANNOT_SCREEN => Ok(PdfPageAnnotationType::Screen),
            FPDF_ANNOT_PRINTERMARK => Ok(PdfPageAnnotationType::PrinterMark),
            FPDF_ANNOT_TRAPNET => Ok(PdfPageAnnotationType::TrapNet),
            FPDF_ANNOT_WATERMARK => Ok(PdfPageAnnotationType::Watermark),
            FPDF_ANNOT_THREED => Ok(PdfPageAnnotationType::ThreeD),
            FPDF_ANNOT_RICHMEDIA => Ok(PdfPageAnnotationType::RichMedia),
            FPDF_ANNOT_XFAWIDGET => Ok(PdfPageAnnotationType::XfaWidget),
            FPDF_ANNOT_REDACT => Ok(PdfPageAnnotationType::Redact),
            _ => Err(PdfiumError::UnknownPdfAnnotationType),
        }
    }

    #[allow(dead_code)]
    // The as_pdfium() function is not currently used, but we expect it to be in future
    pub(crate) fn as_pdfium(&self) -> FPDF_ANNOTATION_SUBTYPE {
        (match self {
            PdfPageAnnotationType::Unknown => FPDF_ANNOT_UNKNOWN,
            PdfPageAnnotationType::Text => FPDF_ANNOT_TEXT,
            PdfPageAnnotationType::Link => FPDF_ANNOT_LINK,
            PdfPageAnnotationType::FreeText => FPDF_ANNOT_FREETEXT,
            PdfPageAnnotationType::Line => FPDF_ANNOT_LINE,
            PdfPageAnnotationType::Square => FPDF_ANNOT_SQUARE,
            PdfPageAnnotationType::Circle => FPDF_ANNOT_CIRCLE,
            PdfPageAnnotationType::Polygon => FPDF_ANNOT_POLYGON,
            PdfPageAnnotationType::Polyline => FPDF_ANNOT_POLYLINE,
            PdfPageAnnotationType::Highlight => FPDF_ANNOT_HIGHLIGHT,
            PdfPageAnnotationType::Underline => FPDF_ANNOT_UNDERLINE,
            PdfPageAnnotationType::Squiggly => FPDF_ANNOT_SQUIGGLY,
            PdfPageAnnotationType::Strikeout => FPDF_ANNOT_STRIKEOUT,
            PdfPageAnnotationType::Stamp => FPDF_ANNOT_STAMP,
            PdfPageAnnotationType::Caret => FPDF_ANNOT_CARET,
            PdfPageAnnotationType::Ink => FPDF_ANNOT_INK,
            PdfPageAnnotationType::Popup => FPDF_ANNOT_POPUP,
            PdfPageAnnotationType::FileAttachment => FPDF_ANNOT_FILEATTACHMENT,
            PdfPageAnnotationType::Sound => FPDF_ANNOT_SOUND,
            PdfPageAnnotationType::Movie => FPDF_ANNOT_MOVIE,
            PdfPageAnnotationType::Widget => FPDF_ANNOT_WIDGET,
            PdfPageAnnotationType::Screen => FPDF_ANNOT_SCREEN,
            PdfPageAnnotationType::PrinterMark => FPDF_ANNOT_PRINTERMARK,
            PdfPageAnnotationType::TrapNet => FPDF_ANNOT_TRAPNET,
            PdfPageAnnotationType::Watermark => FPDF_ANNOT_WATERMARK,
            PdfPageAnnotationType::ThreeD => FPDF_ANNOT_THREED,
            PdfPageAnnotationType::RichMedia => FPDF_ANNOT_RICHMEDIA,
            PdfPageAnnotationType::XfaWidget => FPDF_ANNOT_XFAWIDGET,
            PdfPageAnnotationType::Redact => FPDF_ANNOT_REDACT,
        }) as FPDF_ANNOTATION_SUBTYPE
    }
}

/// A single user annotation on a `PdfPage`.
pub enum PdfPageAnnotation<'a> {
    Circle(PdfPageCircleAnnotation<'a>),
    FreeText(PdfPageFreeTextAnnotation<'a>),
    Highlight(PdfPageHighlightAnnotation<'a>),
    Ink(PdfPageInkAnnotation<'a>),
    Link(PdfPageLinkAnnotation<'a>),
    Popup(PdfPagePopupAnnotation<'a>),
    Square(PdfPageSquareAnnotation<'a>),
    Squiggly(PdfPageSquigglyAnnotation<'a>),
    Stamp(PdfPageStampAnnotation<'a>),
    Strikeout(PdfPageStrikeoutAnnotation<'a>),
    Text(PdfPageTextAnnotation<'a>),
    Underline(PdfPageUnderlineAnnotation<'a>),

    /// Common properties shared by all [PdfPageAnnotation] types can still be accessed for
    /// annotations not supported by Pdfium, but annotation-specific functionality
    /// will be unavailable.
    Unsupported(PdfPageUnsupportedAnnotation<'a>),
}

impl<'a> PdfPageAnnotation<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        let annotation_type =
            PdfPageAnnotationType::from_pdfium(bindings.FPDFAnnot_GetSubtype(handle))
                .unwrap_or(PdfPageAnnotationType::Unknown);

        match annotation_type {
            PdfPageAnnotationType::Circle => {
                PdfPageAnnotation::Circle(PdfPageCircleAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::FreeText => PdfPageAnnotation::FreeText(
                PdfPageFreeTextAnnotation::from_pdfium(handle, bindings),
            ),
            PdfPageAnnotationType::Highlight => PdfPageAnnotation::Highlight(
                PdfPageHighlightAnnotation::from_pdfium(handle, bindings),
            ),
            PdfPageAnnotationType::Ink => {
                PdfPageAnnotation::Ink(PdfPageInkAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::Link => {
                PdfPageAnnotation::Link(PdfPageLinkAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::Popup => {
                PdfPageAnnotation::Popup(PdfPagePopupAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::Square => {
                PdfPageAnnotation::Square(PdfPageSquareAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::Squiggly => PdfPageAnnotation::Squiggly(
                PdfPageSquigglyAnnotation::from_pdfium(handle, bindings),
            ),
            PdfPageAnnotationType::Stamp => {
                PdfPageAnnotation::Stamp(PdfPageStampAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::Strikeout => PdfPageAnnotation::Strikeout(
                PdfPageStrikeoutAnnotation::from_pdfium(handle, bindings),
            ),
            PdfPageAnnotationType::Text => {
                PdfPageAnnotation::Text(PdfPageTextAnnotation::from_pdfium(handle, bindings))
            }
            PdfPageAnnotationType::Underline => PdfPageAnnotation::Underline(
                PdfPageUnderlineAnnotation::from_pdfium(handle, bindings),
            ),
            _ => PdfPageAnnotation::Unsupported(PdfPageUnsupportedAnnotation::from_pdfium(
                annotation_type,
                handle,
                bindings,
            )),
        }
    }

    #[inline]
    pub(crate) fn unwrap_as_trait(&self) -> &dyn PdfPageAnnotationPrivate {
        match self {
            PdfPageAnnotation::Circle(annotation) => annotation,
            PdfPageAnnotation::FreeText(annotation) => annotation,
            PdfPageAnnotation::Highlight(annotation) => annotation,
            PdfPageAnnotation::Ink(annotation) => annotation,
            PdfPageAnnotation::Link(annotation) => annotation,
            PdfPageAnnotation::Popup(annotation) => annotation,
            PdfPageAnnotation::Square(annotation) => annotation,
            PdfPageAnnotation::Squiggly(annotation) => annotation,
            PdfPageAnnotation::Stamp(annotation) => annotation,
            PdfPageAnnotation::Strikeout(annotation) => annotation,
            PdfPageAnnotation::Text(annotation) => annotation,
            PdfPageAnnotation::Underline(annotation) => annotation,
            PdfPageAnnotation::Unsupported(annotation) => annotation,
        }
    }

    /// The type of this [PdfPageAnnotation].
    ///
    /// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
    /// currently support embedded sound or movie files, embedded 3D animations, or embedded
    /// file attachments generally.
    ///
    /// Pdfium currently supports creating, editing, and rendering the following types of annotations:
    ///
    /// * [PdfPageAnnotationType::Circle]
    /// * [PdfPageAnnotationType::FreeText]
    /// * [PdfPageAnnotationType::Highlight]
    /// * [PdfPageAnnotationType::Ink]
    /// * [PdfPageAnnotationType::Link]
    /// * [PdfPageAnnotationType::Popup]
    /// * [PdfPageAnnotationType::Square]
    /// * [PdfPageAnnotationType::Squiggly]
    /// * [PdfPageAnnotationType::Stamp]
    /// * [PdfPageAnnotationType::Strikeout]
    /// * [PdfPageAnnotationType::Text]
    /// * [PdfPageAnnotationType::Underline]
    #[inline]
    pub fn annotation_type(&self) -> PdfPageAnnotationType {
        match self {
            PdfPageAnnotation::Circle(_) => PdfPageAnnotationType::Circle,
            PdfPageAnnotation::FreeText(_) => PdfPageAnnotationType::FreeText,
            PdfPageAnnotation::Highlight(_) => PdfPageAnnotationType::Highlight,
            PdfPageAnnotation::Ink(_) => PdfPageAnnotationType::Ink,
            PdfPageAnnotation::Link(_) => PdfPageAnnotationType::Link,
            PdfPageAnnotation::Popup(_) => PdfPageAnnotationType::Popup,
            PdfPageAnnotation::Square(_) => PdfPageAnnotationType::Square,
            PdfPageAnnotation::Squiggly(_) => PdfPageAnnotationType::Squiggly,
            PdfPageAnnotation::Stamp(_) => PdfPageAnnotationType::Stamp,
            PdfPageAnnotation::Strikeout(_) => PdfPageAnnotationType::Strikeout,
            PdfPageAnnotation::Text(_) => PdfPageAnnotationType::Text,
            PdfPageAnnotation::Underline(_) => PdfPageAnnotationType::Underline,
            PdfPageAnnotation::Unsupported(annotation) => annotation.get_type(),
        }
    }

    /// Returns `true` if Pdfium supports creating, editing, and rendering this type of
    /// [PdfPageAnnotation].
    ///
    /// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
    /// currently support embedded sound or movie files, embedded 3D animations, or embedded
    /// file attachments generally.
    ///
    /// Pdfium currently supports creating, editing, and rendering the following types of annotations:
    ///
    /// * [PdfPageAnnotationType::Circle]
    /// * [PdfPageAnnotationType::FreeText]
    /// * [PdfPageAnnotationType::Highlight]
    /// * [PdfPageAnnotationType::Ink]
    /// * [PdfPageAnnotationType::Link]
    /// * [PdfPageAnnotationType::Popup]
    /// * [PdfPageAnnotationType::Square]
    /// * [PdfPageAnnotationType::Squiggly]
    /// * [PdfPageAnnotationType::Stamp]
    /// * [PdfPageAnnotationType::Strikeout]
    /// * [PdfPageAnnotationType::Text]
    /// * [PdfPageAnnotationType::Underline]
    #[inline]
    pub fn is_supported(&self) -> bool {
        !self.is_unsupported()
    }

    /// Returns `true` if Pdfium does _not_ support creating, editing, and rendering this type of
    /// [PdfPageAnnotation].
    ///
    /// Not all PDF annotation types are supported by Pdfium. For example, Pdfium does not
    /// currently support embedded sound or movie files, embedded 3D animations, or embedded
    /// file attachments generally.
    ///
    /// Pdfium currently supports creating, editing, and rendering the following types of annotations:
    ///
    /// * [PdfPageAnnotationType::Circle]
    /// * [PdfPageAnnotationType::FreeText]
    /// * [PdfPageAnnotationType::Highlight]
    /// * [PdfPageAnnotationType::Ink]
    /// * [PdfPageAnnotationType::Link]
    /// * [PdfPageAnnotationType::Popup]
    /// * [PdfPageAnnotationType::Square]
    /// * [PdfPageAnnotationType::Squiggly]
    /// * [PdfPageAnnotationType::Stamp]
    /// * [PdfPageAnnotationType::Strikeout]
    /// * [PdfPageAnnotationType::Text]
    /// * [PdfPageAnnotationType::Underline]
    #[inline]
    pub fn is_unsupported(&self) -> bool {
        matches!(self, PdfPageAnnotation::Unsupported(_))
    }

    /// Returns the underlying [PdfPageCircleAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Circle].
    #[inline]
    pub fn as_circle_annotation(&self) -> Option<&PdfPageCircleAnnotation> {
        match self {
            PdfPageAnnotation::Circle(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageFreeTextAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::FreeText].
    #[inline]
    pub fn as_free_text_annotation(&self) -> Option<&PdfPageFreeTextAnnotation> {
        match self {
            PdfPageAnnotation::FreeText(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageHighlightAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Highlight].
    #[inline]
    pub fn as_highlight_annotation(&self) -> Option<&PdfPageHighlightAnnotation> {
        match self {
            PdfPageAnnotation::Highlight(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageInkAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Ink].
    #[inline]
    pub fn as_ink_annotation(&self) -> Option<&PdfPageInkAnnotation> {
        match self {
            PdfPageAnnotation::Ink(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageLinkAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Link].
    #[inline]
    pub fn as_link_annotation(&self) -> Option<&PdfPageLinkAnnotation> {
        match self {
            PdfPageAnnotation::Link(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPagePopupAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Popup].
    #[inline]
    pub fn as_popup_annotation(&self) -> Option<&PdfPagePopupAnnotation> {
        match self {
            PdfPageAnnotation::Popup(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageSquareAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Square].
    #[inline]
    pub fn as_square_annotation(&self) -> Option<&PdfPageSquareAnnotation> {
        match self {
            PdfPageAnnotation::Square(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageSquigglyAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Squiggly].
    #[inline]
    pub fn as_squiggly_annotation(&self) -> Option<&PdfPageSquigglyAnnotation> {
        match self {
            PdfPageAnnotation::Squiggly(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageStampAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Stamp].
    #[inline]
    pub fn as_stamp_annotation(&self) -> Option<&PdfPageStampAnnotation> {
        match self {
            PdfPageAnnotation::Stamp(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageStrikeoutAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Strikeout].
    #[inline]
    pub fn as_strikeout_annotation(&self) -> Option<&PdfPageStrikeoutAnnotation> {
        match self {
            PdfPageAnnotation::Strikeout(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageTextAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Text].
    #[inline]
    pub fn as_text_annotation(&self) -> Option<&PdfPageTextAnnotation> {
        match self {
            PdfPageAnnotation::Text(annotation) => Some(annotation),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageUnderlineAnnotation] for this [PdfPageAnnotation],
    /// if this annotation has an annotation type of [PdfPageAnnotationType::Underline].
    #[inline]
    pub fn as_underline_annotation(&self) -> Option<&PdfPageUnderlineAnnotation> {
        match self {
            PdfPageAnnotation::Underline(annotation) => Some(annotation),
            _ => None,
        }
    }
}

/// Functionality common to all [PdfPageAnnotation] objects, regardless of their [PdfPageAnnotationType].
pub trait PdfPageAnnotationCommon {
    /// Returns the name of this [PdfPageAnnotation], if any. This is a text string uniquely identifying
    /// this annotation among all the annotations attached to the containing page.
    fn name(&self) -> Option<String>;

    /// Returns the bounding box of this [PdfPageAnnotation].
    fn bounds(&self) -> Result<PdfRect, PdfiumError>;

    /// Returns the text to be displayed for this [PdfPageAnnotation], or, if this type of annotation
    /// does not display text, an alternate description of the annotation's contents in human-readable
    /// form. In either case this text is useful when extracting the document's contents in support
    /// of accessibility to users with disabilities or for other purposes.
    fn contents(&self) -> Option<String>;

    /// Returns the name of the creator of this [PdfPageAnnotation], if any.
    fn creator(&self) -> Option<String>;

    /// Returns the date and time when this [PdfPageAnnotation] was originally created, if any.
    fn creation_date(&self) -> Option<String>;

    /// Returns the date and time when this [PdfPageAnnotation] was last modified, if any.
    fn modification_date(&self) -> Option<String>;
}

// Blanket implementation for all PdfPageAnnotation types.

impl<T> PdfPageAnnotationCommon for T
where
    T: PdfPageAnnotationPrivate,
{
    #[inline]
    fn name(&self) -> Option<String> {
        self.name_impl()
    }

    #[inline]
    fn bounds(&self) -> Result<PdfRect, PdfiumError> {
        self.bounds_impl()
    }

    #[inline]
    fn contents(&self) -> Option<String> {
        self.contents_impl()
    }

    #[inline]
    fn creator(&self) -> Option<String> {
        self.creator_impl()
    }

    #[inline]
    fn creation_date(&self) -> Option<String> {
        self.creation_date_impl()
    }

    #[inline]
    fn modification_date(&self) -> Option<String> {
        self.modification_date_impl()
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageAnnotation<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_ANNOTATION {
        self.unwrap_as_trait().get_handle()
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.unwrap_as_trait().get_bindings()
    }
}

impl<'a> Drop for PdfPageAnnotation<'a> {
    /// Closes this [PdfPageAnnotation], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        self.get_bindings().FPDFPage_CloseAnnot(*self.get_handle());
    }
}
