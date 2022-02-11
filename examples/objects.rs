use pdfium_render::page_object::PdfPageObjectCommon;
use pdfium_render::pdfium::Pdfium;

pub fn main() {
    // This example differs from export.rs in that we iterate over every page object in each page,
    // outputting information about each to the console. Comments that would duplicate
    // those in export.rs have been removed; only code that substantively differs is commented.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            Pdfium::new(bindings)
                .load_pdf_from_file("test/export-test.pdf", None) // Load the sample file...
                .unwrap()
                .pages()
                .iter()
                .for_each(|page| {
                    println!("=============== Page {} ===============", page.index());

                    page.objects().iter().for_each(|object| {
                        println!(
                            "Page {} object {} is of type {:#?}",
                            page.index(),
                            object.index(),
                            object.object_type()
                        );

                        if let Some(object) = object.as_text_object() {
                            println!(
                                "{} {}-pt: {}",
                                object.font().name(),
                                object.font_size().value,
                                object.text()
                            );
                        }
                    });
                });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
