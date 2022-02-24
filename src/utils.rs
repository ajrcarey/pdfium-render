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
