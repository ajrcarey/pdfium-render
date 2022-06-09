use image::ImageFormat;
use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file("test/form-test.pdf", None)?; // Load the sample file...

    println!("PDF file version: {:#?}", document.version());

    println!("PDF page mode: {:#?}", document.pages().page_mode());

    println!("PDF metadata tags:");

    document
        .metadata()
        .iter()
        .enumerate()
        .for_each(|(index, tag)| println!("{}: {:#?} = {}", index, tag.tag_type(), tag.value()));

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

    for (index, page) in document.pages().iter().enumerate() {
        if let Some(label) = page.label() {
            println!("Page {} has a label: {}", index, label);
        }

        if let Ok(rotation) = page.rotation() {
            if rotation != PdfBitmapRotation::None {
                println!(
                    "Page {} has embedded rotation of type {:#?}",
                    index, rotation
                );
            }
        }

        for boundary in page.boundaries().iter() {
            println!(
                "Page {} has defined {:#?} box ({}, {}) - ({}, {})",
                index,
                boundary.box_type,
                boundary.bounds.left.value,
                boundary.bounds.top.value,
                boundary.bounds.right.value,
                boundary.bounds.bottom.value,
            );
        }

        page.get_bitmap_with_config(&render_config)?
            .as_image()
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(format!("form-page-{}.jpg", index), ImageFormat::Jpeg)
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}
