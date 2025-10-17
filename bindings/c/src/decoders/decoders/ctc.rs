use std::ffi::{c_char, CStr};
use std::ptr;
use tokenizers::decoders::ctc::CTC;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new CTC decoder.
///
/// # Arguments
/// * `pad_token` - The pad token used by CTC to delimit a new token (e.g., "<pad>")
/// * `word_delimiter_token` - The word delimiter token, will be replaced by a space (e.g., "|")
/// * `cleanup` - Whether to cleanup tokenization artifacts (spaces before punctuation, etc.)
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created CTC decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_ctc_decoder_new(
    pad_token: *const c_char,
    word_delimiter_token: *const c_char,
    cleanup: bool,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if pad_token.is_null() || word_delimiter_token.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let pad_str = match unsafe { CStr::from_ptr(pad_token) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let delimiter_str = match unsafe { CStr::from_ptr(word_delimiter_token) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let decoder = CTC::new(pad_str.to_string(), delimiter_str.to_string(), cleanup);
    let wrapper = Box::new(tokenizers::DecoderWrapper::CTC(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a CTC decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the CTC decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_ctc_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_ctc_decoder_new() {
        let pad = CString::new("<pad>").unwrap();
        let delimiter = CString::new("|").unwrap();
        let mut status = 0;

        let ptr = tokenizers_ctc_decoder_new(pad.as_ptr(), delimiter.as_ptr(), true, &mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_ctc_decoder_free(ptr);
    }

    #[test]
    fn test_ctc_decoder_null_input() {
        let mut status = 0;
        let delimiter = CString::new("|").unwrap();

        let ptr = tokenizers_ctc_decoder_new(ptr::null(), delimiter.as_ptr(), true, &mut status);

        assert_eq!(status, 1);
        assert!(ptr.is_null());
    }
}
