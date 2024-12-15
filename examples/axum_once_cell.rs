use axum::{http::StatusCode, routing::get, Router};
use itertools::Itertools;
use pdfium_render::prelude::*;
use tokio::sync::{Mutex, OnceCell};

// A demonstration of thread-safe use of Pdfium in an asynchronous task context.
//
// It is recommended to review the documentation in the thread_safe.rs example first.
//
// This example must be compiled with pdfium-render's sync feature enabled. At the time
// of writing the sync feature was _not_ enabled by default; it must be specified manually
// when compiling this example:
//
// cargo run --example axum_once_cell --features="sync"
//
// Load http://localhost:3000/test in your browser once the example is running to see
// the output. Press CTRL-C in your terminal to quit the example.

// A single Pdfium instance that will be shared by all asynchronous tasks.

static PDFIUM: OnceCell<Mutex<Pdfium>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    // Install a Pdfium instance into the OnceCell.

    PDFIUM
        .get_or_init(|| async { Mutex::new(Pdfium::default()) })
        .await;

    // Create an Axum application with a simple route.

    let app = Router::new().route("/test", get(test));

    // Run our application with Hyper, listening globally on port 3000.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test() -> Result<String, StatusCode> {
    // Scoping access to Pdfium allows the mutex guard to be dropped early, minimizing
    // blocking on other tasks.

    let text = {
        let pdfium = PDFIUM
            .get()
            .expect("Could not get Pdfium from OnceCell")
            .lock()
            .await;

        // Load a document...

        let pdf = pdfium
            .load_pdf_from_file("test/text-test.pdf", None)
            .expect("Could not open test file");

        // ... and return all text in the document.

        pdf.pages()
            .iter()
            .map(|page| page.text().expect("Could not access PdfPageText").all())
            .join("")
    }; // The mutex guard on Pdfium is dropped once the scope closes, unblocking other tasks.

    Ok(text)
}
