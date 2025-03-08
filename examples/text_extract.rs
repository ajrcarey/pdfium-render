use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    Pdfium::default()
        .load_pdf_from_file("test/form-test.pdf", None)?
        .pages()
        .iter()
        .enumerate()
        .for_each(|(index, page)| {
            // For each page in the document, output the text on the page to the console.

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

            // For most documents, this approach will return the same result as
            // PdfPageText::all(). There is an edge case, however: page objects
            // of type PdfPageXObjectFormObject are containers which can themselves
            // contain child text objects. To correctly handle this edge case,
            // a visitor pattern approach is necessary:

            // visit_all_text_objects(page.objects().iter());

            // fn visit_all_text_objects(iterator: PdfPageObjectsIterator) {
            //     for object in iterator {
            //         if let Some(text_object) = object.as_text_object() {
            //             // Do something with this text object
            //         } else if let Some(container) = object.as_x_object_form_object() {
            //             // Visit child objects in this container
            //             visit_all_text_objects(container.iter());
            //         }
            //     }
            // }
        });

    Ok(())
}
