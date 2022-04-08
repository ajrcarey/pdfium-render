#![doc = include_str!("../README.md")]

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
pub mod page_object_image;
pub mod page_object_path;
pub mod page_object_shading;
pub mod page_object_text;
pub mod page_object_unsupported;
pub mod page_objects;
pub mod page_size;
pub mod page_text;
pub mod pages;
pub mod pdfium;
mod utils;

/// A prelude for conveniently importing all public `pdfium-render` functionality at once.
///
/// Usage: `use pdfium_render::prelude::*`;
pub mod prelude {
    pub use super::{
        action::*, action_destination::*, bindings::*, bitmap::*, bitmap_config::*, bookmark::*,
        bookmarks::*, color::*, document::*, error::*, font::*, form::*, metadata::*, page::*,
        page_annotation::*, page_annotation_circle::*, page_annotation_free_text::*,
        page_annotation_highlight::*, page_annotation_ink::*, page_annotation_link::*,
        page_annotation_popup::*, page_annotation_square::*, page_annotation_squiggly::*,
        page_annotation_stamp::*, page_annotation_strikeout::*, page_annotation_text::*,
        page_annotation_underline::*, page_annotation_unsupported::*, page_annotations::*,
        page_boundaries::*, page_object::*, page_object_form_fragment::*, page_object_image::*,
        page_object_path::*, page_object_shading::*, page_object_text::*,
        page_object_unsupported::*, page_objects::*, page_size::*, page_text::*, pages::*,
        pdfium::*,
    };
}

// Conditional compilation is used to compile different implementations of
// the PdfiumLibraryBindings trait depending on whether we are compiling to a
// WASM module, a native shared library, or a statically linked library.

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "static"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "static")]
mod linked;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(test)]
pub mod tests {
    use crate::prelude::*;
    use image::ImageFormat;

    #[test]
    #[cfg(not(feature = "static"))]
    fn dynamic_bindings() {
        let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library());

        assert!(bindings.is_ok());

        let render_config = PdfBitmapConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true)
            .render_form_data(true)
            .render_annotations(true);

        Pdfium::new(bindings.unwrap())
            .load_pdf_from_file("./test/form-test.pdf", None)
            .unwrap()
            .pages()
            .iter()
            .for_each(|page| {
                let result = page
                    .get_bitmap_with_config(&render_config)
                    .unwrap()
                    .as_image()
                    .as_rgba8()
                    .unwrap()
                    .save_with_format(
                        format!("form-test-page-{}.jpg", page.index()),
                        ImageFormat::Jpeg,
                    );

                assert!(result.is_ok());
            });
    }

    #[test]
    #[cfg(feature = "static")]
    fn static_bindings() {
        use crate::prelude::*;

        // Simply checks that the static bindings contain no compilation errors.

        Pdfium::bind_to_statically_linked_library().unwrap();
    }
}
