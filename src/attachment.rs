//! Defines the [PdfAttachment] struct, exposing functionality related to a single
//! attachment in a `PdfAttachments` collection.

use crate::bindgen::{FPDF_ATTACHMENT, FPDF_WCHAR};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::utils::mem::create_byte_buffer;
use crate::utils::utf16le::get_string_from_pdfium_utf16le_bytes;
use std::io::Write;
use std::os::raw::{c_ulong, c_void};

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Uint8Array};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use web_sys::Blob;

// The following dummy declaration is used only when running cargo doc.
// It allows documentation of WASM-specific functionality to be included
// in documentation generated on non-WASM targets.

#[cfg(doc)]
struct Blob;

/// A single attached data file embedded in a `PdfDocument`.
pub struct PdfAttachment<'a> {
    handle: FPDF_ATTACHMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfAttachment<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        handle: FPDF_ATTACHMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfAttachment { handle, bindings }
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

    /// Returns the size of this [PdfAttachment] in bytes.
    pub fn len(&self) -> usize {
        // Calling FPDFAttachment_GetFile() with a null buffer will retrieve the length of the
        // data in bytes without allocating any additional memory.

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
            out_buflen as usize
        } else {
            0
        }
    }

    /// Returns `true` if there is no byte data associated with this [PdfAttachment].
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Writes this [PdfAttachment] to a new byte buffer, returning the byte buffer.
    pub fn save_to_bytes(&self) -> Result<Vec<u8>, PdfiumError> {
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

            Ok(buffer)
        } else {
            Err(PdfiumError::NoDataInAttachment)
        }
    }

    /// Writes this [PdfAttachment] to the given writer.
    pub fn save_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), PdfiumError> {
        self.save_to_bytes().and_then(|bytes| {
            writer
                .write_all(bytes.as_slice())
                .map_err(PdfiumError::IoError)
        })
    }

    /// Writes this [PdfAttachment] to the file at the given path.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// saving attachment data in WASM:
    /// * Use either the [PdfAttachment::save_to_writer()] or the [PdfAttachment::save_to_bytes()] functions,
    ///   both of which are available when compiling to WASM.
    /// * Use the [PdfAttachment::save_to_blob()] function to save attachment data directly into a new
    ///   Javascript `Blob` object. This function is only available when compiling to WASM.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_file(&self, path: &(impl AsRef<Path> + ?Sized)) -> Result<(), PdfiumError> {
        self.save_to_writer(&mut File::create(path).map_err(PdfiumError::IoError)?)
    }

    /// Writes this [PdfAttachment] to a new `Blob`, returning the `Blob`.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub fn save_to_blob(&self) -> Result<Blob, PdfiumError> {
        let bytes = self.save_to_bytes()?;

        let array = Uint8Array::new_with_length(bytes.len() as u32);

        array.copy_from(bytes.as_slice());

        let blob =
            Blob::new_with_u8_array_sequence(&JsValue::from(Array::of1(&JsValue::from(array))))
                .map_err(|_| PdfiumError::JsSysErrorConstructingBlobFromBytes)?;

        Ok(blob)
    }
}
