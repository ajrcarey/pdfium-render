use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // Joins several existing test PDFs together into a new in-memory document, then tiles
    // the pages in that document into a new file.

    // Create a new blank document...

    let document = pdfium.create_new_pdf()?;

    // ... append the pages from three test files...

    let mut pages = document.pages();

    pages.append(&pdfium.load_pdf_from_file("test/text-test.pdf", None)?)?;
    pages.append(&pdfium.load_pdf_from_file("test/export-test.pdf", None)?)?;
    pages.append(&pdfium.load_pdf_from_file("test/form-test.pdf", None)?)?;

    // ... and tile the pages into a new A3 landscape document,
    // saving the tiled document to a file.

    pages
        .tile_into_new_document(2, 3, PdfPagePaperSize::a3().landscape())?
        .save_to_file("test/tile-test.pdf")?;

    Ok(())
}
