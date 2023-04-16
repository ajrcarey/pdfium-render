use std::fs::File;

use pdfium_render::prelude::*;

fn how_many_pages(pdfium: &Pdfium, path: &str) -> Result<u16, PdfiumError> {
    let reader = File::open(path).map_err(PdfiumError::IoError)?;
    let document = pdfium.load_pdf_from_reader(reader, None)?;

    Ok(document.pages().len())
}

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let paths = ["test/form-test.pdf", "test/annotations-test.pdf"];

    for path in paths {
        let page_count = how_many_pages(&pdfium, path)?;
        println!("{:} has {:} pages", path, page_count);
    }

    Ok(())
}
