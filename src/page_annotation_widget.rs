//! Defines the [PdfPageWidgetAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Widget`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::form_field::PdfFormField;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

pub struct PdfPageWidgetAnnotation<'a> {
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
    objects: PdfPageAnnotationObjects<'a>,
    form_field: Option<PdfFormField<'a>>,
}

impl<'a> PdfPageWidgetAnnotation<'a> {
    pub(crate) fn from_pdfium(
        annotation_handle: FPDF_ANNOTATION,
        page_handle: FPDF_PAGE,
        document: &'a PdfDocument<'a>,
    ) -> Self {
        PdfPageWidgetAnnotation {
            annotation_handle,
            bindings: document.bindings(),
            objects: PdfPageAnnotationObjects::from_pdfium(
                *document.handle(),
                page_handle,
                annotation_handle,
                document.bindings(),
            ),
            form_field: document.form().and_then(|form| {
                PdfFormField::from_pdfium(*form.handle(), annotation_handle, document.bindings())
            }),
        }
    }

    /// Returns the [PdfFormField] wrapped by this [PdfPageWidgetAnnotation], if any.
    #[inline]
    pub fn form_field(&self) -> Option<&PdfFormField> {
        self.form_field.as_ref()
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageWidgetAnnotation<'a> {
    #[inline]
    fn handle(&self) -> &FPDF_ANNOTATION {
        &self.annotation_handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn objects_impl(&self) -> &PdfPageAnnotationObjects {
        &self.objects
    }

    #[inline]
    fn objects_mut_impl(&mut self) -> &mut PdfPageAnnotationObjects<'a> {
        &mut self.objects
    }
}

// impl<'a> PdfFormFieldPrivate<'a> for PdfPageWidgetAnnotation<'a> {
//     #[inline]
//     fn form_handle(&self) -> &FPDF_FORMHANDLE {
//         self.form_handle.as_ref().unwrap()
//     }
//
//     #[inline]
//     fn annotation_handle(&self) -> &FPDF_ANNOTATION {
//         &self.annotation_handle
//     }
//
//     #[inline]
//     fn bindings(&self) -> &dyn PdfiumLibraryBindings {
//         self.bindings
//     }
// }
//
// impl<'a> PdfFormField for PdfPageWidgetAnnotation<'a> {
//     fn field_type(&self) -> PdfFormFieldType {
//         todo!()
//     }
//
//     fn as_push_button_field(&self) -> Option<&PdfFormPushButtonField> {
//         todo!()
//     }
//
//     fn as_checkbox_field(&self) -> Option<&PdfFormCheckboxField> {
//         todo!()
//     }
//
//     fn as_radio_button_field(&self) -> Option<&PdfFormRadioButtonField> {
//         todo!()
//     }
//
//     fn as_combo_box_field(&self) -> Option<&PdfFormComboBoxField> {
//         todo!()
//     }
//
//     fn as_list_box_field(&self) -> Option<&PdfFormListBoxField> {
//         todo!()
//     }
//
//     fn as_signature_field(&self) -> Option<&PdfFormSignatureField> {
//         todo!()
//     }
//
//     fn as_text_field(&self) -> Option<&PdfFormTextField> {
//         todo!()
//     }
//
//     fn as_unknown_field(&self) -> Option<&PdfFormUnknownField> {
//         todo!()
//     }
// }
