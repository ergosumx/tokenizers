use std::ffi::{c_char, CStr};
use std::ptr;
use tokenizers::decoders::metaspace::{Metaspace, PrependScheme};

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new Metaspace decoder.
///
/// # Arguments
/// * `replacement` - Single character replacement (e.g., '▁' U+2581). Must be exactly one char.
/// * `prepend_scheme` - Prepend behavior: 0=Always, 1=First, 2=Never
/// * `split` - Whether to split on the replacement character
/// * `status` - Output status code (0 = success, 1 = error)
///
/// # Returns
/// Pointer to the created Metaspace decoder, or null on error
#[no_mangle]
pub extern "C" fn tokenizers_metaspace_decoder_new(
    replacement: *const c_char,
    prepend_scheme: u8,
    split: bool,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if replacement.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let replacement_str = match unsafe { CStr::from_ptr(replacement) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    // Extract first character
    let replacement_char = match replacement_str.chars().next() {
        Some(c) => c,
        None => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let scheme = match prepend_scheme {
        0 => PrependScheme::Always,
        1 => PrependScheme::First,
        2 => PrependScheme::Never,
        _ => {
            set_status(status, 1);
            return ptr::null_mut();
        }
    };

    let decoder = Metaspace::new(replacement_char, scheme, split);
    let wrapper = Box::new(tokenizers::DecoderWrapper::Metaspace(decoder));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a Metaspace decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the Metaspace decoder to free
#[no_mangle]
pub extern "C" fn tokenizers_metaspace_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_metaspace_decoder_new() {
        let replacement = CString::new("▁").unwrap();
        let mut status = 0;

        let ptr = tokenizers_metaspace_decoder_new(replacement.as_ptr(), 0, true, &mut status);

        assert_eq!(status, 0);
        assert!(!ptr.is_null());

        tokenizers_metaspace_decoder_free(ptr);
    }

    #[test]
    fn test_metaspace_decoder_null_input() {
        let mut status = 0;

        let ptr = tokenizers_metaspace_decoder_new(ptr::null(), 0, true, &mut status);

        assert_eq!(status, 1);
        assert!(ptr.is_null());
    }
}
