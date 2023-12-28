use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // For each path object in our single-page sample file...

    let document = pdfium.load_pdf_from_file("test/segments-test.pdf", None)?;

    for (object_index, object) in document.pages().first()?.objects().iter().enumerate() {
        if let Some(object) = object.as_path_object() {
            // ... output information about every path segment in the object to the console.

            println!("=============== Object {} ===============", object_index);

            println!(
                "Bounds: {:?}, width: {:?}, height: {:?}",
                object.bounds()?,
                object.width()?,
                object.height()?
            );

            for (segment_index, segment) in object.segments().iter().enumerate() {
                println!(
                    "Segment {}: {:#?} with x = {}, y = {}",
                    segment_index,
                    segment.segment_type(),
                    segment.x().value,
                    segment.y().value
                );
            }
        }
    }

    Ok(())
}
