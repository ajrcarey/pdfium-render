use image::ImageFormat;
use pdfium_render::prelude::*;
#[allow(unused_imports)] // rayon will be used when we switch to parallel iteration
use rayon::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // A demonstration of multi-threaded rendering (or, rather, Pdfium's inability to safely provide
    // multi-threaded rendering). To run this code, the definition of the PdfiumLibraryBindings
    // in src/bindings.rs must be changed to:

    // pub trait PdfiumLibraryBindings: Send + Sync {

    // At the time of writing, this was on line 41 of src/bindings.rs.

    // Create a single set of bindings that are shared across multiple threads. We definitely
    // shouldn't create separate bindings in each thread, because when the bindings object is dropped
    // at the exit of the thread it will call FPDF_DestroyLibrary(), poisoning the bindings of any
    // thread still running.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // Set shared rendering settings that we'll apply to every page in every source file.

    let render_config = PdfBitmapConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

    // Launch separate, simultaneous rendering jobs on different threads using rayon::par_iter().

    vec![
        "test/export-test.pdf",
        "test/form-test.pdf",
        "test/create-test.pdf",
        "test/path-test.pdf",
    ]
    .iter() // Change this to .par_iter() to make Pdfium segfault
    .for_each(|path| assert!(render(&pdfium, &render_config, path).is_ok()));

    println!("All done!");

    Ok(())
}

fn render(pdfium: &Pdfium, render_config: &PdfBitmapConfig, path: &str) -> Result<(), PdfiumError> {
    // Render each page in the document at the given path out to a JPG file, using the
    // given bindings and rendering configuration.

    let document = pdfium.load_pdf_from_file(path, None)?;

    println!("{} is version {:?}", path, document.version());
    println!("{} pages in {}", document.pages().len(), path);

    for (index, page) in document.pages().iter().enumerate() {
        println!("Rendering page {} of {}", index, path);

        page.get_bitmap_with_config(render_config)?
            .as_image()
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(format!("{}-{}.jpg", path, index), ImageFormat::Jpeg)
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}
