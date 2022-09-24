use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // For every attachment embedded in our sample file...

    let document = pdfium.load_pdf_from_file("test/attachments-test.pdf", None)?;

    for (index, attachment) in document.attachments().iter().enumerate() {
        // ... output information about the attachment to the console.

        println!("=============== Attachment {} ===============", index);

        println!("Name: {:?}", attachment.name());
        println!("Content length: {} bytes", attachment.bytes().len());
    }

    Ok(())
}
