//! Defines the [PdfAttachments] struct, a collection of all the `PdfAttachment` objects in a
//! `PdfDocument`.

use crate::bindgen::FPDF_DOCUMENT;
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::attachment::PdfAttachment;
use std::io::Read;
use std::ops::{Range, RangeInclusive};
use std::os::raw::{c_int, c_ulong, c_void};

#[cfg(not(target_arch = "wasm32"))]
use {std::fs::File, std::path::Path};

#[cfg(target_arch = "wasm32")]
use {
    js_sys::{ArrayBuffer, Uint8Array},
    wasm_bindgen::JsCast,
    wasm_bindgen_futures::JsFuture,
    web_sys::{window, Blob, Response},
};

// The following dummy declaration is used only when running cargo doc.
// It allows documentation of WASM-specific functionality to be included
// in documentation generated on non-WASM targets.

#[cfg(doc)]
struct Blob;

pub type PdfAttachmentIndex = u16;

/// The collection of [PdfAttachment] objects embedded in a `PdfDocument`.
pub struct PdfAttachments<'a> {
    document_handle: FPDF_DOCUMENT,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfAttachments<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfAttachments {
            document_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfAttachments] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the number of attachments in this [PdfAttachments] collection.
    pub fn len(&self) -> PdfAttachmentIndex {
        self.bindings()
            .FPDFDoc_GetAttachmentCount(self.document_handle) as PdfAttachmentIndex
    }

    /// Returns `true` if this [PdfAttachments] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of attachments)` for this [PdfAttachments] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfAttachmentIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of attachments - 1)`
    /// for this [PdfAttachments] collection.
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfAttachmentIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single [PdfAttachment] from this [PdfAttachments] collection.
    pub fn get(&self, index: PdfAttachmentIndex) -> Result<PdfAttachment<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::AttachmentIndexOutOfBounds);
        }

        let handle = self
            .bindings()
            .FPDFDoc_GetAttachment(self.document_handle, index as c_int);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfAttachment::from_pdfium(handle, self.bindings()))
        }
    }

    /// Attempts to add a new [PdfAttachment] to this collection, using the given name and the
    /// data in the given byte buffer. An error will be returned if the given name is not
    /// unique in the list of attachments already present in the containing PDF document.
    pub fn create_attachment_from_bytes(
        &mut self,
        name: &str,
        bytes: &[u8],
    ) -> Result<PdfAttachment, PdfiumError> {
        // Creating the attachment is a two step operation. First, we create the FPDF_ATTACHMENT
        // handle using the given name. Then, we add the given byte data to the FPDF_ATTACHMENT.

        let handle = self
            .bindings()
            .FPDFDoc_AddAttachment_str(self.document_handle, name);

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            // With the FPDF_ATTACHMENT correctly created, we can now apply the byte data to the attachment.

            if self
                .bindings()
                .is_true(self.bindings().FPDFAttachment_SetFile(
                    handle,
                    self.document_handle,
                    bytes.as_ptr() as *const c_void,
                    bytes.len() as c_ulong,
                ))
            {
                Ok(PdfAttachment::from_pdfium(handle, self.bindings))
            } else {
                // The return value from FPDFAttachment_SetFile() indicates failure.

                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            }
        }
    }

    /// Attempts to add a new [PdfAttachment] to this collection, using the given name and file path.
    /// Byte data from the given file path will be embedded directly into the containing document.
    /// An error will be returned if the given name is not unique in the list of attachments
    /// already present in the containing PDF document.
    ///
    /// This function is not available when compiling to WASM. You have several options for
    /// loading attachment data in WASM:
    /// * Use the [PdfAttachments::create_attachment_from_fetch()] function to download attachment data
    ///   from a URL using the browser's built-in `fetch()` API. This function is only available when
    ///   compiling to WASM.
    /// * Use the [PdfAttachments::create_attachment_from_blob()] function to load attachment data
    ///   from a Javascript `File` or `Blob` object (such as a `File` object returned from an HTML
    ///   `<input type="file">` element). This function is only available when compiling to WASM.
    /// * Use another method to retrieve the bytes of the target attachment over the network,
    ///   then load those bytes into Pdfium using the [PdfAttachments::create_attachment_from_bytes()] function.
    /// * Embed the bytes of the target attachment directly into the compiled WASM module
    ///   using the `include_bytes!()` macro.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_attachment_from_file(
        &mut self,
        name: &str,
        path: &(impl AsRef<Path> + ?Sized),
    ) -> Result<PdfAttachment, PdfiumError> {
        self.create_attachment_from_reader(name, File::open(path).map_err(PdfiumError::IoError)?)
    }

    /// Attempts to add a new [PdfAttachment] to this collection, using the given name
    /// and the given reader. Byte data from the given reader will be embedded directly into
    /// the containing document. An error will be returned if the given name is not
    /// unique in the list of attachments already present in the containing PDF document.
    pub fn create_attachment_from_reader<R: Read>(
        &mut self,
        name: &str,
        mut reader: R,
    ) -> Result<PdfAttachment, PdfiumError> {
        let mut bytes = Vec::new();

        reader
            .read_to_end(&mut bytes)
            .map_err(PdfiumError::IoError)?;

        self.create_attachment_from_bytes(name, bytes.as_slice())
    }

    /// Attempts to add a new [PdfAttachment] to this collection by loading attachment data
    /// from the given URL. The Javascript `fetch()` API is used to download data over the network.
    /// Byte data retrieved from the given URL will be embedded directly into the containing document.
    /// An error will be returned if the given name is not unique in the list of attachments
    /// already present in the containing PDF document.
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub async fn create_attachment_from_fetch(
        &'a mut self,
        name: &str,
        url: impl ToString,
    ) -> Result<PdfAttachment<'a>, PdfiumError> {
        if let Some(window) = window() {
            let fetch_result = JsFuture::from(window.fetch_with_str(url.to_string().as_str()))
                .await
                .map_err(PdfiumError::WebSysFetchError)?;

            debug_assert!(fetch_result.is_instance_of::<Response>());

            let response: Response = fetch_result
                .dyn_into()
                .map_err(|_| PdfiumError::WebSysInvalidResponseError)?;

            let blob: Blob =
                JsFuture::from(response.blob().map_err(PdfiumError::WebSysFetchError)?)
                    .await
                    .map_err(PdfiumError::WebSysFetchError)?
                    .into();

            self.create_attachment_from_blob(name, blob).await
        } else {
            Err(PdfiumError::WebSysWindowObjectNotAvailable)
        }
    }

    /// Attempts to create a new [PdfAttachment] to this collection, using the given name and
    /// the given `Blob`. Byte data from the given `Blob` will be embedded directly into
    /// the containing document. An error will be returned if the given name is not
    /// unique in the list of attachments already present in the containing PDF document.
    /// A `File` object returned from a `FileList` is a suitable `Blob`:
    ///
    /// ```text
    /// <input id="filePicker" type="file">
    ///
    /// const file = document.getElementById('filePicker').files[0];
    /// ```
    ///
    /// This function is only available when compiling to WASM.
    #[cfg(any(doc, target_arch = "wasm32"))]
    pub async fn create_attachment_from_blob(
        &'a mut self,
        name: &str,
        blob: Blob,
    ) -> Result<PdfAttachment<'a>, PdfiumError> {
        let array_buffer: ArrayBuffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(PdfiumError::WebSysFetchError)?
            .into();

        let u8_array: Uint8Array = Uint8Array::new(&array_buffer);

        let bytes: Vec<u8> = u8_array.to_vec();

        self.create_attachment_from_bytes(name, bytes.as_slice())
    }

    /// Deletes the attachment at the given index from this [PdfAttachments] collection.
    ///
    /// Pdfium's current implementation of this action does not remove the attachment data
    /// from the document; it simply removes the attachment's index entry from the document,
    /// so that the attachment no longer appears in the list of attachments.
    /// This behavior may change in the future.
    pub fn delete_at_index(&mut self, index: PdfAttachmentIndex) -> Result<(), PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::AttachmentIndexOutOfBounds);
        }

        if self.bindings().is_true(
            self.bindings()
                .FPDFDoc_DeleteAttachment(self.document_handle, index as c_int),
        ) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Returns an iterator over all the attachments in this [PdfAttachments] collection.
    #[inline]
    pub fn iter(&self) -> PdfAttachmentsIterator {
        PdfAttachmentsIterator::new(self)
    }
}

/// An iterator over all the [PdfAttachment] objects in a [PdfAttachments] collection.
pub struct PdfAttachmentsIterator<'a> {
    attachments: &'a PdfAttachments<'a>,
    next_index: PdfAttachmentIndex,
}

impl<'a> PdfAttachmentsIterator<'a> {
    #[inline]
    pub(crate) fn new(signatures: &'a PdfAttachments<'a>) -> Self {
        PdfAttachmentsIterator {
            attachments: signatures,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfAttachmentsIterator<'a> {
    type Item = PdfAttachment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.attachments.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
