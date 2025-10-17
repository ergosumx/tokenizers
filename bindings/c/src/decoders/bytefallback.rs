use tokenizers::decoders::byte_fallback::ByteFallback;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new ByteFallback decoder.
///
/// ByteFallback is a simple trick which converts tokens looking like `<0x61>`
/// to pure bytes, and attempts to make them into a string. If the tokens
/// cannot be decoded you will get � instead for each inconvertible byte token.
///
/// # Arguments
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created ByteFallback decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_bytefallback_decoder_new(
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    let decoder = ByteFallback::default();
    let wrapper = Box::new(tokenizers::DecoderWrapper::ByteFallback(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a ByteFallback decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the ByteFallback decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_bytefallback_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_bytefallback_decoder_new() {
        let mut status = 0;

        let ptr = tokenizers_bytefallback_decoder_new(&mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_bytefallback_decoder_free(ptr);
    }
}
