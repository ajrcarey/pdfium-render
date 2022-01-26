// Initialize bindings between the Pdfium WASM module and pdfium-render.

var FPDF = {
	bytesPerPixel: 4
};

function initializePdfiumRender() {
	// Wrap functions exposed by Pdfium into the FPDF top-level object.
	// Certain functions that expect pointers to byte buffers have to be
	// handled carefully, since we cannot retrieve a pointer to a buffer from
	// one WASM module and then hand it over to another; WASM memory heaps are
	// completely separate from one another. This affects the LoadMemDocument() function,
	// for instance, and some others.

	// The Pdfium WASM module is compiled using emscripten, so we can use utility
	// functions provided by emscripten's exported Module top-level object.
	// Module.ccall() calls a function in the WASM module; Module.cwrap() curries
	// a function in the WASM module; and the Module.HEAP* functions expose the raw
	// WASM memory heap in a variety of memory cell sizes (HEAP8 = i8, HEAPU8 = u8, ...
	// HEAP32 = i32, HEAPU32 = u32).

	FPDF.InitLibrary = Module.cwrap('PDFium_Init');
	FPDF.DestroyLibrary = Module.cwrap('FPDF_DestroyLibrary');
	FPDF.GetLastError = Module.cwrap('FPDF_GetLastError', 'number');

	FPDF.LoadMemDocument = function(bytes, password) {
		// Allocate memory for this document buffer.

		FPDF._documentBuffer = Module._malloc(bytes.length);
		Module.HEAPU8.set(bytes, FPDF._documentBuffer);

		return Module.ccall('FPDF_LoadMemDocument', 'number', ['number', 'number', 'string'], [FPDF._documentBuffer, bytes.length, password]);
	};
	FPDF.CloseDocument = function(handle) {
		Module.ccall('FPDF_CloseDocument', '', ['number'], [handle]);

		if (FPDF._documentBuffer) {
			// Free memory allocated to a previous document buffer.

			Module._free(FPDF._documentBuffer);
			delete FPDF._documentBuffer;
		}
	};
	FPDF.GetFileVersion = function(handle) {
		// FPDF_GetFileVersion takes a pointer to a mutable integer and writes
		// the file version to that pointer, returning a boolean result indicating
		// success. We adjust things slightly here so that we always return the
		// integer file version if FPDF_GetFileVersion() returned true, or
		// -1 if FPDF_GetFileVersion() returned false.

		let BYTES_PER_I32 = 4; // c_int is an i32, i.e. 4 bytes
		let ptr = Module._malloc(BYTES_PER_I32);

		let result = Module.ccall('FPDF_GetFileVersion', 'number', ['number', 'number'], [handle, ptr]);

		if (result) {
			// The function call succeeded. Retrieve the value written to the pointer.

			// We use HEAP32 to access the memory heap since we want an i32 back;
			// if we wanted a u32, for instance, we would use HEAPU32 instead.

			let result = Module.HEAP32[ptr / BYTES_PER_I32];
			Module._free(ptr);

			return result;
		} else {
			// The function call failed.

			return -1;
		}
	}
	FPDF.GetFormType = Module.cwrap('FPDF_GetFormType', 'number', ['number']);
	FPDF.GetMetaText = function(memory, document, tag, bufferPtr, bufferLength) {
		// The tag parameter is a null-terminated C-style char array. Pdfium expects
		// to receive a pointer to this array, not the array itself.

		let tagPtr = Module._malloc(tag.length);
		Module.HEAPU8.set(tag, tagPtr);

		// If bufferLength != 0, then pdfium-render has created a buffer for us in its
		// memory heap in which Pdfium's result should be stored. However, Pdfium cannot
		// access this buffer itself; Pdfium's memory heap is separate. We create a local
		// buffer for Pdfium to use...

		let localBufferPtr = bufferLength > 0 ? Module._malloc(bufferLength) : undefined;

		// ... let Pdfium write its result into that local buffer...

		let result = Module.ccall('FPDF_GetMetaText', 'number', ['number', 'number', 'number', 'number'], [document, tagPtr, localBufferPtr, bufferLength]);
		Module._free(tagPtr);

		// ... then copy the contents of the local buffer containing Pdfium's result
		// back into pdfium-render's memory heap.

		if (bufferLength > 0) {
			let buffer = new Uint8Array(memory.buffer, bufferPtr, bufferLength);
			buffer.set(Module.HEAPU8.slice(localBufferPtr, localBufferPtr + bufferLength));
			Module._free(localBufferPtr);
		}

		return result;
	}
	FPDF.GetPageCount = Module.cwrap('FPDF_GetPageCount', 'number', ['number']);

	FPDF.LoadPage = Module.cwrap('FPDF_LoadPage', 'number', ['number', 'number']);
	FPDF.ClosePage = Module.cwrap('FPDF_ClosePage', '', ['number']);
	FPDF.GetPageLabel = function(document, page_index, buffer, buflen) {
		return Module.ccall('FPDF_GetPageLabel', 'number', ['number', 'number', 'number', 'number'], [document, page_index, buffer, buflen]);
	}
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
		// Retrieve a pointer to the bitmap buffer in Pdfium's WASM memory heap...

		let bitmapBufferHeapPtr = Module.ccall('FPDFBitmap_GetBuffer', 'number', ['number'], [handle]);

		// ... and return a copy of the bytes in the heap at that location.

		let bitmapLength = FPDF.Bitmap_GetWidth(handle) * FPDF.Bitmap_GetHeight(handle) * FPDF.bytesPerPixel;

		let bytes = new Uint8Array(bitmapLength);
		bytes.set(Module.HEAPU8.slice(bitmapBufferHeapPtr, bitmapBufferHeapPtr + bitmapLength));

		return bytes;
	}

	FPDF.DOC_InitFormFillEnvironment = function(memory, handle, formInfoPtr, formInfoLength) {
		// pdfium-render has passed us a pointer to a FPDF_FORMFILLINFO structure in its
		// own memory heap. However, Pdfium cannot access this structure itself; it can
		// only access structures in its own memory heap. We therefore copy the FPDF_FORMFILLINFO
		// structure from pdfium-render's memory heap into Pdfium's memory heap.

		FPDF._formInfoBuffer = Module._malloc(formInfoLength);

		Module.HEAPU8.set(new Uint8Array(memory, formInfoPtr, formInfoLength), FPDF._formInfoBuffer);

		// Zero out the space occupied by the FPDF_FORMFILLINFO struct...

		for (let i = 0; i < formInfoLength; i ++) {
			Module.HEAPU8[FPDF._formInfoBuffer + i] = 0;
		}

		// ... and manually set the first, and only required, field in the struct,
		// the FPDF_FORMFILLINFO.version field.

		Module.HEAPU8[FPDF._formInfoBuffer] = 2;

		return Module.ccall('FPDFDOC_InitFormFillEnvironment', 'number', ['number', 'number'], [handle, FPDF._formInfoBuffer]);
	}
	FPDF.DOC_ExitFormFillEnvironment = function(handle) {
		Module.ccall('FPDFDOC_ExitFormFillEnvironment', '', ['number'], [handle]);

		if (FPDF._formInfoBuffer) {
			Module._free(FPDF._formInfoBuffer);
			delete FPDF._formInfoBuffer;
		}
	}
	FPDF.SetFormFieldHighlightColor = Module.cwrap('FPDF_SetFormFieldHighlightColor', '', ['number', 'number', 'number']);
	FPDF.SetFormFieldHighlightAlpha = Module.cwrap('FPDF_SetFormFieldHighlightAlpha', '', ['number', 'number']);
	FPDF.FFLDraw = Module.cwrap('FPDF_FFLDraw', '', ['number', 'number', 'number', 'number', 'number', 'number', 'number', 'number', 'number']);
}
