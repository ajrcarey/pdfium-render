//! Defines the [PdfRenderConfig] struct, a builder-based approach to configuring
//! the rendering of `PdfBitmap` objects from one or more [PdfPage] objects.

use crate::bindgen::{
    FPDF_ANNOT, FPDF_CONVERT_FILL_TO_STROKE, FPDF_DWORD, FPDF_GRAYSCALE, FPDF_LCD_TEXT,
    FPDF_NO_NATIVETEXT, FPDF_PRINTING, FPDF_RENDER_FORCEHALFTONE, FPDF_RENDER_LIMITEDIMAGECACHE,
    FPDF_RENDER_NO_SMOOTHIMAGE, FPDF_RENDER_NO_SMOOTHPATH, FPDF_RENDER_NO_SMOOTHTEXT,
    FPDF_REVERSE_BYTE_ORDER, FS_MATRIX, FS_RECTF,
};
use crate::bitmap::{PdfBitmapFormat, PdfBitmapRotation, Pixels};
use crate::color::PdfColor;
use crate::error::PdfiumError;
use crate::form::PdfFormFieldType;
use crate::page::PdfPageOrientation::{Landscape, Portrait};
use crate::page::{PdfPage, PdfPageOrientation, PdfPoints};
use crate::prelude::PdfMatrixValue;
use std::os::raw::c_int;
use vecmath::{mat3_det, row_mat3_mul, Matrix3};

// TODO: AJRC - 29/7/22 - remove deprecated PdfBitmapConfig struct in 0.9.0 as part of tracking issue
// https://github.com/ajrcarey/pdfium-render/issues/36
#[deprecated(
    since = "0.7.12",
    note = "This struct has been renamed to better reflect its purpose. Use the PdfRenderConfig struct instead."
)]
#[doc(hidden)]
pub struct PdfBitmapConfig {}

#[allow(deprecated)]
impl PdfBitmapConfig {
    /// Creates a new [PdfRenderConfig] object with all settings initialized with their default values.
    #[deprecated(
        since = "0.7.12",
        note = "This struct has been renamed to better reflect its purpose. Use the PdfRenderConfig::new() function instead."
    )]
    #[inline]
    #[doc(hidden)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> PdfRenderConfig {
        PdfRenderConfig::new()
    }

    #[deprecated(
        since = "0.7.12",
        note = "This struct has been renamed to better reflect its purpose. Use the PdfRenderConfig::default() function instead."
    )]
    #[inline]
    #[doc(hidden)]
    pub fn default() -> PdfRenderConfig {
        PdfRenderConfig::default()
    }
}

/// Configures the scaling, rotation, and rendering settings that should be applied to
/// a [PdfPage] to create a `PdfBitmap` for that page. [PdfRenderConfig] can accommodate pages of
/// different sizes while correctly maintaining each page's aspect ratio, automatically
/// rotate portrait or landscape pages, generate page thumbnails, apply maximum and
/// minimum pixel sizes to the scaled width and height of the final bitmap, highlight form fields
/// with different colors, apply custom transforms to the page during rendering, and set
/// internal Pdfium rendering flags.
///
/// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
/// a custom transformation matrix, but not both at the same time. Applying any transformation
/// automatically disables rendering of form data. If you must render form data while simultaneously
/// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
/// form elements and form data into the containing page.
pub struct PdfRenderConfig {
    target_width: Option<Pixels>,
    target_height: Option<Pixels>,
    scale_width_factor: Option<f32>,
    scale_height_factor: Option<f32>,
    maximum_width: Option<Pixels>,
    maximum_height: Option<Pixels>,
    portrait_rotation: PdfBitmapRotation,
    portrait_rotation_do_rotate_constraints: bool,
    landscape_rotation: PdfBitmapRotation,
    landscape_rotation_do_rotate_constraints: bool,
    format: PdfBitmapFormat,
    do_clear_bitmap_before_rendering: bool,
    clear_color: PdfColor,
    do_render_form_data: bool,
    form_field_highlight: Option<Vec<(PdfFormFieldType, PdfColor)>>,
    transformation_matrix: Matrix3<f32>,
    clip_rect: Option<(Pixels, Pixels, Pixels, Pixels)>,

    // The fields below set Pdfium's page rendering flags. Coverage for the
    // FPDF_DEBUG_INFO and FPDF_NO_CATCH flags is omitted since they are obsolete.
    do_set_flag_render_annotations: bool,     // Sets FPDF_ANNOT
    do_set_flag_use_lcd_text_rendering: bool, // Sets FPDF_LCD_TEXT
    do_set_flag_no_native_text: bool,         // Sets FPDF_NO_NATIVETEXT
    do_set_flag_grayscale: bool,              // Sets FPDF_GRAYSCALE
    do_set_flag_render_limited_image_cache: bool, // Sets FPDF_RENDER_LIMITEDIMAGECACHE
    do_set_flag_render_force_half_tone: bool, // Sets FPDF_RENDER_FORCEHALFTONE
    do_set_flag_render_for_printing: bool,    // Sets FPDF_PRINTING
    do_set_flag_render_no_smooth_text: bool,  // Sets FPDF_RENDER_NO_SMOOTHTEXT
    do_set_flag_render_no_smooth_image: bool, // Sets FPDF_RENDER_NO_SMOOTHIMAGE
    do_set_flag_render_no_smooth_path: bool,  // Sets FPDF_RENDER_NO_SMOOTHPATH
    do_set_flag_reverse_byte_order: bool,     // Sets FPDF_REVERSE_BYTE_ORDER
    do_set_flag_convert_fill_to_stroke: bool, // Sets FPDF_CONVERT_FILL_TO_STROKE
}

impl PdfRenderConfig {
    /// Creates a new [PdfRenderConfig] object with all settings initialized with their default values.
    pub fn new() -> Self {
        PdfRenderConfig {
            target_width: None,
            target_height: None,
            scale_width_factor: None,
            scale_height_factor: None,
            maximum_width: None,
            maximum_height: None,
            portrait_rotation: PdfBitmapRotation::None,
            portrait_rotation_do_rotate_constraints: false,
            landscape_rotation: PdfBitmapRotation::None,
            landscape_rotation_do_rotate_constraints: false,
            format: PdfBitmapFormat::default(),
            do_clear_bitmap_before_rendering: true,
            clear_color: PdfColor::SOLID_WHITE,
            do_render_form_data: true,
            form_field_highlight: None,
            transformation_matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            clip_rect: None,
            do_set_flag_render_annotations: true,
            do_set_flag_use_lcd_text_rendering: false,
            do_set_flag_no_native_text: false,
            do_set_flag_grayscale: false,
            do_set_flag_render_limited_image_cache: false,
            do_set_flag_render_force_half_tone: false,
            do_set_flag_render_for_printing: false,
            do_set_flag_render_no_smooth_text: false,
            do_set_flag_render_no_smooth_image: false,
            do_set_flag_render_no_smooth_path: false,
            do_set_flag_convert_fill_to_stroke: false,

            // We ask Pdfium to reverse its bitmap byte order from BGR8 to RGB8 in order
            // to make working with Image::DynamicImage easier after version 0.24. See:
            // https://github.com/ajrcarey/pdfium-render/issues/9
            do_set_flag_reverse_byte_order: true,
        }
    }

    /// Applies settings suitable for generating a thumbnail.
    ///
    /// * The source [PdfPage] will be rendered with a maximum width and height of the given
    /// pixel size.
    /// * The page will not be rotated, irrespective of its orientation.
    /// * Image quality settings will be reduced to improve performance.
    /// * Annotations and user-filled form field data will not be rendered.
    ///
    /// These settings are applied to this [PdfRenderConfig] object immediately and can be
    /// selectively overridden by later function calls. For instance, a later call to
    /// [PdfRenderConfig::rotate()] can specify a custom rotation setting that will apply
    /// to the thumbnail.
    #[inline]
    pub fn thumbnail(self, size: Pixels) -> Self {
        self.set_target_size(size, size)
            .set_maximum_width(size)
            .set_maximum_height(size)
            .rotate(PdfBitmapRotation::None, false)
            .use_print_quality(false)
            .set_image_smoothing(false)
            .render_annotations(false)
            .render_form_data(false)
    }

    /// Converts the width and height of a [PdfPage] from points to pixels, scaling each
    /// dimension to the given target pixel sizes. The aspect ratio of the source page
    /// will not be maintained.
    #[inline]
    pub fn set_target_size(self, width: Pixels, height: Pixels) -> Self {
        self.set_target_width(width).set_target_height(height)
    }

    /// Converts the width of a [PdfPage] from points to pixels, scaling the source page
    /// width to the given target pixel width. The aspect ratio of the source page
    /// will be maintained so long as there is no call to [PdfRenderConfig::set_target_size()]
    /// or [PdfRenderConfig::set_target_height()] that overrides it.
    #[inline]
    pub fn set_target_width(mut self, width: Pixels) -> Self {
        self.target_width = Some(width);

        self
    }

    /// Converts the height of a [PdfPage] from points to pixels, scaling the source page
    /// height to the given target pixel height. The aspect ratio of the source page
    /// will be maintained so long as there is no call to [PdfRenderConfig::set_target_size()]
    /// or [PdfRenderConfig::set_target_width()] that overrides it.
    #[inline]
    pub fn set_target_height(mut self, height: Pixels) -> Self {
        self.target_height = Some(height);

        self
    }

    /// Applies settings to this [PdfRenderConfig] suitable for filling the given
    /// screen display size.
    ///
    /// The source page's dimensions will be scaled so that both width and height attempt
    /// to fill, but do not exceed, the given pixel dimensions. The aspect ratio of the
    /// source page will be maintained. Landscape pages will be automatically rotated
    /// by 90 degrees and will be scaled down if necessary to fit the display width.
    #[inline]
    pub fn scale_page_to_display_size(mut self, width: Pixels, height: Pixels) -> Self {
        self.scale_width_factor = None;
        self.scale_height_factor = None;

        self.set_target_width(width)
            .set_maximum_width(width)
            .set_maximum_height(height)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true)
    }

    /// Converts the width and height of a [PdfPage] from points to pixels by applying
    /// the given scale factor to both dimensions. The aspect ratio of the source page
    /// will be maintained. Overrides any previous call to [PdfRenderConfig::scale_page_by_factor()],
    /// [PdfRenderConfig::scale_page_width_by_factor()], or [PdfRenderConfig::scale_page_height_by_factor()].
    #[inline]
    pub fn scale_page_by_factor(self, scale: f32) -> Self {
        let result = self.scale_page_width_by_factor(scale);

        result.scale_page_height_by_factor(scale)
    }

    /// Converts the width of the [PdfPage] from points to pixels by applying the given
    /// scale factor. The aspect ratio of the source page will not be maintained if a
    /// different scale factor is applied to the height. Overrides any previous call to
    /// [PdfRenderConfig::scale_page_by_factor()], [PdfRenderConfig::scale_page_width_by_factor()],
    /// or [PdfRenderConfig::scale_page_height_by_factor()].
    #[inline]
    pub fn scale_page_width_by_factor(mut self, scale: f32) -> Self {
        self.scale_width_factor = Some(scale);

        self
    }

    /// Converts the height of the [PdfPage] from points to pixels by applying the given
    /// scale factor. The aspect ratio of the source page will not be maintained if a
    /// different scale factor is applied to the width. Overrides any previous call to
    /// [PdfRenderConfig::scale_page_by_factor()], [PdfRenderConfig::scale_page_width_by_factor()],
    /// or [PdfRenderConfig::scale_page_height_by_factor()].
    #[inline]
    pub fn scale_page_height_by_factor(mut self, scale: f32) -> Self {
        self.scale_height_factor = Some(scale);

        self
    }

    /// Specifies that the final pixel width of the [PdfPage] will not exceed the given maximum.
    #[inline]
    pub fn set_maximum_width(mut self, width: Pixels) -> Self {
        self.maximum_width = Some(width);

        self
    }

    /// Specifies that the final pixel height of the [PdfPage] will not exceed the given maximum.
    #[inline]
    pub fn set_maximum_height(mut self, height: Pixels) -> Self {
        self.maximum_height = Some(height);

        self
    }

    /// Applies the given clockwise rotation setting to the [PdfPage] during rendering, irrespective
    /// of its orientation. If the given flag is set to `true` then any maximum
    /// constraint on the final pixel width set by a call to [PdfRenderConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height, and any
    /// maximum constraint on the final pixel height set by a call to [PdfRenderConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate(self, rotation: PdfBitmapRotation, do_rotate_constraints: bool) -> Self {
        self.rotate_if_portrait(rotation, do_rotate_constraints)
            .rotate_if_landscape(rotation, do_rotate_constraints)
    }

    // TODO: AJRC - 30/7/22 - remove deprecated rotate_if_portait() function in 0.9.0 as part
    // of tracking issue https://github.com/ajrcarey/pdfium-render/issues/36
    /// Applies the given clockwise rotation settings to the [PdfPage] during rendering, if the page
    /// is in portrait orientation. If the given flag is set to `true` and the given
    /// rotation setting is [PdfBitmapRotation::Degrees90] or [PdfBitmapRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfRenderConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfRenderConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[deprecated(
        since = "0.7.12",
        note = "This function has been renamed to correct a typo. Use the PdfRenderConfig::rotate_if_portrait() function instead."
    )]
    #[doc(hidden)]
    #[inline]
    pub fn rotate_if_portait(
        self,
        rotation: PdfBitmapRotation,
        do_rotate_constraints: bool,
    ) -> Self {
        self.rotate_if_portrait(rotation, do_rotate_constraints)
    }

    /// Applies the given clockwise rotation settings to the [PdfPage] during rendering, if the page
    /// is in portrait orientation. If the given flag is set to `true` and the given
    /// rotation setting is [PdfBitmapRotation::Degrees90] or [PdfBitmapRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfRenderConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfRenderConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate_if_portrait(
        mut self,
        rotation: PdfBitmapRotation,
        do_rotate_constraints: bool,
    ) -> Self {
        self.portrait_rotation = rotation;

        if rotation == PdfBitmapRotation::Degrees90 || rotation == PdfBitmapRotation::Degrees270 {
            self.portrait_rotation_do_rotate_constraints = do_rotate_constraints;
        }

        self
    }

    /// Applies the given rotation settings to the [PdfPage] during rendering, if the page
    /// is in landscape orientation. If the given flag is set to `true` and the given
    /// rotation setting is [PdfBitmapRotation::Degrees90] or [PdfBitmapRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfRenderConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfRenderConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate_if_landscape(
        mut self,
        rotation: PdfBitmapRotation,
        do_rotate_constraints: bool,
    ) -> Self {
        self.landscape_rotation = rotation;

        if rotation == PdfBitmapRotation::Degrees90 || rotation == PdfBitmapRotation::Degrees270 {
            self.landscape_rotation_do_rotate_constraints = do_rotate_constraints;
        }

        self
    }

    /// Sets the pixel format that will be used during rendering of the [PdfPage].
    /// The default is [PdfBitmapFormat::BGRA].
    #[inline]
    pub fn set_format(mut self, format: PdfBitmapFormat) -> Self {
        self.format = format;

        self
    }

    /// Controls whether the destination bitmap should be cleared by setting every pixel to a
    /// known color value before rendering the [PdfPage]. The default is `true`.
    /// The color used during clearing can be customised by calling [PdfRenderConfig::set_clear_color()].
    #[inline]
    pub fn clear_before_rendering(mut self, do_clear: bool) -> Self {
        self.do_clear_bitmap_before_rendering = do_clear;

        self
    }

    /// Sets the color applied to every pixel in the destination bitmap when clearing the bitmap
    /// before rendering the [PdfPage]. The default is [PdfColor::SOLID_WHITE]. This setting
    /// has no effect if [PdfRenderConfig::clear_before_rendering()] is set to `false`.
    #[inline]
    pub fn set_clear_color(mut self, color: PdfColor) -> Self {
        self.clear_color = color;

        self
    }

    /// Controls whether form data widgets and user-supplied form data should be included
    /// during rendering of the [PdfPage]. The default is `true`.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically sets this value to `false`, disabling rendering of form data.
    #[inline]
    pub fn render_form_data(mut self, do_render: bool) -> Self {
        self.do_render_form_data = do_render;

        self
    }

    /// Controls whether user-supplied annotations should be included during rendering of
    /// the [PdfPage]. The default is `true`.
    #[inline]
    pub fn render_annotations(mut self, do_render: bool) -> Self {
        self.do_set_flag_render_annotations = do_render;

        self
    }

    /// Controls whether text rendering should be optimized for LCD display.
    /// The default is `false`.
    /// Has no effect if anti-aliasing of text has been disabled by a call to
    /// `PdfRenderConfig::set_text_smoothing(false)`.
    #[inline]
    pub fn use_lcd_text_rendering(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_use_lcd_text_rendering = do_set_flag;

        self
    }

    /// Controls whether platform text rendering should be disabled on platforms that support it.
    /// The alternative is for Pdfium to render all text internally, which may give more
    /// consistent rendering results across platforms but may also be slower.
    /// The default is `false`.
    #[inline]
    pub fn disable_native_text_rendering(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_no_native_text = do_set_flag;

        self
    }

    /// Controls whether rendering output should be grayscale rather than full color.
    /// The default is `false`.
    #[inline]
    pub fn use_grayscale_rendering(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_grayscale = do_set_flag;

        self
    }

    /// Controls whether Pdfium should limit its image cache size during rendering.
    /// A smaller cache size may result in lower memory usage at the cost of slower rendering.
    /// The default is `false`.
    #[inline]
    pub fn limit_render_image_cache_size(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_limited_image_cache = do_set_flag;

        self
    }

    /// Controls whether Pdfium should always use halftone for image stretching.
    /// Halftone image stretching is often higher quality than linear image stretching
    /// but is much slower. The default is `false`.
    #[inline]
    pub fn force_half_tone(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_force_half_tone = do_set_flag;

        self
    }

    /// Controls whether Pdfium should render for printing. The default is `false`.
    ///
    /// Certain PDF files may stipulate different quality settings for on-screen display
    /// compared to printing. For these files, changing this setting to `true` will result
    /// in a higher quality rendered bitmap but slower performance. For PDF files that do
    /// not stipulate different quality settings, changing this setting will have no effect.
    #[inline]
    pub fn use_print_quality(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_for_printing = do_set_flag;

        self
    }

    /// Controls whether rendered text should be anti-aliased.
    /// The default is `true`.
    /// The enabling of LCD-optimized text rendering via a call to
    /// `PdfiumBitmapConfig::use_lcd_text_rendering(true)` has no effect if this flag
    /// is set to `false`.
    #[inline]
    pub fn set_text_smoothing(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_no_smooth_text = !do_set_flag;

        self
    }

    /// Controls whether rendered images should be anti-aliased.
    /// The default is `true`.
    #[inline]
    pub fn set_image_smoothing(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_no_smooth_image = !do_set_flag;

        self
    }

    /// Controls whether rendered vector paths should be anti-aliased.
    /// The default is `true`.
    #[inline]
    pub fn set_path_smoothing(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_no_smooth_path = !do_set_flag;

        self
    }

    /// Controls whether the byte order of generated image data should be reversed
    /// during rendering. The default is `true`, so that Pdfium returns pixel data as RGB8
    /// rather than its default BGR8. There should generally be no need to change this flag,
    /// unless you want to do raw image processing and specifically need the pixel data returned
    /// by the `PdfBitmap::as_bytes()` function to be in BGR8 format.
    #[inline]
    pub fn set_reverse_byte_order(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_reverse_byte_order = do_set_flag;

        self
    }

    /// Controls whether rendered vector fill paths need to be stroked.
    /// The default is `false`.
    #[inline]
    pub fn render_fills_as_strokes(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_convert_fill_to_stroke = do_set_flag;

        self
    }

    /// Highlights all rendered form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_all_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::Unknown, color)
    }

    /// Highlights all rendered push button form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_button_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::PushButton, color)
    }

    /// Highlights all rendered checkbox form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_checkbox_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::Checkbox, color)
    }

    /// Highlights all rendered radio button form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_radio_button_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::RadioButton, color)
    }

    /// Highlights all rendered combobox form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_combobox_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::ComboBox, color)
    }

    /// Highlights all rendered listbox form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_listbox_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::ListBox, color)
    }

    /// Highlights all rendered text entry form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_text_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::TextField, color)
    }

    /// Highlights all rendered signature form fields with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_signature_form_fields(self, color: PdfColor) -> Self {
        self.highlight_form_fields_of_type(PdfFormFieldType::Signature, color)
    }

    /// Highlights all rendered form fields matching the given type with the given color.
    /// Note that specifying a solid color with no opacity will overprint any user data in the field.
    #[inline]
    pub fn highlight_form_fields_of_type(
        mut self,
        form_field_type: PdfFormFieldType,
        color: PdfColor,
    ) -> Self {
        if let Some(form_field_highlight) = self.form_field_highlight.as_mut() {
            form_field_highlight.push((form_field_type, color));
        } else {
            self.form_field_highlight = Some(vec![(form_field_type, color)]);
        }

        self
    }

    /// Applies the given transformation, expressed as six values representing the six configurable
    /// elements of a nine-element 3x3 PDF transformation matrix, to a [PdfPage] during rendering.
    ///
    /// The transformation will be rejected with an error if applying it would result in
    /// a transformation matrix with a determinant of zero. Pdfium's page rendering behaviour
    /// is undefined if the transformation matrix has a determinant of zero.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    ///
    /// To move, scale, rotate, or skew a [PdfPage] during rendering, consider using one or more of the
    /// following functions. Internally they all use [PdfRenderConfig::transform()], but are
    /// probably easier to use (and certainly clearer in their intent) in most situations.
    ///
    /// * [PdfRenderConfig::translate()]: changes the origin of the rendered [PdfPage].
    /// * [PdfRenderConfig::scale()]: changes the size of the rendered [PdfPage].
    /// * [PdfRenderConfig::rotate_clockwise_degrees()], [PdfRenderConfig::rotate_counter_clockwise_degrees()],
    /// [PdfRenderConfig::rotate_clockwise_radians()], [PdfRenderConfig::rotate_counter_clockwise_radians()]:
    /// rotates the rendered [PdfPage] around the origin.
    /// * [PdfRenderConfig::skew_degrees()], [PdfRenderConfig::skew_radians()]: skews the rendered [PdfPage]
    /// relative to its axes.
    ///
    /// **The order in which transformations are applied is significant.**
    /// For example, the result of rotating _then_ translating a page during rendering may be
    /// vastly different from translating _then_ rotating the same page. In general, to obtain the
    /// expected results, transformations should be performed in the following order:
    /// * Scale and/or skew
    /// * Rotate
    /// * Translate
    ///
    /// An overview of PDF transformation matrices can be found in the PDF Reference Manual
    /// version 1.7 on page 204; a detailed description can be founded in section 4.2.3 on page 207.
    // AJRC - 5/3/23 - unfortunately we cannot (easily) implement the WriteTransforms trait
    // for PdfRenderConfig, because functions in PdfRenderConfig take mut self but functions
    // in the trait take &mut self. I'm not sure there's an elegant way around this yet, aside
    // from having separate traits - but since only PdfRenderConfig uses the mut self function
    // chaining pattern, a separate trait would defeat the purpose.
    pub fn transform(
        mut self,
        a: PdfMatrixValue,
        b: PdfMatrixValue,
        c: PdfMatrixValue,
        d: PdfMatrixValue,
        e: PdfMatrixValue,
        f: PdfMatrixValue,
    ) -> Result<Self, PdfiumError> {
        let result = row_mat3_mul(
            self.transformation_matrix,
            [[a, b, 0.0], [c, d, 0.0], [e, f, 1.0]],
        );

        if mat3_det(result) == 0.0 {
            Err(PdfiumError::InvalidTransformationMatrix)
        } else {
            self.transformation_matrix = result;
            self.do_render_form_data = false;

            Ok(self)
        }
    }

    /// Moves the origin of a [PdfPage] by the given horizontal and vertical distances during rendering.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn translate(self, delta_x: PdfPoints, delta_y: PdfPoints) -> Result<Self, PdfiumError> {
        self.transform(
            1.0,
            0.0,
            0.0,
            1.0,
            delta_x.value as f32,
            delta_y.value as f32,
        )
    }

    /// Changes the size of a [PdfPage] during rendering, scaling it by the given horizontal and
    /// vertical scale factors.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn scale(
        self,
        horizontal_scale_factor: f32,
        vertical_scale_factor: f32,
    ) -> Result<Self, PdfiumError> {
        self.transform(
            horizontal_scale_factor,
            0.0,
            0.0,
            vertical_scale_factor,
            0.0,
            0.0,
        )
    }

    /// Flips a [PdfPage] horizontally around the origin during rendering by applying
    /// a horizontal scale factor of -1.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn flip_horizontally(self) -> Result<Self, PdfiumError> {
        self.scale(-1.0, 1.0)
    }

    /// Flips a [PdfPage] vertically around the origin during rendering by applying
    /// a vertical scale factor of -1.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn flip_vertically(self) -> Result<Self, PdfiumError> {
        self.scale(1.0, -1.0)
    }

    /// Reflects a [PdfPage] by flipping it both horizontally and vertically around the origin
    /// during rendering.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn reflect(self) -> Result<Self, PdfiumError> {
        self.scale(-1.0, -1.0)
    }

    /// Rotates a [PdfPage] counter-clockwise during rendering by the given number of degrees.
    ///
    /// Any constraints on the final pixel width or height of the rendered image, set by
    /// a call to [PdfRenderConfig::set_maximum_width()] or [PdfRenderConfig::set_maximum_height()],
    /// are not rotated. To rotate a page by a 90-degree increment while also rotating any constraints
    /// on the final pixel width and height of the rendered image, use one of the
    /// [PdfRenderConfig::rotate()], [PdfRenderConfig::rotate_if_portrait()], or
    /// [PdfRenderConfig::rotate_if_landscape()] functions.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn rotate_counter_clockwise_degrees(self, degrees: f32) -> Result<Self, PdfiumError> {
        self.rotate_clockwise_degrees(-degrees)
    }

    /// Rotates a [PdfPage] clockwise during rendering by the given number of degrees.
    ///
    /// Any constraints on the final pixel width or height of the rendered image, set by
    /// a call to [PdfRenderConfig::set_maximum_width()] or [PdfRenderConfig::set_maximum_height()],
    /// are not rotated. To rotate a page by a 90-degree increment while also rotating any constraints
    /// on the final pixel width and height of the rendered image, use one of the
    /// [PdfRenderConfig::rotate()], [PdfRenderConfig::rotate_if_portrait()], or
    /// [PdfRenderConfig::rotate_if_landscape()] functions.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn rotate_clockwise_degrees(self, degrees: f32) -> Result<Self, PdfiumError> {
        self.rotate_clockwise_radians(degrees.to_radians())
    }

    /// Rotates a [PdfPage] counter-clockwise during rendering by the given number of radians.
    ///
    /// Any constraints on the final pixel width or height of the rendered image, set by
    /// a call to [PdfRenderConfig::set_maximum_width()] or [PdfRenderConfig::set_maximum_height()],
    /// are not rotated. To rotate a page by a 90-degree increment while also rotating any constraints
    /// on the final pixel width and height of the rendered image, use one of the
    /// [PdfRenderConfig::rotate()], [PdfRenderConfig::rotate_if_portrait()], or
    /// [PdfRenderConfig::rotate_if_landscape()] functions.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn rotate_counter_clockwise_radians(self, radians: f32) -> Result<Self, PdfiumError> {
        self.rotate_clockwise_radians(-radians)
    }

    /// Rotates a [PdfPage] clockwise during rendering by the given number of radians.
    ///
    /// Any constraints on the final pixel width or height of the rendered image, set by
    /// a call to [PdfRenderConfig::set_maximum_width()] or [PdfRenderConfig::set_maximum_height()],
    /// are not rotated. To rotate a page by a 90-degree increment while also rotating any constraints
    /// on the final pixel width and height of the rendered image, use one of the
    /// [PdfRenderConfig::rotate()], [PdfRenderConfig::rotate_if_portrait()], or
    /// [PdfRenderConfig::rotate_if_landscape()] functions.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn rotate_clockwise_radians(self, radians: f32) -> Result<Self, PdfiumError> {
        let cos_theta = radians.cos();

        let sin_theta = radians.sin();

        self.transform(cos_theta, sin_theta, -sin_theta, cos_theta, 0.0, 0.0)
    }

    /// Skews the axes of a [PdfPage] during rendering by the given angles in degrees.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn skew_degrees(self, x_axis_skew: f32, y_axis_skew: f32) -> Result<Self, PdfiumError> {
        self.skew_radians(x_axis_skew.to_radians(), y_axis_skew.to_radians())
    }

    /// Skews the axes of a [PdfPage] during rendering by the given angles in radians.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
    /// a custom transformation matrix, but not both at the same time. Applying any transformation
    /// automatically disables rendering of form data. If you must render form data while simultaneously
    /// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn skew_radians(self, x_axis_skew: f32, y_axis_skew: f32) -> Result<Self, PdfiumError> {
        let tan_alpha = x_axis_skew.tan();

        let tan_beta = y_axis_skew.tan();

        self.transform(1.0, tan_alpha, tan_beta, 1.0, 0.0, 0.0)
    }

    /// Clips rendering output to the given pixel coordinates.
    #[inline]
    pub fn clip(mut self, left: Pixels, top: Pixels, right: Pixels, bottom: Pixels) -> Self {
        self.clip_rect = Some((left, top, right, bottom));

        self
    }

    /// Computes the pixel dimensions and rotation settings for the given [PdfPage]
    /// based on the configuration of this [PdfRenderConfig].
    #[inline]
    pub(crate) fn apply_to_page(&self, page: &PdfPage) -> PdfRenderSettings {
        let source_width = page.width();

        let source_height = page.height();

        let source_orientation =
            PdfPageOrientation::from_width_and_height(source_width, source_height);

        // Do we need to apply any rotation?

        let (target_rotation, do_rotate_constraints) = if source_orientation == Portrait
            && self.portrait_rotation != PdfBitmapRotation::None
        {
            (
                self.portrait_rotation,
                self.portrait_rotation_do_rotate_constraints,
            )
        } else if source_orientation == Landscape
            && self.landscape_rotation != PdfBitmapRotation::None
        {
            (
                self.landscape_rotation,
                self.landscape_rotation_do_rotate_constraints,
            )
        } else {
            (PdfBitmapRotation::None, false)
        };

        let width_scale = if let Some(scale) = self.scale_width_factor {
            Some(scale)
        } else {
            self.target_width
                .map(|target| (target as f32) / source_width.value)
        };

        let height_scale = if let Some(scale) = self.scale_height_factor {
            Some(scale)
        } else {
            self.target_height
                .map(|target| (target as f32) / source_height.value)
        };

        // Maintain source aspect ratio if only one dimension's scale is set.

        let (do_maintain_aspect_ratio, mut width_scale, mut height_scale) =
            match (width_scale, height_scale) {
                (Some(width_scale), Some(height_scale)) => {
                    (width_scale == height_scale, width_scale, height_scale)
                }
                (Some(width_scale), None) => (true, width_scale, width_scale),
                (None, Some(height_scale)) => (true, height_scale, height_scale),
                (None, None) => {
                    // Set default scale to 1.0 if neither dimension is specified.

                    (false, 1.0, 1.0)
                }
            };

        // Apply constraints on maximum width and height, if any.

        let (source_width, source_height, width_constraint, height_constraint) =
            if do_rotate_constraints {
                (
                    source_height,
                    source_width,
                    self.maximum_height,
                    self.maximum_width,
                )
            } else {
                (
                    source_width,
                    source_height,
                    self.maximum_width,
                    self.maximum_height,
                )
            };

        if let Some(maximum) = width_constraint {
            let maximum = maximum as f32;

            if source_width.value * width_scale > maximum {
                // Constrain the width, so it does not exceed the maximum.

                width_scale = maximum / source_width.value;

                if do_maintain_aspect_ratio {
                    height_scale = width_scale;
                }
            }
        }

        if let Some(maximum) = height_constraint {
            let maximum = maximum as f32;

            if source_height.value * height_scale > maximum {
                // Constrain the height, so it does not exceed the maximum.

                height_scale = maximum / source_height.value;

                if do_maintain_aspect_ratio {
                    width_scale = height_scale;
                }
            }
        }

        // Compose render flags.

        let mut render_flags = 0;

        if self.do_set_flag_render_annotations {
            render_flags |= FPDF_ANNOT;
        }

        if self.do_set_flag_use_lcd_text_rendering {
            render_flags |= FPDF_LCD_TEXT;
        }

        if self.do_set_flag_no_native_text {
            render_flags |= FPDF_NO_NATIVETEXT;
        }

        if self.do_set_flag_grayscale {
            render_flags |= FPDF_GRAYSCALE;
        }

        if self.do_set_flag_render_limited_image_cache {
            render_flags |= FPDF_RENDER_LIMITEDIMAGECACHE;
        }

        if self.do_set_flag_render_force_half_tone {
            render_flags |= FPDF_RENDER_FORCEHALFTONE;
        }

        if self.do_set_flag_render_for_printing {
            render_flags |= FPDF_PRINTING;
        }

        if self.do_set_flag_render_no_smooth_text {
            render_flags |= FPDF_RENDER_NO_SMOOTHTEXT;
        }

        if self.do_set_flag_render_no_smooth_image {
            render_flags |= FPDF_RENDER_NO_SMOOTHIMAGE;
        }

        if self.do_set_flag_render_no_smooth_path {
            render_flags |= FPDF_RENDER_NO_SMOOTHPATH;
        }

        if self.do_set_flag_reverse_byte_order {
            render_flags |= FPDF_REVERSE_BYTE_ORDER;
        }

        if self.do_set_flag_convert_fill_to_stroke {
            render_flags |= FPDF_CONVERT_FILL_TO_STROKE;
        }

        let output_width = (source_width.value * width_scale) as i32;

        let output_height = (source_height.value * height_scale) as i32;

        // Pages can be rendered either _with_ transformation matrices but _without_ form data,
        // or _with_ form data but _without_ transformation matrices. We need to be prepared
        // for either option. If rendering of form data is disabled then the scaled output
        // width and height and any user-specified 90-degree rotation need to be applied to the
        // transformation matrix now.

        let transformation_matrix = if !self.do_render_form_data {
            let result = if target_rotation != PdfBitmapRotation::None {
                // Translate the origin to the center of the page before rotating.

                let (delta_x, delta_y) = match target_rotation {
                    PdfBitmapRotation::None => unreachable!(),
                    PdfBitmapRotation::Degrees90 => (0.0, -source_width.value),
                    PdfBitmapRotation::Degrees180 => (-source_width.value, -source_height.value),
                    PdfBitmapRotation::Degrees270 => (-source_height.value, 0.0),
                };

                let result = row_mat3_mul(
                    self.transformation_matrix,
                    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [delta_x, delta_y, 1.0]],
                );

                let cos_theta = target_rotation.as_radians_cos();

                let sin_theta = target_rotation.as_radians_sin();

                row_mat3_mul(
                    result,
                    [
                        [cos_theta, sin_theta, 0.0],
                        [-sin_theta, cos_theta, 0.0],
                        [0.0, 0.0, 1.0],
                    ],
                )
            } else {
                self.transformation_matrix
            };

            row_mat3_mul(
                result,
                [
                    [width_scale, 0.0, 0.0],
                    [0.0, height_scale, 0.0],
                    [0.0, 0.0, 1.0],
                ],
            )
        } else {
            self.transformation_matrix
        };

        PdfRenderSettings {
            width: output_width,
            height: output_height,
            format: self.format.as_pdfium() as i32,
            rotate: target_rotation.as_pdfium(),
            do_clear_bitmap_before_rendering: self.do_clear_bitmap_before_rendering,
            clear_color: self.clear_color.as_pdfium_color(),
            do_render_form_data: self.do_render_form_data,
            form_field_highlight: if !self.do_render_form_data
                || self.form_field_highlight.is_none()
            {
                None
            } else {
                Some(
                    self.form_field_highlight
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(form_field_type, color)| {
                            (
                                form_field_type.as_pdfium() as i32,
                                color.as_pdfium_color_with_alpha(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            },
            matrix: FS_MATRIX {
                a: transformation_matrix[0][0],
                b: transformation_matrix[0][1],
                c: transformation_matrix[1][0],
                d: transformation_matrix[1][1],
                e: transformation_matrix[2][0],
                f: transformation_matrix[2][1],
            },
            clipping: if let Some((left, top, right, bottom)) = self.clip_rect {
                FS_RECTF {
                    left: left as f32,
                    top: top as f32,
                    right: right as f32,
                    bottom: bottom as f32,
                }
            } else {
                FS_RECTF {
                    left: 0.0,
                    top: 0.0,
                    right: output_width as f32,
                    bottom: output_height as f32,
                }
            },
            render_flags: render_flags as i32,
        }
    }
}

impl Default for PdfRenderConfig {
    #[inline]
    fn default() -> Self {
        PdfRenderConfig::new()
    }
}

/// Finalized rendering settings, ready to be passed to a Pdfium rendering function.
/// Generated by calling [PdfRenderConfig::apply_to_page()].
#[derive(Debug, Clone)]
pub(crate) struct PdfRenderSettings {
    pub(crate) width: c_int,
    pub(crate) height: c_int,
    pub(crate) format: c_int,
    pub(crate) rotate: c_int,
    pub(crate) do_clear_bitmap_before_rendering: bool,
    pub(crate) clear_color: FPDF_DWORD,
    pub(crate) do_render_form_data: bool,
    pub(crate) form_field_highlight: Option<Vec<(c_int, (FPDF_DWORD, u8))>>,
    pub(crate) matrix: FS_MATRIX,
    pub(crate) clipping: FS_RECTF,
    pub(crate) render_flags: c_int,
}
