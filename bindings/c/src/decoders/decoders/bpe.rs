use std::ffi::{c_char, CStr};
use std::ptr;
use tokenizers::decoders::bpe::BPEDecoder;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new BPE decoder.
///
/// # Arguments
/// * `suffix` - The suffix that characterizes an end-of-word (e.g., "</w>"). Will be replaced by whitespace during decoding.
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created BPE decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_bpe_decoder_new(
    suffix: *const c_char,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if suffix.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let suffix_str = match unsafe { CStr::from_ptr(suffix) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let decoder = BPEDecoder::new(suffix_str.to_string());
    let wrapper = Box::new(tokenizers::DecoderWrapper::BPE(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a BPE decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the BPE decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_bpe_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
    if !decoder.is_null() {
        unsafe {
            let _ = Box::from_raw(decoder);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_bpe_decoder_new() {
        let suffix = CString::new("</w>").unwrap();
        let mut status = 0;

        let ptr = tokenizers_bpe_decoder_new(suffix.as_ptr(), &mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_bpe_decoder_free(ptr);
    }

    #[test]
    fn test_bpe_decoder_null_input() {
        let mut status = 0;

        let ptr = tokenizers_bpe_decoder_new(ptr::null(), &mut status);

        assert_eq!(status, 1);
        assert!(ptr.is_null());
    }
}
