use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    )
    .load_pdf_from_file("test/form-test.pdf", None)?
    .pages()
    .iter()
    .enumerate()
    .for_each(|(index, page)| {
        // For each page in the document, output the images on the page to files.

        println!("=============== Page {} ===============", index);

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

    Ok(())
}
