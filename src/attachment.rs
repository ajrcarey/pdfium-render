//! Defines the [PdfAttachment] struct, exposing functionality related to a single
//! attachment in a `PdfAttachments` collection.

use crate::bindgen::{FPDF_ATTACHMENT, FPDF_DOCUMENT, FPDF_WCHAR};
use crate::bindings::PdfiumLibraryBindings;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::os::raw::{c_ulong, c_void};

/// A single attached data file embedded in a `PdfDocument`.
pub struct PdfAttachment<'a> {
    handle: FPDF_ATTACHMENT,
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfAttachment<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ATTACHMENT,
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfAttachment {
            handle,
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfAttachment].
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the name of this [PdfAttachment].
    pub fn name(&self) -> String {
        // Retrieving the attachment name from Pdfium is a two-step operation. First, we call
        // FPDFAttachment_GetName() with a null buffer; this will retrieve the length of
        // the name in bytes. If the length is zero, then there is no name associated
        // with this attachment.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFAttachment_GetName() again with a pointer to the buffer;
        // this will write the name to the buffer in UTF16-LE format.

        let buffer_length =
            self.bindings()
                .FPDFAttachment_GetName(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // There is no name given for this attachment.

            return String::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings().FPDFAttachment_GetName(
            self.handle,
            buffer.as_mut_ptr() as *mut FPDF_WCHAR,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer).unwrap_or_default()
    }

    /// Returns the byte data for this [PdfAttachment].
    pub fn bytes(&self) -> Vec<u8> {
        // Retrieving the attachment data from Pdfium is a two-step operation. First, we call
        // FPDFAttachment_GetFile() with a null buffer; this will retrieve the length of
        // the data in bytes. If the length is zero, then there is no data associated
        // with this attachment. (This can be the case if the attachment is newly created,
        // and data for the attachment is yet to be embedded in the containing document.)

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFAttachment_GetFile() again with a pointer to the buffer;
        // this will write the file data to the buffer.

        let mut out_buflen: c_ulong = 0;

        if self
            .bindings()
            .is_true(self.bindings().FPDFAttachment_GetFile(
                self.handle,
                std::ptr::null_mut(),
                0,
                &mut out_buflen,
            ))
        {
            // out_buflen now contains the length of the file data.

            let buffer_length = out_buflen;

            let mut buffer = create_byte_buffer(buffer_length as usize);

            let result = self.bindings().FPDFAttachment_GetFile(
                self.handle,
                buffer.as_mut_ptr() as *mut c_void,
                buffer_length,
                &mut out_buflen,
            );

            assert!(self.bindings.is_true(result));
            assert_eq!(buffer_length, out_buflen);

            buffer
        } else {
            // There is no file data for this attachment.

            Vec::new()
        }
    }
}
