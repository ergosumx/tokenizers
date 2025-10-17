use std::ffi::{c_char, CStr};
use std::ptr;
use tokenizers::decoders::wordpiece::WordPiece;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new WordPiece decoder.
///
/// # Arguments
/// * `prefix` - The prefix used for subwords that are not a beginning-of-word (e.g., "##")
/// * `cleanup` - Whether to cleanup tokenization artifacts (spaces before punctuation, etc.)
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created WordPiece decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_wordpiece_decoder_new(
    prefix: *const c_char,
    cleanup: bool,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if prefix.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let prefix_str = match unsafe { CStr::from_ptr(prefix) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let decoder = WordPiece::new(prefix_str.to_string(), cleanup);
    let wrapper = Box::new(tokenizers::DecoderWrapper::WordPiece(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a WordPiece decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the WordPiece decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_wordpiece_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_wordpiece_decoder_new() {
        let prefix = CString::new("##").unwrap();
        let mut status = 0;

        let ptr = tokenizers_wordpiece_decoder_new(prefix.as_ptr(), true, &mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_wordpiece_decoder_free(ptr);
    }

    #[test]
    fn test_wordpiece_decoder_null_input() {
        let mut status = 0;

        let ptr = tokenizers_wordpiece_decoder_new(ptr::null(), true, &mut status);

        assert_eq!(status, 1);
        assert!(ptr.is_null());
    }
}
