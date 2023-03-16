use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.create_new_pdf()?;

    // Log characteristics of the 14 built-in PDF fonts to the console.

    let fonts = vec![
        PdfFontBuiltin::TimesRoman,
        PdfFontBuiltin::TimesBold,
        PdfFontBuiltin::TimesItalic,
        PdfFontBuiltin::TimesBoldItalic,
        PdfFontBuiltin::Helvetica,
        PdfFontBuiltin::HelveticaBold,
        PdfFontBuiltin::HelveticaOblique,
        PdfFontBuiltin::HelveticaBoldOblique,
        PdfFontBuiltin::Courier,
        PdfFontBuiltin::CourierBold,
        PdfFontBuiltin::CourierOblique,
        PdfFontBuiltin::CourierBoldOblique,
        PdfFontBuiltin::Symbol,
        PdfFontBuiltin::ZapfDingbats,
    ];

    let font_size = PdfPoints::new(12.0);

    for (index, built_in) in fonts.into_iter().enumerate() {
        let font = PdfFont::new_built_in(&document, built_in);

        // At the time of writing, Pdfium does not reliably return font weights,
        // italic angles, and certain other properties correctly for built-in fonts.

        println!(
            "Built-in PDF font {} is built-in {:?}: name = {}, is symbolic? {}, is non-symbolic? {}, ascent {:?}, descent {:?}, number of glyphs: {}",
            index,
            built_in,
            font.name(),
            font.is_symbolic(),
            font.is_non_symbolic(),
            font.ascent(font_size),
            font.descent(font_size),
            font.glyphs().len()
        );
    }

    // At the time of writing, Pdfium does not reliably return font weights,
    // italic angles, and certain other properties correctly for built-in fonts.
    // So let's also output these properties for some fonts embedded into a file.

    let document = pdfium.load_pdf_from_file("test/form-test.pdf", None)?;

    for (page_index, page) in document.pages().iter().enumerate() {
        for (font_index, font) in page.fonts().iter().enumerate() {
            println!(
            "Font {} on page {} is embedded: name = {}, is symbolic? {}, is non-symbolic? {}, ascent {:?}, descent {:?}, number of glyphs: {}",
            font_index,
            page_index,
            font.name(),
            font.is_symbolic(),
            font.is_non_symbolic(),
            font.ascent(font_size),
            font.descent(font_size),
            font.glyphs().len()
        );
        }
    }

    Ok(())
}
