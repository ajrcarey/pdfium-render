// Initialize bindings between the pdfium and pdfium-render WASM modules.

var FPDF = {
	bytesPerPixel: 4
};

function initializePdfiumRender() {
	// Export functions exposed by pdfium.js into 'FPDF' object namespace.
	// Certain functions that expect pointers to byte buffers have to be
	// handled carefully, since we cannot retrieve a pointer to a buffer from
	// one WASM module and then hand it over to another. This affects the
	// LoadMemDocument() function.

	FPDF.InitLibrary = Module.cwrap('PDFium_Init');
	FPDF.DestroyLibrary = Module.cwrap('FPDF_DestroyLibrary');
	FPDF.GetLastError = Module.cwrap('FPDF_GetLastError', 'number');

	FPDF.LoadMemDocument = function(bytes, password) {
		let f = Module.cwrap('FPDF_LoadMemDocument', 'number', ['number', 'number', 'string']);

		// Allocate memory for this document buffer.

		FPDF._documentBuffer = Module._malloc(bytes.length);
		Module.HEAPU8.set(bytes, FPDF._documentBuffer);

		return f(FPDF._documentBuffer, bytes.length, password);
	};
	FPDF.CloseDocument = function(handle) {
		let f = Module.cwrap('FPDF_CloseDocument', '', ['number']);

		f(handle);

		if (FPDF._documentBuffer) {
			// Free memory allocated to a previous document buffer.

			Module._free(FPDF._documentBuffer);
			delete FPDF._documentBuffer;
		}
	};
	FPDF.GetPageCount = Module.cwrap('FPDF_GetPageCount', 'number', ['number']);

	FPDF.LoadPage = Module.cwrap('FPDF_LoadPage', 'number', ['number', 'number']);
	FPDF.ClosePage = Module.cwrap('FPDF_ClosePage', '', ['number']);
	FPDF.GetPageSizeByIndex = Module.cwrap('FPDF_GetPageSizeByIndex', 'number', ['number', 'number', 'number', 'number']);
	FPDF.GetPageWidthF = Module.cwrap('FPDF_GetPageWidthF', 'number', ['number']);
	FPDF.GetPageHeightF = Module.cwrap('FPDF_GetPageHeightF', 'number', ['number']);
	FPDF.RenderPageBitmap = Module.cwrap('FPDF_RenderPageBitmap', '', ['number', 'number', 'number', 'number', 'number', 'number', 'number', 'number']);

	FPDF.Bitmap_CreateEx = Module.cwrap('FPDFBitmap_CreateEx', 'number', ['number', 'number', 'number', 'number', 'number']);
	FPDF.Bitmap_Destroy = Module.cwrap('FPDFBitmap_Destroy', '', ['number']);
	FPDF.Bitmap_FillRect = Module.cwrap('FPDFBitmap_FillRect', '', ['number', 'number', 'number', 'number', 'number', 'number']);
	FPDF.Bitmap_GetWidth = Module.cwrap('FPDFBitmap_GetWidth', 'number', ['number']);
	FPDF.Bitmap_GetHeight = Module.cwrap('FPDFBitmap_GetHeight', 'number', ['number']);
	FPDF.Bitmap_GetStride = Module.cwrap('FPDFBitmap_GetStride', 'number', ['number']);
	FPDF.Bitmap_GetBuffer = function(handle) {
		let f = Module.cwrap('FPDFBitmap_GetBuffer', 'number', ['number']);

		// Retrieve a pointer to the bitmap buffer in pdfium's WASM memory heap ...

		let bitmapBufferHeapPtr = f(handle);

		// ... and return a copy of the bytes in the heap at that location.

		let bytes = [];

		let width = FPDF.Bitmap_GetWidth(handle);
		let height = FPDF.Bitmap_GetHeight(handle);

		for (let i = 0; i < width * height * FPDF.bytesPerPixel; i ++) {
			bytes.push(Module.HEAPU8[bitmapBufferHeapPtr + i]);
		}

		return bytes;
	}
}
