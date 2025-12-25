use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // Configure platform-specific font search paths
    #[cfg(target_os = "linux")]
    let font_paths = vec![
        "/usr/share/fonts/truetype/".to_string(),
        "/usr/local/share/fonts/".to_string(),
    ];

    #[cfg(target_os = "macos")]
    let font_paths = vec![
        "/Library/Fonts/".to_string(),
        "/System/Library/Fonts/".to_string(),
    ];

    #[cfg(target_os = "windows")]
    let font_paths = vec![
        "C:\\Windows\\Fonts\\".to_string(),
    ];

    // Initialize Pdfium with custom font paths
    let config = PdfiumConfig::new().set_user_font_paths(font_paths);

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library())?;

    let pdfium = Pdfium::new_with_config(bindings, &config);

    // Create a new PDF document
    let mut document = pdfium.create_new_pdf()?;

    // Create an A4 page
    let mut page = document
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    // Add text using built-in Helvetica font
    let font = document.fonts_mut().helvetica();

    page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(700.0),
        "Custom Font Paths Demo",
        font,
        PdfPoints::new(24.0),
    )?;

    // Add smaller text with system fonts available
    let font_bold = document.fonts_mut().helvetica_bold();

    page.objects_mut().create_text_object(
        PdfPoints::new(100.0),
        PdfPoints::new(650.0),
        "This PDF was created with custom font search paths configured.",
        font_bold,
        PdfPoints::new(12.0),
    )?;

    // Save the document
    document.save_to_file("test/font-config-test.pdf")?;

    println!("Success! Created PDF with custom font paths at test/font-config-test.pdf");
    println!("Document has {} page(s)", document.pages().len());

    Ok(())
}
