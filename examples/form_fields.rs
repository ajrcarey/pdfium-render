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

        for (annotation_index, annotation) in annotations.iter().enumerate() {
            // The PdfPageAnnotation::as_form_field() helper function handles the filtering out
            // of non-form-field-wrapping annotations for us.

            if let Some(field) = annotation.as_form_field() {
                println!(
                    "Page {}, annotation {}: {:?}, {:?} has form field value: {}",
                    page_index,
                    annotation_index,
                    field.field_type(),
                    field.name(),
                    // The field value depends on the field type.
                    if let Some(checkbox) = field.as_checkbox_field() {
                        checkbox.is_checked()?.to_string()
                    } else if let Some(radio) = field.as_radio_button_field() {
                        radio.is_checked()?.to_string()
                    } else if let Some(text) = field.as_text_field() {
                        format!("{:?}", text.value())
                    } else if let Some(combo) = field.as_combo_box_field() {
                        format!(
                            "{:?} [out of {} available options]",
                            combo.value(),
                            combo.options().len()
                        )
                    } else if let Some(list) = field.as_list_box_field() {
                        format!(
                            "{:?} [out of {} available options]",
                            list.value(),
                            list.options().len()
                        )
                    } else {
                        "None".to_string()
                    }
                );
            }
        }
    }

    // Alternatively, we can use the PdfForm::field_values() function to capture string
    // representations of all values for us in a single function call. This saves us
    // iterating over the annotations ourselves, but at the cost of reduced flexibility.

    if let Some(form) = document.form() {
        for (key, value) in form.field_values(&pages).iter() {
            println!("{:?} => {:?}", key, value);
        }
    }

    Ok(())
}
