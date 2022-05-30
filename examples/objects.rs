use pdfium_render::prelude::*;

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
                .enumerate()
                .for_each(|(page_index, page)| {
                    // ... output information about every object on the page to the console.

                    println!("=============== Page {} ===============", page_index);

                    page.objects()
                        .iter()
                        .enumerate()
                        .for_each(|(object_index, object)| {
                            println!(
                                "Page {} object {} is of type {:?}",
                                page_index,
                                object_index,
                                object.object_type()
                            );

                            println!(
                                "Bounds: {:?}, width: {:?}, height: {:?}",
                                object.bounds(),
                                object.width(),
                                object.height()
                            );

                            // For text objects, we take the extra step of outputting the text
                            // contained by the object.

                            if let Some(object) = object.as_text_object() {
                                println!(
                                    "Text: {} {}-pt {:?}: \"{}\"",
                                    object.font().name(),
                                    object.font_size().value,
                                    object.font().weight().unwrap(),
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
