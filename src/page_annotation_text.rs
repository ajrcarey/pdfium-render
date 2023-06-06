//! Defines the [PdfPageTextAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Text`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE, FPDF_WCHAR};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;

/// A single `PdfPageAnnotation` of type `PdfPageAnnotationType::Text`.
pub struct PdfPageTextAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageTextAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageTextAnnotation {
            handle: annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
                bindings,
            ),
            bindings,
        }
    }

    /// Returns the text associated with this [PdfPageTextAnnotation], if any.
    pub fn text(&self) -> Option<String> {
        // Retrieving the annotation text from Pdfium is a two-step operation. First, we call
        // FPDFAnnot_GetStringValue() with a null buffer; this will retrieve the length of
        // the annotation text in bytes. If the length is zero, then there is no text.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFAnnot_GetStringValue() again with a pointer to the buffer;
        // this will write the annotation text to the buffer in UTF16-LE format.

        let buffer_length = self.bindings.FPDFAnnot_GetStringValue(
            self.handle,
            "Contents",
            std::ptr::null_mut(),
            0,
        );

        if buffer_length == 0 {
            // No text is defined.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFAnnot_GetStringValue(
            self.handle,
            "Contents",
            buffer.as_mut_ptr() as *mut FPDF_WCHAR,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer)
    }

    /// Sets the text associated with this [PdfPageTextAnnotation].
    pub fn set_text(&mut self, text: &str) -> Result<(), PdfiumError> {
        if self
            .bindings()
            .is_true(
                self.bindings()
                    .FPDFAnnot_SetStringValue_str(self.handle, "Contents", text),
            )
        {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageTextAnnotation<'a> {
    #[inline]
    fn handle(&self) -> FPDF_ANNOTATION {
        self.handle
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
