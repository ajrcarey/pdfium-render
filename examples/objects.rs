use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // For every page in our sample file...

    let document = pdfium.load_pdf_from_file("test/export-test.pdf", None)?;

    for (page_index, page) in document.pages().iter().enumerate() {
        // ... output information about every object on the page to the console.

        println!("=============== Page {} ===============", page_index);

        for (object_index, object) in page.objects().iter().enumerate() {
            println!(
                "Page {} object {} is of type {:?}",
                page_index,
                object_index,
                object.object_type()
            );

            println!(
                "Bounds: {:?}, width: {:?}, height: {:?}",
                object.bounds()?,
                object.width()?,
                object.height()?
            );

            // For text objects, we take the extra step of outputting the text
            // contained by the object.

            if let Some(object) = object.as_text_object() {
                println!(
                    "Text: {} {}-pt {:?}: \"{}\"",
                    object.font().name(),
                    object.unscaled_font_size().value,
                    object.font().weight()?,
                    object.text()
                );
            }

            // Retrieving the text from a text object is done internally by loading the "text page"
            // associated with the page the object is attached to, then asking that text page for the
            // text related to the object. Therefore, when iterating over many text objects (as we
            // are doing here), it is slightly faster to load the text page once rather than loading
            // it and closing it every time we access an object:

            // let text_page = page.text()?; // Opens the text page once.
            //
            // for object in <page objects iterator> {
            //     let object_text = text_page.for_object(object)?;
            // }

            // The text page will be closed when the text_page binding falls out of scope.

            // If we wanted to extract _all_ the text contained by all the text objects
            // on the page, an easier way would be to simply use:

            // page.text()?.all()
        }
    }

    Ok(())
}
