pub(crate) mod mem {
    /// Creates an empty byte buffer of the given length.
    #[inline]
    pub(crate) fn create_byte_buffer(length: usize) -> Vec<u8> {
        create_sized_buffer::<u8>(length)
    }

    /// Creates an empty buffer of the given type with the given capacity.
    /// The contents of the buffer will be uninitialized.
    #[inline]
    #[allow(clippy::uninit_vec)]
    pub(crate) fn create_sized_buffer<T>(capacity: usize) -> Vec<T> {
        let mut buffer = Vec::<T>::with_capacity(capacity);

        unsafe {
            buffer.set_len(capacity);
        }

        buffer
    }
}

pub(crate) mod utf16le {
    use utf16string::{LittleEndian, WString};

    /// Converts the given Rust &str into an UTF16-LE encoded byte buffer.
    #[inline]
    pub(crate) fn get_pdfium_utf16le_bytes_from_str(str: &str) -> Vec<u8> {
        let mut bytes = WString::<LittleEndian>::from(str).into_bytes();

        // Pdfium appears to expect C-style null termination. Since we are dealing with
        // wide (16-bit) characters, we need two bytes of nulls.

        bytes.push(0);
        bytes.push(0);

        bytes
    }

    /// Converts the bytes in the given buffer from UTF16-LE to a standard Rust String.
    #[allow(unused_mut)] // The buffer must be mutable when compiling to WASM.
    pub(crate) fn get_string_from_pdfium_utf16le_bytes(mut buffer: Vec<u8>) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::TextDecoder;

            // Attempt to perform the conversion using the browser's native TextDecoder
            // functionality; if that doesn't work, fall back to using the same WString method
            // used in non-WASM builds.

            if let Ok(decoder) = TextDecoder::new_with_label("utf-16le") {
                if let Ok(result) = decoder.decode_with_u8_array(&mut buffer) {
                    let result = result.trim_end_matches(char::from(0));

                    if !result.is_empty() {
                        return Some(result.to_owned());
                    } else {
                        return None;
                    }
                }
            }
        }

        if let Ok(string) = WString::<LittleEndian>::from_utf16(buffer) {
            // Trim any trailing nulls. UTF-16LE strings returned from Pdfium are generally
            // terminated by two null bytes.

            let result = string.to_utf8().trim_end_matches(char::from(0)).to_owned();

            if !result.is_empty() {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub(crate) mod files {
    use crate::bindgen::{FPDF_FILEACCESS, FPDF_FILEWRITE};
    use std::ffi::c_void;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::ops::Deref;
    use std::os::raw::{c_int, c_uchar, c_ulong};
    use std::ptr::null_mut;
    use std::slice;

    // These functions return wrapped versions of Pdfium's file access structs. They are used
    // in callback functions to connect Pdfium's file access operations to an underlying
    // Rust reader or writer.

    // C++ examples such as https://github.com/lydstory/pdfium_example/blob/master/pdfium_rect.cpp
    // demonstrate that the intention is for the implementor to use C++'s "struct inheritance"
    // feature to derive their own struct from FPDF_FILEACCESS or FPDF_FILEWRITE that contains
    // whatever additional custom data they wish.

    // Since Rust does not provide struct inheritance, we define new structs with the same field
    // layout as Pdfium's PDF_FILEACCESS and PDF_FILEWRITE, adding to each a custom field
    // that stores the user-provided Rust reader or writer. The callback function invoked by Pdfium
    // can then access the relevant Rust reader or writer directly in order to fulfil Pdfium's
    // file access request.

    // The writer will be used immediately and synchronously. It can be dropped as soon as it
    // is used. The reader, however, can be used repeatedly throughout the lifetime of a document
    // as Pdfium streams in data from the underlying provider on an as-needed basis. This has two
    // important ramifications: (a) we must connect the lifetime of the reader to the PdfDocument
    // we create, so that the reader is not dropped before the document is closed with a call to
    // FPDF_CloseDocument(); and (b) we must box the reader, so that transferring it into the
    // PdfDocument will not change its memory location. Breaking either of these two invariants
    // will result in a segfault.

    /// Returns a wrapped Pdfium `FPDF_FILEACCESS` struct that uses the given reader as an
    /// input source for Pdfium's file access callback function.
    ///
    /// Because Pdfium must know the total content length in advance prior to loading
    /// any portion of it, the given reader must implement the `Seek` trait as well as
    /// the `Read` trait.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    // This function is never used when compiling to WASM.
    pub(crate) fn get_pdfium_file_accessor_from_reader<R: Read + Seek + 'static>(
        mut reader: R,
    ) -> Box<FpdfFileAccessExt> {
        let content_length = reader.seek(SeekFrom::End(0)).unwrap_or(0) as c_ulong;

        let mut result = Box::new(FpdfFileAccessExt {
            content_length,
            get_block: Some(read_block_from_callback),
            file_access_ptr: null_mut(), // We'll set this value in just a moment.
            reader: Box::new(reader),
        });

        // Update the struct with a pointer to its own memory location. This pointer will
        // be passed to the callback function that Pdfium invokes, allowing that callback to
        // retrieve this FpdfFileAccessExt struct and, from there, the boxed Rust reader.

        let file_access_ptr: *const FpdfFileAccessExt = result.deref();

        result.as_mut().file_access_ptr = file_access_ptr as *mut FpdfFileAccessExt;

        result
    }

    trait PdfiumDocumentReader: Read + Seek {
        // A tiny trait that lets us perform type-erasure on the user-provided Rust reader.
        // This means FpdfFileAccessExt does not need to carry a generic parameter, which in turn
        // means that any PdfDocument containing a bound FpdfFileAccessExt does not need to carry
        // a generic parameter either.
    }

    impl<R: Read + Seek> PdfiumDocumentReader for R {}

    #[repr(C)]
    pub(crate) struct FpdfFileAccessExt {
        // An extension of Pdfium's FPDF_FILEACCESS struct that adds an extra field to carry the
        // user-provided Rust reader.
        content_length: c_ulong,
        get_block: Option<
            unsafe extern "C" fn(
                reader_ptr: *mut FpdfFileAccessExt,
                position: c_ulong,
                buf: *mut c_uchar,
                size: c_ulong,
            ) -> c_int,
        >,
        file_access_ptr: *mut FpdfFileAccessExt,
        reader: Box<dyn PdfiumDocumentReader>, // Type-erased equivalent of <R: Read + Seek>
    }

    impl FpdfFileAccessExt {
        /// Returns an `FPDF_FILEACCESS` pointer suitable for passing to `FPDF_LoadCustomDocument()`.
        #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
        // This function is never used when compiling to WASM.
        #[cfg_attr(feature = "thread_safe", allow(dead_code))]
        // This function is never used when compiling with the thread_safe feature enabled.
        #[inline]
        pub(crate) fn as_fpdf_file_access_mut_ptr(&mut self) -> &mut FPDF_FILEACCESS {
            unsafe { &mut *(self as *mut FpdfFileAccessExt as *mut FPDF_FILEACCESS) }
        }
    }

    // The callback function invoked by Pdfium.
    pub(crate) extern "C" fn read_block_from_callback(
        file_access_ptr: *mut FpdfFileAccessExt,
        position: c_ulong,
        buf: *mut c_uchar,
        size: c_ulong,
    ) -> c_int {
        unsafe {
            let reader = (*file_access_ptr).reader.as_mut();

            let result = match reader.seek(SeekFrom::Start(position as u64)) {
                Ok(_) => reader
                    .read(slice::from_raw_parts_mut(buf, size as usize))
                    .unwrap_or(0),
                Err(_) => 0,
            };

            result as c_int
        }
    }

    /// Returns a wrapped Pdfium `FPDF_FILEWRITE` struct that uses the given writer as an
    /// output source for Pdfium's file writing callback function.
    pub(crate) fn get_pdfium_file_writer_from_writer<W: Write + 'static>(
        writer: &mut W,
    ) -> FpdfFileWriteExt {
        FpdfFileWriteExt {
            version: 1,
            write_block: Some(write_block_from_callback),
            writer,
        }
    }

    trait PdfiumDocumentWriter: Write {
        // A tiny trait that lets us perform type-erasure on the user-provided Rust writer.
        // This means FpdfFileWriteExt does not need to carry a generic parameter, which simplifies
        // callback overloading in the WASM bindings implementation.

        // Additionally, since Pdfium's save operations are synchronous and immediate, we do
        // not need to take ownership of the user-provided Rust writer; a temporary mutable
        // reference is sufficient.
    }

    impl<W: Write> PdfiumDocumentWriter for W {}

    #[repr(C)]
    pub(crate) struct FpdfFileWriteExt<'a> {
        // An extension of Pdfium's FPDF_FILEWRITE struct that adds an extra field to carry the
        // user-provided Rust writer.
        version: c_int,
        write_block: Option<
            unsafe extern "C" fn(
                file_write_ext_ptr: *mut FpdfFileWriteExt,
                buf: *const c_void,
                size: c_ulong,
            ) -> c_int,
        >,
        writer: &'a mut dyn PdfiumDocumentWriter, // Type-erased equivalent of <W: Write>
    }

    impl<'a> FpdfFileWriteExt<'a> {
        /// Returns an `FPDF_FILEWRITE` pointer suitable for passing to `FPDF_SaveAsCopy()`
        /// or `FPDF_SaveWithVersion()`.
        #[inline]
        pub(crate) fn as_fpdf_file_write_mut_ptr(&mut self) -> &mut FPDF_FILEWRITE {
            unsafe { &mut *(self as *mut FpdfFileWriteExt as *mut FPDF_FILEWRITE) }
        }

        /// Flushes the buffer of the underlying Rust writer.
        #[inline]
        pub(crate) fn flush(&mut self) -> std::io::Result<()> {
            self.writer.flush()
        }
    }

    // The callback function invoked by Pdfium.
    pub(crate) extern "C" fn write_block_from_callback(
        file_write_ext_ptr: *mut FpdfFileWriteExt,
        buf: *const c_void,
        size: c_ulong,
    ) -> c_int {
        let result = unsafe {
            match (*file_write_ext_ptr)
                .writer
                .write_all(slice::from_raw_parts(buf as *const u8, size as usize))
            {
                Ok(()) => 1,
                Err(_) => 0,
            }
        };

        result
    }
}

#[cfg(test)]
pub(crate) mod tests {
    // Provides a function that binds to the correct Pdfium configuration during unit tests,
    // depending on selected crate features.

    use crate::pdfium::Pdfium;

    #[inline]
    #[cfg(feature = "static")]
    pub(crate) fn tests_bind_to_pdfium() -> Pdfium {
        Pdfium::default()
    }

    #[inline]
    #[cfg(not(feature = "static"))]
    pub(crate) fn tests_bind_to_pdfium() -> Pdfium {
        Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .unwrap(),
        )
    }
}
