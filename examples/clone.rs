use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file("test/text-test.pdf", None)?; // Load the sample file...

    // Move all objects on the bottom half of the first page to the first page of a new document.

    let source_page = document.pages().get(0)?;

    document
        .pages()
        .delete_page_range(1..document.pages().len())?;

    let mut source_objects = source_page.objects().create_group(|object| {
        object
            .bounds()
            .map(|bounds| {
                // Only select objects on the bottom half of the page.

                bounds.top > source_page.height() / 2.0
            })
            .unwrap_or(false)
    })?;

    println!("{} objects selected on page", source_objects.len());

    source_objects.retain_if_cloneable();

    println!("{} objects to clone", source_objects.len());

    let mut destination_page = document
        .pages()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    let destination_objects = source_objects.try_clone_onto_page(&mut destination_page)?;

    println!("{} objects cloned onto page", destination_objects.len());

    source_objects.remove_objects_from_page()?;

    println!("{} objects left over", source_objects.len());

    document.save_to_file("test/clone-test.pdf")?;

    Ok(())
}