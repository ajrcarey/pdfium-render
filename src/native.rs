use crate::bindgen::{
    size_t, FPDFANNOT_COLORTYPE, FPDF_ACTION, FPDF_ANNOTATION, FPDF_ANNOTATION_SUBTYPE,
    FPDF_ANNOT_APPEARANCEMODE, FPDF_ATTACHMENT, FPDF_BITMAP, FPDF_BOOKMARK, FPDF_BOOL,
    FPDF_BYTESTRING, FPDF_CLIPPATH, FPDF_DEST, FPDF_DOCUMENT, FPDF_DUPLEXTYPE, FPDF_DWORD,
    FPDF_FILEACCESS, FPDF_FILEIDTYPE, FPDF_FILEWRITE, FPDF_FONT, FPDF_FORMFILLINFO,
    FPDF_FORMHANDLE, FPDF_GLYPHPATH, FPDF_IMAGEOBJ_METADATA, FPDF_LINK, FPDF_OBJECT_TYPE,
    FPDF_PAGE, FPDF_PAGELINK, FPDF_PAGEOBJECT, FPDF_PAGEOBJECTMARK, FPDF_PAGERANGE,
    FPDF_PATHSEGMENT, FPDF_SCHHANDLE, FPDF_SIGNATURE, FPDF_STRING, FPDF_STRUCTELEMENT,
    FPDF_STRUCTTREE, FPDF_TEXTPAGE, FPDF_TEXT_RENDERMODE, FPDF_WCHAR, FPDF_WIDESTRING, FS_FLOAT,
    FS_MATRIX, FS_POINTF, FS_QUADPOINTSF, FS_RECTF,
};
use crate::bindings::PdfiumLibraryBindings;
use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

pub(crate) struct DynamicPdfiumBindings {
    library: Library,
}

impl DynamicPdfiumBindings {
    pub fn new(library: Library) -> Result<Self, libloading::Error> {
        let result = DynamicPdfiumBindings { library };

        // Make sure the library correctly exports all the functions we expect.

        result.extern_FPDF_InitLibrary()?;
        result.extern_FPDF_DestroyLibrary()?;
        result.extern_FPDF_GetLastError()?;
        result.extern_FPDF_CreateNewDocument()?;
        result.extern_FPDF_LoadDocument()?;
        result.extern_FPDF_LoadMemDocument64()?;
        result.extern_FPDF_LoadCustomDocument()?;
        result.extern_FPDF_SaveAsCopy()?;
        result.extern_FPDF_SaveWithVersion()?;
        result.extern_FPDF_CloseDocument()?;
        result.extern_FPDF_DeviceToPage()?;
        result.extern_FPDF_PageToDevice()?;
        result.extern_FPDF_GetFileVersion()?;
        result.extern_FPDF_GetFileIdentifier()?;
        result.extern_FPDF_GetFormType()?;
        result.extern_FPDF_GetMetaText()?;
        result.extern_FPDF_GetDocPermissions()?;
        result.extern_FPDF_GetSecurityHandlerRevision()?;
        result.extern_FPDF_GetPageCount()?;
        result.extern_FPDF_LoadPage()?;
        result.extern_FPDF_ClosePage()?;
        result.extern_FPDF_ImportPagesByIndex()?;
        result.extern_FPDF_ImportPages()?;
        result.extern_FPDF_ImportNPagesToOne()?;
        result.extern_FPDF_GetPageLabel()?;
        result.extern_FPDF_GetPageBoundingBox()?;
        result.extern_FPDF_GetPageWidthF()?;
        result.extern_FPDF_GetPageHeightF()?;
        result.extern_FPDFText_GetCharIndexFromTextIndex()?;
        result.extern_FPDFText_GetTextIndexFromCharIndex()?;
        result.extern_FPDF_GetSignatureCount()?;
        result.extern_FPDF_GetSignatureObject()?;
        result.extern_FPDFSignatureObj_GetContents()?;
        result.extern_FPDFSignatureObj_GetByteRange()?;
        result.extern_FPDFSignatureObj_GetSubFilter()?;
        result.extern_FPDFSignatureObj_GetReason()?;
        result.extern_FPDFSignatureObj_GetTime()?;
        result.extern_FPDFSignatureObj_GetDocMDPPermission()?;
        result.extern_FPDF_StructTree_GetForPage()?;
        result.extern_FPDF_StructTree_Close()?;
        result.extern_FPDF_StructTree_CountChildren()?;
        result.extern_FPDF_StructTree_GetChildAtIndex()?;
        result.extern_FPDF_StructElement_GetAltText()?;
        result.extern_FPDF_StructElement_GetID()?;
        result.extern_FPDF_StructElement_GetLang()?;
        result.extern_FPDF_StructElement_GetStringAttribute()?;
        result.extern_FPDF_StructElement_GetMarkedContentID()?;
        result.extern_FPDF_StructElement_GetType()?;
        result.extern_FPDF_StructElement_GetTitle()?;
        result.extern_FPDF_StructElement_CountChildren()?;
        result.extern_FPDF_StructElement_GetChildAtIndex()?;
        result.extern_FPDFPage_New()?;
        result.extern_FPDFPage_Delete()?;
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
        result.extern_FPDFPage_TransFormWithClip()?;
        result.extern_FPDFPageObj_TransformClipPath()?;
        result.extern_FPDFPageObj_GetClipPath()?;
        result.extern_FPDFClipPath_CountPaths()?;
        result.extern_FPDFClipPath_CountPathSegments()?;
        result.extern_FPDFClipPath_GetPathSegment()?;
        result.extern_FPDF_CreateClipPath()?;
        result.extern_FPDF_DestroyClipPath()?;
        result.extern_FPDFPage_InsertClipPath()?;
        result.extern_FPDFPage_HasTransparency()?;
        result.extern_FPDFPage_GenerateContent()?;
        result.extern_FPDFBitmap_CreateEx()?;
        result.extern_FPDFBitmap_Destroy()?;
        result.extern_FPDFBitmap_GetFormat()?;
        result.extern_FPDFBitmap_FillRect()?;
        result.extern_FPDFBitmap_GetBuffer()?;
        result.extern_FPDFBitmap_GetWidth()?;
        result.extern_FPDFBitmap_GetHeight()?;
        result.extern_FPDFBitmap_GetStride()?;
        result.extern_FPDF_RenderPageBitmap()?;
        result.extern_FPDF_RenderPageBitmapWithMatrix()?;
        result.extern_FPDFAnnot_IsSupportedSubtype()?;
        result.extern_FPDFPage_CreateAnnot()?;
        result.extern_FPDFPage_GetAnnotCount()?;
        result.extern_FPDFPage_GetAnnot()?;
        result.extern_FPDFPage_GetAnnotIndex()?;
        result.extern_FPDFPage_CloseAnnot()?;
        result.extern_FPDFPage_RemoveAnnot()?;
        result.extern_FPDFAnnot_GetSubtype()?;
        result.extern_FPDFAnnot_IsObjectSupportedSubtype()?;
        result.extern_FPDFAnnot_UpdateObject()?;
        result.extern_FPDFAnnot_AddInkStroke()?;
        result.extern_FPDFAnnot_RemoveInkList()?;
        result.extern_FPDFAnnot_AppendObject()?;
        result.extern_FPDFAnnot_GetObjectCount()?;
        result.extern_FPDFAnnot_GetObject()?;
        result.extern_FPDFAnnot_RemoveObject()?;
        result.extern_FPDFAnnot_SetColor()?;
        result.extern_FPDFAnnot_HasAttachmentPoints()?;
        result.extern_FPDFAnnot_SetAttachmentPoints()?;
        result.extern_FPDFAnnot_AppendAttachmentPoints()?;
        result.extern_FPDFAnnot_CountAttachmentPoints()?;
        result.extern_FPDFAnnot_GetAttachmentPoints()?;
        result.extern_FPDFAnnot_SetRect()?;
        result.extern_FPDFAnnot_GetRect()?;
        result.extern_FPDFAnnot_GetVertices()?;
        result.extern_FPDFAnnot_GetInkListCount()?;
        result.extern_FPDFAnnot_GetInkListPath()?;
        result.extern_FPDFAnnot_GetLine()?;
        result.extern_FPDFAnnot_SetBorder()?;
        result.extern_FPDFAnnot_GetBorder()?;
        result.extern_FPDFAnnot_HasKey()?;
        result.extern_FPDFAnnot_GetValueType()?;
        result.extern_FPDFAnnot_SetStringValue()?;
        result.extern_FPDFAnnot_GetStringValue()?;
        result.extern_FPDFAnnot_GetNumberValue()?;
        result.extern_FPDFAnnot_SetAP()?;
        result.extern_FPDFAnnot_GetAP()?;
        result.extern_FPDFAnnot_GetLinkedAnnot()?;
        result.extern_FPDFAnnot_GetFlags()?;
        result.extern_FPDFAnnot_SetFlags()?;
        result.extern_FPDFAnnot_GetFormFieldFlags()?;
        result.extern_FPDFAnnot_GetFormFieldAtPoint()?;
        result.extern_FPDFAnnot_GetFormFieldName()?;
        result.extern_FPDFAnnot_GetFormFieldType()?;
        result.extern_FPDFAnnot_GetFormFieldValue()?;
        result.extern_FPDFAnnot_GetOptionCount()?;
        result.extern_FPDFAnnot_GetOptionLabel()?;
        result.extern_FPDFAnnot_IsOptionSelected()?;
        result.extern_FPDFAnnot_GetFontSize()?;
        result.extern_FPDFAnnot_IsChecked()?;
        result.extern_FPDFAnnot_SetFocusableSubtypes()?;
        result.extern_FPDFAnnot_GetFocusableSubtypesCount()?;
        result.extern_FPDFAnnot_GetFocusableSubtypes()?;
        result.extern_FPDFAnnot_GetLink()?;
        result.extern_FPDFAnnot_GetFormControlCount()?;
        result.extern_FPDFAnnot_GetFormControlIndex()?;
        result.extern_FPDFAnnot_GetFormFieldExportValue()?;
        result.extern_FPDFAnnot_SetURI()?;
        result.extern_FPDFDOC_InitFormFillEnvironment()?;
        result.extern_FPDFDOC_ExitFormFillEnvironment()?;
        result.extern_FORM_OnAfterLoadPage()?;
        result.extern_FORM_OnBeforeClosePage()?;
        result.extern_FPDFDoc_GetPageMode()?;
        result.extern_FPDFPage_Flatten()?;
        result.extern_FPDF_SetFormFieldHighlightColor()?;
        result.extern_FPDF_SetFormFieldHighlightAlpha()?;
        result.extern_FPDF_FFLDraw()?;
        result.extern_FPDFBookmark_GetFirstChild()?;
        result.extern_FPDFBookmark_GetNextSibling()?;
        result.extern_FPDFBookmark_GetTitle()?;
        result.extern_FPDFBookmark_GetCount()?;
        result.extern_FPDFBookmark_Find()?;
        result.extern_FPDFBookmark_GetDest()?;
        result.extern_FPDFBookmark_GetAction()?;
        result.extern_FPDFAction_GetType()?;
        result.extern_FPDFAction_GetDest()?;
        result.extern_FPDFAction_GetFilePath()?;
        result.extern_FPDFAction_GetURIPath()?;
        result.extern_FPDFDest_GetDestPageIndex()?;
        result.extern_FPDFDest_GetView()?;
        result.extern_FPDFDest_GetLocationInPage()?;
        result.extern_FPDFLink_GetLinkAtPoint()?;
        result.extern_FPDFLink_GetLinkZOrderAtPoint()?;
        result.extern_FPDFLink_GetDest()?;
        result.extern_FPDFLink_GetAction()?;
        result.extern_FPDFLink_Enumerate()?;
        result.extern_FPDFLink_GetAnnot()?;
        result.extern_FPDFLink_GetAnnotRect()?;
        result.extern_FPDFLink_CountQuadPoints()?;
        result.extern_FPDFLink_GetQuadPoints()?;
        result.extern_FPDF_GetPageAAction()?;
        result.extern_FPDFText_LoadPage()?;
        result.extern_FPDFText_ClosePage()?;
        result.extern_FPDFText_CountChars()?;
        result.extern_FPDFText_GetUnicode()?;
        result.extern_FPDFText_GetFontSize()?;
        result.extern_FPDFText_GetFontInfo()?;
        result.extern_FPDFText_GetFontWeight()?;
        result.extern_FPDFText_GetTextRenderMode()?;
        result.extern_FPDFText_GetFillColor()?;
        result.extern_FPDFText_GetStrokeColor()?;
        result.extern_FPDFText_GetCharAngle()?;
        result.extern_FPDFText_GetCharBox()?;
        result.extern_FPDFText_GetLooseCharBox()?;
        result.extern_FPDFText_GetMatrix()?;
        result.extern_FPDFText_GetCharOrigin()?;
        result.extern_FPDFText_GetCharIndexAtPos()?;
        result.extern_FPDFText_GetText()?;
        result.extern_FPDFText_CountRects()?;
        result.extern_FPDFText_GetRect()?;
        result.extern_FPDFText_GetBoundedText()?;
        result.extern_FPDFText_FindStart()?;
        result.extern_FPDFText_FindNext()?;
        result.extern_FPDFText_FindPrev()?;
        result.extern_FPDFText_GetSchResultIndex()?;
        result.extern_FPDFText_GetSchCount()?;
        result.extern_FPDFText_FindClose()?;
        result.extern_FPDFLink_LoadWebLinks()?;
        result.extern_FPDFLink_CountWebLinks()?;
        result.extern_FPDFLink_GetURL()?;
        result.extern_FPDFLink_CountRects()?;
        result.extern_FPDFLink_GetRect()?;
        result.extern_FPDFLink_GetTextRange()?;
        result.extern_FPDFLink_CloseWebLinks()?;
        result.extern_FPDFPage_GetDecodedThumbnailData()?;
        result.extern_FPDFPage_GetRawThumbnailData()?;
        result.extern_FPDFPage_GetThumbnailAsBitmap()?;
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
        result.extern_FPDFText_LoadFont()?;
        result.extern_FPDFText_LoadStandardFont()?;
        result.extern_FPDFFont_Close()?;
        result.extern_FPDFPath_MoveTo()?;
        result.extern_FPDFPath_LineTo()?;
        result.extern_FPDFPath_BezierTo()?;
        result.extern_FPDFPath_Close()?;
        result.extern_FPDFPath_SetDrawMode()?;
        result.extern_FPDFPath_GetDrawMode()?;
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
        result.extern_FPDFPath_CountSegments()?;
        result.extern_FPDFPath_GetPathSegment()?;
        result.extern_FPDFPathSegment_GetPoint()?;
        result.extern_FPDFPathSegment_GetType()?;
        result.extern_FPDFPathSegment_GetClose()?;
        result.extern_FPDFFont_GetFontName()?;
        result.extern_FPDFFont_GetFlags()?;
        result.extern_FPDFFont_GetWeight()?;
        result.extern_FPDFFont_GetItalicAngle()?;
        result.extern_FPDFFont_GetAscent()?;
        result.extern_FPDFFont_GetDescent()?;
        result.extern_FPDFFont_GetGlyphWidth()?;
        result.extern_FPDFFont_GetGlyphPath()?;
        result.extern_FPDFGlyphPath_CountGlyphSegments()?;
        result.extern_FPDFGlyphPath_GetGlyphPathSegment()?;
        result.extern_FPDF_VIEWERREF_GetPrintScaling()?;
        result.extern_FPDF_VIEWERREF_GetNumCopies()?;
        result.extern_FPDF_VIEWERREF_GetPrintPageRange()?;
        result.extern_FPDF_VIEWERREF_GetPrintPageRangeCount()?;
        result.extern_FPDF_VIEWERREF_GetPrintPageRangeElement()?;
        result.extern_FPDF_VIEWERREF_GetDuplex()?;
        result.extern_FPDF_VIEWERREF_GetName()?;
        result.extern_FPDFDoc_GetAttachmentCount()?;
        result.extern_FPDFDoc_AddAttachment()?;
        result.extern_FPDFDoc_GetAttachment()?;
        result.extern_FPDFDoc_DeleteAttachment()?;
        result.extern_FPDFAttachment_GetName()?;
        result.extern_FPDFAttachment_HasKey()?;
        result.extern_FPDFAttachment_GetValueType()?;
        result.extern_FPDFAttachment_SetStringValue()?;
        result.extern_FPDFAttachment_GetStringValue()?;
        result.extern_FPDFAttachment_SetFile()?;
        result.extern_FPDFAttachment_GetFile()?;
        result.extern_FPDFCatalog_IsTagged()?;

        Ok(result)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_InitLibrary(&self) -> Result<Symbol<unsafe extern "C" fn()>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_InitLibrary\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_DestroyLibrary(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn()>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_DestroyLibrary\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetLastError(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn() -> c_ulong>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetLastError\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_CreateNewDocument(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn() -> FPDF_DOCUMENT>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_CreateNewDocument\0") }
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
        unsafe { self.library.get(b"FPDF_LoadDocument\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadMemDocument64(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                data_buf: *const c_void,
                size: c_ulong,
                password: FPDF_BYTESTRING,
            ) -> FPDF_DOCUMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_LoadMemDocument64\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadCustomDocument(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                pFileAccess: *mut FPDF_FILEACCESS,
                password: FPDF_BYTESTRING,
            ) -> FPDF_DOCUMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_LoadCustomDocument\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_SaveAsCopy(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                pFileWrite: *mut FPDF_FILEWRITE,
                flags: FPDF_DWORD,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_SaveAsCopy\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_SaveWithVersion(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                pFileWrite: *mut FPDF_FILEWRITE,
                flags: FPDF_DWORD,
                fileVersion: c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_SaveWithVersion\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_CloseDocument(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT)>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_CloseDocument\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_DeviceToPage(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
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
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_DeviceToPage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_PageToDevice(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
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
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_PageToDevice\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetFileVersion(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetFileVersion\0") }
    }

    #[allow(non_snake_case)]
    fn extern_FPDF_GetFileIdentifier(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                id_type: FPDF_FILEIDTYPE,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetFileIdentifier\0") }
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
        unsafe { self.library.get(b"FPDF_GetMetaText\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetDocPermissions(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_ulong>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetDocPermissions\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetSecurityHandlerRevision(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetSecurityHandlerRevision\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetPageCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_LoadPage(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int) -> FPDF_PAGE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_LoadPage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_ClosePage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_ClosePage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_ImportPagesByIndex(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                dest_doc: FPDF_DOCUMENT,
                src_doc: FPDF_DOCUMENT,
                page_indices: *const c_int,
                length: c_ulong,
                index: c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_ImportPagesByIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_ImportPages(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                dest_doc: FPDF_DOCUMENT,
                src_doc: FPDF_DOCUMENT,
                pagerange: FPDF_BYTESTRING,
                index: c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_ImportPages\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_ImportNPagesToOne(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                src_doc: FPDF_DOCUMENT,
                output_width: c_float,
                output_height: c_float,
                num_pages_on_x_axis: size_t,
                num_pages_on_y_axis: size_t,
            ) -> FPDF_DOCUMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_ImportNPagesToOne\0") }
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
        unsafe { self.library.get(b"FPDF_GetPageLabel\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageBoundingBox(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, rect: *mut FS_RECTF) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetPageBoundingBox\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageWidthF(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_float>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetPageWidthF\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageHeightF(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_float>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_GetPageHeightF\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetCharIndexFromTextIndex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, nTextIndex: c_int) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetCharIndexFromTextIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetTextIndexFromCharIndex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, nCharIndex: c_int) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetTextIndexFromCharIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetSignatureCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetSignatureCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetSignatureObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetSignatureObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFSignatureObj_GetContents(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                signature: FPDF_SIGNATURE,
                buffer: *mut c_void,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFSignatureObj_GetContents\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFSignatureObj_GetByteRange(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                signature: FPDF_SIGNATURE,
                buffer: *mut c_int,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFSignatureObj_GetByteRange\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFSignatureObj_GetSubFilter(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                signature: FPDF_SIGNATURE,
                buffer: *mut c_char,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFSignatureObj_GetSubFilter\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFSignatureObj_GetReason(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                signature: FPDF_SIGNATURE,
                buffer: *mut c_void,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFSignatureObj_GetReason\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFSignatureObj_GetTime(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                signature: FPDF_SIGNATURE,
                buffer: *mut c_char,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFSignatureObj_GetTime\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFSignatureObj_GetDocMDPPermission(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(signature: FPDF_SIGNATURE) -> c_uint>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFSignatureObj_GetDocMDPPermission\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructTree_GetForPage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_STRUCTTREE>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_StructTree_GetForPage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructTree_Close(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_StructTree_Close\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructTree_CountChildren(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructTree_CountChildren\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructTree_GetChildAtIndex(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(struct_tree: FPDF_STRUCTTREE, index: c_int) -> FPDF_STRUCTELEMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructTree_GetChildAtIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetAltText(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetAltText\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetID(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetID\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetLang(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetLang\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetStringAttribute(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                attr_name: FPDF_BYTESTRING,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetStringAttribute\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetMarkedContentID(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetMarkedContentID\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetType(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetType\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetTitle(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                buffer: *mut c_void,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetTitle\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_CountChildren(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(struct_element: FPDF_STRUCTELEMENT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_CountChildren\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_StructElement_GetChildAtIndex(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                struct_element: FPDF_STRUCTELEMENT,
                index: c_int,
            ) -> FPDF_STRUCTELEMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_StructElement_GetChildAtIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_New(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                page_index: c_int,
                width: c_double,
                height: c_double,
            ) -> FPDF_PAGE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_New\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_Delete(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, page_index: c_int)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_Delete\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetRotation(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_GetRotation\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_SetRotation(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE, rotate: c_int)>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPage_SetRotation\0") }
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
        unsafe { self.library.get(b"FPDFPage_GetMediaBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_GetCropBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_GetBleedBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_GetTrimBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_GetArtBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_SetMediaBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_SetCropBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_SetBleedBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_SetTrimBox\0") }
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
        unsafe { self.library.get(b"FPDFPage_SetArtBox\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_TransFormWithClip(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                matrix: *const FS_MATRIX,
                clipRect: *const FS_RECTF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_TransFormWithClip\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_TransformClipPath(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page_object: FPDF_PAGEOBJECT,
                a: f64,
                b: f64,
                c: f64,
                d: f64,
                e: f64,
                f: f64,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_TransformClipPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetClipPath(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetClipPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFClipPath_CountPaths(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(clip_path: FPDF_CLIPPATH) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFClipPath_CountPaths\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFClipPath_CountPathSegments(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFClipPath_CountPathSegments\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFClipPath_GetPathSegment(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                clip_path: FPDF_CLIPPATH,
                path_index: c_int,
                segment_index: c_int,
            ) -> FPDF_PATHSEGMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFClipPath_GetPathSegment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_CreateClipPath(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_CreateClipPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_DestroyClipPath(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(clipPath: FPDF_CLIPPATH)>, libloading::Error> {
        unsafe { self.library.get(b"FPDF_DestroyClipPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_InsertClipPath(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, clipPath: FPDF_CLIPPATH)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_InsertClipPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_HasTransparency(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_HasTransparency\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GenerateContent(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BOOL>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_GenerateContent\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_TransformAnnots(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
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
        unsafe { self.library.get(b"FPDFPage_TransformAnnots\0") }
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
        unsafe { self.library.get(b"FPDFBitmap_CreateEx\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_Destroy(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_Destroy\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetFormat(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetFormat\0") }
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
        unsafe { self.library.get(b"FPDFBitmap_FillRect\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetBuffer(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> *mut c_void>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFBitmap_GetBuffer\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetWidth(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetWidth\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetHeight(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetHeight\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBitmap_GetStride(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bitmap: FPDF_BITMAP) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFBitmap_GetStride\0") }
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
        unsafe { self.library.get(b"FPDF_RenderPageBitmap\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDF_RenderPageBitmapWithMatrix(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                bitmap: FPDF_BITMAP,
                page: FPDF_PAGE,
                matrix: *const FS_MATRIX,
                clipping: *const FS_RECTF,
                flags: c_int,
            ),
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_RenderPageBitmapWithMatrix\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_IsSupportedSubtype(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_IsSupportedSubtype\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_CreateAnnot(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                subtype: FPDF_ANNOTATION_SUBTYPE,
            ) -> FPDF_ANNOTATION,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_CreateAnnot\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetAnnotCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_GetAnnotCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetAnnot(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetAnnot\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetAnnotIndex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetAnnotIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_CloseAnnot(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_CloseAnnot\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_RemoveAnnot(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_RemoveAnnot\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetSubtype(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetSubtype\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_IsObjectSupportedSubtype(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_IsObjectSupportedSubtype\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_UpdateObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_UpdateObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_AddInkStroke(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                points: *const FS_POINTF,
                point_count: size_t,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_AddInkStroke\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_RemoveInkList(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_RemoveInkList\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_AppendObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_AppendObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetObjectCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_GetObjectCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_RemoveObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_RemoveObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDFAnnot_SetColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                color_type: FPDFANNOT_COLORTYPE,
                R: c_uint,
                G: c_uint,
                B: c_uint,
                A: c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetColor\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn extern_FPDFAnnot_GetColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                color_type: FPDFANNOT_COLORTYPE,
                R: *mut c_uint,
                G: *mut c_uint,
                B: *mut c_uint,
                A: *mut c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetColor\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_HasAttachmentPoints(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_HasAttachmentPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetAttachmentPoints(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                quad_index: size_t,
                quad_points: *const FS_QUADPOINTSF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetAttachmentPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_AppendAttachmentPoints(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                quad_points: *const FS_QUADPOINTSF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_AppendAttachmentPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_CountAttachmentPoints(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> size_t>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_CountAttachmentPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetAttachmentPoints(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                quad_index: size_t,
                quad_points: *mut FS_QUADPOINTSF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetAttachmentPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetRect(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetRect\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetRect(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetRect\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetVertices(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                buffer: *mut FS_POINTF,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetVertices\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetInkListCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_ulong>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_GetInkListCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetInkListPath(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                path_index: c_ulong,
                buffer: *mut FS_POINTF,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetInkListPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetLine(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                start: *mut FS_POINTF,
                end: *mut FS_POINTF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetLine\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetBorder(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                horizontal_radius: f32,
                vertical_radius: f32,
                border_width: f32,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetBorder\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetBorder(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                horizontal_radius: *mut f32,
                vertical_radius: *mut f32,
                border_width: *mut f32,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetBorder\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_HasKey(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_HasKey\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetValueType(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_OBJECT_TYPE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetValueType\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetStringValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                key: FPDF_BYTESTRING,
                value: FPDF_WIDESTRING,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetStringValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetStringValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                key: FPDF_BYTESTRING,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetStringValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetNumberValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                key: FPDF_BYTESTRING,
                value: *mut f32,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetNumberValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetAP(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
                value: FPDF_WIDESTRING,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetAP\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetAP(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                annot: FPDF_ANNOTATION,
                appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetAP\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetLinkedAnnot(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(annot: FPDF_ANNOTATION, key: FPDF_BYTESTRING) -> FPDF_ANNOTATION,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetLinkedAnnot\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFlags(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_GetFlags\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetFlags(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetFlags\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormFieldFlags(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormFieldFlags\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormFieldAtPoint(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                page: FPDF_PAGE,
                point: *const FS_POINTF,
            ) -> FPDF_ANNOTATION,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormFieldAtPoint\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormFieldName(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                annot: FPDF_ANNOTATION,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormFieldName\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormFieldType(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormFieldType\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormFieldValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                annot: FPDF_ANNOTATION,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormFieldValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetOptionCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetOptionCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetOptionLabel(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                annot: FPDF_ANNOTATION,
                index: c_int,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetOptionLabel\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_IsOptionSelected(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                handle: FPDF_FORMHANDLE,
                annot: FPDF_ANNOTATION,
                index: c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_IsOptionSelected\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFontSize(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                annot: FPDF_ANNOTATION,
                value: *mut f32,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFontSize\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_IsChecked(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_IsChecked\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetFocusableSubtypes(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                subtypes: *const FPDF_ANNOTATION_SUBTYPE,
                count: size_t,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetFocusableSubtypes\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFocusableSubtypesCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_GetFocusableSubtypesCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFocusableSubtypes(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
                count: size_t,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFocusableSubtypes\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetLink(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION) -> FPDF_LINK>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAnnot_GetLink\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormControlCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormControlCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormControlIndex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormControlIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_GetFormFieldExportValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                hHandle: FPDF_FORMHANDLE,
                annot: FPDF_ANNOTATION,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_GetFormFieldExportValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAnnot_SetURI(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(annot: FPDF_ANNOTATION, uri: *const c_char) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAnnot_SetURI\0") }
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
        unsafe { self.library.get(b"FPDFDOC_InitFormFillEnvironment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDOC_ExitFormFillEnvironment(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFDOC_ExitFormFillEnvironment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FORM_OnAfterLoadPage(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, handle: FPDF_FORMHANDLE)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FORM_OnAfterLoadPage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FORM_OnBeforeClosePage(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, handle: FPDF_FORMHANDLE)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FORM_OnBeforeClosePage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDoc_GetPageMode(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFDoc_GetPageMode\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_Flatten(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, nFlag: c_int) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_Flatten\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_SetFormFieldHighlightColor(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE, field_type: c_int, color: c_ulong)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_SetFormFieldHighlightColor\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_SetFormFieldHighlightAlpha(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(handle: FPDF_FORMHANDLE, alpha: c_uchar)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_SetFormFieldHighlightAlpha\0") }
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
        unsafe { self.library.get(b"FPDF_FFLDraw\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetFormType(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_GetFormType\0") }
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
        unsafe { self.library.get(b"FPDFBookmark_GetFirstChild\0") }
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
        unsafe { self.library.get(b"FPDFBookmark_GetNextSibling\0") }
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
        unsafe { self.library.get(b"FPDFBookmark_GetTitle\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFBookmark_GetCount\0") }
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
        unsafe { self.library.get(b"FPDFBookmark_Find\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetDest(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetDest\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFBookmark_GetAction(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(bookmark: FPDF_BOOKMARK) -> FPDF_ACTION>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFBookmark_GetAction\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAction_GetType(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(action: FPDF_ACTION) -> c_ulong>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFAction_GetType\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAction_GetDest(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, action: FPDF_ACTION) -> FPDF_DEST>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAction_GetDest\0") }
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
        unsafe { self.library.get(b"FPDFAction_GetFilePath\0") }
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
        unsafe { self.library.get(b"FPDFAction_GetURIPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDest_GetDestPageIndex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDest_GetDestPageIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDest_GetView(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                dest: FPDF_DEST,
                pNumParams: *mut c_ulong,
                pParams: *mut FS_FLOAT,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDest_GetView\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn extern_FPDFDest_GetLocationInPage(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                dest: FPDF_DEST,
                hasXVal: *mut FPDF_BOOL,
                hasYVal: *mut FPDF_BOOL,
                hasZoomVal: *mut FPDF_BOOL,
                x: *mut FS_FLOAT,
                y: *mut FS_FLOAT,
                zoom: *mut FS_FLOAT,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDest_GetLocationInPage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetLinkAtPoint(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, x: c_double, y: c_double) -> FPDF_LINK>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetLinkAtPoint\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetLinkZOrderAtPoint(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, x: c_double, y: c_double) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetLinkZOrderAtPoint\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetDest(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetDest\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetAction(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(link: FPDF_LINK) -> FPDF_ACTION>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFLink_GetAction\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_Enumerate(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                page: FPDF_PAGE,
                start_pos: *mut c_int,
                link_annot: *mut FPDF_LINK,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_Enumerate\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetAnnot(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetAnnot\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetAnnotRect(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetAnnotRect\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_CountQuadPoints(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(link_annot: FPDF_LINK) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFLink_CountQuadPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetQuadPoints(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                link_annot: FPDF_LINK,
                quad_index: c_int,
                quad_points: *mut FS_QUADPOINTSF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetQuadPoints\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_GetPageAAction(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_GetPageAAction\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_LoadPage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_TEXTPAGE>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_LoadPage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_ClosePage(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFText_ClosePage\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_CountChars(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_CountChars\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetUnicode(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetUnicode\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetFontSize(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_double>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetFontSize\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetFontInfo(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                buffer: *mut c_void,
                buflen: c_ulong,
                flags: *mut c_int,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetFontInfo\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetFontWeight(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetFontWeight\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetTextRenderMode(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> FPDF_TEXT_RENDERMODE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetTextRenderMode\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetFillColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                R: *mut c_uint,
                G: *mut c_uint,
                B: *mut c_uint,
                A: *mut c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetFillColor\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetStrokeColor(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                R: *mut c_uint,
                G: *mut c_uint,
                B: *mut c_uint,
                A: *mut c_uint,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetStrokeColor\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetCharAngle(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE, index: c_int) -> c_float>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetCharAngle\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetCharBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                left: *mut c_double,
                right: *mut c_double,
                bottom: *mut c_double,
                top: *mut c_double,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetCharBox\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetLooseCharBox(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                rect: *mut FS_RECTF,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetLooseCharBox\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetMatrix(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                matrix: *mut FS_MATRIX,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetMatrix\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetCharOrigin(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                index: c_int,
                x: *mut c_double,
                y: *mut c_double,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetCharOrigin\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetCharIndexAtPos(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                x: c_double,
                y: c_double,
                xTolerance: c_double,
                yTolerance: c_double,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetCharIndexAtPos\0") }
    }

    #[allow(non_snake_case)]
    fn extern_FPDFText_GetText(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                start_index: c_int,
                count: c_int,
                result: *mut c_ushort,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetText\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_CountRects(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                start_index: c_int,
                count: c_int,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_CountRects\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetRect(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                rect_index: c_int,
                left: *mut c_double,
                top: *mut c_double,
                right: *mut c_double,
                bottom: *mut c_double,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetRect\0") }
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
                left: c_double,
                top: c_double,
                right: c_double,
                bottom: c_double,
                buffer: *mut c_ushort,
                buflen: c_int,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_GetBoundedText\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_FindStart(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                text_page: FPDF_TEXTPAGE,
                findwhat: FPDF_WIDESTRING,
                flags: c_ulong,
                start_index: c_int,
            ) -> FPDF_SCHHANDLE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_FindStart\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_FindNext(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_FindNext\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_FindPrev(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_FindPrev\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetSchResultIndex(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_GetSchResultIndex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_GetSchCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_SCHHANDLE) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFText_GetSchCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_FindClose(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(handle: FPDF_SCHHANDLE)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFText_FindClose\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_LoadWebLinks(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_LoadWebLinks\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_CountWebLinks(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(link_page: FPDF_PAGELINK) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFLink_CountWebLinks\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetURL(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                link_page: FPDF_PAGELINK,
                link_index: c_int,
                buffer: *mut c_ushort,
                buflen: c_int,
            ) -> c_int,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetURL\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_CountRects(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(link_page: FPDF_PAGELINK, link_index: c_int) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_CountRects\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn extern_FPDFLink_GetRect(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                link_page: FPDF_PAGELINK,
                link_index: c_int,
                rect_index: c_int,
                left: *mut c_double,
                top: *mut c_double,
                right: *mut c_double,
                bottom: *mut c_double,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetRect\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_GetTextRange(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                link_page: FPDF_PAGELINK,
                link_index: c_int,
                start_char_index: *mut c_int,
                char_count: *mut c_int,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFLink_GetTextRange\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFLink_CloseWebLinks(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(link_page: FPDF_PAGELINK)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFLink_CloseWebLinks\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetDecodedThumbnailData(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetDecodedThumbnailData\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetRawThumbnailData(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(page: FPDF_PAGE, buffer: *mut c_void, buflen: c_ulong) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetRawThumbnailData\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetThumbnailAsBitmap(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> FPDF_BITMAP>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPage_GetThumbnailAsBitmap\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFormObj_CountObjects(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(form_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFormObj_CountObjects\0") }
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
        unsafe { self.library.get(b"FPDFFormObj_GetObject\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_CreateTextObj\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetTextRenderMode(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_TEXT_RENDERMODE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFTextObj_GetTextRenderMode\0") }
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
        unsafe { self.library.get(b"FPDFTextObj_SetTextRenderMode\0") }
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
        unsafe { self.library.get(b"FPDFTextObj_GetText\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetFont(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(text: FPDF_PAGEOBJECT) -> FPDF_FONT>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFTextObj_GetFont\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFTextObj_GetFontSize(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(text: FPDF_PAGEOBJECT, size: *mut c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFTextObj_GetFontSize\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_NewTextObj\0") }
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
        unsafe { self.library.get(b"FPDFText_SetText\0") }
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
        unsafe { self.library.get(b"FPDFText_SetCharcodes\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_LoadFont(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                data: *const c_uchar,
                size: c_uint,
                font_type: c_int,
                cid: FPDF_BOOL,
            ) -> FPDF_FONT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_LoadFont\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFText_LoadStandardFont(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, font: FPDF_BYTESTRING) -> FPDF_FONT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFText_LoadStandardFont\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_Close(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(font: FPDF_FONT)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFFont_Close\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_MoveTo(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPath_MoveTo\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_LineTo(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPath_LineTo\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_BezierTo(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                path: FPDF_PAGEOBJECT,
                x1: c_float,
                y1: c_float,
                x2: c_float,
                y2: c_float,
                x3: c_float,
                y3: c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPath_BezierTo\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_Close(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPath_Close\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_SetDrawMode(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                path: FPDF_PAGEOBJECT,
                fillmode: c_int,
                stroke: FPDF_BOOL,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPath_SetDrawMode\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_GetDrawMode(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                path: FPDF_PAGEOBJECT,
                fillmode: *mut c_int,
                stroke: *mut FPDF_BOOL,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPath_GetDrawMode\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_InsertObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_InsertObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_RemoveObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, page_obj: FPDF_PAGEOBJECT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_RemoveObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_CountObjects(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page: FPDF_PAGE) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPage_CountObjects\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPage_GetObject(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page: FPDF_PAGE, index: c_int) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPage_GetObject\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_Destroy(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(page_obj: FPDF_PAGEOBJECT)>, libloading::Error> {
        unsafe { self.library.get(b"FPDFPageObj_Destroy\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_HasTransparency(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_HasTransparency\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetType(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetType\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_Transform\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetMatrix\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetMatrix(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT, matrix: *const FS_MATRIX) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetMatrix\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_NewImageObj(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_NewImageObj\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_CountMarks(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_CountMarks\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetMark\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_AddMark\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_RemoveMark\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_GetName\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObjMark_CountParams(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(mark: FPDF_PAGEOBJECTMARK) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPageObjMark_CountParams\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamKey\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamValueType\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamIntValue\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamStringValue\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_GetParamBlobValue\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_SetIntParam\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_SetStringParam\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_SetBlobParam\0") }
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
        unsafe { self.library.get(b"FPDFPageObjMark_RemoveParam\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_LoadJpegFile\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_LoadJpegFileInline\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_SetMatrix\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_SetBitmap\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetBitmap(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> FPDF_BITMAP>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetBitmap\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_GetRenderedBitmap\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_GetImageDataDecoded\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_GetImageDataRaw\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFImageObj_GetImageFilterCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(image_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFImageObj_GetImageFilterCount\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_GetImageFilter\0") }
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
        unsafe { self.library.get(b"FPDFImageObj_GetImageMetadata\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_CreateNewPath(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(x: c_float, y: c_float) -> FPDF_PAGEOBJECT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_CreateNewPath\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_CreateNewRect\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetBounds\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetBlendMode(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, blend_mode: FPDF_BYTESTRING)>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetBlendMode\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_SetStrokeColor\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetStrokeColor\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetStrokeWidth(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, width: c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetStrokeWidth\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetStrokeWidth\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetLineJoin(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetLineJoin\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetLineJoin(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_join: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetLineJoin\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetLineCap(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetLineCap\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetLineCap(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, line_cap: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetLineCap\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_SetFillColor\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetFillColor\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetDashPhase\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_SetDashPhase(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT, phase: c_float) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_SetDashPhase\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPageObj_GetDashCount(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(page_object: FPDF_PAGEOBJECT) -> c_int>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPageObj_GetDashCount\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_GetDashArray\0") }
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
        unsafe { self.library.get(b"FPDFPageObj_SetDashArray\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_CountSegments(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPath_CountSegments\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPath_GetPathSegment(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPath_GetPathSegment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPathSegment_GetPoint(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(segment: FPDF_PATHSEGMENT, x: *mut f32, y: *mut f32) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPathSegment_GetPoint\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPathSegment_GetType(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFPathSegment_GetType\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFPathSegment_GetClose(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(segment: FPDF_PATHSEGMENT) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFPathSegment_GetClose\0") }
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
        unsafe { self.library.get(b"FPDFFont_GetFontName\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetFlags(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(font: FPDF_FONT) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFFont_GetFlags\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetWeight(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(font: FPDF_FONT) -> c_int>, libloading::Error> {
        unsafe { self.library.get(b"FPDFFont_GetWeight\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetItalicAngle(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFont_GetItalicAngle\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetAscent(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                font: FPDF_FONT,
                font_size: c_float,
                ascent: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFont_GetAscent\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetDescent(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                font: FPDF_FONT,
                font_size: c_float,
                descent: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFont_GetDescent\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetGlyphWidth(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                font: FPDF_FONT,
                glyph: c_uint,
                font_size: c_float,
                width: *mut c_float,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFont_GetGlyphWidth\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFFont_GetGlyphPath(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                font: FPDF_FONT,
                glyph: c_uint,
                font_size: c_float,
            ) -> FPDF_GLYPHPATH,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFFont_GetGlyphPath\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFGlyphPath_CountGlyphSegments(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFGlyphPath_CountGlyphSegments\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFGlyphPath_GetGlyphPathSegment(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(glyphpath: FPDF_GLYPHPATH, index: c_int) -> FPDF_PATHSEGMENT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFGlyphPath_GetGlyphPathSegment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetPrintScaling(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_VIEWERREF_GetPrintScaling\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetNumCopies(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_VIEWERREF_GetNumCopies\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetPrintPageRange(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_PAGERANGE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_VIEWERREF_GetPrintPageRange\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetPrintPageRangeCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(pagerange: FPDF_PAGERANGE) -> size_t>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDF_VIEWERREF_GetPrintPageRangeCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(pagerange: FPDF_PAGERANGE, index: size_t) -> c_int>,
        libloading::Error,
    > {
        unsafe {
            self.library
                .get(b"FPDF_VIEWERREF_GetPrintPageRangeElement\0")
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetDuplex(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_VIEWERREF_GetDuplex\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDF_VIEWERREF_GetName(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                document: FPDF_DOCUMENT,
                key: FPDF_BYTESTRING,
                buffer: *mut c_char,
                length: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDF_VIEWERREF_GetName\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDoc_GetAttachmentCount(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> c_int>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFDoc_GetAttachmentCount\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDoc_AddAttachment(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(document: FPDF_DOCUMENT, name: FPDF_WIDESTRING) -> FPDF_ATTACHMENT,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDoc_AddAttachment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDoc_GetAttachment(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDoc_GetAttachment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFDoc_DeleteAttachment(
        &self,
    ) -> Result<
        Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL>,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFDoc_DeleteAttachment\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_GetName(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                attachment: FPDF_ATTACHMENT,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_GetName\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_HasKey(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(attachment: FPDF_ATTACHMENT, key: FPDF_BYTESTRING) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_HasKey\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_GetValueType(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                attachment: FPDF_ATTACHMENT,
                key: FPDF_BYTESTRING,
            ) -> FPDF_OBJECT_TYPE,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_GetValueType\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_SetStringValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                attachment: FPDF_ATTACHMENT,
                key: FPDF_BYTESTRING,
                value: FPDF_WIDESTRING,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_SetStringValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_GetStringValue(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                attachment: FPDF_ATTACHMENT,
                key: FPDF_BYTESTRING,
                buffer: *mut FPDF_WCHAR,
                buflen: c_ulong,
            ) -> c_ulong,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_GetStringValue\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_SetFile(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                attachment: FPDF_ATTACHMENT,
                document: FPDF_DOCUMENT,
                contents: *const c_void,
                len: c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_SetFile\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFAttachment_GetFile(
        &self,
    ) -> Result<
        Symbol<
            unsafe extern "C" fn(
                attachment: FPDF_ATTACHMENT,
                buffer: *mut c_void,
                buflen: c_ulong,
                out_buflen: *mut c_ulong,
            ) -> FPDF_BOOL,
        >,
        libloading::Error,
    > {
        unsafe { self.library.get(b"FPDFAttachment_GetFile\0") }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn extern_FPDFCatalog_IsTagged(
        &self,
    ) -> Result<Symbol<unsafe extern "C" fn(document: FPDF_DOCUMENT) -> FPDF_BOOL>, libloading::Error>
    {
        unsafe { self.library.get(b"FPDFCatalog_IsTagged\0") }
    }
}

impl PdfiumLibraryBindings for DynamicPdfiumBindings {
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
    fn FPDF_CreateNewDocument(&self) -> FPDF_DOCUMENT {
        unsafe { self.extern_FPDF_CreateNewDocument().unwrap()() }
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
    fn FPDF_LoadMemDocument64(&self, bytes: &[u8], password: Option<&str>) -> FPDF_DOCUMENT {
        let c_password = CString::new(password.unwrap_or("")).unwrap();

        unsafe {
            self.extern_FPDF_LoadMemDocument64().unwrap()(
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

        unsafe { self.extern_FPDF_LoadCustomDocument().unwrap()(pFileAccess, c_password.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_SaveAsCopy(
        &self,
        document: FPDF_DOCUMENT,
        pFileWrite: *mut FPDF_FILEWRITE,
        flags: FPDF_DWORD,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDF_SaveAsCopy().unwrap()(document, pFileWrite, flags) }
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
        unsafe {
            self.extern_FPDF_SaveWithVersion().unwrap()(document, pFileWrite, flags, fileVersion)
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
            self.extern_FPDF_DeviceToPage().unwrap()(
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
            self.extern_FPDF_PageToDevice().unwrap()(
                page, start_x, start_y, size_x, size_y, rotate, page_x, page_y, device_x, device_y,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetFileVersion(&self, doc: FPDF_DOCUMENT, fileVersion: *mut c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDF_GetFileVersion().unwrap()(doc, fileVersion) }
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
        unsafe { self.extern_FPDF_GetFileIdentifier().unwrap()(document, id_type, buffer, buflen) }
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
    fn FPDF_GetDocPermissions(&self, document: FPDF_DOCUMENT) -> c_ulong {
        unsafe { self.extern_FPDF_GetDocPermissions().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSecurityHandlerRevision(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDF_GetSecurityHandlerRevision().unwrap()(document) }
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
    fn FPDF_ImportPagesByIndex(
        &self,
        dest_doc: FPDF_DOCUMENT,
        src_doc: FPDF_DOCUMENT,
        page_indices: *const c_int,
        length: c_ulong,
        index: c_int,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDF_ImportPagesByIndex().unwrap()(
                dest_doc,
                src_doc,
                page_indices,
                length,
                index,
            )
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

        unsafe {
            self.extern_FPDF_ImportPages().unwrap()(dest_doc, src_doc, c_pagerange.as_ptr(), index)
        }
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
            self.extern_FPDF_ImportNPagesToOne().unwrap()(
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
    fn FPDFText_GetCharIndexFromTextIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nTextIndex: c_int,
    ) -> c_int {
        unsafe { self.extern_FPDFText_GetCharIndexFromTextIndex().unwrap()(text_page, nTextIndex) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetTextIndexFromCharIndex(
        &self,
        text_page: FPDF_TEXTPAGE,
        nCharIndex: c_int,
    ) -> c_int {
        unsafe { self.extern_FPDFText_GetTextIndexFromCharIndex().unwrap()(text_page, nCharIndex) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSignatureCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDF_GetSignatureCount().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetSignatureObject(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_SIGNATURE {
        unsafe { self.extern_FPDF_GetSignatureObject().unwrap()(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetContents(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFSignatureObj_GetContents().unwrap()(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetByteRange(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_int,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFSignatureObj_GetByteRange().unwrap()(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetSubFilter(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFSignatureObj_GetSubFilter().unwrap()(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetReason(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_void,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFSignatureObj_GetReason().unwrap()(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetTime(
        &self,
        signature: FPDF_SIGNATURE,
        buffer: *mut c_char,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFSignatureObj_GetTime().unwrap()(signature, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFSignatureObj_GetDocMDPPermission(&self, signature: FPDF_SIGNATURE) -> c_uint {
        unsafe { self.extern_FPDFSignatureObj_GetDocMDPPermission().unwrap()(signature) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetForPage(&self, page: FPDF_PAGE) -> FPDF_STRUCTTREE {
        unsafe { self.extern_FPDF_StructTree_GetForPage().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_Close(&self, struct_tree: FPDF_STRUCTTREE) {
        unsafe { self.extern_FPDF_StructTree_Close().unwrap()(struct_tree) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_CountChildren(&self, struct_tree: FPDF_STRUCTTREE) -> c_int {
        unsafe { self.extern_FPDF_StructTree_CountChildren().unwrap()(struct_tree) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructTree_GetChildAtIndex(
        &self,
        struct_tree: FPDF_STRUCTTREE,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT {
        unsafe { self.extern_FPDF_StructTree_GetChildAtIndex().unwrap()(struct_tree, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetAltText(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            self.extern_FPDF_StructElement_GetAltText().unwrap()(struct_element, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetID(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDF_StructElement_GetID().unwrap()(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetLang(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDF_StructElement_GetLang().unwrap()(struct_element, buffer, buflen) }
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
            self.extern_FPDF_StructElement_GetStringAttribute().unwrap()(
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
        unsafe { self.extern_FPDF_StructElement_GetMarkedContentID().unwrap()(struct_element) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetType(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDF_StructElement_GetType().unwrap()(struct_element, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetTitle(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe {
            self.extern_FPDF_StructElement_GetTitle().unwrap()(struct_element, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_CountChildren(&self, struct_element: FPDF_STRUCTELEMENT) -> c_int {
        unsafe { self.extern_FPDF_StructElement_CountChildren().unwrap()(struct_element) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_StructElement_GetChildAtIndex(
        &self,
        struct_element: FPDF_STRUCTELEMENT,
        index: c_int,
    ) -> FPDF_STRUCTELEMENT {
        unsafe { self.extern_FPDF_StructElement_GetChildAtIndex().unwrap()(struct_element, index) }
    }

    #[allow(non_snake_case)]
    fn FPDFPage_New(
        &self,
        document: FPDF_DOCUMENT,
        page_index: c_int,
        width: c_double,
        height: c_double,
    ) -> FPDF_PAGE {
        unsafe { self.extern_FPDFPage_New().unwrap()(document, page_index, width, height) }
    }

    #[allow(non_snake_case)]
    fn FPDFPage_Delete(&self, document: FPDF_DOCUMENT, page_index: c_int) {
        unsafe { self.extern_FPDFPage_Delete().unwrap()(document, page_index) }
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
    fn FPDFPage_TransFormWithClip(
        &self,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipRect: *const FS_RECTF,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPage_TransFormWithClip().unwrap()(page, matrix, clipRect) }
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
        unsafe {
            self.extern_FPDFPageObj_TransformClipPath().unwrap()(page_object, a, b, c, d, e, f)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPageObj_GetClipPath(&self, page_object: FPDF_PAGEOBJECT) -> FPDF_CLIPPATH {
        unsafe { self.extern_FPDFPageObj_GetClipPath().unwrap()(page_object) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPaths(&self, clip_path: FPDF_CLIPPATH) -> c_int {
        unsafe { self.extern_FPDFClipPath_CountPaths().unwrap()(clip_path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_CountPathSegments(&self, clip_path: FPDF_CLIPPATH, path_index: c_int) -> c_int {
        unsafe { self.extern_FPDFClipPath_CountPathSegments().unwrap()(clip_path, path_index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFClipPath_GetPathSegment(
        &self,
        clip_path: FPDF_CLIPPATH,
        path_index: c_int,
        segment_index: c_int,
    ) -> FPDF_PATHSEGMENT {
        unsafe {
            self.extern_FPDFClipPath_GetPathSegment().unwrap()(clip_path, path_index, segment_index)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_CreateClipPath(&self, left: f32, bottom: f32, right: f32, top: f32) -> FPDF_CLIPPATH {
        unsafe { self.extern_FPDF_CreateClipPath().unwrap()(left, bottom, right, top) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_DestroyClipPath(&self, clipPath: FPDF_CLIPPATH) {
        unsafe { self.extern_FPDF_DestroyClipPath().unwrap()(clipPath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_InsertClipPath(&self, page: FPDF_PAGE, clipPath: FPDF_CLIPPATH) {
        unsafe { self.extern_FPDFPage_InsertClipPath().unwrap()(page, clipPath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_HasTransparency(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPage_HasTransparency().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GenerateContent(&self, page: FPDF_PAGE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPage_GenerateContent().unwrap()(page) }
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
    fn FPDFBitmap_GetFormat(&self, bitmap: FPDF_BITMAP) -> c_int {
        unsafe { self.extern_FPDFBitmap_GetFormat().unwrap()(bitmap) }
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
    fn FPDF_RenderPageBitmapWithMatrix(
        &self,
        bitmap: FPDF_BITMAP,
        page: FPDF_PAGE,
        matrix: *const FS_MATRIX,
        clipping: *const FS_RECTF,
        flags: c_int,
    ) {
        unsafe {
            self.extern_FPDF_RenderPageBitmapWithMatrix().unwrap()(
                bitmap, page, matrix, clipping, flags,
            );
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_IsSupportedSubtype().unwrap()(subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CreateAnnot(
        &self,
        page: FPDF_PAGE,
        subtype: FPDF_ANNOTATION_SUBTYPE,
    ) -> FPDF_ANNOTATION {
        unsafe { self.extern_FPDFPage_CreateAnnot().unwrap()(page, subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotCount(&self, page: FPDF_PAGE) -> c_int {
        unsafe { self.extern_FPDFPage_GetAnnotCount().unwrap()(page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_ANNOTATION {
        unsafe { self.extern_FPDFPage_GetAnnot().unwrap()(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetAnnotIndex(&self, page: FPDF_PAGE, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { self.extern_FPDFPage_GetAnnotIndex().unwrap()(page, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_CloseAnnot(&self, annot: FPDF_ANNOTATION) {
        unsafe { self.extern_FPDFPage_CloseAnnot().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_RemoveAnnot(&self, page: FPDF_PAGE, index: c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPage_RemoveAnnot().unwrap()(page, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetSubtype(&self, annot: FPDF_ANNOTATION) -> FPDF_ANNOTATION_SUBTYPE {
        unsafe { self.extern_FPDFAnnot_GetSubtype().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsObjectSupportedSubtype(&self, subtype: FPDF_ANNOTATION_SUBTYPE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_IsObjectSupportedSubtype().unwrap()(subtype) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_UpdateObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_UpdateObject().unwrap()(annot, obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AddInkStroke(
        &self,
        annot: FPDF_ANNOTATION,
        points: *const FS_POINTF,
        point_count: size_t,
    ) -> c_int {
        unsafe { self.extern_FPDFAnnot_AddInkStroke().unwrap()(annot, points, point_count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveInkList(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_RemoveInkList().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendObject(&self, annot: FPDF_ANNOTATION, obj: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_AppendObject().unwrap()(annot, obj) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObjectCount(&self, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetObjectCount().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_PAGEOBJECT {
        unsafe { self.extern_FPDFAnnot_GetObject().unwrap()(annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_RemoveObject(&self, annot: FPDF_ANNOTATION, index: c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_RemoveObject().unwrap()(annot, index) }
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
        unsafe { self.extern_FPDFAnnot_SetColor().unwrap()(annot, color_type, R, G, B, A) }
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
        unsafe { self.extern_FPDFAnnot_GetColor().unwrap()(annot, color_type, R, G, B, A) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_HasAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_HasAttachmentPoints().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFAnnot_SetAttachmentPoints().unwrap()(annot, quad_index, quad_points)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_AppendAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_points: *const FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_AppendAttachmentPoints().unwrap()(annot, quad_points) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_CountAttachmentPoints(&self, annot: FPDF_ANNOTATION) -> size_t {
        unsafe { self.extern_FPDFAnnot_CountAttachmentPoints().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetAttachmentPoints(
        &self,
        annot: FPDF_ANNOTATION,
        quad_index: size_t,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFAnnot_GetAttachmentPoints().unwrap()(annot, quad_index, quad_points)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetRect(&self, annot: FPDF_ANNOTATION, rect: *const FS_RECTF) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_SetRect().unwrap()(annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetRect(&self, annot: FPDF_ANNOTATION, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_GetRect().unwrap()(annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetVertices(
        &self,
        annot: FPDF_ANNOTATION,
        buffer: *mut FS_POINTF,
        length: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFAnnot_GetVertices().unwrap()(annot, buffer, length) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetInkListCount(&self, annot: FPDF_ANNOTATION) -> c_ulong {
        unsafe { self.extern_FPDFAnnot_GetInkListCount().unwrap()(annot) }
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
        unsafe {
            self.extern_FPDFAnnot_GetInkListPath().unwrap()(annot, path_index, buffer, length)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLine(
        &self,
        annot: FPDF_ANNOTATION,
        start: *mut FS_POINTF,
        end: *mut FS_POINTF,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_GetLine().unwrap()(annot, start, end) }
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
            self.extern_FPDFAnnot_SetBorder().unwrap()(
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
            self.extern_FPDFAnnot_GetBorder().unwrap()(
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

        unsafe { self.extern_FPDFAnnot_HasKey().unwrap()(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetValueType(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { self.extern_FPDFAnnot_GetValueType().unwrap()(annot, c_key.as_ptr()) }
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

        unsafe { self.extern_FPDFAnnot_SetStringValue().unwrap()(annot, c_key.as_ptr(), value) }
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

        unsafe {
            self.extern_FPDFAnnot_GetStringValue().unwrap()(annot, c_key.as_ptr(), buffer, buflen)
        }
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

        unsafe { self.extern_FPDFAnnot_GetNumberValue().unwrap()(annot, c_key.as_ptr(), value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetAP(
        &self,
        annot: FPDF_ANNOTATION,
        appearanceMode: FPDF_ANNOT_APPEARANCEMODE,
        value: FPDF_WIDESTRING,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_SetAP().unwrap()(annot, appearanceMode, value) }
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
        unsafe { self.extern_FPDFAnnot_GetAP().unwrap()(annot, appearanceMode, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLinkedAnnot(&self, annot: FPDF_ANNOTATION, key: &str) -> FPDF_ANNOTATION {
        let c_key = CString::new(key).unwrap();

        unsafe { self.extern_FPDFAnnot_GetLinkedAnnot().unwrap()(annot, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFlags(&self, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetFlags().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFlags(&self, annot: FPDF_ANNOTATION, flags: c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_SetFlags().unwrap()(annot, flags) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldFlags(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetFormFieldFlags().unwrap()(handle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldAtPoint(
        &self,
        hHandle: FPDF_FORMHANDLE,
        page: FPDF_PAGE,
        point: *const FS_POINTF,
    ) -> FPDF_ANNOTATION {
        unsafe { self.extern_FPDFAnnot_GetFormFieldAtPoint().unwrap()(hHandle, page, point) }
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
        unsafe { self.extern_FPDFAnnot_GetFormFieldName().unwrap()(hHandle, annot, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormFieldType(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetFormFieldType().unwrap()(hHandle, annot) }
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
        unsafe {
            self.extern_FPDFAnnot_GetFormFieldValue().unwrap()(hHandle, annot, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetOptionCount(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetOptionCount().unwrap()(hHandle, annot) }
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
        unsafe {
            self.extern_FPDFAnnot_GetOptionLabel().unwrap()(hHandle, annot, index, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsOptionSelected(
        &self,
        handle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        index: c_int,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_IsOptionSelected().unwrap()(handle, annot, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFontSize(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
        value: *mut f32,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_GetFontSize().unwrap()(hHandle, annot, value) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_IsChecked(&self, hHandle: FPDF_FORMHANDLE, annot: FPDF_ANNOTATION) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_IsChecked().unwrap()(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *const FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_SetFocusableSubtypes().unwrap()(hHandle, subtypes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypesCount(&self, hHandle: FPDF_FORMHANDLE) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetFocusableSubtypesCount().unwrap()(hHandle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFocusableSubtypes(
        &self,
        hHandle: FPDF_FORMHANDLE,
        subtypes: *mut FPDF_ANNOTATION_SUBTYPE,
        count: size_t,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFAnnot_GetFocusableSubtypes().unwrap()(hHandle, subtypes, count) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetLink(&self, annot: FPDF_ANNOTATION) -> FPDF_LINK {
        unsafe { self.extern_FPDFAnnot_GetLink().unwrap()(annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlCount(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetFormControlCount().unwrap()(hHandle, annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_GetFormControlIndex(
        &self,
        hHandle: FPDF_FORMHANDLE,
        annot: FPDF_ANNOTATION,
    ) -> c_int {
        unsafe { self.extern_FPDFAnnot_GetFormControlIndex().unwrap()(hHandle, annot) }
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
        unsafe {
            self.extern_FPDFAnnot_GetFormFieldExportValue().unwrap()(hHandle, annot, buffer, buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAnnot_SetURI(&self, annot: FPDF_ANNOTATION, uri: &str) -> FPDF_BOOL {
        let c_uri = CString::new(uri).unwrap();

        unsafe { self.extern_FPDFAnnot_SetURI().unwrap()(annot, c_uri.as_ptr()) }
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
    fn FORM_OnAfterLoadPage(&self, page: FPDF_PAGE, handle: FPDF_FORMHANDLE) {
        unsafe {
            self.extern_FORM_OnAfterLoadPage().unwrap()(page, handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FORM_OnBeforeClosePage(&self, page: FPDF_PAGE, handle: FPDF_FORMHANDLE) {
        unsafe {
            self.extern_FORM_OnBeforeClosePage().unwrap()(page, handle);
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetPageMode(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDFDoc_GetPageMode().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_Flatten(&self, page: FPDF_PAGE, nFlag: c_int) -> c_int {
        unsafe { self.extern_FPDFPage_Flatten().unwrap()(page, nFlag) }
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
    fn FPDFBookmark_GetCount(&self, bookmark: FPDF_BOOKMARK) -> c_int {
        unsafe { self.extern_FPDFBookmark_GetCount().unwrap()(bookmark) }
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
    fn FPDFDest_GetView(
        &self,
        dest: FPDF_DEST,
        pNumParams: *mut c_ulong,
        pParams: *mut FS_FLOAT,
    ) -> c_ulong {
        unsafe { self.extern_FPDFDest_GetView().unwrap()(dest, pNumParams, pParams) }
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
            self.extern_FPDFDest_GetLocationInPage().unwrap()(
                dest, hasXVal, hasYVal, hasZoomVal, x, y, zoom,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> FPDF_LINK {
        unsafe { self.extern_FPDFLink_GetLinkAtPoint().unwrap()(page, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetLinkZOrderAtPoint(&self, page: FPDF_PAGE, x: c_double, y: c_double) -> c_int {
        unsafe { self.extern_FPDFLink_GetLinkZOrderAtPoint().unwrap()(page, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetDest(&self, document: FPDF_DOCUMENT, link: FPDF_LINK) -> FPDF_DEST {
        unsafe { self.extern_FPDFLink_GetDest().unwrap()(document, link) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAction(&self, link: FPDF_LINK) -> FPDF_ACTION {
        unsafe { self.extern_FPDFLink_GetAction().unwrap()(link) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_Enumerate(
        &self,
        page: FPDF_PAGE,
        start_pos: *mut c_int,
        link_annot: *mut FPDF_LINK,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFLink_Enumerate().unwrap()(page, start_pos, link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnot(&self, page: FPDF_PAGE, link_annot: FPDF_LINK) -> FPDF_ANNOTATION {
        unsafe { self.extern_FPDFLink_GetAnnot().unwrap()(page, link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetAnnotRect(&self, link_annot: FPDF_LINK, rect: *mut FS_RECTF) -> FPDF_BOOL {
        unsafe { self.extern_FPDFLink_GetAnnotRect().unwrap()(link_annot, rect) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountQuadPoints(&self, link_annot: FPDF_LINK) -> c_int {
        unsafe { self.extern_FPDFLink_CountQuadPoints().unwrap()(link_annot) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_GetQuadPoints(
        &self,
        link_annot: FPDF_LINK,
        quad_index: c_int,
        quad_points: *mut FS_QUADPOINTSF,
    ) -> FPDF_BOOL {
        unsafe {
            self.extern_FPDFLink_GetQuadPoints().unwrap()(link_annot, quad_index, quad_points)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_GetPageAAction(&self, page: FPDF_PAGE, aa_type: c_int) -> FPDF_ACTION {
        unsafe { self.extern_FPDF_GetPageAAction().unwrap()(page, aa_type) }
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

    #[allow(non_snake_case)]
    fn FPDFText_GetUnicode(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_uint {
        unsafe { self.extern_FPDFText_GetUnicode().unwrap()(text_page, index) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetFontSize(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_double {
        unsafe { self.extern_FPDFText_GetFontSize().unwrap()(text_page, index) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetFontInfo(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        buffer: *mut c_void,
        buflen: c_ulong,
        flags: *mut c_int,
    ) -> c_ulong {
        unsafe {
            self.extern_FPDFText_GetFontInfo().unwrap()(text_page, index, buffer, buflen, flags)
        }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetFontWeight(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_int {
        unsafe { self.extern_FPDFText_GetFontWeight().unwrap()(text_page, index) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetTextRenderMode(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
    ) -> FPDF_TEXT_RENDERMODE {
        unsafe { self.extern_FPDFText_GetTextRenderMode().unwrap()(text_page, index) }
    }

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
        unsafe { self.extern_FPDFText_GetFillColor().unwrap()(text_page, index, R, G, B, A) }
    }

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
        unsafe { self.extern_FPDFText_GetStrokeColor().unwrap()(text_page, index, R, G, B, A) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetCharAngle(&self, text_page: FPDF_TEXTPAGE, index: c_int) -> c_float {
        unsafe { self.extern_FPDFText_GetCharAngle().unwrap()(text_page, index) }
    }

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
        unsafe {
            self.extern_FPDFText_GetCharBox().unwrap()(text_page, index, left, right, bottom, top)
        }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetLooseCharBox(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        rect: *mut FS_RECTF,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_GetLooseCharBox().unwrap()(text_page, index, rect) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetMatrix(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        matrix: *mut FS_MATRIX,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_GetMatrix().unwrap()(text_page, index, matrix) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetCharOrigin(
        &self,
        text_page: FPDF_TEXTPAGE,
        index: c_int,
        x: *mut c_double,
        y: *mut c_double,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_GetCharOrigin().unwrap()(text_page, index, x, y) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetCharIndexAtPos(
        &self,
        text_page: FPDF_TEXTPAGE,
        x: c_double,
        y: c_double,
        xTolerance: c_double,
        yTolerance: c_double,
    ) -> c_int {
        unsafe {
            self.extern_FPDFText_GetCharIndexAtPos().unwrap()(
                text_page, x, y, xTolerance, yTolerance,
            )
        }
    }

    #[allow(non_snake_case)]
    fn FPDFText_GetText(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
        result: *mut c_ushort,
    ) -> c_int {
        unsafe { self.extern_FPDFText_GetText().unwrap()(text_page, start_index, count, result) }
    }

    #[allow(non_snake_case)]
    fn FPDFText_CountRects(
        &self,
        text_page: FPDF_TEXTPAGE,
        start_index: c_int,
        count: c_int,
    ) -> c_int {
        unsafe { self.extern_FPDFText_CountRects().unwrap()(text_page, start_index, count) }
    }

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
        unsafe {
            self.extern_FPDFText_GetRect().unwrap()(text_page, rect_index, left, top, right, bottom)
        }
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
    fn FPDFText_FindStart(
        &self,
        text_page: FPDF_TEXTPAGE,
        findwhat: FPDF_WIDESTRING,
        flags: c_ulong,
        start_index: c_int,
    ) -> FPDF_SCHHANDLE {
        unsafe {
            self.extern_FPDFText_FindStart().unwrap()(text_page, findwhat, flags, start_index)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindNext(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_FindNext().unwrap()(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindPrev(&self, handle: FPDF_SCHHANDLE) -> FPDF_BOOL {
        unsafe { self.extern_FPDFText_FindPrev().unwrap()(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetSchResultIndex(&self, handle: FPDF_SCHHANDLE) -> c_int {
        unsafe { self.extern_FPDFText_GetSchResultIndex().unwrap()(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_GetSchCount(&self, handle: FPDF_SCHHANDLE) -> c_int {
        unsafe { self.extern_FPDFText_GetSchCount().unwrap()(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_FindClose(&self, handle: FPDF_SCHHANDLE) {
        unsafe { self.extern_FPDFText_FindClose().unwrap()(handle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_LoadWebLinks(&self, text_page: FPDF_TEXTPAGE) -> FPDF_PAGELINK {
        unsafe { self.extern_FPDFLink_LoadWebLinks().unwrap()(text_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountWebLinks(&self, link_page: FPDF_PAGELINK) -> c_int {
        unsafe { self.extern_FPDFLink_CountWebLinks().unwrap()(link_page) }
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
        unsafe { self.extern_FPDFLink_GetURL().unwrap()(link_page, link_index, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CountRects(&self, link_page: FPDF_PAGELINK, link_index: c_int) -> c_int {
        unsafe { self.extern_FPDFLink_CountRects().unwrap()(link_page, link_index) }
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
            self.extern_FPDFLink_GetRect().unwrap()(
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
            self.extern_FPDFLink_GetTextRange().unwrap()(
                link_page,
                link_index,
                start_char_index,
                char_count,
            )
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFLink_CloseWebLinks(&self, link_page: FPDF_PAGELINK) {
        unsafe { self.extern_FPDFLink_CloseWebLinks().unwrap()(link_page) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetDecodedThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFPage_GetDecodedThumbnailData().unwrap()(page, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetRawThumbnailData(
        &self,
        page: FPDF_PAGE,
        buffer: *mut c_void,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFPage_GetRawThumbnailData().unwrap()(page, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPage_GetThumbnailAsBitmap(&self, page: FPDF_PAGE) -> FPDF_BITMAP {
        unsafe { self.extern_FPDFPage_GetThumbnailAsBitmap().unwrap()(page) }
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
        font: &str,
        font_size: c_float,
    ) -> FPDF_PAGEOBJECT {
        let c_font = CString::new(font).unwrap();

        unsafe {
            self.extern_FPDFPageObj_NewTextObj().unwrap()(document, c_font.as_ptr(), font_size)
        }
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
    fn FPDFText_LoadFont(
        &self,
        document: FPDF_DOCUMENT,
        data: *const c_uchar,
        size: c_uint,
        font_type: c_int,
        cid: FPDF_BOOL,
    ) -> FPDF_FONT {
        unsafe { self.extern_FPDFText_LoadFont().unwrap()(document, data, size, font_type, cid) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFText_LoadStandardFont(&self, document: FPDF_DOCUMENT, font: &str) -> FPDF_FONT {
        let c_font = CString::new(font).unwrap();

        unsafe { self.extern_FPDFText_LoadStandardFont().unwrap()(document, c_font.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_Close(&self, font: FPDF_FONT) {
        unsafe { self.extern_FPDFFont_Close().unwrap()(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_MoveTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPath_MoveTo().unwrap()(path, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_LineTo(&self, path: FPDF_PAGEOBJECT, x: c_float, y: c_float) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPath_LineTo().unwrap()(path, x, y) }
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
        unsafe { self.extern_FPDFPath_BezierTo().unwrap()(path, x1, y1, x2, y2, x3, y3) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_Close(&self, path: FPDF_PAGEOBJECT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPath_Close().unwrap()(path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_SetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: c_int,
        stroke: FPDF_BOOL,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPath_SetDrawMode().unwrap()(path, fillmode, stroke) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetDrawMode(
        &self,
        path: FPDF_PAGEOBJECT,
        fillmode: *mut c_int,
        stroke: *mut FPDF_BOOL,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPath_GetDrawMode().unwrap()(path, fillmode, stroke) }
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
    fn FPDFPageObj_AddMark(&self, page_object: FPDF_PAGEOBJECT, name: &str) -> FPDF_PAGEOBJECTMARK {
        let c_name = CString::new(name).unwrap();

        unsafe { self.extern_FPDFPageObj_AddMark().unwrap()(page_object, c_name.as_ptr()) }
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
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { self.extern_FPDFPageObjMark_GetParamValueType().unwrap()(mark, c_key.as_ptr()) }
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

        unsafe {
            self.extern_FPDFPageObjMark_GetParamIntValue().unwrap()(mark, c_key.as_ptr(), out_value)
        }
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
            self.extern_FPDFPageObjMark_GetParamStringValue().unwrap()(
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
            self.extern_FPDFPageObjMark_GetParamBlobValue().unwrap()(
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
            self.extern_FPDFPageObjMark_SetIntParam().unwrap()(
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
            self.extern_FPDFPageObjMark_SetStringParam().unwrap()(
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
            self.extern_FPDFPageObjMark_SetBlobParam().unwrap()(
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

        unsafe {
            self.extern_FPDFPageObjMark_RemoveParam().unwrap()(page_object, mark, c_key.as_ptr())
        }
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
    fn FPDFPageObj_SetBlendMode(&self, page_object: FPDF_PAGEOBJECT, blend_mode: &str) {
        let c_blend_mode = CString::new(blend_mode).unwrap();

        unsafe {
            self.extern_FPDFPageObj_SetBlendMode().unwrap()(page_object, c_blend_mode.as_ptr())
        }
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
    fn FPDFPath_CountSegments(&self, path: FPDF_PAGEOBJECT) -> c_int {
        unsafe { self.extern_FPDFPath_CountSegments().unwrap()(path) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPath_GetPathSegment(&self, path: FPDF_PAGEOBJECT, index: c_int) -> FPDF_PATHSEGMENT {
        unsafe { self.extern_FPDFPath_GetPathSegment().unwrap()(path, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetPoint(
        &self,
        segment: FPDF_PATHSEGMENT,
        x: *mut c_float,
        y: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPathSegment_GetPoint().unwrap()(segment, x, y) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetType(&self, segment: FPDF_PATHSEGMENT) -> c_int {
        unsafe { self.extern_FPDFPathSegment_GetType().unwrap()(segment) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFPathSegment_GetClose(&self, segment: FPDF_PATHSEGMENT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFPathSegment_GetClose().unwrap()(segment) }
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

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetFlags(&self, font: FPDF_FONT) -> c_int {
        unsafe { self.extern_FPDFFont_GetFlags().unwrap()(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetWeight(&self, font: FPDF_FONT) -> c_int {
        unsafe { self.extern_FPDFFont_GetWeight().unwrap()(font) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetItalicAngle(&self, font: FPDF_FONT, angle: *mut c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFFont_GetItalicAngle().unwrap()(font, angle) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetAscent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        ascent: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFFont_GetAscent().unwrap()(font, font_size, ascent) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetDescent(
        &self,
        font: FPDF_FONT,
        font_size: c_float,
        descent: *mut c_float,
    ) -> FPDF_BOOL {
        unsafe { self.extern_FPDFFont_GetDescent().unwrap()(font, font_size, descent) }
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
        unsafe { self.extern_FPDFFont_GetGlyphWidth().unwrap()(font, glyph, font_size, width) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFFont_GetGlyphPath(
        &self,
        font: FPDF_FONT,
        glyph: c_uint,
        font_size: c_float,
    ) -> FPDF_GLYPHPATH {
        unsafe { self.extern_FPDFFont_GetGlyphPath().unwrap()(font, glyph, font_size) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_CountGlyphSegments(&self, glyphpath: FPDF_GLYPHPATH) -> c_int {
        unsafe { self.extern_FPDFGlyphPath_CountGlyphSegments().unwrap()(glyphpath) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFGlyphPath_GetGlyphPathSegment(
        &self,
        glyphpath: FPDF_GLYPHPATH,
        index: c_int,
    ) -> FPDF_PATHSEGMENT {
        unsafe { self.extern_FPDFGlyphPath_GetGlyphPathSegment().unwrap()(glyphpath, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintScaling(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        unsafe { self.extern_FPDF_VIEWERREF_GetPrintScaling().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetNumCopies(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDF_VIEWERREF_GetNumCopies().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRange(&self, document: FPDF_DOCUMENT) -> FPDF_PAGERANGE {
        unsafe { self.extern_FPDF_VIEWERREF_GetPrintPageRange().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeCount(&self, pagerange: FPDF_PAGERANGE) -> size_t {
        unsafe { self.extern_FPDF_VIEWERREF_GetPrintPageRangeCount().unwrap()(pagerange) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetPrintPageRangeElement(
        &self,
        pagerange: FPDF_PAGERANGE,
        index: size_t,
    ) -> c_int {
        unsafe {
            self.extern_FPDF_VIEWERREF_GetPrintPageRangeElement()
                .unwrap()(pagerange, index)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDF_VIEWERREF_GetDuplex(&self, document: FPDF_DOCUMENT) -> FPDF_DUPLEXTYPE {
        unsafe { self.extern_FPDF_VIEWERREF_GetDuplex().unwrap()(document) }
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

        unsafe {
            self.extern_FPDF_VIEWERREF_GetName().unwrap()(document, c_key.as_ptr(), buffer, length)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachmentCount(&self, document: FPDF_DOCUMENT) -> c_int {
        unsafe { self.extern_FPDFDoc_GetAttachmentCount().unwrap()(document) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_AddAttachment(
        &self,
        document: FPDF_DOCUMENT,
        name: FPDF_WIDESTRING,
    ) -> FPDF_ATTACHMENT {
        unsafe { self.extern_FPDFDoc_AddAttachment().unwrap()(document, name) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_GetAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_ATTACHMENT {
        unsafe { self.extern_FPDFDoc_GetAttachment().unwrap()(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFDoc_DeleteAttachment(&self, document: FPDF_DOCUMENT, index: c_int) -> FPDF_BOOL {
        unsafe { self.extern_FPDFDoc_DeleteAttachment().unwrap()(document, index) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetName(
        &self,
        attachment: FPDF_ATTACHMENT,
        buffer: *mut FPDF_WCHAR,
        buflen: c_ulong,
    ) -> c_ulong {
        unsafe { self.extern_FPDFAttachment_GetName().unwrap()(attachment, buffer, buflen) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_HasKey(&self, attachment: FPDF_ATTACHMENT, key: &str) -> FPDF_BOOL {
        let c_key = CString::new(key).unwrap();

        unsafe { self.extern_FPDFAttachment_HasKey().unwrap()(attachment, c_key.as_ptr()) }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFAttachment_GetValueType(
        &self,
        attachment: FPDF_ATTACHMENT,
        key: &str,
    ) -> FPDF_OBJECT_TYPE {
        let c_key = CString::new(key).unwrap();

        unsafe { self.extern_FPDFAttachment_GetValueType().unwrap()(attachment, c_key.as_ptr()) }
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

        unsafe {
            self.extern_FPDFAttachment_SetStringValue().unwrap()(attachment, c_key.as_ptr(), value)
        }
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
            self.extern_FPDFAttachment_GetStringValue().unwrap()(
                attachment,
                c_key.as_ptr(),
                buffer,
                buflen,
            )
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
        unsafe {
            self.extern_FPDFAttachment_SetFile().unwrap()(attachment, document, contents, len)
        }
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
        unsafe {
            self.extern_FPDFAttachment_GetFile().unwrap()(attachment, buffer, buflen, out_buflen)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn FPDFCatalog_IsTagged(&self, document: FPDF_DOCUMENT) -> FPDF_BOOL {
        unsafe { self.extern_FPDFCatalog_IsTagged().unwrap()(document) }
    }
}
