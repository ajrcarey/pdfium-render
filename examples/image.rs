use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // We'll use rendered pages from existing test PDFs for our images. We'll generate
    // bitmaps from the PDFs in the following list.

    let test_documents = vec![
        "test/path-test.pdf",
        "test/form-test.pdf",
        "test/create-test.pdf",
        "test/text-test.pdf",
    ];

    // Create a new blank document...

    let mut document = pdfium.create_new_pdf()?;

    // ... add a new page...

    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?;

    // ... add some image objects to the page...

    let origin_x = page.width() / 2.0;

    let origin_y = page.height() / 2.0;

    for (index, degrees) in (0..360).step_by(40).enumerate() {
        // Create the image to use for this object.

        let path = test_documents[index % test_documents.len()];

        let target_object_width_on_page = 40.0 + (degrees as f32) * 1.5;

        let target_pixel_width_of_bitmap = (250.0 * (target_object_width_on_page / 100.0)) as u16;

        let image = pdfium
            .load_pdf_from_file(path, None)?
            .pages()
            .first()?
            .render_with_config(
                &PdfRenderConfig::new().set_target_width(target_pixel_width_of_bitmap.into()),
            )?
            .as_image();

        let mut object = PdfPageImageObject::new_with_width(
            &document,
            &image,
            PdfPoints::new(target_object_width_on_page),
        )?;

        // The order of transformations is important here. In particular, the positioning
        // of the object on the page - the call to object.translate() - must take
        // place _after_ the call to object.rotate...(), otherwise the translated
        // co-ordinates will be rotated as well.

        // Progressively rotate the image as we loop.
        object.rotate_clockwise_degrees(degrees as f32)?;

        // Move the object into position in the center of the page.
        object.translate(origin_x, origin_y)?;

        // Add the object to the page, triggering content regeneration.
        page.objects_mut().add_image_object(object)?;
    }

    // ... log details of the objects we just created to the console...

    page.objects()
        .iter()
        .enumerate()
        .for_each(|(index, object)| {
            println!(
                "Page object {} is of type {:?}",
                index,
                object.object_type()
            );

            println!(
                "Bounds: {:?}, width: {:?}, height: {:?}",
                object.bounds(),
                object.width(),
                object.height()
            );
        });

    // ... and save the result to a file.

    document.save_to_file("test/image-test.pdf")
}
