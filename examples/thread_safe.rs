use image::ImageFormat;
use pdfium_render::prelude::*;
use rayon::prelude::*;
use std::thread;

// A demonstration of thread-safe use of Pdfium in a multi-threaded context.
//
// Pdfium makes no guarantees about thread safety and should be assumed _not_ to be thread safe.
// The Pdfium authors specifically recommend that parallel processing, not multi-threading,
// be used to process multiple documents simultaneously. pdfium-render achieves thread safety by
// locking access to Pdfium behind a mutex; each thread must acquire exclusive access to this
// mutex in order to make any call to Pdfium. This has the effect of sequencing all calls to
// Pdfium as if they were single-threaded, even when using pdfium-render from multiple threads.
//
// This example demonstrates this behaviour. A set of source files are rendered using a parallel
// iterator. In theory, each rendering task should be processed simultaneously using one thread
// per task; in practice, pdfium-render will block the threads to ensure Pdfium is only ever
// accessed safely by a single thread at a time. As a result, the processing of the files sees
// no performance benefit, but at least it doesn't crash the application!
//
// This example must be compiled with pdfium-render's thread_safe feature enabled. At the time
// of writing the thread_safe feature was enabled by default; it can also be specified manually
// when compiling this example:

// cargo run --example thread_safe --features="thread_safe"

// (Without using pdfium-render's thread_safe feature, Pdfium would randomly segfault
// on simultaneous memory access from different threads.)
//
// I emphasise again that this approach _offers no performance benefit_. The thread_safe feature
// simply ensures that pdfium-render will not crash when running as part of a multi-threaded
// application. To see an actual performance benefit, you _must_ use parallel processing,
// _not_ multi-threading.

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    // Define bitmap rendering settings that will be used by all threads.

    let config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // Launch separate, simultaneous rendering tasks on different threads using rayon::par_iter().
    // In theory, all tasks should execute concurrently; in practice, pdfium-render will force
    // threads to block in order to maintain thread-safe access to Pdfium.

    vec![
        "test/export-test.pdf",
        "test/form-test.pdf",
        "test/create-test.pdf",
        "test/path-test.pdf",
    ]
    .par_iter() // rayon will spawn a separate thread for each task
    .for_each(|path| assert!(render(&config, path).is_ok()));

    println!("All done!");

    Ok(())
}

fn render(render_config: &PdfRenderConfig, path: &str) -> Result<(), PdfiumError> {
    // Render each page in the document at the given path out to a JPG file, using the
    // given bindings and rendering configuration.

    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_file(path, None)?;

    println!(
        "{:?}: {} is version {:?}",
        thread::current().id(),
        path,
        document.version()
    );
    println!(
        "{:?}: {} pages in {}",
        thread::current().id(),
        document.pages().len(),
        path
    );

    for (index, page) in document.pages().iter().enumerate() {
        println!(
            "{:?}: Rendering page {} of {}",
            thread::current().id(),
            index,
            path
        );

        page.render_with_config(render_config)?
            .as_image()
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(format!("./{}-{}.jpg", path, index), ImageFormat::Jpeg)
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}
