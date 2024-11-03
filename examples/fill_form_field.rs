use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_file("test/form-test.pdf", None)?;

    match document.form() {
        Some(form) => println!(
            "PDF contains an embedded form of type {:#?}",
            form.form_type()
        ),
        None => println!("PDF does not contain an embedded form"),
    };

    // Form fields in Pdfium are wrapped within page annotation objects, specifically annotations
    // of type `PdfPageAnnotationType::Widget` or `PdfPageAnnotationType::XfaWidget` (depending on
    // the type of form embedded in the document). To retrieve the form field values, we iterate
    // over each annotation on each page in the document, examining just the annotations capable of
    // wrapping a form field.

    let pages = document.pages();

    for (page_index, page) in pages.iter().enumerate() {
        let annotations = page.annotations();

        for (annotation_index, mut annotation) in annotations.iter().enumerate() {
            // The PdfPageAnnotation::as_form_field() helper function handles the filtering out
            // of non-form-field-wrapping annotations for us.

            if let Some(field) = annotation.as_form_field_mut() {
                // If this field is a fillable form field...

                // TODO: AJRC - 13/6/24 - radio button and checkbox implementations in progress
                // as part of https://github.com/ajrcarey/pdfium-render/issues/132.

                if let Some(field) = field.as_radio_button_field_mut() {
                    // ... select its radio button.

                    println!(
                        "Page {}, radio button {}: {:?} currently has value: {}",
                        page_index,
                        annotation_index,
                        field.name(),
                        format!("{:?}", field.is_checked()),
                    );

                    field.set_checked()?;

                    println!(
                        "Page {}, radio button {}: {:?} now has updated value: {}",
                        page_index,
                        annotation_index,
                        field.name(),
                        format!("{:?}", field.is_checked()),
                    );
                } else if let Some(field) = field.as_checkbox_field_mut() {
                    // ... check its checkbox.

                    println!(
                        "Page {}, checkbox {}: {:?} currently has value: {}",
                        page_index,
                        annotation_index,
                        field.name(),
                        format!("{:?}", field.is_checked()),
                    );

                    field.set_checked(true)?;

                    println!(
                        "Page {}, checkbox {}: {:?} now has updated value: {}",
                        page_index,
                        annotation_index,
                        field.name(),
                        format!("{:?}", field.is_checked()),
                    );
                } else if let Some(field) = field.as_text_field_mut() {
                    // ... set its value to the field's internal name.

                    println!(
                        "Page {}, text field {}: {:?} currently has value: {}",
                        page_index,
                        annotation_index,
                        field.name(),
                        format!("{:?}", field.value()),
                    );

                    field.set_value(
                        field
                            .name()
                            .unwrap_or_else(|| format!("field-{}-{}", page_index, annotation_index))
                            .as_str(),
                    )?;

                    println!(
                        "Page {}, text field {}: {:?} now has updated value: {}",
                        page_index,
                        annotation_index,
                        field.name(),
                        format!("{:?}", field.value()),
                    );
                }
            }
        }
    }

    document.save_to_file("test/fill-form-test.pdf")
}
