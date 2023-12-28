use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_file("test/form-test.pdf", None)?;

    for (page_index, page) in document.pages().iter().enumerate() {
        // For each page in the document, iterate over the annotations attached to that page.

        println!("=============== Page {} ===============", page_index);

        for (annotation_index, annotation) in page.annotations().iter().enumerate() {
            println!(
                "Annotation {} is of type {:?} with bounds {:?}",
                annotation_index,
                annotation.annotation_type(),
                annotation.bounds()
            );

            println!(
                "Annotation {} text: {:?}",
                annotation_index,
                page.text().unwrap().for_annotation(&annotation).ok()
            );

            println!(
                "Annotation {} name: {:?}",
                annotation_index,
                annotation.name()
            );

            println!(
                "Annotation {} contents: {:?}",
                annotation_index,
                annotation.contents()
            );

            println!(
                "Annotation {} author: {:?}",
                annotation_index,
                annotation.creator()
            );

            println!(
                "Annotation {} created: {:?}",
                annotation_index,
                annotation.creation_date()
            );

            println!(
                "Annotation {} last modified: {:?}",
                annotation_index,
                annotation.modification_date()
            );

            println!(
                "Annotation {} contains {} page objects",
                annotation_index,
                annotation.objects().len()
            );

            for (object_index, object) in annotation.objects().iter().enumerate() {
                println!(
                    "Annotation {} page object {} is of type {:?}",
                    annotation_index,
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
            }
        }
    }

    Ok(())
}
