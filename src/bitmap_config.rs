//! Defines the [PdfBitmapConfig] struct, a builder-based approach to configuring
//! the rendering of `PdfBitmap` objects from one or more `PdfPage` objects.

use crate::bindgen::{
    FPDF_ANNOT, FPDF_CONVERT_FILL_TO_STROKE, FPDF_DWORD, FPDF_GRAYSCALE, FPDF_LCD_TEXT,
    FPDF_NO_NATIVETEXT, FPDF_PRINTING, FPDF_RENDER_FORCEHALFTONE, FPDF_RENDER_LIMITEDIMAGECACHE,
    FPDF_RENDER_NO_SMOOTHIMAGE, FPDF_RENDER_NO_SMOOTHPATH, FPDF_RENDER_NO_SMOOTHTEXT,
    FPDF_REVERSE_BYTE_ORDER,
};
use crate::bitmap::{PdfBitmapFormat, PdfBitmapRotation};
use crate::color::PdfColor;
use crate::form::PdfFormFieldType;
use crate::page::PdfPageOrientation::{Landscape, Portrait};
use crate::page::{PdfPage, PdfPageOrientation};

/// Configures the scaling, rotation, and rendering settings that should be applied to
/// a PdfPage to create a PdfBitmap for that page. PdfBitmapConfig can accommodate pages of
/// different sizes while correctly maintaining each page's aspect ratio, automatically
/// rotate portrait or landscape pages, generate page thumbnails, and apply maximum and
/// minimum pixel sizes to the scaled width and height of the final bitmap.
pub struct PdfBitmapConfig {
    target_width: Option<u16>,
    target_height: Option<u16>,
    scale_width_factor: Option<f32>,
    scale_height_factor: Option<f32>,
    maximum_width: Option<u16>,
    maximum_height: Option<u16>,
    portrait_rotation: PdfBitmapRotation,
    portrait_rotation_do_rotate_constraints: bool,
    landscape_rotation: PdfBitmapRotation,
    landscape_rotation_do_rotate_constraints: bool,
    format: PdfBitmapFormat,
    do_render_form_data: bool,
    form_field_highlight: Vec<(PdfFormFieldType, PdfColor)>,

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

impl PdfBitmapConfig {
    /// Creates a new PdfBitmapConfig with all settings unconfigured.
    pub fn new() -> Self {
        PdfBitmapConfig {
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
            format: PdfBitmapFormat::BGRA,
            do_render_form_data: true,
            form_field_highlight: vec![],
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
            do_set_flag_reverse_byte_order: false,
            do_set_flag_convert_fill_to_stroke: false,
        }
    }

    /// Converts the width and height of a PdfPage from points to pixels, scaling each
    /// dimension to the given target pixel sizes. The aspect ratio of the source page
    /// will not be maintained.
    #[inline]
    pub fn set_target_size(self, width: u16, height: u16) -> Self {
        let result = self.set_target_width(width);

        result.set_target_height(height)
    }

    /// Converts the width of a PdfPage from points to pixels, scaling the source page
    /// width to the given target pixel width. The aspect ratio of the source page
    /// will be maintained so long as there is no call to [PdfBitmapConfig::set_target_size()]
    /// or [PdfBitmapConfig::set_target_height()] that overrides it.
    #[inline]
    pub fn set_target_width(mut self, width: u16) -> Self {
        self.target_width = Some(width);

        self
    }

    /// Converts the height of a PdfPage from points to pixels, scaling the source page
    /// height to the given target pixel height. The aspect ratio of the source page
    /// will be maintained so long as there is no call to [PdfBitmapConfig::set_target_size()]
    /// or [PdfBitmapConfig::set_target_width()] that overrides it.
    #[inline]
    pub fn set_target_height(mut self, height: u16) -> Self {
        self.target_height = Some(height);

        self
    }

    /// Applies settings to this PdfBitmapConfig suitable for filling the given
    /// screen display size.
    ///
    /// The source page's dimensions will be scaled so that both width and height attempt
    /// to fill, but do not exceed, the given pixel dimensions. The aspect ratio of the
    /// source page will be maintained. Landscape pages will be automatically rotated
    /// by 90 degrees and will be scaled down if necessary to fit the display width.
    #[inline]
    pub fn scale_page_to_display_size(mut self, width: u16, height: u16) -> Self {
        self.scale_width_factor = None;
        self.scale_height_factor = None;

        let result = self.set_target_width(width);

        let result = result.set_maximum_width(width);

        let result = result.set_maximum_height(height);

        result.rotate_if_landscape(PdfBitmapRotation::Degrees90, true)
    }

    /// Converts the width and height of a PdfPage from points to pixels by applying
    /// the given scale factor to both dimensions. The aspect ratio of the source page
    /// will be maintained. Overrides any previous call to [PdfBitmapConfig::scale_page_by_factor()],
    /// [PdfBitmapConfig::scale_page_width_by_factor()], or [PdfBitmapConfig::scale_page_height_by_factor()].
    #[inline]
    pub fn scale_page_by_factor(self, scale: f32) -> Self {
        let result = self.scale_page_width_by_factor(scale);

        result.scale_page_height_by_factor(scale)
    }

    /// Converts the width of the PdfPage from points to pixels by applying the given
    /// scale factor. The aspect ratio of the source page will not be maintained if a
    /// different scale factor is applied to the height. Overrides any previous call to
    /// [PdfBitmapConfig::scale_page_by_factor()], [PdfBitmapConfig::scale_page_width_by_factor()],
    /// or [PdfBitmapConfig::scale_page_height_by_factor()].
    #[inline]
    pub fn scale_page_width_by_factor(mut self, scale: f32) -> Self {
        self.scale_width_factor = Some(scale);

        self
    }

    /// Converts the height of the PdfPage from points to pixels by applying the given
    /// scale factor. The aspect ratio of the source page will not be maintained if a
    /// different scale factor is applied to the width. Overrides any previous call to
    /// [PdfBitmapConfig::scale_page_by_factor()], [PdfBitmapConfig::scale_page_width_by_factor()],
    /// or [PdfBitmapConfig::scale_page_height_by_factor()].
    #[inline]
    pub fn scale_page_height_by_factor(mut self, scale: f32) -> Self {
        self.scale_height_factor = Some(scale);

        self
    }

    /// Specifies that the final pixel width of the PdfPage will not exceed the given maximum.
    #[inline]
    pub fn set_maximum_width(mut self, width: u16) -> Self {
        self.maximum_width = Some(width);

        self
    }

    /// Specifies that the final pixel height of the PdfPage will not exceed the given maximum.
    #[inline]
    pub fn set_maximum_height(mut self, height: u16) -> Self {
        self.maximum_height = Some(height);

        self
    }

    /// Applies the given rotation setting to the PdfPage during rendering, irrespective
    /// of its orientation. If the given flag is set to [true], then any maximum
    /// constraint on the final pixel width set by a call to [PdfBitmapConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height, and any
    /// maximum constraint on the final pixel height set by a call to [PdfBitmapConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate(self, rotation: PdfBitmapRotation, do_rotate_constraints: bool) -> Self {
        let result = self.rotate_if_portait(rotation, do_rotate_constraints);

        result.rotate_if_landscape(rotation, do_rotate_constraints)
    }

    /// Applies the given rotation settings to the PdfPage during rendering, if the page
    /// is in portrait orientation. If the given flag is set to [true] and the given
    /// rotation setting is [PdfBitmapRotation::Degrees90] or [PdfBitmapRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfBitmapConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfBitmapConfig::set_maximum_height()]
    /// will be rotated so it becomes a constraint on the final pixel width.
    #[inline]
    pub fn rotate_if_portait(
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

    /// Applies the given rotation settings to the PdfPage during rendering, if the page
    /// is in landscape orientation. If the given flag is set to [true] and the given
    /// rotation setting is [PdfBitmapRotation::Degrees90] or [PdfBitmapRotation::Degrees270]
    /// then any maximum constraint on the final pixel width set by a call to [PdfBitmapConfig::set_maximum_width()]
    /// will be rotated so it becomes a constraint on the final pixel height and any
    /// maximum constraint on the final pixel height set by a call to [PdfBitmapConfig::set_maximum_height()]
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

    /// Sets the pixel format that will be used during rendering of the PdfPage.
    /// The default is [PdfBitmapFormat::BGRA].
    #[inline]
    pub fn set_format(mut self, format: PdfBitmapFormat) -> Self {
        self.format = format;

        self
    }

    /// Controls whether form data widgets and user-supplied form data should be included
    /// during rendering of the PdfPage. The default is true. The setting has no effect
    /// if the PdfDocument containing the PdfPage does not include an embedded PdfForm.
    #[inline]
    pub fn render_form_data(mut self, do_render: bool) -> Self {
        self.do_render_form_data = do_render;

        self
    }

    /// Controls whether user-supplied annotations should be included during rendering of
    /// the PdfPage. The default is true.
    #[inline]
    pub fn render_annotations(mut self, do_render: bool) -> Self {
        self.do_set_flag_render_annotations = do_render;

        self
    }

    /// Controls whether text rendering should be optimized for LCD display.
    /// The default is false.
    /// Has no effect if anti-aliasing of text has been disabled by a call to
    /// [PdfBitmapConfig::set_text_smoothing(false)].
    #[inline]
    pub fn use_lcd_text_rendering(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_use_lcd_text_rendering = do_set_flag;

        self
    }

    /// Controls whether platform text rendering should be disabled on platforms that support it.
    /// The alternative is for Pdfium to render all text internally, which may give more
    /// consistent rendering results across platforms but may also be slower.
    /// The default is false.
    #[inline]
    pub fn disable_native_text_rendering(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_no_native_text = do_set_flag;

        self
    }

    /// Controls whether rendering output should be grayscale rather than full color.
    /// The default is false.
    #[inline]
    pub fn use_grayscale_rendering(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_grayscale = do_set_flag;

        self
    }

    /// Controls whether Pdfium should limit its image cache size during rendering.
    /// A smaller cache size may result in lower memory usage at the cost of slower rendering.
    /// The default is false.
    #[inline]
    pub fn limit_render_image_cache_size(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_limited_image_cache = do_set_flag;

        self
    }

    /// Controls whether Pdfium should always use halftone for image stretching.
    /// Halftone image stretching is often higher quality than linear image stretching
    /// but is much slower. The default is false.
    #[inline]
    pub fn force_half_tone(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_force_half_tone = do_set_flag;

        self
    }

    /// Controls whether Pdfium should render for printing. The default is false.
    #[inline]
    pub fn use_print_quality(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_for_printing = do_set_flag;

        self
    }

    /// Controls whether rendered text should be anti-aliased.
    /// The default is true.
    /// The enabling of LCD-optimized text rendering via a call to
    /// [PdfiumBitmapConfig::use_lcd_text_rendering(true)] has no effect if this flag
    /// is set to true.
    #[inline]
    pub fn set_text_smoothing(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_no_smooth_text = !do_set_flag;

        self
    }

    /// Controls whether rendered images should be anti-aliased.
    /// The default is true.
    #[inline]
    pub fn set_image_smoothing(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_no_smooth_image = !do_set_flag;

        self
    }

    /// Controls whether rendered vector paths should be anti-aliased.
    /// The default is true.
    #[inline]
    pub fn set_path_smoothing(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_render_no_smooth_path = !do_set_flag;

        self
    }

    /// Controls whether the byte order of generated image data should be reversed
    /// during rendering. The default is false. There should generally be no need
    /// to set this flag, unless you want to do raw image processing and specifically
    /// need the byte data returned by [crate::bitmap::PdfBitmap::as_bytes()] to be reversed.
    #[inline]
    pub fn set_reverse_byte_order(mut self, do_set_flag: bool) -> Self {
        self.do_set_flag_reverse_byte_order = do_set_flag;

        self
    }

    /// Controls whether rendered vector fill paths need to be stroked.
    /// The default is false.
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
        self.form_field_highlight.push((form_field_type, color));

        self
    }

    /// Computes the pixel dimensions and rotation settings for the given PdfPage
    /// based on the configuration of this PdfBitmapConfig.
    #[inline]
    pub(crate) fn apply_to_page(&self, page: &PdfPage) -> PdfBitmapRenderSettings {
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
                .map(|target| (target as f32) / source_width)
        };

        let height_scale = if let Some(scale) = self.scale_height_factor {
            Some(scale)
        } else {
            self.target_height
                .map(|target| (target as f32) / source_height)
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

            if source_width * width_scale > maximum {
                // Constrain the width, so it does not exceed the maximum.

                width_scale = maximum / source_width;

                if do_maintain_aspect_ratio {
                    height_scale = width_scale;
                }
            }
        }

        if let Some(maximum) = height_constraint {
            let maximum = maximum as f32;

            if source_height * height_scale > maximum {
                // Constrain the height, so it does not exceed the maximum.

                height_scale = maximum / source_height;

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

        PdfBitmapRenderSettings {
            width: (source_width * width_scale) as i32,
            height: (source_height * height_scale) as i32,
            format: self.format.as_pdfium() as i32,
            rotate: target_rotation.as_pdfium(),
            do_render_form_data: self.do_render_form_data,
            form_field_highlight: self
                .form_field_highlight
                .iter()
                .map(|(form_field_type, color)| {
                    (
                        form_field_type.as_pdfium() as i32,
                        color.as_pdfium_color_with_alpha(),
                    )
                })
                .collect::<Vec<_>>(),
            render_flags: render_flags as i32,
        }
    }
}

impl Default for PdfBitmapConfig {
    #[inline]
    fn default() -> Self {
        PdfBitmapConfig::new()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PdfBitmapRenderSettings {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) format: i32,
    pub(crate) rotate: i32,
    pub(crate) do_render_form_data: bool,
    pub(crate) form_field_highlight: Vec<(i32, (FPDF_DWORD, u8))>,
    pub(crate) render_flags: i32,
}
