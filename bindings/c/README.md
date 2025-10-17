# C Bindings for HuggingFace Tokenizers

[![Build Status](https://github.com/huggingface/tokenizers/workflows/Release%20C%20Bindings/badge.svg)](https://github.com/huggingface/tokenizers/actions)
[![License](https://img.shields.io/github/license/huggingface/tokenizers.svg?color=blue)](https://github.com/huggingface/tokenizers/blob/main/LICENSE)

## Overview

This directory contains **C bindings** for the [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) library, providing a Foreign Function Interface (FFI) for integrating high-performance tokenization into C, C++, C#, and other languages that can interop with C libraries.

### Purpose

The C bindings enable:
- **Cross-language Integration**: Use HuggingFace Tokenizers from any language with C FFI support (.NET, Java, Python ctypes, Go, etc.)
- **High Performance**: Direct access to Rust's blazing-fast tokenization without Python overhead
- **Production Deployments**: Embed tokenization in native applications, mobile apps, and embedded systems
- **Minimal Dependencies**: Standalone shared library with no Python or Node.js runtime required

### What's Included

The C bindings expose core tokenizer functionality:
- ✅ **Core Operations**: Create, encode, decode, free tokenizers
- ✅ **Model Types**: BPE, WordPiece, WordLevel, Unigram
- ✅ **Decoders**: BPE, ByteLevel, ByteFallback, CTC, Fuse, Metaspace, Replace, Strip, WordPiece
- ✅ **Encoding Access**: Token IDs, tokens, offsets, type IDs, attention masks, word IDs
- ✅ **Configuration**: Padding, truncation, special tokens, vocabulary management
- ✅ **Position Mapping**: Character-to-token, token-to-word conversions

### What's NOT Included

The C bindings do **NOT** expose:
- ❌ **Normalizers**: Lowercase, BERT, Unicode normalizations (handled internally when loading from JSON)
- ❌ **Pre-tokenizers**: Whitespace, BERT, ByteLevel splitting (handled internally when loading from JSON)
- ❌ **Post-processors**: Template processing, special token insertion (handled internally when loading from JSON)

**Why?** Most use cases load pre-configured tokenizers from HuggingFace Hub using `Tokenizer::from_pretrained()`. The tokenizer JSON contains serialized normalizers, pre-tokenizers, and post-processors that Rust applies internally without needing individual C functions for each component.

## Supported Platforms

Pre-built binaries are available for:

| Platform | Architecture | Library File | Status |
|----------|-------------|--------------|--------|
| **Linux** | x86_64 | `libtokenizers.so` | ✅ Supported |
| **Windows** | x86_64 | `tokenizers.dll` | ✅ Supported |
| **macOS** | x86_64 (Intel) | `libtokenizers.dylib` | ✅ Supported |
| **macOS** | ARM64 (Apple Silicon) | `libtokenizers.dylib` | ✅ Supported |
| **iOS** | ARM64 | `libtokenizers.a` | ✅ Supported |
| **Android** | ARM64 | `libtokenizers.so` | ✅ Supported |
| **WebAssembly** | wasm32 | `tokenizers.wasm` | ✅ Supported |

## Installation

### Option 1: Download Pre-built Binaries (Recommended)

Download the latest release from [GitHub Releases](https://github.com/huggingface/tokenizers/releases):

```bash
# Linux x64
wget https://github.com/huggingface/tokenizers/releases/download/c-v0.1.0/tokenizers-c-linux-x64.tar.gz
tar -xzf tokenizers-c-linux-x64.tar.gz

# Windows x64
# Download tokenizers-c-win-x64.zip and extract

# macOS ARM64 (Apple Silicon)
wget https://github.com/huggingface/tokenizers/releases/download/c-v0.1.0/tokenizers-c-osx-arm64.tar.gz
tar -xzf tokenizers-c-osx-arm64.tar.gz
```

### Option 2: Build from Source

**Prerequisites:**
- Rust 1.70+ (`rustup install stable`)
- C compiler (gcc, clang, or MSVC)
- Cargo

**Build:**
```bash
cd bindings/c
cargo build --release

# Output locations:
# Linux:   target/release/libtokenizers.so
# Windows: target/release/tokenizers.dll
# macOS:   target/release/libtokenizers.dylib
```

**Cross-compilation:**
```bash
# Install target
rustup target add aarch64-apple-darwin

# Build for target
cargo build --release --target aarch64-apple-darwin
```

## Usage Example

### C
```c
#include <stdio.h>
#include <stdint.h>

// Function declarations (from the library)
extern void* tokenizers_create(const char* json, int* status);
extern void* tokenizers_encode(void* tokenizer, const char* text, const char* pair, 
                                 int add_special_tokens, size_t* length, int* status);
extern void tokenizers_encoding_get_ids(void* encoding, uint32_t* buffer, size_t length);
extern void tokenizers_free(void* tokenizer);
extern void tokenizers_encoding_free(void* encoding);

int main() {
    int status = 0;
    
    // Create tokenizer from JSON config
    const char* json = "{\"model\":{\"type\":\"BPE\",\"vocab\":{...}}}";
    void* tokenizer = tokenizers_create(json, &status);
    
    // Encode text
    size_t length = 0;
    void* encoding = tokenizers_encode(tokenizer, "Hello, world!", NULL, 1, &length, &status);
    
    // Get token IDs
    uint32_t* ids = malloc(length * sizeof(uint32_t));
    tokenizers_encoding_get_ids(encoding, ids, length);
    
    // Print token IDs
    for (size_t i = 0; i < length; i++) {
        printf("%u ", ids[i]);
    }
    
    // Cleanup
    free(ids);
    tokenizers_encoding_free(encoding);
    tokenizers_free(tokenizer);
    
    return 0;
}
```

### C# (.NET)

See [ErgoX.VecraX.ML.NLP.Tokenizers](https://github.com/ergox/vecrax) for a complete .NET wrapper with:
- Modern C# API (using LibraryImport)
- NuGet package distribution
- Comprehensive test coverage
- Production-ready implementation

**Quick example:**
```csharp
using ErgoX.VecraX.ML.NLP.Tokenizers.HuggingFace;

var tokenizer = Tokenizer.FromPretrained("bert-base-uncased");
var encoding = tokenizer.Encode("Hello, world!");

Console.WriteLine(string.Join(", ", encoding.Ids));        // Token IDs
Console.WriteLine(string.Join(", ", encoding.Tokens));     // Tokens
```

## API Reference

### Core Functions

```c
// Create tokenizer from JSON configuration
void* tokenizers_create(const char* json, int* status);

// Free tokenizer
void tokenizers_free(void* tokenizer);

// Encode text to tokens
void* tokenizers_encode(void* tokenizer, const char* sequence, const char* pair,
                         int add_special_tokens, size_t* length, int* status);

// Decode token IDs to text
const char* tokenizers_decode(void* tokenizer, const uint32_t* ids, size_t length,
                                int skip_special_tokens, int* status);

// Free encoding
void tokenizers_encoding_free(void* encoding);

// Free string
void tokenizers_free_string(const char* str);

// Get last error message
const char* tokenizers_get_last_error();
```

### Encoding Access

```c
// Get token IDs
void tokenizers_encoding_get_ids(void* encoding, uint32_t* buffer, size_t length);

// Get tokens as strings
void tokenizers_encoding_get_tokens(void* encoding, const char** buffer, size_t length);

// Get character offsets
void tokenizers_encoding_get_offsets(void* encoding, uint32_t* buffer, size_t length);

// Get type IDs (for pair sequences)
void tokenizers_encoding_get_type_ids(void* encoding, uint32_t* buffer, size_t length);

// Get attention mask
void tokenizers_encoding_get_attention_mask(void* encoding, uint32_t* buffer, size_t length);
```

See [src/lib.rs](src/lib.rs) for the complete API surface.

## Building for Different Platforms

### Android (using NDK)

```bash
# Install Android target
rustup target add aarch64-linux-android

# Set up NDK environment (adjust path)
export ANDROID_NDK_ROOT=/path/to/android-ndk-r26d

# Configure cargo
cat >> ~/.cargo/config.toml << EOF
[target.aarch64-linux-android]
ar = "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android24-clang"
EOF

# Build
cargo build --release --target aarch64-linux-android
```

### iOS (using Xcode toolchain)

```bash
# Install iOS target
rustup target add aarch64-apple-ios

# Build
cargo build --release --target aarch64-apple-ios

# Output: target/aarch64-apple-ios/release/libtokenizers.a
```

### WebAssembly

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Build
cargo build --release --target wasm32-unknown-unknown

# Or use wasm-pack for browser compatibility
cargo install wasm-pack
wasm-pack build --target web
```

## Contributing

### Contributions to C Bindings

We welcome contributions to improve the C bindings! This includes:
- ✅ Bug fixes in existing C functions
- ✅ Performance improvements
- ✅ Better error handling
- ✅ Additional platform support
- ✅ Documentation improvements
- ✅ Build system enhancements

**Please submit PRs to this directory (`bindings/c/`) following these guidelines:**

1. **Test your changes**: Ensure builds work on all target platforms
2. **Update documentation**: Keep this README and inline comments current
3. **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
4. **Add examples**: Demonstrate new functionality
5. **Update changelog**: Document changes in CHANGELOG.md

### Contributions to Core Library

For changes to the core tokenizers library (Rust implementation, Python bindings, Node bindings), please contribute to the [main HuggingFace Tokenizers repository](https://github.com/huggingface/tokenizers).

**Scope separation:**
- **`bindings/c/`**: C FFI layer, cross-platform builds, .NET integration
- **`tokenizers/`**: Core Rust implementation, algorithms, models
- **`bindings/python/`**: Python bindings
- **`bindings/node/`**: Node.js bindings

## ErgoX VecraX Integration

These C bindings are the foundation for the **ErgoX VecraX ML.NLP.Tokenizers** library, which provides:
- High-level C# API for .NET developers
- NuGet package distribution
- Comprehensive test suite (179/180 tests passing)
- Production-ready implementation
- Native library loading for all platforms

**Learn more:** [ErgoX VecraX Repository](https://github.com/ergox/vecrax)

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](../../LICENSE) file for details.

Same license as the main HuggingFace Tokenizers library.

## Support & Issues

- **C Bindings Issues**: [GitHub Issues](https://github.com/huggingface/tokenizers/issues) (label: `c-bindings`)
- **Core Library Issues**: [HuggingFace Tokenizers Issues](https://github.com/huggingface/tokenizers/issues)
- **ErgoX VecraX Issues**: [ErgoX VecraX Issues](https://github.com/ergox/vecrax/issues)

## Acknowledgments

- **HuggingFace Team**: For the incredible Tokenizers library
- **ErgoX Team**: For C# wrapper and production deployment
- **Community Contributors**: For platform support and bug fixes

---

**Quick Links:**
- [Main Tokenizers Repo](https://github.com/huggingface/tokenizers)
- [Python Bindings](../python/)
- [Node Bindings](../node/)
- [ErgoX VecraX (.NET)](https://github.com/ergox/vecrax)
- [Release Builds](https://github.com/huggingface/tokenizers/releases)

## Features

- No Python or Node.js runtime dependencies
- Pure C ABI exports
- Compatible with .NET P/Invoke
- Supports all major tokenizer algorithms (BPE, WordPiece, Unigram)

## Building

```bash
cargo build --release
```

This produces:

- **Linux**: `libtokenizers.so`
- **Windows**: `tokenizers.dll`
- **macOS**: `libtokenizers.dylib`

## C API

### Basic Usage

```c
// Load tokenizer from file
CTokenizer* tok = tokenizer_from_file("tokenizer.json");

// Encode text
size_t len;
uint32_t* ids = tokenizer_encode(tok, "Hello world", true, &len);

// Decode IDs
char* text = tokenizer_decode(tok, ids, len, true);

// Cleanup
tokenizer_free_string(text);
tokenizer_free_ids(ids, len);
tokenizer_free(tok);
```

## .NET P/Invoke Example

```csharp
using System;
using System.Runtime.InteropServices;

public class TokenizerWrapper
{
    [DllImport("tokenizers", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr tokenizer_from_file(string path);

    [DllImport("tokenizers", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr tokenizer_encode(
        IntPtr tokenizer,
        string text,
        bool addSpecialTokens,
        out UIntPtr outLen
    );

    [DllImport("tokenizers", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr tokenizer_decode(
        IntPtr tokenizer,
        uint[] ids,
        UIntPtr len,
        bool skipSpecialTokens
    );

    [DllImport("tokenizers", CallingConvention = CallingConvention.Cdecl)]
    private static extern void tokenizer_free(IntPtr tokenizer);

    [DllImport("tokenizers", CallingConvention = CallingConvention.Cdecl)]
    private static extern void tokenizer_free_ids(IntPtr ids, UIntPtr len);

    [DllImport("tokenizers", CallingConvention = CallingConvention.Cdecl)]
    private static extern void tokenizer_free_string(IntPtr str);
}
```

## License

Apache-2.0
