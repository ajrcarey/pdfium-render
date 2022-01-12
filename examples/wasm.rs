#[cfg(target_arch = "wasm32")]
use pdfium_render::pdfium::Pdfium;
#[cfg(target_arch = "wasm32")]
use pdfium_render::PdfPageIndex;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// See https://github.com/ajrcarey/pdfium-render/tree/master/examples for information
// on how to build and package this example alongside a WASM build of Pdfium, suitable
// for running in a browser.

// We embed the sample PDF file directly into our WASM binary.

#[cfg(target_arch = "wasm32")]
const PDF: &[u8] = include_bytes!("../test/test.pdf");

/// Logs the width and height of each page in the sample PDF to the Javascript console.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn log_page_metrics_to_console() {
    #[cfg(target_arch = "wasm32")]
    console_log::init().expect("Error initializing console-based logging.");

    // Our only option when targeting WASM is to bind to the "system library"
    // (a separate WASM build of Pdfium).

    let bindings = Pdfium::bind_to_system_library().unwrap();

    // Report metrics for each page in the PDF file to the console.

    Pdfium::new(bindings)
        .load_pdf_from_bytes(PDF, None)
        .unwrap()
        .pages()
        .for_each(|page| {
            log::info!(
                "page index: {}, width: {}, height: {}",
                page.index(),
                page.width(),
                page.height()
            );
        });
}

/// Returns the raw JPEG byte data for the first page in the PDF file. This can be used
/// to populate an HTML <img> tag.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_image_data_for_page(index: PdfPageIndex, width: u16, height: u16) -> Vec<u8> {
    Pdfium::new(Pdfium::bind_to_system_library().unwrap())
        .load_pdf_from_bytes(PDF, None)
        .unwrap()
        .get_page(index)
        .unwrap()
        .get_bitmap(width, height, None)
        .unwrap()
        .as_bytes()
        .to_owned()
}

// Source files in examples/ directory are expected to always have a main() entry-point.
// Since we're compiling to WASM, we'll never actually use this.
#[allow(dead_code)]
fn main() {}
