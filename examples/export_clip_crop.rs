use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // Renders a subset of a page to a file, clipping and cropping the rendering
    // surface. Specifically, we want to output just the area of the target page
    // delineated by the red rectangle.

    // The rectangle is the border outline of a square annotation. Our strategy is
    // to find this annotation, measure its bounding box, convert that bounding box
    // to pixels, clip rendering of the page to that pixel area, and crop the resulting
    // bitmap to that pixel area before saving it to a file.

    let document = pdfium.load_pdf_from_file("test/export-clip-crop-test.pdf", None)?;

    for (page_index, page) in document.pages().iter().enumerate() {
        for (annotation_index, annotation) in page.annotations().iter().enumerate() {
            if annotation.annotation_type() == PdfPageAnnotationType::Square {
                // This is the target annotation. We want to render everything within
                // the bounds of this annotation, and nothing outside the bounds of
                // this annotation.

                let bounds = annotation.bounds()?;

                // Determine the pixel bounds of the square annotation, based on
                // our output target rendering size, and specify this pixel bounds area
                // as the clipping rectangle.

                let config = PdfRenderConfig::new().set_target_width(2000);

                let (clip_left, clip_top) =
                    page.points_to_pixels(bounds.left, bounds.top, &config)?;

                let (clip_right, clip_bottom) =
                    page.points_to_pixels(bounds.right, bounds.bottom, &config)?;

                // Render the portion of the page within the clipping rectangle...

                let bitmap = page.render_with_config(&config.clip(
                    clip_left,
                    clip_top,
                    clip_right,
                    clip_bottom,
                ))?;

                // ... and save it to a JPG file, cropping out everything outside
                // the clipping rectangle.

                bitmap
                    .as_image()
                    .crop(
                        // Crop the output image to the clipping rectangle.
                        clip_left as u32,
                        clip_top as u32,
                        (clip_right - clip_left) as u32,
                        (clip_bottom - clip_top) as u32,
                    )
                    .as_rgba8()
                    .ok_or(PdfiumError::ImageError)?
                    .save_with_format(
                        format!(
                            "export-clip-crop-test-{}-{}.jpg",
                            page_index, annotation_index
                        ),
                        image::ImageFormat::Jpeg,
                    )
                    .map_err(|_| PdfiumError::ImageError)?;
            }
        }
    }

    Ok(())
}
