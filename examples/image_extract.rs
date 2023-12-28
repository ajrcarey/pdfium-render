use image::ImageFormat;
use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    Pdfium::default()
        .load_pdf_from_file("test/image-test.pdf", None)?
        .pages()
        .iter()
        .enumerate()
        .for_each(|(page_index, page)| {
            // For each page in the document, output the images on the page to separate files.

            println!("=============== Page {} ===============", page_index);

            page.objects()
                .iter()
                .enumerate()
                .for_each(|(object_index, object)| {
                    if let Some(image) = object.as_image_object() {
                        if let Ok(image) = image.get_raw_image() {
                            println!("Exporting image with object index {} to file", object_index);

                            assert!(image
                                .save_with_format(
                                    format!(
                                        "image-test-page-{}-image-{}.jpg",
                                        page_index, object_index
                                    ),
                                    ImageFormat::Jpeg,
                                )
                                .is_ok());
                        }
                    }
                });
        });

    Ok(())
}
