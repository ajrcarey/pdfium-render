use pdfium_render::pdfium::Pdfium;

pub fn main() {
    // This example differs from export.rs in that we extract text from each page
    // rather than exporting a bitmap of the page to a file. Comments that would duplicate
    // those in export.rs have been removed; only code that substantively differs is commented.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            Pdfium::new(bindings)
                .load_pdf_from_file("test/text-test.pdf", None) // Load the sample file...
                .unwrap()
                .pages()
                .iter()
                .for_each(|page| {
                    println!("=============== Page {} ===============", page.index());

                    println!("{}", page.text().unwrap().all());
                });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
