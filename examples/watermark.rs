use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.load_pdf_from_file("test/text-test.pdf", None)?;

    // Add a page number and a large text watermark to every page in the document.

    let font = document.fonts_mut().helvetica();

    document.pages().watermark(|group, index, width, height| {
        // Create a page number at the very top of the page.

        let mut page_number = PdfPageTextObject::new(
            &document,
            format!("Page {}", index + 1),
            font,
            PdfPoints::new(14.0),
        )?;

        page_number.set_fill_color(PdfColor::GREEN)?;

        page_number.translate(
            (width - page_number.width()?) / 2.0, // Horizontally center the page number...
            height - page_number.height()?,       // ... and vertically position it at the page top.
        )?;

        group.push(&mut page_number.into())?;

        // Create a large text watermark in the center of the page.

        let mut watermark =
            PdfPageTextObject::new(&document, "Watermark", font, PdfPoints::new(150.0))?;

        watermark.set_fill_color(PdfColor::BLUE.with_alpha(128))?;
        watermark.rotate_counter_clockwise_degrees(45.0)?;
        watermark.translate(
            (width - watermark.width()?) / 2.0 + PdfPoints::new(75.0),
            (height - watermark.height()?) / 2.0,
        )?;

        group.push(&mut watermark.into())?;

        Ok(())
    })?;

    document.save_to_file("test/watermark-test.pdf")
}
