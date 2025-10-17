/// Decoders module for tokenizers C bindings
///
/// This module provides C-compatible bindings for all decoder types.
/// Each decoder has its own dedicated file for better code organization.
pub mod bpe;
pub mod bytefallback;
pub mod bytelevel;
pub mod ctc;
pub mod fuse;
pub mod metaspace;
pub mod replace;
pub mod strip;
pub mod wordpiece;
