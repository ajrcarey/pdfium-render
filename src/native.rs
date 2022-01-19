use crate::bindgen::{
    FPDF_BITMAP, FPDF_BYTESTRING, FPDF_DOCUMENT, FPDF_DWORD, FPDF_PAGE, FPDF_STRING,
};
use crate::bindings::PdfiumLibraryBindings;
use libloading::{Library, Symbol};

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
        result.extern_FPDF_GetPageCount()?;
        result.extern_FPDF_LoadPage()?;
        result.extern_FPDF_ClosePage()?;
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
    ) -> Result<Symbol<unsafe extern "C" fn() -> ::std::os::raw::c_ulong>, libloading::Error> {
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
                data_buf: *const ::std::os::raw::c_void,
                size: ::std::os::raw::c_int,
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
    fn extern_FPDF_GetPageCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> ::std::os::raw::c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetPageCount") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadPage(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page_index: ::std::os::raw::c_int,
            ) -> FPDF_PAGE,
        >,
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
                width: ::std::os::raw::c_int,
                height: ::std::os::raw::c_int,
                format: ::std::os::raw::c_int,
                first_scan: *mut ::std::os::raw::c_void,
                stride: ::std::os::raw::c_int,
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
                left: ::std::os::raw::c_int,
                top: ::std::os::raw::c_int,
                width: ::std::os::raw::c_int,
                height: ::std::os::raw::c_int,
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
    ) -> Result<
        Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut ::std::os::raw::c_void>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBitmap_GetBuffer") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetWidth(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBitmap_GetWidth") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetHeight(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBitmap_GetHeight") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetStride(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int>,
        libloading::Error,
    > {
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
                start_x: ::std::os::raw::c_int,
                start_y: ::std::os::raw::c_int,
                size_x: ::std::os::raw::c_int,
                size_y: ::std::os::raw::c_int,
                rotate: ::std::os::raw::c_int,
                flags: ::std::os::raw::c_int,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_RenderPageBitmap") }
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
    fn FPDF_GetLastError(&self) -> ::std::os::raw::c_ulong {
        unsafe { self.extern_FPDF_GetLastError().unwrap()() }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT {
        let c_file_path = std::ffi::CString::new(file_path).unwrap();
        let c_password = std::ffi::CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            self.extern_FPDF_LoadDocument().unwrap()(c_file_path.as_ptr(), c_password.as_ptr())
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        let c_password = std::ffi::CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            self.extern_FPDF_LoadMemDocument().unwrap()(
                bytes.as_ptr() as *const std::ffi::c_void,
                bytes.len() as ::std::os::raw::c_int,
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
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> ::std::os::raw::c_int {
        unsafe { self.extern_FPDF_GetPageCount().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(
        &self,
        document: FPDF_DOCUMENT,
        page_index: ::std::os::raw::c_int,
    ) -> FPDF_PAGE {
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
    fn FPDFBitmap_CreateEx(
        &self,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        first_scan: *mut std::ffi::c_void,
        stride: ::std::os::raw::c_int,
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
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: FPDF_DWORD,
    ) {
        unsafe {
            self.extern_FPDFBitmap_FillRect().unwrap()(bitmap, left, top, width, height, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut std::os::raw::c_void {
        unsafe { self.extern_FPDFBitmap_GetBuffer().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int {
        unsafe { self.extern_FPDFBitmap_GetWidth().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int {
        unsafe { self.extern_FPDFBitmap_GetHeight().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> ::std::os::raw::c_int {
        unsafe { self.extern_FPDFBitmap_GetStride().unwrap()(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmap(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: ::std::os::raw::c_int,
        start_y: ::std::os::raw::c_int,
        size_x: ::std::os::raw::c_int,
        size_y: ::std::os::raw::c_int,
        rotate: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) {
        unsafe {
            self.extern_FPDF_RenderPageBitmap().unwrap()(
                bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
            );
        }
    }
}
