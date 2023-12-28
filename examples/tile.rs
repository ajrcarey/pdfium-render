use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // Joins several existing test PDFs together into a new in-memory document, then tiles
    // the pages in that document into a new file.

    // Create a new blank document...

    let mut document = pdfium.create_new_pdf()?;

    // ... append the pages from three test files...

    let pages = document.pages_mut();

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
