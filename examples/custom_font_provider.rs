use image_025::ImageFormat;
use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    // We initialize Pdfium with a custom configuration that includes the custom font
    // provider implementation below. Pdfium will use this font provider to resolve
    // fonts in the sample document we load. Due to the (deliberate) limitations of
    // our font provider, the same font will be used for every text object contained in
    // the sample document, irrespective of the font that was requested.

    let mut pdfium = Pdfium::new_with_config(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))?,
        PdfiumLibraryConfig::new().clear_user_font_paths(), // Now Pdfium's internal font mapper won't know where to find any fonts
    );

    pdfium.set_custom_font_provider(Box::new(LatoFontProvider::new()));

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // We target a hand-crafted sample document: a single page containing a single text object
    // that references a custom font that is not embedded in the document. The hand-crafted
    // sample is a plain text file, so you can open it in a text editor to see how it was
    // constructed. It includes a single text object that uses the font "AFontThatDoesNotExist".

    let document = pdfium.load_pdf_from_file("test/custom-font-provider-test.pdf", None)?;

    // Any request to render the text object will trigger font substitution.

    let result = document
        .pages()
        .first()?
        .render_with_config(&render_config)?
        .as_image()?
        .into_rgb8()
        .save_with_format("test/custom-font-provider-test.jpg", ImageFormat::Jpeg);

    assert!(result.is_ok());

    Ok(())
}

// A trivial implementation of the PdfiumCustomFontProvider trait.
//
// This implementation responds to every font lookup query from Pdfium by returning
// the Lato Regular TrueType font.
pub struct LatoFontProvider {
    next_id: PdfiumCustomFontHandle,
    lato: &'static [u8],
}

impl LatoFontProvider {
    fn new() -> Self {
        LatoFontProvider {
            next_id: 0,
            lato: include_bytes!("../test/Lato-Regular.ttf"),
        }
    }
}

impl PdfiumCustomFontProvider for LatoFontProvider {
    fn provide(
        &mut self,
        request: PdfiumCustomFontProviderRequest,
    ) -> Option<PdfiumCustomFontProviderResponse> {
        // This trait method processes font lookup requests from Pdfium. In a
        // real implementation, the fields in the request would be used to
        // select the correct font to use, perhaps using fonts stored on disk,
        // cached in memory, retrieved from a database, from the network, or even
        // generated dynamically at runtime. But in this sample implementation, we'll
        // simply return the same font for every request we receive from Pdfium.

        self.next_id += 1;

        println!(
            "Responding to font request {} from Pdfium: {}",
            self.next_id, request.font_face
        );

        Some(PdfiumCustomFontProviderResponse {
            id: self.next_id,
            font_face: request.font_face.clone(),
            character_set: request.character_set,
            data: self.lato.to_vec(),
        })
    }
}
