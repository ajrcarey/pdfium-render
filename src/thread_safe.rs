// Wraps an architecture-specific single-threaded implementation of the PdfiumLibraryBindings trait
// behind a mutex marshaller to create a thread-safe trait implementation.

// Pdfium itself is not thread-safe, so acquiring an exclusive lock on access to Pdfium is the
// only way to guarantee thread safety. We acquire the lock on the first call to FPDF_InitLibrary(),
// and release the lock on the last call to FPDF_DestroyLibrary().

use crate::bindgen::{
    size_t, FPDFANNOT_COLORTYPE, FPDF_ACTION, FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE,
    FPDF_ANNOT_APPEARANCEMODE, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL, FPDF_DEST, FPDF_DOCUMENT,
    FPDF_DUPLEXTYPE, FPDF_DWORD, FPDF_FILEACCESS, FPDF_FILEWRITE, FPDF_FONT, FPDF_FORMFILLINFO,
    FPDF_FORMHANDLE, FPDF_GLYPHPATH, FPDF_IMAGEOBJ_METADATA, FPDF_LINK, FPDF_OBJECT_TYPE,
    FPDF_PAGE, FPDF_PAGEOBJECT, FPDF_PAGEOBJECTMARK, FPDF_PAGERANGE, FPDF_PATHSEGMENT,
    FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR, FPDF_WIDESTRING, FS_MATRIX, FS_POINTF,
    FS_QUADPOINTSF, FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref PDFIUM_THREAD_MARSHALL: Mutex<PdfiumThreadMarshall> =
        Mutex::new(PdfiumThreadMarshall::new());
}

struct PdfiumThreadMarshall {}

impl PdfiumThreadMarshall {
    #[inline]
    fn new() -> Self {
        PdfiumThreadMarshall {}
    }

    /// Returns exclusive read-write access to the global [PdfiumThreadMarshall] singleton.
    /// The currently running thread will block until the lock is acquired.
    /// Once this thread acquires the lock, all other threads will block until the lock is released.
    #[inline]
    fn lock() -> MutexGuard<'static, PdfiumThreadMarshall> {
        match PDFIUM_THREAD_MARSHALL.lock() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!(
                    "PdfiumThreadMarshall::lock(): unable to acquire thread lock: {:#?}",
                    err
                );
                log::error!("This may indicate a programming error in pdfium-render. Please file an issue: https://github.com/ajrcarey/pdfium-render/issues");

                panic!()
            }
        }
    }
}

impl Default for PdfiumThreadMarshall {
    #[inline]
    fn default() -> Self {
        PdfiumThreadMarshall::new()
    }
}

pub(crate) struct ThreadSafePdfiumBindings<T: PdfiumLibraryBindings> {
    bindings: T,
    lock: RefCell<Option<MutexGuard<'static, PdfiumThreadMarshall>>>,
}

impl<T: PdfiumLibraryBindings> ThreadSafePdfiumBindings<T> {
    #[inline]
    pub fn new(single_threaded_bindings: T) -> Self {
        ThreadSafePdfiumBindings {
            bindings: single_threaded_bindings,
            lock: RefCell::new(None),
        }
    }
}

impl<T: PdfiumLibraryBindings> PdfiumLibraryBindings for ThreadSafePdfiumBindings<T> {
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self) {
        // Take an exclusive lock over access to Pdfium. Any other thread attempting to
        // use Pdfium will block.

        if self.lock.borrow().is_none() {
            self.lock.replace(Some(PdfiumThreadMarshall::lock()));
            self.bindings.FPDF_InitLibrary();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self) {
        // Release the exclusive lock we hold over access to Pdfium. Any other thread waiting
        // to use Pdfium will be able to continue.

        if self.lock.borrow().is_some() {
            self.bindings.FPDF_DestroyLibrary();
            self.lock.replace(None);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong {
        self.bindings.FPDF_GetLastError()
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        self.bindings.FPDF_CreateNewDocument()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT {
        self.bindings.FPDF_LoadDocument(file_path, password)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument64(&self, data_buf: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        self.bindings.FPDF_LoadMemDocument64(data_buf, password)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: *mut FPDF_FILEACCESS,
        password: Option<&str>,
    ) -> FPDF_DOCUMENT {
        self.bindings.FPDF_LoadCustomDocument(pFileAccess, password)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL {
        self.bindings.FPDF_SaveAsCopy(document, pFileWrite, flags)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SaveWithVersion(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: c_int,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDF_SaveWithVersion(document, pFileWrite, flags, fileVersion)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        self.bindings.FPDF_CloseDocument(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL {
        self.bindings.FPDF_GetFileVersion(doc, fileVersion)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int {
        self.bindings.FPDF_GetFormType(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetMetaText(
        &self,
        document: FPDF_DOCUMENT,
        tag: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDF_GetMetaText(document, tag, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> c_ulong {
        self.bindings.FPDF_GetDocPermissions(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSecurityHandlerRevision(&self, document: FPDF_DOCUMENT) -> c_int {
        self.bindings.FPDF_GetSecurityHandlerRevision(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int {
        self.bindings.FPDF_GetPageCount(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE {
        self.bindings.FPDF_LoadPage(document, page_index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        self.bindings.FPDF_ClosePage(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportPagesByIndex(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDF_ImportPagesByIndex(dest_doc, src_doc, page_indices, length, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportPages(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: &str,
        index: c_int,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDF_ImportPages(dest_doc, src_doc, pagerange, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ImportNPagesToOne(
        &self,
        src_doc: FPDF_DOCUMENT,
        output_width: c_float,
        output_height: c_float,
        num_pages_on_x_axis: size_t,
        num_pages_on_y_axis: size_t,
    ) -> FPDF_DOCUMENT {
        self.bindings.FPDF_ImportNPagesToOne(
            src_doc,
            output_width,
            output_height,
            num_pages_on_x_axis,
            num_pages_on_y_axis,
        )
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> c_float {
        self.bindings.FPDF_GetPageWidthF(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> c_float {
        self.bindings.FPDF_GetPageHeightF(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageLabel(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDF_GetPageLabel(document, page_index, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_New(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: c_double,
        height: c_double,
    ) -> FPDF_PAGE {
        self.bindings
            .FPDFPage_New(document, page_index, width, height)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: c_int) {
        self.bindings.FPDFPage_Delete(document, page_index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int {
        self.bindings.FPDFPage_GetRotation(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int) {
        self.bindings.FPDFPage_SetRotation(page, rotate)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL {
        self.bindings.FPDF_GetPageBoundingBox(page, rect)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPage_GetMediaBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetCropBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPage_GetCropBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPage_GetBleedBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPage_GetTrimBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetArtBox(
        &self,
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPage_GetArtBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetMediaBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        self.bindings
            .FPDFPage_SetMediaBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetCropBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        self.bindings
            .FPDFPage_SetCropBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetBleedBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        self.bindings
            .FPDFPage_SetBleedBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetTrimBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        self.bindings
            .FPDFPage_SetTrimBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetArtBox(
        &self,
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ) {
        self.bindings
            .FPDFPage_SetArtBox(page, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        self.bindings.FPDFPage_HasTransparency(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        self.bindings.FPDFPage_GenerateContent(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_CreateEx(
        &self,
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP {
        self.bindings
            .FPDFBitmap_CreateEx(width, height, format, first_scan, stride)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        self.bindings.FPDFBitmap_Destroy(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> c_int {
        self.bindings.FPDFBitmap_GetFormat(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_FillRect(
        &self,
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    ) {
        self.bindings
            .FPDFBitmap_FillRect(bitmap, left, top, width, height, color)
    }

    #[inline]
    #[allow(non_snake_case)]
    #[cfg(not(target_arch = "wasm32"))]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void {
        self.bindings.FPDFBitmap_GetBuffer(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    #[cfg(target_arch = "wasm32")]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *const c_void {
        self.bindings.FPDFBitmap_GetBuffer(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_SetBuffer(&self, bitmap: FPDF_BITMAP, buffer: &[u8]) -> bool {
        self.bindings.FPDFBitmap_SetBuffer(bitmap, buffer)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int {
        self.bindings.FPDFBitmap_GetWidth(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int {
        self.bindings.FPDFBitmap_GetHeight(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int {
        self.bindings.FPDFBitmap_GetStride(bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
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
    ) {
        self.bindings.FPDF_RenderPageBitmap(
            bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        )
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_IsSupportedSubtype(subtype)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION {
        self.bindings.FPDFPage_CreateAnnot(page, subtype)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> c_int {
        self.bindings.FPDFPage_GetAnnotCount(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION {
        self.bindings.FPDFPage_GetAnnot(page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int {
        self.bindings.FPDFPage_GetAnnotIndex(page, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) {
        self.bindings.FPDFPage_CloseAnnot(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL {
        self.bindings.FPDFPage_RemoveAnnot(page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE {
        self.bindings.FPDFAnnot_GetSubtype(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_IsObjectSupportedSubtype(subtype)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_UpdateObject(annot, obj)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int {
        self.bindings
            .FPDFAnnot_AddInkStroke(annot, points, point_count)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_RemoveInkList(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_AppendObject(annot, obj)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> c_int {
        self.bindings.FPDFAnnot_GetObjectCount(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT {
        self.bindings.FPDFAnnot_GetObject(annot, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_RemoveObject(annot, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetColor(
        &self,
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_SetColor(annot, color_type, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetColor(
        &self,
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_GetColor(annot, color_type, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_HasAttachmentPoints(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_SetAttachmentPoints(annot, quad_index, quad_points)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_AppendAttachmentPoints(annot, quad_points)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> size_t {
        self.bindings.FPDFAnnot_CountAttachmentPoints(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_GetAttachmentPoints(annot, quad_index, quad_points)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_SetRect(annot, rect)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_GetRect(annot, rect)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        self.bindings.FPDFAnnot_GetVertices(annot, buffer, length)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> c_ulong {
        self.bindings.FPDFAnnot_GetInkListCount(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListPath(
        &self,
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetInkListPath(annot, path_index, buffer, length)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_GetLine(annot, start, end)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: c_float,
        vertical_radius: c_float,
        border_width: c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_SetBorder(annot, horizontal_radius, vertical_radius, border_width)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut c_float,
        vertical_radius: *mut c_float,
        border_width: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_GetBorder(annot, horizontal_radius, vertical_radius, border_width)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasKey(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_HasKey(annot, key)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_OBJECT_TYPE {
        self.bindings.FPDFAnnot_GetValueType(annot, key)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_SetStringValue(annot, key, value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetStringValue(annot, key, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetNumberValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_GetNumberValue(annot, key, value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_SetAP(annot, appearanceMode, value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetAP(annot, appearanceMode, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_ANNOTATION {
        self.bindings.FPDFAnnot_GetLinkedAnnot(annot, key)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> c_int {
        self.bindings.FPDFAnnot_GetFlags(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_SetFlags(annot, flags)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        self.bindings.FPDFAnnot_GetFormFieldFlags(handle, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION {
        self.bindings
            .FPDFAnnot_GetFormFieldAtPoint(hHandle, page, point)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldName(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetFormFieldName(hHandle, annot, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldType(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        self.bindings.FPDFAnnot_GetFormFieldType(hHandle, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetFormFieldValue(hHandle, annot, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int {
        self.bindings.FPDFAnnot_GetOptionCount(hHandle, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionLabel(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetOptionLabel(hHandle, annot, index, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_IsOptionSelected(handle, annot, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_GetFontSize(hHandle, annot, value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_IsChecked(hHandle, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_SetFocusableSubtypes(hHandle, subtypes, count)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> c_int {
        self.bindings.FPDFAnnot_GetFocusableSubtypesCount(hHandle)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFAnnot_GetFocusableSubtypes(hHandle, subtypes, count)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK {
        self.bindings.FPDFAnnot_GetLink(annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        self.bindings.FPDFAnnot_GetFormControlCount(hHandle, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        self.bindings.FPDFAnnot_GetFormControlIndex(hHandle, annot)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldExportValue(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAnnot_GetFormFieldExportValue(hHandle, annot, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: &str) -> FPDF_BOOL {
        self.bindings.FPDFAnnot_SetURI(annot, uri)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        self.bindings
            .FPDFDOC_InitFormFillEnvironment(document, form_info)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE) {
        self.bindings.FPDFDOC_ExitFormFillEnvironment(handle)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int {
        self.bindings.FPDFDoc_GetPageMode(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_Flatten(&self, page: FPDF_PAGE, nFlag: c_int) -> c_int {
        self.bindings.FPDFPage_Flatten(page, nFlag)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    ) {
        self.bindings
            .FPDF_SetFormFieldHighlightColor(handle, field_type, color)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar) {
        self.bindings.FPDF_SetFormFieldHighlightAlpha(handle, alpha)
    }

    #[inline]
    #[allow(non_snake_case)]
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
    ) {
        self.bindings.FPDF_FFLDraw(
            handle, bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
        )
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        self.bindings.FPDFBookmark_GetFirstChild(document, bookmark)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        self.bindings
            .FPDFBookmark_GetNextSibling(document, bookmark)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFBookmark_GetTitle(bookmark, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK {
        self.bindings.FPDFBookmark_Find(document, title)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST {
        self.bindings.FPDFBookmark_GetDest(document, bookmark)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION {
        self.bindings.FPDFBookmark_GetAction(bookmark)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong {
        self.bindings.FPDFAction_GetType(action)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST {
        self.bindings.FPDFAction_GetDest(document, action)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings.FPDFAction_GetFilePath(action, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetURIPath(
        &self,
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFAction_GetURIPath(document, action, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int {
        self.bindings.FPDFDest_GetDestPageIndex(document, dest)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE {
        self.bindings.FPDFText_LoadPage(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        self.bindings.FPDFText_ClosePage(text_page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int {
        self.bindings.FPDFText_CountChars(text_page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetUnicode(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint {
        self.bindings.FPDFText_GetUnicode(text_page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontSize(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_double {
        self.bindings.FPDFText_GetFontSize(text_page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontInfo(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        flags: *mut c_int,
    ) -> c_ulong {
        self.bindings
            .FPDFText_GetFontInfo(text_page, index, buffer, buflen, flags)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontWeight(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_int {
        self.bindings.FPDFText_GetFontWeight(text_page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextRenderMode(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
    ) -> FPDF_TEXT_RENDERMODE {
        self.bindings.FPDFText_GetTextRenderMode(text_page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFillColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFText_GetFillColor(text_page, index, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetStrokeColor(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFText_GetStrokeColor(text_page, index, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharAngle(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_float {
        self.bindings.FPDFText_GetCharAngle(text_page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        left: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
        top: *mut c_double,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFText_GetCharBox(text_page, index, left, right, bottom, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetLooseCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFText_GetLooseCharBox(text_page, index, rect)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetMatrix(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        self.bindings.FPDFText_GetMatrix(text_page, index, matrix)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharOrigin(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        x: *mut c_double,
        y: *mut c_double,
    ) -> FPDF_BOOL {
        self.bindings.FPDFText_GetCharOrigin(text_page, index, x, y)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexAtPos(
        &self,
        text_page: FPDF_TEXTPAGE,
        x: c_double,
        y: c_double,
        xTolerance: c_double,
        yTolerance: c_double,
    ) -> c_int {
        self.bindings
            .FPDFText_GetCharIndexAtPos(text_page, x, y, xTolerance, yTolerance)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetText(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
        result: *mut c_ushort,
    ) -> c_int {
        self.bindings
            .FPDFText_GetText(text_page, start_index, count, result)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountRects(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
    ) -> c_int {
        self.bindings
            .FPDFText_CountRects(text_page, start_index, count)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetRect(
        &self,
        text_page: FPDF_TEXTPAGE,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFText_GetRect(text_page, rect_index, left, top, right, bottom)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetBoundedText(
        &self,
        text_page: FPDF_TEXTPAGE,
        left: c_double,
        top: c_double,
        right: c_double,
        bottom: c_double,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int {
        self.bindings
            .FPDFText_GetBoundedText(text_page, left, top, right, bottom, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFFormObj_CountObjects(form_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECT {
        self.bindings.FPDFFormObj_GetObject(form_object, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        self.bindings
            .FPDFPageObj_CreateTextObj(document, font, font_size)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE {
        self.bindings.FPDFTextObj_GetTextRenderMode(text)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFTextObj_SetTextRenderMode(text, render_mode)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetText(
        &self,
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFTextObj_GetText(text_object, text_page, buffer, length)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT {
        self.bindings.FPDFTextObj_GetFont(text)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL {
        self.bindings.FPDFTextObj_GetFontSize(text, size)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_Close(&self, font: FPDF_FONT) {
        self.bindings.FPDFFont_Close(font)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        self.bindings.FPDFPath_MoveTo(path, x, y)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        self.bindings.FPDFPath_LineTo(path, x, y)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_BezierTo(
        &self,
        path: FPDF_PAGEOBJECT,
        x1: c_float,
        y1: c_float,
        x2: c_float,
        y2: c_float,
        x3: c_float,
        y3: c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPath_BezierTo(path, x1, y1, x2, y2, x3, y3)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        self.bindings.FPDFPath_Close(path)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_SetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPath_SetDrawMode(path, fillmode, stroke)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPath_GetDrawMode(path, fillmode, stroke)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: &str,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        self.bindings
            .FPDFPageObj_NewTextObj(document, font, font_size)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL {
        self.bindings.FPDFText_SetText(text_object, text)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFText_SetCharcodes(text_object, charcodes, count)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadFont(
        &self,
        document: FPDF_DOCUMENT,
        data: *const c_uchar,
        size: c_uint,
        font_type: c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT {
        self.bindings
            .FPDFText_LoadFont(document, data, size, font_type, cid)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadStandardFont(&self, document: FPDF_DOCUMENT, font: &str) -> FPDF_FONT {
        self.bindings.FPDFText_LoadStandardFont(document, font)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) {
        self.bindings.FPDFPage_InsertObject(page, page_obj)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        self.bindings.FPDFPage_RemoveObject(page, page_obj)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> c_int {
        self.bindings.FPDFPage_CountObjects(page)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT {
        self.bindings.FPDFPage_GetObject(page, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Destroy(&self, page_obj: FPDF_PAGEOBJECT) {
        self.bindings.FPDFPageObj_Destroy(page_obj)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_HasTransparency(page_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFPageObj_GetType(page_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Transform(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) {
        self.bindings
            .FPDFPageObj_Transform(page_object, a, b, c, d, e, f)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_GetMatrix(page_object, matrix)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetMatrix(&self, path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_SetMatrix(path, matrix)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT {
        self.bindings.FPDFPageObj_NewImageObj(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFPageObj_CountMarks(page_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECTMARK {
        self.bindings.FPDFPageObj_GetMark(page_object, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_AddMark(&self, page_object: FPDF_PAGEOBJECT, name: &str) -> FPDF_PAGEOBJECTMARK {
        self.bindings.FPDFPageObj_AddMark(page_object, name)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_RemoveMark(page_object, mark)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetName(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_GetName(mark, buffer, buflen, out_buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> c_int {
        self.bindings.FPDFPageObjMark_CountParams(mark)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamKey(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        index: c_ulong,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_GetParamKey(mark, index, buffer, buflen, out_buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        self.bindings.FPDFPageObjMark_GetParamValueType(mark, key)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        out_value: *mut c_int,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_GetParamIntValue(mark, key, out_value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_GetParamStringValue(mark, key, buffer, buflen, out_buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_GetParamBlobValue(mark, key, buffer, buflen, out_buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetIntParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: c_int,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_SetIntParam(document, page_object, mark, key, value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetStringParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: &str,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_SetStringParam(document, page_object, mark, key, value)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPageObjMark_SetBlobParam(
            document,
            page_object,
            mark,
            key,
            value,
            value_len,
        )
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObjMark_RemoveParam(page_object, mark, key)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFile(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFImageObj_LoadJpegFile(pages, count, image_object, file_access)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_LoadJpegFileInline(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFImageObj_LoadJpegFileInline(pages, count, image_object, file_access)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_SetMatrix(
        &self,
        image_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) -> FPDF_BOOL {
        #[allow(deprecated)]
        self.bindings
            .FPDFImageObj_SetMatrix(image_object, a, b, c, d, e, f)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_SetBitmap(
        &self,
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFImageObj_SetBitmap(pages, count, image_object, bitmap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP {
        self.bindings.FPDFImageObj_GetBitmap(image_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        self.bindings
            .FPDFImageObj_GetRenderedBitmap(document, page, image_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFImageObj_GetImageDataDecoded(image_object, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFImageObj_GetImageDataRaw(image_object, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFImageObj_GetImageFilterCount(image_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilter(
        &self,
        image_object: FPDF_PAGEOBJECT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDFImageObj_GetImageFilter(image_object, index, buffer, buflen)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFImageObj_GetImageMetadata(image_object, page, metadata)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewPath(&self, x: c_float, y: c_float) -> FPDF_PAGEOBJECT {
        self.bindings.FPDFPageObj_CreateNewPath(x, y)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewRect(
        &self,
        x: c_float,
        y: c_float,
        w: c_float,
        h: c_float,
    ) -> FPDF_PAGEOBJECT {
        self.bindings.FPDFPageObj_CreateNewRect(x, y, w, h)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetBounds(
        &self,
        page_object: FPDF_PAGEOBJECT,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_GetBounds(page_object, left, bottom, right, top)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: &str) {
        self.bindings
            .FPDFPageObj_SetBlendMode(page_object, blend_mode)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_SetStrokeColor(page_object, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_GetStrokeColor(page_object, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_SetStrokeWidth(page_object, width)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_GetStrokeWidth(page_object, width)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFPageObj_GetLineJoin(page_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_SetLineJoin(page_object, line_join)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFPageObj_GetLineCap(page_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_SetLineCap(page_object, line_cap)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_SetFillColor(page_object, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetFillColor(
        &self,
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_GetFillColor(page_object, R, G, B, A)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_GetDashPhase(page_object, phase)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL {
        self.bindings.FPDFPageObj_SetDashPhase(page_object, phase)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        self.bindings.FPDFPageObj_GetDashCount(page_object)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_GetDashArray(page_object, dash_array, dash_count)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const c_float,
        dash_count: size_t,
        phase: c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFPageObj_SetDashArray(page_object, dash_array, dash_count, phase)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        self.bindings.FPDFFont_GetFontName(font, buffer, length)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> c_int {
        self.bindings.FPDFFont_GetFlags(font)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> c_int {
        self.bindings.FPDFFont_GetWeight(font)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetItalicAngle(&self, font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL {
        self.bindings.FPDFFont_GetItalicAngle(font, angle)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetAscent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        ascent: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFFont_GetAscent(font, font_size, ascent)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetDescent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        descent: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings.FPDFFont_GetDescent(font, font_size, descent)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphWidth(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        self.bindings
            .FPDFFont_GetGlyphWidth(font, glyph, font_size, width)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphPath(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
    ) -> FPDF_GLYPHPATH {
        self.bindings.FPDFFont_GetGlyphPath(font, glyph, font_size)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: FPDF_GLYPHPATH) -> c_int {
        self.bindings.FPDFGlyphPath_CountGlyphSegments(glyphpath)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: FPDF_GLYPHPATH,
        index: c_int,
    ) -> FPDF_PATHSEGMENT {
        self.bindings
            .FPDFGlyphPath_GetGlyphPathSegment(glyphpath, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        self.bindings.FPDF_VIEWERREF_GetPrintScaling(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetNumCopies(&self, document: FPDF_DOCUMENT) -> c_int {
        self.bindings.FPDF_VIEWERREF_GetNumCopies(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRange(&self, document: FPDF_DOCUMENT) -> FPDF_PAGERANGE {
        self.bindings.FPDF_VIEWERREF_GetPrintPageRange(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> size_t {
        self.bindings
            .FPDF_VIEWERREF_GetPrintPageRangeCount(pagerange)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: FPDF_PAGERANGE,
        index: size_t,
    ) -> c_int {
        self.bindings
            .FPDF_VIEWERREF_GetPrintPageRangeElement(pagerange, index)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE {
        self.bindings.FPDF_VIEWERREF_GetDuplex(document)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetName(
        &self,
        document: FPDF_DOCUMENT,
        key: &str,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        self.bindings
            .FPDF_VIEWERREF_GetName(document, key, buffer, length)
    }
}
