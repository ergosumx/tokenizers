use tokenizers::decoders::fuse::Fuse;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new Fuse decoder.
///
/// Fuse simply fuses every token into a single string.
/// This is the last step of decoding, this decoder exists only if
/// there is need to add other decoders *after* the fusion.
///
/// # Arguments
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created Fuse decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_fuse_decoder_new(status: *mut i32) -> *mut tokenizers::DecoderWrapper {
    let decoder = Fuse::default();
    let wrapper = Box::new(tokenizers::DecoderWrapper::Fuse(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a Fuse decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the Fuse decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_fuse_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_fuse_decoder_new() {
        let mut status = 0;

        let ptr = tokenizers_fuse_decoder_new(&mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_fuse_decoder_free(ptr);
    }
}
