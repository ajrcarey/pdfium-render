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

    #[cfg(not(target_arch = "wasm32"))]
    /// Converts the bytes in the given buffer from UTF16-LE to a standard Rust String.
    pub(crate) fn get_string_from_pdfium_utf16le_bytes(buffer: Vec<u8>) -> Option<String> {
        get_string_from_pdfium_utf16le_bytes_native(buffer)
    }

    #[inline]
    pub fn get_string_from_pdfium_utf16le_bytes_native(buffer: Vec<u8>) -> Option<String> {
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

    #[cfg(target_arch = "wasm32")]
    use web_sys::TextDecoder;

    #[cfg(target_arch = "wasm32")]
    /// Converts the bytes in the given buffer from UTF16-LE to a standard Rust String.
    ///
    /// The browser's native TextDecoder functionality is used to handle the conversion,
    /// if available.
    pub(crate) fn get_string_from_pdfium_utf16le_bytes(mut buffer: Vec<u8>) -> Option<String> {
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

        // The TextDecoder functionality is not available, or signalled an error.
        // Revert to native conversion instead.

        get_string_from_pdfium_utf16le_bytes_native(buffer)
    }
}
