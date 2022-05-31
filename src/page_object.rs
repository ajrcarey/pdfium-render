//! Defines the [PdfPageObject] enum, exposing functionality related to a single page object.

use crate::bindgen::{
    FPDF_LINECAP_BUTT, FPDF_LINECAP_PROJECTING_SQUARE, FPDF_LINECAP_ROUND, FPDF_LINEJOIN_BEVEL,
    FPDF_LINEJOIN_MITER, FPDF_LINEJOIN_ROUND, FPDF_PAGE, FPDF_PAGEOBJECT, FPDF_PAGEOBJ_FORM,
    FPDF_PAGEOBJ_IMAGE, FPDF_PAGEOBJ_PATH, FPDF_PAGEOBJ_SHADING, FPDF_PAGEOBJ_TEXT,
    FPDF_PAGEOBJ_UNKNOWN,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::color::PdfColor;
use crate::error::PdfiumError;
use crate::page::{PdfPoints, PdfRect};
use crate::page_object_form_fragment::PdfPageFormFragmentObject;
use crate::page_object_image::PdfPageImageObject;
use crate::page_object_path::PdfPagePathObject;
use crate::page_object_private::internal::PdfPageObjectPrivate;
use crate::page_object_shading::PdfPageShadingObject;
use crate::page_object_text::PdfPageTextObject;
use crate::page_object_unsupported::PdfPageUnsupportedObject;
use std::convert::TryInto;
use std::os::raw::{c_int, c_uint};

/// The type of a single [PdfPageObject].
///
/// Note that Pdfium does not support or recognize all PDF page object types. For instance,
/// Pdfium does not currently support or recognize the External Object ("XObject") page object
/// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. In these cases, Pdfium
/// will return `PdfPageObjectType::Unsupported`.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfPageObjectType {
    Unsupported = FPDF_PAGEOBJ_UNKNOWN as isize,
    Text = FPDF_PAGEOBJ_TEXT as isize,
    Path = FPDF_PAGEOBJ_PATH as isize,
    Image = FPDF_PAGEOBJ_IMAGE as isize,
    Shading = FPDF_PAGEOBJ_SHADING as isize,
    FormFragment = FPDF_PAGEOBJ_FORM as isize,
}

impl PdfPageObjectType {
    pub(crate) fn from_pdfium(value: u32) -> Result<PdfPageObjectType, PdfiumError> {
        match value {
            FPDF_PAGEOBJ_UNKNOWN => Ok(PdfPageObjectType::Unsupported),
            FPDF_PAGEOBJ_TEXT => Ok(PdfPageObjectType::Text),
            FPDF_PAGEOBJ_PATH => Ok(PdfPageObjectType::Path),
            FPDF_PAGEOBJ_IMAGE => Ok(PdfPageObjectType::Image),
            FPDF_PAGEOBJ_SHADING => Ok(PdfPageObjectType::Shading),
            FPDF_PAGEOBJ_FORM => Ok(PdfPageObjectType::FormFragment),
            _ => Err(PdfiumError::UnknownPdfPageObjectType),
        }
    }
}

/// The method used to combine overlapping colors when painting one [PdfPageObject] on top of
/// another.
///
/// The color being newly painted is the source color;the existing color being painted onto is the
/// backdrop color.
///
/// A formal definition of these blend modes can be found in Section 7.2.4 of
/// The PDF Reference Manual, version 1.7, on page 520.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PdfPageObjectBlendMode {
    /// Selects the source color, ignoring the backdrop.
    Normal,

    /// Multiplies the backdrop and source color values. The resulting color is always at least
    /// as dark as either of the two constituent colors. Multiplying any color with black
    /// produces black; multiplying with white leaves the original color unchanged.
    /// Painting successive overlapping objects with a color other than black or white
    /// produces progressively darker colors.
    Multiply,

    /// Multiplies the complements of the backdrop and source color values, then complements
    /// the result.

    /// The result color is always at least as light as either of the two constituent colors.
    /// Screening any color with white produces white; screening with black leaves the original
    /// color unchanged. The effect is similar to projecting multiple photographic slides
    /// simultaneously onto a single screen.
    Screen,

    /// Multiplies or screens the colors, depending on the backdrop color value. Source colors
    /// overlay the backdrop while preserving its highlights and shadows. The backdrop color is
    /// not replaced but is mixed with the source color to reflect the lightness or darkness of
    /// the backdrop.
    Overlay,

    /// Selects the darker of the backdrop and source colors. The backdrop is replaced with the
    /// source where the source is darker; otherwise, it is left unchanged.
    Darken,

    /// Selects the lighter of the backdrop and source colors. The backdrop is replaced with the
    /// source where the source is lighter; otherwise, it is left unchanged.
    Lighten,

    /// Brightens the backdrop color to reflect the source color. Painting with black produces no
    /// changes.
    ColorDodge,

    /// Darkens the backdrop color to reflect the source color. Painting with white produces no
    /// change.
    ColorBurn,

    /// Multiplies or screens the colors, depending on the source color value. The effect is similar
    /// to shining a harsh spotlight on the backdrop.
    HardLight,

    /// Darkens or lightens the colors, depending on the source color value. The effect is similar
    /// to shining a diffused spotlight on the backdrop.
    SoftLight,

    /// Subtracts the darker of the two constituent colors from the lighter color.
    /// Painting with white inverts the backdrop color; painting with black produces no change.
    Difference,

    /// Produces an effect similar to that of the Difference mode but lower in contrast.
    /// Painting with white inverts the backdrop color; painting with black produces no change.
    Exclusion,

    /// Preserves the luminosity of the backdrop color while adopting the hue and saturation
    /// of the source color.
    HSLColor,

    /// Preserves the luminosity and saturation of the backdrop color while adopting the hue
    /// of the source color.
    HSLHue,

    /// Preserves the hue and saturation of the backdrop color while adopting the luminosity
    /// of the source color.
    HSLLuminosity,

    /// Preserves the luminosity and hue of the backdrop color while adopting the saturation
    /// of the source color.
    HSLSaturation,
}

impl PdfPageObjectBlendMode {
    pub(crate) fn as_pdfium(&self) -> &str {
        match self {
            PdfPageObjectBlendMode::HSLColor => "Color",
            PdfPageObjectBlendMode::ColorBurn => "ColorBurn",
            PdfPageObjectBlendMode::ColorDodge => "ColorDodge",
            PdfPageObjectBlendMode::Darken => "Darken",
            PdfPageObjectBlendMode::Difference => "Difference",
            PdfPageObjectBlendMode::Exclusion => "Exclusion",
            PdfPageObjectBlendMode::HardLight => "HardLight",
            PdfPageObjectBlendMode::HSLHue => "Hue",
            PdfPageObjectBlendMode::Lighten => "Lighten",
            PdfPageObjectBlendMode::HSLLuminosity => "Luminosity",
            PdfPageObjectBlendMode::Multiply => "Multiply",
            PdfPageObjectBlendMode::Normal => "Normal",
            PdfPageObjectBlendMode::Overlay => "Overlay",
            PdfPageObjectBlendMode::HSLSaturation => "Saturation",
            PdfPageObjectBlendMode::Screen => "Screen",
            PdfPageObjectBlendMode::SoftLight => "SoftLight",
        }
    }
}

/// The shape that should be used at the corners of stroked paths.
///
/// Join styles are significant only at points where consecutive segments of a path
/// connect at an angle; segments that meet or intersect fortuitously receive no special treatment.
///
/// A formal definition of these styles can be found in Section 4.3.2 of
/// The PDF Reference Manual, version 1.7, on page 216.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PdfPageObjectLineJoin {
    /// The outer edges of the strokes for the two path segments are extended
    /// until they meet at an angle, as in a picture frame. If the segments meet at too
    /// sharp an angle, a bevel join is used instead.
    Miter = FPDF_LINEJOIN_MITER as isize,

    /// An arc of a circle with a diameter equal to the line width is drawn
    /// around the point where the two path segments meet, connecting the outer edges of
    /// the strokes for the two segments. This pie-slice-shaped figure is filled in,
    /// producing a rounded corner.
    Round = FPDF_LINEJOIN_ROUND as isize,

    /// The two path segments are finished with butt caps and the resulting notch
    /// beyond the ends of the segments is filled with a triangle.
    Bevel = FPDF_LINEJOIN_BEVEL as isize,
}

impl PdfPageObjectLineJoin {
    pub(crate) fn from_pdfium(value: c_int) -> Option<Self> {
        match value as u32 {
            FPDF_LINEJOIN_MITER => Some(Self::Miter),
            FPDF_LINEJOIN_ROUND => Some(Self::Round),
            FPDF_LINEJOIN_BEVEL => Some(Self::Bevel),
            _ => None,
        }
    }

    pub(crate) fn as_pdfium(&self) -> u32 {
        match self {
            PdfPageObjectLineJoin::Miter => FPDF_LINEJOIN_MITER,
            PdfPageObjectLineJoin::Round => FPDF_LINEJOIN_ROUND,
            PdfPageObjectLineJoin::Bevel => FPDF_LINEJOIN_BEVEL,
        }
    }
}

/// The shape that should be used at the ends of open stroked paths.
///
/// A formal definition of these styles can be found in Section 4.3.2 of
/// The PDF Reference Manual, version 1.7, on page 216.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PdfPageObjectLineCap {
    /// The stroke is squared off at the endpoint of the path. There is no
    /// projection beyond the end of the path.
    Butt = FPDF_LINECAP_BUTT as isize,

    /// A semicircular arc with a diameter equal to the line width is
    /// drawn around the endpoint and filled in.
    Round = FPDF_LINECAP_ROUND as isize,

    /// The stroke continues beyond the endpoint of the path
    /// for a distance equal to half the line width and is squared off.
    Square = FPDF_LINECAP_PROJECTING_SQUARE as isize,
}

impl PdfPageObjectLineCap {
    pub(crate) fn from_pdfium(value: c_int) -> Option<Self> {
        match value as u32 {
            FPDF_LINECAP_BUTT => Some(Self::Butt),
            FPDF_LINECAP_ROUND => Some(Self::Round),
            FPDF_LINECAP_PROJECTING_SQUARE => Some(Self::Square),
            _ => None,
        }
    }

    pub(crate) fn as_pdfium(&self) -> u32 {
        match self {
            PdfPageObjectLineCap::Butt => FPDF_LINECAP_BUTT,
            PdfPageObjectLineCap::Round => FPDF_LINECAP_ROUND,
            PdfPageObjectLineCap::Square => FPDF_LINECAP_PROJECTING_SQUARE,
        }
    }
}

/// A single object on a `PdfPage`.
pub enum PdfPageObject<'a> {
    Text(PdfPageTextObject<'a>),
    Path(PdfPagePathObject<'a>),
    Image(PdfPageImageObject<'a>),
    Shading(PdfPageShadingObject<'a>),
    FormFragment(PdfPageFormFragmentObject<'a>),

    /// Common properties shared by all [PdfPageObject] types can still be accessed for
    /// page objects not recognized by Pdfium, but object-specific functionality
    /// will be unavailable.
    Unsupported(PdfPageUnsupportedObject<'a>),
}

impl<'a> PdfPageObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: FPDF_PAGE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        match PdfPageObjectType::from_pdfium(bindings.FPDFPageObj_GetType(object_handle) as u32)
            .unwrap_or(PdfPageObjectType::Unsupported)
        {
            PdfPageObjectType::Unsupported => PdfPageObject::Unsupported(
                PdfPageUnsupportedObject::from_pdfium(object_handle, page_handle, bindings),
            ),
            PdfPageObjectType::Text => PdfPageObject::Text(PdfPageTextObject::from_pdfium(
                object_handle,
                page_handle,
                bindings,
            )),
            PdfPageObjectType::Path => PdfPageObject::Path(PdfPagePathObject::from_pdfium(
                object_handle,
                page_handle,
                bindings,
            )),
            PdfPageObjectType::Image => PdfPageObject::Image(PdfPageImageObject::from_pdfium(
                object_handle,
                page_handle,
                bindings,
            )),
            PdfPageObjectType::Shading => PdfPageObject::Shading(
                PdfPageShadingObject::from_pdfium(object_handle, page_handle, bindings),
            ),
            PdfPageObjectType::FormFragment => PdfPageObject::FormFragment(
                PdfPageFormFragmentObject::from_pdfium(object_handle, page_handle, bindings),
            ),
        }
    }

    #[inline]
    pub(crate) fn unwrap_as_trait(&'a self) -> &'a dyn PdfPageObjectPrivate<'a> {
        match self {
            PdfPageObject::Text(object) => object,
            PdfPageObject::Path(object) => object,
            PdfPageObject::Image(object) => object,
            PdfPageObject::Shading(object) => object,
            PdfPageObject::FormFragment(object) => object,
            PdfPageObject::Unsupported(object) => object,
        }
    }

    /// The object type of this [PdfPageObject].
    ///
    /// Note that Pdfium does not support or recognize all PDF page object types. For instance,
    /// Pdfium does not currently support or recognize the External Object ("XObject") page object
    /// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. In these cases, Pdfium
    /// will return `PdfPageObjectType::Unsupported`.
    #[inline]
    pub fn object_type(&self) -> PdfPageObjectType {
        match self {
            PdfPageObject::Text(_) => PdfPageObjectType::Text,
            PdfPageObject::Path(_) => PdfPageObjectType::Path,
            PdfPageObject::Image(_) => PdfPageObjectType::Image,
            PdfPageObject::Shading(_) => PdfPageObjectType::Shading,
            PdfPageObject::FormFragment(_) => PdfPageObjectType::FormFragment,
            PdfPageObject::Unsupported(_) => PdfPageObjectType::Unsupported,
        }
    }

    /// Returns `true` if this [PdfPageObject] has an object type other than [PdfPageObjectType::Unsupported].
    ///
    /// The [PdfPageObject::as_text_object()], [PdfPageObject::as_path_object()], [PdfPageObject::as_image_object()],
    /// [PdfPageObject::as_shading_object()], and [PdfPageObject::as_form_fragment_object()] functions
    /// can be used to access properties and functions pertaining to a specific page object type.
    #[inline]
    pub fn is_supported(&self) -> bool {
        !self.is_unsupported()
    }

    /// Returns `true` if this [PdfPageObject] has an object type of [PdfPageObjectType::Unsupported].
    ///
    /// Common properties shared by all [PdfPageObject] types can still be accessed for
    /// page objects not recognized by Pdfium, but object-specific functionality
    /// will be unavailable.
    #[inline]
    pub fn is_unsupported(&self) -> bool {
        self.object_type() == PdfPageObjectType::Unsupported
    }

    /// Returns the underlying [PdfPageTextObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Text].
    #[inline]
    pub fn as_text_object(&self) -> Option<&PdfPageTextObject> {
        match self {
            PdfPageObject::Text(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPagePathObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Path].
    #[inline]
    pub fn as_path_object(&self) -> Option<&PdfPagePathObject> {
        match self {
            PdfPageObject::Path(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageImageObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Image].
    #[inline]
    pub fn as_image_object(&self) -> Option<&PdfPageImageObject> {
        match self {
            PdfPageObject::Image(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageShadingObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Shading].
    #[inline]
    pub fn as_shading_object(&self) -> Option<&PdfPageShadingObject> {
        match self {
            PdfPageObject::Shading(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageFormFragmentObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::FormFragment].
    #[inline]
    pub fn as_form_fragment_object(&self) -> Option<&PdfPageFormFragmentObject> {
        match self {
            PdfPageObject::FormFragment(object) => Some(object),
            _ => None,
        }
    }
}

/// Functionality common to all [PdfPageObject] objects, regardless of their [PdfPageObjectType].
pub trait PdfPageObjectCommon<'a> {
    /// Returns `true` if this [PdfPageObject] contains transparency.
    fn has_transparency(&self) -> bool;

    /// Returns the bounding box of this [PdfPageObject].
    fn bounds(&self) -> Result<PdfRect, PdfiumError>;

    /// Returns the width of this [PdfPageObject].
    #[inline]
    fn width(&self) -> Result<PdfPoints, PdfiumError> {
        self.bounds().map(|bounds| bounds.right - bounds.left)
    }

    /// Returns the height of this [PdfPageObject].
    #[inline]
    fn height(&self) -> Result<PdfPoints, PdfiumError> {
        self.bounds().map(|bounds| bounds.top - bounds.bottom)
    }

    /// Returns `true` if the bounds of this [PdfPageObject] lie entirely within the given rectangle.
    #[inline]
    fn is_inside_rect(&self, rect: &PdfRect) -> bool {
        self.bounds()
            .map(|bounds| bounds.is_inside(rect))
            .unwrap_or(false)
    }

    /// Returns `true` if the bounds of this [PdfPageObject] lie at least partially within
    /// the given rectangle.
    #[inline]
    fn does_overlap_rect(&self, rect: &PdfRect) -> bool {
        self.bounds()
            .map(|bounds| bounds.does_overlap(rect))
            .unwrap_or(false)
    }

    /// Applies the given transformation, expressed as six values representing the six configurable
    /// elements of a nine-element 3x3 PDF transformation matrix, to this [PdfPageObject].
    ///
    /// To move, scale, rotate, or skew a [PdfPageObject], consider using one or more of the
    /// following functions. Internally they all use [PdfPageObjectCommon::transform()], but are
    /// probably easier to use (and certainly clearer in their intent) in most situations.
    ///
    /// * [PdfPageObjectCommon::translate()]: changes the position of a [PdfPageObject].
    /// * [PdfPageObjectCommon::scale()]: changes the size of a [PdfPageObject].
    /// * [PdfPageObjectCommon::rotate_clockwise_degrees()], [PdfPageObjectCommon::rotate_counter_clockwise_degrees()],
    /// [PdfPageObjectCommon::rotate_clockwise_radians()], [PdfPageObjectCommon::rotate_counter_clockwise_radians()]:
    /// rotates a [PdfPageObject] around its origin.
    /// * [PdfPageObjectCommon::skew_degrees()], [PdfPageObjectCommon::skew_radians()]: skews a [PdfPageObject]
    /// relative to its axes.
    ///
    /// **The order in which transformations are applied to a page object is significant.**
    /// For example, the result of rotating _then_ translating a page object may be vastly different
    /// from translating _then_ rotating the same page object.
    ///
    /// An overview of PDF transformation matrices can be found in the PDF Reference Manual
    /// version 1.7 on page 204; a detailed description can be founded in section 4.2.3 on page 207.
    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);

    /// Moves the origin of this [PdfPageObject] by the given horizontal and vertical delta distances.
    #[inline]
    fn translate(&mut self, delta_x: PdfPoints, delta_y: PdfPoints) {
        self.transform(
            1.0,
            0.0,
            0.0,
            1.0,
            delta_x.value as f64,
            delta_y.value as f64,
        )
    }

    /// Changes the size of this [PdfPageObject], scaling it by the given horizontal and
    /// vertical scale factors.
    #[inline]
    fn scale(&mut self, horizontal_scale_factor: f64, vertical_scale_factor: f64) {
        self.transform(
            horizontal_scale_factor,
            0.0,
            0.0,
            vertical_scale_factor,
            0.0,
            0.0,
        )
    }

    /// Rotates this [PdfPageObject] counter-clockwise by the given number of degrees.
    #[inline]
    fn rotate_counter_clockwise_degrees(&mut self, degrees: f32) {
        self.rotate_counter_clockwise_radians(degrees.to_radians())
    }

    /// Rotates this [PdfPageObject] clockwise by the given number of degrees.
    #[inline]
    fn rotate_clockwise_degrees(&mut self, degrees: f32) {
        self.rotate_counter_clockwise_degrees(-degrees)
    }

    /// Rotates this [PdfPageObject] counter-clockwise by the given number of radians.
    #[inline]
    fn rotate_counter_clockwise_radians(&mut self, radians: f32) {
        let cos_theta = radians.cos() as f64;

        let sin_theta = radians.sin() as f64;

        self.transform(cos_theta, sin_theta, -sin_theta, cos_theta, 0.0, 0.0);
    }

    /// Rotates this [PdfPageObject] clockwise by the given number of radians.
    #[inline]
    fn rotate_clockwise_radians(&mut self, radians: f32) {
        self.rotate_counter_clockwise_radians(-radians.to_degrees())
    }

    /// Skews the axes of this [PdfPageObject] by the given angles in degrees.
    #[inline]
    fn skew_degrees(&mut self, x_axis_skew: f32, y_axis_skew: f32) {
        self.skew_radians(x_axis_skew.to_radians(), y_axis_skew.to_radians())
    }

    /// Skews the axes of this [PdfPageObject] by the given angles in radians.
    #[inline]
    fn skew_radians(&mut self, x_axis_skew: f32, y_axis_skew: f32) {
        let tan_alpha = x_axis_skew.tan() as f64;

        let tan_beta = y_axis_skew.tan() as f64;

        self.transform(1.0, tan_alpha, tan_beta, 1.0, 0.0, 0.0);
    }

    /// Sets the blend mode that will be applied when painting this [PdfPageObject].
    fn set_blend_mode(&mut self, blend_mode: PdfPageObjectBlendMode);

    /// Returns the color of any filled paths in this [PdfPageObject].
    fn fill_color(&self) -> Result<PdfColor, PdfiumError>;

    /// Sets the color of any filled paths in this [PdfPageObject].
    fn set_fill_color(&self, fill_color: PdfColor) -> Result<(), PdfiumError>;

    /// Returns the color of any stroked lines in this [PdfPageObject].
    fn stroke_color(&self) -> Result<PdfColor, PdfiumError>;

    /// Sets the color of any stroked lines in this [PdfPageObject].
    fn set_stroke_color(&self, stroke_color: PdfColor) -> Result<(), PdfiumError>;

    /// Returns the width of any stroked lines in this [PdfPageObject].
    fn stroke_width(&self) -> Result<PdfPoints, PdfiumError>;

    /// Sets the width of any stroked lines in this [PdfPageObject].
    ///
    /// A line width of 0 denotes the thinnest line that can be rendered at device resolution:
    /// 1 device pixel wide. However, some devices cannot reproduce 1-pixel lines,
    /// and on high-resolution devices, they are nearly invisible. Since the results of rendering
    /// such zero-width lines are device-dependent, their use is not recommended.
    fn set_stroke_width(&self, stroke_width: PdfPoints) -> Result<(), PdfiumError>;

    /// Returns the line join style that will be used when painting stroked path segments
    /// in this [PdfPageObject].
    fn line_join(&self) -> Result<PdfPageObjectLineJoin, PdfiumError>;

    /// Sets the line join style that will be used when painting stroked path segments
    /// in this [PdfPageObject].
    fn set_line_join(&self, line_join: PdfPageObjectLineJoin) -> Result<(), PdfiumError>;

    /// Returns the line cap style that will be used when painting stroked path segments
    /// in this [PdfPageObject].
    fn line_cap(&self) -> Result<PdfPageObjectLineCap, PdfiumError>;

    /// Sets the line join style that will be used when painting stroked path segments
    /// in this [PdfPageObject].
    fn set_line_cap(&self, line_cap: PdfPageObjectLineCap) -> Result<(), PdfiumError>;
}

// Blanket implementation for all PdfPageObject types.

impl<'a, T> PdfPageObjectCommon<'a> for T
where
    T: PdfPageObjectPrivate<'a>,
{
    #[inline]
    fn has_transparency(&self) -> bool {
        self.has_transparency_impl()
    }

    #[inline]
    fn bounds(&self) -> Result<PdfRect, PdfiumError> {
        self.bounds_impl()
    }

    #[inline]
    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.transform_impl(a, b, c, d, e, f)
    }

    #[inline]
    fn set_blend_mode(&mut self, blend_mode: PdfPageObjectBlendMode) {
        self.get_bindings()
            .FPDFPageObj_SetBlendMode(*self.get_handle(), blend_mode.as_pdfium());
    }

    fn fill_color(&self) -> Result<PdfColor, PdfiumError> {
        let mut r = 0;

        let mut g = 0;

        let mut b = 0;

        let mut a = 0;

        if self
            .get_bindings()
            .is_true(self.get_bindings().FPDFPageObj_GetFillColor(
                *self.get_handle(),
                &mut r,
                &mut g,
                &mut b,
                &mut a,
            ))
        {
            Ok(PdfColor::new(
                r.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
                g.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
                b.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
                a.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
            ))
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn set_fill_color(&self, fill_color: PdfColor) -> Result<(), PdfiumError> {
        if self
            .get_bindings()
            .is_true(self.get_bindings().FPDFPageObj_SetFillColor(
                *self.get_handle(),
                fill_color.red() as c_uint,
                fill_color.green() as c_uint,
                fill_color.blue() as c_uint,
                fill_color.alpha() as c_uint,
            ))
        {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn stroke_color(&self) -> Result<PdfColor, PdfiumError> {
        let mut r = 0;

        let mut g = 0;

        let mut b = 0;

        let mut a = 0;

        if self
            .get_bindings()
            .is_true(self.get_bindings().FPDFPageObj_GetStrokeColor(
                *self.get_handle(),
                &mut r,
                &mut g,
                &mut b,
                &mut a,
            ))
        {
            Ok(PdfColor::new(
                r.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
                g.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
                b.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
                a.try_into()
                    .map_err(PdfiumError::UnableToConvertPdfiumColorValueToRustu8)?,
            ))
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn set_stroke_color(&self, stroke_color: PdfColor) -> Result<(), PdfiumError> {
        if self
            .get_bindings()
            .is_true(self.get_bindings().FPDFPageObj_SetStrokeColor(
                *self.get_handle(),
                stroke_color.red() as c_uint,
                stroke_color.green() as c_uint,
                stroke_color.blue() as c_uint,
                stroke_color.alpha() as c_uint,
            ))
        {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn stroke_width(&self) -> Result<PdfPoints, PdfiumError> {
        let mut width = 0.0;

        if self.get_bindings().is_true(
            self.get_bindings()
                .FPDFPageObj_GetStrokeWidth(*self.get_handle(), &mut width),
        ) {
            Ok(PdfPoints::new(width))
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn set_stroke_width(&self, stroke_width: PdfPoints) -> Result<(), PdfiumError> {
        if self.get_bindings().is_true(
            self.get_bindings()
                .FPDFPageObj_SetStrokeWidth(*self.get_handle(), stroke_width.value),
        ) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn line_join(&self) -> Result<PdfPageObjectLineJoin, PdfiumError> {
        PdfPageObjectLineJoin::from_pdfium(
            self.get_bindings()
                .FPDFPageObj_GetLineJoin(*self.get_handle()),
        )
        .ok_or(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
    }

    fn set_line_join(&self, line_join: PdfPageObjectLineJoin) -> Result<(), PdfiumError> {
        if self.get_bindings().is_true(
            self.get_bindings()
                .FPDFPageObj_SetLineJoin(*self.get_handle(), line_join.as_pdfium() as c_int),
        ) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }

    fn line_cap(&self) -> Result<PdfPageObjectLineCap, PdfiumError> {
        PdfPageObjectLineCap::from_pdfium(
            self.get_bindings()
                .FPDFPageObj_GetLineCap(*self.get_handle()),
        )
        .ok_or(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
    }

    fn set_line_cap(&self, line_cap: PdfPageObjectLineCap) -> Result<(), PdfiumError> {
        if self.get_bindings().is_true(
            self.get_bindings()
                .FPDFPageObj_SetLineCap(*self.get_handle(), line_cap.as_pdfium() as c_int),
        ) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumFunctionReturnValueIndicatedFailure)
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageObject<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_PAGEOBJECT {
        self.unwrap_as_trait().get_handle()
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.unwrap_as_trait().get_bindings()
    }

    #[inline]
    fn is_object_memory_owned_by_page(&self) -> bool {
        self.unwrap_as_trait().is_object_memory_owned_by_page()
    }

    #[inline]
    fn set_object_memory_owned_by_page(&mut self, page: FPDF_PAGE) {
        // Trying to define a self.unwrap_as_trait_mut() fn gets us into all sorts of
        // arguments with the borrow checker. It's easier just to unwrap the inner object inline.

        match self {
            PdfPageObject::Text(object) => object.set_object_memory_owned_by_page(page),
            PdfPageObject::Path(object) => object.set_object_memory_owned_by_page(page),
            PdfPageObject::Image(object) => object.set_object_memory_owned_by_page(page),
            PdfPageObject::Shading(object) => object.set_object_memory_owned_by_page(page),
            PdfPageObject::FormFragment(object) => object.set_object_memory_owned_by_page(page),
            PdfPageObject::Unsupported(object) => object.set_object_memory_owned_by_page(page),
        }
    }

    #[inline]
    fn set_object_memory_released_by_page(&mut self) {
        // Trying to define a self.unwrap_as_trait_mut() fn gets us into all sorts of
        // arguments with the borrow checker. It's easier just to unwrap the inner object inline.

        match self {
            PdfPageObject::Text(object) => object.set_object_memory_released_by_page(),
            PdfPageObject::Path(object) => object.set_object_memory_released_by_page(),
            PdfPageObject::Image(object) => object.set_object_memory_released_by_page(),
            PdfPageObject::Shading(object) => object.set_object_memory_released_by_page(),
            PdfPageObject::FormFragment(object) => object.set_object_memory_released_by_page(),
            PdfPageObject::Unsupported(object) => object.set_object_memory_released_by_page(),
        }
    }
}

impl<'a> Drop for PdfPageObject<'a> {
    /// Closes this [PdfPageObject], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        // The documentation for FPDFPageObj_Destroy() states that we only need
        // call the function for page objects created by FPDFPageObj_CreateNew*() or
        // FPDFPageObj_New*Obj() _and_ where the newly-created object was _not_ subsequently
        // added to a PdfPage via a call to FPDFPage_InsertObject() or FPDFAnnot_AppendObject().
        // In other words, retrieving a page object that already exists in a document evidently
        // does not allocate any additional resources, so we don't need to free anything.
        // (Indeed, if we try to, Pdfium segfaults.)

        if !self.is_object_memory_owned_by_page() {
            self.get_bindings().FPDFPageObj_Destroy(*self.get_handle());
        }
    }
}
