//! Defines the [PdfiumLibraryBindings] trait, containing run-time bindings to the FPDF_*
//! functions exported by the Pdfium library.
//!
//! By default, `pdfium-render` attempts to bind against the latest released version
//! of the Pdfium API. To explicitly bind against an older version, select one of the
//! crate's Pdfium version feature flags when taking `pdfium-render` as a dependency
//! in your project's `Cargo.toml`.
//!
//! Doc comments on functions in this trait are taken directly from the Pdfium header files
//! and as such are copyright by the Pdfium authors and Google. They are reproduced here
//! as a courtesy for API consumers. The original comments can be found in the Pdfium repository at:
//! <https://pdfium.googlesource.com/pdfium/+/refs/heads/main/public/>

// Include the appropriate implementation of the PdfiumLibraryBindings trait for the
// target architecture and threading model.

// Conditional compilation is used to compile different implementations of
// the PdfiumLibraryBindings trait depending on whether we are compiling to a WASM module,
// a native shared library, or a statically linked library.

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "static"))]
pub(crate) mod dynamic;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "static")]
pub(crate) mod static_bindings;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

// These implementations are all single-threaded (because Pdfium itself is single-threaded).
// Any of them can be wrapped by thread_safe::ThreadSafePdfiumBindings to
// create a thread-safe architecture-specific implementation of the PdfiumLibraryBindings trait.

#[cfg(feature = "thread_safe")]
pub(crate) mod thread_safe;

pub mod version;

use crate::bindgen::{
    size_t, FPDFANNOT_COLORTYPE, FPDF_ACTION, FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE,
    FPDF_ANNOT_APPEARANCEMODE, FPDF_ATTACHMENT, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL,
    FPDF_CLIPPATH, FPDF_DEST, FPDF_DOCUMENT, FPDF_DUPLEXTYPE, FPDF_DWORD, FPDF_FILEACCESS,
    FPDF_FILEIDTYPE, FPDF_FILEWRITE, FPDF_FONT, FPDF_FORMFILLINFO, FPDF_FORMHANDLE, FPDF_GLYPHPATH,
    FPDF_IMAGEOBJ_METADATA, FPDF_LINK, FPDF_OBJECT_TYPE, FPDF_PAGE, FPDF_PAGELINK, FPDF_PAGEOBJECT,
    FPDF_PAGEOBJECTMARK, FPDF_PAGERANGE, FPDF_PATHSEGMENT, FPDF_SCHHANDLE, FPDF_SIGNATURE,
    FPDF_STRUCTELEMENT, FPDF_STRUCTTREE, FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR,
    FPDF_WIDESTRING, FS_FLOAT, FS_MATRIX, FS_POINTF, FS_QUADPOINTSF, FS_RECTF, FS_SIZEF,
};
use crate::bindings::version::PdfiumApiVersion;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::PdfPageObject;
use crate::pdf::document::page::PdfPage;
use crate::pdf::document::PdfDocument;
use crate::utils::pixels::{
    bgra_to_rgba, rgba_to_bgra, unaligned_bgr_to_rgba, unaligned_rgb_to_bgra,
};
use crate::utils::utf16le::{
    get_pdfium_utf16le_bytes_from_str, get_string_from_pdfium_utf16le_bytes,
};
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/// Platform-independent function bindings to an external Pdfium library.
/// On most platforms this will be an external shared library loaded dynamically
/// at runtime, either bundled alongside your compiled Rust application or provided as a system
/// library by the platform. On WASM, this will be a set of Javascript functions exposed by a
/// separate WASM module that is imported into the same browser context.
///
/// Pdfium's API uses three different string types: classic C-style null-terminated char arrays,
/// UTF-8 byte arrays, and a UTF-16LE byte array type named `FPDF_WIDESTRING`. For functions that take a
/// C-style string or a UTF-8 byte array, `pdfium-render`'s binding will take the standard Rust `&str` type.
/// For functions that take an `FPDF_WIDESTRING`, `pdfium-render` exposes two functions: the vanilla
/// `FPDF_*()` function that takes an `FPDF_WIDESTRING`, and an additional `FPDF_*_str()` helper function
/// that takes a standard Rust `&str` and converts it internally to an `FPDF_WIDESTRING` before calling
/// Pdfium. Examples of functions with additional `_str()` helpers include `FPDFBookmark_Find()`,
/// `FPDFAnnot_SetStringValue()`, and `FPDFText_SetText()`.
///
/// The [PdfiumLibraryBindings::get_pdfium_utf16le_bytes_from_str()] and
/// [PdfiumLibraryBindings::get_string_from_pdfium_utf16le_bytes()] functions are provided
/// for converting to and from UTF-16LE in your own code.
///
/// The following Pdfium functions have different signatures in this trait compared to their
/// native function signatures in Pdfium:
/// * [PdfiumLibraryBindings::FPDF_LoadDocument()]: this function is not available when compiling to WASM.
/// * [PdfiumLibraryBindings::FPDFBitmap_GetBuffer()]: the return type of this function is modified
///   when compiling to WASM. Instead of returning `*mut c_void`, it returns `*const c_void`.
///   This is to encourage callers to avoid directly mutating the returned buffer, as this is not
///   supported when compiling to WASM. Instead, callers should use the provided
///   [PdfiumLibraryBindings::FPDFBitmap_SetBuffer()] convenience function to apply modified pixel data
///   to a bitmap.
pub trait PdfiumLibraryBindings {
    /// Returns the canonical C-style boolean integer value 1, indicating `true`.
    #[inline]
    #[allow(non_snake_case)]
    fn TRUE(&self) -> FPDF_BOOL {
        1
    }

    /// Returns the canonical C-style boolean integer value 0, indicating `false`.
    #[inline]
    #[allow(non_snake_case)]
    fn FALSE(&self) -> FPDF_BOOL {
        0
    }

    /// Converts from a C-style boolean integer to a Rust `bool`.
    ///
    /// Assumes `PdfiumLibraryBindings::FALSE()` indicates `false` and any other value indicates `true`.
    #[inline]
    fn is_true(&self, bool: FPDF_BOOL) -> bool {
        bool != self.FALSE()
    }

    /// Converts the given Rust `bool` into a Pdfium `FPDF_BOOL`.
    #[inline]
    fn bool_to_pdfium(&self, bool: bool) -> FPDF_BOOL {
        if bool {
            self.TRUE()
        } else {
            self.FALSE()
        }
    }

    /// Converts from a C-style boolean integer to a Rust `Result`.
    ///
    /// Assumes `PdfiumLibraryBindings::FALSE()` indicates `false` and any other value indicates `true`.
    ///
    /// A value of `PdfiumLibraryBindings::FALSE()` will return a [PdfiumInternalError::Unknown].
    /// All other values will return `Ok(())`.
    #[inline]
    fn to_result(&self, bool: FPDF_BOOL) -> Result<(), PdfiumError> {
        if self.is_true(bool) {
            Ok(())
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }

    /// Converts the given Rust `&str` into an UTF16-LE encoded byte buffer.
    #[inline]
    fn get_pdfium_utf16le_bytes_from_str(&self, str: &str) -> Vec<u8> {
        get_pdfium_utf16le_bytes_from_str(str)
    }

    /// Converts the bytes in the given buffer from UTF16-LE to a standard Rust `String`.
    #[inline]
    #[allow(unused_mut)] // The buffer must be mutable when compiling to WASM.
    fn get_string_from_pdfium_utf16le_bytes(&self, mut buffer: Vec<u8>) -> Option<String> {
        get_string_from_pdfium_utf16le_bytes(buffer)
    }

    /// Converts the given byte array, containing pixel data encoded as three-channel BGR,
    /// into pixel data encoded as four-channel RGBA. A new alpha channel is created with full opacity.
    #[inline]
    fn bgr_to_rgba(&self, bgr: &[u8]) -> Vec<u8> {
        unaligned_bgr_to_rgba(bgr)
    }

    /// Converts the given byte array, containing pixel data encoded as four-channel BGRA,
    /// into pixel data encoded as four-channel RGBA.
    #[inline]
    fn bgra_to_rgba(&self, bgra: &[u8]) -> Vec<u8> {
        bgra_to_rgba(bgra)
    }

    /// Converts the given byte array, containing pixel data encoded as three-channel RGB,
    /// into pixel data encoded as four-channel BGRA. A new alpha channel is created with full opacity.
    #[inline]
    fn rgb_to_bgra(&self, rgb: &[u8]) -> Vec<u8> {
        unaligned_rgb_to_bgra(rgb)
    }

    /// Converts the given byte array, containing pixel data encoded as four-channel RGBA,
    /// into pixel data encoded as four-channel BGRA.
    #[inline]
    fn rgba_to_bgra(&self, rgba: &[u8]) -> Vec<u8> {
        rgba_to_bgra(rgba)
    }

    /// Returns Pdfium's internal `FPDF_DOCUMENT` handle for the given [PdfDocument].
    #[inline]
    fn get_handle_from_document(&self, document: &PdfDocument) -> FPDF_DOCUMENT {
        document.handle()
    }

    /// Returns Pdfium's internal `FPDF_PAGE` handle for the given [PdfPage].
    #[inline]
    fn get_handle_from_page(&self, page: &PdfPage) -> FPDF_PAGE {
        page.page_handle()
    }

    /// Returns Pdfium's internal `FPDF_PAGEOBJECT` handle for the given [PdfPageObject].
    #[inline]
    fn get_handle_from_object(&self, object: &PdfPageObject) -> FPDF_PAGEOBJECT {
        object.get_object_handle()
    }

    /// Returns the API version of the Pdfium FPDF_* API currently in use.
    ///
    /// By default, `pdfium-render` attempts to bind against the latest released version
    /// of the Pdfium API. To explicitly bind against an older version, select one of the
    /// crate's Pdfium version feature flags when taking `pdfium-render` as a dependency
    /// in your project's `Cargo.toml`.
    #[inline]
    fn version(&self) -> PdfiumApiVersion {
        PdfiumApiVersion::current()
    }

    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT;

    /// This function is not available when compiling to WASM. You must use one of the
    /// [PdfiumLibraryBindings::FPDF_LoadMemDocument()], [PdfiumLibraryBindings::FPDF_LoadMemDocument64()],
    /// or [PdfiumLibraryBindings::FPDF_LoadCustomDocument()] functions instead.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT;

    /// Note that all calls to [PdfiumLibraryBindings::FPDF_LoadMemDocument()] are
    /// internally upgraded to [PdfiumLibraryBindings::FPDF_LoadMemDocument64()].
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        self.FPDF_LoadMemDocument64(bytes, password)
    }

    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument64(&self, data_buf: &[u8], password: Option<&str>) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: *mut FPDF_FILEACCESS,
        password: Option<&str>,
    ) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_SaveWithVersion(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT);

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDF_DeviceToPage(
        &self,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        device_x: c_int,
        device_y: c_int,
        page_x: *mut c_double,
        page_y: *mut c_double,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDF_PageToDevice(
        &self,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        page_x: c_double,
        page_y: c_double,
        device_x: *mut c_int,
        device_y: *mut c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_GetSecurityHandlerRevision(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE;

    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE);

    #[allow(non_snake_case)]
    fn FPDF_ImportPagesByIndex(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL;

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportPagesByIndex_vec(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: Vec<c_int>,
        index: c_int,
    ) -> FPDF_BOOL {
        self.FPDF_ImportPagesByIndex(
            dest_doc,
            src_doc,
            page_indices.as_ptr(),
            page_indices.len() as c_ulong,
            index,
        )
    }

    #[allow(non_snake_case)]
    fn FPDF_ImportPages(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: &str,
        index: c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_ImportNPagesToOne(
        &self,
        src_doc: FPDF_DOCUMENT,
        output_width: c_float,
        output_height: c_float,
        num_pages_on_x_axis: size_t,
        num_pages_on_y_axis: size_t,
    ) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> c_float;

    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> c_float;

    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexFromTextIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nTextIndex: c_int,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_GetTextIndexFromCharIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nCharIndex: c_int,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_GetSignatureCount(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_GetSignatureObject(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE;

    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetContents(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetByteRange(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_int,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetSubFilter(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetReason(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetTime(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetDocMDPPermission(&self, signature: FPDF_SIGNATURE) -> c_uint;

    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetForPage(&self, page: FPDF_PAGE) -> FPDF_STRUCTTREE;

    #[allow(non_snake_case)]
    fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE);

    #[allow(non_snake_case)]
    fn FPDF_StructTree_CountChildren(&self, struct_tree: FPDF_STRUCTTREE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: FPDF_STRUCTTREE,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetAltText(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetLang(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetStringAttribute(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetMarkedContentID(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetTitle(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_CountChildren(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetChildAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT;

    #[allow(non_snake_case)]
    fn FPDFPage_New(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: c_double,
        height: c_double,
    ) -> FPDF_PAGE;

    #[allow(non_snake_case)]
    fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: c_int);

    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int);

    #[allow(non_snake_case)]
    fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_GetPageSizeByIndexF(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        size: *mut FS_SIZEF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GetCropBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GetArtBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_SetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    );

    #[allow(non_snake_case)]
    fn FPDFPage_SetCropBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    );

    #[allow(non_snake_case)]
    fn FPDFPage_SetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    );

    #[allow(non_snake_case)]
    fn FPDFPage_SetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    );

    #[allow(non_snake_case)]
    fn FPDFPage_SetArtBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    );

    #[allow(non_snake_case)]
    fn FPDFPage_TransFormWithClip(
        &self,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFPageObj_TransformClipPath(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    );

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetClipPath(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH;

    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPaths(&self, clip_path: FPDF_CLIPPATH) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPathSegments(&self, clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT;

    #[allow(non_snake_case)]
    fn FPDF_CreateClipPath(&self, left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH;

    #[allow(non_snake_case)]
    fn FPDF_DestroyClipPath(&self, clipPath: FPDF_CLIPPATH);

    #[allow(non_snake_case)]
    fn FPDFPage_InsertClipPath(&self, page: FPDF_PAGE, clipPath: FPDF_CLIPPATH);

    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFPage_TransformAnnots(
        &self,
        page: FPDF_PAGE,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    );

    #[allow(non_snake_case)]
    fn FPDFBitmap_CreateEx(
        &self,
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP;

    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP);

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    );

    /// Note that the return type of this function is modified when compiling to WASM. Instead
    /// of returning `*mut c_void`, it returns `*const c_void`.
    ///
    /// When compiling to WASM, Pdfium's internal pixel data buffer for the given bitmap resides
    /// in a separate WASM memory module, so any buffer returned by this function is necessarily
    /// a copy; mutating that copy does not alter the buffer in Pdfium's WASM module and, since
    /// there is no way for `pdfium-render` to know when the caller has finished mutating the
    /// copied buffer, there is no reliable way for `pdfium-render` to transfer any changes made
    /// to the copy across to Pdfium's WASM module.
    ///
    /// To avoid having to maintain different code for different platform targets, it is
    /// recommended that all callers use the provided [PdfiumLibraryBindings::FPDFBitmap_SetBuffer()]
    /// convenience function to apply modified pixel data to a bitmap instead of mutating the
    /// buffer returned by this function.
    #[allow(non_snake_case)]
    #[cfg(not(target_arch = "wasm32"))]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void;

    /// Note that the return type of this function is modified when compiling to WASM. Instead
    /// of returning `*mut c_void`, it returns `*const c_void`.
    ///
    /// When compiling to WASM, Pdfium's internal pixel data buffer for the given bitmap resides
    /// in a separate WASM memory module, so any buffer returned by this function is necessarily
    /// a copy; mutating that copy does not alter the buffer in Pdfium's WASM module and, since
    /// there is no way for `pdfium-render` to know when the caller has finished mutating the
    /// copied buffer, there is no reliable way for `pdfium-render` to transfer any changes made
    /// to the copy across to Pdfium's WASM module.
    ///
    /// **Do not mutate the pixel data in the buffer returned by this function.** Instead, use the
    /// [PdfiumLibraryBindings::FPDFBitmap_SetBuffer()] function to apply a new pixel data
    /// buffer to the bitmap.
    #[allow(non_snake_case)]
    #[cfg(target_arch = "wasm32")]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *const c_void;

    /// This function is not part of the Pdfium API. It is provided by `pdfium-render` as an
    /// alternative to directly mutating the data returned by
    /// [PdfiumLibraryBindings::FPDFBitmap_GetBuffer()].
    ///
    /// Replaces all pixel data for the given bitmap with the pixel data in the given buffer,
    /// returning `true` once the new pixel data has been applied. If the given buffer
    /// does not have the same length as the bitmap's current buffer then the current buffer
    /// will be unchanged and a value of `false` will be returned.
    #[allow(non_snake_case)]
    #[cfg(not(target_arch = "wasm32"))]
    fn FPDFBitmap_SetBuffer(&self, bitmap: FPDF_BITMAP, buffer: &[u8]) -> bool {
        let buffer_length =
            (self.FPDFBitmap_GetStride(bitmap) * self.FPDFBitmap_GetHeight(bitmap)) as usize;

        if buffer.len() != buffer_length {
            return false;
        }

        let buffer_start = self.FPDFBitmap_GetBuffer(bitmap);

        let destination =
            unsafe { std::slice::from_raw_parts_mut(buffer_start as *mut u8, buffer_length) };

        destination.copy_from_slice(buffer);

        true
    }

    /// This function is not part of the Pdfium API. It is provided by `pdfium-render` as an
    /// alternative to directly mutating the data returned by
    /// [PdfiumLibraryBindings::FPDFBitmap_GetBuffer()].
    ///
    /// Replaces all pixel data of the given bitmap with the pixel data in the given buffer,
    /// returning `true` once the new pixel data has been applied. If the given buffer
    /// does not have the same length as the bitmap's current buffer then the current buffer
    /// will be unchanged and a value of `false` will be returned.
    #[allow(non_snake_case)]
    #[cfg(target_arch = "wasm32")]
    fn FPDFBitmap_SetBuffer(&self, bitmap: FPDF_BITMAP, buffer: &[u8]) -> bool;

    /// This function is not part of the Pdfium API. It is provided by `pdfium-render` as a
    /// more performant WASM-specific variant of [PdfiumLibraryBindings::FPDFBitmap_GetBuffer()].
    /// Since it avoids a (potentially large) bitmap allocation and copy, it is both faster and
    /// more memory efficient than [PdfiumLibraryBindings::FPDFBitmap_GetBuffer()].
    ///
    /// This function is only available when compiling to WASM.
    #[allow(non_snake_case)]
    #[cfg(target_arch = "wasm32")]
    fn FPDFBitmap_GetArray(&self, bitmap: FPDF_BITMAP) -> js_sys::Uint8Array;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDF_RenderPageBitmap(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    );

    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmapWithMatrix(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    );

    /// Checks if an annotation subtype is currently supported for creation.
    /// Currently supported subtypes:
    ///
    ///    - circle
    ///
    ///    - file attachment
    ///
    ///    - freetext
    ///
    ///    - highlight
    ///
    ///    - ink
    ///
    ///    - link
    ///
    ///    - popup
    ///
    ///    - square
    ///
    ///    - squiggly
    ///
    ///    - stamp
    ///
    ///    - strikeout
    ///
    ///    - text
    ///
    ///    - underline
    ///
    ///   `subtype`   - the subtype to be checked.
    ///
    /// Returns `true` if this subtype supported.
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL;

    /// Creates an annotation in `page` of the subtype `subtype`. If the specified
    /// subtype is illegal or unsupported, then a new annotation will not be created.
    /// Must call [PdfiumLibraryBindings::FPDFPage_CloseAnnot] when the annotation returned by this
    /// function is no longer needed.
    ///
    ///   `page`      - handle to a page.
    ///
    ///   `subtype`   - the subtype of the new annotation.
    ///
    /// Returns a handle to the new annotation object, or `NULL` on failure.
    #[allow(non_snake_case)]
    fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION;

    /// Gets the number of annotations in `page`.
    ///
    ///   `page`   - handle to a page.
    ///
    /// Returns the number of annotations in `page`.
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> c_int;

    /// Gets annotation in `page` at `index`. Must call [PdfiumLibraryBindings::FPDFPage_CloseAnnot] when the
    /// annotation returned by this function is no longer needed.
    ///
    ///   `page`  - handle to a page.
    ///
    ///   `index` - the index of the annotation.
    ///
    /// Returns a handle to the annotation object, or `NULL` on failure.
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION;

    /// Gets the index of `annot` in `page`. This is the opposite of
    /// [PdfiumLibraryBindings::FPDFPage_GetAnnot].
    ///
    ///   `page`  - handle to the page that the annotation is on.
    ///
    ///   `annot` - handle to an annotation.
    ///
    /// Returns the index of `annot`, or -1 on failure.
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int;

    /// Closes an annotation. Must be called when the annotation returned by
    /// [PdfiumLibraryBindings::FPDFPage_CreateAnnot] or [PdfiumLibraryBindings::FPDFPage_GetAnnot]
    /// is no longer needed. This function does not remove the annotation from the document.
    ///
    ///   `annot`  - handle to an annotation.
    #[allow(non_snake_case)]
    fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION);

    /// Removes the annotation in `page` at `index`.
    ///
    ///   `page`  - handle to a page.
    ///
    ///   `index` - the index of the annotation.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL;

    /// Gets the subtype of an annotation.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    /// Returns the annotation subtype.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE;

    /// Checks if an annotation subtype is currently supported for object extraction,
    /// update, and removal.
    ///
    /// Currently supported subtypes: ink and stamp.
    ///
    ///   `subtype`   - the subtype to be checked.
    ///
    /// Returns `true` if this subtype supported.
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL;

    /// Updates `obj` in `annot`. `obj` must be in `annot` already and must have
    /// been retrieved by [PdfiumLibraryBindings::FPDFAnnot_GetObject]. Currently, only ink and stamp
    /// annotations are supported by this API. Also note that only path, image, and
    /// text objects have APIs for modification; see `FPDFPath_*()`, `FPDFText_*()`, and
    /// `FPDFImageObj_*()`.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `obj`    - handle to the object that `annot` needs to update.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    /// Adds a new InkStroke, represented by an array of points, to the InkList of
    /// `annot`. The API creates an InkList if one doesn't already exist in `annot`.
    /// This API works only for ink annotations. Please refer to ISO 32000-1:2008
    /// spec, section 12.5.6.13.
    ///
    ///   `annot`       - handle to an annotation.
    ///
    ///   `points`      - pointer to a `FS_POINTF` array representing input points.
    ///
    ///   `point_count` - number of elements in `points` array. This should not exceed
    ///                   the maximum value that can be represented by an `int32_t`.
    ///
    /// Returns the 0-based index at which the new InkStroke is added in the InkList
    /// of the `annot`. Returns -1 on failure.
    #[allow(non_snake_case)]
    fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int;

    /// Removes an InkList in `annot`.
    ///
    /// This API works only for ink annotations.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    /// Return true on successful removal of `/InkList` entry from context of the
    /// non-null ink `annot`. Returns `false` on failure.
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL;

    /// Adds `obj` to `annot`. `obj` must have been created by
    /// [PdfiumLibraryBindings::FPDFPageObj_CreateNewPath], [PdfiumLibraryBindings::FPDFPageObj_CreateNewRect],
    /// [PdfiumLibraryBindings::FPDFPageObj_NewTextObj], or [PdfiumLibraryBindings::FPDFPageObj_NewImageObj], and
    /// will be owned by `annot`. Note that an `obj` cannot belong to more than one
    /// `annot`. Currently, only ink and stamp annotations are supported by this API.
    /// Also note that only path, image, and text objects have APIs for creation.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `obj`    - handle to the object that is to be added to `annot`.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    /// Gets the total number of objects in `annot`, including path objects, text
    /// objects, external objects, image objects, and shading objects.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    /// Returns the number of objects in `annot`.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> c_int;

    /// Gets the object in `annot` at `index`.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `index`  - the index of the object.
    ///
    /// Returns a handle to the object, or `NULL` on failure.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT;

    /// Removes the object in `annot` at `index`.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `index`  - the index of the object to be removed.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL;

    /// Sets the color of an annotation. Fails when called on annotations with
    /// appearance streams already defined; instead use
    /// [PdfiumLibraryBindings::FPDFPageObj_SetStrokeColor] or [PdfiumLibraryBindings::FPDFPageObj_SetFillColor].
    ///
    ///   `annot`        - handle to an annotation.
    ///
    ///   `type`         - type of the color to be set.
    ///
    ///   `R`, `G`, `B`  - buffers to hold the RGB values of the color. Ranges from 0 to 255.
    ///
    ///   `A`            - buffers to hold the opacity. Ranges from 0 to 255.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetColor(
        &self,
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL;

    /// Gets the color of an annotation. If no color is specified, default to yellow
    /// for highlight annotation, black for all else. Fails when called on
    /// annotations with appearance streams already defined; instead use
    /// [PdfiumLibraryBindings::FPDFPageObj_GetStrokeColor] or [PdfiumLibraryBindings::FPDFPageObj_GetFillColor].
    ///
    ///   `annot`        - handle to an annotation.
    ///
    ///   `type`         - type of the color requested.
    ///
    ///   `R`, `G`, `B`  - buffers to hold the RGB values of the color. Ranges from 0 to 255.
    ///
    ///   `A`            - buffer to hold the opacity. Ranges from 0 to 255.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetColor(
        &self,
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL;

    /// Checks if the annotation is of a type that has attachment points
    /// (i.e. quadpoints). Quadpoints are the vertices of the rectangle that
    /// encompasses the texts affected by the annotation. They provide the
    /// coordinates in the page where the annotation is attached. Only text markup
    /// annotations (i.e. highlight, strikeout, squiggly, and underline) and link
    /// annotations have quadpoints.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    /// Returns `true` if the annotation is of a type that has quadpoints.
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL;

    /// Replaces the attachment points (i.e. quadpoints) set of an annotation at
    /// `quad_index`. This index needs to be within the result of
    /// [PdfiumLibraryBindings::FPDFAnnot_CountAttachmentPoints].
    ///
    /// If the annotation's appearance stream is defined and this annotation is of a
    /// type with quadpoints, then update the bounding box too if the new quadpoints
    /// define a bigger one.
    ///
    ///   `annot`       - handle to an annotation.
    ///
    ///   `quad_index`  - index of the set of quadpoints.
    ///
    ///   `quad_points` - the quadpoints to be set.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    /// Appends to the list of attachment points (i.e. quadpoints) of an annotation.
    /// If the annotation's appearance stream is defined and this annotation is of a
    /// type with quadpoints, then update the bounding box too if the new quadpoints
    /// define a bigger one.
    ///
    ///   `annot`       - handle to an annotation.
    ///
    ///   `quad_points` - the quadpoints to be set.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    /// Gets the number of sets of quadpoints of an annotation.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    /// Returns the number of sets of quadpoints, or 0 on failure.
    #[allow(non_snake_case)]
    fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> size_t;

    /// Gets the attachment points (i.e. quadpoints) of an annotation.
    ///
    ///   `annot`       - handle to an annotation.
    ///
    ///   `quad_index`  - index of the set of quadpoints.
    ///
    ///   `quad_points` - receives the quadpoints; must not be `NULL`.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    /// Sets the annotation rectangle defining the location of the annotation. If the
    /// annotation's appearance stream is defined and this annotation is of a type
    /// without quadpoints, then update the bounding box too if the new rectangle
    /// defines a bigger one.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `rect`   - the annotation rectangle to be set.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL;

    /// Gets the annotation rectangle defining the location of the annotation.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `rect`   - receives the rectangle; must not be `NULL`.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL;

    /// Gets the vertices of a polygon or polyline annotation. `buffer` is an array of
    /// points of the annotation. If `length` is less than the returned length, or
    /// `annot` or `buffer` is `NULL`, `buffer` will not be modified.
    ///
    ///   `annot`  - handle to an annotation, as returned by e.g. [PdfiumLibraryBindings::FPDFPage_GetAnnot]
    ///
    ///   `buffer` - buffer for holding the points.
    ///
    ///   `length` - length of the buffer in points.
    ///
    /// Returns the number of points if the annotation is of type polygon or
    /// polyline, 0 otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong;

    /// Gets the number of paths in the ink list of an ink annotation.
    ///
    ///   `annot`  - handle to an annotation, as returned by e.g. [PdfiumLibraryBindings::FPDFPage_GetAnnot]
    ///
    /// Returns the number of paths in the ink list if the annotation is of type ink,
    /// 0 otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> c_ulong;

    /// Gets a path in the ink list of an ink annotation. `buffer` is an array of
    /// points of the path. If `length` is less than the returned length, or `annot`
    /// or `buffer` is `NULL`, `buffer` will not be modified.
    ///
    ///   `annot`  - handle to an annotation, as returned by e.g. [PdfiumLibraryBindings::FPDFPage_GetAnnot]
    ///
    ///   `path_index` - index of the path.
    ///
    ///   `buffer` - buffer for holding the points.
    ///
    ///   `length` - length of the buffer in points.
    ///
    /// Returns the number of points of the path if the annotation is of type ink, 0
    /// otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListPath(
        &self,
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong;

    /// Gets the starting and ending coordinates of a line annotation.
    ///
    ///   `annot`  - handle to an annotation, as returned by e.g. [PdfiumLibraryBindings::FPDFPage_GetAnnot]
    ///
    ///   `start` - starting point
    ///
    ///   `end` - ending point
    ///
    /// Returns `true` if the annotation is of type line and `start` and `end` are not
    /// `NULL`, `false` otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL;

    /// Sets the characteristics of the annotation's border (rounded rectangle).
    ///
    ///   `annot`              - handle to an annotation.
    ///
    ///   `horizontal_radius`  - horizontal corner radius, in default user space units.
    ///
    ///   `vertical_radius`    - vertical corner radius, in default user space units.
    ///
    ///   `border_width`       - border width, in default user space units.
    ///
    /// Returns `true` if setting the border for `annot` succeeds, `false` otherwise.
    ///
    /// If `annot` contains an appearance stream that overrides the border values,
    /// then the appearance stream will be removed on success.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: c_float,
        vertical_radius: c_float,
        border_width: c_float,
    ) -> FPDF_BOOL;

    /// Gets the characteristics of the annotation's border (rounded rectangle).
    ///
    ///   `annot`              - handle to an annotation.
    ///
    ///   `horizontal_radius`  - horizontal corner radius, in default user space units.
    ///
    ///   `vertical_radius`    - vertical corner radius, in default user space units.
    ///
    ///   `border_width`       - border width, in default user space units.
    ///
    /// Returns `true` if `horizontal_radius`, `vertical_radius` and `border_width` are
    /// not `NULL`, `false` otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut c_float,
        vertical_radius: *mut c_float,
        border_width: *mut c_float,
    ) -> FPDF_BOOL;

    /// Check if `annot`'s dictionary has `key` as a key.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to look for, encoded in UTF-8.
    ///
    /// Returns `true` if `key` exists.
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasKey(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_BOOL;

    /// Gets the type of the value corresponding to `key` in `annot`'s dictionary.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to look for, encoded in UTF-8.
    ///
    /// Returns the type of the dictionary value.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_OBJECT_TYPE;

    /// Sets the string value corresponding to `key` in `annot`'s dictionary,
    /// overwriting the existing value if any. The value type would be
    /// `FPDF_OBJECT_STRING` after this function call succeeds.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to the dictionary entry to be set, encoded in UTF-8.
    ///
    ///   `value`  - the string value to be set, encoded in UTF-16LE.
    ///
    /// Returns `true` if successful.
    ///
    /// A [&str]-friendly helper function is available for this function.
    /// See [PdfiumLibraryBindings::FPDFAnnot_SetStringValue_str].
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL;

    /// A [&str]-friendly helper function for [PdfiumLibraryBindings::FPDFAnnot_SetStringValue].
    ///
    /// Sets the string value corresponding to `key` in `annot`'s dictionary,
    /// overwriting the existing value if any. The value type would be
    /// `FPDF_OBJECT_STRING` after this function call succeeds.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to the dictionary entry to be set.
    ///
    ///   `value`  - the string value to be set.
    ///
    /// Returns `true` if successful.
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetStringValue_str(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: &str,
    ) -> FPDF_BOOL {
        self.FPDFAnnot_SetStringValue(
            annot,
            key,
            get_pdfium_utf16le_bytes_from_str(value).as_ptr() as FPDF_WIDESTRING,
        )
    }

    /// Gets the string value corresponding to `key` in `annot`'s dictionary. `buffer`
    /// is only modified if `buflen` is longer than the length of contents. Note that
    /// if `key` does not exist in the dictionary or if `key`'s corresponding value
    /// in the dictionary is not a string (i.e. the value is not of type
    /// `FPDF_OBJECT_STRING` or `FPDF_OBJECT_NAME`), then an empty string would be copied
    /// to `buffer` and the return value would be 2. On other errors, nothing would
    /// be added to `buffer` and the return value would be 0.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to the requested dictionary entry, encoded in UTF-8.
    ///
    ///   `buffer` - buffer for holding the value string, encoded in UTF-16LE.
    ///
    ///   `buflen` - length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the float value corresponding to `key` in `annot`'s dictionary. Writes
    /// value to `value` and returns `true` if `key` exists in the dictionary and
    /// `key`'s corresponding value is a number (`FPDF_OBJECT_NUMBER`), `false`
    /// otherwise.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to the requested dictionary entry, encoded in UTF-8.
    ///
    ///   `value`  - receives the value, must not be `NULL`.
    ///
    /// Returns `true` if value found, `false` otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetNumberValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: *mut c_float,
    ) -> FPDF_BOOL;

    /// Sets the AP (appearance string) in `annot`'s dictionary for a given
    /// `appearanceMode`.
    ///
    ///   `annot`          - handle to an annotation.
    ///
    ///   `appearanceMode` - the appearance mode (normal, rollover or down) for which
    ///                      to set the AP.
    ///
    ///   `value`          - the string value to be set, encoded in UTF-16LE. If
    ///                      `nullptr` is passed, the AP is cleared for that mode. If the
    ///                      mode is Normal, APs for all modes are cleared.
    ///
    /// Returns `true` if successful.
    ///
    /// A [&str]-friendly helper function is available for this function.
    /// See [PdfiumLibraryBindings::FPDFAnnot_SetAP_str].
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL;

    /// A [&str]-friendly helper function for [PdfiumLibraryBindings::FPDFAnnot_SetAP].
    ///
    /// Sets the AP (appearance string) in `annot`'s dictionary for a given
    /// `appearanceMode`.
    ///
    ///   `annot`          - handle to an annotation.
    ///
    ///   `appearanceMode` - the appearance mode (normal, rollover or down) for which
    ///                      to set the AP.
    ///
    ///   `value`          - the string value to be set.
    ///
    /// Returns `true` if successful.
    ///
    /// Note that this helper function cannot clear appearance strings, since it cannot pass
    /// a null pointer for `value`. To clear an appearance string, use [PdfiumLibraryBindings::FPDFAnnot_SetAP].
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP_str(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: &str,
    ) -> FPDF_BOOL {
        self.FPDFAnnot_SetAP(
            annot,
            appearanceMode,
            get_pdfium_utf16le_bytes_from_str(value).as_ptr() as FPDF_WIDESTRING,
        )
    }

    /// Gets the AP (appearance string) from `annot`'s dictionary for a given
    /// `appearanceMode`.
    ///
    /// `buffer` is only modified if `buflen` is large enough to hold the whole AP
    /// string. If `buflen` is smaller, the total size of the AP is still returned,
    /// but nothing is copied.
    ///
    /// If there is no appearance stream for `annot` in `appearanceMode`, an empty
    /// string is written to `buf` and 2 is returned.
    ///
    /// On other errors, nothing is written to `buffer` and 0 is returned.
    ///
    ///   `annot`          - handle to an annotation.
    ///
    ///   `appearanceMode` - the appearance mode (normal, rollover or down) for which
    ///                      to get the AP.
    ///
    ///   `buffer`         - buffer for holding the value string, encoded in UTF-16LE.
    ///
    ///   `buflen`         - length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the annotation corresponding to `key` in `annot`'s dictionary. Common
    /// keys for linking annotations include "IRT" and "Popup". Must call
    /// [PdfiumLibraryBindings::FPDFPage_CloseAnnot] when the annotation returned by this function
    /// is no longer needed.
    ///
    ///   `annot`  - handle to an annotation.
    ///
    ///   `key`    - the key to the requested dictionary entry, encoded in UTF-8.
    ///
    /// Returns a handle to the linked annotation object, or `NULL` on failure.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_ANNOTATION;

    /// Gets the annotation flags of `annot`.
    ///
    ///   `annot`    - handle to an annotation.
    ///
    /// Returns the annotation flags.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> c_int;

    /// Sets the `annot`'s flags to be of the value `flags`.
    ///
    ///   `annot`      - handle to an annotation.
    ///
    ///   `flags`      - the flag values to be set.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL;

    /// Gets the annotation flags of `annot`.
    ///
    ///    `hHandle`    -   handle to the form fill module, returned by
    ///                     [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///    `annot`      -   handle to an interactive form annotation.
    ///
    /// Returns the annotation flags specific to interactive forms.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldFlags(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int;

    /// Retrieves an interactive form annotation whose rectangle contains a given
    /// point on a page. Must call [PdfiumLibraryBindings::FPDFPage_CloseAnnot] when the
    /// annotation returned is no longer needed.
    ///
    ///    `hHandle`    -   handle to the form fill module, returned by
    ///                     [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///    `page`       -   handle to the page, returned by [PdfiumLibraryBindings::FPDF_LoadPage] function.
    ///
    ///    `point`      -   position in PDF "user space".
    ///
    /// Returns the interactive form annotation whose rectangle contains the given
    /// coordinates on the page. If there is no such annotation, return `NULL`.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION;

    /// Gets the name of `annot`, which is an interactive form annotation.
    /// `buffer` is only modified if `buflen` is longer than the length of contents.
    /// In case of error, nothing will be added to `buffer` and the return value will
    /// be 0. Note that return value of empty string is 2 for "\0\0".
    ///
    ///    `hHandle`     -   handle to the form fill module, returned by
    ///                      [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///    `annot`       -   handle to an interactive form annotation.
    ///
    ///    `buffer`      -   buffer for holding the name string, encoded in UTF-16LE.
    ///
    ///    `buflen`      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldName(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the form field type of `annot`, which is an interactive form annotation.
    ///
    ///    `hHandle`     -   handle to the form fill module, returned by
    ///                      [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///    `annot`       -   handle to an interactive form annotation.
    ///
    /// Returns the type of the form field (one of the `FPDF_FORMFIELD_*` values) on
    /// success. Returns -1 on error.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldType(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION)
        -> c_int;

    /// Gets the value of `annot`, which is an interactive form annotation.
    /// `buffer` is only modified if `buflen` is longer than the length of contents.
    /// In case of error, nothing will be added to `buffer` and the return value will
    /// be 0. Note that return value of empty string is 2 for "\0\0".
    ///
    ///    `hHandle`     -   handle to the form fill module, returned by
    ///                      [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///    `annot`       -   handle to an interactive form annotation.
    ///
    ///    `buffer`      -   buffer for holding the value string, encoded in UTF-16LE.
    ///
    ///    `buflen`      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the number of options in the `annot`'s "Opt" dictionary. Intended for
    /// use with listbox and combobox widget annotations.
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    /// Returns the number of options in "Opt" dictionary on success. Return value
    /// will be -1 if annotation does not have an "Opt" dictionary or other error.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int;

    /// Gets the string value for the label of the option at `index` in `annot`'s
    /// "Opt" dictionary. Intended for use with listbox and combobox widget
    /// annotations. `buffer` is only modified if `buflen` is longer than the length
    /// of contents. If index is out of range or in case of other error, nothing
    /// will be added to `buffer` and the return value will be 0. Note that
    /// return value of empty string is 2 for "\0\0".
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    ///   `index`   - numeric index of the option in the "Opt" array.
    ///
    ///   `buffer`  - buffer for holding the value string, encoded in UTF-16LE.
    ///
    ///   `buflen`  - length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    /// If `annot` does not have an "Opt" array, `index` is out of range or if any
    /// other error occurs, returns 0.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionLabel(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Determines whether or not the option at `index` in `annot`'s "Opt" dictionary
    /// is selected. Intended for use with listbox and combobox widget annotations.
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    ///   `index`   - numeric index of the option in the "Opt" array.
    ///
    /// Returns `true` if the option at `index` in `annot`'s "Opt" dictionary is
    /// selected, `false` otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsOptionSelected(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL;

    /// Gets the float value of the font size for an `annot` with variable text.
    /// If 0, the font is to be auto-sized: its size is computed as a function of
    /// the height of the annotation rectangle.
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    ///   `value`   - Required. Float which will be set to font size on success.
    ///
    /// Returns `true` if the font size was set in `value`, `false` on error or if
    /// `value` not provided.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut c_float,
    ) -> FPDF_BOOL;

    /// Determines if `annot` is a form widget that is checked. Intended for use with
    /// checkbox and radio button widgets.
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    /// Returns `true` if `annot` is a form widget and is checked, `false` otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL;

    /// Sets the list of focusable annotation subtypes. Annotations of subtype
    /// `FPDF_ANNOT_WIDGET` are by default focusable. New subtypes set using this API
    /// will override the existing subtypes.
    ///
    ///   `hHandle`  - handle to the form fill module, returned by
    ///                [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `subtypes` - list of annotation subtype which can be tabbed over.
    ///
    ///   `count`    - total number of annotation subtype in list.
    ///
    /// Returns `true` if list of annotation subtype is set successfully, `false` otherwise.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL;

    /// Gets the count of focusable annotation subtypes as set by host
    /// for a `hHandle`.
    ///
    ///   `hHandle`  - handle to the form fill module, returned by
    ///                [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    /// Returns the count of focusable annotation subtypes or -1 on error.
    ///
    /// Note: Annotations of type `FPDF_ANNOT_WIDGET` are by default focusable.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> c_int;

    /// Gets the list of focusable annotation subtype as set by host.
    ///
    ///   `hHandle`  - handle to the form fill module, returned by
    ///                [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `subtypes` - receives the list of annotation subtype which can be tabbed
    ///                over. Caller must have allocated `subtypes` more than or
    ///                equal to the count obtained from
    ///                [PdfiumLibraryBindings::FPDFAnnot_GetFocusableSubtypesCount] API.
    ///
    ///   `count`    - size of `subtypes`.
    ///
    /// Returns `true` on success and set list of annotation subtype to `subtypes`,
    /// `false` otherwise.
    ///
    /// Note: Annotations of type `FPDF_ANNOT_WIDGET` are by default focusable.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL;

    /// Gets `FPDF_LINK` object for `annot`. Intended to use for link annotations.
    ///
    ///   `annot`   - handle to an annotation.
    ///
    /// Returns `FPDF_LINK` from the `FPDF_ANNOTATION` and `NULL` on failure,
    /// if the input annot is `NULL`, or input annot's subtype is not link.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK;

    /// Gets the count of annotations in the `annot`'s control group.
    ///
    /// A group of interactive form annotations is collectively called a form
    /// control group. Here, `annot`, an interactive form annotation, should be
    /// either a radio button or a checkbox.
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    /// Returns number of controls in its control group or -1 on error.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int;

    /// Gets the index of `annot` in `annot`'s control group.
    ///
    /// A group of interactive form annotations is collectively called a form
    /// control group. Here, `annot`, an interactive form annotation, should be
    /// either a radio button or a checkbox.
    ///
    ///   `hHandle` - handle to the form fill module, returned by
    ///               [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///   `annot`   - handle to an annotation.
    ///
    /// Returns index of a given `annot` in its control group or -1 on error.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int;

    /// Gets the export value of `annot` which is an interactive form annotation.
    ///
    /// Intended for use with radio button and checkbox widget annotations.
    ///
    /// `buffer` is only modified if `buflen` is longer than the length of contents.
    /// In case of error, nothing will be added to `buffer` and the return value
    /// will be 0. Note that return value of empty string is 2 for "\0\0".
    ///
    ///    `hHandle`     -   handle to the form fill module, returned by
    ///                      [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    ///    `annot`       -   handle to an interactive form annotation.
    ///
    ///    `buffer`      -   buffer for holding the value string, encoded in UTF-16LE.
    ///
    ///    `buflen`      -   length of the buffer in bytes.
    ///
    /// Returns the length of the string value in bytes.
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldExportValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Add a URI action to `annot`, overwriting the existing action, if any.
    ///
    ///   `annot`  - handle to a link annotation.
    ///
    ///   `uri`    - the URI to be set, encoded in 7-bit ASCII.
    ///
    /// Returns `true` if successful.
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: &str) -> FPDF_BOOL;

    ///  Initializes the form fill environment.
    ///
    ///    `document` - Handle to document from [PdfiumLibraryBindings::FPDF_LoadDocument].
    ///
    ///    `formInfo` - Pointer to a `FPDF_FORMFILLINFO` structure.
    ///
    /// Return Value:
    ///        Handle to the form fill module, or `NULL` on failure.
    ///
    /// Comments:
    ///        This function should be called before any form fill operation.
    ///        The `FPDF_FORMFILLINFO` passed in via `form_info` must remain valid until
    ///        the returned `FPDF_FORMHANDLE` is closed.
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE;

    /// Takes ownership of `hHandle` and exits the form fill environment.
    ///
    ///    `hHandle`  -   Handle to the form fill module, as returned by
    ///                   [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    ///
    /// This function is a no-op when `hHandle` is null.
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, hHandle: FPDF_FORMHANDLE);

    /// This method is required for implementing all the form related
    /// functions. Should be invoked after user successfully loaded a
    /// PDF page, and [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment] has been invoked.
    ///
    ///    `hHandle`   -   Handle to the form fill module, as returned by
    ///                    [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    #[allow(non_snake_case)]
    fn FORM_OnAfterLoadPage(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE);

    /// This method is required for implementing all the form related
    /// functions. Should be invoked before user closes the PDF page.
    ///
    ///    `page`      -   Handle to the page, as returned by [PdfiumLibraryBindings::FPDF_LoadPage].
    ///
    ///    `hHandle`   -   Handle to the form fill module, as returned by
    ///                    [PdfiumLibraryBindings::FPDFDOC_InitFormFillEnvironment].
    #[allow(non_snake_case)]
    fn FORM_OnBeforeClosePage(&self, page: FPDF_PAGE, hHandle: FPDF_FORMHANDLE);

    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPage_Flatten(&self, page: FPDF_PAGE, nFlag: c_int) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    );

    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar);

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDF_FFLDraw(
        &self,
        handle: FPDF_FORMHANDLE,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    );

    /// Gets the first child of `bookmark`, or the first top-level bookmark item.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `bookmark` - handle to the current bookmark. Pass `NULL` for the first top
    ///                level item.
    ///
    /// Returns a handle to the first child of `bookmark` or the first top-level
    /// bookmark item. `NULL` if no child or top-level bookmark found.
    /// Note that another name for the bookmarks is the document outline, as
    /// described in ISO 32000-1:2008, section 12.3.3.
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK;

    /// Gets the next sibling of `bookmark`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `bookmark` - handle to the current bookmark.
    ///
    /// Returns a handle to the next sibling of `bookmark`, or `NULL` if this is the
    /// last bookmark at this level.
    ///
    /// Note that the caller is responsible for handling circular bookmark
    /// references, as may arise from malformed documents.
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK;

    /// Gets the title of `bookmark`.
    ///
    ///   `bookmark` - handle to the bookmark.
    ///
    ///   `buffer`   - buffer for the title. May be `NULL`.
    ///
    ///   `buflen`   - the length of the buffer in bytes. May be 0.
    ///
    /// Returns the number of bytes in the title, including the terminating `NUL`
    /// character. The number of bytes is returned regardless of the `buffer` and
    /// `buflen` parameters.
    ///
    /// Regardless of the platform, the `buffer` is always in UTF-16LE encoding. The
    /// string is terminated by a UTF16 `NUL` character. If `buflen` is less than the
    /// required length, or `buffer` is `NULL`, `buffer` will not be modified.
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the number of children of `bookmark`.
    ///
    ///   `bookmark` - handle to the bookmark.
    ///
    /// Returns a signed integer that represents the number of sub-items the given
    /// bookmark has. If the value is positive, child items shall be shown by default
    /// (open state). If the value is negative, child items shall be hidden by
    /// default (closed state). Please refer to PDF 32000-1:2008, Table 153.
    /// Returns 0 if the bookmark has no children or is invalid.
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetCount(&self, bookmark: FPDF_BOOKMARK) -> c_int;

    /// Finds the bookmark with `title` in `document`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `title`    - the UTF-16LE encoded Unicode title for which to search.
    ///
    /// Returns the handle to the bookmark, or `NULL` if `title` can't be found.
    ///
    /// `FPDFBookmark_Find()` will always return the first bookmark found even if
    /// multiple bookmarks have the same `title`.
    ///
    /// A [&str]-friendly helper function is available for this function.
    /// See [PdfiumLibraryBindings::FPDFBookmark_Find_str].
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK;

    /// A [&str]-friendly helper function for [PdfiumLibraryBindings::FPDFBookmark_Find].
    ///
    /// Finds the bookmark with `title` in `document`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `title`    - the title for which to search.
    ///
    /// Returns the handle to the bookmark, or `NULL` if `title` can't be found.
    ///
    /// `FPDFBookmark_Find_str()` will always return the first bookmark found even if
    /// multiple bookmarks have the same `title`.
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find_str(&self, document: FPDF_DOCUMENT, title: &str) -> FPDF_BOOKMARK {
        self.FPDFBookmark_Find(
            document,
            get_pdfium_utf16le_bytes_from_str(title).as_ptr() as FPDF_WIDESTRING,
        )
    }

    /// Gets the destination associated with `bookmark`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `bookmark` - handle to the bookmark.
    ///
    /// Returns the handle to the destination data, or `NULL` if no destination is
    /// associated with `bookmark`.
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST;

    /// Gets the action associated with `bookmark`.
    ///
    ///   `bookmark` - handle to the bookmark.
    ///
    /// Returns the handle to the action data, or `NULL` if no action is associated
    /// with `bookmark`.
    ///
    /// If this function returns a valid handle, it is valid as long as `bookmark` is
    /// valid.
    ///
    /// If this function returns `NULL`, `FPDFBookmark_GetDest()` should be called to get
    /// the `bookmark` destination data.
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION;

    /// Gets the type of `action`.
    ///
    ///   `action` - handle to the action.
    ///
    /// Returns one of:
    ///   - `PDFACTION_UNSUPPORTED`
    ///   - `PDFACTION_GOTO`
    ///   - `PDFACTION_REMOTEGOTO`
    ///   - `PDFACTION_URI`
    ///   - `PDFACTION_LAUNCH`
    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong;

    /// Gets the destination of `action`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `action`   - handle to the action. `action` must be a `PDFACTION_GOTO` or
    ///                `PDFACTION_REMOTEGOTO`.
    ///
    /// Returns a handle to the destination data, or `NULL` on error, typically
    /// because the arguments were bad or the action was of the wrong type.
    ///
    /// In the case of `PDFACTION_REMOTEGOTO`, you must first call
    /// `FPDFAction_GetFilePath()`, then load the document at that path, then pass
    /// the document handle from that document as `document` to `FPDFAction_GetDest()`.
    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST;

    /// Gets the file path of `action`.
    ///
    ///   `action` - handle to the action. `action` must be a `PDFACTION_LAUNCH` or
    ///              `PDFACTION_REMOTEGOTO`.
    ///
    ///   `buffer` - a buffer for output the path string. May be `NULL`.
    ///
    ///   `buflen` - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the file path, including the trailing `NUL`
    /// character, or 0 on error, typically because the arguments were bad or the
    /// action was of the wrong type.
    ///
    /// Regardless of the platform, the `buffer` is always in UTF-8 encoding.
    /// If `buflen` is less than the returned length, or `buffer` is `NULL`, `buffer`
    /// will not be modified.
    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the URI path of `action`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `action`   - handle to the action. Must be a `PDFACTION_URI`.
    ///
    ///   `buffer`   - a buffer for the path string. May be `NULL`.
    ///
    ///   `buflen`   - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the URI path, including the trailing `NUL`
    /// character, or 0 on error, typically because the arguments were bad or the
    /// action was of the wrong type.
    ///
    /// The `buffer` may contain badly encoded data. The caller should validate the
    /// output, i.e. check to see if it is UTF-8.
    ///
    /// If `buflen` is less than the returned length, or `buffer` is `NULL`, buffer`
    /// will not be modified.
    ///
    /// Historically, the documentation for this API claimed `buffer` is always
    /// encoded in 7-bit ASCII, but did not actually enforce it.
    /// <https://pdfium.googlesource.com/pdfium.git/+/d609e84cee2e14a18333247485af91df48a40592>
    /// added that enforcement, but that did not work well for real world PDFs that
    /// used UTF-8. As of this writing, this API reverted back to its original
    /// behavior prior to commit d609e84cee.
    #[allow(non_snake_case)]
    fn FPDFAction_GetURIPath(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the page index of `dest`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `dest`     - handle to the destination.
    ///
    /// Returns the 0-based page index containing `dest`. Returns -1 on error.
    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int;

    /// Gets the view (fit type) specified by `dest`.
    ///
    ///   `dest`         - handle to the destination.
    ///
    ///   `pNumParams`   - receives the number of view parameters, which is at most 4.
    ///
    ///   `pParams`      - buffer to write the view parameters. Must be at least 4
    ///                    `FS_FLOAT`s long.
    ///
    /// Returns one of the `PDFDEST_VIEW_*` constants, or `PDFDEST_VIEW_UNKNOWN_MODE` if
    /// `dest` does not specify a view.
    #[allow(non_snake_case)]
    fn FPDFDest_GetView(
        &self,
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong;

    /// Gets the (`x`, `y`, `zoom`) location of `dest` in the destination page, if the
    /// destination is in `page /XYZ x y zoom` syntax.
    ///
    ///   `dest`       - handle to the destination.
    ///
    ///   `hasXVal`    - out parameter; true if the `x` value is not null
    ///
    ///   `hasYVal`    - out parameter; true if the `y` value is not null
    ///
    ///   `hasZoomVal` - out parameter; true if the `zoom` value is not null
    ///
    ///   `x`          - out parameter; the `x` coordinate, in page coordinates.
    ///
    ///   `y`          - out parameter; the `y` coordinate, in page coordinates.
    ///
    ///   `zoom`       - out parameter; the `zoom` value.
    ///
    /// Returns `true` on successfully reading the `/XYZ` value.
    ///
    /// Note the `x`, `y`, `zoom` values are only set if the corresponding `hasXVal`,
    /// `hasYVal`, or `hasZoomVal` flags are true.
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFDest_GetLocationInPage(
        &self,
        dest: FPDF_DEST,
        hasXVal: *mut FPDF_BOOL,
        hasYVal: *mut FPDF_BOOL,
        hasZoomVal: *mut FPDF_BOOL,
        x: *mut FS_FLOAT,
        y: *mut FS_FLOAT,
        zoom: *mut FS_FLOAT,
    ) -> FPDF_BOOL;

    /// Finds a link at point (`x`, `y`) on `page`.
    ///
    ///   `page` - handle to the document page.
    ///
    ///   `x`    - the `x` coordinate, in the page coordinate system.
    ///
    ///   `y`    - the `y` coordinate, in the page coordinate system.
    ///
    /// Returns a handle to the link, or `NULL` if no link found at the given point.
    ///
    /// You can convert coordinates from screen coordinates to page coordinates using
    /// `FPDF_DeviceToPage()`.
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> FPDF_LINK;

    /// Finds the Z-order of link at point (`x`, `y`) on `page`.
    ///
    ///   `page` - handle to the document page.
    ///
    ///   `x`    - the `x` coordinate, in the page coordinate system.
    ///
    ///   `y`    - the `y` coordinate, in the page coordinate system.
    ///
    /// Returns the Z-order of the link, or -1 if no link found at the given point.
    /// Larger Z-order numbers are closer to the front.
    ///
    /// You can convert coordinates from screen coordinates to page coordinates using
    /// `FPDF_DeviceToPage()`.
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkZOrderAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> c_int;

    /// Gets destination info for `link`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `link`     - handle to the link.
    ///
    /// Returns a handle to the destination, or `NULL` if there is no destination
    /// associated with the link. In this case, you should call `FPDFLink_GetAction()`
    /// to retrieve the action associated with `link`.
    #[allow(non_snake_case)]
    fn FPDFLink_GetDest(&self, document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST;

    /// Gets action info for `link`.
    ///
    ///   `link` - handle to the link.
    ///
    /// Returns a handle to the action associated to `link`, or `NULL` if no action.
    /// If this function returns a valid handle, it is valid as long as `link` is
    /// valid.
    #[allow(non_snake_case)]
    fn FPDFLink_GetAction(&self, link: FPDF_LINK) -> FPDF_ACTION;

    /// Enumerates all the link annotations in `page`.
    ///
    ///   `page`       - handle to the page.
    ///
    ///   `start_pos`  - the start position, should initially be 0 and is updated with
    ///                  the next start position on return.
    ///
    ///   `link_annot` - the link handle for `startPos`.
    ///
    /// Returns `true` on success.
    #[allow(non_snake_case)]
    fn FPDFLink_Enumerate(
        &self,
        page: FPDF_PAGE,
        start_pos: *mut c_int,
        link_annot: *mut FPDF_LINK,
    ) -> FPDF_BOOL;

    /// Gets `FPDF_ANNOTATION` object for `link_annot`.
    ///
    ///   `page`       - handle to the page in which `FPDF_LINK` object is present.
    ///
    ///   `link_annot` - handle to link annotation.
    ///
    /// Returns `FPDF_ANNOTATION` from the `FPDF_LINK` or `NULL` on failure,
    /// if the input link annot or page is `NULL`.
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnot(&self, page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION;

    /// Gets the rectangle for `link_annot`.
    ///
    ///   `link_annot` - handle to the link annotation.
    ///
    ///   `rect`       - the annotation rectangle.
    ///
    /// Returns `true` on success.
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnotRect(&self, link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL;

    /// Gets the count of quadrilateral points to the `link_annot`.
    ///
    ///   `link_annot` - handle to the link annotation.
    ///
    /// Returns the count of quadrilateral points.
    #[allow(non_snake_case)]
    fn FPDFLink_CountQuadPoints(&self, link_annot: FPDF_LINK) -> c_int;

    /// Gets the quadrilateral points for the specified `quad_index` in `link_annot`.
    ///
    ///   `link_annot`  - handle to the link annotation.
    ///
    ///   `quad_index`  - the specified quad point index.
    ///
    ///   `quad_points` - receives the quadrilateral points.
    ///
    /// Returns `true` on success.
    #[allow(non_snake_case)]
    fn FPDFLink_GetQuadPoints(
        &self,
        link_annot: FPDF_LINK,
        quad_index: c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    /// Gets an additional-action from `page`.
    ///
    ///   `page`      - handle to the page, as returned by `FPDF_LoadPage()`.
    ///
    ///   `aa_type`   - the type of the page object's additional-action, defined
    ///                 in `public/fpdf_formfill.h`
    ///
    ///   Returns the handle to the action data, or `NULL` if there is no
    ///   additional-action of type `aa_type`.
    ///
    ///   If this function returns a valid handle, it is valid as long as `page` is
    ///   valid.
    #[allow(non_snake_case)]
    fn FPDF_GetPageAAction(&self, page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION;

    /// Gets the file identifier defined in the trailer of `document`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `id_type`  - the file identifier type to retrieve.
    ///
    ///   `buffer`   - a buffer for the file identifier. May be `NULL`.
    ///
    ///   `buflen`   - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the file identifier, including the `NUL`
    /// terminator.
    ///
    /// The `buffer` is always a byte string. The `buffer` is followed by a `NUL`
    /// terminator.  If `buflen` is less than the returned length, or `buffer` is
    /// `NULL`, `buffer` will not be modified.
    #[allow(non_snake_case)]
    fn FPDF_GetFileIdentifier(
        &self,
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets meta-data `tag` content from `document`.
    ///
    ///   `document` - handle to the document.
    ///
    ///   `tag`      - the tag to retrieve. The tag can be one of:
    ///                Title, Author, Subject, Keywords, Creator, Producer,
    ///                CreationDate, or ModDate.
    ///                For detailed explanations of these tags and their respective
    ///                values, please refer to PDF Reference 1.6, section 10.2.1,
    ///                "Document Information Dictionary".
    ///
    ///   `buffer`   - a buffer for the tag. May be `NULL`.
    ///
    ///   `buflen`   - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the tag, including trailing zeros.
    ///
    /// The |buffer| is always encoded in UTF-16LE. The `buffer` is followed by two
    /// bytes of zeros indicating the end of the string.  If `buflen` is less than
    /// the returned length, or `buffer` is `NULL`, `buffer` will not be modified.
    ///
    /// For linearized files, `FPDFAvail_IsFormAvail()` must be called before this, and
    /// it must have returned `PDF_FORM_AVAIL` or `PDF_FORM_NOTEXIST`. Before that, there
    /// is no guarantee the metadata has been loaded.
    #[allow(non_snake_case)]
    fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Gets the page label for `page_index` from `document`.
    ///
    ///   `document`    - handle to the document.
    ///
    ///   `page_index`  - the 0-based index of the page.
    ///
    ///   `buffer`      - a buffer for the page label. May be `NULL`.
    ///
    ///   `buflen`      - the length of the buffer, in bytes. May be 0.
    ///
    /// Returns the number of bytes in the page label, including trailing zeros.
    ///
    /// The `buffer` is always encoded in UTF-16LE. The `buffer` is followed by two
    /// bytes of zeros indicating the end of the string.  If `buflen` is less than
    /// the returned length, or `buffer` is `NULL`, `buffer` will not be modified.
    #[allow(non_snake_case)]
    fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE;

    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE);

    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_GetUnicode(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint;

    #[cfg(any(feature = "pdfium_6611", feature = "pdfium_future"))]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextObject(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFText_GetFontSize(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_double;

    #[allow(non_snake_case)]
    fn FPDFText_GetFontInfo(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        flags: *mut c_int,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFText_GetFontWeight(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_int;

    #[cfg(any(
        feature = "pdfium_6569",
        feature = "pdfium_6555",
        feature = "pdfium_6490",
        feature = "pdfium_6406",
        feature = "pdfium_6337",
        feature = "pdfium_6295",
        feature = "pdfium_6259",
        feature = "pdfium_6164",
        feature = "pdfium_6124",
        feature = "pdfium_6110",
        feature = "pdfium_6084",
        feature = "pdfium_6043",
        feature = "pdfium_6015",
        feature = "pdfium_5961"
    ))]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextRenderMode(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
    ) -> FPDF_TEXT_RENDERMODE;

    #[allow(non_snake_case)]
    fn FPDFText_GetFillColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetStrokeColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetCharAngle(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_float;

    #[allow(non_snake_case)]
    fn FPDFText_GetCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        left: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
        top: *mut c_double,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetLooseCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetMatrix(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetCharOrigin(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        x: *mut c_double,
        y: *mut c_double,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexAtPos(
        &self,
        text_page: FPDF_TEXTPAGE,
        x: c_double,
        y: c_double,
        xTolerance: c_double,
        yTolerance: c_double,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_GetText(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
        result: *mut c_ushort,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_CountRects(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_GetRect(
        &self,
        text_page: FPDF_TEXTPAGE,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFText_GetBoundedText(
        &self,
        text_page: FPDF_TEXTPAGE,
        left: c_double,
        top: c_double,
        right: c_double,
        bottom: c_double,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_FindStart(
        &self,
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE;

    #[allow(non_snake_case)]
    fn FPDFText_FindStart_str(
        &self,
        text_page: FPDF_TEXTPAGE,
        findwhat: &str,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE {
        self.FPDFText_FindStart(
            text_page,
            get_pdfium_utf16le_bytes_from_str(findwhat).as_ptr() as FPDF_WIDESTRING,
            flags,
            start_index,
        )
    }

    #[allow(non_snake_case)]
    fn FPDFText_FindNext(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_FindPrev(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_GetSchResultIndex(&self, handle: FPDF_SCHHANDLE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_GetSchCount(&self, handle: FPDF_SCHHANDLE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE);

    #[allow(non_snake_case)]
    fn FPDFLink_LoadWebLinks(&self, text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK;

    #[allow(non_snake_case)]
    fn FPDFLink_CountWebLinks(&self, link_page: FPDF_PAGELINK) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFLink_GetURL(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFLink_CountRects(&self, link_page: FPDF_PAGELINK, link_index: c_int) -> c_int;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFLink_GetRect(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFLink_GetTextRange(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        start_char_index: *mut c_int,
        char_count: *mut c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK);

    #[allow(non_snake_case)]
    fn FPDFPage_GetDecodedThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFPage_GetRawThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFPage_GetThumbnailAsBitmap(&self, page: FPDF_PAGE) -> FPDF_BITMAP;

    #[allow(non_snake_case)]
    fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE;

    #[allow(non_snake_case)]
    fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFTextObj_GetText(
        &self,
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT;

    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFFont_Close(&self, font: FPDF_FONT);

    #[allow(non_snake_case)]
    fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFPath_BezierTo(
        &self,
        path: FPDF_PAGEOBJECT,
        x1: c_float,
        y1: c_float,
        x2: c_float,
        y2: c_float,
        x3: c_float,
        y3: c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPath_SetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPath_GetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: &str,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL;

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetText_str(&self, text_object: FPDF_PAGEOBJECT, text: &str) -> FPDF_BOOL {
        self.FPDFText_SetText(
            text_object,
            get_pdfium_utf16le_bytes_from_str(text).as_ptr() as FPDF_WIDESTRING,
        )
    }

    #[allow(non_snake_case)]
    fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFText_LoadFont(
        &self,
        document: FPDF_DOCUMENT,
        data: *const c_uchar,
        size: c_uint,
        font_type: c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT;

    #[allow(non_snake_case)]
    fn FPDFText_LoadStandardFont(&self, document: FPDF_DOCUMENT, font: &str) -> FPDF_FONT;

    #[allow(non_snake_case)]
    fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT);

    #[allow(non_snake_case)]
    fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFPageObj_Destroy(&self, page_obj: FPDF_PAGEOBJECT);

    #[allow(non_snake_case)]
    fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn FPDFPageObj_Transform(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    );

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetMatrix(&self, path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECTMARK;

    #[allow(non_snake_case)]
    fn FPDFPageObj_AddMark(&self, page_object: FPDF_PAGEOBJECT, name: &str) -> FPDF_PAGEOBJECTMARK;

    #[allow(non_snake_case)]
    fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetName(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamKey(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        index: c_ulong,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_OBJECT_TYPE;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        out_value: *mut c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetIntParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetStringParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: &str,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFile(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFileInline(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        note = "Prefer FPDFPageObj_SetMatrix() over FPDFImageObj_SetMatrix(). FPDFImageObj_SetMatrix() is deprecated and will likely be removed in a future version of Pdfium."
    )]
    fn FPDFImageObj_SetMatrix(
        &self,
        image_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFImageObj_SetBitmap(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilter(
        &self,
        image_object: FPDF_PAGEOBJECT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewPath(&self, x: c_float, y: c_float) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewRect(
        &self,
        x: c_float,
        y: c_float,
        w: c_float,
        h: c_float,
    ) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetBounds(
        &self,
        page_object: FPDF_PAGEOBJECT,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: &str);

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeWidth(&self, page_object: FPDF_PAGEOBJECT, width: c_float)
        -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const c_float,
        dash_count: size_t,
        phase: c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPath_CountSegments(&self, path: FPDF_PAGEOBJECT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPath_GetPathSegment(&self, path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT;

    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetPoint(
        &self,
        segment: FPDF_PATHSEGMENT,
        x: *mut c_float,
        y: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetType(&self, segment: FPDF_PATHSEGMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetClose(&self, segment: FPDF_PATHSEGMENT) -> FPDF_BOOL;

    // TODO: AJRC - 4-Aug-2024 - FPDFFont_GetBaseFontName() is in Pdfium export headers
    // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
    #[cfg(feature = "pdfium_future")]
    #[allow(non_snake_case)]
    fn FPDFFont_GetBaseFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: size_t,
    ) -> size_t;

    // TODO: AJRC - 4-Aug-2024 - pointer type updated in FPDFFont_GetBaseFontName() definition,
    // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
    #[cfg(feature = "pdfium_future")]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFamilyName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: size_t,
    ) -> size_t;

    #[cfg(feature = "pdfium_6611")]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFamilyName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong;

    #[cfg(any(
        feature = "pdfium_6569",
        feature = "pdfium_6555",
        feature = "pdfium_6490",
        feature = "pdfium_6406",
        feature = "pdfium_6337",
        feature = "pdfium_6295",
        feature = "pdfium_6259",
        feature = "pdfium_6164",
        feature = "pdfium_6124",
        feature = "pdfium_6110",
        feature = "pdfium_6084",
        feature = "pdfium_6043",
        feature = "pdfium_6015",
        feature = "pdfium_5961"
    ))]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFFont_GetFontData(
        &self,
        font: FPDF_FONT,
        buffer: *mut u8,
        buflen: size_t,
        out_buflen: *mut size_t,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFFont_GetIsEmbedded(&self, font: FPDF_FONT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFFont_GetItalicAngle(&self, font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFFont_GetAscent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        ascent: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFFont_GetDescent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        descent: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphWidth(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
        width: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphPath(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
    ) -> FPDF_GLYPHPATH;

    #[allow(non_snake_case)]
    fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: FPDF_GLYPHPATH) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: FPDF_GLYPHPATH,
        index: c_int,
    ) -> FPDF_PATHSEGMENT;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetNumCopies(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRange(&self, document: FPDF_DOCUMENT) -> FPDF_PAGERANGE;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> size_t;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: FPDF_PAGERANGE,
        index: size_t,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE;

    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetName(
        &self,
        document: FPDF_DOCUMENT,
        key: &str,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong;

    /// Get the number of embedded files in `document`.
    ///
    ///   document - handle to a document.
    ///
    /// Returns the number of embedded files in `document`.
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachmentCount(&self, document: FPDF_DOCUMENT) -> c_int;

    /// Add an embedded file with `name` in `document`. If `name` is empty, or if
    /// `name` is the name of a existing embedded file in `document`, or if
    /// `document`'s embedded file name tree is too deep (i.e. `document` has too
    /// many embedded files already), then a new attachment will not be added.
    ///
    ///   document - handle to a document.
    ///
    ///   name     - name of the new attachment.
    ///
    /// Returns a handle to the new attachment object, or NULL on failure.
    ///
    /// A [&str]-friendly helper function is available for this function.
    /// See [PdfiumLibraryBindings::FPDFDoc_AddAttachment_str].
    #[allow(non_snake_case)]
    fn FPDFDoc_AddAttachment(
        &self,
        document: FPDF_DOCUMENT,
        name: FPDF_WIDESTRING,
    ) -> FPDF_ATTACHMENT;

    /// A [&str]-friendly helper function for [PdfiumLibraryBindings::FPDFDoc_AddAttachment].
    ///
    /// Add an embedded file with `name` in `document`. If `name` is empty, or if
    /// `name` is the name of a existing embedded file in `document`, or if
    /// `document`'s embedded file name tree is too deep (i.e. `document` has too
    /// many embedded files already), then a new attachment will not be added.
    ///
    ///   document - handle to a document.
    ///
    ///   name     - name of the new attachment.
    ///
    /// Returns a handle to the new attachment object, or NULL on failure.
    #[allow(non_snake_case)]
    fn FPDFDoc_AddAttachment_str(&self, document: FPDF_DOCUMENT, name: &str) -> FPDF_ATTACHMENT {
        self.FPDFDoc_AddAttachment(
            document,
            get_pdfium_utf16le_bytes_from_str(name).as_ptr() as FPDF_WIDESTRING,
        )
    }

    /// Get the embedded attachment at `index` in `document`. Note that the returned
    /// attachment handle is only valid while `document` is open.
    ///
    ///   document - handle to a document.
    ///
    ///   index    - the index of the requested embedded file.
    ///
    /// Returns the handle to the attachment object, or NULL on failure.
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT;

    /// Delete the embedded attachment at `index` in `document`. Note that this does
    /// not remove the attachment data from the PDF file; it simply removes the
    /// file's entry in the embedded files name tree so that it does not appear in
    /// the attachment list. This behavior may change in the future.
    ///
    ///   document - handle to a document.
    ///
    ///   index    - the index of the embedded file to be deleted.
    ///
    /// Returns true if successful.
    #[allow(non_snake_case)]
    fn FPDFDoc_DeleteAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL;

    /// Get the name of the `attachment` file. `buffer` is only modified if `buflen`
    /// is longer than the length of the file name. On errors, `buffer` is unmodified
    /// and the returned length is 0.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   buffer     - buffer for holding the file name, encoded in UTF-16LE.
    ///
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the file name in bytes.
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetName(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Check if the params dictionary of `attachment` has `key` as a key.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   key        - the key to look for, encoded in UTF-8.
    ///
    /// Returns true if `key` exists.
    #[allow(non_snake_case)]
    fn FPDFAttachment_HasKey(&self, attachment: FPDF_ATTACHMENT, key: &str) -> FPDF_BOOL;

    /// Get the type of the value corresponding to `key` in the params dictionary of
    /// the embedded `attachment`.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   key        - the key to look for, encoded in UTF-8.
    ///
    /// Returns the type of the dictionary value.
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetValueType(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
    ) -> FPDF_OBJECT_TYPE;

    /// Set the string value corresponding to `key` in the params dictionary of the
    /// embedded file `attachment`, overwriting the existing value if any. The value
    /// type should be FPDF_OBJECT_STRING after this function call succeeds.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   key        - the key to the dictionary entry, encoded in UTF-8.
    ///
    ///   value      - the string value to be set, encoded in UTF-16LE.
    ///
    /// Returns true if successful.
    ///
    /// A [&str]-friendly helper function is available for this function.
    /// See [PdfiumLibraryBindings::FPDFAttachment_SetStringValue_str].
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL;

    /// A [&str]-friendly helper function for [PdfiumLibraryBindings::FPDFAttachment_SetStringValue].
    ///
    /// Set the string value corresponding to `key` in the params dictionary of the
    /// embedded file `attachment`, overwriting the existing value if any. The value
    /// type should be FPDF_OBJECT_STRING after this function call succeeds.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   key        - the key to the dictionary entry.
    ///
    ///   value      - the string value to be set.
    ///
    /// Returns true if successful.
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetStringValue_str(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        value: &str,
    ) -> FPDF_BOOL {
        self.FPDFAttachment_SetStringValue(
            attachment,
            key,
            get_pdfium_utf16le_bytes_from_str(value).as_ptr() as FPDF_WIDESTRING,
        )
    }

    /// Get the string value corresponding to `key` in the params dictionary of the
    /// embedded file `attachment`. `buffer` is only modified if `buflen` is longer
    /// than the length of the string value. Note that if `key` does not exist in the
    /// dictionary or if `key`'s corresponding value in the dictionary is not a
    /// string (i.e. the value is not of type FPDF_OBJECT_STRING or
    /// FPDF_OBJECT_NAME), then an empty string would be copied to `buffer` and the
    /// return value would be 2. On other errors, nothing would be added to `buffer`
    /// and the return value would be 0.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   key        - the key to the requested string value, encoded in UTF-8.
    ///
    ///   buffer     - buffer for holding the string value encoded in UTF-16LE.
    ///
    ///   buflen     - length of the buffer in bytes.
    ///
    /// Returns the length of the dictionary value string in bytes.
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    /// Set the file data of `attachment`, overwriting the existing file data if any.
    /// The creation date and checksum will be updated, while all other dictionary
    /// entries will be deleted. Note that only contents with `len` smaller than
    /// INT_MAX is supported.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   contents   - buffer holding the file data to write to `attachment`.
    ///
    ///   len        - length of file data in bytes.
    ///
    /// Returns true if successful.
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const c_void,
        len: c_ulong,
    ) -> FPDF_BOOL;

    /// Get the file data of `attachment`.
    ///
    /// When the attachment file data is readable, true is returned, and `out_buflen`
    /// is updated to indicate the file data size. `buffer` is only modified if
    /// `buflen` is non-null and long enough to contain the entire file data. Callers
    /// must check both the return value and the input `buflen` is no less than the
    /// returned `out_buflen` before using the data.
    ///
    /// Otherwise, when the attachment file data is unreadable or when `out_buflen`
    /// is null, false is returned and `buffer` and `out_buflen` remain unmodified.
    ///
    ///   attachment - handle to an attachment.
    ///
    ///   buffer     - buffer for holding the file data from `attachment`.
    ///
    ///   buflen     - length of the buffer in bytes.
    ///
    ///   out_buflen - pointer to the variable that will receive the minimum buffer
    ///                size to contain the file data of `attachment`.
    ///
    /// Returns true on success, false otherwise.
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL;

    /// Determines if `document` represents a tagged PDF.
    ///
    /// For the definition of tagged PDF, see 10.7 "Tagged PDF" in PDF Reference 1.7.
    ///
    ///    `document` - handle to a document.
    ///
    /// Returns `true` if `document` is a tagged PDF.
    #[allow(non_snake_case)]
    fn FPDFCatalog_IsTagged(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils::test::test_bind_to_pdfium;

    #[test]
    fn test_is_true() -> Result<(), PdfiumError> {
        let pdfium = test_bind_to_pdfium();

        assert!(!pdfium.bindings().is_true(0));
        assert!(pdfium.bindings().is_true(1));
        assert!(pdfium.bindings().is_true(-1));

        Ok(())
    }
}
