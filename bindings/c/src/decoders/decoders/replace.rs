use std::ffi::{c_char, CStr};
use std::ptr;
use tokenizers::normalizers::replace::Replace;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new Replace decoder.
///
/// This decoder is to be used in tandem with the Replace pre-tokenizer.
///
/// # Arguments
/// * `pattern` - The pattern to search for (regex or literal string)
/// * `content` - The content to replace the pattern with
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created Replace decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_replace_decoder_new(
    pattern: *const c_char,
    content: *const c_char,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if pattern.is_null() || content.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let pattern_str = match unsafe { CStr::from_ptr(pattern) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let content_str = match unsafe { CStr::from_ptr(content) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    // Replace decoder uses the Replace normalizer internally
    let decoder = match Replace::new(pattern_str, content_str) {
        Ok(r) => r,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let wrapper = Box::new(tokenizers::DecoderWrapper::Replace(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a Replace decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the Replace decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_replace_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_replace_decoder_new() {
        let pattern = CString::new("_").unwrap();
        let content = CString::new(" ").unwrap();
        let mut status = 0;

        let ptr = tokenizers_replace_decoder_new(pattern.as_ptr(), content.as_ptr(), &mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_replace_decoder_free(ptr);
    }

    #[test]
    fn test_replace_decoder_null_input() {
        let mut status = 0;
        let content = CString::new(" ").unwrap();

        let ptr = tokenizers_replace_decoder_new(ptr::null(), content.as_ptr(), &mut status);

        assert_eq!(status, 1);
        assert!(ptr.is_null());
    }
}
