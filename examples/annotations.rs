use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    )
    .load_pdf_from_file("test/annotations-test.pdf", None)?
    .pages()
    .iter()
    .enumerate()
    .for_each(|(page_index, page)| {
        // For each page in the document, iterate over the annotations attached to that page.

        println!("=============== Page {} ===============", page_index);

        page.annotations()
            .iter()
            .enumerate()
            .for_each(|(annotation_index, annotation)| {
                println!(
                    "Annotation {} is of type {:?} with bounds {:?}",
                    annotation_index,
                    annotation.annotation_type(),
                    annotation.bounds()
                );

                println!(
                    "{}",
                    page.text().unwrap().for_annotation(&annotation).unwrap()
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
            });
    });

    Ok(())
}
