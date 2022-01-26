#[cfg(target_arch = "wasm32")]
use pdfium_render::bitmap_config::PdfBitmapConfig;
#[cfg(target_arch = "wasm32")]
use pdfium_render::color::PdfColor;
#[cfg(target_arch = "wasm32")]
use pdfium_render::pages::PdfPageIndex;
#[cfg(target_arch = "wasm32")]
use pdfium_render::pdfium::Pdfium;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// See https://github.com/ajrcarey/pdfium-render/tree/master/examples for information
// on how to build and package this example alongside a WASM build of Pdfium, suitable
// for running in a browser.

// We embed the sample PDF file directly into our WASM binary.

#[cfg(target_arch = "wasm32")]
const PDF: &[u8] = include_bytes!("../test/form-test.pdf");

/// Logs the width and height of each page in the sample PDF, along with other
/// document metrics, to the Javascript console.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn log_page_metrics_to_console() {
    console_log::init().expect("Error initializing console-based logging.");

    // Our only option when targeting WASM is to bind to the "system library"
    // (a separate WASM build of Pdfium).

    let bindings = Pdfium::bind_to_system_library().unwrap();

    let pdfium = Pdfium::new(bindings);

    let document = pdfium.load_pdf_from_bytes(PDF, None).unwrap();

    // Output metadata and form information for the PDF file to the console.

    log::info!("PDF file version: {:#?}", document.version());

    log::info!("PDF metadata tags:");
    document
        .metadata()
        .iter()
        .enumerate()
        .for_each(|(index, tag)| log::info!("{}: {:#?} = {}", index, tag.tag_type(), tag.value()));

    match document.form() {
        Some(form) => log::info!(
            "PDF contains an embedded form of type {:#?}",
            form.form_type()
        ),
        None => log::info!("PDF does not contain an embedded form"),
    };

    // Report labels and metrics for each page to the console.

    document.pages().iter().for_each(|page| {
        if let Some(label) = page.label() {
            log::info!("Page {} has a label: {}", page.index(), label);
        }

        log::info!(
            "page index: {}, width: {}, height: {}",
            page.index(),
            page.width(),
            page.height()
        );
    });
}

/// Returns the raw image byte data for a nominated page in the PDF file.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_image_data_for_page(index: PdfPageIndex, width: u16, height: u16) -> Vec<u8> {
    Pdfium::new(Pdfium::bind_to_system_library().unwrap())
        .load_pdf_from_bytes(PDF, None)
        .unwrap()
        .pages()
        .get(index)
        .unwrap()
        .get_bitmap_with_config(
            &PdfBitmapConfig::new()
                .set_target_size(width, height)
                .render_form_data(true)
                .highlight_text_form_fields(PdfColor::SOLID_YELLOW.with_alpha(128))
                .highlight_checkbox_form_fields(PdfColor::SOLID_BLUE.with_alpha(128)),
        )
        .unwrap()
        .as_bytes()
        .to_owned()
}

// Source files in examples/ directory are expected to always have a main() entry-point.
// Since we're compiling to WASM, we'll never actually use this.
#[allow(dead_code)]
fn main() {}
