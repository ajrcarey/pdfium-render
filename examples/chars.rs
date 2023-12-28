use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.create_new_pdf()?;

    let mut page = document
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    // Create a new text object.

    let object = page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(100.0),
        "This is a sentence containing several pleasing words.",
        document.fonts_mut().helvetica(),
        PdfPoints::new(12.0),
    )?;

    // Scan for word boundaries inside our text object.

    let mut word = String::new();

    let mut start_of_word: Option<PdfRect> = None;

    if let Some(object) = object.as_text_object() {
        let text = page.text()?;

        let chars = text.chars_for_object(object)?;

        for (index, char) in chars.iter().enumerate() {
            assert!(char.unicode_string().is_some());

            let str = char.unicode_string().unwrap();

            if start_of_word.is_none() {
                // We found the start of a new word.

                start_of_word = Some(char.loose_bounds()?);
                word += &str;
            } else if str == " " {
                if let Some(start) = start_of_word {
                    // We found the end of the current word.

                    println!(
                        "{}: ({}, {}) - ({}, {})",
                        word,
                        start.left.value,
                        start.bottom.value,
                        char.loose_bounds()?.left.value, // The word ends at the space's leading (left) edge
                        char.loose_bounds()?.top.value
                    );

                    // Prepare for the next word.

                    start_of_word = None;
                    word = String::new();
                }
            } else {
                // We're progressing through the middle of a word.

                word += &str;
            }

            if index == chars.len() - 1 {
                // We're at the end of the text string. Output the final word.

                if let Some(start) = start_of_word {
                    println!(
                        "{}: ({}, {}) - ({}, {})",
                        word,
                        start.left.value,
                        start.bottom.value,
                        char.loose_bounds()?.right.value,
                        char.loose_bounds()?.top.value
                    );
                }
            }
        }
    }

    Ok(())
}
