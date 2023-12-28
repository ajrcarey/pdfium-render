use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // For every digital signature in our sample file...

    let document = pdfium.load_pdf_from_file("test/signatures-test.pdf", None)?;

    for (index, signature) in document.signatures().iter().enumerate() {
        // ... output information about the signature to the console.

        println!("=============== Signature {} ===============", index);

        if let Some(reason) = signature.reason() {
            println!("Reason: {}", reason);
        }

        if let Some(signing_date) = signature.signing_date() {
            println!("Signing date: {}", signing_date);
        }

        let contents = signature.bytes();

        println!("Content length: {} bytes", contents.len());
        println!("Content: {:?}", contents);
    }

    Ok(())
}
