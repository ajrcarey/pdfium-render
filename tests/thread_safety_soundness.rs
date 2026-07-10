//! Reproduces the unsoundness of the default `thread_safe` feature.
//!
//! With `thread_safe` (a default feature) `Pdfium` is `unsafe impl Send + Sync`,
//! yet there is no serialization of pdfium's non-reentrant C API. This test is
//! 100% safe Rust (no `unsafe` block) but shares one `Pdfium` across threads
//! and renders concurrently, which the `Send + Sync` bounds permit. On a sound
//! implementation it completes; on current master it races pdfium's C state and
//! crashes (SIGSEGV / heap corruption / abort).

use pdfium_render::prelude::*;
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_render_shared_pdfium_races_pdfium_c_state() {
    let pdfium = Arc::new(Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .expect("bind pdfium"),
    ));

    let num_threads = 8;
    let iters = 200;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pdfium = Arc::clone(&pdfium);
            thread::spawn(move || {
                let config = PdfRenderConfig::new().set_target_width(300);
                for _ in 0..iters {
                    let doc = pdfium
                        .load_pdf_from_file("test/export-test.pdf", None)
                        .expect("load document");
                    for page in doc.pages().iter() {
                        let _bitmap = page.render_with_config(&config).expect("render page");
                    }
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("worker thread panicked");
    }
}
