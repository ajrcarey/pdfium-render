use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.create_new_pdf()?;

    let helvetica = document.fonts_mut().helvetica_bold();

    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?;

    // Create three page objects of contrasting types on the page...

    let _text_object = page.objects_mut().create_text_object(
        PdfPoints::new(75.0),
        PdfPoints::new(150.0),
        "A text object",
        helvetica,
        PdfPoints::new(32.0),
    )?;

    let _path_object = page.objects_mut().create_path_object_circle_at(
        PdfPoints::new(300.0),
        PdfPoints::new(350.0),
        PdfPoints::new(75.0),
        Some(PdfColor::RED),
        Some(PdfPoints::new(10.0)),
        Some(PdfColor::GREEN),
    )?;

    let render_target = pdfium.load_pdf_from_file("test/signatures-test.pdf", None)?;

    let image = render_target
        .pages()
        .first()?
        .render(300, 450, None)?
        .as_image();

    let _image_object = page.objects_mut().create_image_object(
        PdfPoints::new(400.0),
        PdfPoints::new(500.0),
        &image,
        Some(PdfPoints::new(150.0)),
        Some(PdfPoints::new(225.0)),
    )?;

    // ... and attach a variety of annotations to those objects.

    // Annotations can be created and positioned manually. This allows for maximum flexibility.

    let mut text_annotation = page
        .annotations_mut()
        .create_text_annotation("A pop-up comment on this pretty picture")?;

    println!(
        "Text annotation creation date: {:?}",
        text_annotation.creation_date()
    );

    text_annotation.set_position(PdfPoints::new(150.0), PdfPoints::new(400.0))?;
    text_annotation.set_width(PdfPoints::new(75.0))?;
    text_annotation.set_height(PdfPoints::new(30.0))?;

    println!(
        "Text annotation modification date after positioning: {:?}",
        text_annotation.modification_date()
    );

    let mut free_text_annotation = page
        .annotations_mut()
        .create_free_text_annotation("An inline comment on this pretty picture")?;

    println!(
        "Free text annotation creation date: {:?}",
        free_text_annotation.creation_date()
    );

    free_text_annotation.set_position(PdfPoints::new(150.0), PdfPoints::new(450.0))?;
    free_text_annotation.set_width(PdfPoints::new(100.0))?;
    free_text_annotation.set_height(PdfPoints::new(50.0))?;

    println!(
        "Free text annotation modification date after positioning: {:?}",
        free_text_annotation.modification_date()
    );

    let mut link_annotation = page
        .annotations_mut()
        .create_link_annotation("https://www.google.com")?;

    println!(
        "Link annotation creation date: {:?}",
        link_annotation.creation_date()
    );

    link_annotation.set_position(PdfPoints::new(250.0), PdfPoints::new(550.0))?;
    link_annotation.set_width(PdfPoints::new(100.0))?;
    link_annotation.set_height(PdfPoints::new(50.0))?;
    link_annotation
        .attachment_points_mut()
        .create_attachment_point_at_end(PdfQuadPoints::from_rect(PdfRect::new_from_values(
            100.0, 100.0, 150.0, 150.0,
        )))?;

    for attachment_point in link_annotation.attachment_points().iter() {
        println!(
            "Attachment point in link annotation: {:#?}",
            attachment_point
        );
    }

    println!(
        "Link annotation modification date after positioning: {:?}",
        link_annotation.modification_date()
    );

    // PdfPageAnnotations also includes convenience functions for creating, positioning,
    // and configuring markup annotations relative to a page object in a single function call.
    // This doesn't offer the same flexibility as creating and configuring the annotation
    // manually, but for the most common scenarios it is the easiest and most convenient
    // way of creating an annotation.

    let squiggly_annotation = page
        .annotations_mut()
        .create_squiggly_annotation_under_object(
            &_text_object,
            PdfColor::DARK_RED,
            Some("This is a squiggly annotation"),
        )?;

    println!(
        "Squiggly annotation creation date: {:?}",
        squiggly_annotation.creation_date()
    );

    for attachment_point in squiggly_annotation.attachment_points().iter() {
        println!(
            "Attachment point in squiggly annotation: {:#?}",
            attachment_point
        );
    }

    let strikeout_annotation = page
        .annotations_mut()
        .create_strikeout_annotation_through_object(
            &_text_object,
            PdfColor::ORANGE_RED,
            Some("This is a strikeout annotation"),
        )?;

    println!(
        "Strikeout annotation creation date: {:?}",
        strikeout_annotation.creation_date()
    );

    for attachment_point in strikeout_annotation.attachment_points().iter() {
        println!(
            "Attachment point in strikeout annotation: {:#?}",
            attachment_point
        );
    }

    let highlight_annotation = page
        .annotations_mut()
        .create_highlight_annotation_over_object(
            &_text_object,
            PdfColor::YELLOW,
            Some("This is a highlight annotation"),
        )?;

    println!(
        "Highlight annotation creation date: {:?}",
        highlight_annotation.creation_date()
    );

    for attachment_point in highlight_annotation.attachment_points().iter() {
        println!(
            "Attachment point in highlight annotation: {:#?}",
            attachment_point
        );
    }

    document.save_to_file("test/create-annotations-test.pdf")
}
