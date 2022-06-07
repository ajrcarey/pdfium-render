use pdfium_render::prelude::*;
use rand::random;

fn main() {
    // For general comments about pdfium-render and binding to Pdfium, see comments in export.rs.

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(bindings) => {
            let pdfium = Pdfium::new(bindings);

            // Create a new blank document...

            let document = pdfium.create_new_pdf().unwrap();

            // ... add a new page...

            let mut pages = document.pages();

            let mut page = pages.create_page_at_start(PdfPagePaperSize::a4()).unwrap();

            // ... add some text objects to the page...

            let font = PdfFont::courier_bold(&document);

            let base_font_size = 10.0;

            let origin_x = page.width() / 2.0;

            let origin_y = page.height() / 2.0;

            for (index, degrees) in (0..360).step_by(10).enumerate() {
                let index = index as f32;

                let mut object = PdfPageTextObject::new(
                    &document,
                    "Hello world from Pdfium!",
                    &font,
                    PdfPoints::new(base_font_size + index),
                )
                .unwrap();

                object
                    .set_fill_color(PdfColor::new(random(), random(), random(), 255))
                    .unwrap();

                object.set_blend_mode(PdfPageObjectBlendMode::Multiply);

                // Create a little bit of indent space before the text, so the start of "Hello"
                // is visible without all the rotated objects overlapping too much.
                object.translate(PdfPoints::new(30.0), PdfPoints::ZERO);

                // The order of transformations is important here. In particular, the positioning
                // of the object on the page - the call to object.translate() - must take
                // place _after_ the call to object.rotate...(), otherwise the translated
                // co-ordinates will be rotated as well.

                // Progressively skew the text as we loop.
                object.skew_degrees(0.0, index / 2.0);

                // Progressively rotate the text as we loop.
                object.rotate_clockwise_degrees(degrees as f32);

                // Move the object into position in the center of the page.
                object.translate(origin_x, origin_y);

                // Add the object to the page, triggering content regeneration.
                page.objects_mut().add_text_object(object).unwrap();
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

            document.save_to_file("test/create-test.pdf").unwrap();
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
