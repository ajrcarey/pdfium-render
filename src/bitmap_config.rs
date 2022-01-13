use crate::bitmap::PdfBitmapRotation;
use crate::page::PdfPageOrientation::{Landscape, Portrait};
use crate::page::{PdfPage, PdfPageOrientation};

/// Configures the scaling and rotation settings that should be applied to a PdfPage
/// to create a PdfBitmap for that page. PdfBitmapConfig can accommodate pages of
/// different sizes while correctly maintaining each page's aspect ratio,
/// automatically rotate portrait or landscape pages, generate page thumbnails,
/// and apply maximum and minimum pixel sizes to the scaled width and height of the
/// final bitmap.
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

    /// Computes the pixel dimensions and rotation settings for the given PdfPage
    /// based on the configuration of this PdfBitmapConfig.
    #[inline]
    pub(crate) fn apply_to_page(&self, page: &PdfPage) -> (u16, u16, Option<PdfBitmapRotation>) {
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

        (
            (source_width * width_scale) as u16,
            (source_height * height_scale) as u16,
            if target_rotation == PdfBitmapRotation::None {
                None
            } else {
                Some(target_rotation)
            },
        )
    }
}

impl Default for PdfBitmapConfig {
    #[inline]
    fn default() -> Self {
        PdfBitmapConfig::new()
    }
}
