use image::ImageFormat;
use pdfium_render::bitmap::PdfBitmapRotation;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;

pub fn main() {
    // Attempt to bind to a pdfium library in the current working directory; failing that,
    // attempt to bind to the system-provided library.

    // The library name will differ depending on the current platform. On Linux,
    // the library will be named libpdfium.so by default; on Windows, pdfium.dll; and on
    // MacOS, libpdfium.dylib. We can use the Pdfium::pdfium_platform_library_name_at_path()
    // function to append the correct library name for the current platform to a path we specify.

    let bindings = Pdfium::bind_to_library(
        // Attempt to bind to a pdfium library in the current working directory...
        Pdfium::pdfium_platform_library_name_at_path("./"),
    )
        .or_else(
            // ... and fall back to binding to a system-provided pdfium library.
            |_| Pdfium::bind_to_system_library(),
        );

    match bindings {
        Ok(bindings) => {
            // The code below simply unwraps every Result<> returned from Pdfium.
            // In production code, you would actually want to check the results, rather
            // than just unwrapping them :)

            // First, create a set of shared settings that we'll apply to each page in the
            // sample file when rendering. Sharing the same rendering configuration is a good way
            // to ensure homogenous output across all pages in the document.

            let dpi = 200.0;
            let scale = dpi as f32 / 72.0;

            let render_config = PdfBitmapConfig::new()
                .scale_page_by_factor(scale);

            let pdfium = Pdfium::new(bindings);
            let mut doc = pdfium
                .load_pdf_from_file("test/form.pdf", None) // Load the sample file...
                .unwrap();

            let handle = doc.load_form_data();

            doc.pages() // ... get an iterator across all pages ...
                .for_each(|page| {
                    // ... and export each page to a JPEG in the current working directory,
                    // using the rendering configuration we created earlier.

                    let mut bitmap = page
                        .get_bitmap_with_config(&render_config, Some(handle)) // Initializes a bitmap with the given configuration for this page ...
                        .unwrap();

                    let result = bitmap.as_image() // ... renders it to an Image::DynamicImage ...
                        .as_bgra8() // ... sets the correct color space ...
                        .unwrap()
                        .save_with_format(
                            format!("test-page-{}.jpg", page.index()),
                            ImageFormat::Jpeg,
                        ); // ... and exports it to a JPEG.

                    assert!(result.is_ok());
                });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
