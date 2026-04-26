//! Defines the [PdfRenderConfig] struct, a builder-based approach to configuring
//! the rendering of [PdfBitmap] objects from one or more [PdfPage] objects.

use crate::bindgen::{
    FPDF_ANNOT, FPDF_CONVERT_FILL_TO_STROKE, FPDF_DWORD, FPDF_GRAYSCALE, FPDF_LCD_TEXT,
    FPDF_NO_NATIVETEXT, FPDF_PRINTING, FPDF_RENDER_FORCEHALFTONE, FPDF_RENDER_LIMITEDIMAGECACHE,
    FPDF_RENDER_NO_SMOOTHIMAGE, FPDF_RENDER_NO_SMOOTHPATH, FPDF_RENDER_NO_SMOOTHTEXT,
    FPDF_REVERSE_BYTE_ORDER, FS_MATRIX, FS_RECTF,
};
use crate::create_transform_setters;
use crate::error::PdfiumError;
use crate::pdf::bitmap::{PdfBitmap, PdfBitmapFormat, Pixels};
use crate::pdf::color::PdfColor;
use crate::pdf::document::page::field::PdfFormFieldType;
use crate::pdf::document::page::PdfPageOrientation::{Landscape, Portrait};
use crate::pdf::document::page::{PdfPage, PdfPageOrientation, PdfPageRenderRotation};
use crate::pdf::matrix::{PdfMatrix, PdfMatrixValue};
use crate::pdf::points::PdfPoints;
use std::os::raw::c_int;

/// Configures the scaling, rotation, and rendering settings that should be applied to
/// a [PdfPage] to create a [PdfBitmap] for that page. [PdfRenderConfig] can accommodate pages of
/// different sizes while correctly maintaining each page's aspect ratio, automatically
/// rotate portrait or landscape pages, generate page thumbnails, apply maximum pixel size
/// constraints to the scaled width and height of the final rendering, highlight form fields
/// with different colors, apply custom transforms to the page during rendering, and set
/// internal Pdfium rendering flags.
///
/// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
/// a custom transformation matrix, but not both at the same time. Applying any transformation
/// automatically disables rendering of form data. If you must render form data while simultaneously
/// applying transformations, consider using the [PdfPage::flatten()] function to flatten the
/// form elements and form data into the containing page.
pub struct PdfRenderConfig {
    use_auto_scaling: bool,
    fixed_width: Option<Pixels>,
    fixed_height: Option<Pixels>,
    target_width: Option<Pixels>,
    target_height: Option<Pixels>,
    scale_width_factor: Option<f32>,
    scale_height_factor: Option<f32>,
    maximum_width: Option<Pixels>,
    maximum_height: Option<Pixels>,
    portrait_rotation: PdfPageRenderRotation,
    portrait_rotation_do_rotate_constraints: bool,
    landscape_rotation: PdfPageRenderRotation,
    landscape_rotation_do_rotate_constraints: bool,
    format: PdfBitmapFormat,
    do_clear_bitmap_before_rendering: bool,
    clear_color: PdfColor,
    do_render_form_data: bool,
    form_field_highlight: Option<Vec<(PdfFormFieldType, PdfColor)>>,
    transformation_matrix: PdfMatrix,
    clip_rect: Option<(Pixels, Pixels, Pixels, Pixels)>,
    /// When `true`, the matrix render path composes the page's intrinsic
    /// `/Rotate` with [`Self::transformation_matrix`] before passing the
    /// result to `FPDF_RenderPageBitmapWithMatrix`. Lets matrix-path
    /// callers write their transform in display-oriented coordinates
    /// (the same coordinate system the form-data path speaks) instead
    /// of hand-deriving a per-`/Rotate` matrix. Default `false` —
    /// existing callers supply a matrix in pre-rotation page coordinates
    /// and changing that under their feet would silently flip output for
    /// every rotated PDF.
    auto_apply_intrinsic_rotation: bool,

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
    /// Creates a new [PdfRenderConfig] object with all settings initialized to their default values.
    pub fn new() -> Self {
        PdfRenderConfig {
            use_auto_scaling: true,
            fixed_width: None,
            fixed_height: None,
            target_width: None,
            target_height: None,
            scale_width_factor: None,
            scale_height_factor: None,
            maximum_width: None,
            maximum_height: None,
            portrait_rotation: PdfPageRenderRotation::None,
            portrait_rotation_do_rotate_constraints: false,
            landscape_rotation: PdfPageRenderRotation::None,
            landscape_rotation_do_rotate_constraints: false,
            format: PdfBitmapFormat::default(),
            do_clear_bitmap_before_rendering: true,
            clear_color: PdfColor::WHITE,
            do_render_form_data: true,
            form_field_highlight: None,
            transformation_matrix: PdfMatrix::IDENTITY,
            clip_rect: None,
            auto_apply_intrinsic_rotation: false,
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
    ///   pixel size.
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
            .rotate(PdfPageRenderRotation::None, false)
            .use_print_quality(false)
            .set_image_smoothing(false)
            .render_annotations(false)
            .render_form_data(false)
    }

    /// Sets the desired pixel width and height of a rendered [PdfPage] to the
    /// width and height of the given [PdfBitmap]. No attempt will be made to scale or adjust
    /// the aspect ratio to match the source page. Overrides any previous call to
    /// [PdfRenderConfig::set_target_size], [PdfRenderConfig::set_target_width], or
    /// [PdfRenderConfig::set_target_height].
    #[inline]
    pub fn set_fixed_size_to_bitmap(self, bitmap: &PdfBitmap) -> Self {
        self.set_fixed_size(bitmap.width(), bitmap.height())
    }

    /// Sets the desired pixel width and height of a rendered [PdfPage] to the given
    /// pixel values. No attempt will be made to scale or adjust the aspect ratio to
    /// match the source page. Overrides any previous call to [PdfRenderConfig::set_target_size],
    /// [PdfRenderConfig::set_target_width], or [PdfRenderConfig::set_target_height].
    #[inline]
    pub fn set_fixed_size(self, width: Pixels, height: Pixels) -> Self {
        self.set_fixed_width(width).set_fixed_height(height)
    }

    /// Sets the desired pixel width of a rendered [PdfPage] to the given value. Overrides
    /// any previous call to [PdfRenderConfig::set_target_size] or [PdfRenderConfig::set_target_width].
    #[inline]
    pub fn set_fixed_width(mut self, width: Pixels) -> Self {
        self.use_auto_scaling = false;
        self.fixed_width = Some(width);

        self
    }

    /// Sets the desired pixel height of a rendered [PdfPage] to the given value. Overrides
    /// any previous call to [PdfRenderConfig::set_target_size] or [PdfRenderConfig::set_target_height].
    #[inline]
    pub fn set_fixed_height(mut self, height: Pixels) -> Self {
        self.use_auto_scaling = false;
        self.fixed_height = Some(height);

        self
    }

    /// Converts the width and height of a [PdfPage] from points to pixels, scaling each
    /// dimension to the given target pixel sizes. The aspect ratio of the source page
    /// will not be maintained. Overrides any previous call to [PdfRenderConfig::set_fixed_width()]
    /// or [PdfRenderConfig::set_fixed_height()].
    #[inline]
    pub fn set_target_size(self, width: Pixels, height: Pixels) -> Self {
        self.set_target_width(width).set_target_height(height)
    }

    /// Converts the width of a [PdfPage] from points to pixels, scaling the source page
    /// width to the given target pixel width. The aspect ratio of the source page
    /// will be maintained so long as there is no call to [PdfRenderConfig::set_target_size()]
    /// or [PdfRenderConfig::set_target_height()] that overrides it. Overrides any previous
    /// call to [PdfRenderConfig::set_fixed_width()] or [PdfRenderConfig::set_fixed_height()].
    #[inline]
    pub fn set_target_width(mut self, width: Pixels) -> Self {
        self.use_auto_scaling = true;
        self.target_width = Some(width);

        self
    }

    /// Converts the height of a [PdfPage] from points to pixels, scaling the source page
    /// height to the given target pixel height. The aspect ratio of the source page
    /// will be maintained so long as there is no call to [PdfRenderConfig::set_target_size()]
    /// or [PdfRenderConfig::set_target_width()] that overrides it. Overrides any previous
    /// call to [PdfRenderConfig::set_fixed_width()] or [PdfRenderConfig::set_fixed_height()].
    #[inline]
    pub fn set_target_height(mut self, height: Pixels) -> Self {
        self.use_auto_scaling = true;
        self.target_height = Some(height);

        self
    }

    /// Applies settings to this [PdfRenderConfig] suitable for filling the given [PdfBitmap].
    ///
    /// The source page's dimensions will be scaled so that both width and height attempt
    /// to fill, but do not exceed, the pixel dimensions of the bitmap. The aspect ratio
    /// of the source page will be maintained. Landscape pages will be automatically rotated
    /// by 90 degrees and will be scaled down if necessary to fit the bitmap width.
    #[inline]
    pub fn scale_page_to_bitmap(self, bitmap: &PdfBitmap) -> Self {
        self.scale_page_to_display_size(bitmap.width(), bitmap.height())
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
            .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)
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
    pub fn rotate(self, rotation: PdfPageRenderRotation, do_rotate_constraints: bool) -> Self {
        self.rotate_if_portrait(rotation, do_rotate_constraints)
            .rotate_if_landscape(rotation, do_rotate_constraints)
    }

    /// Applies the given clockwise rotation settings to the [PdfPage] during rendering, if the page
    /// is in portrait orientation. If the given flag is set to `true` and the given
    /// rotation setting is [PdfPageRenderRotation::Degrees90] or [PdfPageRenderRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfRenderConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfRenderConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate_if_portrait(
        mut self,
        rotation: PdfPageRenderRotation,
        do_rotate_constraints: bool,
    ) -> Self {
        self.portrait_rotation = rotation;

        if rotation == PdfPageRenderRotation::Degrees90
            || rotation == PdfPageRenderRotation::Degrees270
        {
            self.portrait_rotation_do_rotate_constraints = do_rotate_constraints;
        }

        self
    }

    /// Applies the given rotation settings to the [PdfPage] during rendering, if the page
    /// is in landscape orientation. If the given flag is set to `true` and the given
    /// rotation setting is [PdfPageRenderRotation::Degrees90] or [PdfPageRenderRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfRenderConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfRenderConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate_if_landscape(
        mut self,
        rotation: PdfPageRenderRotation,
        do_rotate_constraints: bool,
    ) -> Self {
        self.landscape_rotation = rotation;

        if rotation == PdfPageRenderRotation::Degrees90
            || rotation == PdfPageRenderRotation::Degrees270
        {
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
    /// before rendering the [PdfPage]. The default is [PdfColor::WHITE]. This setting
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
    /// during rendering. The default is `true`, so that Pdfium returns pixel data as
    /// four-channel RGBA rather than its default of four-channel BGRA.
    ///
    /// There should generally be no need to change this flag unless you want to do raw
    /// image processing and specifically need the pixel data returned by the
    /// [PdfBitmap::as_raw_bytes()] function to be in BGR8 format.
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
        self.highlight_form_fields_of_type(PdfFormFieldType::Text, color)
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

    create_transform_setters!(
        Self,
        Result<Self, PdfiumError>,
        "the [PdfPage] during rendering",
        "the [PdfPage] during rendering.",
        "the [PdfPage] during rendering,",
        "Pdfium's rendering pipeline supports _either_ rendering with form data _or_ rendering with
            a custom transformation matrix, but not both at the same time. Applying any transformation
            automatically disables rendering of form data. If you must render form data while simultaneously
            applying transformations, consider using the [PdfPage::flatten()] function to flatten the
            form elements and form data into the containing page."
    );

    // The internal implementation of the transform() function used by the create_transform_setters!() macro.
    fn transform_impl(
        mut self,
        a: PdfMatrixValue,
        b: PdfMatrixValue,
        c: PdfMatrixValue,
        d: PdfMatrixValue,
        e: PdfMatrixValue,
        f: PdfMatrixValue,
    ) -> Result<Self, PdfiumError> {
        let result = self
            .transformation_matrix
            .multiply(PdfMatrix::new(a, b, c, d, e, f));

        if result.determinant() == 0.0 {
            Err(PdfiumError::InvalidTransformationMatrix)
        } else {
            self.transformation_matrix = result;
            self.do_render_form_data = false;

            Ok(self)
        }
    }

    // The internal implementation of the reset_matrix() function used by the create_transform_setters!() macro.
    fn reset_matrix_impl(mut self, matrix: PdfMatrix) -> Result<Self, PdfiumError> {
        self.transformation_matrix = matrix;

        Ok(self)
    }

    /// Clips rendering output to the given pixel coordinates. Pdfium will not render outside
    /// the clipping area; any existing image data in the destination [PdfBitmap] will remain
    /// intact.
    ///
    /// Pdfium's rendering pipeline supports _either_ rendering with form data _or_ clipping rendering
    /// output, but not both at the same time. Applying a clipping rectangle automatically disables
    /// rendering of form data. If you must render form data while simultaneously applying a
    /// clipping rectangle, consider using the [PdfPage::flatten()] function to flatten the
    /// form elements and form data into the containing page.
    #[inline]
    pub fn clip(mut self, left: Pixels, top: Pixels, right: Pixels, bottom: Pixels) -> Self {
        self.clip_rect = Some((left, top, right, bottom));
        self.do_render_form_data = false;

        self
    }

    /// Controls whether the matrix render path composes the page's intrinsic
    /// `/Rotate` with any matrix supplied via [`Self::transform()`] /
    /// [`Self::reset_matrix()`] before passing the result to
    /// `FPDF_RenderPageBitmapWithMatrix`.
    ///
    /// The form-data render path (`FPDF_RenderPageBitmap`) always honours
    /// the page's `/Rotate` automatically. The matrix render path does
    /// not — every caller using a custom transformation on a page with a
    /// non-zero `/Rotate` has to hand-derive a per-`/Rotate` matrix that
    /// bakes the rotation into the transform itself. Setting this flag
    /// to `true` lets callers write their transform in **display-oriented**
    /// page coordinates (the same coordinate system the form-data path
    /// speaks) and have the rotation applied for them.
    ///
    /// Default is `false` to preserve existing matrix-path semantics —
    /// every existing user supplies a matrix in pre-rotation page
    /// coordinates, and changing that under their feet would silently
    /// flip output for every rotated PDF in the wild.
    ///
    /// Has no effect when the form-data path is in use.
    #[inline]
    pub fn set_auto_apply_intrinsic_rotation(mut self, yes: bool) -> Self {
        self.auto_apply_intrinsic_rotation = yes;

        self
    }

    /// Computes the pixel dimensions and rotation settings for the given [PdfPage]
    /// based on the configuration of this [PdfRenderConfig].
    #[inline]
    pub(crate) fn apply_to_page(&self, page: &PdfPage) -> PdfPageRenderSettings {
        let source_width = page.width();

        let source_height = page.height();

        let source_orientation =
            PdfPageOrientation::from_width_and_height(source_width, source_height);

        // Do we need to apply any rotation?

        let (target_rotation, do_rotate_constraints) = if source_orientation == Portrait
            && self.portrait_rotation != PdfPageRenderRotation::None
        {
            (
                self.portrait_rotation,
                self.portrait_rotation_do_rotate_constraints,
            )
        } else if source_orientation == Landscape
            && self.landscape_rotation != PdfPageRenderRotation::None
        {
            (
                self.landscape_rotation,
                self.landscape_rotation_do_rotate_constraints,
            )
        } else {
            (PdfPageRenderRotation::None, false)
        };

        let (output_width, output_height, width_scale, height_scale) = if self.use_auto_scaling {
            // Compute output width and height based on target sizes and page dimensions.

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

            (
                (source_width.value * width_scale).round() as c_int,
                (source_height.value * height_scale).round() as c_int,
                width_scale,
                height_scale,
            )
        } else {
            // Take output width and height directly from user's fixed settings.

            (
                self.fixed_width.unwrap_or(0) as c_int,
                self.fixed_height.unwrap_or(0) as c_int,
                self.scale_width_factor.unwrap_or(1.0),
                self.scale_height_factor.unwrap_or(1.0),
            )
        };

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

        // Pages can be rendered either _with_ transformation matrices and clipping
        // but _without_ form data, or _with_ form data but _without_ transformation matrices
        // and clipping. We need to be prepared for either option. If rendering of form data
        // is disabled, then the scaled output width and height and any user-specified
        // 90-degree rotation need to be applied to the transformation matrix now.

        let transformation_matrix = if !self.do_render_form_data {
            // If `auto_apply_intrinsic_rotation` is set, pre-compose the page's
            // `/Rotate` mapping with the user-supplied matrix. The user's matrix
            // is then interpreted as operating on display-oriented coordinates
            // — the same coordinate system the form-data path speaks — rather
            // than pre-rotation page coordinates that the matrix render path
            // would otherwise expect.
            let starting_matrix = if self.auto_apply_intrinsic_rotation {
                let intrinsic = page.rotation().unwrap_or(PdfPageRenderRotation::None);

                if intrinsic != PdfPageRenderRotation::None {
                    // `source_width` / `source_height` come from `page.width()` /
                    // `page.height()` which return display-oriented dims (already
                    // swapped for `/Rotate 90/270`). Recover the pre-rotation dims
                    // for the `R` matrix.
                    let (pre_w, pre_h) = match intrinsic {
                        PdfPageRenderRotation::Degrees90 | PdfPageRenderRotation::Degrees270 => {
                            (source_height.value, source_width.value)
                        }
                        _ => (source_width.value, source_height.value),
                    };

                    intrinsic_rotation_matrix(intrinsic, pre_w, pre_h)
                        .multiply(self.transformation_matrix)
                } else {
                    self.transformation_matrix
                }
            } else {
                self.transformation_matrix
            };

            let result = if target_rotation != PdfPageRenderRotation::None {
                // Translate the origin to the center of the page before rotating.

                let (delta_x, delta_y) = match target_rotation {
                    PdfPageRenderRotation::None => unreachable!(),
                    PdfPageRenderRotation::Degrees90 => (PdfPoints::ZERO, -source_width),
                    PdfPageRenderRotation::Degrees180 => (-source_width, -source_height),
                    PdfPageRenderRotation::Degrees270 => (-source_height, PdfPoints::ZERO),
                };

                starting_matrix
                    .translate(delta_x, delta_y)
                    .and_then(|result| {
                        result.rotate_clockwise_degrees(target_rotation.as_degrees())
                    })
            } else {
                Ok(starting_matrix)
            };

            result.and_then(|result| result.scale(width_scale, height_scale))
        } else {
            Ok(PdfMatrix::identity())
        };

        PdfPageRenderSettings {
            width: output_width,
            height: output_height,
            format: self.format.as_pdfium() as c_int,
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
                                form_field_type.as_pdfium() as c_int,
                                color.as_pdfium_color_with_alpha(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            },
            matrix: transformation_matrix
                .unwrap_or(PdfMatrix::IDENTITY)
                .as_pdfium(),
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
            render_flags: render_flags as c_int,
            is_reversed_byte_order_flag_set: self.do_set_flag_reverse_byte_order,
        }
    }
}

impl Default for PdfRenderConfig {
    #[inline]
    fn default() -> Self {
        PdfRenderConfig::new()
    }
}

/// Returns the PDF transformation matrix mapping pre-rotation page coordinates
/// (PDF user space, y-up, origin bottom-left) to display-oriented page
/// coordinates (still PDF user space, y-up, but with the page rotated to
/// match its `/Rotate` value). `pre_w` / `pre_h` are pre-rotation page
/// dimensions in PDF points.
///
/// Composing this matrix with a display-oriented user matrix via
/// [`PdfMatrix::multiply`] produces the full pre-rotation → bitmap matrix
/// that `FPDF_RenderPageBitmapWithMatrix` expects, so callers don't have to
/// hand-derive a per-`/Rotate` matrix.
fn intrinsic_rotation_matrix(
    rotation: PdfPageRenderRotation,
    pre_w: PdfMatrixValue,
    pre_h: PdfMatrixValue,
) -> PdfMatrix {
    match rotation {
        PdfPageRenderRotation::None => PdfMatrix::IDENTITY,
        // /Rotate 90 cw: pre-rotation (x, y) -> display (y, w_pre - x).
        PdfPageRenderRotation::Degrees90 => PdfMatrix::new(0.0, -1.0, 1.0, 0.0, 0.0, pre_w),
        // /Rotate 180: pre-rotation (x, y) -> display (w_pre - x, h_pre - y).
        PdfPageRenderRotation::Degrees180 => PdfMatrix::new(-1.0, 0.0, 0.0, -1.0, pre_w, pre_h),
        // /Rotate 270 cw (== 90 ccw): pre-rotation (x, y) -> display (h_pre - y, x).
        PdfPageRenderRotation::Degrees270 => PdfMatrix::new(0.0, 1.0, -1.0, 0.0, pre_h, 0.0),
    }
}

/// Finalized rendering settings, ready to be passed to a Pdfium rendering function.
/// Generated by calling [PdfRenderConfig::apply_to_page()].
#[derive(Debug, Clone)]
pub(crate) struct PdfPageRenderSettings {
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
    pub(crate) is_reversed_byte_order_flag_set: bool,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium; // Temporary until PdfParagraph is included in the prelude.

    #[test]
    fn test_fixed_size_render_config() -> Result<(), PdfiumError> {
        let render_settings =
            get_render_settings_from_config(PdfRenderConfig::new().set_fixed_size(2000, 2000))?;

        assert_eq!(render_settings.width, 2000);
        assert_eq!(render_settings.height, 2000);

        // Applying scaling does not affect the rendered bitmap size.

        let render_settings = get_render_settings_from_config(
            PdfRenderConfig::new()
                .set_fixed_size(2000, 2000)
                .scale_page_by_factor(5.0),
        )?;

        assert_eq!(render_settings.width, 2000);
        assert_eq!(render_settings.height, 2000);

        Ok(())
    }

    #[test]
    fn test_target_size_render_config() -> Result<(), PdfiumError> {
        let render_settings = get_render_settings_from_config(
            PdfRenderConfig::new().scale_page_to_display_size(2000, 2000),
        )?;

        assert_eq!(render_settings.width, 1414);
        assert_eq!(render_settings.height, 2000);

        // Applying scaling does affected the rendered bitmap size.

        let render_settings = get_render_settings_from_config(
            PdfRenderConfig::new()
                .set_target_size(2000, 2000)
                .scale_page_by_factor(5.0),
        )?;

        assert_eq!(render_settings.width, 2976);
        assert_eq!(render_settings.height, 4209);

        Ok(())
    }

    fn get_render_settings_from_config(
        config: PdfRenderConfig,
    ) -> Result<PdfPageRenderSettings, PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        let mut document = pdfium.create_new_pdf()?;
        let page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::Portrait(PdfPagePaperStandardSize::A4))?;

        Ok(config.apply_to_page(&page))
    }

    // ---------------------------------------------------------------------
    // auto_apply_intrinsic_rotation tests.
    //
    // The flag is a one-bool switch on the matrix render path that pre-
    // composes the page's intrinsic `/Rotate` with any caller-supplied
    // matrix, so callers don't have to hand-derive a per-`/Rotate` matrix.
    // The tests below pin:
    //
    //   1. Default-off: emits the identical matrix as before.
    //   2. On a /Rotate 0 page: composing identity changes nothing.
    //   3. On a /Rotate 90/180/270 page: produces the matrix that maps
    //      pre-rotation page coords → display-oriented coords, composed
    //      with the user matrix in the right order.
    //
    // Tests (1)–(3) live below. Render-level integration coverage
    // (matrix-path-with-auto-rotate ≡ form-data-path baseline) is left to
    // downstream consumers like libviprs's `pdfium_streaming_rotation_matrix`
    // cross-product test, which already pins it for the four /Rotate values
    // against pdfium's own form-data rasterizer.
    // ---------------------------------------------------------------------

    /// Builds a matrix-path config (form data disabled) so apply_to_page
    /// runs through the auto-rotate branch.
    fn matrix_path_config_with(matrix: PdfMatrix) -> PdfRenderConfig {
        PdfRenderConfig::new()
            .render_form_data(false)
            .reset_matrix(matrix)
            .unwrap()
    }

    #[test]
    fn test_auto_apply_intrinsic_rotation_default_false() -> Result<(), PdfiumError> {
        // The default must be off. Flipping it under existing callers'
        // feet would silently change the bytes their matrix-path renders
        // produce on every rotated PDF in the wild.
        assert!(!PdfRenderConfig::new().auto_apply_intrinsic_rotation);
        Ok(())
    }

    #[test]
    fn test_auto_apply_intrinsic_rotation_on_zero_rotate_page_is_identity(
    ) -> Result<(), PdfiumError> {
        // Composing identity must change nothing. Same input, same output.
        let pdfium = test_bind_to_pdfium();
        let mut document = pdfium.create_new_pdf()?;
        let page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::Portrait(PdfPagePaperStandardSize::A4))?;
        assert_eq!(page.rotation()?, PdfPageRenderRotation::None);

        let user = PdfMatrix::IDENTITY.scale(2.0, 3.0)?;
        let m_off = matrix_path_config_with(user).apply_to_page(&page).matrix;
        let m_on = matrix_path_config_with(user)
            .set_auto_apply_intrinsic_rotation(true)
            .apply_to_page(&page)
            .matrix;
        assert_eq!(
            (m_off.a, m_off.b, m_off.c, m_off.d, m_off.e, m_off.f),
            (m_on.a, m_on.b, m_on.c, m_on.d, m_on.e, m_on.f)
        );
        Ok(())
    }

    #[test]
    fn test_auto_apply_intrinsic_rotation_composes_for_each_rotate_value() -> Result<(), PdfiumError>
    {
        // For each /Rotate ∈ {90, 180, 270}, set the page's rotation
        // and assert apply_to_page emits the expected pre-composed
        // matrix. This is the bug-pinning anchor for the rotation
        // matrix derivation.
        //
        // The user matrix is identity so apply_to_page emits exactly
        // `R` (the intrinsic rotation matrix) — easy to read off.
        let pdfium = test_bind_to_pdfium();
        let mut document = pdfium.create_new_pdf()?;
        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::Portrait(PdfPagePaperStandardSize::A4))?;

        // Pre-rotation A4 dims in pt.
        let pre_w = page.width().value;
        let pre_h = page.height().value;

        let cases = [
            // (rotation, expected R = [a, b, c, d, e, f])
            (
                PdfPageRenderRotation::Degrees90,
                [0.0, -1.0, 1.0, 0.0, 0.0, pre_w],
            ),
            (
                PdfPageRenderRotation::Degrees180,
                [-1.0, 0.0, 0.0, -1.0, pre_w, pre_h],
            ),
            (
                PdfPageRenderRotation::Degrees270,
                [0.0, 1.0, -1.0, 0.0, pre_h, 0.0],
            ),
        ];

        for (rotation, expected) in cases {
            page.set_rotation(rotation);
            assert_eq!(page.rotation()?, rotation);

            let m = matrix_path_config_with(PdfMatrix::IDENTITY)
                .set_auto_apply_intrinsic_rotation(true)
                .apply_to_page(&page)
                .matrix;

            let actual = [m.a, m.b, m.c, m.d, m.e, m.f];
            for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
                assert!(
                    (a - e).abs() < 1e-3,
                    "/Rotate {:?}: matrix component {} diverged: {} vs expected {}",
                    rotation,
                    i,
                    a,
                    e,
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_auto_apply_intrinsic_rotation_composes_user_matrix_after_rotation(
    ) -> Result<(), PdfiumError> {
        // User matrix ∘ R ordering: R applied first (pre-rotation -> display),
        // user matrix applied second (display -> bitmap). For /Rotate 90 the
        // libviprs strip-matrix derivation gives, with user = display->pixel
        // [s, 0, 0, -s, 0, s·display_h_pt - y_off]:
        //   composed = [0, s, s, 0, 0, -y_off]
        // We pin one such composition here.
        let pdfium = test_bind_to_pdfium();
        let mut document = pdfium.create_new_pdf()?;
        let mut page = document
            .pages_mut()
            .create_page_at_start(PdfPagePaperSize::Portrait(PdfPagePaperStandardSize::A4))?;
        page.set_rotation(PdfPageRenderRotation::Degrees90);

        // Pre-rotation height is the configured PdfPage height (we set
        // /Rotate 90 *after* page creation, so width()/height() now
        // return display-oriented = swapped dims). Pre-rotation w == display h.
        // Pre-rotation A4: 595 × 842 pt. Display under /Rotate 90: 842 × 595.
        let display_h_pt = page.height().value; // 595
        let s: f32 = 2.0;
        let y_off: f32 = 100.0;

        // user matrix: display-oriented (y-up) -> bitmap pixel (y-down) with strip offset.
        let user = PdfMatrix::new(s, 0.0, 0.0, -s, 0.0, s * display_h_pt - y_off);

        let m = matrix_path_config_with(user)
            .set_auto_apply_intrinsic_rotation(true)
            .apply_to_page(&page)
            .matrix;

        // Expected per libviprs derivation: [0, s, s, 0, 0, -y_off].
        let expected = [0.0, s, s, 0.0, 0.0, -y_off];
        let actual = [m.a, m.b, m.c, m.d, m.e, m.f];
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (a - e).abs() < 1e-3,
                "matrix component {} diverged: {} vs expected {}",
                i,
                a,
                e,
            );
        }
        Ok(())
    }
}
