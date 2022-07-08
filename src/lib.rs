#![doc = include_str!("../README.md")]

extern crate core;

mod bindgen {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!("bindgen.rs");
}

pub mod action;
pub mod action_destination;
pub mod bindings;
pub mod bitmap;
pub mod bitmap_config;
pub mod bookmark;
pub mod bookmarks;
pub mod color;
mod color_space;
pub mod document;
pub mod error;
pub mod font;
pub mod form;
pub mod metadata;
pub mod page;
pub mod page_annotation;
pub mod page_annotation_circle;
pub mod page_annotation_free_text;
pub mod page_annotation_highlight;
pub mod page_annotation_ink;
pub mod page_annotation_link;
pub mod page_annotation_popup;
mod page_annotation_private; // Keep private so that the PdfPageAnnotationPrivate trait is not exposed.
pub mod page_annotation_square;
pub mod page_annotation_squiggly;
pub mod page_annotation_stamp;
pub mod page_annotation_strikeout;
pub mod page_annotation_text;
pub mod page_annotation_underline;
pub mod page_annotation_unsupported;
pub mod page_annotations;
pub mod page_boundaries;
pub mod page_object;
pub mod page_object_form_fragment;
pub mod page_object_group;
pub mod page_object_image;
pub mod page_object_path;
mod page_object_private; // Keep private so that the PdfPageObjectPrivate trait is not exposed.
pub mod page_object_shading;
pub mod page_object_text;
pub mod page_object_unsupported;
pub mod page_objects;
pub mod page_size;
pub mod page_text;
pub mod page_text_char;
pub mod page_text_chars;
pub mod page_text_segment;
pub mod page_text_segments;
pub mod pages;
mod paragraph; // Keep private while PdfParagraph is still in development.
pub mod pdfium;
pub mod permissions;
mod utils;

/// A prelude for conveniently importing all public `pdfium-render` definitions at once.
///
/// Usage:
/// ```
/// use pdfium_render::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        action::*, action_destination::*, bindings::*, bitmap::*, bitmap_config::*, bookmark::*,
        bookmarks::*, color::*, color_space::*, document::*, error::*, font::*, form::*,
        metadata::*, page::*, page_annotation::*, page_annotation_circle::*,
        page_annotation_free_text::*, page_annotation_highlight::*, page_annotation_ink::*,
        page_annotation_link::*, page_annotation_popup::*, page_annotation_square::*,
        page_annotation_squiggly::*, page_annotation_stamp::*, page_annotation_strikeout::*,
        page_annotation_text::*, page_annotation_underline::*, page_annotation_unsupported::*,
        page_annotations::*, page_boundaries::*, page_object::*, page_object_form_fragment::*,
        page_object_group::*, page_object_image::*, page_object_path::*, page_object_shading::*,
        page_object_text::*, page_object_unsupported::*, page_objects::*, page_size::*,
        page_text::*, page_text_char::*, page_text_chars::*, page_text_segment::*,
        page_text_segments::*, pages::*, pdfium::*, permissions::*,
    };
}

// Include the appropriate implementation of the PdfiumLibraryBindings trait for the
// target architecture and threading model.

// Conditional compilation is used to compile different implementations of
// the PdfiumLibraryBindings trait depending on whether we are compiling to a WASM module,
// a native shared library, or a statically linked library.

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "static"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "static")]
mod linked;

#[cfg(target_arch = "wasm32")]
mod wasm;

// These implementations are all single-threaded (because Pdfium itself is single-threaded).
// Any of them can be wrapped by thread_safe::ThreadSafePdfiumBindings to
// create a thread-safe architecture-specific implementation of the PdfiumLibraryBindings trait.

#[cfg(feature = "thread_safe")]
mod thread_safe;

#[cfg(test)]
pub mod tests {
    use crate::prelude::*;
    use image::ImageFormat;

    #[test]
    #[cfg(not(feature = "static"))]
    fn test_readme_example() -> Result<(), PdfiumError> {
        // Runs the code in the main example at the top of README.md.

        fn export_pdf_to_jpegs(path: &str, password: Option<&str>) -> Result<(), PdfiumError> {
            // Renders each page in the given test PDF file to a separate JPEG file.

            // Bind to a Pdfium library in the same directory as our application;
            // failing that, fall back to using a Pdfium library provided by the operating system.

            let pdfium = Pdfium::new(
                Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                    .or_else(|_| Pdfium::bind_to_system_library())?,
            );

            // Open the PDF document...

            let document = pdfium.load_pdf_from_file(path, password)?;

            // ... set rendering options that will apply to all pages...

            let bitmap_render_config = PdfBitmapConfig::new()
                .set_target_width(2000)
                .set_maximum_height(2000)
                .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

            // ... then render each page to a bitmap image, saving each image to a JPEG file.

            for (index, page) in document.pages().iter().enumerate() {
                page.get_bitmap_with_config(&bitmap_render_config)?
                    .as_image() // Renders this page to an Image::DynamicImage...
                    .as_rgba8() // ... then converts it to an Image::Image
                    .ok_or(PdfiumError::ImageError)?
                    .save_with_format(format!("test-page-{}.jpg", index), image::ImageFormat::Jpeg)
                    .map_err(|_| PdfiumError::ImageError)?;
            }

            Ok(())
        }

        export_pdf_to_jpegs("./test/export-test.pdf", None)
    }

    #[test]
    #[cfg(not(feature = "static"))]
    fn test_dynamic_bindings() -> Result<(), PdfiumError> {
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );

        let document = pdfium.load_pdf_from_file("./test/form-test.pdf", None)?;

        let render_config = PdfBitmapConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true)
            .render_form_data(true)
            .render_annotations(true);

        for (index, page) in document.pages().iter().enumerate() {
            let result = page
                .get_bitmap_with_config(&render_config)?
                .as_image()
                .as_rgba8()
                .ok_or(PdfiumError::ImageError)?
                .save_with_format(format!("form-test-page-{}.jpg", index), ImageFormat::Jpeg);

            assert!(result.is_ok());
        }

        Ok(())
    }

    #[test]
    #[cfg(feature = "static")]
    fn test_static_bindings() {
        use crate::prelude::*;

        // Simply checks that the static bindings contain no compilation errors.

        Pdfium::bind_to_statically_linked_library().unwrap();
    }
}
