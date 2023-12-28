use pdfium_render::prelude::*;
use rand::random;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // Create a new blank document...

    let mut document = pdfium.create_new_pdf()?;

    // ... add a new page...

    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?;

    let page_width = page.width();

    let page_height = page.height();

    // ... add some path objects to the page...

    page.objects_mut().create_path_object_line(
        PdfPoints::new(0.0),
        PdfPoints::new(0.0),
        PdfPoints::new(100.0),
        PdfPoints::new(100.0),
        PdfColor::BLUE,
        PdfPoints::new(5.0),
    )?;

    page.objects_mut().create_path_object_line(
        page_width,
        PdfPoints::new(0.0),
        page_width - PdfPoints::new(100.0),
        PdfPoints::new(100.0),
        PdfColor::GREEN,
        PdfPoints::new(5.0),
    )?;

    page.objects_mut().create_path_object_line(
        PdfPoints::new(0.0),
        page_height,
        PdfPoints::new(100.0),
        page_height - PdfPoints::new(100.0),
        PdfColor::RED,
        PdfPoints::new(5.0),
    )?;

    page.objects_mut().create_path_object_line(
        page_width,
        page_height,
        page_width - PdfPoints::new(100.0),
        page_height - PdfPoints::new(100.0),
        PdfColor::YELLOW,
        PdfPoints::new(5.0),
    )?;

    // ... some manually placed filled shapes...

    page.objects_mut().create_path_object_rect(
        PdfRect::new(
            PdfPoints::new(200.0),
            PdfPoints::new(100.0),
            PdfPoints::new(400.0),
            PdfPoints::new(400.0),
        ),
        Some(PdfColor::MAGENTA),
        Some(PdfPoints::new(7.0)),
        Some(PdfColor::YELLOW),
    )?;

    page.objects_mut().create_path_object_circle_at(
        PdfPoints::new(400.0),
        PdfPoints::new(450.0),
        PdfPoints::new(150.0),
        Some(PdfColor::CYAN),
        Some(PdfPoints::new(4.0)),
        Some(PdfColor::RED.with_alpha(127)),
    )?;

    page.objects_mut().create_path_object_ellipse_at(
        page_width / 2.0,
        page_height - PdfPoints::new(200.0),
        page_width / 2.0 * 0.75,
        PdfPoints::new(75.0),
        Some(PdfColor::GREEN.with_alpha(127)),
        Some(PdfPoints::new(4.0)),
        Some(PdfColor::BLUE.with_alpha(127)),
    )?;

    // ... some randomly-generated unfilled shapes...

    for (index, degrees) in (0..360).enumerate() {
        // Fizz buzz it!

        let x1 = PdfPoints::new(random::<f32>() * page_width.value);

        let y1 = PdfPoints::new(random::<f32>() * page_height.value);

        let stroke_width = PdfPoints::new(random::<f32>() * 5.0);

        let stroke_color = PdfColor::new(random(), random(), random(), random());

        let mut object = match (index % 3, index % 5) {
            (0, 0) => {
                // Ellipses for fizz buzz.

                let x_radius = PdfPoints::new(random::<f32>() * page_width.value * 0.5);

                let y_radius = PdfPoints::new(random::<f32>() * page_height.value * 0.5);

                PdfPagePathObject::new_ellipse_at(
                    &document,
                    x1,
                    y1,
                    x_radius,
                    y_radius,
                    Some(stroke_color),
                    Some(stroke_width),
                    None,
                )?
            }
            (0, _) => {
                // Rectangles for fizz.

                let x2 = PdfPoints::new(random::<f32>() * page_width.value);

                let y2 = PdfPoints::new(random::<f32>() * page_height.value);

                PdfPagePathObject::new_rect(
                    &document,
                    PdfRect::new(y1, x1, y2, x2),
                    Some(stroke_color),
                    Some(stroke_width),
                    None,
                )?
            }
            (_, 0) => {
                // Circles for buzz.

                let radius = PdfPoints::new(random::<f32>() * page_width.value * 0.5);

                PdfPagePathObject::new_circle_at(
                    &document,
                    x1,
                    y1,
                    radius,
                    Some(stroke_color),
                    Some(stroke_width),
                    None,
                )?
            }
            _ => {
                // Lines for everything else.

                let x2 = PdfPoints::new(random::<f32>() * page_width.value);

                let y2 = PdfPoints::new(random::<f32>() * page_height.value);

                PdfPagePathObject::new_line(&document, x1, y1, x2, y2, stroke_color, stroke_width)?
            }
        };

        object.set_blend_mode(PdfPageObjectBlendMode::Multiply)?;
        object.skew_degrees(0.0, index as f32 / 2.0)?;
        object.rotate_clockwise_degrees(degrees as f32)?;
        page.objects_mut().add_path_object(object)?;
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

            println!(
                "Fill color: {:?}, stroke color: {:?}, stroke width: {:?}",
                object.fill_color(),
                object.stroke_color(),
                object.stroke_width()
            );

            if let Some(path) = object.as_path_object() {
                println!(
                    "Fill mode: {:?}, is stroked? {:?}",
                    path.fill_mode(),
                    path.is_stroked()
                );
            }
        });

    // ... and save the result to a file.

    document.save_to_file("test/path-test.pdf")
}
