use std::ptr;

use tokenizers::decoders::sequence::Sequence;

/// Helper to set status code
fn set_status(status: *mut i32, code: i32) {
    if !status.is_null() {
        unsafe {
            *status = code;
        }
    }
}

/// Creates a new Sequence decoder.
///
/// This decoder allows chaining multiple decoders.
///
/// # Arguments
/// * `decoders` - Pointer to an array of decoder handles.
/// * `length` - Number of decoders in the array.
/// * `status` - Output status code (0 = success, 1 = error).
///
/// # Returns
/// Pointer to the created Sequence decoder, or null on error.
#[no_mangle]
pub extern "C" fn tokenizers_sequence_decoder_new(
    decoders: *const *mut tokenizers::DecoderWrapper,
    length: usize,
    status: *mut i32,
) -> *mut tokenizers::DecoderWrapper {
    if length == 0 || decoders.is_null() {
        set_status(status, 1);
        return ptr::null_mut();
    }

    let decoder_ptrs = unsafe { std::slice::from_raw_parts(decoders, length) };

    let mut sequence_decoders = Vec::with_capacity(length);
    for decoder_ptr in decoder_ptrs {
        if decoder_ptr.is_null() {
            set_status(status, 1);
            return ptr::null_mut();
        }

        let decoder_ref = unsafe { &**decoder_ptr };
        sequence_decoders.push(decoder_ref.clone());
    }

    let sequence = Sequence::new(sequence_decoders);
    let wrapper = Box::new(tokenizers::DecoderWrapper::Sequence(sequence));
    set_status(status, 0);
    Box::into_raw(wrapper)
}

/// Frees a Sequence decoder.
///
/// # Arguments
/// * `decoder` - Pointer to the Sequence decoder to free.
#[no_mangle]
pub extern "C" fn tokenizers_sequence_decoder_free(decoder: *mut tokenizers::DecoderWrapper) {
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
    fn test_sequence_decoder_new() {
        let mut status = 0;

        let replacement = CString::new("▁").unwrap();

        let metaspace = crate::decoders::metaspace::tokenizers_metaspace_decoder_new(
            replacement.as_ptr(),
            0,
            true,
            &mut status,
        );
        assert_eq!(status, 0);
        assert!(!metaspace.is_null());

        let fuse = crate::decoders::fuse::tokenizers_fuse_decoder_new(&mut status);
        assert_eq!(status, 0);
        assert!(!fuse.is_null());

        let decoders = [metaspace, fuse];
        let sequence = tokenizers_sequence_decoder_new(decoders.as_ptr(), decoders.len(), &mut status);

        assert_eq!(status, 0);
        assert!(!sequence.is_null());

        tokenizers_sequence_decoder_free(sequence);
        crate::decoders::metaspace::tokenizers_metaspace_decoder_free(metaspace);
        crate::decoders::fuse::tokenizers_fuse_decoder_free(fuse);
    }

    #[test]
    fn test_sequence_decoder_with_null_entry_fails() {
        let mut status = 0;
        let decoders = [ptr::null_mut()];
        let sequence = tokenizers_sequence_decoder_new(decoders.as_ptr(), decoders.len(), &mut status);

        assert_eq!(status, 1);
        assert!(sequence.is_null());
    }

    #[test]
    fn test_sequence_decoder_with_empty_slice_fails() {
        let mut status = 0;
        let sequence = tokenizers_sequence_decoder_new(ptr::null(), 0, &mut status);

        assert_eq!(status, 1);
        assert!(sequence.is_null());
    }
}
