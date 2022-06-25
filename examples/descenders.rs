use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    let bindings =
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")).unwrap();

    let pdfium = Pdfium::new(bindings);

    let document = pdfium.create_new_pdf()?;

    let mut page = document
        .pages()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    // Create a new text object.

    let object = page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(100.0),
        "This is a sentence containing several pleasing words.",
        &PdfFont::helvetica(&document),
        PdfPoints::new(12.0),
    )?;

    let object = object.as_text_object().unwrap();

    let text = page.text()?;

    // Check the text object for descenders.

    println!("has descenders? {:?}", object.has_descenders(&text));

    // Check each character in the text object to see if it's a descender.

    for char in object.chars(&text)?.iter() {
        if char.has_descender() {
            println!("{:?} has a descender", char.unicode_string().unwrap());
        }
    }

    Ok(())
}
