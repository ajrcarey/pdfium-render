use crate::bindgen::{
    size_t, FPDFANNOT_COLORTYPE, FPDF_ACTION, FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE,
    FPDF_ANNOT_APPEARANCEMODE, FPDF_ATTACHMENT, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL,
    FPDF_BYTESTRING, FPDF_CLIPPATH, FPDF_DEST, FPDF_DOCUMENT, FPDF_DUPLEXTYPE, FPDF_DWORD,
    FPDF_FILEACCESS, FPDF_FILEIDTYPE, FPDF_FILEWRITE, FPDF_FONT, FPDF_FORMFILLINFO,
    FPDF_FORMHANDLE, FPDF_GLYPHPATH, FPDF_IMAGEOBJ_METADATA, FPDF_LINK, FPDF_OBJECT_TYPE,
    FPDF_PAGE, FPDF_PAGELINK, FPDF_PAGEOBJECT, FPDF_PAGEOBJECTMARK, FPDF_PAGERANGE,
    FPDF_PATHSEGMENT, FPDF_SCHHANDLE, FPDF_SIGNATURE, FPDF_STRING, FPDF_STRUCTELEMENT,
    FPDF_STRUCTTREE, FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR, FPDF_WIDESTRING, FS_FLOAT,
    FS_MATRIX, FS_POINTF, FS_QUADPOINTSF, FS_RECTF, FS_SIZEF,
};
use crate::bindings::PdfiumLibraryBindings;
use libloading::Library;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

#[allow(non_snake_case)]
pub(crate) struct DynamicPdfiumBindings {
    #[allow(dead_code)]
    // We take ownership of the libloading::Library to ensure it has the same lifetime
    // as the dynamic bindings we expose, but we never expect to use the library directly
    // inside this crate.
    library: Library,

    // Instead of using the library directly, we cache function pointers to all exposed
    // Pdfium functionality.
    extern_FPDF_InitLibrary: unsafe extern "C" fn(),
    extern_FPDF_DestroyLibrary: unsafe extern "C" fn(),
    extern_FPDF_GetLastError: unsafe extern "C" fn() -> c_ulong,
    extern_FPDF_CreateNewDocument: unsafe extern "C" fn() -> FPDF_DOCUMENT,
    extern_FPDF_LoadDocument:
        unsafe extern "C" fn(file_path: FPDF_STRING, password: FPDF_BYTESTRING) -> FPDF_DOCUMENT,
    extern_FPDF_LoadMemDocument64: unsafe extern "C" fn(
        data_buf: *const c_void,
        size: c_ulong,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT,
    extern_FPDF_LoadCustomDocument: unsafe extern "C" fn(
        pFileAccess: *mut FPDF_FILEACCESS,
        password: FPDF_BYTESTRING,
    ) -> FPDF_DOCUMENT,
    extern_FPDF_SaveAsCopy: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL,
    extern_FPDF_SaveWithVersion: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
        fileVersion: c_int,
    ) -> FPDF_BOOL,
    extern_FPDF_CloseDocument: unsafe extern "C" fn(document: FPDF_DOCUMENT),
    extern_FPDF_DeviceToPage: unsafe extern "C" fn(
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
    ) -> FPDF_BOOL,
    extern_FPDF_PageToDevice: unsafe extern "C" fn(
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
    ) -> FPDF_BOOL,
    extern_FPDF_GetFileVersion:
        unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL,
    extern_FPDF_GetFileIdentifier: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_GetMetaText: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        tag: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_GetDocPermissions: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_ulong,
    extern_FPDF_GetSecurityHandlerRevision: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDF_GetPageCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDF_LoadPage:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE,
    extern_FPDF_ClosePage: unsafe extern "C" fn(page: FPDF_PAGE),
    extern_FPDF_ImportPagesByIndex: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL,
    extern_FPDF_ImportPages: unsafe extern "C" fn(
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        pagerange: FPDF_BYTESTRING,
        index: c_int,
    ) -> FPDF_BOOL,
    extern_FPDF_ImportNPagesToOne: unsafe extern "C" fn(
        src_doc: FPDF_DOCUMENT,
        output_width: c_float,
        output_height: c_float,
        num_pages_on_x_axis: size_t,
        num_pages_on_y_axis: size_t,
    ) -> FPDF_DOCUMENT,
    extern_FPDF_GetPageLabel: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_GetPageBoundingBox:
        unsafe extern "C" fn(page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL,
    extern_FPDF_GetPageSizeByIndexF: unsafe extern "C" fn(
        page: FPDF_DOCUMENT,
        page_index: c_int,
        size: *mut FS_SIZEF,
    ) -> FPDF_BOOL,
    extern_FPDF_GetPageWidthF: unsafe extern "C" fn(page: FPDF_PAGE) -> c_float,
    extern_FPDF_GetPageHeightF: unsafe extern "C" fn(page: FPDF_PAGE) -> c_float,
    extern_FPDFText_GetCharIndexFromTextIndex:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, nTextIndex: c_int) -> c_int,
    extern_FPDFText_GetTextIndexFromCharIndex:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, nCharIndex: c_int) -> c_int,
    extern_FPDF_GetSignatureCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDF_GetSignatureObject:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE,
    extern_FPDFSignatureObj_GetContents: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFSignatureObj_GetByteRange: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_int,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFSignatureObj_GetSubFilter: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFSignatureObj_GetReason: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFSignatureObj_GetTime: unsafe extern "C" fn(
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFSignatureObj_GetDocMDPPermission:
        unsafe extern "C" fn(signature: FPDF_SIGNATURE) -> c_uint,
    extern_FPDF_StructTree_GetForPage: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_STRUCTTREE,
    extern_FPDF_StructTree_Close: unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE),
    extern_FPDF_StructTree_CountChildren:
        unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE) -> c_int,
    extern_FPDF_StructTree_GetChildAtIndex:
        unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE, index: c_int) -> FPDF_STRUCTELEMENT,
    extern_FPDF_StructElement_GetAltText: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_StructElement_GetID: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_StructElement_GetLang: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_StructElement_GetStringAttribute: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_StructElement_GetMarkedContentID:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    extern_FPDF_StructElement_GetType: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_StructElement_GetTitle: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDF_StructElement_CountChildren:
        unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int,
    extern_FPDF_StructElement_GetChildAtIndex: unsafe extern "C" fn(
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT,
    extern_FPDFPage_New: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: c_double,
        height: c_double,
    ) -> FPDF_PAGE,
    extern_FPDFPage_Delete: unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int),
    extern_FPDFPage_GetRotation: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    extern_FPDFPage_SetRotation: unsafe extern "C" fn(page: FPDF_PAGE, rotate: c_int),
    extern_FPDFPage_GetMediaBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPage_GetCropBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPage_GetBleedBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPage_GetTrimBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPage_GetArtBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPage_SetMediaBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ),
    extern_FPDFPage_SetCropBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ),
    extern_FPDFPage_SetBleedBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ),
    extern_FPDFPage_SetTrimBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ),
    extern_FPDFPage_SetArtBox: unsafe extern "C" fn(
        page: FPDF_PAGE,
        left: c_float,
        bottom: c_float,
        right: c_float,
        top: c_float,
    ),
    extern_FPDFPage_TransFormWithClip: unsafe extern "C" fn(
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_TransformClipPath: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ),
    extern_FPDFPageObj_GetClipPath:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH,
    extern_FPDFClipPath_CountPaths: unsafe extern "C" fn(clip_path: FPDF_CLIPPATH) -> c_int,
    extern_FPDFClipPath_CountPathSegments:
        unsafe extern "C" fn(clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int,
    extern_FPDFClipPath_GetPathSegment: unsafe extern "C" fn(
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT,
    extern_FPDF_CreateClipPath:
        unsafe extern "C" fn(left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH,
    extern_FPDF_DestroyClipPath: unsafe extern "C" fn(clipPath: FPDF_CLIPPATH),
    extern_FPDFPage_InsertClipPath: unsafe extern "C" fn(page: FPDF_PAGE, clipPath: FPDF_CLIPPATH),
    extern_FPDFPage_HasTransparency: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    extern_FPDFPage_GenerateContent: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL,
    extern_FPDFPage_TransformAnnots: unsafe extern "C" fn(
        page: FPDF_PAGE,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ),
    extern_FPDFBitmap_CreateEx: unsafe extern "C" fn(
        width: c_int,
        height: c_int,
        format: c_int,
        first_scan: *mut c_void,
        stride: c_int,
    ) -> FPDF_BITMAP,
    extern_FPDFBitmap_Destroy: unsafe extern "C" fn(bitmap: FPDF_BITMAP),
    extern_FPDFBitmap_GetFormat: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    extern_FPDFBitmap_FillRect: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
        color: FPDF_DWORD,
    ),
    extern_FPDFBitmap_GetBuffer: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut c_void,
    extern_FPDFBitmap_GetWidth: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    extern_FPDFBitmap_GetHeight: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    extern_FPDFBitmap_GetStride: unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int,
    extern_FPDF_RenderPageBitmap: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        start_x: c_int,
        start_y: c_int,
        size_x: c_int,
        size_y: c_int,
        rotate: c_int,
        flags: c_int,
    ),
    extern_FPDF_RenderPageBitmapWithMatrix: unsafe extern "C" fn(
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ),
    extern_FPDFAnnot_IsSupportedSubtype:
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
    extern_FPDFPage_CreateAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_ANNOTATION,
    extern_FPDFPage_GetAnnotCount: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    extern_FPDFPage_GetAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION,
    extern_FPDFPage_GetAnnotIndex:
        unsafe extern "C" fn(page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFPage_CloseAnnot: unsafe extern "C" fn(annot: FPDF_ANNOTATION),
    extern_FPDFPage_RemoveAnnot: unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_BOOL,
    extern_FPDFAnnot_GetSubtype:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE,
    extern_FPDFAnnot_IsObjectSupportedSubtype:
        unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL,
    extern_FPDFAnnot_UpdateObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    extern_FPDFAnnot_AddInkStroke: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int,
    extern_FPDFAnnot_RemoveInkList: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    extern_FPDFAnnot_AppendObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    extern_FPDFAnnot_GetObjectCount: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_GetObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT,
    extern_FPDFAnnot_RemoveObject:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL,
    extern_FPDFAnnot_SetColor: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetColor: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        color_type: FPDFANNOT_COLORTYPE,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_HasAttachmentPoints: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    extern_FPDFAnnot_SetAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_AppendAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_CountAttachmentPoints: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> size_t,
    extern_FPDFAnnot_GetAttachmentPoints: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_SetRect:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL,
    extern_FPDFAnnot_GetRect:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL,
    extern_FPDFAnnot_GetVertices: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_GetInkListCount: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_ulong,
    extern_FPDFAnnot_GetInkListPath: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        path_index: c_ulong,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_GetLine: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_SetBorder: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetBorder: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut f32,
        vertical_radius: *mut f32,
        border_width: *mut f32,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_HasKey:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_BOOL,
    extern_FPDFAnnot_GetValueType:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    extern_FPDFAnnot_SetStringValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetStringValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_GetNumberValue: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        key: FPDF_BYTESTRING,
        value: *mut f32,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_SetAP: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetAP: unsafe extern "C" fn(
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_GetLinkedAnnot:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_ANNOTATION,
    extern_FPDFAnnot_GetFlags: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_SetFlags:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL,
    extern_FPDFAnnot_GetFormFieldFlags:
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_GetFormFieldAtPoint: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION,
    extern_FPDFAnnot_GetFormFieldName: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_GetFormFieldType:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_GetFormFieldValue: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_GetOptionCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_GetOptionLabel: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_IsOptionSelected: unsafe extern "C" fn(
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetFontSize: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_IsChecked:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL,
    extern_FPDFAnnot_SetFocusableSubtypes: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetFocusableSubtypesCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> c_int,
    extern_FPDFAnnot_GetFocusableSubtypes: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL,
    extern_FPDFAnnot_GetLink: unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_LINK,
    extern_FPDFAnnot_GetFormControlCount:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_GetFormControlIndex:
        unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int,
    extern_FPDFAnnot_GetFormFieldExportValue: unsafe extern "C" fn(
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAnnot_SetURI:
        unsafe extern "C" fn(annot: FPDF_ANNOTATION, uri: *const c_char) -> FPDF_BOOL,
    extern_FPDFDOC_InitFormFillEnvironment: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE,
    extern_FPDFDOC_ExitFormFillEnvironment: unsafe extern "C" fn(handle: FPDF_FORMHANDLE),
    extern_FORM_OnAfterLoadPage: unsafe extern "C" fn(page: FPDF_PAGE, handle: FPDF_FORMHANDLE),
    extern_FORM_OnBeforeClosePage: unsafe extern "C" fn(page: FPDF_PAGE, handle: FPDF_FORMHANDLE),
    extern_FPDFDoc_GetPageMode: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDFPage_Flatten: unsafe extern "C" fn(page: FPDF_PAGE, nFlag: c_int) -> c_int,
    extern_FPDF_SetFormFieldHighlightColor:
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, field_type: c_int, color: c_ulong),
    extern_FPDF_SetFormFieldHighlightAlpha:
        unsafe extern "C" fn(handle: FPDF_FORMHANDLE, alpha: c_uchar),
    extern_FPDF_FFLDraw: unsafe extern "C" fn(
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
    extern_FPDF_GetFormType: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDFBookmark_GetFirstChild:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
    extern_FPDFBookmark_GetNextSibling:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK,
    extern_FPDFBookmark_GetTitle: unsafe extern "C" fn(
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFBookmark_GetCount: unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> c_int,
    extern_FPDFBookmark_Find:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK,
    extern_FPDFBookmark_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST,
    extern_FPDFBookmark_GetAction: unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> FPDF_ACTION,
    extern_FPDFAction_GetType: unsafe extern "C" fn(action: FPDF_ACTION) -> c_ulong,
    extern_FPDFAction_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST,
    extern_FPDFAction_GetFilePath:
        unsafe extern "C" fn(action: FPDF_ACTION, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    extern_FPDFAction_GetURIPath: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFDest_GetDestPageIndex:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int,
    extern_FPDFDest_GetView: unsafe extern "C" fn(
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong,
    extern_FPDFDest_GetLocationInPage: unsafe extern "C" fn(
        dest: FPDF_DEST,
        hasXVal: *mut FPDF_BOOL,
        hasYVal: *mut FPDF_BOOL,
        hasZoomVal: *mut FPDF_BOOL,
        x: *mut FS_FLOAT,
        y: *mut FS_FLOAT,
        zoom: *mut FS_FLOAT,
    ) -> FPDF_BOOL,
    extern_FPDFLink_GetLinkAtPoint:
        unsafe extern "C" fn(page: FPDF_PAGE, x: c_double, y: c_double) -> FPDF_LINK,
    extern_FPDFLink_GetLinkZOrderAtPoint:
        unsafe extern "C" fn(page: FPDF_PAGE, x: c_double, y: c_double) -> c_int,
    extern_FPDFLink_GetDest:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST,
    extern_FPDFLink_GetAction: unsafe extern "C" fn(link: FPDF_LINK) -> FPDF_ACTION,
    extern_FPDFLink_Enumerate: unsafe extern "C" fn(
        page: FPDF_PAGE,
        start_pos: *mut c_int,
        link_annot: *mut FPDF_LINK,
    ) -> FPDF_BOOL,
    extern_FPDFLink_GetAnnot:
        unsafe extern "C" fn(page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION,
    extern_FPDFLink_GetAnnotRect:
        unsafe extern "C" fn(link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL,
    extern_FPDFLink_CountQuadPoints: unsafe extern "C" fn(link_annot: FPDF_LINK) -> c_int,
    extern_FPDFLink_GetQuadPoints: unsafe extern "C" fn(
        link_annot: FPDF_LINK,
        quad_index: c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL,
    extern_FPDF_GetPageAAction:
        unsafe extern "C" fn(page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION,
    extern_FPDFText_LoadPage: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_TEXTPAGE,
    extern_FPDFText_ClosePage: unsafe extern "C" fn(text_page: FPDF_TEXTPAGE),
    extern_FPDFText_CountChars: unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> c_int,
    extern_FPDFText_GetUnicode:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint,
    extern_FPDFText_GetTextObject:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> FPDF_PAGEOBJECT,
    extern_FPDFText_GetFontSize:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_double,
    extern_FPDFText_GetFontInfo: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        flags: *mut c_int,
    ) -> c_ulong,
    extern_FPDFText_GetFontWeight:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_int,
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
    extern_FPDFText_GetTextRenderMode:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> FPDF_TEXT_RENDERMODE,
    extern_FPDFText_GetFillColor: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetStrokeColor: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetCharAngle:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_float,
    extern_FPDFText_GetCharBox: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        left: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
        top: *mut c_double,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetLooseCharBox: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetMatrix: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetCharOrigin: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        x: *mut c_double,
        y: *mut c_double,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetCharIndexAtPos: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        x: c_double,
        y: c_double,
        xTolerance: c_double,
        yTolerance: c_double,
    ) -> c_int,
    extern_FPDFText_GetText: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
        result: *mut c_ushort,
    ) -> c_int,
    extern_FPDFText_CountRects:
        unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, start_index: c_int, count: c_int) -> c_int,
    extern_FPDFText_GetRect: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL,
    extern_FPDFText_GetBoundedText: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        left: c_double,
        top: c_double,
        right: c_double,
        bottom: c_double,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int,
    extern_FPDFText_FindStart: unsafe extern "C" fn(
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE,
    extern_FPDFText_FindNext: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL,
    extern_FPDFText_FindPrev: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL,
    extern_FPDFText_GetSchResultIndex: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> c_int,
    extern_FPDFText_GetSchCount: unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> c_int,
    extern_FPDFText_FindClose: unsafe extern "C" fn(handle: FPDF_SCHHANDLE),
    extern_FPDFLink_LoadWebLinks: unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK,
    extern_FPDFLink_CountWebLinks: unsafe extern "C" fn(link_page: FPDF_PAGELINK) -> c_int,
    extern_FPDFLink_GetURL: unsafe extern "C" fn(
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int,
    extern_FPDFLink_CountRects:
        unsafe extern "C" fn(link_page: FPDF_PAGELINK, link_index: c_int) -> c_int,
    extern_FPDFLink_GetRect: unsafe extern "C" fn(
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        rect_index: c_int,
        left: *mut c_double,
        top: *mut c_double,
        right: *mut c_double,
        bottom: *mut c_double,
    ) -> FPDF_BOOL,
    extern_FPDFLink_GetTextRange: unsafe extern "C" fn(
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        start_char_index: *mut c_int,
        char_count: *mut c_int,
    ) -> FPDF_BOOL,
    extern_FPDFLink_CloseWebLinks: unsafe extern "C" fn(link_page: FPDF_PAGELINK),
    extern_FPDFPage_GetDecodedThumbnailData:
        unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    extern_FPDFPage_GetRawThumbnailData:
        unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
    extern_FPDFPage_GetThumbnailAsBitmap: unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BITMAP,
    extern_FPDFFormObj_CountObjects: unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFFormObj_GetObject:
        unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT, index: c_ulong) -> FPDF_PAGEOBJECT,
    extern_FPDFPageObj_CreateTextObj: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT,
    extern_FPDFTextObj_GetTextRenderMode:
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE,
    extern_FPDFTextObj_SetTextRenderMode:
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, render_mode: FPDF_TEXT_RENDERMODE) -> FPDF_BOOL,
    extern_FPDFTextObj_GetText: unsafe extern "C" fn(
        text_object: FPDF_PAGEOBJECT,
        text_page: FPDF_TEXTPAGE,
        buffer: *mut FPDF_WCHAR,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFTextObj_GetFont: unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_FONT,
    extern_FPDFTextObj_GetFontSize:
        unsafe extern "C" fn(text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL,
    extern_FPDFPageObj_NewTextObj: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        font: FPDF_BYTESTRING,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT,
    extern_FPDFText_SetText:
        unsafe extern "C" fn(text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL,
    extern_FPDFText_SetCharcodes: unsafe extern "C" fn(
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL,
    extern_FPDFText_LoadFont: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        data: *const c_uchar,
        size: c_uint,
        font_type: c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT,
    extern_FPDFText_LoadStandardFont:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, font: FPDF_BYTESTRING) -> FPDF_FONT,
    extern_FPDFFont_Close: unsafe extern "C" fn(font: FPDF_FONT),
    extern_FPDFPath_MoveTo:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL,
    extern_FPDFPath_LineTo:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL,
    extern_FPDFPath_BezierTo: unsafe extern "C" fn(
        path: FPDF_PAGEOBJECT,
        x1: c_float,
        y1: c_float,
        x2: c_float,
        y2: c_float,
        x3: c_float,
        y3: c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPath_Close: unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    extern_FPDFPath_SetDrawMode: unsafe extern "C" fn(
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL,
    extern_FPDFPath_GetDrawMode: unsafe extern "C" fn(
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL,
    extern_FPDFPage_InsertObject: unsafe extern "C" fn(page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT),
    extern_FPDFPage_RemoveObject:
        unsafe extern "C" fn(page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    extern_FPDFPage_CountObjects: unsafe extern "C" fn(page: FPDF_PAGE) -> c_int,
    extern_FPDFPage_GetObject:
        unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT,
    extern_FPDFPageObj_Destroy: unsafe extern "C" fn(page_obj: FPDF_PAGEOBJECT),
    extern_FPDFPageObj_HasTransparency:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL,
    extern_FPDFPageObj_GetType: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFPageObj_Transform: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ),
    extern_FPDFPageObj_GetMatrix:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, matrix: *mut FS_MATRIX) -> FPDF_BOOL,
    extern_FPDFPageObj_SetMatrix:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL,
    extern_FPDFPageObj_NewImageObj:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT,
    extern_FPDFPageObj_CountMarks: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFPageObj_GetMark:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, index: c_ulong) -> FPDF_PAGEOBJECTMARK,
    extern_FPDFPageObj_AddMark: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        name: FPDF_BYTESTRING,
    ) -> FPDF_PAGEOBJECTMARK,
    extern_FPDFPageObj_RemoveMark:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, mark: FPDF_PAGEOBJECTMARK) -> FPDF_BOOL,
    extern_FPDFPageObjMark_GetName: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_CountParams: unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK) -> c_int,
    extern_FPDFPageObjMark_GetParamKey: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        index: c_ulong,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_GetParamValueType:
        unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    extern_FPDFPageObjMark_GetParamIntValue: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        out_value: *mut c_int,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_GetParamStringValue: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_GetParamBlobValue: unsafe extern "C" fn(
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_SetIntParam: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: c_int,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_SetStringParam: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: FPDF_BYTESTRING,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_SetBlobParam: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFPageObjMark_RemoveParam: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
        key: FPDF_BYTESTRING,
    ) -> FPDF_BOOL,
    extern_FPDFImageObj_LoadJpegFile: unsafe extern "C" fn(
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL,
    extern_FPDFImageObj_LoadJpegFileInline: unsafe extern "C" fn(
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        file_access: *mut FPDF_FILEACCESS,
    ) -> FPDF_BOOL,
    extern_FPDFImageObj_SetMatrix: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        a: c_double,
        b: c_double,
        c: c_double,
        d: c_double,
        e: c_double,
        f: c_double,
    ) -> FPDF_BOOL,
    extern_FPDFImageObj_SetBitmap: unsafe extern "C" fn(
        pages: *mut FPDF_PAGE,
        count: c_int,
        image_object: FPDF_PAGEOBJECT,
        bitmap: FPDF_BITMAP,
    ) -> FPDF_BOOL,
    extern_FPDFImageObj_GetBitmap:
        unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP,
    extern_FPDFImageObj_GetRenderedBitmap: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP,
    extern_FPDFImageObj_GetImageDataDecoded: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFImageObj_GetImageDataRaw: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFImageObj_GetImageFilterCount:
        unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFImageObj_GetImageFilter: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFImageObj_GetImageMetadata: unsafe extern "C" fn(
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_CreateNewPath:
        unsafe extern "C" fn(x: c_float, y: c_float) -> FPDF_PAGEOBJECT,
    extern_FPDFPageObj_CreateNewRect:
        unsafe extern "C" fn(x: c_float, y: c_float, w: c_float, h: c_float) -> FPDF_PAGEOBJECT,
    extern_FPDFPageObj_GetBounds: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        left: *mut c_float,
        bottom: *mut c_float,
        right: *mut c_float,
        top: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_SetBlendMode:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING),
    extern_FPDFPageObj_SetStrokeColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_GetStrokeColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_SetStrokeWidth:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: c_float) -> FPDF_BOOL,
    extern_FPDFPageObj_GetStrokeWidth:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: *mut c_float) -> FPDF_BOOL,
    extern_FPDFPageObj_GetLineJoin: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFPageObj_SetLineJoin:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL,
    extern_FPDFPageObj_GetLineCap: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFPageObj_SetLineCap:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL,
    extern_FPDFPageObj_SetFillColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: c_uint,
        G: c_uint,
        B: c_uint,
        A: c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_GetFillColor: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        R: *mut c_uint,
        G: *mut c_uint,
        B: *mut c_uint,
        A: *mut c_uint,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_GetDashPhase:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: *mut c_float) -> FPDF_BOOL,
    extern_FPDFPageObj_SetDashPhase:
        unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL,
    extern_FPDFPageObj_GetDashCount: unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFPageObj_GetDashArray: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL,
    extern_FPDFPageObj_SetDashArray: unsafe extern "C" fn(
        page_object: FPDF_PAGEOBJECT,
        dash_array: *const c_float,
        dash_count: size_t,
        phase: c_float,
    ) -> FPDF_BOOL,
    extern_FPDFPath_CountSegments: unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> c_int,
    extern_FPDFPath_GetPathSegment:
        unsafe extern "C" fn(path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT,
    extern_FPDFPathSegment_GetPoint:
        unsafe extern "C" fn(segment: FPDF_PATHSEGMENT, x: *mut f32, y: *mut f32) -> FPDF_BOOL,
    extern_FPDFPathSegment_GetType: unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> c_int,
    extern_FPDFPathSegment_GetClose: unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> FPDF_BOOL,
    // TODO: AJRC - 4-Aug-2024 - FPDFFont_GetBaseFontName() is in Pdfium export headers
    // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
    #[cfg(feature = "pdfium_future")]
    extern_FPDFFont_GetBaseFontName:
        unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: usize) -> usize,
    // TODO: AJRC - 4-Aug-2024 - pointer type updated in FPDFFont_GetBaseFontName() definition,
    // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
    #[cfg(feature = "pdfium_future")]
    extern_FPDFFont_GetFamilyName:
        unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: usize) -> usize,
    #[cfg(feature = "pdfium_6611")]
    extern_FPDFFont_GetFamilyName:
        unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: c_ulong) -> c_ulong,
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
    extern_FPDFFont_GetFontName:
        unsafe extern "C" fn(font: FPDF_FONT, buffer: *mut c_char, length: c_ulong) -> c_ulong,
    extern_FPDFFont_GetFontData: unsafe extern "C" fn(
        font: FPDF_FONT,
        buffer: *mut u8,
        buflen: usize,
        out_buflen: *mut usize,
    ) -> FPDF_BOOL,
    extern_FPDFFont_GetIsEmbedded: unsafe extern "C" fn(font: FPDF_FONT) -> c_int,
    extern_FPDFFont_GetFlags: unsafe extern "C" fn(font: FPDF_FONT) -> c_int,
    extern_FPDFFont_GetWeight: unsafe extern "C" fn(font: FPDF_FONT) -> c_int,
    extern_FPDFFont_GetItalicAngle:
        unsafe extern "C" fn(font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL,
    extern_FPDFFont_GetAscent: unsafe extern "C" fn(
        font: FPDF_FONT,
        font_size: c_float,
        ascent: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFFont_GetDescent: unsafe extern "C" fn(
        font: FPDF_FONT,
        font_size: c_float,
        descent: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFFont_GetGlyphWidth: unsafe extern "C" fn(
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
        width: *mut c_float,
    ) -> FPDF_BOOL,
    extern_FPDFFont_GetGlyphPath:
        unsafe extern "C" fn(font: FPDF_FONT, glyph: c_uint, font_size: c_float) -> FPDF_GLYPHPATH,
    extern_FPDFGlyphPath_CountGlyphSegments:
        unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH) -> c_int,
    extern_FPDFGlyphPath_GetGlyphPathSegment:
        unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH, index: c_int) -> FPDF_PATHSEGMENT,
    extern_FPDF_VIEWERREF_GetPrintScaling:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
    extern_FPDF_VIEWERREF_GetNumCopies: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDF_VIEWERREF_GetPrintPageRange:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGERANGE,
    extern_FPDF_VIEWERREF_GetPrintPageRangeCount:
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE) -> size_t,
    extern_FPDF_VIEWERREF_GetPrintPageRangeElement:
        unsafe extern "C" fn(pagerange: FPDF_PAGERANGE, index: size_t) -> c_int,
    extern_FPDF_VIEWERREF_GetDuplex:
        unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE,
    extern_FPDF_VIEWERREF_GetName: unsafe extern "C" fn(
        document: FPDF_DOCUMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong,
    extern_FPDFDoc_GetAttachmentCount: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int,
    extern_FPDFDoc_AddAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
    extern_FPDFDoc_GetAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT,
    extern_FPDFDoc_DeleteAttachment:
        unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL,
    extern_FPDFAttachment_GetName: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAttachment_HasKey:
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_BOOL,
    extern_FPDFAttachment_GetValueType:
        unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
    extern_FPDFAttachment_SetStringValue: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL,
    extern_FPDFAttachment_GetStringValue: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        key: FPDF_BYTESTRING,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong,
    extern_FPDFAttachment_SetFile: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const c_void,
        len: c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFAttachment_GetFile: unsafe extern "C" fn(
        attachment: FPDF_ATTACHMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL,
    extern_FPDFCatalog_IsTagged: unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL,
}

impl DynamicPdfiumBindings {
    pub fn new(library: Library) -> Result<Self, libloading::Error> {
        let result = unsafe {
            DynamicPdfiumBindings {
                extern_FPDF_InitLibrary: *(library.get(b"FPDF_InitLibrary\0")?),
                extern_FPDF_DestroyLibrary: *(library.get(b"FPDF_DestroyLibrary\0")?),
                extern_FPDF_GetLastError: *(library.get(b"FPDF_GetLastError\0")?),
                extern_FPDF_CreateNewDocument: *(library.get(b"FPDF_CreateNewDocument\0")?),
                extern_FPDF_LoadDocument: *(library.get(b"FPDF_LoadDocument\0")?),
                extern_FPDF_LoadMemDocument64: *(library.get(b"FPDF_LoadMemDocument64\0")?),
                extern_FPDF_LoadCustomDocument: *(library.get(b"FPDF_LoadCustomDocument\0")?),
                extern_FPDF_SaveAsCopy: *(library.get(b"FPDF_SaveAsCopy\0")?),
                extern_FPDF_SaveWithVersion: *(library.get(b"FPDF_SaveWithVersion\0")?),
                extern_FPDF_CloseDocument: *(library.get(b"FPDF_CloseDocument\0")?),
                extern_FPDF_DeviceToPage: *(library.get(b"FPDF_DeviceToPage\0")?),
                extern_FPDF_PageToDevice: *(library.get(b"FPDF_PageToDevice\0")?),
                extern_FPDF_GetFileVersion: *(library.get(b"FPDF_GetFileVersion\0")?),
                extern_FPDF_GetFileIdentifier: *(library.get(b"FPDF_GetFileIdentifier\0")?),
                extern_FPDF_GetMetaText: *(library.get(b"FPDF_GetMetaText\0")?),
                extern_FPDF_GetDocPermissions: *(library.get(b"FPDF_GetDocPermissions\0")?),
                extern_FPDF_GetSecurityHandlerRevision: *(library
                    .get(b"FPDF_GetSecurityHandlerRevision\0")?),
                extern_FPDF_GetPageCount: *(library.get(b"FPDF_GetPageCount\0")?),
                extern_FPDF_LoadPage: *(library.get(b"FPDF_LoadPage\0")?),
                extern_FPDF_ClosePage: *(library.get(b"FPDF_ClosePage\0")?),
                extern_FPDF_ImportPagesByIndex: *(library.get(b"FPDF_ImportPagesByIndex\0")?),
                extern_FPDF_ImportPages: *(library.get(b"FPDF_ImportPages\0")?),
                extern_FPDF_ImportNPagesToOne: *(library.get(b"FPDF_ImportNPagesToOne\0")?),
                extern_FPDF_GetPageLabel: *(library.get(b"FPDF_GetPageLabel\0")?),
                extern_FPDF_GetPageBoundingBox: *(library.get(b"FPDF_GetPageBoundingBox\0")?),
                extern_FPDF_GetPageSizeByIndexF: *(library.get(b"FPDF_GetPageSizeByIndexF\0")?),
                extern_FPDF_GetPageWidthF: *(library.get(b"FPDF_GetPageWidthF\0")?),
                extern_FPDF_GetPageHeightF: *(library.get(b"FPDF_GetPageHeightF\0")?),
                extern_FPDFText_GetCharIndexFromTextIndex: *(library
                    .get(b"FPDFText_GetCharIndexFromTextIndex\0")?),
                extern_FPDFText_GetTextIndexFromCharIndex: *(library
                    .get(b"FPDFText_GetTextIndexFromCharIndex\0")?),
                extern_FPDF_GetSignatureCount: *(library.get(b"FPDF_GetSignatureCount\0")?),
                extern_FPDF_GetSignatureObject: *(library.get(b"FPDF_GetSignatureObject\0")?),
                extern_FPDFSignatureObj_GetContents: *(library
                    .get(b"FPDFSignatureObj_GetContents\0")?),
                extern_FPDFSignatureObj_GetByteRange: *(library
                    .get(b"FPDFSignatureObj_GetByteRange\0")?),
                extern_FPDFSignatureObj_GetSubFilter: *(library
                    .get(b"FPDFSignatureObj_GetSubFilter\0")?),
                extern_FPDFSignatureObj_GetReason: *(library
                    .get(b"FPDFSignatureObj_GetReason\0")?),
                extern_FPDFSignatureObj_GetTime: *(library.get(b"FPDFSignatureObj_GetTime\0")?),
                extern_FPDFSignatureObj_GetDocMDPPermission: *(library
                    .get(b"FPDFSignatureObj_GetDocMDPPermission\0")?),
                extern_FPDF_StructTree_GetForPage: *(library
                    .get(b"FPDF_StructTree_GetForPage\0")?),
                extern_FPDF_StructTree_Close: *(library.get(b"FPDF_StructTree_Close\0")?),
                extern_FPDF_StructTree_CountChildren: *(library
                    .get(b"FPDF_StructTree_CountChildren\0")?),
                extern_FPDF_StructTree_GetChildAtIndex: *(library
                    .get(b"FPDF_StructTree_GetChildAtIndex\0")?),
                extern_FPDF_StructElement_GetAltText: *(library
                    .get(b"FPDF_StructElement_GetAltText\0")?),
                extern_FPDF_StructElement_GetID: *(library.get(b"FPDF_StructElement_GetID\0")?),
                extern_FPDF_StructElement_GetLang: *(library
                    .get(b"FPDF_StructElement_GetLang\0")?),
                extern_FPDF_StructElement_GetStringAttribute: *(library
                    .get(b"FPDF_StructElement_GetStringAttribute\0")?),
                extern_FPDF_StructElement_GetMarkedContentID: *(library
                    .get(b"FPDF_StructElement_GetMarkedContentID\0")?),
                extern_FPDF_StructElement_GetType: *(library
                    .get(b"FPDF_StructElement_GetType\0")?),
                extern_FPDF_StructElement_GetTitle: *(library
                    .get(b"FPDF_StructElement_GetTitle\0")?),
                extern_FPDF_StructElement_CountChildren: *(library
                    .get(b"FPDF_StructElement_CountChildren\0")?),
                extern_FPDF_StructElement_GetChildAtIndex: *(library
                    .get(b"FPDF_StructElement_GetChildAtIndex\0")?),
                extern_FPDFPage_New: *(library.get(b"FPDFPage_New\0")?),
                extern_FPDFPage_Delete: *(library.get(b"FPDFPage_Delete\0")?),
                extern_FPDFPage_GetRotation: *(library.get(b"FPDFPage_GetRotation\0")?),
                extern_FPDFPage_SetRotation: *(library.get(b"FPDFPage_SetRotation\0")?),
                extern_FPDFPage_GetMediaBox: *(library.get(b"FPDFPage_GetMediaBox\0")?),
                extern_FPDFPage_GetCropBox: *(library.get(b"FPDFPage_GetCropBox\0")?),
                extern_FPDFPage_GetBleedBox: *(library.get(b"FPDFPage_GetBleedBox\0")?),
                extern_FPDFPage_GetTrimBox: *(library.get(b"FPDFPage_GetTrimBox\0")?),
                extern_FPDFPage_GetArtBox: *(library.get(b"FPDFPage_GetArtBox\0")?),
                extern_FPDFPage_SetMediaBox: *(library.get(b"FPDFPage_SetMediaBox\0")?),
                extern_FPDFPage_SetCropBox: *(library.get(b"FPDFPage_SetCropBox\0")?),
                extern_FPDFPage_SetBleedBox: *(library.get(b"FPDFPage_SetBleedBox\0")?),
                extern_FPDFPage_SetTrimBox: *(library.get(b"FPDFPage_SetTrimBox\0")?),
                extern_FPDFPage_SetArtBox: *(library.get(b"FPDFPage_SetArtBox\0")?),
                extern_FPDFPage_TransFormWithClip: *(library
                    .get(b"FPDFPage_TransFormWithClip\0")?),
                extern_FPDFPageObj_TransformClipPath: *(library
                    .get(b"FPDFPageObj_TransformClipPath\0")?),
                extern_FPDFPageObj_GetClipPath: *(library.get(b"FPDFPageObj_GetClipPath\0")?),
                extern_FPDFClipPath_CountPaths: *(library.get(b"FPDFClipPath_CountPaths\0")?),
                extern_FPDFClipPath_CountPathSegments: *(library
                    .get(b"FPDFClipPath_CountPathSegments\0")?),
                extern_FPDFClipPath_GetPathSegment: *(library
                    .get(b"FPDFClipPath_GetPathSegment\0")?),
                extern_FPDF_CreateClipPath: *(library.get(b"FPDF_CreateClipPath\0")?),
                extern_FPDF_DestroyClipPath: *(library.get(b"FPDF_DestroyClipPath\0")?),
                extern_FPDFPage_InsertClipPath: *(library.get(b"FPDFPage_InsertClipPath\0")?),
                extern_FPDFPage_HasTransparency: *(library.get(b"FPDFPage_HasTransparency\0")?),
                extern_FPDFPage_GenerateContent: *(library.get(b"FPDFPage_GenerateContent\0")?),
                extern_FPDFPage_TransformAnnots: *(library.get(b"FPDFPage_TransformAnnots\0")?),
                extern_FPDFBitmap_CreateEx: *(library.get(b"FPDFBitmap_CreateEx\0")?),
                extern_FPDFBitmap_Destroy: *(library.get(b"FPDFBitmap_Destroy\0")?),
                extern_FPDFBitmap_GetFormat: *(library.get(b"FPDFBitmap_GetFormat\0")?),
                extern_FPDFBitmap_FillRect: *(library.get(b"FPDFBitmap_FillRect\0")?),
                extern_FPDFBitmap_GetBuffer: *(library.get(b"FPDFBitmap_GetBuffer\0")?),
                extern_FPDFBitmap_GetWidth: *(library.get(b"FPDFBitmap_GetWidth\0")?),
                extern_FPDFBitmap_GetHeight: *(library.get(b"FPDFBitmap_GetHeight\0")?),
                extern_FPDFBitmap_GetStride: *(library.get(b"FPDFBitmap_GetStride\0")?),
                extern_FPDF_RenderPageBitmap: *(library.get(b"FPDF_RenderPageBitmap\0")?),
                extern_FPDF_RenderPageBitmapWithMatrix: *(library
                    .get(b"FPDF_RenderPageBitmapWithMatrix\0")?),
                extern_FPDFAnnot_IsSupportedSubtype: *(library
                    .get(b"FPDFAnnot_IsSupportedSubtype\0")?),
                extern_FPDFPage_CreateAnnot: *(library.get(b"FPDFPage_CreateAnnot\0")?),
                extern_FPDFPage_GetAnnotCount: *(library.get(b"FPDFPage_GetAnnotCount\0")?),
                extern_FPDFPage_GetAnnot: *(library.get(b"FPDFPage_GetAnnot\0")?),
                extern_FPDFPage_GetAnnotIndex: *(library.get(b"FPDFPage_GetAnnotIndex\0")?),
                extern_FPDFPage_CloseAnnot: *(library.get(b"FPDFPage_CloseAnnot\0")?),
                extern_FPDFPage_RemoveAnnot: *(library.get(b"FPDFPage_RemoveAnnot\0")?),
                extern_FPDFAnnot_GetSubtype: *(library.get(b"FPDFAnnot_GetSubtype\0")?),
                extern_FPDFAnnot_IsObjectSupportedSubtype: *(library
                    .get(b"FPDFAnnot_IsObjectSupportedSubtype\0")?),
                extern_FPDFAnnot_UpdateObject: *(library.get(b"FPDFAnnot_UpdateObject\0")?),
                extern_FPDFAnnot_AddInkStroke: *(library.get(b"FPDFAnnot_AddInkStroke\0")?),
                extern_FPDFAnnot_RemoveInkList: *(library.get(b"FPDFAnnot_RemoveInkList\0")?),
                extern_FPDFAnnot_AppendObject: *(library.get(b"FPDFAnnot_AppendObject\0")?),
                extern_FPDFAnnot_GetObjectCount: *(library.get(b"FPDFAnnot_GetObjectCount\0")?),
                extern_FPDFAnnot_GetObject: *(library.get(b"FPDFAnnot_GetObject\0")?),
                extern_FPDFAnnot_RemoveObject: *(library.get(b"FPDFAnnot_RemoveObject\0")?),
                extern_FPDFAnnot_SetColor: *(library.get(b"FPDFAnnot_SetColor\0")?),
                extern_FPDFAnnot_GetColor: *(library.get(b"FPDFAnnot_GetColor\0")?),
                extern_FPDFAnnot_HasAttachmentPoints: *(library
                    .get(b"FPDFAnnot_HasAttachmentPoints\0")?),
                extern_FPDFAnnot_SetAttachmentPoints: *(library
                    .get(b"FPDFAnnot_SetAttachmentPoints\0")?),
                extern_FPDFAnnot_AppendAttachmentPoints: *(library
                    .get(b"FPDFAnnot_AppendAttachmentPoints\0")?),
                extern_FPDFAnnot_CountAttachmentPoints: *(library
                    .get(b"FPDFAnnot_CountAttachmentPoints\0")?),
                extern_FPDFAnnot_GetAttachmentPoints: *(library
                    .get(b"FPDFAnnot_GetAttachmentPoints\0")?),
                extern_FPDFAnnot_SetRect: *(library.get(b"FPDFAnnot_SetRect\0")?),
                extern_FPDFAnnot_GetRect: *(library.get(b"FPDFAnnot_GetRect\0")?),
                extern_FPDFAnnot_GetVertices: *(library.get(b"FPDFAnnot_GetVertices\0")?),
                extern_FPDFAnnot_GetInkListCount: *(library.get(b"FPDFAnnot_GetInkListCount\0")?),
                extern_FPDFAnnot_GetInkListPath: *(library.get(b"FPDFAnnot_GetInkListPath\0")?),
                extern_FPDFAnnot_GetLine: *(library.get(b"FPDFAnnot_GetLine\0")?),
                extern_FPDFAnnot_SetBorder: *(library.get(b"FPDFAnnot_SetBorder\0")?),
                extern_FPDFAnnot_GetBorder: *(library.get(b"FPDFAnnot_GetBorder\0")?),
                extern_FPDFAnnot_HasKey: *(library.get(b"FPDFAnnot_HasKey\0")?),
                extern_FPDFAnnot_GetValueType: *(library.get(b"FPDFAnnot_GetValueType\0")?),
                extern_FPDFAnnot_SetStringValue: *(library.get(b"FPDFAnnot_SetStringValue\0")?),
                extern_FPDFAnnot_GetStringValue: *(library.get(b"FPDFAnnot_GetStringValue\0")?),
                extern_FPDFAnnot_GetNumberValue: *(library.get(b"FPDFAnnot_GetNumberValue\0")?),
                extern_FPDFAnnot_SetAP: *(library.get(b"FPDFAnnot_SetAP\0")?),
                extern_FPDFAnnot_GetAP: *(library.get(b"FPDFAnnot_GetAP\0")?),
                extern_FPDFAnnot_GetLinkedAnnot: *(library.get(b"FPDFAnnot_GetLinkedAnnot\0")?),
                extern_FPDFAnnot_GetFlags: *(library.get(b"FPDFAnnot_GetFlags\0")?),
                extern_FPDFAnnot_SetFlags: *(library.get(b"FPDFAnnot_SetFlags\0")?),
                extern_FPDFAnnot_GetFormFieldFlags: *(library
                    .get(b"FPDFAnnot_GetFormFieldFlags\0")?),
                extern_FPDFAnnot_GetFormFieldAtPoint: *(library
                    .get(b"FPDFAnnot_GetFormFieldAtPoint\0")?),
                extern_FPDFAnnot_GetFormFieldName: *(library
                    .get(b"FPDFAnnot_GetFormFieldName\0")?),
                extern_FPDFAnnot_GetFormFieldType: *(library
                    .get(b"FPDFAnnot_GetFormFieldType\0")?),
                extern_FPDFAnnot_GetFormFieldValue: *(library
                    .get(b"FPDFAnnot_GetFormFieldValue\0")?),
                extern_FPDFAnnot_GetOptionCount: *(library.get(b"FPDFAnnot_GetOptionCount\0")?),
                extern_FPDFAnnot_GetOptionLabel: *(library.get(b"FPDFAnnot_GetOptionLabel\0")?),
                extern_FPDFAnnot_IsOptionSelected: *(library
                    .get(b"FPDFAnnot_IsOptionSelected\0")?),
                extern_FPDFAnnot_GetFontSize: *(library.get(b"FPDFAnnot_GetFontSize\0")?),
                extern_FPDFAnnot_IsChecked: *(library.get(b"FPDFAnnot_IsChecked\0")?),
                extern_FPDFAnnot_SetFocusableSubtypes: *(library
                    .get(b"FPDFAnnot_SetFocusableSubtypes\0")?),
                extern_FPDFAnnot_GetFocusableSubtypesCount: *(library
                    .get(b"FPDFAnnot_GetFocusableSubtypesCount\0")?),
                extern_FPDFAnnot_GetFocusableSubtypes: *(library
                    .get(b"FPDFAnnot_GetFocusableSubtypes\0")?),
                extern_FPDFAnnot_GetLink: *(library.get(b"FPDFAnnot_GetLink\0")?),
                extern_FPDFAnnot_GetFormControlCount: *(library
                    .get(b"FPDFAnnot_GetFormControlCount\0")?),
                extern_FPDFAnnot_GetFormControlIndex: *(library
                    .get(b"FPDFAnnot_GetFormControlIndex\0")?),
                extern_FPDFAnnot_GetFormFieldExportValue: *(library
                    .get(b"FPDFAnnot_GetFormFieldExportValue\0")?),
                extern_FPDFAnnot_SetURI: *(library.get(b"FPDFAnnot_SetURI\0")?),
                extern_FPDFDOC_InitFormFillEnvironment: *(library
                    .get(b"FPDFDOC_InitFormFillEnvironment\0")?),
                extern_FPDFDOC_ExitFormFillEnvironment: *(library
                    .get(b"FPDFDOC_ExitFormFillEnvironment\0")?),
                extern_FORM_OnAfterLoadPage: *(library.get(b"FORM_OnAfterLoadPage\0")?),
                extern_FORM_OnBeforeClosePage: *(library.get(b"FORM_OnBeforeClosePage\0")?),
                extern_FPDFDoc_GetPageMode: *(library.get(b"FPDFDoc_GetPageMode\0")?),
                extern_FPDFPage_Flatten: *(library.get(b"FPDFPage_Flatten\0")?),
                extern_FPDF_SetFormFieldHighlightColor: *(library
                    .get(b"FPDF_SetFormFieldHighlightColor\0")?),
                extern_FPDF_SetFormFieldHighlightAlpha: *(library
                    .get(b"FPDF_SetFormFieldHighlightAlpha\0")?),
                extern_FPDF_FFLDraw: *(library.get(b"FPDF_FFLDraw\0")?),
                extern_FPDF_GetFormType: *(library.get(b"FPDF_GetFormType\0")?),
                extern_FPDFBookmark_GetFirstChild: *(library
                    .get(b"FPDFBookmark_GetFirstChild\0")?),
                extern_FPDFBookmark_GetNextSibling: *(library
                    .get(b"FPDFBookmark_GetNextSibling\0")?),
                extern_FPDFBookmark_GetTitle: *(library.get(b"FPDFBookmark_GetTitle\0")?),
                extern_FPDFBookmark_GetCount: *(library.get(b"FPDFBookmark_GetCount\0")?),
                extern_FPDFBookmark_Find: *(library.get(b"FPDFBookmark_Find\0")?),
                extern_FPDFBookmark_GetDest: *(library.get(b"FPDFBookmark_GetDest\0")?),
                extern_FPDFBookmark_GetAction: *(library.get(b"FPDFBookmark_GetAction\0")?),
                extern_FPDFAction_GetType: *(library.get(b"FPDFAction_GetType\0")?),
                extern_FPDFAction_GetDest: *(library.get(b"FPDFAction_GetDest\0")?),
                extern_FPDFAction_GetFilePath: *(library.get(b"FPDFAction_GetFilePath\0")?),
                extern_FPDFAction_GetURIPath: *(library.get(b"FPDFAction_GetURIPath\0")?),
                extern_FPDFDest_GetDestPageIndex: *(library.get(b"FPDFDest_GetDestPageIndex\0")?),
                extern_FPDFDest_GetView: *(library.get(b"FPDFDest_GetView\0")?),
                extern_FPDFDest_GetLocationInPage: *(library
                    .get(b"FPDFDest_GetLocationInPage\0")?),
                extern_FPDFLink_GetLinkAtPoint: *(library.get(b"FPDFLink_GetLinkAtPoint\0")?),
                extern_FPDFLink_GetLinkZOrderAtPoint: *(library
                    .get(b"FPDFLink_GetLinkZOrderAtPoint\0")?),
                extern_FPDFLink_GetDest: *(library.get(b"FPDFLink_GetDest\0")?),
                extern_FPDFLink_GetAction: *(library.get(b"FPDFLink_GetAction\0")?),
                extern_FPDFLink_Enumerate: *(library.get(b"FPDFLink_Enumerate\0")?),
                extern_FPDFLink_GetAnnot: *(library.get(b"FPDFLink_GetAnnot\0")?),
                extern_FPDFLink_GetAnnotRect: *(library.get(b"FPDFLink_GetAnnotRect\0")?),
                extern_FPDFLink_CountQuadPoints: *(library.get(b"FPDFLink_CountQuadPoints\0")?),
                extern_FPDFLink_GetQuadPoints: *(library.get(b"FPDFLink_GetQuadPoints\0")?),
                extern_FPDF_GetPageAAction: *(library.get(b"FPDF_GetPageAAction\0")?),
                extern_FPDFText_LoadPage: *(library.get(b"FPDFText_LoadPage\0")?),
                extern_FPDFText_ClosePage: *(library.get(b"FPDFText_ClosePage\0")?),
                extern_FPDFText_CountChars: *(library.get(b"FPDFText_CountChars\0")?),
                extern_FPDFText_GetUnicode: *(library.get(b"FPDFText_GetUnicode\0")?),
                #[cfg(any(feature = "pdfium_6611", feature = "pdfium_future"))]
                extern_FPDFText_GetTextObject: *(library.get(b"FPDFText_GetTextObject\0")?),
                extern_FPDFText_GetFontSize: *(library.get(b"FPDFText_GetFontSize\0")?),
                extern_FPDFText_GetFontInfo: *(library.get(b"FPDFText_GetFontInfo\0")?),
                extern_FPDFText_GetFontWeight: *(library.get(b"FPDFText_GetFontWeight\0")?),
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
                extern_FPDFText_GetTextRenderMode: *(library
                    .get(b"FPDFText_GetTextRenderMode\0")?),
                extern_FPDFText_GetFillColor: *(library.get(b"FPDFText_GetFillColor\0")?),
                extern_FPDFText_GetStrokeColor: *(library.get(b"FPDFText_GetStrokeColor\0")?),
                extern_FPDFText_GetCharAngle: *(library.get(b"FPDFText_GetCharAngle\0")?),
                extern_FPDFText_GetCharBox: *(library.get(b"FPDFText_GetCharBox\0")?),
                extern_FPDFText_GetLooseCharBox: *(library.get(b"FPDFText_GetLooseCharBox\0")?),
                extern_FPDFText_GetMatrix: *(library.get(b"FPDFText_GetMatrix\0")?),
                extern_FPDFText_GetCharOrigin: *(library.get(b"FPDFText_GetCharOrigin\0")?),
                extern_FPDFText_GetCharIndexAtPos: *(library
                    .get(b"FPDFText_GetCharIndexAtPos\0")?),
                extern_FPDFText_GetText: *(library.get(b"FPDFText_GetText\0")?),
                extern_FPDFText_CountRects: *(library.get(b"FPDFText_CountRects\0")?),
                extern_FPDFText_GetRect: *(library.get(b"FPDFText_GetRect\0")?),
                extern_FPDFText_GetBoundedText: *(library.get(b"FPDFText_GetBoundedText\0")?),
                extern_FPDFText_FindStart: *(library.get(b"FPDFText_FindStart\0")?),
                extern_FPDFText_FindNext: *(library.get(b"FPDFText_FindNext\0")?),
                extern_FPDFText_FindPrev: *(library.get(b"FPDFText_FindPrev\0")?),
                extern_FPDFText_GetSchResultIndex: *(library
                    .get(b"FPDFText_GetSchResultIndex\0")?),
                extern_FPDFText_GetSchCount: *(library.get(b"FPDFText_GetSchCount\0")?),
                extern_FPDFText_FindClose: *(library.get(b"FPDFText_FindClose\0")?),
                extern_FPDFLink_LoadWebLinks: *(library.get(b"FPDFLink_LoadWebLinks\0")?),
                extern_FPDFLink_CountWebLinks: *(library.get(b"FPDFLink_CountWebLinks\0")?),
                extern_FPDFLink_GetURL: *(library.get(b"FPDFLink_GetURL\0")?),
                extern_FPDFLink_CountRects: *(library.get(b"FPDFLink_CountRects\0")?),
                extern_FPDFLink_GetRect: *(library.get(b"FPDFLink_GetRect\0")?),
                extern_FPDFLink_GetTextRange: *(library.get(b"FPDFLink_GetTextRange\0")?),
                extern_FPDFLink_CloseWebLinks: *(library.get(b"FPDFLink_CloseWebLinks\0")?),
                extern_FPDFPage_GetDecodedThumbnailData: *(library
                    .get(b"FPDFPage_GetDecodedThumbnailData\0")?),
                extern_FPDFPage_GetRawThumbnailData: *(library
                    .get(b"FPDFPage_GetRawThumbnailData\0")?),
                extern_FPDFPage_GetThumbnailAsBitmap: *(library
                    .get(b"FPDFPage_GetThumbnailAsBitmap\0")?),
                extern_FPDFFormObj_CountObjects: *(library.get(b"FPDFFormObj_CountObjects\0")?),
                extern_FPDFFormObj_GetObject: *(library.get(b"FPDFFormObj_GetObject\0")?),
                extern_FPDFPageObj_CreateTextObj: *(library.get(b"FPDFPageObj_CreateTextObj\0")?),
                extern_FPDFTextObj_GetTextRenderMode: *(library
                    .get(b"FPDFTextObj_GetTextRenderMode\0")?),
                extern_FPDFTextObj_SetTextRenderMode: *(library
                    .get(b"FPDFTextObj_SetTextRenderMode\0")?),
                extern_FPDFTextObj_GetText: *(library.get(b"FPDFTextObj_GetText\0")?),
                extern_FPDFTextObj_GetFont: *(library.get(b"FPDFTextObj_GetFont\0")?),
                extern_FPDFTextObj_GetFontSize: *(library.get(b"FPDFTextObj_GetFontSize\0")?),
                extern_FPDFPageObj_NewTextObj: *(library.get(b"FPDFPageObj_NewTextObj\0")?),
                extern_FPDFText_SetText: *(library.get(b"FPDFText_SetText\0")?),
                extern_FPDFText_SetCharcodes: *(library.get(b"FPDFText_SetCharcodes\0")?),
                extern_FPDFText_LoadFont: *(library.get(b"FPDFText_LoadFont\0")?),
                extern_FPDFText_LoadStandardFont: *(library.get(b"FPDFText_LoadStandardFont\0")?),
                extern_FPDFFont_Close: *(library.get(b"FPDFFont_Close\0")?),
                extern_FPDFPath_MoveTo: *(library.get(b"FPDFPath_MoveTo\0")?),
                extern_FPDFPath_LineTo: *(library.get(b"FPDFPath_LineTo\0")?),
                extern_FPDFPath_BezierTo: *(library.get(b"FPDFPath_BezierTo\0")?),
                extern_FPDFPath_Close: *(library.get(b"FPDFPath_Close\0")?),
                extern_FPDFPath_SetDrawMode: *(library.get(b"FPDFPath_SetDrawMode\0")?),
                extern_FPDFPath_GetDrawMode: *(library.get(b"FPDFPath_GetDrawMode\0")?),
                extern_FPDFPage_InsertObject: *(library.get(b"FPDFPage_InsertObject\0")?),
                extern_FPDFPage_RemoveObject: *(library.get(b"FPDFPage_RemoveObject\0")?),
                extern_FPDFPage_CountObjects: *(library.get(b"FPDFPage_CountObjects\0")?),
                extern_FPDFPage_GetObject: *(library.get(b"FPDFPage_GetObject\0")?),
                extern_FPDFPageObj_Destroy: *(library.get(b"FPDFPageObj_Destroy\0")?),
                extern_FPDFPageObj_HasTransparency: *(library
                    .get(b"FPDFPageObj_HasTransparency\0")?),
                extern_FPDFPageObj_GetType: *(library.get(b"FPDFPageObj_GetType\0")?),
                extern_FPDFPageObj_Transform: *(library.get(b"FPDFPageObj_Transform\0")?),
                extern_FPDFPageObj_GetMatrix: *(library.get(b"FPDFPageObj_GetMatrix\0")?),
                extern_FPDFPageObj_SetMatrix: *(library.get(b"FPDFPageObj_SetMatrix\0")?),
                extern_FPDFPageObj_NewImageObj: *(library.get(b"FPDFPageObj_NewImageObj\0")?),
                extern_FPDFPageObj_CountMarks: *(library.get(b"FPDFPageObj_CountMarks\0")?),
                extern_FPDFPageObj_GetMark: *(library.get(b"FPDFPageObj_GetMark\0")?),
                extern_FPDFPageObj_AddMark: *(library.get(b"FPDFPageObj_AddMark\0")?),
                extern_FPDFPageObj_RemoveMark: *(library.get(b"FPDFPageObj_RemoveMark\0")?),
                extern_FPDFPageObjMark_GetName: *(library.get(b"FPDFPageObjMark_GetName\0")?),
                extern_FPDFPageObjMark_CountParams: *(library
                    .get(b"FPDFPageObjMark_CountParams\0")?),
                extern_FPDFPageObjMark_GetParamKey: *(library
                    .get(b"FPDFPageObjMark_GetParamKey\0")?),
                extern_FPDFPageObjMark_GetParamValueType: *(library
                    .get(b"FPDFPageObjMark_GetParamValueType\0")?),
                extern_FPDFPageObjMark_GetParamIntValue: *(library
                    .get(b"FPDFPageObjMark_GetParamIntValue\0")?),
                extern_FPDFPageObjMark_GetParamStringValue: *(library
                    .get(b"FPDFPageObjMark_GetParamStringValue\0")?),
                extern_FPDFPageObjMark_GetParamBlobValue: *(library
                    .get(b"FPDFPageObjMark_GetParamBlobValue\0")?),
                extern_FPDFPageObjMark_SetIntParam: *(library
                    .get(b"FPDFPageObjMark_SetIntParam\0")?),
                extern_FPDFPageObjMark_SetStringParam: *(library
                    .get(b"FPDFPageObjMark_SetStringParam\0")?),
                extern_FPDFPageObjMark_SetBlobParam: *(library
                    .get(b"FPDFPageObjMark_SetBlobParam\0")?),
                extern_FPDFPageObjMark_RemoveParam: *(library
                    .get(b"FPDFPageObjMark_RemoveParam\0")?),
                extern_FPDFImageObj_LoadJpegFile: *(library.get(b"FPDFImageObj_LoadJpegFile\0")?),
                extern_FPDFImageObj_LoadJpegFileInline: *(library
                    .get(b"FPDFImageObj_LoadJpegFileInline\0")?),
                extern_FPDFImageObj_SetMatrix: *(library.get(b"FPDFImageObj_SetMatrix\0")?),
                extern_FPDFImageObj_SetBitmap: *(library.get(b"FPDFImageObj_SetBitmap\0")?),
                extern_FPDFImageObj_GetBitmap: *(library.get(b"FPDFImageObj_GetBitmap\0")?),
                extern_FPDFImageObj_GetRenderedBitmap: *(library
                    .get(b"FPDFImageObj_GetRenderedBitmap\0")?),
                extern_FPDFImageObj_GetImageDataDecoded: *(library
                    .get(b"FPDFImageObj_GetImageDataDecoded\0")?),
                extern_FPDFImageObj_GetImageDataRaw: *(library
                    .get(b"FPDFImageObj_GetImageDataRaw\0")?),
                extern_FPDFImageObj_GetImageFilterCount: *(library
                    .get(b"FPDFImageObj_GetImageFilterCount\0")?),
                extern_FPDFImageObj_GetImageFilter: *(library
                    .get(b"FPDFImageObj_GetImageFilter\0")?),
                extern_FPDFImageObj_GetImageMetadata: *(library
                    .get(b"FPDFImageObj_GetImageMetadata\0")?),
                extern_FPDFPageObj_CreateNewPath: *(library.get(b"FPDFPageObj_CreateNewPath\0")?),
                extern_FPDFPageObj_CreateNewRect: *(library.get(b"FPDFPageObj_CreateNewRect\0")?),
                extern_FPDFPageObj_GetBounds: *(library.get(b"FPDFPageObj_GetBounds\0")?),
                extern_FPDFPageObj_SetBlendMode: *(library.get(b"FPDFPageObj_SetBlendMode\0")?),
                extern_FPDFPageObj_SetStrokeColor: *(library
                    .get(b"FPDFPageObj_SetStrokeColor\0")?),
                extern_FPDFPageObj_GetStrokeColor: *(library
                    .get(b"FPDFPageObj_GetStrokeColor\0")?),
                extern_FPDFPageObj_SetStrokeWidth: *(library
                    .get(b"FPDFPageObj_SetStrokeWidth\0")?),
                extern_FPDFPageObj_GetStrokeWidth: *(library
                    .get(b"FPDFPageObj_GetStrokeWidth\0")?),
                extern_FPDFPageObj_GetLineJoin: *(library.get(b"FPDFPageObj_GetLineJoin\0")?),
                extern_FPDFPageObj_SetLineJoin: *(library.get(b"FPDFPageObj_SetLineJoin\0")?),
                extern_FPDFPageObj_GetLineCap: *(library.get(b"FPDFPageObj_GetLineCap\0")?),
                extern_FPDFPageObj_SetLineCap: *(library.get(b"FPDFPageObj_SetLineCap\0")?),
                extern_FPDFPageObj_SetFillColor: *(library.get(b"FPDFPageObj_SetFillColor\0")?),
                extern_FPDFPageObj_GetFillColor: *(library.get(b"FPDFPageObj_GetFillColor\0")?),
                extern_FPDFPageObj_GetDashPhase: *(library.get(b"FPDFPageObj_GetDashPhase\0")?),
                extern_FPDFPageObj_SetDashPhase: *(library.get(b"FPDFPageObj_SetDashPhase\0")?),
                extern_FPDFPageObj_GetDashCount: *(library.get(b"FPDFPageObj_GetDashCount\0")?),
                extern_FPDFPageObj_GetDashArray: *(library.get(b"FPDFPageObj_GetDashArray\0")?),
                extern_FPDFPageObj_SetDashArray: *(library.get(b"FPDFPageObj_SetDashArray\0")?),
                extern_FPDFPath_CountSegments: *(library.get(b"FPDFPath_CountSegments\0")?),
                extern_FPDFPath_GetPathSegment: *(library.get(b"FPDFPath_GetPathSegment\0")?),
                extern_FPDFPathSegment_GetPoint: *(library.get(b"FPDFPathSegment_GetPoint\0")?),
                extern_FPDFPathSegment_GetType: *(library.get(b"FPDFPathSegment_GetType\0")?),
                extern_FPDFPathSegment_GetClose: *(library.get(b"FPDFPathSegment_GetClose\0")?),
                // TODO: AJRC - 4-Aug-2024 - FPDFFont_GetBaseFontName() is in Pdfium export headers
                // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
                #[cfg(feature = "pdfium_future")]
                extern_FPDFFont_GetBaseFontName: *(library.get(b"FPDFFont_GetBaseFontName\0")?),
                #[cfg(any(feature = "pdfium_future", feature = "pdfium_6611"))]
                extern_FPDFFont_GetFamilyName: *(library.get(b"FPDFFont_GetFamilyName\0")?),
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
                extern_FPDFFont_GetFontName: *(library.get(b"FPDFFont_GetFontName\0")?),
                extern_FPDFFont_GetFontData: *(library.get(b"FPDFFont_GetFontData\0")?),
                extern_FPDFFont_GetIsEmbedded: *(library.get(b"FPDFFont_GetIsEmbedded\0")?),
                extern_FPDFFont_GetFlags: *(library.get(b"FPDFFont_GetFlags\0")?),
                extern_FPDFFont_GetWeight: *(library.get(b"FPDFFont_GetWeight\0")?),
                extern_FPDFFont_GetItalicAngle: *(library.get(b"FPDFFont_GetItalicAngle\0")?),
                extern_FPDFFont_GetAscent: *(library.get(b"FPDFFont_GetAscent\0")?),
                extern_FPDFFont_GetDescent: *(library.get(b"FPDFFont_GetDescent\0")?),
                extern_FPDFFont_GetGlyphWidth: *(library.get(b"FPDFFont_GetGlyphWidth\0")?),
                extern_FPDFFont_GetGlyphPath: *(library.get(b"FPDFFont_GetGlyphPath\0")?),
                extern_FPDFGlyphPath_CountGlyphSegments: *(library
                    .get(b"FPDFGlyphPath_CountGlyphSegments\0")?),
                extern_FPDFGlyphPath_GetGlyphPathSegment: *(library
                    .get(b"FPDFGlyphPath_GetGlyphPathSegment\0")?),
                extern_FPDF_VIEWERREF_GetPrintScaling: *(library
                    .get(b"FPDF_VIEWERREF_GetPrintScaling\0")?),
                extern_FPDF_VIEWERREF_GetNumCopies: *(library
                    .get(b"FPDF_VIEWERREF_GetNumCopies\0")?),
                extern_FPDF_VIEWERREF_GetPrintPageRange: *(library
                    .get(b"FPDF_VIEWERREF_GetPrintPageRange\0")?),
                extern_FPDF_VIEWERREF_GetPrintPageRangeCount: *(library
                    .get(b"FPDF_VIEWERREF_GetPrintPageRangeCount\0")?),
                extern_FPDF_VIEWERREF_GetPrintPageRangeElement: *(library
                    .get(b"FPDF_VIEWERREF_GetPrintPageRangeElement\0")?),
                extern_FPDF_VIEWERREF_GetDuplex: *(library.get(b"FPDF_VIEWERREF_GetDuplex\0")?),
                extern_FPDF_VIEWERREF_GetName: *(library.get(b"FPDF_VIEWERREF_GetName\0")?),
                extern_FPDFDoc_GetAttachmentCount: *(library
                    .get(b"FPDFDoc_GetAttachmentCount\0")?),
                extern_FPDFDoc_AddAttachment: *(library.get(b"FPDFDoc_AddAttachment\0")?),
                extern_FPDFDoc_GetAttachment: *(library.get(b"FPDFDoc_GetAttachment\0")?),
                extern_FPDFDoc_DeleteAttachment: *(library.get(b"FPDFDoc_DeleteAttachment\0")?),
                extern_FPDFAttachment_GetName: *(library.get(b"FPDFAttachment_GetName\0")?),
                extern_FPDFAttachment_HasKey: *(library.get(b"FPDFAttachment_HasKey\0")?),
                extern_FPDFAttachment_GetValueType: *(library
                    .get(b"FPDFAttachment_GetValueType\0")?),
                extern_FPDFAttachment_SetStringValue: *(library
                    .get(b"FPDFAttachment_SetStringValue\0")?),
                extern_FPDFAttachment_GetStringValue: *(library
                    .get(b"FPDFAttachment_GetStringValue\0")?),
                extern_FPDFAttachment_SetFile: *(library.get(b"FPDFAttachment_SetFile\0")?),
                extern_FPDFAttachment_GetFile: *(library.get(b"FPDFAttachment_GetFile\0")?),
                extern_FPDFCatalog_IsTagged: *(library.get(b"FPDFCatalog_IsTagged\0")?),
                library,
            }
        };

        Ok(result)
    }
}

impl PdfiumLibraryBindings for DynamicPdfiumBindings {
    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_InitLibrary(&self) {
        unsafe {
            (self.extern_FPDF_InitLibrary)();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyLibrary(&self) {
        unsafe {
            (self.extern_FPDF_DestroyLibrary)();
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetLastError(&self) -> c_ulong {
        unsafe { (self.extern_FPDF_GetLastError)() }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        unsafe { (self.extern_FPDF_CreateNewDocument)() }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadDocument(&self, file_path: &str, password: Option<&str>) -> FPDF_DOCUMENT {
        let c_file_path = CString::new(file_path).unwrap();
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe { (self.extern_FPDF_LoadDocument)(c_file_path.as_ptr(), c_password.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadMemDocument64(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            (self.extern_FPDF_LoadMemDocument64)(
                bytes.as_ptr() as *const c_void,
                bytes.len() as c_ulong,
                c_password.as_ptr(),
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadCustomDocument(
        &self,
        pFileAccess: *mut FPDF_FILEACCESS,
        password: Option<&str>,
    ) -> FPDF_DOCUMENT {
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe { (self.extern_FPDF_LoadCustomDocument)(pFileAccess, c_password.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDF_SaveAsCopy)(document, pFileWrite, flags) }
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
        unsafe { (self.extern_FPDF_SaveWithVersion)(document, pFileWrite, flags, fileVersion) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CloseDocument(&self, document: FPDF_DOCUMENT) {
        unsafe {
            (self.extern_FPDF_CloseDocument)(document);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
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
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDF_DeviceToPage)(
                page, start_x, start_y, size_x, size_y, rotate, device_x, device_y, page_x, page_y,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
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
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDF_PageToDevice)(
                page, start_x, start_y, size_x, size_y, rotate, page_x, page_y, device_x, device_y,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDF_GetFileVersion)(doc, fileVersion) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileIdentifier(
        &self,
        document: FPDF_DOCUMENT,
        id_type: FPDF_FILEIDTYPE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDF_GetFileIdentifier)(document, id_type, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFormType(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDF_GetFormType)(document) }
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

        unsafe { (self.extern_FPDF_GetMetaText)(document, c_tag.as_ptr(), buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> c_ulong {
        unsafe { (self.extern_FPDF_GetDocPermissions)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSecurityHandlerRevision(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDF_GetSecurityHandlerRevision)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDF_GetPageCount)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_LoadPage(&self, document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE {
        unsafe { (self.extern_FPDF_LoadPage)(document, page_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_ClosePage(&self, page: FPDF_PAGE) {
        unsafe {
            (self.extern_FPDF_ClosePage)(page);
        }
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
        unsafe {
            (self.extern_FPDF_ImportPagesByIndex)(dest_doc, src_doc, page_indices, length, index)
        }
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
        let c_pagerange = CString::new(pagerange).unwrap();

        unsafe { (self.extern_FPDF_ImportPages)(dest_doc, src_doc, c_pagerange.as_ptr(), index) }
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
        unsafe {
            (self.extern_FPDF_ImportNPagesToOne)(
                src_doc,
                output_width,
                output_height,
                num_pages_on_x_axis,
                num_pages_on_y_axis,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageWidthF(&self, page: FPDF_PAGE) -> c_float {
        unsafe { (self.extern_FPDF_GetPageWidthF)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageHeightF(&self, page: FPDF_PAGE) -> c_float {
        unsafe { (self.extern_FPDF_GetPageHeightF)(page) }
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
        unsafe { (self.extern_FPDF_GetPageLabel)(document, page_index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexFromTextIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nTextIndex: c_int,
    ) -> c_int {
        unsafe { (self.extern_FPDFText_GetCharIndexFromTextIndex)(text_page, nTextIndex) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextIndexFromCharIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nCharIndex: c_int,
    ) -> c_int {
        unsafe { (self.extern_FPDFText_GetTextIndexFromCharIndex)(text_page, nCharIndex) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSignatureCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDF_GetSignatureCount)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSignatureObject(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE {
        unsafe { (self.extern_FPDF_GetSignatureObject)(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetContents(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFSignatureObj_GetContents)(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetByteRange(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_int,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFSignatureObj_GetByteRange)(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetSubFilter(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFSignatureObj_GetSubFilter)(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetReason(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFSignatureObj_GetReason)(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetTime(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFSignatureObj_GetTime)(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetDocMDPPermission(&self, signature: FPDF_SIGNATURE) -> c_uint {
        unsafe { (self.extern_FPDFSignatureObj_GetDocMDPPermission)(signature) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetForPage(&self, page: FPDF_PAGE) -> FPDF_STRUCTTREE {
        unsafe { (self.extern_FPDF_StructTree_GetForPage)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE) {
        unsafe { (self.extern_FPDF_StructTree_Close)(struct_tree) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_CountChildren(&self, struct_tree: FPDF_STRUCTTREE) -> c_int {
        unsafe { (self.extern_FPDF_StructTree_CountChildren)(struct_tree) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: FPDF_STRUCTTREE,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT {
        unsafe { (self.extern_FPDF_StructTree_GetChildAtIndex)(struct_tree, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetAltText(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDF_StructElement_GetAltText)(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDF_StructElement_GetID)(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetLang(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDF_StructElement_GetLang)(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetStringAttribute(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        attr_name: &str,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        let c_attr_name = CString::new(attr_name).unwrap();

        unsafe {
            (self.extern_FPDF_StructElement_GetStringAttribute)(
                struct_element,
                c_attr_name.as_ptr(),
                buffer,
                buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetMarkedContentID(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int {
        unsafe { (self.extern_FPDF_StructElement_GetMarkedContentID)(struct_element) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDF_StructElement_GetType)(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetTitle(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDF_StructElement_GetTitle)(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_CountChildren(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int {
        unsafe { (self.extern_FPDF_StructElement_CountChildren)(struct_element) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetChildAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT {
        unsafe { (self.extern_FPDF_StructElement_GetChildAtIndex)(struct_element, index) }
    }

    #[allow(non_snake_case)]
    fn FPDFPage_New(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: c_double,
        height: c_double,
    ) -> FPDF_PAGE {
        unsafe { (self.extern_FPDFPage_New)(document, page_index, width, height) }
    }

    #[allow(non_snake_case)]
    fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: c_int) {
        unsafe { (self.extern_FPDFPage_Delete)(document, page_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRotation(&self, page: FPDF_PAGE) -> c_int {
        unsafe { (self.extern_FPDFPage_GetRotation)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_SetRotation(&self, page: FPDF_PAGE, rotate: c_int) {
        unsafe { (self.extern_FPDFPage_SetRotation)(page, rotate) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageBoundingBox(&self, page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { (self.extern_FPDF_GetPageBoundingBox)(page, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageSizeByIndexF(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        size: *mut FS_SIZEF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDF_GetPageSizeByIndexF)(document, page_index, size) }
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
        unsafe { (self.extern_FPDFPage_GetMediaBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_GetCropBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_GetBleedBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_GetTrimBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_GetArtBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_SetMediaBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_SetCropBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_SetBleedBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_SetTrimBox)(page, left, bottom, right, top) }
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
        unsafe { (self.extern_FPDFPage_SetArtBox)(page, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_TransFormWithClip(
        &self,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPage_TransFormWithClip)(page, matrix, clipRect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_TransformClipPath(
        &self,
        page_object: FPDF_PAGEOBJECT,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        unsafe { (self.extern_FPDFPageObj_TransformClipPath)(page_object, a, b, c, d, e, f) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetClipPath(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH {
        unsafe { (self.extern_FPDFPageObj_GetClipPath)(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPaths(&self, clip_path: FPDF_CLIPPATH) -> c_int {
        unsafe { (self.extern_FPDFClipPath_CountPaths)(clip_path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPathSegments(&self, clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int {
        unsafe { (self.extern_FPDFClipPath_CountPathSegments)(clip_path, path_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT {
        unsafe { (self.extern_FPDFClipPath_GetPathSegment)(clip_path, path_index, segment_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CreateClipPath(&self, left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH {
        unsafe { (self.extern_FPDF_CreateClipPath)(left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyClipPath(&self, clipPath: FPDF_CLIPPATH) {
        unsafe { (self.extern_FPDF_DestroyClipPath)(clipPath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertClipPath(&self, page: FPDF_PAGE, clipPath: FPDF_CLIPPATH) {
        unsafe { (self.extern_FPDFPage_InsertClipPath)(page, clipPath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPage_HasTransparency)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPage_GenerateContent)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_TransformAnnots(
        &self,
        page: FPDF_PAGE,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) {
        unsafe { (self.extern_FPDFPage_TransformAnnots)(page, a, b, c, d, e, f) }
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
        unsafe { (self.extern_FPDFBitmap_CreateEx)(width, height, format, first_scan, stride) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_Destroy(&self, bitmap: FPDF_BITMAP) {
        unsafe { (self.extern_FPDFBitmap_Destroy)(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { (self.extern_FPDFBitmap_GetFormat)(bitmap) }
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
            (self.extern_FPDFBitmap_FillRect)(bitmap, left, top, width, height, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetBuffer(&self, bitmap: FPDF_BITMAP) -> *mut c_void {
        unsafe { (self.extern_FPDFBitmap_GetBuffer)(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetWidth(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { (self.extern_FPDFBitmap_GetWidth)(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetHeight(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { (self.extern_FPDFBitmap_GetHeight)(bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBitmap_GetStride(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { (self.extern_FPDFBitmap_GetStride)(bitmap) }
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
            (self.extern_FPDF_RenderPageBitmap)(
                bitmap, page, start_x, start_y, size_x, size_y, rotate, flags,
            );
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_RenderPageBitmapWithMatrix(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ) {
        unsafe {
            (self.extern_FPDF_RenderPageBitmapWithMatrix)(bitmap, page, matrix, clipping, flags);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_IsSupportedSubtype)(subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION {
        unsafe { (self.extern_FPDFPage_CreateAnnot)(page, subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> c_int {
        unsafe { (self.extern_FPDFPage_GetAnnotCount)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION {
        unsafe { (self.extern_FPDFPage_GetAnnot)(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { (self.extern_FPDFPage_GetAnnotIndex)(page, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) {
        unsafe { (self.extern_FPDFPage_CloseAnnot)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPage_RemoveAnnot)(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE {
        unsafe { (self.extern_FPDFAnnot_GetSubtype)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_IsObjectSupportedSubtype)(subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_UpdateObject)(annot, obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int {
        unsafe { (self.extern_FPDFAnnot_AddInkStroke)(annot, points, point_count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_RemoveInkList)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_AppendObject)(annot, obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetObjectCount)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFAnnot_GetObject)(annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_RemoveObject)(annot, index) }
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
        unsafe { (self.extern_FPDFAnnot_SetColor)(annot, color_type, R, G, B, A) }
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
        unsafe { (self.extern_FPDFAnnot_GetColor)(annot, color_type, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_HasAttachmentPoints)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_SetAttachmentPoints)(annot, quad_index, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_AppendAttachmentPoints)(annot, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> size_t {
        unsafe { (self.extern_FPDFAnnot_CountAttachmentPoints)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_GetAttachmentPoints)(annot, quad_index, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_SetRect)(annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_GetRect)(annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFAnnot_GetVertices)(annot, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> c_ulong {
        unsafe { (self.extern_FPDFAnnot_GetInkListCount)(annot) }
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
        unsafe { (self.extern_FPDFAnnot_GetInkListPath)(annot, path_index, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_GetLine)(annot, start, end) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: f32,
        vertical_radius: f32,
        border_width: f32,
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDFAnnot_SetBorder)(
                annot,
                horizontal_radius,
                vertical_radius,
                border_width,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetBorder(
        &self,
        annot: FPDF_ANNOTATION,
        horizontal_radius: *mut f32,
        vertical_radius: *mut f32,
        border_width: *mut f32,
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDFAnnot_GetBorder)(
                annot,
                horizontal_radius,
                vertical_radius,
                border_width,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasKey(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAnnot_HasKey)(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAnnot_GetValueType)(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetStringValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAnnot_SetStringValue)(annot, c_key.as_ptr(), value) }
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
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAnnot_GetStringValue)(annot, c_key.as_ptr(), buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetNumberValue(
        &self,
        annot: FPDF_ANNOTATION,
        key: &str,
        value: *mut f32,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAnnot_GetNumberValue)(annot, c_key.as_ptr(), value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_SetAP)(annot, appearanceMode, value) }
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
        unsafe { (self.extern_FPDFAnnot_GetAP)(annot, appearanceMode, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_ANNOTATION {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAnnot_GetLinkedAnnot)(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetFlags)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_SetFlags)(annot, flags) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetFormFieldFlags)(handle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION {
        unsafe { (self.extern_FPDFAnnot_GetFormFieldAtPoint)(hHandle, page, point) }
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
        unsafe { (self.extern_FPDFAnnot_GetFormFieldName)(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldType(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetFormFieldType)(hHandle, annot) }
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
        unsafe { (self.extern_FPDFAnnot_GetFormFieldValue)(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetOptionCount)(hHandle, annot) }
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
        unsafe { (self.extern_FPDFAnnot_GetOptionLabel)(hHandle, annot, index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_IsOptionSelected)(handle, annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_GetFontSize)(hHandle, annot, value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_IsChecked)(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_SetFocusableSubtypes)(hHandle, subtypes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetFocusableSubtypesCount)(hHandle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAnnot_GetFocusableSubtypes)(hHandle, subtypes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK {
        unsafe { (self.extern_FPDFAnnot_GetLink)(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetFormControlCount)(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { (self.extern_FPDFAnnot_GetFormControlIndex)(hHandle, annot) }
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
        unsafe { (self.extern_FPDFAnnot_GetFormFieldExportValue)(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: &str) -> FPDF_BOOL {
        let c_uri = CString::new(uri).unwrap();

        unsafe { (self.extern_FPDFAnnot_SetURI)(annot, c_uri.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_InitFormFillEnvironment(
        &self,
        document: FPDF_DOCUMENT,
        form_info: *mut FPDF_FORMFILLINFO,
    ) -> FPDF_FORMHANDLE {
        unsafe { (self.extern_FPDFDOC_InitFormFillEnvironment)(document, form_info) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDOC_ExitFormFillEnvironment(&self, handle: FPDF_FORMHANDLE) {
        unsafe {
            (self.extern_FPDFDOC_ExitFormFillEnvironment)(handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FORM_OnAfterLoadPage(&self, page: FPDF_PAGE, handle: FPDF_FORMHANDLE) {
        unsafe {
            (self.extern_FORM_OnAfterLoadPage)(page, handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FORM_OnBeforeClosePage(&self, page: FPDF_PAGE, handle: FPDF_FORMHANDLE) {
        unsafe {
            (self.extern_FORM_OnBeforeClosePage)(page, handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDFDoc_GetPageMode)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_Flatten(&self, page: FPDF_PAGE, nFlag: c_int) -> c_int {
        unsafe { (self.extern_FPDFPage_Flatten)(page, nFlag) }
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
            (self.extern_FPDF_SetFormFieldHighlightColor)(handle, field_type, color);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SetFormFieldHighlightAlpha(&self, handle: FPDF_FORMHANDLE, alpha: c_uchar) {
        unsafe {
            (self.extern_FPDF_SetFormFieldHighlightAlpha)(handle, alpha);
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
            (self.extern_FPDF_FFLDraw)(
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
        unsafe { (self.extern_FPDFBookmark_GetFirstChild)(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetNextSibling(
        &self,
        document: FPDF_DOCUMENT,
        bookmark: FPDF_BOOKMARK,
    ) -> FPDF_BOOKMARK {
        unsafe { (self.extern_FPDFBookmark_GetNextSibling)(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetTitle(
        &self,
        bookmark: FPDF_BOOKMARK,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFBookmark_GetTitle)(bookmark, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetCount(&self, bookmark: FPDF_BOOKMARK) -> c_int {
        unsafe { (self.extern_FPDFBookmark_GetCount)(bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_Find(&self, document: FPDF_DOCUMENT, title: FPDF_WIDESTRING) -> FPDF_BOOKMARK {
        unsafe { (self.extern_FPDFBookmark_Find)(document, title) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetDest(&self, document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST {
        unsafe { (self.extern_FPDFBookmark_GetDest)(document, bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFBookmark_GetAction(&self, bookmark: FPDF_BOOKMARK) -> FPDF_ACTION {
        unsafe { (self.extern_FPDFBookmark_GetAction)(bookmark) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetType(&self, action: FPDF_ACTION) -> c_ulong {
        unsafe { (self.extern_FPDFAction_GetType)(action) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetDest(&self, document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST {
        unsafe { (self.extern_FPDFAction_GetDest)(document, action) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAction_GetFilePath(
        &self,
        action: FPDF_ACTION,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFAction_GetFilePath)(action, buffer, buflen) }
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
        unsafe { (self.extern_FPDFAction_GetURIPath)(document, action, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetDestPageIndex(&self, document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int {
        unsafe { (self.extern_FPDFDest_GetDestPageIndex)(document, dest) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDest_GetView(
        &self,
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFDest_GetView)(dest, pNumParams, pParams) }
    }

    #[inline]
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
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDFDest_GetLocationInPage)(dest, hasXVal, hasYVal, hasZoomVal, x, y, zoom)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> FPDF_LINK {
        unsafe { (self.extern_FPDFLink_GetLinkAtPoint)(page, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkZOrderAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> c_int {
        unsafe { (self.extern_FPDFLink_GetLinkZOrderAtPoint)(page, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetDest(&self, document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST {
        unsafe { (self.extern_FPDFLink_GetDest)(document, link) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAction(&self, link: FPDF_LINK) -> FPDF_ACTION {
        unsafe { (self.extern_FPDFLink_GetAction)(link) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_Enumerate(
        &self,
        page: FPDF_PAGE,
        start_pos: *mut c_int,
        link_annot: *mut FPDF_LINK,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFLink_Enumerate)(page, start_pos, link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnot(&self, page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION {
        unsafe { (self.extern_FPDFLink_GetAnnot)(page, link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnotRect(&self, link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFLink_GetAnnotRect)(link_annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountQuadPoints(&self, link_annot: FPDF_LINK) -> c_int {
        unsafe { (self.extern_FPDFLink_CountQuadPoints)(link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetQuadPoints(
        &self,
        link_annot: FPDF_LINK,
        quad_index: c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFLink_GetQuadPoints)(link_annot, quad_index, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageAAction(&self, page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION {
        unsafe { (self.extern_FPDF_GetPageAAction)(page, aa_type) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadPage(&self, page: FPDF_PAGE) -> FPDF_TEXTPAGE {
        unsafe { (self.extern_FPDFText_LoadPage)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_ClosePage(&self, text_page: FPDF_TEXTPAGE) {
        unsafe {
            (self.extern_FPDFText_ClosePage)(text_page);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountChars(&self, text_page: FPDF_TEXTPAGE) -> c_int {
        unsafe { (self.extern_FPDFText_CountChars)(text_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetUnicode(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint {
        unsafe { (self.extern_FPDFText_GetUnicode)(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextObject(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFText_GetTextObject)(text_page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontSize(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_double {
        unsafe { (self.extern_FPDFText_GetFontSize)(text_page, index) }
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
        unsafe { (self.extern_FPDFText_GetFontInfo)(text_page, index, buffer, buflen, flags) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetFontWeight(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_int {
        unsafe { (self.extern_FPDFText_GetFontWeight)(text_page, index) }
    }

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
    ) -> FPDF_TEXT_RENDERMODE {
        unsafe { (self.extern_FPDFText_GetTextRenderMode)(text_page, index) }
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
        unsafe { (self.extern_FPDFText_GetFillColor)(text_page, index, R, G, B, A) }
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
        unsafe { (self.extern_FPDFText_GetStrokeColor)(text_page, index, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetCharAngle(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_float {
        unsafe { (self.extern_FPDFText_GetCharAngle)(text_page, index) }
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
        unsafe { (self.extern_FPDFText_GetCharBox)(text_page, index, left, right, bottom, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetLooseCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFText_GetLooseCharBox)(text_page, index, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetMatrix(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFText_GetMatrix)(text_page, index, matrix) }
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
        unsafe { (self.extern_FPDFText_GetCharOrigin)(text_page, index, x, y) }
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
        unsafe { (self.extern_FPDFText_GetCharIndexAtPos)(text_page, x, y, xTolerance, yTolerance) }
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
        unsafe { (self.extern_FPDFText_GetText)(text_page, start_index, count, result) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_CountRects(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
    ) -> c_int {
        unsafe { (self.extern_FPDFText_CountRects)(text_page, start_index, count) }
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
        unsafe { (self.extern_FPDFText_GetRect)(text_page, rect_index, left, top, right, bottom) }
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
            (self.extern_FPDFText_GetBoundedText)(
                text_page, left, top, right, bottom, buffer, buflen,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindStart(
        &self,
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE {
        unsafe { (self.extern_FPDFText_FindStart)(text_page, findwhat, flags, start_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindNext(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFText_FindNext)(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindPrev(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFText_FindPrev)(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetSchResultIndex(&self, handle: FPDF_SCHHANDLE) -> c_int {
        unsafe { (self.extern_FPDFText_GetSchResultIndex)(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetSchCount(&self, handle: FPDF_SCHHANDLE) -> c_int {
        unsafe { (self.extern_FPDFText_GetSchCount)(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE) {
        unsafe { (self.extern_FPDFText_FindClose)(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_LoadWebLinks(&self, text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK {
        unsafe { (self.extern_FPDFLink_LoadWebLinks)(text_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountWebLinks(&self, link_page: FPDF_PAGELINK) -> c_int {
        unsafe { (self.extern_FPDFLink_CountWebLinks)(link_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetURL(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        buffer: *mut c_ushort,
        buflen: c_int,
    ) -> c_int {
        unsafe { (self.extern_FPDFLink_GetURL)(link_page, link_index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountRects(&self, link_page: FPDF_PAGELINK, link_index: c_int) -> c_int {
        unsafe { (self.extern_FPDFLink_CountRects)(link_page, link_index) }
    }

    #[inline]
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
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDFLink_GetRect)(
                link_page, link_index, rect_index, left, top, right, bottom,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetTextRange(
        &self,
        link_page: FPDF_PAGELINK,
        link_index: c_int,
        start_char_index: *mut c_int,
        char_count: *mut c_int,
    ) -> FPDF_BOOL {
        unsafe {
            (self.extern_FPDFLink_GetTextRange)(link_page, link_index, start_char_index, char_count)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK) {
        unsafe { (self.extern_FPDFLink_CloseWebLinks)(link_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetDecodedThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFPage_GetDecodedThumbnailData)(page, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRawThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFPage_GetRawThumbnailData)(page, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetThumbnailAsBitmap(&self, page: FPDF_PAGE) -> FPDF_BITMAP {
        unsafe { (self.extern_FPDFPage_GetThumbnailAsBitmap)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_CountObjects(&self, form_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFFormObj_CountObjects)(form_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFormObj_GetObject(
        &self,
        form_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFFormObj_GetObject)(form_object, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: FPDF_FONT,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFPageObj_CreateTextObj)(document, font, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetTextRenderMode(&self, text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE {
        unsafe { (self.extern_FPDFTextObj_GetTextRenderMode)(text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_SetTextRenderMode(
        &self,
        text: FPDF_PAGEOBJECT,
        render_mode: FPDF_TEXT_RENDERMODE,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFTextObj_SetTextRenderMode)(text, render_mode) }
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
        unsafe { (self.extern_FPDFTextObj_GetText)(text_object, text_page, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFont(&self, text: FPDF_PAGEOBJECT) -> FPDF_FONT {
        unsafe { (self.extern_FPDFTextObj_GetFont)(text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFTextObj_GetFontSize(&self, text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFTextObj_GetFontSize)(text, size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewTextObj(
        &self,
        document: FPDF_DOCUMENT,
        font: &str,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        let c_font = CString::new(font).unwrap();

        unsafe { (self.extern_FPDFPageObj_NewTextObj)(document, c_font.as_ptr(), font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetText(&self, text_object: FPDF_PAGEOBJECT, text: FPDF_WIDESTRING) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFText_SetText)(text_object, text) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_SetCharcodes(
        &self,
        text_object: FPDF_PAGEOBJECT,
        charcodes: *const c_uint,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFText_SetCharcodes)(text_object, charcodes, count) }
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
        unsafe { (self.extern_FPDFText_LoadFont)(document, data, size, font_type, cid) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadStandardFont(&self, document: FPDF_DOCUMENT, font: &str) -> FPDF_FONT {
        let c_font = CString::new(font).unwrap();

        unsafe { (self.extern_FPDFText_LoadStandardFont)(document, c_font.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_Close(&self, font: FPDF_FONT) {
        unsafe { (self.extern_FPDFFont_Close)(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPath_MoveTo)(path, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPath_LineTo)(path, x, y) }
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
        unsafe { (self.extern_FPDFPath_BezierTo)(path, x1, y1, x2, y2, x3, y3) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPath_Close)(path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_SetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPath_SetDrawMode)(path, fillmode, stroke) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPath_GetDrawMode)(path, fillmode, stroke) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) {
        unsafe { (self.extern_FPDFPage_InsertObject)(page, page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveObject(&self, page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPage_RemoveObject)(page, page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CountObjects(&self, page: FPDF_PAGE) -> c_int {
        unsafe { (self.extern_FPDFPage_CountObjects)(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetObject(&self, page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFPage_GetObject)(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_Destroy(&self, page_obj: FPDF_PAGEOBJECT) {
        unsafe { (self.extern_FPDFPageObj_Destroy)(page_obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_HasTransparency(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_HasTransparency)(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetType(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFPageObj_GetType)(page_object) }
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
        unsafe { (self.extern_FPDFPageObj_Transform)(page_object, a, b, c, d, e, f) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMatrix(
        &self,
        page_object: FPDF_PAGEOBJECT,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_GetMatrix)(page_object, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetMatrix(&self, path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_SetMatrix)(path, matrix) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_NewImageObj(&self, document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFPageObj_NewImageObj)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CountMarks(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFPageObj_CountMarks)(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        index: c_ulong,
    ) -> FPDF_PAGEOBJECTMARK {
        unsafe { (self.extern_FPDFPageObj_GetMark)(page_object, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_AddMark(&self, page_object: FPDF_PAGEOBJECT, name: &str) -> FPDF_PAGEOBJECTMARK {
        let c_name = CString::new(name).unwrap();

        unsafe { (self.extern_FPDFPageObj_AddMark)(page_object, c_name.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_RemoveMark(
        &self,
        page_object: FPDF_PAGEOBJECT,
        mark: FPDF_PAGEOBJECTMARK,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_RemoveMark)(page_object, mark) }
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
        unsafe { (self.extern_FPDFPageObjMark_GetName)(mark, buffer, buflen, out_buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_CountParams(&self, mark: FPDF_PAGEOBJECTMARK) -> c_int {
        unsafe { (self.extern_FPDFPageObjMark_CountParams)(mark) }
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
            (self.extern_FPDFPageObjMark_GetParamKey)(mark, index, buffer, buflen, out_buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamValueType(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFPageObjMark_GetParamValueType)(mark, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObjMark_GetParamIntValue(
        &self,
        mark: FPDF_PAGEOBJECTMARK,
        key: &str,
        out_value: *mut c_int,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFPageObjMark_GetParamIntValue)(mark, c_key.as_ptr(), out_value) }
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
        let c_key = CString::new(key).unwrap();

        unsafe {
            (self.extern_FPDFPageObjMark_GetParamStringValue)(
                mark,
                c_key.as_ptr(),
                buffer,
                buflen,
                out_buflen,
            )
        }
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
        let c_key = CString::new(key).unwrap();

        unsafe {
            (self.extern_FPDFPageObjMark_GetParamBlobValue)(
                mark,
                c_key.as_ptr(),
                buffer,
                buflen,
                out_buflen,
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
        key: &str,
        value: c_int,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe {
            (self.extern_FPDFPageObjMark_SetIntParam)(
                document,
                page_object,
                mark,
                c_key.as_ptr(),
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
        key: &str,
        value: &str,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        let c_value = CString::new(value).unwrap();

        unsafe {
            (self.extern_FPDFPageObjMark_SetStringParam)(
                document,
                page_object,
                mark,
                c_key.as_ptr(),
                c_value.as_ptr(),
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
        key: &str,
        value: *mut c_void,
        value_len: c_ulong,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe {
            (self.extern_FPDFPageObjMark_SetBlobParam)(
                document,
                page_object,
                mark,
                c_key.as_ptr(),
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
        key: &str,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFPageObjMark_RemoveParam)(page_object, mark, c_key.as_ptr()) }
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
        unsafe { (self.extern_FPDFImageObj_LoadJpegFile)(pages, count, image_object, file_access) }
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
            (self.extern_FPDFImageObj_LoadJpegFileInline)(pages, count, image_object, file_access)
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
        unsafe { (self.extern_FPDFImageObj_SetMatrix)(image_object, a, b, c, d, e, f) }
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
        unsafe { (self.extern_FPDFImageObj_SetBitmap)(pages, count, image_object, bitmap) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetBitmap(&self, image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP {
        unsafe { (self.extern_FPDFImageObj_GetBitmap)(image_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetRenderedBitmap(
        &self,
        document: FPDF_DOCUMENT,
        page: FPDF_PAGE,
        image_object: FPDF_PAGEOBJECT,
    ) -> FPDF_BITMAP {
        unsafe { (self.extern_FPDFImageObj_GetRenderedBitmap)(document, page, image_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataDecoded(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFImageObj_GetImageDataDecoded)(image_object, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageDataRaw(
        &self,
        image_object: FPDF_PAGEOBJECT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFImageObj_GetImageDataRaw)(image_object, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageFilterCount(&self, image_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFImageObj_GetImageFilterCount)(image_object) }
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
        unsafe { (self.extern_FPDFImageObj_GetImageFilter)(image_object, index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFImageObj_GetImageMetadata(
        &self,
        image_object: FPDF_PAGEOBJECT,
        page: FPDF_PAGE,
        metadata: *mut FPDF_IMAGEOBJ_METADATA,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFImageObj_GetImageMetadata)(image_object, page, metadata) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_CreateNewPath(&self, x: c_float, y: c_float) -> FPDF_PAGEOBJECT {
        unsafe { (self.extern_FPDFPageObj_CreateNewPath)(x, y) }
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
        unsafe { (self.extern_FPDFPageObj_CreateNewRect)(x, y, w, h) }
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
        unsafe { (self.extern_FPDFPageObj_GetBounds)(page_object, left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: &str) {
        let c_blend_mode = CString::new(blend_mode).unwrap();

        unsafe { (self.extern_FPDFPageObj_SetBlendMode)(page_object, c_blend_mode.as_ptr()) }
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
        unsafe { (self.extern_FPDFPageObj_SetStrokeColor)(page_object, R, G, B, A) }
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
        unsafe { (self.extern_FPDFPageObj_GetStrokeColor)(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: c_float,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_SetStrokeWidth)(page_object, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetStrokeWidth(
        &self,
        page_object: FPDF_PAGEOBJECT,
        width: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_GetStrokeWidth)(page_object, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineJoin(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFPageObj_GetLineJoin)(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineJoin(&self, page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_SetLineJoin)(page_object, line_join) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetLineCap(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFPageObj_GetLineCap)(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetLineCap(&self, page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_SetLineCap)(page_object, line_cap) }
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
        unsafe { (self.extern_FPDFPageObj_SetFillColor)(page_object, R, G, B, A) }
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
        unsafe { (self.extern_FPDFPageObj_GetFillColor)(page_object, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashPhase(
        &self,
        page_object: FPDF_PAGEOBJECT,
        phase: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_GetDashPhase)(page_object, phase) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_SetDashPhase(&self, page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_SetDashPhase)(page_object, phase) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashCount(&self, page_object: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFPageObj_GetDashCount)(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetDashArray(
        &self,
        page_object: FPDF_PAGEOBJECT,
        dash_array: *mut c_float,
        dash_count: size_t,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPageObj_GetDashArray)(page_object, dash_array, dash_count) }
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
            (self.extern_FPDFPageObj_SetDashArray)(page_object, dash_array, dash_count, phase)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_CountSegments(&self, path: FPDF_PAGEOBJECT) -> c_int {
        unsafe { (self.extern_FPDFPath_CountSegments)(path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetPathSegment(&self, path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT {
        unsafe { (self.extern_FPDFPath_GetPathSegment)(path, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetPoint(
        &self,
        segment: FPDF_PATHSEGMENT,
        x: *mut c_float,
        y: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPathSegment_GetPoint)(segment, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetType(&self, segment: FPDF_PATHSEGMENT) -> c_int {
        unsafe { (self.extern_FPDFPathSegment_GetType)(segment) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetClose(&self, segment: FPDF_PATHSEGMENT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFPathSegment_GetClose)(segment) }
    }

    // TODO: AJRC - 4-Aug-2024 - FPDFFont_GetBaseFontName() is in Pdfium export headers
    // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
    #[cfg(feature = "pdfium_future")]
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetBaseFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: usize,
    ) -> usize {
        unsafe { (self.extern_FPDFFont_GetBaseFontName)(font, buffer, length) }
    }

    // TODO: AJRC - 4-Aug-2024 - pointer type updated in FPDFFont_GetBaseFontName() definition,
    // but changes not yet released. Tracking issue: https://github.com/ajrcarey/pdfium-render/issues/152
    #[cfg(feature = "pdfium_future")]
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFamilyName(&self, font: FPDF_FONT, buffer: *mut c_char, length: usize) -> usize {
        unsafe { (self.extern_FPDFFont_GetFamilyName)(font, buffer, length) }
    }

    #[cfg(feature = "pdfium_6611")]
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFamilyName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFFont_GetFamilyName)(font, buffer, length) }
    }

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
    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontName(
        &self,
        font: FPDF_FONT,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFFont_GetFontName)(font, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFontData(
        &self,
        font: FPDF_FONT,
        buffer: *mut u8,
        buflen: usize,
        out_buflen: *mut usize,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFFont_GetFontData)(font, buffer, buflen, out_buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetIsEmbedded(&self, font: FPDF_FONT) -> c_int {
        unsafe { (self.extern_FPDFFont_GetIsEmbedded)(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> c_int {
        unsafe { (self.extern_FPDFFont_GetFlags)(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> c_int {
        unsafe { (self.extern_FPDFFont_GetWeight)(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetItalicAngle(&self, font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFFont_GetItalicAngle)(font, angle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetAscent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        ascent: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFFont_GetAscent)(font, font_size, ascent) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetDescent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        descent: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFFont_GetDescent)(font, font_size, descent) }
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
        unsafe { (self.extern_FPDFFont_GetGlyphWidth)(font, glyph, font_size, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphPath(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
    ) -> FPDF_GLYPHPATH {
        unsafe { (self.extern_FPDFFont_GetGlyphPath)(font, glyph, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: FPDF_GLYPHPATH) -> c_int {
        unsafe { (self.extern_FPDFGlyphPath_CountGlyphSegments)(glyphpath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: FPDF_GLYPHPATH,
        index: c_int,
    ) -> FPDF_PATHSEGMENT {
        unsafe { (self.extern_FPDFGlyphPath_GetGlyphPathSegment)(glyphpath, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDF_VIEWERREF_GetPrintScaling)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetNumCopies(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDF_VIEWERREF_GetNumCopies)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRange(&self, document: FPDF_DOCUMENT) -> FPDF_PAGERANGE {
        unsafe { (self.extern_FPDF_VIEWERREF_GetPrintPageRange)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> size_t {
        unsafe { (self.extern_FPDF_VIEWERREF_GetPrintPageRangeCount)(pagerange) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: FPDF_PAGERANGE,
        index: size_t,
    ) -> c_int {
        unsafe { (self.extern_FPDF_VIEWERREF_GetPrintPageRangeElement)(pagerange, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE {
        unsafe { (self.extern_FPDF_VIEWERREF_GetDuplex)(document) }
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
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDF_VIEWERREF_GetName)(document, c_key.as_ptr(), buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachmentCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { (self.extern_FPDFDoc_GetAttachmentCount)(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_AddAttachment(
        &self,
        document: FPDF_DOCUMENT,
        name: FPDF_WIDESTRING,
    ) -> FPDF_ATTACHMENT {
        unsafe { (self.extern_FPDFDoc_AddAttachment)(document, name) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT {
        unsafe { (self.extern_FPDFDoc_GetAttachment)(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_DeleteAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFDoc_DeleteAttachment)(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetName(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { (self.extern_FPDFAttachment_GetName)(attachment, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_HasKey(&self, attachment: FPDF_ATTACHMENT, key: &str) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAttachment_HasKey)(attachment, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetValueType(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAttachment_GetValueType)(attachment, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { (self.extern_FPDFAttachment_SetStringValue)(attachment, c_key.as_ptr(), value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetStringValue(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        let c_key = CString::new(key).unwrap();

        unsafe {
            (self.extern_FPDFAttachment_GetStringValue)(attachment, c_key.as_ptr(), buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_SetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        document: FPDF_DOCUMENT,
        contents: *const c_void,
        len: c_ulong,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAttachment_SetFile)(attachment, document, contents, len) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetFile(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
        out_buflen: *mut c_ulong,
    ) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFAttachment_GetFile)(attachment, buffer, buflen, out_buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFCatalog_IsTagged(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        unsafe { (self.extern_FPDFCatalog_IsTagged)(document) }
    }
}
