use std::ffi::{c_char, CStr};
use std::ptr;
use tokenizers::decoders::strip::Strip;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new Strip decoder.
///
/// Strips n left characters or n right characters of each token.
///
/// # Arguments
/// * `content` - The character to strip
/// * `left` - Number of characters to strip from the left
/// * `right` - Number of characters to strip from the right
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created Strip decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_strip_decoder_new(
    content: *const c_char,
    left: usize,
    right: usize,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if content.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let content_str = match unsafe { CStr::from_ptr(content) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    // Extract first character (default to space if empty)
    let content_char = content_str.chars().next().unwrap_or(' ');

    let decoder = Strip::new(content_char, left, right);
    let wrapper = Box::new(tokenizers::DecoderWrapper::Strip(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a Strip decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the Strip decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_strip_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_strip_decoder_new() {
        let content = CString::new(" ").unwrap();
        let mut status = 0;

        let ptr = tokenizers_strip_decoder_new(content.as_ptr(), 1, 0, &mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_strip_decoder_free(ptr);
    }

    #[test]
    fn test_strip_decoder_null_input() {
        let mut status = 0;

        let ptr = tokenizers_strip_decoder_new(ptr::null(), 1, 0, &mut status);

        assert_eq!(status, 1);
        assert!(ptr.is_null());
    }
}
