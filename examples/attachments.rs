use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // Create a new, empty document, and attach some of our other sample files to it.

    let mut document = pdfium.create_new_pdf()?;

    document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?
        .objects_mut()
        .create_text_object(
            PdfPoints::new(100.0),
            PdfPoints::new(700.0),
            "This document contains three embedded attachments.",
            document.fonts_mut().helvetica(),
            PdfPoints::new(12.0),
        )?;

    assert_eq!(document.attachments().len(), 0);

    document
        .attachments_mut()
        .create_attachment_from_file("annotations-test.pdf", "test/annotations-test.pdf")?;

    assert_eq!(document.attachments().len(), 1);

    document
        .attachments_mut()
        .create_attachment_from_file("create-test.pdf", "test/create-test.pdf")?;

    assert_eq!(document.attachments().len(), 2);

    document
        .attachments_mut()
        .create_attachment_from_file("path-test.pdf", "test/path-test.pdf")?;

    assert_eq!(document.attachments().len(), 3);

    document.save_to_file("test/attachments-test.pdf")?;

    // Now read back the attachments.

    // For every attachment embedded in our sample file...

    let document = pdfium.load_pdf_from_file("test/attachments-test.pdf", None)?;

    for (index, attachment) in document.attachments().iter().enumerate() {
        // ... output information about the attachment to the console.

        println!("=============== Attachment {} ===============", index);

        println!("Name: {:?}", attachment.name());
        println!("Content length: {} bytes", attachment.len());
    }

    Ok(())
}
