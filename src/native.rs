use crate::bindgen::{
    FPDF_BITMAP, FPDF_BYTESTRING, FPDF_DOCUMENT, FPDF_DWORD, FPDF_FORMFILLINFO, FPDF_FORMHANDLE,
    FPDF_PAGE, FPDF_STRING,
};
use crate::bindings::PdfiumLibraryBindings;
use libloading::{Library, Symbol};
use std::ffi::{c_void, CString};
use std::os::raw::{c_int, c_uchar, c_ulong};

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
        result.extern_FPDF_GetPageWidthF()?;
        result.extern_FPDF_GetPageHeightF()?;
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
        result.extern_FPDF_SetFormFieldHighlightColor()?;
        result.extern_FPDF_SetFormFieldHighlightAlpha()?;
        result.extern_FPDF_FFLDraw()?;

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
        Symbol<unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> bool>,
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
    fn extern_FPDF_GetPageWidthF(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> f32>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetPageWidthF") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageHeightF(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> f32>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetPageHeightF") }
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
    #[allow(clippy::type_complexity)]
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
    #[allow(clippy::type_complexity)]
    fn extern_FPDFDOC_ExitFormFillEnvironment(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFDOC_ExitFormFillEnvironment") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
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
    #[allow(clippy::type_complexity)]
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
    #[allow(clippy::type_complexity)]
    fn extern_FPDF_GetFormType(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetFormType") }
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
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> bool {
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
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> f32 {
        unsafe { self.extern_FPDF_GetPageWidthF().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> f32 {
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
}
