use image::ImageFormat;
use pdfium_render::bitmap::PdfBitmapRotation;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;

pub fn main() {
    match Pdfium::bind_to_library("./libpdfium.so") {
        Ok(bindings) => {
            let render_config = PdfBitmapConfig::new()
                .set_target_width(2000)
                .set_maximum_height(2000)
                .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

            Pdfium::new(bindings)
                .load_pdf_from_file("test/test.pdf", None)
                .unwrap()
                .pages()
                .for_each(|page| {
                    let result = page
                        .get_bitmap_with_config(&render_config)
                        .unwrap()
                        .as_image()
                        .as_bgra8()
                        .unwrap()
                        .save_with_format(
                            format!("test-page-{}.jpg", page.index()),
                            ImageFormat::Jpeg,
                        );

                    assert!(result.is_ok());
                });
        }
        Err(err) => eprintln!("Error loading pdfium library: {:#?}", err),
    }
}
