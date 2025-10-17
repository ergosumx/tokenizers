use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::{CEncoding, set_last_error, set_status};

/// Merge multiple encodings into a single encoding
/// 
/// Note: This function creates a new merged encoding. The C API uses a flattened
/// representation (CEncoding) which doesn't preserve all Encoding internals needed
/// for efficient merging. Use the Rust API directly for complex operations.
#[no_mangle]
pub extern "C" fn tokenizers_encoding_merge(
    _encodings: *const *const CEncoding,
    _count: usize,
    _growing_offsets: bool,
    _len_ptr: *mut usize,
    status: *mut c_int,
) -> *mut CEncoding {
    set_last_error("tokenizers_encoding_merge: not supported in C API - use Rust tokenizer.encode_batch() instead");
    set_status(status, 1);
    ptr::null_mut()
}

/// Pad an encoding to the specified length
///
/// Note: CEncoding uses a flattened representation. Padding requires reconstructing
/// the full Encoding object. For padding, use tokenizer.enable_padding() before encoding.
#[no_mangle]
pub extern "C" fn tokenizers_encoding_pad(
    _encoding: *mut CEncoding,
    _target_length: usize,
    _pad_id: u32,
    _pad_type_id: u32,
    _pad_token: *const c_char,
    _direction: c_int,
    status: *mut c_int,
) -> c_int {
    set_last_error("tokenizers_encoding_pad: not supported in C API - use tokenizer.enable_padding() before encoding");
    set_status(status, 1);
    0
}

/// Truncate an encoding to the specified maximum length
///
/// Note: CEncoding uses a flattened representation. Truncation requires reconstructing
/// the full Encoding object. For truncation, use tokenizer.with_truncation() before encoding.
#[no_mangle]
pub extern "C" fn tokenizers_encoding_truncate(
    _encoding: *mut CEncoding,
    _max_length: usize,
    _stride: usize,
    _direction: c_int,
    status: *mut c_int,
) -> c_int {
    set_last_error("tokenizers_encoding_truncate: not supported in C API - use tokenizer.with_truncation() before encoding");
    set_status(status, 1);
    0
}

/// Set the sequence ID for all tokens in the encoding
///
/// Note: CEncoding uses a flattened representation. This operation requires reconstructing
/// the full Encoding object. Sequence IDs are set during encoding.
#[no_mangle]
pub extern "C" fn tokenizers_encoding_set_sequence_id(
    _encoding: *mut CEncoding,
    _sequence_id: usize,
    status: *mut c_int,
) -> c_int {
    set_last_error("tokenizers_encoding_set_sequence_id: not supported in C API - sequence IDs are set during encoding");
    set_status(status, 1);
    0
}

/// Get the token range for a specific word in a sequence
#[no_mangle]
pub extern "C" fn tokenizers_encoding_word_to_tokens(
    encoding: *const CEncoding,
    word_index: u32,
    sequence_index: usize,
    start_ptr: *mut usize,
    end_ptr: *mut usize,
    status: *mut c_int,
) -> bool {
    if encoding.is_null() || start_ptr.is_null() || end_ptr.is_null() {
        set_last_error("tokenizers_encoding_word_to_tokens received null pointer");
        set_status(status, 1);
        return false;
    }

    let c_encoding = unsafe { &*encoding };
    
    // Find the first and last token with matching word_id and sequence_id
    let mut first_token: Option<usize> = None;
    let mut last_token: Option<usize> = None;
    
    for (i, (word_id_opt, seq_id_opt)) in c_encoding.word_ids.iter().zip(c_encoding.sequence_ids.iter()).enumerate() {
        if let (Some(wid), Some(sid)) = (word_id_opt, seq_id_opt) {
            if *wid == word_index && *sid == sequence_index {
                if first_token.is_none() {
                    first_token = Some(i);
                }
                last_token = Some(i);
            }
        }
    }
    
    match (first_token, last_token) {
        (Some(start), Some(end)) => {
            unsafe {
                *start_ptr = start;
                *end_ptr = end + 1; // Rust uses exclusive end
            }
            set_status(status, 0);
            true
        }
        _ => {
            set_status(status, 0);
            false
        }
    }
}

/// Get the character offsets for a specific word in a sequence
#[no_mangle]
pub extern "C" fn tokenizers_encoding_word_to_chars(
    encoding: *const CEncoding,
    word_index: u32,
    sequence_index: usize,
    start_ptr: *mut usize,
    end_ptr: *mut usize,
    status: *mut c_int,
) -> bool {
    if encoding.is_null() || start_ptr.is_null() || end_ptr.is_null() {
        set_last_error("tokenizers_encoding_word_to_chars received null pointer");
        set_status(status, 1);
        return false;
    }

    let c_encoding = unsafe { &*encoding };
    
    // Find the first and last token with matching word_id and sequence_id
    let mut first_offset: Option<(u32, u32)> = None;
    let mut last_offset: Option<(u32, u32)> = None;
    
    for (i, (word_id_opt, seq_id_opt)) in c_encoding.word_ids.iter().zip(c_encoding.sequence_ids.iter()).enumerate() {
        if let (Some(wid), Some(sid)) = (word_id_opt, seq_id_opt) {
            if *wid == word_index && *sid == sequence_index {
                if first_offset.is_none() {
                    first_offset = c_encoding.offsets.get(i).copied();
                }
                last_offset = c_encoding.offsets.get(i).copied();
            }
        }
    }
    
    match (first_offset, last_offset) {
        (Some((start, _)), Some((_, end))) => {
            unsafe {
                *start_ptr = start as usize;
                *end_ptr = end as usize;
            }
            set_status(status, 0);
            true
        }
        _ => {
            set_status(status, 0);
            false
        }
    }
}

/// Get the sequence index for a specific token
#[no_mangle]
pub extern "C" fn tokenizers_encoding_token_to_sequence(
    encoding: *const CEncoding,
    token_index: usize,
    status: *mut c_int,
) -> i32 {
    if encoding.is_null() {
        set_last_error("tokenizers_encoding_token_to_sequence received null pointer");
        set_status(status, 1);
        return -1;
    }

    let c_encoding = unsafe { &*encoding };
    match c_encoding.sequence_ids.get(token_index) {
        Some(Some(seq_id)) => {
            set_status(status, 0);
            *seq_id as i32
        }
        _ => {
            set_status(status, 0);
            -1
        }
    }
}

/// Get the character offsets for a specific token
#[no_mangle]
pub extern "C" fn tokenizers_encoding_token_to_chars(
    encoding: *const CEncoding,
    token_index: usize,
    sequence_ptr: *mut usize,
    start_ptr: *mut usize,
    end_ptr: *mut usize,
    status: *mut c_int,
) -> bool {
    if encoding.is_null() || sequence_ptr.is_null() || start_ptr.is_null() || end_ptr.is_null() {
        set_last_error("tokenizers_encoding_token_to_chars received null pointer");
        set_status(status, 1);
        return false;
    }

    let c_encoding = unsafe { &*encoding };
    
    match (c_encoding.sequence_ids.get(token_index), c_encoding.offsets.get(token_index)) {
        (Some(Some(seq_id)), Some((start, end))) => {
            unsafe {
                *sequence_ptr = *seq_id;
                *start_ptr = *start as usize;
                *end_ptr = *end as usize;
            }
            set_status(status, 0);
            true
        }
        _ => {
            set_status(status, 0);
            false
        }
    }
}

/// Get the word index for a specific token
#[no_mangle]
pub extern "C" fn tokenizers_encoding_token_to_word(
    encoding: *const CEncoding,
    token_index: usize,
    sequence_ptr: *mut usize,
    word_ptr: *mut u32,
    status: *mut c_int,
) -> bool {
    if encoding.is_null() || sequence_ptr.is_null() || word_ptr.is_null() {
        set_last_error("tokenizers_encoding_token_to_word received null pointer");
        set_status(status, 1);
        return false;
    }

    let c_encoding = unsafe { &*encoding };
    
    match (c_encoding.sequence_ids.get(token_index), c_encoding.word_ids.get(token_index)) {
        (Some(Some(seq_id)), Some(Some(word_id))) => {
            unsafe {
                *sequence_ptr = *seq_id;
                *word_ptr = *word_id;
            }
            set_status(status, 0);
            true
        }
        _ => {
            set_status(status, 0);
            false
        }
    }
}

/// Get the token index for a character position
#[no_mangle]
pub extern "C" fn tokenizers_encoding_char_to_token(
    encoding: *const CEncoding,
    char_pos: usize,
    sequence_index: usize,
    status: *mut c_int,
) -> i32 {
    if encoding.is_null() {
        set_last_error("tokenizers_encoding_char_to_token received null pointer");
        set_status(status, 1);
        return -1;
    }

    let c_encoding = unsafe { &*encoding };
    
    // Find token containing this character position in the specified sequence
    for (i, (seq_id_opt, (start, end))) in c_encoding.sequence_ids.iter().zip(c_encoding.offsets.iter()).enumerate() {
        if let Some(sid) = seq_id_opt {
            if *sid == sequence_index {
                let start_pos = *start as usize;
                let end_pos = *end as usize;
                if char_pos >= start_pos && char_pos < end_pos {
                    set_status(status, 0);
                    return i as i32;
                }
            }
        }
    }
    
    set_status(status, 0);
    -1
}

/// Get the word index for a character position
#[no_mangle]
pub extern "C" fn tokenizers_encoding_char_to_word(
    encoding: *const CEncoding,
    char_pos: usize,
    sequence_index: usize,
    status: *mut c_int,
) -> i32 {
    if encoding.is_null() {
        set_last_error("tokenizers_encoding_char_to_word received null pointer");
        set_status(status, 1);
        return -1;
    }

    let c_encoding = unsafe { &*encoding };
    
    // Find token containing this character position in the specified sequence, then get its word ID
    for (seq_id_opt, word_id_opt, (start, end)) in c_encoding.sequence_ids.iter()
        .zip(c_encoding.word_ids.iter())
        .zip(c_encoding.offsets.iter())
        .map(|((s, w), o)| (s, w, o)) {
        if let Some(sid) = seq_id_opt {
            if *sid == sequence_index {
                let start_pos = *start as usize;
                let end_pos = *end as usize;
                if char_pos >= start_pos && char_pos < end_pos {
                    if let Some(wid) = word_id_opt {
                        set_status(status, 0);
                        return *wid as i32;
                    }
                }
            }
        }
    }
    
    set_status(status, 0);
    -1
}
