use pdfium_render::prelude::*;

fn main() {
    // For general comments about pdfium-render and binding to Pdfium, see comments in export.rs.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            let pdfium = Pdfium::new(bindings);

            // Joins several existing test PDFs together into a new in-memory document, then tiles
            // the pages in that document into a new file.

            // Create a new blank document...

            let mut document = pdfium.create_new_pdf().unwrap();

            // ... append all pages from three test files using PdfDocument::append() ...

            document
                .append(
                    &pdfium
                        .load_pdf_from_file("test/text-test.pdf", None)
                        .unwrap(),
                )
                .unwrap();

            document
                .append(
                    &pdfium
                        .load_pdf_from_file("test/export-test.pdf", None)
                        .unwrap(),
                )
                .unwrap();

            document
                .append(
                    &pdfium
                        .load_pdf_from_file("test/form-test.pdf", None)
                        .unwrap(),
                )
                .unwrap();

            // ... and tile the pages into a new A3 landscape document,
            // saving the tiled document to a file.

            document
                .pages()
                .tile_into_new_document(2, 3, PdfPagePaperSize::a3().landscape())
                .unwrap()
                .save_to_file("test/tile-test.pdf")
                .unwrap();
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
