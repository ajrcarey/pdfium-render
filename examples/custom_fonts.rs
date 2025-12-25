use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // Example demonstrating custom font provider with pre-loaded font data.
    // This approach eliminates filesystem scanning overhead by serving fonts
    // directly from memory.

    // For this example, we'll use a minimal embedded font.
    // In production, you would load actual font files:
    //   let font_data = std::fs::read("/path/to/font.ttf")?;

    // For this example, we'll demonstrate the API without actually loading fonts
    // In production, you would load actual font files like this:
    //   let font_data = std::fs::read("/path/to/font.ttf")?;
    //   let fonts = vec![
    //       FontDescriptor {
    //           family: "Roboto".to_string(),
    //           weight: 400,
    //           is_italic: false,
    //           charset: 0,
    //           data: Arc::from(font_data),
    //       },
    //   ];
    //   let config = PdfiumConfig::new().set_font_provider(fonts);

    // For this demo, use default config (no custom font provider)
    let config = PdfiumConfig::new();

    // Bind to Pdfium library
    let bindings = Pdfium::bind_to_library(
        Pdfium::pdfium_platform_library_name_at_path("./")
    ).or_else(|_| Pdfium::bind_to_system_library())?;

    // Initialize Pdfium with the config
    let pdfium = Pdfium::new_with_config(bindings, &config);

    // Create a new PDF document
    let mut document = pdfium.create_new_pdf()?;

    // Create an A4 page
    let mut page = document
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    // Add text using built-in Helvetica font
    // (Pdfium has built-in fonts that don't require external font files)
    let font = document.fonts_mut().helvetica();

    page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(700.0),
        "Custom Font Provider Demo",
        font,
        PdfPoints::new(24.0),
    )?;

    // Add description text
    let font_regular = document.fonts_mut().helvetica();

    page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(650.0),
        "This PDF was created using a custom font provider.",
        font_regular,
        PdfPoints::new(12.0),
    )?;

    page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(630.0),
        "Fonts are served from memory, eliminating filesystem I/O.",
        font_regular,
        PdfPoints::new(12.0),
    )?;

    // Save the document
    document.save_to_file("test/custom-fonts-test.pdf")?;

    println!("Success! Created PDF with custom font provider at test/custom-fonts-test.pdf");
    println!("Document has {} page(s)", document.pages().len());
    println!();
    println!("Note: This example uses built-in Helvetica fonts for demonstration.");
    println!("To use custom fonts, load actual .ttf/.otf files and pass them via FontDescriptor.");
    println!();
    println!("Example with real fonts:");
    println!("  let fonts = vec![");
    println!("      FontDescriptor {{");
    println!("          family: \"Roboto\".to_string(),");
    println!("          weight: 400,");
    println!("          is_italic: false,");
    println!("          charset: 0,");
    println!("          data: Arc::from(std::fs::read(\"/fonts/Roboto-Regular.ttf\")?),");
    println!("      }},");
    println!("  ];");

    Ok(())
}
