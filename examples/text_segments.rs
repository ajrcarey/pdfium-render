use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );
    let page = pdfium
        .load_pdf_from_file("test/text-test.pdf", None)?
        .pages()
        .first()?;
    let page_text = page.text().unwrap();
    let segments = page_text.select_segments(10, 300);
    segments
        .iter()
        .for_each(|d| println!("segement: {:?}, {:?}", d.text(), d.bounds()));
    Ok(())
}
