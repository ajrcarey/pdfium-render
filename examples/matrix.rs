use pdfium_render::prelude::*;
use rand::random;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // A variety of objects provided by Pdfium can be transformed: positioned, rotated, scaled,
    // and skewed. All transformable objects provide convenient functions for doing so, but
    // occasionally it is more convenient to "queue up" our transformation in advance using a
    // transformation matrix that can then later be applied to any transformable object. This
    // example demonstrates this. First, we set up a transformation we wish to apply by
    // configuring a transformation matrix. Then, we create a variety of PDF objects, transforming
    // each in turn by applying the matrix.

    let matrix = PdfMatrix::IDENTITY
        .scale(1.5, 1.2)? // Uneven horizontal and vertical scale factors will "squish" the object
        .skew_degrees(0.0, 10.0)? // "Lean" the object to the right
        // PdfMatrix uses the builder pattern with function chaining, so we could
        // "queue up" any number of operations here if we wished, e.g.:
        //
        // .rotate_clockwise_degrees()?
        // .flip_vertically()?
        // .translate(..., ...)?
        //
        // ... and so on.
        ;

    // Our transformation matrix is now ready. Let's create an empty page in a new document...

    let mut document = pdfium.create_new_pdf()?;

    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?;

    // ... and now place some random objects on the page, transforming each using our matrix.

    let page_width = page.width();

    let page_height = page.height();

    for index in 0..100 {
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
        object.apply_matrix(matrix)?;
        page.objects_mut().add_path_object(object)?;
    }

    // We can effect a transformation of all objects on a page in a single operation
    // by applying the matrix directly to the page itself. (In this example, we use the same matrix
    // that we already applied to each object individually; this has the effect of applying
    // the matrix to every object twice.)

    page.apply_matrix(matrix)?;

    // Save the result.

    document.save_to_file("test/matrix-test.pdf")
}
