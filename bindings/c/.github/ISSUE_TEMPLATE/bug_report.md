---
name: Bug Report
about: Report a bug in the C bindings
title: '[BUG] '
labels: 'bug, c-bindings'
assignees: ''
---

## Bug Description

**Clear and concise description of the bug.**

## Environment

- **Platform**: (e.g., Linux x64, Windows x64, macOS ARM64, iOS, Android, WASM)
- **OS Version**: (e.g., Ubuntu 22.04, Windows 11, macOS 14.2)
- **Rust Version**: (output of `rustc --version`)
- **C Bindings Version**: (e.g., c-v0.2.0)
- **Compiler**: (e.g., gcc 11.3, clang 15, MSVC 2022)

## Steps to Reproduce

1. Step 1
2. Step 2
3. Step 3
4. See error

## Minimal Reproducible Example

```c
// C code demonstrating the issue
#include <stdio.h>

extern void* tokenizers_create(const char* json, int* status);

int main() {
    // Your minimal example here
    return 0;
}
```

Or Rust code:

```rust
// Rust code calling the FFI function
use std::ffi::CString;

fn main() {
    // Your minimal example here
}
```

## Expected Behavior

**What you expected to happen.**

## Actual Behavior

**What actually happened.**

## Error Output

```
Paste full error messages, stack traces, or crash logs here
```

## Additional Context

**Any other relevant information:**
- Does this happen consistently or intermittently?
- Did this work in a previous version?
- Memory usage at time of error?
- Valgrind/AddressSanitizer output?

## Possible Solution

(Optional) If you have ideas on what might be causing the issue or how to fix it.

## Checklist

- [ ] I have searched existing issues to ensure this is not a duplicate
- [ ] I have provided a minimal reproducible example
- [ ] I have included full error output
- [ ] I have tested with the latest release
- [ ] This is specific to the C bindings (not core library behavior)
