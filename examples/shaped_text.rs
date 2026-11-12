use pdfium_render::prelude::*;
use std::fmt::Write;

// Writes text that has already been shaped as real, selectable text.
//
// A text shaper turns characters into glyphs and positions. The result cannot always be
// written with PdfPageTextObject::set_text(), which looks each character up in the font's
// own character map and advances by the font's widths: a ligature is one glyph standing for
// several characters, and shaped positions include kerning the font widths do not.
//
// Instead, this example loads the font with PdfFonts::load_cid_type2_from_bytes(), choosing
// a character ID (CID) for each glyph and supplying the ToUnicode CMap that tells PDF viewers
// which text each CID stands for, then writes the CIDs with set_charcodes() and the shaped
// positions with set_positions().

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.create_new_pdf()?;

    let font_data = std::fs::read("test/Lato-Regular.ttf").unwrap();

    let face = ttf_parser::Face::parse(&font_data, 0).unwrap();

    // The shaped text: "office" with its "fi" drawn as the font's ligature glyph. Each entry
    // is one glyph and the text it stands for. A real application would take these from a
    // text shaper such as HarfBuzz.

    let fi_ligature = face
        .glyph_index_by_name("fi")
        .or_else(|| face.glyph_index_by_name("f_i"))
        .expect("Lato has an fi ligature glyph");

    let glyph = |c: char| face.glyph_index(c).unwrap();

    let shaped = [
        (glyph('o'), "o"),
        (glyph('f'), "f"),
        (fi_ligature, "fi"),
        (glyph('c'), "c"),
        (glyph('e'), "e"),
    ];

    // Give each glyph a CID. CID 0 is reserved for the .notdef glyph.

    let mut cid_to_gid_map = vec![0u8, 0u8];

    let mut to_unicode = String::new();

    let mut charcodes = Vec::new();

    for (index, (glyph_id, text)) in shaped.iter().enumerate() {
        let cid = index as u32 + 1;

        cid_to_gid_map.extend_from_slice(&glyph_id.0.to_be_bytes());

        let utf16: String = text
            .encode_utf16()
            .map(|unit| format!("{:04X}", unit))
            .collect();

        writeln!(to_unicode, "<{:04X}> <{}>", cid, utf16).unwrap();

        charcodes.push(cid);
    }

    let to_unicode_cmap = format!(
        "/CIDInit /ProcSet findresource begin\n\
         12 dict begin\n\
         begincmap\n\
         /CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n\
         /CMapName /Adobe-Identity-UCS def\n\
         /CMapType 2 def\n\
         1 begincodespacerange\n\
         <0000> <FFFF>\n\
         endcodespacerange\n\
         {} beginbfchar\n\
         {}endbfchar\n\
         endcmap\n\
         CMapName currentdict /CMap defineresource pop\n\
         end\n\
         end\n",
        shaped.len(),
        to_unicode
    );

    let font = document.fonts_mut().load_cid_type2_from_bytes(
        &font_data,
        &to_unicode_cmap,
        &cid_to_gid_map,
    )?;

    // Position each glyph after the first along the baseline, in points from the start of
    // the text object. Here the positions come from the font's advance widths plus a little
    // extra tracking, to show that the positions are honoured rather than recomputed.

    let font_size = 36.0;

    let scale = font_size / face.units_per_em() as f32;

    let tracking = 2.0;

    let mut positions = Vec::new();

    let mut x = 0.0;

    for (glyph_id, _) in shaped.iter().take(shaped.len() - 1) {
        x += face.glyph_hor_advance(*glyph_id).unwrap() as f32 * scale + tracking;

        positions.push(x);
    }

    let mut page = document
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    let mut object = PdfPageTextObject::new(&document, " ", font, PdfPoints::new(font_size))?;

    object.set_charcodes(&charcodes)?;

    object.set_positions(&positions)?;

    object.translate(PdfPoints::new(72.0), PdfPoints::new(720.0))?;

    page.objects_mut().add_text_object(object)?;

    document.save_to_file("test/shaped-text-test.pdf")?;

    // Read the document back: text extraction returns the characters, with the ligature
    // glyph expanded back into the two characters it stands for.

    let document = pdfium.load_pdf_from_file("test/shaped-text-test.pdf", None)?;

    let text = document.pages().first()?.text()?.all();

    println!("Extracted text: {}", text);

    assert_eq!(text, "office");

    Ok(())
}
