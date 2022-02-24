use pdfium_render::pdfium::Pdfium;

pub fn main() {
    // For general comments about pdfium-render and binding to Pdfium, see comments in export.rs.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            // For every page in our sample file...

            Pdfium::new(bindings)
                .load_pdf_from_file("test/text-test.pdf", None)
                .unwrap()
                .pages()
                .iter()
                .for_each(|page| {
                    // ... output the text on the page to the console.

                    println!("=============== Page {} ===============", page.index());

                    println!("{}", page.text().unwrap().all());

                    // PdfPageText::all() returns all text across all page objects of type
                    // PdfPageObjectType::Text on the page - this is convenience function,
                    // since it is often useful to extract all the page text in one operation.
                    // We could achieve exactly the same result by iterating over all the page
                    // text objects manually and concatenating the text strings extracted from
                    // each object together, like so:

                    // println!(
                    //     "{}",
                    //     page.objects()
                    //         .iter()
                    //         .filter_map(|object| object
                    //             .as_text_object()
                    //             .map(|object| object.text()))
                    //         .collect::<Vec<_>>()
                    //         .join("")
                    // );
                });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
