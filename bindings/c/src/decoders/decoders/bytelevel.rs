use tokenizers::decoders::byte_level::ByteLevel;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new ByteLevel decoder.
///
/// This decoder is to be used in tandem with the ByteLevel pre-tokenizer.
///
/// # Arguments
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created ByteLevel decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_bytelevel_decoder_new(
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    let decoder = ByteLevel::default();
    let wrapper = Box::new(tokenizers::DecoderWrapper::ByteLevel(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a ByteLevel decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the ByteLevel decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_bytelevel_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
    if !decoder.is_null() {
        unsafe {
            let _ = Box::from_raw(decoder);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytelevel_decoder_new() {
        let mut status = 0;

        let ptr = tokenizers_bytelevel_decoder_new(&mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_bytelevel_decoder_free(ptr);
    }
}
