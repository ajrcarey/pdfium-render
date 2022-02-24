use pdfium_render::page_object::PdfPageObjectCommon;
use pdfium_render::pdfium::Pdfium;

pub fn main() {
    // For general comments about pdfium-render and binding to Pdfium, see comments in export.rs.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            // For every page in our sample file...

            Pdfium::new(bindings)
                .load_pdf_from_file("test/export-test.pdf", None)
                .unwrap()
                .pages()
                .iter()
                .for_each(|page| {
                    // ... output information about every object on the page to the console.

                    println!("=============== Page {} ===============", page.index());

                    page.objects().iter().for_each(|object| {
                        println!(
                            "Page {} object {} is of type {:#?}",
                            page.index(),
                            object.index(),
                            object.object_type()
                        );

                        // For text objects, we take the extra step of outputting the text
                        // contained by the object.

                        if let Some(object) = object.as_text_object() {
                            println!(
                                "{} {}-pt: {}",
                                object.font().name(),
                                object.font_size().value,
                                object.text()
                            );
                        }

                        // If we wanted to extract _all_ the text contained by all the text objects
                        // on the page, an easier way would be to simply use

                        // page.text().unwrap().all()
                    });
                });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
