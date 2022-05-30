//! Defines the [PdfiumLibraryBindings] trait, containing run-time bindings to the FPDF_* functions
//! exported by the Pdfium library.

use crate::bindgen::{
    size_t, FPDFANNOT_COLORTYPE, FPDF_ACTION, FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE,
    FPDF_ANNOT_APPEARANCEMODE, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL, FPDF_DEST, FPDF_DOCUMENT,
    FPDF_DWORD, FPDF_FILEACCESS, FPDF_FILEWRITE, FPDF_FONT, FPDF_FORMFILLINFO, FPDF_FORMHANDLE,
    FPDF_GLYPHPATH, FPDF_IMAGEOBJ_METADATA, FPDF_LINK, FPDF_OBJECT_TYPE, FPDF_PAGE,
    FPDF_PAGEOBJECT, FPDF_PAGEOBJECTMARK, FPDF_PATHSEGMENT, FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE,
    FPDF_WCHAR, FPDF_WIDESTRING, FS_MATRIX, FS_POINTF, FS_QUADPOINTSF, FS_RECTF,
};
use crate::error::PdfiumInternalError;
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
/// Note that the [PdfiumLibraryBindings::FPDF_LoadDocument()] function is not available when
/// compiling to WASM. Either embed the target PDF document directly using the [include_bytes!()]
/// macro, or use Javascript's `fetch()` API to retrieve the bytes of the target document over
/// the network, then load those bytes into Pdfium using the [PdfiumLibraryBindings::FPDF_LoadMemDocument()] function.
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
    /// Assumes `PdfiumLibraryBindings::TRUE()` indicates `true` and any other value indicates `false`.
    #[inline]
    fn is_true(&self, bool: FPDF_BOOL) -> bool {
        bool == self.TRUE()
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

    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self);

    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT;

    /// This function is not available when compiling to WASM. Either embed the target PDF document
    /// directly using the [include_bytes!()] macro, or use Javascript's `fetch()` API to retrieve
    /// the bytes of the target document over the network, then load those bytes into Pdfium using
    /// the [PdfiumLibraryBindings::FPDF_LoadMemDocument()] function.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT;

    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT;

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
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int;

    #[allow(non_snake_case)]
    fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

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
    fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

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
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL;

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
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    );

    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void;

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
    fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION;

    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION;

    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION);

    #[allow(non_snake_case)]
    fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE;

    #[allow(non_snake_case)]
    fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT;

    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL;

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

    #[allow(non_snake_case)]
    fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> size_t;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListPath(
        &self,
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: c_float,
        vertical_radius: c_float,
        border_width: c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut c_float,
        vertical_radius: *mut c_float,
        border_width: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_HasKey(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_OBJECT_TYPE;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL;

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

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetNumberValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL;

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

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_ANNOTATION;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldFlags(&self, handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION)
        -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldName(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldType(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION)
        -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionLabel(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut c_float,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldExportValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: &str) -> FPDF_BOOL;

    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE;

    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE);

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

    #[allow(non_snake_case)]
    fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK;

    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK;

    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK;

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find_str(&self, document: FPDF_DOCUMENT, title: &str) -> FPDF_BOOKMARK {
        self.FPDFBookmark_Find(
            document,
            get_pdfium_utf16le_bytes_from_str(title).as_ptr() as FPDF_WIDESTRING,
        )
    }

    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST;

    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION;

    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST;

    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFAction_GetURIPath(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong;

    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int;

    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE;

    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE);

    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int;

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
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong;

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

    /// Retrieves the error code of the last error, if any, recorded by the external
    /// Pdfium library and maps it to a [PdfiumInternalError] enum value.
    #[inline]
    fn get_pdfium_last_error(&self) -> Option<PdfiumInternalError> {
        let result = self.FPDF_GetLastError() as u32;

        match result {
            crate::bindgen::FPDF_ERR_SUCCESS => None,
            crate::bindgen::FPDF_ERR_UNKNOWN => Some(PdfiumInternalError::Unknown),
            crate::bindgen::FPDF_ERR_FILE => Some(PdfiumInternalError::FileError),
            crate::bindgen::FPDF_ERR_FORMAT => Some(PdfiumInternalError::FormatError),
            crate::bindgen::FPDF_ERR_PASSWORD => Some(PdfiumInternalError::PasswordError),
            crate::bindgen::FPDF_ERR_SECURITY => Some(PdfiumInternalError::SecurityError),
            crate::bindgen::FPDF_ERR_PAGE => Some(PdfiumInternalError::PageError),
            _ => Some(PdfiumInternalError::Unknown),
        }
    }
}
