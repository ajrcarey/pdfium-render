use pdfium_render::pdfium::Pdfium;
use pdfium_render::PdfPageIndex;
use wasm_bindgen::prelude::*;

// To build this example:

// wasm-pack build examples/wasm --target no-modules

// To run this example:

// * Download PDFium compiled to WASM release tarball from https://github.com/paulo-coutinho/pdfium-lib/releases
// * Extract the files release/node/pdfium.js and release/node/pdfium.wasm from the downloaded tarball
// * Copy extracted files into same folder as build artifacts from wasm-pack build
// * Copy examples/wasm/index.html and examples/wasm/serve.sh into same folder as build artifacts from wasm-pack build
// * Run serve.sh to spin up a server, then visit localhost:4000 in your browser

// Embed the sample PDF file directly into our WASM binary.

const PDF: &[u8] = include_bytes!("../test/test.pdf");

/// Logs the width and height of each page in the sample PDF to the Javascript console.
#[wasm_bindgen]
pub fn log_page_metrics_to_console() {
    console_log::init().expect("Error initializing console-based logging.");

    // Bind to the system library when targeting WASM. The hosting page
    // must have already loaded a pdfium.wasm assembly (e.g. from
    // https://github.com/paulo-coutinho/pdfium-lib/releases) prior to calling
    // this function, or binding will fail.

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
