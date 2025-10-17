# ErgoX Fork Information

**Repository:** ergosumx/vecrax-hf-tokenizers  
**Upstream:** huggingface/tokenizers  
**Purpose:** C bindings development and cross-platform releases  
**Status:** ⚠️ Experimental - In Development

---

## Quick Facts

| Aspect | Details |
|--------|---------|
| **Fork Owner** | ErgoX |
| **Upstream** | HuggingFace Tokenizers |
| **Primary Focus** | C bindings (`bindings/c/`) |
| **Platform Support** | 7 platforms (Linux, Windows, macOS x2, iOS, Android, WASM) |
| **Current Status** | Experimental - API may change |
| **Test Coverage** | 179/180 tests passing (99.4%) |
| **FFI Functions** | 80 C functions |
| **Consumer** | ErgoX.VecraX.ML.NLP.Tokenizers (.NET library) |

---

## Why This Fork Exists

### Problem
ErgoX VecraX needs high-performance tokenization for .NET applications, but:
- Python bindings require Python runtime
- Node.js bindings require Node runtime
- Rust library requires Rust interop
- No official C bindings exist

### Solution
This fork:
1. Maintains **C bindings** (`bindings/c/`) for cross-language FFI
2. Tests C bindings against **each HuggingFace release**
3. Releases **cross-platform binaries** (7 platforms)
4. Supports **ErgoX VecraX .NET library**

---

## What We Do

### ✅ Maintained by ErgoX

- **C bindings development** (`bindings/c/`)
  - 80 FFI functions exposing core tokenization
  - Memory-safe wrappers around Rust API
  - Cross-platform compatibility
  
- **Cross-platform CI/CD**
  - GitHub Actions workflows for 7 platforms
  - Automated binary releases
  - SHA-256 checksums for security
  
- **Testing and validation**
  - Test against new HuggingFace releases
  - Platform-specific testing
  - Integration with ErgoX VecraX

- **Documentation**
  - C bindings README
  - API reference
  - Platform build guides
  - Release process

### ❌ NOT Changed by ErgoX

- Core tokenization algorithms (use upstream)
- Python bindings (use upstream)
- Node.js bindings (use upstream)
- Rust library (use upstream)
- Normalizers, pre-tokenizers, post-processors (use upstream)

---

## Release Process

### Workflow

```
┌─────────────────────────────────────┐
│ HuggingFace Releases v0.20.5        │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ ErgoX Syncs Fork                    │
│ git fetch upstream                  │
│ git merge upstream/main             │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ Test C Bindings                     │
│ cd bindings/c                       │
│ cargo test --release                │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ Update C Bindings (if needed)       │
│ - Fix breaking changes              │
│ - Add new features                  │
│ - Update docs                       │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ Release C Bindings                  │
│ git tag c-v0.2.0                    │
│ git push origin c-v0.2.0            │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ GitHub Actions (Automated)          │
│ - Build 7 platforms                 │
│ - Create release                    │
│ - Upload binaries + checksums       │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│ ErgoX VecraX .NET Library           │
│ - Download new binaries             │
│ - Update NuGet package              │
│ - Release to users                  │
└─────────────────────────────────────┘
```

### Version Tags

| Tag Format | Owner | Purpose |
|------------|-------|---------|
| `v*.*.*` | HuggingFace | Rust library (upstream) |
| `python-v*.*.*` | HuggingFace | Python bindings (upstream) |
| `node-v*.*.*` | HuggingFace | Node.js bindings (upstream) |
| **`c-v*.*.*`** | **ErgoX** | **C bindings (this fork)** ✅ |

---

## Current Status

### Versions

- **Upstream Base:** HuggingFace Tokenizers v0.20.4
- **C Bindings:** c-v0.1.0 (experimental)
- **API Stability:** ⚠️ May change during development

### Quality Metrics

- **Build:** 0 warnings, 0 errors
- **Tests:** 179/180 passing (99.4%)
- **SonarQube:** 0 issues
- **FFI Surface:** 80 functions (minimal, accurate)
- **Code Coverage:** Core paths tested

### Platform Support

| Platform | Architecture | Status | Binary Type |
|----------|-------------|--------|-------------|
| Linux | x64 | ✅ Supported | `.so` |
| Windows | x64 | ✅ Supported | `.dll` |
| macOS | x64 (Intel) | ✅ Supported | `.dylib` |
| macOS | ARM64 (Apple Silicon) | ✅ Supported | `.dylib` |
| iOS | ARM64 | ✅ Supported | `.a` (static) |
| Android | ARM64 | ✅ Supported | `.so` |
| WebAssembly | wasm32 | ✅ Supported | `.wasm` |

---

## Contributing

### Where to Contribute

**C bindings (this repository) ✅**
- Bug fixes in FFI functions
- New C API functions (exposing existing Rust features)
- Platform compatibility fixes
- Build system improvements
- Documentation updates
- Submit PRs to: `ergosumx/vecrax-hf-tokenizers`

**Core library (upstream) ⬆️**
- Tokenization algorithms
- New models (BPE, WordPiece, Unigram)
- Normalizers, pre-tokenizers, post-processors
- Python/Node.js bindings
- Submit PRs to: `huggingface/tokenizers`

**.NET wrapper (ErgoX VecraX) 🔷**
- C# API improvements
- NuGet package issues
- .NET integration
- Submit PRs to: `ergox/vecrax`

### Contribution Guidelines

See:
- [C Bindings CONTRIBUTING.md](bindings/c/CONTRIBUTING.md)
- [C Bindings RELEASE.md](bindings/c/RELEASE.md)

---

## Architecture

### Component Relationships

```
┌──────────────────────────────────────┐
│ ErgoX VecraX .NET Library            │
│ (C# wrapper, NuGet package)          │
└──────────────┬───────────────────────┘
               │ P/Invoke (DllImport)
               ↓
┌──────────────────────────────────────┐
│ C Bindings (this fork)               │
│ 80 FFI functions                     │
│ bindings/c/src/lib.rs                │
└──────────────┬───────────────────────┘
               │ Rust FFI
               ↓
┌──────────────────────────────────────┐
│ HuggingFace Tokenizers (Rust)        │
│ Core algorithms, models              │
│ (upstream - not modified)            │
└──────────────────────────────────────┘
```

### Data Flow

```
User C# Code
    ↓
ErgoX.VecraX.ML.NLP.Tokenizers (C# API)
    ↓
C Bindings (FFI layer)
    ↓
HuggingFace Tokenizers (Rust)
    ↓
Tokenization Result
    ↓
C Bindings (convert to C types)
    ↓
ErgoX.VecraX.ML.NLP.Tokenizers (C# objects)
    ↓
User C# Code
```

---

## Documentation

### For Users

- **Main README:** [README.md](README.md) - Fork purpose, installation
- **C Bindings README:** [bindings/c/README.md](bindings/c/README.md) - API, examples, platforms
- **ErgoX VecraX Docs:** [github.com/ergox/vecrax](https://github.com/ergox/vecrax) - C# wrapper

### For Contributors

- **Contributing Guide:** [bindings/c/CONTRIBUTING.md](bindings/c/CONTRIBUTING.md)
- **Release Process:** [bindings/c/RELEASE.md](bindings/c/RELEASE.md)
- **Issue Templates:** [.github/ISSUE_TEMPLATE/](bindings/c/.github/ISSUE_TEMPLATE/)
- **PR Template:** [.github/PULL_REQUEST_TEMPLATE.md](bindings/c/.github/PULL_REQUEST_TEMPLATE.md)

### For Developers

- **API Reference:** [bindings/c/src/lib.rs](bindings/c/src/lib.rs) - All FFI functions
- **GitHub Workflow:** [.github/workflows/release-c-bindings.yml](.github/workflows/release-c-bindings.yml)

---

## FAQ

### Why not contribute C bindings to upstream?

We initially planned to, but:
- HuggingFace focuses on Python/Node.js
- C bindings require different testing/release cycles
- Independent fork allows faster iteration
- We can release patches without waiting for upstream

If HuggingFace wants to adopt C bindings, we're happy to contribute!

### Will you modify core algorithms?

**No.** We only maintain the C FFI layer. All algorithm improvements go to upstream HuggingFace.

### How do you stay in sync with upstream?

We merge upstream changes regularly:
```bash
git fetch upstream
git merge upstream/main
```

Test compatibility, release new C bindings version if needed.

### What if upstream makes breaking changes?

We update the C bindings to maintain compatibility. Users of the C bindings get stable API even when Rust internals change.

### Can I use C bindings without ErgoX VecraX?

**Yes!** The C bindings work standalone. See [bindings/c/README.md](bindings/c/README.md) for usage from C, C++, or any language with C FFI.

### How stable is the C API?

**Experimental** - API may change during development. After v1.0.0, we'll follow semantic versioning strictly.

### What platforms will you support?

Currently: Linux, Windows, macOS (x2), iOS, Android, WASM

Open to adding more platforms if there's demand and CI feasibility.

---

## Contact

- **Issues (C bindings):** [GitHub Issues](https://github.com/ergosumx/vecrax-hf-tokenizers/issues)
- **Issues (core library):** [HuggingFace Issues](https://github.com/huggingface/tokenizers/issues)
- **Issues (.NET wrapper):** [ErgoX VecraX Issues](https://github.com/ergox/vecrax/issues)
- **Discussions:** [GitHub Discussions](https://github.com/huggingface/tokenizers/discussions)

---

## License

**Apache License 2.0** (same as upstream)

See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- **HuggingFace Team** - For the incredible Tokenizers library
- **Rust Community** - For amazing FFI capabilities
- **ErgoX Team** - For funding C bindings development
- **Contributors** - For testing and feedback

---

**Last Updated:** October 17, 2025  
**Fork Version:** Based on HuggingFace Tokenizers v0.20.4  
**C Bindings Version:** c-v0.1.0 (experimental)
