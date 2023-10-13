use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let search_term = "French";

    let search_options = PdfSearchOptions::new()
        // Experiment with how the search results change when uncommenting
        // the following search options.

        // .match_whole_word(true)
        // .match_case(true)
        ;

    // Find the position of all occurrences of the search term
    // on the first page of the target document.

    let pdfium = Pdfium::default();

    let mut document = pdfium.load_pdf_from_file("test/text-test.pdf", None)?;

    let mut page = document.pages().first()?;

    let mut search_results_bounds = page
        .text()?
        .search(search_term, &search_options)
        .iter(PdfSearchDirection::SearchForward)
        .enumerate()
        .flat_map(|(index, segments)| {
            segments
                .iter()
                .map(|segment| {
                    println!(
                        "Search result {}: `{}` appears at {:#?}",
                        index,
                        segment.text(),
                        segment.bounds()
                    );

                    segment.bounds()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // We now have a list of page areas that contain the search target.
    // Highlight them in yellow...

    for result in search_results_bounds.drain(..) {
        page.objects_mut().create_path_object_rect(
            result,
            None,
            None,
            Some(PdfColor::YELLOW.with_alpha(128)),
        )?;
    }

    // ... and save the result out to a new document.

    while document.pages().len() > 1 {
        document.pages_mut().last()?.delete()?;
    }

    document.save_to_file("test/search-results-test.pdf")?;

    Ok(())
}
