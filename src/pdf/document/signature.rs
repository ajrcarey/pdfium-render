//! Defines the [PdfSignature] struct, exposing functionality related to a single
//! digital signature in a `PdfSignatures` collection.

use crate::bindgen::FPDF_SIGNATURE;
use crate::bindings::PdfiumLibraryBindings;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

/// A single digital signature in a `PdfDocument`.
pub struct PdfSignature<'a> {
    handle: FPDF_SIGNATURE,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfSignature<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_SIGNATURE,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfSignature { handle, bindings }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfSignature].
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the raw byte data for this [PdfSignature].
    ///
    /// For public key signatures, the byte data is either a DER-encoded PKCS#1 binary or
    /// a DER-encoded PKCS#7 binary.
    pub fn bytes(&self) -> Vec<u8> {
        // Retrieving the byte data from Pdfium is a two-step operation. First, we call
        // FPDFSignatureObj_GetContents() with a null buffer; this will retrieve the length of
        // the reason text in bytes. If the length is zero, then there is no reason associated
        // with this signature.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFSignatureObj_GetContents() again with a pointer to the buffer;
        // this will write the reason text to the buffer in UTF16-LE format.

        let buffer_length =
            self.bindings()
                .FPDFSignatureObj_GetContents(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // The signature is empty.

            return Vec::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings().FPDFSignatureObj_GetContents(
            self.handle,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        buffer
    }

    /// Returns the reason for the signing, if any, as a plain text description provided by the
    /// creator of this [PdfSignature].
    pub fn reason(&self) -> Option<String> {
        // Retrieving the reason from Pdfium is a two-step operation. First, we call
        // FPDFSignatureObj_GetReason() with a null buffer; this will retrieve the length of
        // the reason text in bytes. If the length is zero, then there is no reason associated
        // with this signature.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFSignatureObj_GetReason() again with a pointer to the buffer;
        // this will write the reason text to the buffer in UTF16-LE format.

        let buffer_length =
            self.bindings()
                .FPDFSignatureObj_GetReason(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // There is no reason given for this signature.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings().FPDFSignatureObj_GetReason(
            self.handle,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer)
    }

    /// Returns the date, if any, in plain text format as specified by the creator of this [PdfSignature].
    /// The format of the returned value is expected to be D:YYYYMMDDHHMMSS+XX'YY', with precision
    /// to the second and including timezone information.
    ///
    /// This value should only be used if the date of signing is not encoded into the digital signature itself.
    pub fn signing_date(&self) -> Option<String> {
        // Retrieving the signing date from Pdfium is a two-step operation. First, we call
        // FPDFSignatureObj_GetTime() with a null buffer; this will retrieve the length of
        // the timestamp in bytes. If the length is zero, then there is no timestamp associated
        // with this signature.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFSignatureObj_GetTime() again with a pointer to the buffer;
        // this will write the timestamp to the buffer as an array of 7-bit ASCII characters.

        let buffer_length =
            self.bindings()
                .FPDFSignatureObj_GetTime(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // There is no timestamp given for this signature.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings().FPDFSignatureObj_GetTime(
            self.handle,
            buffer.as_mut_ptr() as *mut c_char,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        if let Ok(result) = CString::from_vec_with_nul(buffer) {
            result.into_string().ok()
        } else {
            None
        }
    }
}
