use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let mut document = pdfium.create_new_pdf()?;

    let helvetica = document.fonts_mut().helvetica_bold();

    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?;

    // Create three page objects of contrasting types on the page...

    let text_object = page.objects_mut().create_text_object(
        PdfPoints::new(75.0),
        PdfPoints::new(150.0),
        "A text object",
        helvetica,
        PdfPoints::new(32.0),
    )?;

    let path_object = page.objects_mut().create_path_object_circle_at(
        PdfPoints::new(300.0),
        PdfPoints::new(350.0),
        PdfPoints::new(75.0),
        Some(PdfColor::SOLID_RED),
        Some(PdfPoints::new(10.0)),
        Some(PdfColor::SOLID_GREEN),
    )?;

    let render_target = pdfium.load_pdf_from_file("test/signatures-test.pdf", None)?;

    let image = render_target
        .pages()
        .first()?
        .render(300, 450, None)?
        .as_image();

    let image_object = page.objects_mut().create_image_object(
        PdfPoints::new(400.0),
        PdfPoints::new(500.0),
        &image,
        Some(PdfPoints::new(150.0)),
        Some(PdfPoints::new(225.0)),
    )?;

    // ... and attach a variety of annotations to those objects.

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

    document.save_to_file("test/create-annotations-test.pdf")
}
