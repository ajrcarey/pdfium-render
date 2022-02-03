use image::ImageFormat;
use pdfium_render::bitmap::PdfBitmapRotation;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::color::PdfColor;
use pdfium_render::pdfium::Pdfium;

pub fn main() {
    // This example differs from export.rs in that our sample file now includes
    // an embedded PDF form, which should also be rendered during image export.
    // Comments that would duplicate those in export.rs have been removed; only
    // code that substantively differs is commented.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            let pdfium = Pdfium::new(bindings);

            let document = pdfium
                .load_pdf_from_file("test/form-test.pdf", None) // Load the sample file...
                .unwrap();

            println!("PDF file version: {:#?}", document.version());

            println!("PDF page mode: {:#?}", document.pages().page_mode());

            println!("PDF metadata tags:");
            document
                .metadata()
                .iter()
                .enumerate()
                .for_each(|(index, tag)| {
                    println!("{}: {:#?} = {}", index, tag.tag_type(), tag.value())
                });

            match document.form() {
                Some(form) => println!(
                    "PDF contains an embedded form of type {:#?}",
                    form.form_type()
                ),
                None => println!("PDF does not contain an embedded form"),
            };

            let dpi = 200.0;

            let render_config = PdfBitmapConfig::new()
                .scale_page_by_factor(dpi as f32 / 72.0)
                .render_form_data(true) // Rendering of form data and annotations is the default...
                .render_annotations(true) // ... but for the sake of demonstration we are explicit here.
                .highlight_text_form_fields(PdfColor::SOLID_YELLOW.with_alpha(128))
                .highlight_checkbox_form_fields(PdfColor::SOLID_BLUE.with_alpha(128));

            document.pages().iter().for_each(|page| {
                if let Some(label) = page.label() {
                    println!("Page {} has a label: {}", page.index(), label);
                }

                if let Ok(rotation) = page.rotation() {
                    if rotation != PdfBitmapRotation::None {
                        println!(
                            "Page {} has embedded rotation of type {:#?}",
                            page.index(),
                            rotation
                        );
                    }
                }

                for boundary in page.boundaries().iter() {
                    println!(
                        "Page {} has defined {:#?} box ({}, {}) - ({}, {})",
                        page.index(),
                        boundary.box_type,
                        boundary.bounds.left.value,
                        boundary.bounds.top.value,
                        boundary.bounds.right.value,
                        boundary.bounds.bottom.value,
                    );
                }

                let mut bitmap = page
                    .get_bitmap_with_config(&render_config) // Initializes a bitmap with the given configuration for this page ...
                    .unwrap();

                let result = bitmap
                    .as_image()
                    .as_bgra8()
                    .unwrap()
                    .save_with_format(format!("form-page-{}.jpg", page.index()), ImageFormat::Jpeg);

                assert!(result.is_ok());
            });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
