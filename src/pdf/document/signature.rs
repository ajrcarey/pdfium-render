//! Defines the [PdfSignature] struct, exposing functionality related to a single
//! digital signature in a `PdfSignatures` collection.

use crate::bindgen::FPDF_SIGNATURE;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::ffi::{c_uint, CString};
use std::os::raw::{c_char, c_void};

/// The modification detection permission (MDP) applicable to a single digital signature
/// in a `PdfDocument`.
///
/// For more information on MDP, refer to "DocMDP" in Section 8.7.1 on page 731 of
/// The PDF Reference, Sixth Edition. The permission levels in this enumeration
/// correspond to those listed in table 8.104 on page 733.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfSignatureModificationDetectionPermission {
    /// MDP access permission level 1: no changes to the document are permitted;
    /// any change to the document invalidates the signature.
    Mdp1,

    /// MDP access permission level 2: permitted changes are filling in forms,
    /// instantiating page templates, and signing; other changes invalidate the signature.
    Mdp2,

    /// MDP access permission level 3: permitted changes are the same as for level 2,
    /// as well as annotation creation, deletion, and modification; other changes
    /// invalidate the signature.
    Mdp3,
}

impl PdfSignatureModificationDetectionPermission {
    #[inline]
    pub(crate) fn from_pdfium(raw: c_uint) -> Result<Self, PdfiumError> {
        match raw {
            0 => Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            )),
            1 => Ok(PdfSignatureModificationDetectionPermission::Mdp1),
            2 => Ok(PdfSignatureModificationDetectionPermission::Mdp2),
            3 => Ok(PdfSignatureModificationDetectionPermission::Mdp3),
            _ => Err(PdfiumError::UnknownPdfSignatureModificationDetectionPermissionLevel),
        }
    }
}

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
            self.bindings
                .FPDFSignatureObj_GetContents(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // The signature is empty.

            return Vec::new();
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFSignatureObj_GetContents(
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
            self.bindings
                .FPDFSignatureObj_GetReason(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // There is no reason given for this signature.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFSignatureObj_GetReason(
            self.handle,
            buffer.as_mut_ptr() as *mut c_void,
            buffer_length,
        );

        assert_eq!(result, buffer_length);

        get_string_from_pdfium_utf16le_bytes(buffer)
    }

    /// Returns the date, if any, in plain text format as specified by the creator of this [PdfSignature].
    /// The format of the returned value is expected to be `D:YYYYMMDDHHMMSS+XX'YY'`, with precision
    /// to the second and timezone information included.
    ///
    /// This value should only be used if the date of signing is not available in the
    /// PKCS#7 digital signature.
    pub fn signing_date(&self) -> Option<String> {
        // Retrieving the signing date from Pdfium is a two-step operation. First, we call
        // FPDFSignatureObj_GetTime() with a null buffer; this will retrieve the length of
        // the timestamp in bytes. If the length is zero, then there is no timestamp associated
        // with this signature.

        // If the length is non-zero, then we reserve a byte buffer of the given
        // length and call FPDFSignatureObj_GetTime() again with a pointer to the buffer;
        // this will write the timestamp to the buffer as an array of 7-bit ASCII characters.

        let buffer_length =
            self.bindings
                .FPDFSignatureObj_GetTime(self.handle, std::ptr::null_mut(), 0);

        if buffer_length == 0 {
            // There is no timestamp given for this signature.

            return None;
        }

        let mut buffer = create_byte_buffer(buffer_length as usize);

        let result = self.bindings.FPDFSignatureObj_GetTime(
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

    /// Returns the modification detection permission (MDP) applicable to this [PdfSignature],
    /// if available.
    ///
    /// For more information on MDP, refer to "DocMDP" in Section 8.7.1 on page 731 of
    /// The PDF Reference, Sixth Edition.
    pub fn modification_detection_permission(
        &self,
    ) -> Result<PdfSignatureModificationDetectionPermission, PdfiumError> {
        PdfSignatureModificationDetectionPermission::from_pdfium(
            self.bindings
                .FPDFSignatureObj_GetDocMDPPermission(self.handle),
        )
    }
}
