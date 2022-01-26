let wasm_bindgen;
(function() {
    const __exports = {};
    let wasm;

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* Logs the width and height of each page in the sample PDF, along with other
* document metrics, to the Javascript console.
*/
__exports.log_page_metrics_to_console = function() {
    wasm.log_page_metrics_to_console();
};

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
* Returns the raw image byte data for a nominated page in the PDF file.
* @param {number} index
* @param {number} width
* @param {number} height
* @returns {Uint8Array}
*/
__exports.get_image_data_for_page = function(index, width, height) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.get_image_data_for_page(retptr, index, width, height);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v0 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
};

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        let src;
        if (typeof document === 'undefined') {
            src = location.href;
        } else {
            src = document.currentScript.src;
        }
        input = src.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_debug_675b0ecb65722d2a = function(arg0) {
        console.debug(getObject(arg0));
    };
    imports.wbg.__wbg_error_cc38ce2b4b661e1d = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_info_e0c9813e6fd3bdc1 = function(arg0) {
        console.info(getObject(arg0));
    };
    imports.wbg.__wbg_log_3445347661d4505e = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_warn_5ec7c7c02d0b3841 = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_InitLibrary_56c9e9da180ac27b = typeof FPDF.InitLibrary == 'function' ? FPDF.InitLibrary : notDefined('FPDF.InitLibrary');
    imports.wbg.__wbg_DestroyLibrary_fbd9d19af86a4eb3 = typeof FPDF.DestroyLibrary == 'function' ? FPDF.DestroyLibrary : notDefined('FPDF.DestroyLibrary');
    imports.wbg.__wbg_GetLastError_154bc138563a6fd5 = typeof FPDF.GetLastError == 'function' ? FPDF.GetLastError : notDefined('FPDF.GetLastError');
    imports.wbg.__wbg_LoadMemDocument_d3948bb56adbc8a3 = function(arg0, arg1, arg2, arg3) {
        var ret = FPDF.LoadMemDocument(getArrayU8FromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_CloseDocument_14c0dadea7ec4a0a = typeof FPDF.CloseDocument == 'function' ? FPDF.CloseDocument : notDefined('FPDF.CloseDocument');
    imports.wbg.__wbg_GetFileVersion_5c2029cbf7ba3df2 = typeof FPDF.GetFileVersion == 'function' ? FPDF.GetFileVersion : notDefined('FPDF.GetFileVersion');
    imports.wbg.__wbg_GetFormType_f6d76990355ed506 = typeof FPDF.GetFormType == 'function' ? FPDF.GetFormType : notDefined('FPDF.GetFormType');
    imports.wbg.__wbg_GetMetaText_a454ee23d5e481cb = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        var v0 = getArrayU8FromWasm0(arg2, arg3).slice();
        wasm.__wbindgen_free(arg2, arg3 * 1);
        var ret = FPDF.GetMetaText(takeObject(arg0), arg1, v0, arg4, arg5 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_GetPageCount_d85f48fc892a9243 = typeof FPDF.GetPageCount == 'function' ? FPDF.GetPageCount : notDefined('FPDF.GetPageCount');
    imports.wbg.__wbg_LoadPage_d242e19b12107afc = typeof FPDF.LoadPage == 'function' ? FPDF.LoadPage : notDefined('FPDF.LoadPage');
    imports.wbg.__wbg_ClosePage_1a56ee9940d6e95d = typeof FPDF.ClosePage == 'function' ? FPDF.ClosePage : notDefined('FPDF.ClosePage');
    imports.wbg.__wbg_GetPageLabel_0c7702579b0c0dbb = function(arg0, arg1, arg2, arg3) {
        var ret = FPDF.GetPageLabel(arg0, arg1, arg2, arg3 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_GetPageWidthF_53464b689ffe7595 = typeof FPDF.GetPageWidthF == 'function' ? FPDF.GetPageWidthF : notDefined('FPDF.GetPageWidthF');
    imports.wbg.__wbg_GetPageHeightF_95e032f3a6ad02f3 = typeof FPDF.GetPageHeightF == 'function' ? FPDF.GetPageHeightF : notDefined('FPDF.GetPageHeightF');
    imports.wbg.__wbg_BitmapCreateEx_72941a8ef95faf9d = typeof FPDF.Bitmap_CreateEx == 'function' ? FPDF.Bitmap_CreateEx : notDefined('FPDF.Bitmap_CreateEx');
    imports.wbg.__wbg_BitmapDestroy_89c57cd17c6be7da = typeof FPDF.Bitmap_Destroy == 'function' ? FPDF.Bitmap_Destroy : notDefined('FPDF.Bitmap_Destroy');
    imports.wbg.__wbg_BitmapFillRect_acd3d33f0258a9d5 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        FPDF.Bitmap_FillRect(arg0, arg1, arg2, arg3, arg4, arg5 >>> 0);
    };
    imports.wbg.__wbg_BitmapGetBuffer_6798166972553842 = function(arg0, arg1) {
        var ret = FPDF.Bitmap_GetBuffer(arg1);
        var ptr0 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_BitmapGetWidth_94117d8975b8fcc1 = typeof FPDF.Bitmap_GetWidth == 'function' ? FPDF.Bitmap_GetWidth : notDefined('FPDF.Bitmap_GetWidth');
    imports.wbg.__wbg_BitmapGetHeight_a7c8b49c83b10064 = typeof FPDF.Bitmap_GetHeight == 'function' ? FPDF.Bitmap_GetHeight : notDefined('FPDF.Bitmap_GetHeight');
    imports.wbg.__wbg_BitmapGetStride_7ff14faf1269f322 = typeof FPDF.Bitmap_GetStride == 'function' ? FPDF.Bitmap_GetStride : notDefined('FPDF.Bitmap_GetStride');
    imports.wbg.__wbg_RenderPageBitmap_d9810e3fe32a323d = typeof FPDF.RenderPageBitmap == 'function' ? FPDF.RenderPageBitmap : notDefined('FPDF.RenderPageBitmap');
    imports.wbg.__wbg_DOCInitFormFillEnvironment_1c6d5fb1c6778ac0 = function(arg0, arg1, arg2, arg3) {
        var ret = FPDF.DOC_InitFormFillEnvironment(takeObject(arg0), arg1, arg2, arg3 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_DOCExitFormFillEnvironment_e79c88b99038cfac = typeof FPDF.DOC_ExitFormFillEnvironment == 'function' ? FPDF.DOC_ExitFormFillEnvironment : notDefined('FPDF.DOC_ExitFormFillEnvironment');
    imports.wbg.__wbg_SetFormFieldHighlightColor_d2de2edbb6f8878a = function(arg0, arg1, arg2) {
        FPDF.SetFormFieldHighlightColor(arg0, arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_SetFormFieldHighlightAlpha_230a34e03bf6379b = typeof FPDF.SetFormFieldHighlightAlpha == 'function' ? FPDF.SetFormFieldHighlightAlpha : notDefined('FPDF.SetFormFieldHighlightAlpha');
    imports.wbg.__wbg_FFLDraw_b60b224a0475d7e1 = typeof FPDF.FFLDraw == 'function' ? FPDF.FFLDraw : notDefined('FPDF.FFLDraw');
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

wasm_bindgen = Object.assign(init, __exports);

})();
