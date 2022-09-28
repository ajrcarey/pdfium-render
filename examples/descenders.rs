use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.create_new_pdf()?;

    let mut page = document
        .pages()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    let object = page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(100.0),
        "This is a sentence containing several pleasing words.",
        &PdfFont::helvetica(&document),
        PdfPoints::new(12.0),
    )?;

    let text = page.text()?;

    // Create a new text object.

    if let Some(object) = object.as_text_object() {
        // Check the text object for descenders.

        println!(
            "Text object has descenders? {:?}",
            object.has_descenders(&text).unwrap_or(false)
        );

        // Check each character in the text object to see if it's a descender.

        for (index, char) in object.chars(&text)?.iter().enumerate() {
            if char.has_descender() {
                println!(
                    "Character {}: \"{}\" descends {} points below the baseline",
                    index,
                    char.unicode_string().unwrap(),
                    object.get_vertical_translation().value - char.tight_bounds()?.bottom.value
                );
            }
        }

        println!(
            "Maximum descent of text object: {}",
            object.descent(&text)?.value
        );
    }

    Ok(())
}
