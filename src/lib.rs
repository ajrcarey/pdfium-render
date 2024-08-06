#![doc = include_str!("../README.md")]

mod bindgen {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    // Select the Pdfium FPDF_* API version to use based on crate feature flags.

    #[cfg(feature = "pdfium_future")]
    include!("bindgen/pdfium_future.rs");

    #[cfg(feature = "pdfium_6611")]
    include!("bindgen/pdfium_6611.rs");

    #[cfg(feature = "pdfium_6569")]
    include!("bindgen/pdfium_6569.rs");

    #[cfg(feature = "pdfium_6555")]
    include!("bindgen/pdfium_6555.rs");

    #[cfg(feature = "pdfium_6490")]
    include!("bindgen/pdfium_6490.rs");

    #[cfg(feature = "pdfium_6406")]
    include!("bindgen/pdfium_6406.rs");

    #[cfg(feature = "pdfium_6337")]
    include!("bindgen/pdfium_6337.rs");

    #[cfg(feature = "pdfium_6295")]
    include!("bindgen/pdfium_6295.rs");

    #[cfg(feature = "pdfium_6259")]
    include!("bindgen/pdfium_6259.rs");

    #[cfg(feature = "pdfium_6164")]
    include!("bindgen/pdfium_6164.rs");

    #[cfg(feature = "pdfium_6124")]
    include!("bindgen/pdfium_6124.rs");

    #[cfg(feature = "pdfium_6110")]
    include!("bindgen/pdfium_6110.rs");

    #[cfg(feature = "pdfium_6084")]
    include!("bindgen/pdfium_6084.rs");

    #[cfg(feature = "pdfium_6043")]
    include!("bindgen/pdfium_6043.rs");

    #[cfg(feature = "pdfium_6015")]
    include!("bindgen/pdfium_6015.rs");

    #[cfg(feature = "pdfium_5961")]
    include!("bindgen/pdfium_5961.rs");

    pub type size_t = usize;
}

mod bindings;
mod error;
mod page_index_cache;
mod pdf;
mod pdfium;
mod utils;

/// A prelude for conveniently importing all public `pdfium-render` definitions at once.
///
/// Usage:
/// ```
/// use pdfium_render::prelude::*;
/// ```
pub mod prelude {
    #[allow(deprecated)]
    // TODO: AJRC - 5-Aug-24 - deprecated items will be removed in release 0.9.0. Tracking issue:
    // https://github.com/ajrcarey/pdfium-render/issues/36
    pub use crate::{
        bindings::*,
        error::*,
        pdf::action::*,
        pdf::appearance_mode::*,
        pdf::bitmap::*,
        pdf::color::*,
        pdf::color_space::*,
        pdf::destination::*,
        pdf::document::attachment::*,
        pdf::document::attachments::*,
        pdf::document::bookmark::*,
        pdf::document::bookmarks::*,
        pdf::document::fonts::*,
        pdf::document::form::*,
        pdf::document::metadata::*,
        pdf::document::page::annotation::attachment_points::*,
        pdf::document::page::annotation::circle::*,
        pdf::document::page::annotation::free_text::*,
        pdf::document::page::annotation::highlight::*,
        pdf::document::page::annotation::ink::*,
        pdf::document::page::annotation::link::*,
        pdf::document::page::annotation::objects::*,
        pdf::document::page::annotation::popup::*,
        pdf::document::page::annotation::redacted::*,
        pdf::document::page::annotation::square::*,
        pdf::document::page::annotation::squiggly::*,
        pdf::document::page::annotation::stamp::*,
        pdf::document::page::annotation::strikeout::*,
        pdf::document::page::annotation::text::*,
        pdf::document::page::annotation::underline::*,
        pdf::document::page::annotation::unsupported::*,
        pdf::document::page::annotation::widget::*,
        pdf::document::page::annotation::xfa_widget::*,
        pdf::document::page::annotation::{
            PdfPageAnnotation, PdfPageAnnotationCommon, PdfPageAnnotationType,
        },
        pdf::document::page::annotations::*,
        pdf::document::page::boundaries::*,
        pdf::document::page::field::button::*,
        pdf::document::page::field::checkbox::*,
        pdf::document::page::field::combo::*,
        pdf::document::page::field::list::*,
        pdf::document::page::field::option::*,
        pdf::document::page::field::options::*,
        pdf::document::page::field::radio::*,
        pdf::document::page::field::signature::*,
        pdf::document::page::field::text::*,
        pdf::document::page::field::unknown::*,
        pdf::document::page::field::{PdfFormField, PdfFormFieldCommon, PdfFormFieldType},
        pdf::document::page::links::*,
        pdf::document::page::object::group::*,
        pdf::document::page::object::image::*,
        pdf::document::page::object::path::*,
        pdf::document::page::object::shading::*,
        pdf::document::page::object::text::*,
        pdf::document::page::object::unsupported::*,
        pdf::document::page::object::x_object_form::*,
        pdf::document::page::object::{
            PdfPageObject, PdfPageObjectBlendMode, PdfPageObjectCommon, PdfPageObjectLineCap,
            PdfPageObjectLineJoin, PdfPageObjectType,
        },
        pdf::document::page::objects::common::*,
        pdf::document::page::objects::*,
        pdf::document::page::render_config::*,
        pdf::document::page::size::*,
        pdf::document::page::text::char::*,
        pdf::document::page::text::chars::*,
        pdf::document::page::text::search::*,
        pdf::document::page::text::segment::*,
        pdf::document::page::text::segments::*,
        pdf::document::page::text::*,
        pdf::document::page::{
            PdfBitmapRotation, PdfPage, PdfPageContentRegenerationStrategy, PdfPageOrientation,
            PdfPageRenderRotation,
        },
        pdf::document::pages::*,
        pdf::document::permissions::*,
        pdf::document::signature::*,
        pdf::document::signatures::*,
        pdf::document::{PdfDocument, PdfDocumentVersion},
        pdf::font::glyph::*,
        pdf::font::glyphs::*,
        pdf::font::*,
        pdf::link::*,
        pdf::matrix::*,
        pdf::path::segment::*,
        pdf::path::segments::*,
        pdf::points::*,
        pdf::quad_points::*,
        pdf::rect::*,
        pdfium::*,
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;
    use image::ImageFormat;
    use std::fs::File;
    use std::path::Path;

    #[test]
    #[cfg(not(feature = "static"))]
    fn test_readme_example() -> Result<(), PdfiumError> {
        // Runs the code in the main example at the top of README.md.

        fn export_pdf_to_jpegs(
            path: &impl AsRef<Path>,
            password: Option<&str>,
        ) -> Result<(), PdfiumError> {
            // Renders each page in the given test PDF file to a separate JPEG file.

            // Bind to a Pdfium library in the same directory as our Rust executable.
            // See the "Dynamic linking" section below.

            let pdfium = Pdfium::default();

            // Open the PDF document...

            let document = pdfium.load_pdf_from_file(path, password)?;

            // ... set rendering options that will apply to all pages...

            let render_config = PdfRenderConfig::new()
                .set_target_width(2000)
                .set_maximum_height(2000)
                .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

            // ... then render each page to a bitmap image, saving each image to a JPEG file.

            for (index, page) in document.pages().iter().enumerate() {
                page.render_with_config(&render_config)?
                    .as_image() // Renders this page to an Image::DynamicImage...
                    .into_rgb8() // ... then converts it to an Image::Image ...
                    .save_with_format(format!("test-page-{}.jpg", index), image::ImageFormat::Jpeg) // ... and saves it to a file.
                    .map_err(|_| PdfiumError::ImageError)?;
            }

            Ok(())
        }

        export_pdf_to_jpegs(&"./test/export-test.pdf", None)
    }

    #[test]
    #[cfg(not(feature = "static"))]
    fn test_dynamic_bindings() -> Result<(), PdfiumError> {
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        let document = pdfium.load_pdf_from_file("./test/form-test.pdf", None)?;

        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)
            .render_form_data(true)
            .render_annotations(true);

        for (index, page) in document.pages().iter().enumerate() {
            let result = page
                .render_with_config(&render_config)?
                .as_image()
                .into_rgb8()
                .save_with_format(format!("form-test-page-{}.jpg", index), ImageFormat::Jpeg);

            assert!(result.is_ok());
        }

        Ok(())
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_static_bindings() {
        // Simply checks that the static bindings contain no compilation errors.

        Pdfium::bind_to_statically_linked_library().unwrap();
    }

    #[test]
    fn test_reader_lifetime() -> Result<(), PdfiumError> {
        // Confirms that a reader given to Pdfium::load_pdf_from_reader() does not need
        // a lifetime longer than that of the PdfDocument it is used to create.

        let pdfium = test_bind_to_pdfium();

        let paths = ["test/form-test.pdf", "test/annotations-test.pdf"];

        for path in paths {
            let page_count = {
                let reader = File::open(path).map_err(PdfiumError::IoError)?;

                let document = pdfium.load_pdf_from_reader(reader, None)?;

                document.pages().len()

                // reader will be dropped here, immediately after document.
            };

            println!("{} has {} pages", path, page_count);
        }

        Ok(())
    }
}
