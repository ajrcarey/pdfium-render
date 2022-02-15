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

// Conditional compilation is used to compile different implementations of
// the PdfiumLibraryBindings trait depending on whether we are compiling to a
// WASM module or to a native shared library.

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(test)]
pub mod tests {
    use crate::bitmap::PdfBitmapRotation;
    use crate::bitmap_config::PdfBitmapConfig;
    use crate::pdfium::Pdfium;
    use image::ImageFormat;

    #[test]
    fn test() {
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
}
