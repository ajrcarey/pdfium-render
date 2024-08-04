use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.create_new_pdf()?;

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
        // Adding a built-in font to the document gets us a reusable token...

        let font = document.fonts_mut().new_built_in(built_in);

        // ... that we can then use to retrieve a reference to the font itself.

        let font = document.fonts().get(font).unwrap();

        // At the time of writing, Pdfium does not reliably return font weights,
        // italic angles, and certain other properties correctly for built-in fonts.

        println!(
            "Built-in PDF font {} is built-in {:?}: family = {}, is symbolic? {}, is non-symbolic? {}, ascent {:?}, descent {:?}, number of glyphs: {}, is embedded in document?: {}",
            index,
            built_in,
            font.family(),
            font.is_symbolic(),
            font.is_non_symbolic(),
            font.ascent(font_size),
            font.descent(font_size),
            font.glyphs().len(),
            font.is_embedded()?,
        );
    }

    // At the time of writing, Pdfium does not reliably return font weights,
    // italic angles, and certain other properties correctly for built-in fonts.
    // So let's also output these properties for some fonts embedded into a file.

    let document = pdfium.load_pdf_from_file("test/text-test.pdf", None)?;

    for (page_index, page) in document.pages().iter().enumerate() {
        for (font_index, font) in page.fonts().iter().enumerate() {
            println!(
                "Font {} on page {} is embedded: family = {}, is symbolic? {}, is non-symbolic? {}, ascent {:?}, descent {:?}, number of glyphs: {}, is embedded in document?: {}",
                font_index,
                page_index,
                font.family(),
                font.is_symbolic(),
                font.is_non_symbolic(),
                font.ascent(font_size),
                font.descent(font_size),
                font.glyphs().len(),
                font.is_embedded()?,
            );

            if font.is_embedded()? {
                println!("{} bytes in embedded font data", font.data()?.len());

                // If we wished, we could write the embedded font data out to a file like so:
                // std::fs::write(format!("./{}.ttf", font_index), font.data()?).expect("failed writing font data");
            }
        }
    }

    Ok(())
}
