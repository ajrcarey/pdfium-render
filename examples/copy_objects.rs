use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file("test/text-test.pdf", None)?; // Load the sample file...

    // Move all the page objects on the bottom half of the first page to a new page.

    let source_page = document.pages().get(0)?;

    document
        .pages()
        .delete_page_range(1..document.pages().len())?;

    let mut source_objects = source_page.objects().create_group(|object| {
        object
            .bounds()
            .map(|bounds| {
                // Only select objects on the bottom half of the page.

                bounds.top < source_page.height() / 2.0
            })
            .unwrap_or(false)
    })?;

    println!("{} objects selected on page", source_objects.len());

    source_objects.retain_if_copyable();

    for o in source_objects.iter() {
        if let Some(o) = o.as_text_object() {
            println!("Selected line: {}", o.text());
        }
    }

    println!("{} objects to copy", source_objects.len());

    let mut destination_page = document
        .pages()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    let destination_objects = source_objects.try_copy_onto_existing_page(&mut destination_page)?;

    println!("{} objects copied to page", destination_objects.len());

    source_objects.remove_objects_from_page()?;

    document.save_to_file("test/copy-test.pdf")?;

    Ok(())
}
