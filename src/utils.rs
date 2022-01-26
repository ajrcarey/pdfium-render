pub(crate) mod mem {
    /// Creates an empty byte buffer of the given length.
    #[inline]
    pub(crate) fn create_byte_buffer(length: usize) -> Vec<u8> {
        vec![0u8; length as usize]
    }
}

pub(crate) mod utf16le {
    use utf16string::{LittleEndian, WString};

    /// Converts the bytes in the given buffer from UTF16-LE to a standard Rust String.
    #[inline]
    pub(crate) fn get_string_from_pdfium_utf16le_bytes(buffer: Vec<u8>) -> Option<String> {
        if let Ok(string) = WString::<LittleEndian>::from_utf16(buffer) {
            // Trim any trailing nulls.

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
