# Contributing to HuggingFace Tokenizers C Bindings

Thank you for your interest in contributing to the C bindings for HuggingFace Tokenizers! This document provides guidelines for contributing to the C FFI layer.

## Table of Contents

- [Scope of Contributions](#scope-of-contributions)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contribution Workflow](#contribution-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Platform Support](#platform-support)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)

## Scope of Contributions

### What Goes Here (bindings/c/)

We accept contributions to the **C bindings** specifically:

✅ **Accepted Contributions:**
- Bug fixes in C FFI functions
- New C API functions (if they expose existing Rust functionality)
- Performance improvements in the FFI layer
- Build system enhancements (Cargo.toml, build scripts)
- Cross-platform compatibility fixes
- Documentation improvements
- Platform-specific optimizations
- Memory leak fixes
- Error handling improvements
- CI/CD workflow enhancements

### What Goes Elsewhere

❌ **NOT Accepted Here:**
- Core tokenization algorithm changes → [Main HuggingFace Repo](https://github.com/huggingface/tokenizers)
- Python bindings changes → `bindings/python/`
- Node.js bindings changes → `bindings/node/`
- New tokenization models → Core Rust library
- Changes to normalizers/pre-tokenizers → Core Rust library

**Rule of Thumb:** If your change modifies files outside `bindings/c/`, it likely belongs in the main HuggingFace repository.

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later (`rustup install stable`)
- **Cargo**: Comes with Rust
- **C Compiler**:
  - Linux: gcc/clang
  - Windows: MSVC (Visual Studio Build Tools)
  - macOS: Xcode Command Line Tools
- **Git**: Version control

### Fork and Clone

```bash
# Fork the repository on GitHub first

# Clone your fork
git clone https://github.com/YOUR_USERNAME/tokenizers.git
cd tokenizers/bindings/c

# Add upstream remote
git remote add upstream https://github.com/huggingface/tokenizers.git
```

## Development Setup

### Initial Build

```bash
# From bindings/c/ directory
cargo build --release
cargo test --release
```

### Verify Build Outputs

```bash
# Linux
ls -lh target/release/libtokenizers.so

# Windows
dir target\release\tokenizers.dll

# macOS
ls -lh target/release/libtokenizers.dylib
```

### Development Build (faster, includes debug symbols)

```bash
cargo build
cargo test
```

## Contribution Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

**Branch Naming:**
- `feature/` - New functionality
- `fix/` - Bug fixes
- `docs/` - Documentation only
- `perf/` - Performance improvements
- `build/` - Build system changes

### 2. Make Your Changes

- Keep changes focused and atomic
- Write clear commit messages
- Follow coding standards (see below)
- Add tests for new functionality
- Update documentation

### 3. Test Your Changes

```bash
# Run unit tests
cargo test --release

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check for platform-specific issues (if you have targets installed)
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-apple-darwin
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add support for XYZ"
```

**Commit Message Format:**
```
<type>: <short summary>

<optional body>

<optional footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `perf`: Performance improvement
- `refactor`: Code refactoring
- `test`: Test additions/improvements
- `build`: Build system changes
- `ci`: CI/CD changes

**Example:**
```
fix: prevent memory leak in tokenizers_encode

The encoding result was not being freed when an error occurred
during token ID extraction. Added proper cleanup in error paths.

Fixes #123
```

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Coding Standards

### Rust Code Style

Follow standard Rust conventions:

```rust
// ✅ Good
#[no_mangle]
pub extern "C" fn tokenizers_create(
    json: *const c_char,
    status: *mut c_int,
) -> *mut Tokenizer {
    // Validate input
    if json.is_null() {
        set_error("JSON string is null");
        if !status.is_null() {
            unsafe { *status = -1; }
        }
        return std::ptr::null_mut();
    }
    
    // Implementation...
}

// ❌ Bad (missing null checks, poor error handling)
#[no_mangle]
pub extern "C" fn tokenizers_create(json: *const c_char) -> *mut Tokenizer {
    let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
    Box::into_raw(Box::new(Tokenizer::from_str(json_str).unwrap()))
}
```

**Key Principles:**
1. **Always validate pointers** before dereferencing
2. **Never panic** in FFI functions - return error codes instead
3. **Use proper error handling** - set error messages for debugging
4. **Document memory ownership** - who allocates, who frees
5. **Follow Rust naming conventions** - snake_case for functions
6. **Add safety comments** for unsafe blocks

### Memory Management

```rust
// ✅ Proper ownership documentation
/// Creates a new tokenizer from JSON configuration.
/// 
/// # Memory Management
/// - Caller provides: `json` string (must remain valid during call)
/// - Function allocates: Tokenizer object
/// - Caller must free: Returned tokenizer using `tokenizers_free()`
#[no_mangle]
pub extern "C" fn tokenizers_create(
    json: *const c_char,
    status: *mut c_int,
) -> *mut Tokenizer {
    // ...
}

/// Frees a tokenizer created by `tokenizers_create()`.
/// 
/// # Safety
/// - `tokenizer` must be a valid pointer from `tokenizers_create()`
/// - `tokenizer` must not be used after this call
/// - Calling with NULL is safe (no-op)
#[no_mangle]
pub extern "C" fn tokenizers_free(tokenizer: *mut Tokenizer) {
    if !tokenizer.is_null() {
        unsafe {
            let _ = Box::from_raw(tokenizer);
        }
    }
}
```

### Error Handling

```rust
// ✅ Proper error handling
fn set_error(message: &str) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(message.to_string());
    });
}

#[no_mangle]
pub extern "C" fn tokenizers_get_last_error() -> *const c_char {
    LAST_ERROR.with(|e| {
        match e.borrow().as_ref() {
            Some(err) => err.as_ptr() as *const c_char,
            None => std::ptr::null(),
        }
    })
}

// Use in functions
#[no_mangle]
pub extern "C" fn tokenizers_encode(/* ... */) -> *mut Encoding {
    // ... validation ...
    
    match tokenizer.encode(sequence, add_special_tokens) {
        Ok(encoding) => Box::into_raw(Box::new(encoding)),
        Err(e) => {
            set_error(&format!("Encoding failed: {}", e));
            if !status.is_null() {
                unsafe { *status = -1; }
            }
            std::ptr::null_mut()
        }
    }
}
```

### Documentation

All public FFI functions must have:

```rust
/// Brief one-line description.
///
/// # Parameters
/// - `param1`: Description
/// - `param2`: Description
///
/// # Returns
/// Description of return value and NULL conditions.
///
/// # Memory Management
/// - Who allocates what
/// - Who is responsible for freeing
///
/// # Safety
/// - Preconditions (valid pointers, etc.)
/// - Postconditions (object state)
///
/// # Example
/// ```c
/// int status = 0;
/// void* tokenizer = tokenizers_create(json, &status);
/// if (status == 0) {
///     // Use tokenizer...
///     tokenizers_free(tokenizer);
/// }
/// ```
#[no_mangle]
pub extern "C" fn tokenizers_example(/* ... */) -> *mut T {
    // ...
}
```

## Testing

### Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    #[test]
    fn test_create_tokenizer() {
        let json = CString::new(r#"{"model":{"type":"BPE"}}"#).unwrap();
        let mut status = 0;
        
        let tokenizer = tokenizers_create(json.as_ptr(), &mut status);
        
        assert_eq!(status, 0);
        assert!(!tokenizer.is_null());
        
        tokenizers_free(tokenizer);
    }
    
    #[test]
    fn test_create_tokenizer_invalid_json() {
        let json = CString::new("invalid json").unwrap();
        let mut status = 0;
        
        let tokenizer = tokenizers_create(json.as_ptr(), &mut status);
        
        assert_eq!(status, -1);
        assert!(tokenizer.is_null());
        
        let error = tokenizers_get_last_error();
        assert!(!error.is_null());
    }
}
```

### Running Tests

```bash
# All tests
cargo test --release

# Specific test
cargo test test_create_tokenizer

# With output
cargo test -- --nocapture

# Documentation tests
cargo test --doc
```

### Integration Testing

For C code calling the library:

```c
// tests/integration_test.c
#include <assert.h>
#include <stdio.h>

extern void* tokenizers_create(const char* json, int* status);
extern void tokenizers_free(void* tokenizer);

int main() {
    int status = 0;
    const char* json = "{\"model\":{\"type\":\"BPE\"}}";
    
    void* tokenizer = tokenizers_create(json, &status);
    assert(status == 0);
    assert(tokenizer != NULL);
    
    tokenizers_free(tokenizer);
    
    printf("Integration test passed!\n");
    return 0;
}
```

Compile and run:
```bash
# Linux
gcc tests/integration_test.c -L./target/release -ltokenizers -o test
LD_LIBRARY_PATH=./target/release ./test

# macOS
clang tests/integration_test.c -L./target/release -ltokenizers -o test
DYLD_LIBRARY_PATH=./target/release ./test

# Windows
cl tests/integration_test.c /link /LIBPATH:target\release tokenizers.lib
test.exe
```

## Platform Support

### Testing Cross-Platform Builds

If you're adding platform-specific code:

```bash
# Install targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add aarch64-apple-ios
rustup target add aarch64-linux-android
rustup target add wasm32-unknown-unknown

# Test builds (may require cross-compilation setup)
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-apple-darwin
```

### Platform-Specific Code

Use feature flags and conditional compilation:

```rust
#[cfg(target_os = "linux")]
fn platform_init() {
    // Linux-specific initialization
}

#[cfg(target_os = "windows")]
fn platform_init() {
    // Windows-specific initialization
}

#[cfg(target_os = "macos")]
fn platform_init() {
    // macOS-specific initialization
}
```

## Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code compiles without warnings (`cargo build --release`)
- [ ] All tests pass (`cargo test --release`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Linter passes (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated (README.md, inline docs)
- [ ] Commit messages follow conventions
- [ ] Branch is up-to-date with main

### PR Template

When creating a PR, please include:

```markdown
## Description
Brief description of the changes.

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature causing existing functionality to break)
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Tested on Linux
- [ ] Tested on Windows
- [ ] Tested on macOS
- [ ] Tested on mobile (iOS/Android) if applicable

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings introduced
- [ ] Tests pass locally

## Related Issues
Fixes #123
Relates to #456
```

### Review Process

1. **Automated Checks**: CI must pass (build, test, lint)
2. **Code Review**: Maintainer will review within 1-2 weeks
3. **Feedback**: Address review comments
4. **Approval**: Once approved, maintainer will merge

## Reporting Issues

### Bug Reports

Use the [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md):

**Include:**
- **Platform**: OS, architecture, Rust version
- **Steps to reproduce**: Minimal example
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Error messages**: Full error output
- **Code snippet**: Minimal reproducible example

### Feature Requests

Use the [Feature Request Template](.github/ISSUE_TEMPLATE/feature_request.md):

**Include:**
- **Use case**: Why is this needed?
- **Proposed solution**: How should it work?
- **Alternatives considered**: Other approaches?
- **Additional context**: Examples, references

### Questions

Use the [Question Template](.github/ISSUE_TEMPLATE/question.md) or [Discussions](https://github.com/huggingface/tokenizers/discussions).

## Code of Conduct

### Our Standards

- **Be respectful**: Treat everyone with respect
- **Be collaborative**: Work together constructively
- **Be inclusive**: Welcome diverse perspectives
- **Be professional**: Focus on technical merits
- **Be patient**: Maintainers are volunteers

### Unacceptable Behavior

- Harassment, discrimination, or personal attacks
- Trolling, insulting comments, or inflammatory language
- Publishing others' private information
- Other conduct inappropriate in a professional setting

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

## Questions?

- **General questions**: [GitHub Discussions](https://github.com/huggingface/tokenizers/discussions)
- **C bindings questions**: [GitHub Issues](https://github.com/huggingface/tokenizers/issues) (label: `c-bindings`)
- **ErgoX VecraX (.NET wrapper)**: [ErgoX Repository](https://github.com/ergox/vecrax)

## Thank You!

Your contributions make this project better for everyone. We appreciate your time and effort! 🎉

---

**Quick Links:**
- [Main README](README.md)
- [API Reference](src/lib.rs)
- [Release Process](RELEASE.md)
- [GitHub Issues](https://github.com/huggingface/tokenizers/issues)
