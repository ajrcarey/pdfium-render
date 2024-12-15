// [dependencies]
// axum = "0.7.9"
// tokio = { version = "1.41.1", features = ["full"] }
// tower = "0.5.1"
// pdfium-render = { version = "0.8.26", features = ["sync"] }


use axum::{
    routing::post,
    http::StatusCode,
    Router
};
use pdfium_render::prelude::Pdfium;
use tokio::sync::{Mutex, OnceCell};

static PDFIUM: OnceCell<Mutex<Pdfium>> = OnceCell::const_new();

async fn init_pdfium() -> Mutex<Pdfium> {
    let pdfium = Pdfium::new(match Pdfium::bind_to_system_library() {
        Ok(p) => p,
        Err(e) => {
            panic!("Failed to init Pdfium | Error: {}", e)
        }
    });

    Mutex::new(pdfium)
}

#[tokio::main]
async fn main() {
    // init pdfium here
    PDFIUM.get_or_init(init_pdfium).await;

    // build our application with a route
    let app = Router::new()
        .route("/foos", post(test_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test_post() -> StatusCode {
    let pdfium = PDFIUM.get().expect("Could not get PDFIUM from OnceCell");

    let bytes: Vec<u8> = vec![];
    let pdfium = pdfium.lock().await;
    let pdf = pdfium.load_pdf_from_byte_vec(bytes, None).unwrap();

    // Do whatever with the PDF or pdfium

    // SCOPING PDFIUM: You may want to scope access to PDFIUM that way the mutex guard is dropped asap
    let pdf_text = {
        let pdfium = PDFIUM.get().expect("Could not get PDFIUM from OnceCell");

        // some PDF bytes
        let bytes: Vec<u8> = vec![];
        let pdfium = pdfium.lock().await;
        let pdf = pdfium.load_pdf_from_byte_vec(bytes, None).unwrap();

        let pages = pdf.pages();
        let mut pages_text = vec![];

        for (i, p) in pages.iter().enumerate() {
            let text = p.text().unwrap().to_string();
            pages_text.insert(i, text);
        }

        pages_text
    }; // mutex guard dropped

    StatusCode::OK
}