use crate::bindgen::{
    size_t, FPDF_ACTION, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL, FPDF_BYTESTRING, FPDF_DEST,
    FPDF_DOCUMENT, FPDF_DWORD, FPDF_FILEACCESS, FPDF_FONT, FPDF_FORMFILLINFO, FPDF_FORMHANDLE,
    FPDF_IMAGEOBJ_METADATA, FPDF_OBJECT_TYPE, FPDF_PAGE, FPDF_PAGEOBJECT, FPDF_PAGEOBJECTMARK,
    FPDF_STRING, FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR, FPDF_WIDESTRING, FS_MATRIX,
    FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use libloading::{Library, Symbol};
use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort};

pub(crate) struct NativePdfiumBindings {
    library: Library,
}

impl NativePdfiumBindings {
    pub fn new(library: Library) -> Result<Self, libloading::Error> {
        let result = NativePdfiumBindings { library };

        // Make sure the library correctly exports all the functions we expect.

        result.extern_FPDF_InitLibrary()?;
        result.extern_FPDF_DestroyLibrary()?;
        result.extern_FPDF_GetLastError()?;
        result.extern_FPDF_LoadDocument()?;
        result.extern_FPDF_LoadMemDocument()?;
        result.extern_FPDF_CloseDocument()?;
        result.extern_FPDF_GetFileVersion()?;
        result.extern_FPDF_GetFormType()?;
        result.extern_FPDF_GetMetaText()?;
        result.extern_FPDF_GetPageCount()?;
        result.extern_FPDF_LoadPage()?;
        result.extern_FPDF_ClosePage()?;
        result.extern_FPDF_GetPageLabel()?;
        result.extern_FPDF_GetPageBoundingBox()?;
        result.extern_FPDF_GetPageWidthF()?;
        result.extern_FPDF_GetPageHeightF()?;
        result.extern_FPDFPage_GetRotation()?;
        result.extern_FPDFPage_SetRotation()?;
        result.extern_FPDFPage_GetMediaBox()?;
        result.extern_FPDFPage_GetCropBox()?;
        result.extern_FPDFPage_GetBleedBox()?;
        result.extern_FPDFPage_GetTrimBox()?;
        result.extern_FPDFPage_GetArtBox()?;
        result.extern_FPDFPage_SetMediaBox()?;
        result.extern_FPDFPage_SetCropBox()?;
        result.extern_FPDFPage_SetBleedBox()?;
        result.extern_FPDFPage_SetTrimBox()?;
        result.extern_FPDFPage_SetArtBox()?;
        result.extern_FPDFPage_HasTransparency()?;
        result.extern_FPDFBitmap_CreateEx()?;
        result.extern_FPDFBitmap_Destroy()?;
        result.extern_FPDFBitmap_FillRect()?;
        result.extern_FPDFBitmap_GetBuffer()?;
        result.extern_FPDFBitmap_GetWidth()?;
        result.extern_FPDFBitmap_GetHeight()?;
        result.extern_FPDFBitmap_GetStride()?;
        result.extern_FPDF_RenderPageBitmap()?;
        result.extern_FPDFDOC_InitFormFillEnvironment()?;
        result.extern_FPDFDOC_ExitFormFillEnvironment()?;
        result.extern_FPDFDoc_GetPageMode()?;
        result.extern_FPDF_SetFormFieldHighlightColor()?;
        result.extern_FPDF_SetFormFieldHighlightAlpha()?;
        result.extern_FPDF_FFLDraw()?;
        result.extern_FPDFBookmark_GetFirstChild()?;
        result.extern_FPDFBookmark_GetNextSibling()?;
        result.extern_FPDFBookmark_GetTitle()?;
        result.extern_FPDFBookmark_Find()?;
        result.extern_FPDFBookmark_GetDest()?;
        result.extern_FPDFBookmark_GetAction()?;
        result.extern_FPDFAction_GetType()?;
        result.extern_FPDFAction_GetDest()?;
        result.extern_FPDFAction_GetFilePath()?;
        result.extern_FPDFAction_GetURIPath()?;
        result.extern_FPDFDest_GetDestPageIndex()?;
        result.extern_FPDFText_LoadPage()?;
        result.extern_FPDFText_ClosePage()?;
        result.extern_FPDFText_CountChars()?;
        result.extern_FPDFText_GetBoundedText()?;
        result.extern_FPDFFormObj_CountObjects()?;
        result.extern_FPDFFormObj_GetObject()?;
        result.extern_FPDFPageObj_CreateTextObj()?;
        result.extern_FPDFTextObj_GetTextRenderMode()?;
        result.extern_FPDFTextObj_SetTextRenderMode()?;
        result.extern_FPDFTextObj_GetText()?;
        result.extern_FPDFTextObj_GetFont()?;
        result.extern_FPDFTextObj_GetFontSize()?;
        result.extern_FPDFPageObj_NewTextObj()?;
        result.extern_FPDFText_SetText()?;
        result.extern_FPDFText_SetCharcodes()?;
        result.extern_FPDFPage_InsertObject()?;
        result.extern_FPDFPage_RemoveObject()?;
        result.extern_FPDFPage_CountObjects()?;
        result.extern_FPDFPage_GetObject()?;
        result.extern_FPDFPageObj_Destroy()?;
        result.extern_FPDFPageObj_HasTransparency()?;
        result.extern_FPDFPageObj_GetType()?;
        result.extern_FPDFPageObj_Transform()?;
        result.extern_FPDFPageObj_GetMatrix()?;
        result.extern_FPDFPageObj_SetMatrix()?;
        result.extern_FPDFPageObj_NewImageObj()?;
        result.extern_FPDFPageObj_CountMarks()?;
        result.extern_FPDFPageObj_GetMark()?;
        result.extern_FPDFPageObj_AddMark()?;
        result.extern_FPDFPageObj_RemoveMark()?;
        result.extern_FPDFPageObjMark_GetName()?;
        result.extern_FPDFPageObjMark_CountParams()?;
        result.extern_FPDFPageObjMark_GetParamKey()?;
        result.extern_FPDFPageObjMark_GetParamValueType()?;
        result.extern_FPDFPageObjMark_GetParamIntValue()?;
        result.extern_FPDFPageObjMark_GetParamStringValue()?;
        result.extern_FPDFPageObjMark_GetParamBlobValue()?;
        result.extern_FPDFPageObjMark_SetIntParam()?;
        result.extern_FPDFPageObjMark_SetStringParam()?;
        result.extern_FPDFPageObjMark_SetBlobParam()?;
        result.extern_FPDFPageObjMark_RemoveParam()?;
        result.extern_FPDFImageObj_LoadJpegFile()?;
        result.extern_FPDFImageObj_LoadJpegFileInline()?;
        result.extern_FPDFImageObj_SetMatrix()?;
        result.extern_FPDFImageObj_SetBitmap()?;
        result.extern_FPDFImageObj_GetBitmap()?;
        result.extern_FPDFImageObj_GetRenderedBitmap()?;
        result.extern_FPDFImageObj_GetImageDataDecoded()?;
        result.extern_FPDFImageObj_GetImageDataRaw()?;
        result.extern_FPDFImageObj_GetImageFilterCount()?;
        result.extern_FPDFImageObj_GetImageFilter()?;
        result.extern_FPDFImageObj_GetImageMetadata()?;
        result.extern_FPDFPageObj_CreateNewPath()?;
        result.extern_FPDFPageObj_CreateNewRect()?;
        result.extern_FPDFPageObj_GetBounds()?;
        result.extern_FPDFPageObj_SetBlendMode()?;
        result.extern_FPDFPageObj_SetStrokeColor()?;
        result.extern_FPDFPageObj_GetStrokeColor()?;
        result.extern_FPDFPageObj_SetStrokeWidth()?;
        result.extern_FPDFPageObj_GetStrokeWidth()?;
        result.extern_FPDFPageObj_GetLineJoin()?;
        result.extern_FPDFPageObj_SetLineJoin()?;
        result.extern_FPDFPageObj_GetLineCap()?;
        result.extern_FPDFPageObj_SetLineCap()?;
        result.extern_FPDFPageObj_SetFillColor()?;
        result.extern_FPDFPageObj_GetFillColor()?;
        result.extern_FPDFPageObj_GetDashPhase()?;
        result.extern_FPDFPageObj_SetDashPhase()?;
        result.extern_FPDFPageObj_GetDashCount()?;
        result.extern_FPDFPageObj_GetDashArray()?;
        result.extern_FPDFPageObj_SetDashArray()?;
        result.extern_FPDFFont_GetFontName()?;

        Ok(result)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_InitLibrary(&self) -> Result<Symbol<unsafe extern "C" fn()>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_InitLibrary") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_DestroyLibrary(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn()>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_DestroyLibrary") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetLastError(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn() -> c_ulong>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetLastError") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadDocument(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                file_path: FPDF_STRING,
                password: FPDF_BYTESTRING,
            ) -> FPDF_DOCUMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_LoadDocument") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadMemDocument(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                data_buf: *const c_void,
                size: c_int,
                password: FPDF_BYTESTRING,
            ) -> FPDF_DOCUMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_LoadMemDocument") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_CloseDocument(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT)>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_CloseDocument") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetFileVersion(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetFileVersion") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetMetaText(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                tag: FPDF_BYTESTRING,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetMetaText") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetPageCount") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadPage(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_LoadPage") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_ClosePage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_ClosePage") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageLabel(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page_index: c_int,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetPageLabel") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageBoundingBox(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetPageBoundingBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageWidthF(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_float>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetPageWidthF") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageHeightF(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_float>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetPageHeightF") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetRotation(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_GetRotation") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetRotation(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE, rotate: c_int)>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPage_SetRotation") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetMediaBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: *mut c_float,
                bottom: *mut c_float,
                right: *mut c_float,
                top: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetMediaBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetCropBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: *mut c_float,
                bottom: *mut c_float,
                right: *mut c_float,
                top: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetCropBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetBleedBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: *mut c_float,
                bottom: *mut c_float,
                right: *mut c_float,
                top: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetBleedBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetTrimBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: *mut c_float,
                bottom: *mut c_float,
                right: *mut c_float,
                top: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetTrimBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetArtBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: *mut c_float,
                bottom: *mut c_float,
                right: *mut c_float,
                top: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetArtBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetMediaBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: c_float,
                bottom: c_float,
                right: c_float,
                top: c_float,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_SetMediaBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetCropBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: c_float,
                bottom: c_float,
                right: c_float,
                top: c_float,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_SetCropBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetBleedBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: c_float,
                bottom: c_float,
                right: c_float,
                top: c_float,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_SetBleedBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetTrimBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: c_float,
                bottom: c_float,
                right: c_float,
                top: c_float,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_SetTrimBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetArtBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                left: c_float,
                bottom: c_float,
                right: c_float,
                top: c_float,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_SetArtBox") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_HasTransparency(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_HasTransparency") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_CreateEx(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                width: c_int,
                height: c_int,
                format: c_int,
                first_scan: *mut c_void,
                stride: c_int,
            ) -> FPDF_BITMAP,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBitmap_CreateEx") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_Destroy(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_Destroy") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_FillRect(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                bitmap: FPDF_BITMAP,
                left: c_int,
                top: c_int,
                width: c_int,
                height: c_int,
                color: FPDF_DWORD,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBitmap_FillRect") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetBuffer(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut c_void>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFBitmap_GetBuffer") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetWidth(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetWidth") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetHeight(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetHeight") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetStride(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetStride") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDF_RenderPageBitmap(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                bitmap: FPDF_BITMAP,
                page: FPDF_PAGE,
                start_x: c_int,
                start_y: c_int,
                size_x: c_int,
                size_y: c_int,
                rotate: c_int,
                flags: c_int,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_RenderPageBitmap") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDOC_InitFormFillEnvironment(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                form_info: *mut FPDF_FORMFILLINFO,
            ) -> FPDF_FORMHANDLE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDOC_InitFormFillEnvironment") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDOC_ExitFormFillEnvironment(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFDOC_ExitFormFillEnvironment") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDoc_GetPageMode(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFDoc_GetPageMode") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_SetFormFieldHighlightColor(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE, field_type: c_int, color: c_ulong)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_SetFormFieldHighlightColor") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_SetFormFieldHighlightAlpha(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE, alpha: c_uchar)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_SetFormFieldHighlightAlpha") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDF_FFLDraw(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                handle: FPDF_FORMHANDLE,
                bitmap: FPDF_BITMAP,
                page: FPDF_PAGE,
                start_x: c_int,
                start_y: c_int,
                size_x: c_int,
                size_y: c_int,
                rotate: c_int,
                flags: c_int,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_FFLDraw") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetFormType(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetFormType") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetFirstChild(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetFirstChild") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetNextSibling(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetNextSibling") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetTitle(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                bookmark: FPDF_BOOKMARK,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetTitle") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_Find(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_Find") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetDest(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetDest") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetAction(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> FPDF_ACTION>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetAction") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAction_GetType(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(action: FPDF_ACTION) -> c_ulong>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAction_GetType") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAction_GetDest(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAction_GetDest") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAction_GetFilePath(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                action: FPDF_ACTION,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAction_GetFilePath") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAction_GetURIPath(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                action: FPDF_ACTION,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAction_GetURIPath") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDest_GetDestPageIndex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDest_GetDestPageIndex") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_LoadPage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_TEXTPAGE>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_LoadPage") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_ClosePage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFText_ClosePage") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_CountChars(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_CountChars") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDFText_GetBoundedText(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                left: f64,
                top: f64,
                right: f64,
                bottom: f64,
                buffer: *mut c_ushort,
                buflen: c_int,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetBoundedText") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFormObj_CountObjects(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFormObj_CountObjects") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFormObj_GetObject(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT, index: c_ulong) -> FPDF_PAGEOBJECT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFormObj_GetObject") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_CreateTextObj(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                font: FPDF_FONT,
                font_size: c_float,
            ) -> FPDF_PAGEOBJECT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_CreateTextObj") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetTextRenderMode(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFTextObj_GetTextRenderMode") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_SetTextRenderMode(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text: FPDF_PAGEOBJECT,
                render_mode: FPDF_TEXT_RENDERMODE,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFTextObj_SetTextRenderMode") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetText(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_object: FPDF_PAGEOBJECT,
                text_page: FPDF_TEXTPAGE,
                buffer: *mut FPDF_WCHAR,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFTextObj_GetText") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetFont(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_FONT>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFTextObj_GetFont") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetFontSize(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFTextObj_GetFontSize") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_NewTextObj(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                font: FPDF_BYTESTRING,
                font_size: c_float,
            ) -> FPDF_PAGEOBJECT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_NewTextObj") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_SetText(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_SetText") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_SetCharcodes(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_object: FPDF_PAGEOBJECT,
                charcodes: *const c_uint,
                count: size_t,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_SetCharcodes") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_InsertObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_InsertObject") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_RemoveObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_RemoveObject") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_CountObjects(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_CountObjects") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetObject") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_Destroy(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page_obj: FPDF_PAGEOBJECT)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPageObj_Destroy") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_HasTransparency(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_HasTransparency") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetType(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetType") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDFPageObj_Transform(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                a: c_double,
                b: c_double,
                c: c_double,
                d: c_double,
                e: c_double,
                f: c_double,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_Transform") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetMatrix(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *mut FS_MATRIX) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetMatrix") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetMatrix(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetMatrix") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_NewImageObj(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_NewImageObj") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_CountMarks(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_CountMarks") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetMark(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                index: c_ulong,
            ) -> FPDF_PAGEOBJECTMARK,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetMark") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_AddMark(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                name: FPDF_BYTESTRING,
            ) -> FPDF_PAGEOBJECTMARK,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_AddMark") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_RemoveMark(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                mark: FPDF_PAGEOBJECTMARK,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_RemoveMark") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_GetName(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                mark: FPDF_PAGEOBJECTMARK,
                buffer: *mut c_void,
                buflen: c_ulong,
                out_buflen: *mut c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_GetName") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_CountParams(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPageObjMark_CountParams") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_GetParamKey(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                mark: FPDF_PAGEOBJECTMARK,
                index: c_ulong,
                buffer: *mut c_void,
                buflen: c_ulong,
                out_buflen: *mut c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamKey") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_GetParamValueType(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
            ) -> FPDF_OBJECT_TYPE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamValueType") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_GetParamIntValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
                out_value: *mut c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamIntValue") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_GetParamStringValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
                buffer: *mut c_void,
                buflen: c_ulong,
                out_buflen: *mut c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamStringValue") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_GetParamBlobValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
                buffer: *mut c_void,
                buflen: c_ulong,
                out_buflen: *mut c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamBlobValue") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_SetIntParam(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page_object: FPDF_PAGEOBJECT,
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
                value: c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_SetIntParam") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_SetStringParam(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page_object: FPDF_PAGEOBJECT,
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
                value: FPDF_BYTESTRING,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_SetStringParam") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDFPageObjMark_SetBlobParam(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page_object: FPDF_PAGEOBJECT,
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
                value: *mut c_void,
                value_len: c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_SetBlobParam") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_RemoveParam(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                mark: FPDF_PAGEOBJECTMARK,
                key: FPDF_BYTESTRING,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObjMark_RemoveParam") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_LoadJpegFile(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                pages: *mut FPDF_PAGE,
                count: c_int,
                image_object: FPDF_PAGEOBJECT,
                file_access: *mut FPDF_FILEACCESS,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_LoadJpegFile") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_LoadJpegFileInline(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                pages: *mut FPDF_PAGE,
                count: c_int,
                image_object: FPDF_PAGEOBJECT,
                file_access: *mut FPDF_FILEACCESS,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_LoadJpegFileInline") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDFImageObj_SetMatrix(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                image_object: FPDF_PAGEOBJECT,
                a: c_double,
                b: c_double,
                c: c_double,
                d: c_double,
                e: c_double,
                f: c_double,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_SetMatrix") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_SetBitmap(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                pages: *mut FPDF_PAGE,
                count: c_int,
                image_object: FPDF_PAGEOBJECT,
                bitmap: FPDF_BITMAP,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_SetBitmap") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetBitmap(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetBitmap") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetRenderedBitmap(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page: FPDF_PAGE,
                image_object: FPDF_PAGEOBJECT,
            ) -> FPDF_BITMAP,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetRenderedBitmap") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetImageDataDecoded(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                image_object: FPDF_PAGEOBJECT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetImageDataDecoded") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetImageDataRaw(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                image_object: FPDF_PAGEOBJECT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetImageDataRaw") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetImageFilterCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetImageFilterCount") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetImageFilter(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                image_object: FPDF_PAGEOBJECT,
                index: c_int,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetImageFilter") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetImageMetadata(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                image_object: FPDF_PAGEOBJECT,
                page: FPDF_PAGE,
                metadata: *mut FPDF_IMAGEOBJ_METADATA,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetImageMetadata") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_CreateNewPath(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(x: c_float, y: c_float) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_CreateNewPath") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_CreateNewRect(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(x: c_float, y: c_float, w: c_float, h: c_float) -> FPDF_PAGEOBJECT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_CreateNewRect") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetBounds(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                left: *mut c_float,
                bottom: *mut c_float,
                right: *mut c_float,
                top: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetBounds") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetBlendMode(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetBlendMode") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetStrokeColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                R: c_uint,
                G: c_uint,
                B: c_uint,
                A: c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetStrokeColor") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetStrokeColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                R: *mut c_uint,
                G: *mut c_uint,
                B: *mut c_uint,
                A: *mut c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetStrokeColor") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetStrokeWidth(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetStrokeWidth") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetStrokeWidth(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: *mut c_float) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetStrokeWidth") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetLineJoin(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetLineJoin") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetLineJoin(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetLineJoin") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetLineCap(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetLineCap") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetLineCap(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetLineCap") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetFillColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                R: c_uint,
                G: c_uint,
                B: c_uint,
                A: c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetFillColor") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetFillColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                R: *mut c_uint,
                G: *mut c_uint,
                B: *mut c_uint,
                A: *mut c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetFillColor") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetDashPhase(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: *mut c_float) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetDashPhase") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetDashPhase(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetDashPhase") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetDashCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetDashCount") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetDashArray(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                dash_array: *mut c_float,
                dash_count: size_t,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetDashArray") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetDashArray(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                dash_array: *const c_float,
                dash_count: size_t,
                phase: c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetDashArray") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetFontName(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: c_ulong) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFont_GetFontName") }
    }
}

impl PdfiumLibraryBindings for NativePdfiumBindings {
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self) {
        unsafe {
            self.extern_FPDF_InitLibrary().unwrap()();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self) {
        unsafe {
            self.extern_FPDF_DestroyLibrary().unwrap()();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong {
        unsafe { self.extern_FPDF_GetLastError().unwrap()() }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT {
        let c_file_path = CString::new(file_path).unwrap();
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            self.extern_FPDF_LoadDocument().unwrap()(c_file_path.as_ptr(), c_password.as_ptr())
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            self.extern_FPDF_LoadMemDocument().unwrap()(
                bytes.as_ptr() as *const c_void,
                bytes.len() as c_int,
                c_password.as_ptr(),
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        unsafe {
            self.extern_FPDF_CloseDocument().unwrap()(document);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDF_GetFileVersion().unwrap()(doc, fileVersion) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDF_GetFormType().unwrap()(document) }
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
        let c_tag = CString::new(tag).unwrap();

        unsafe { self.extern_FPDF_GetMetaText().unwrap()(document, c_tag.as_ptr(), buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDF_GetPageCount().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE {
        unsafe { self.extern_FPDF_LoadPage().unwrap()(document, page_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        unsafe {
            self.extern_FPDF_ClosePage().unwrap()(page);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> c_float {
        unsafe { self.extern_FPDF_GetPageWidthF().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> c_float {
        unsafe { self.extern_FPDF_GetPageHeightF().unwrap()(page) }
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
        unsafe { self.extern_FPDF_GetPageLabel().unwrap()(document, page_index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int {
        unsafe { self.extern_FPDFPage_GetRotation().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int) {
        unsafe { self.extern_FPDFPage_SetRotation().unwrap()(page, rotate) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { self.extern_FPDF_GetPageBoundingBox().unwrap()(page, rect) }
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
        unsafe { self.extern_FPDFPage_GetMediaBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_GetCropBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_GetBleedBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_GetTrimBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_GetArtBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_SetMediaBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_SetCropBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_SetBleedBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_SetTrimBox().unwrap()(page, left, bottom, right, top) }
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
        unsafe { self.extern_FPDFPage_SetArtBox().unwrap()(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPage_HasTransparency().unwrap()(page) }
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
        unsafe {
            self.extern_FPDFBitmap_CreateEx().unwrap()(width, height, format, first_scan, stride)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        unsafe { self.extern_FPDFBitmap_Destroy().unwrap()(bitmap) }
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
        unsafe {
            self.extern_FPDFBitmap_FillRect().unwrap()(bitmap, left, top, width, height, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void {
        unsafe { self.extern_FPDFBitmap_GetBuffer().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { self.extern_FPDFBitmap_GetWidth().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { self.extern_FPDFBitmap_GetHeight().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { self.extern_FPDFBitmap_GetStride().unwrap()(bitmap) }
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
        unsafe {
            self.extern_FPDF_RenderPageBitmap().unwrap()(
                bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
            );
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        unsafe { self.extern_FPDFDOC_InitFormFillEnvironment().unwrap()(document, form_info) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE) {
        unsafe {
            self.extern_FPDFDOC_ExitFormFillEnvironment().unwrap()(handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDFDoc_GetPageMode().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightColor(
        &self,
        handle: FPDF_FORMHANDLE,
        field_type: c_int,
        color: FPDF_DWORD,
    ) {
        unsafe {
            self.extern_FPDF_SetFormFieldHighlightColor().unwrap()(handle, field_type, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar) {
        unsafe {
            self.extern_FPDF_SetFormFieldHighlightAlpha().unwrap()(handle, alpha);
        }
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
        unsafe {
            self.extern_FPDF_FFLDraw().unwrap()(
                handle, bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
            );
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetFirstChild(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        unsafe { self.extern_FPDFBookmark_GetFirstChild().unwrap()(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        unsafe { self.extern_FPDFBookmark_GetNextSibling().unwrap()(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFBookmark_GetTitle().unwrap()(bookmark, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK {
        unsafe { self.extern_FPDFBookmark_Find().unwrap()(document, title) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST {
        unsafe { self.extern_FPDFBookmark_GetDest().unwrap()(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION {
        unsafe { self.extern_FPDFBookmark_GetAction().unwrap()(bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong {
        unsafe { self.extern_FPDFAction_GetType().unwrap()(action) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST {
        unsafe { self.extern_FPDFAction_GetDest().unwrap()(document, action) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFAction_GetFilePath().unwrap()(action, buffer, buflen) }
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
        unsafe { self.extern_FPDFAction_GetURIPath().unwrap()(document, action, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int {
        unsafe { self.extern_FPDFDest_GetDestPageIndex().unwrap()(document, dest) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE {
        unsafe { self.extern_FPDFText_LoadPage().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        unsafe {
            self.extern_FPDFText_ClosePage().unwrap()(text_page);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int {
        unsafe { self.extern_FPDFText_CountChars().unwrap()(text_page) }
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
        unsafe {
            self.extern_FPDFText_GetBoundedText().unwrap()(
                text_page, left, top, right, bottom, buffer, buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFFormObj_CountObjects().unwrap()(form_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFFormObj_GetObject().unwrap()(form_object, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFPageObj_CreateTextObj().unwrap()(document, font, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE {
        unsafe { self.extern_FPDFTextObj_GetTextRenderMode().unwrap()(text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFTextObj_SetTextRenderMode().unwrap()(text, render_mode) }
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
        unsafe {
            self.extern_FPDFTextObj_GetText().unwrap()(text_object, text_page, buffer, length)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT {
        unsafe { self.extern_FPDFTextObj_GetFont().unwrap()(text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL {
        unsafe { self.extern_FPDFTextObj_GetFontSize().unwrap()(text, size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_BYTESTRING,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFPageObj_NewTextObj().unwrap()(document, font, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_SetText().unwrap()(text_object, text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_SetCharcodes().unwrap()(text_object, charcodes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) {
        unsafe { self.extern_FPDFPage_InsertObject().unwrap()(page, page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPage_RemoveObject().unwrap()(page, page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> c_int {
        unsafe { self.extern_FPDFPage_CountObjects().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFPage_GetObject().unwrap()(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Destroy(&self, page_obj: FPDF_PAGEOBJECT) {
        unsafe { self.extern_FPDFPageObj_Destroy().unwrap()(page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_HasTransparency().unwrap()(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFPageObj_GetType().unwrap()(page_object) }
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
        unsafe { self.extern_FPDFPageObj_Transform().unwrap()(page_object, a, b, c, d, e, f) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_GetMatrix().unwrap()(page_object, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetMatrix(&self, path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_SetMatrix().unwrap()(path, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFPageObj_NewImageObj().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFPageObj_CountMarks().unwrap()(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECTMARK {
        unsafe { self.extern_FPDFPageObj_GetMark().unwrap()(page_object, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_AddMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        name: FPDF_BYTESTRING,
    ) -> FPDF_PAGEOBJECTMARK {
        unsafe { self.extern_FPDFPageObj_AddMark().unwrap()(page_object, name) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_RemoveMark().unwrap()(page_object, mark) }
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
        unsafe { self.extern_FPDFPageObjMark_GetName().unwrap()(mark, buffer, buflen, out_buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> c_int {
        unsafe { self.extern_FPDFPageObjMark_CountParams().unwrap()(mark) }
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
        unsafe {
            self.extern_FPDFPageObjMark_GetParamKey().unwrap()(
                mark, index, buffer, buflen, out_buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_OBJECT_TYPE {
        unsafe { self.extern_FPDFPageObjMark_GetParamValueType().unwrap()(mark, key) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        out_value: *mut c_int,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObjMark_GetParamIntValue().unwrap()(mark, key, out_value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamStringValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFPageObjMark_GetParamStringValue().unwrap()(
                mark, key, buffer, buflen, out_buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamBlobValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFPageObjMark_GetParamBlobValue().unwrap()(
                mark, key, buffer, buflen, out_buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetIntParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: c_int,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFPageObjMark_SetIntParam().unwrap()(
                document,
                page_object,
                mark,
                key,
                value,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetStringParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFPageObjMark_SetStringParam().unwrap()(
                document,
                page_object,
                mark,
                key,
                value,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_SetBlobParam(
        &self,
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFPageObjMark_SetBlobParam().unwrap()(
                document,
                page_object,
                mark,
                key,
                value,
                value_len,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_RemoveParam(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObjMark_RemoveParam().unwrap()(page_object, mark, key) }
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
        unsafe {
            self.extern_FPDFImageObj_LoadJpegFile().unwrap()(
                pages,
                count,
                image_object,
                file_access,
            )
        }
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
        unsafe {
            self.extern_FPDFImageObj_LoadJpegFileInline().unwrap()(
                pages,
                count,
                image_object,
                file_access,
            )
        }
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
        unsafe { self.extern_FPDFImageObj_SetMatrix().unwrap()(image_object, a, b, c, d, e, f) }
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
        unsafe { self.extern_FPDFImageObj_SetBitmap().unwrap()(pages, count, image_object, bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP {
        unsafe { self.extern_FPDFImageObj_GetBitmap().unwrap()(image_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        unsafe {
            self.extern_FPDFImageObj_GetRenderedBitmap().unwrap()(document, page, image_object)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            self.extern_FPDFImageObj_GetImageDataDecoded().unwrap()(image_object, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFImageObj_GetImageDataRaw().unwrap()(image_object, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFImageObj_GetImageFilterCount().unwrap()(image_object) }
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
        unsafe {
            self.extern_FPDFImageObj_GetImageFilter().unwrap()(image_object, index, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFImageObj_GetImageMetadata().unwrap()(image_object, page, metadata)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewPath(&self, x: c_float, y: c_float) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFPageObj_CreateNewPath().unwrap()(x, y) }
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
        unsafe { self.extern_FPDFPageObj_CreateNewRect().unwrap()(x, y, w, h) }
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
        unsafe {
            self.extern_FPDFPageObj_GetBounds().unwrap()(page_object, left, bottom, right, top)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING) {
        unsafe { self.extern_FPDFPageObj_SetBlendMode().unwrap()(page_object, blend_mode) }
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
        unsafe { self.extern_FPDFPageObj_SetStrokeColor().unwrap()(page_object, R, G, B, A) }
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
        unsafe { self.extern_FPDFPageObj_GetStrokeColor().unwrap()(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: c_float,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_SetStrokeWidth().unwrap()(page_object, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_GetStrokeWidth().unwrap()(page_object, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFPageObj_GetLineJoin().unwrap()(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_SetLineJoin().unwrap()(page_object, line_join) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFPageObj_GetLineCap().unwrap()(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_SetLineCap().unwrap()(page_object, line_cap) }
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
        unsafe { self.extern_FPDFPageObj_SetFillColor().unwrap()(page_object, R, G, B, A) }
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
        unsafe { self.extern_FPDFPageObj_GetFillColor().unwrap()(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_GetDashPhase().unwrap()(page_object, phase) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPageObj_SetDashPhase().unwrap()(page_object, phase) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFPageObj_GetDashCount().unwrap()(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFPageObj_GetDashArray().unwrap()(page_object, dash_array, dash_count)
        }
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
        unsafe {
            self.extern_FPDFPageObj_SetDashArray().unwrap()(
                page_object,
                dash_array,
                dash_count,
                phase,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFFont_GetFontName().unwrap()(font, buffer, length) }
    }
}
