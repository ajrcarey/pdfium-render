use pdfium_render::{page_text_search::SearchOption, prelude::*};

pub fn main() -> Result<(), PdfiumError> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );
    let page = pdfium
        .load_pdf_from_file("test/text-test.pdf", None)?
        .pages()
        .first()?;
    let search_option = SearchOption {
        match_case: false,
        match_whole_world: false,
        consecutive: false,
    };
    let page_text = page.text().unwrap();
    let search = page_text.search("the", &search_option, 0);
    search.iter(true).for_each(|segments| {
        let index_range = segments.index_range();
        println!("search result: {:?}", index_range.1 - index_range.0);
        segments
            .iter()
            .for_each(|d| println!("segement: {:?}", d.text()));
    });
    Ok(())
}
