<p align="center">
    <br>
    <img src="https://huggingface.co/landing/assets/tokenizers/tokenizers-logo.png" width="600"/>
    <br>
<p>

# ErgoX Fork - C Bindings Development

> **⚠️ EXPERIMENTAL - IN DEVELOPMENT**
> 
> This is an ErgoX-maintained fork of [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) focused on **C bindings development and cross-platform releases** for the ErgoX VecraX ML.NLP.Tokenizers library.
>
> **Status:** Development phase - C bindings API may change between releases.

## Purpose

This fork exists to:

1. **Develop and maintain C bindings** (`bindings/c/`) for HuggingFace Tokenizers
2. **Test C bindings** against new HuggingFace Tokenizers releases
3. **Release cross-platform C bindings** for 7 platforms (Linux, Windows, macOS x2, iOS, Android, WASM)
4. **Support ErgoX VecraX** .NET tokenization library with native bindings

## Workflow

```
HuggingFace Tokenizers Release (v0.X.0)
    ↓
ErgoX Fork: Update submodule / merge upstream changes
    ↓
Test C bindings against new version
    ↓
Update C bindings if needed (bug fixes, new features)
    ↓
Release C bindings (c-v0.Y.0)
    ↓
ErgoX VecraX .NET library updates
```

## What This Fork Contains

- ✅ **All upstream HuggingFace Tokenizers code** (Rust, Python, Node.js)
- ✅ **C bindings** (`bindings/c/`) - maintained by ErgoX
- ✅ **Cross-platform CI/CD** for C bindings releases
- ✅ **80 C FFI functions** exposing core tokenization functionality

## What This Fork Does NOT Contain

- ❌ Changes to core tokenization algorithms
- ❌ Python or Node.js binding modifications
- ❌ New tokenization models or trainers

**All core library contributions should go to:** [huggingface/tokenizers](https://github.com/huggingface/tokenizers)

---

<p align="center">
    <img alt="Build" src="https://github.com/huggingface/tokenizers/workflows/Rust/badge.svg">
    <a href="https://github.com/huggingface/tokenizers/blob/main/LICENSE">
        <img alt="GitHub" src="https://img.shields.io/github/license/huggingface/tokenizers.svg?color=blue&cachedrop">
    </a>
    <a href="https://pepy.tech/project/tokenizers">
        <img src="https://pepy.tech/badge/tokenizers/week" />
    </a>
</p>

Provides an implementation of today's most used tokenizers, with a focus on performance and
versatility.

## Main features:

 - Train new vocabularies and tokenize, using today's most used tokenizers.
 - Extremely fast (both training and tokenization), thanks to the Rust implementation. Takes
   less than 20 seconds to tokenize a GB of text on a server's CPU.
 - Easy to use, but also extremely versatile.
 - Designed for research and production.
 - Normalization comes with alignments tracking. It's always possible to get the part of the
   original sentence that corresponds to a given token.
 - Does all the pre-processing: Truncate, Pad, add the special tokens your model needs.

## Performances
Performances can vary depending on hardware, but running the [~/bindings/python/benches/test_tiktoken.py](bindings/python/benches/test_tiktoken.py) should give the following on a g6 aws instance:
![image](https://github.com/user-attachments/assets/2b913d4b-e488-4cbc-b542-f90a6c40643d)


## Bindings

**Upstream HuggingFace bindings:**
  - [Rust](https://github.com/huggingface/tokenizers/tree/main/tokenizers) (Original implementation)
  - [Python](https://github.com/huggingface/tokenizers/tree/main/bindings/python)
  - [Node.js](https://github.com/huggingface/tokenizers/tree/main/bindings/node)
  - [Ruby](https://github.com/ankane/tokenizers-ruby) (Contributed by @ankane, external repo)

**ErgoX-maintained bindings (this fork):**
  - [C Bindings](bindings/c/) - Cross-platform C FFI (⚠️ Experimental)
    - 7 platform targets: Linux, Windows, macOS (x64/ARM64), iOS, Android, WASM
    - 80 FFI functions exposing core tokenization
    - See [C Bindings README](bindings/c/README.md)

**Higher-level wrapper (recommended for .NET):**
  - [ErgoX.VecraX.ML.NLP.Tokenizers](https://github.com/ergox/vecrax) - Production-ready C# library using these C bindings

## Installation

### For HuggingFace Tokenizers (Python/Rust/Node)

**Python:**
```bash
pip install tokenizers
```

**Rust:**
```bash
cargo add tokenizers
```

**Node.js:**
```bash
npm install tokenizers
```

### For C Bindings (this fork)

**Pre-built binaries:**
Download from [Releases](https://github.com/ergosumx/vecrax-hf-tokenizers/releases) (look for `c-v*.*.*` tags)

**Build from source:**
```bash
cd bindings/c
cargo build --release
```

See [C Bindings Documentation](bindings/c/README.md) for platform-specific build instructions.

### For .NET Applications

**Recommended:** Use the ErgoX VecraX library (wraps these C bindings with a C# API):
```bash
dotnet add package ErgoX.VecraX.ML.NLP.Tokenizers
```
 
## Quick example using Python:

> **Note:** Python examples below use the **upstream HuggingFace Tokenizers** library.
> 
> For C bindings or .NET examples, see:
> - [C Bindings Examples](bindings/c/README.md#usage-example)
> - [ErgoX VecraX .NET Examples](https://github.com/ergox/vecrax)

Choose your model between Byte-Pair Encoding, WordPiece or Unigram and instantiate a tokenizer:

```python
from tokenizers import Tokenizer
from tokenizers.models import BPE

tokenizer = Tokenizer(BPE())
```

You can customize how pre-tokenization (e.g., splitting into words) is done:

```python
from tokenizers.pre_tokenizers import Whitespace

tokenizer.pre_tokenizer = Whitespace()
```

Then training your tokenizer on a set of files just takes two lines of codes:

```python
from tokenizers.trainers import BpeTrainer

trainer = BpeTrainer(special_tokens=["[UNK]", "[CLS]", "[SEP]", "[PAD]", "[MASK]"])
tokenizer.train(files=["wiki.train.raw", "wiki.valid.raw", "wiki.test.raw"], trainer=trainer)
```

Once your tokenizer is trained, encode any text with just one line:
```python
output = tokenizer.encode("Hello, y'all! How are you 😁 ?")
print(output.tokens)
# ["Hello", ",", "y", "'", "all", "!", "How", "are", "you", "[UNK]", "?"]
```

Check the [documentation](https://huggingface.co/docs/tokenizers/index)
or the [quicktour](https://huggingface.co/docs/tokenizers/quicktour) to learn more!

---

## ErgoX Fork Information

### Contributing

**Contributions are welcome, but scope matters:**

- ✅ **C bindings contributions (this repo):**
  - Bug fixes in FFI functions
  - New C API functions exposing existing Rust functionality
  - Platform compatibility improvements
  - Build system enhancements
  - Documentation improvements
  - See [C Bindings Contributing Guide](bindings/c/CONTRIBUTING.md)

- ❌ **Core library contributions (upstream repo):**
  - Tokenization algorithm changes
  - New models, normalizers, pre-tokenizers
  - Python/Node.js binding changes
  - → Submit to [huggingface/tokenizers](https://github.com/huggingface/tokenizers)

- 🔷 **.NET wrapper contributions:**
  - C# API improvements
  - .NET integration issues
  - → Submit to [ErgoX VecraX](https://github.com/ergox/vecrax)

### Release Schedule

**C bindings releases:**
- Follow HuggingFace Tokenizers major/minor releases
- Independent patch versions for C bindings-specific fixes
- Tag format: `c-v{MAJOR}.{MINOR}.{PATCH}`
- See [Release Process](bindings/c/RELEASE.md)

**Current status:**
- Based on HuggingFace Tokenizers: `v0.20.4`
- C bindings version: `c-v0.1.0` (experimental)
- Platform support: 7 platforms (Linux, Windows, macOS x2, iOS, Android, WASM)
- Test coverage: 179/180 tests passing (99.4%)

### Upstream Synchronization

This fork periodically syncs with upstream:
```bash
# Update to latest HuggingFace release
git remote add upstream https://github.com/huggingface/tokenizers.git
git fetch upstream
git merge upstream/main

# Test C bindings against new version
cd bindings/c
cargo test --release

# Release if compatible
git tag c-v0.2.0
git push origin c-v0.2.0
```

### License

This fork maintains the same license as the upstream repository: **Apache License 2.0**

See [LICENSE](LICENSE) for details.

### Acknowledgments

- **HuggingFace Team** - For the exceptional Tokenizers library
- **Rust Community** - For amazing tools and ecosystem
- **ErgoX Team** - For C# wrapper and production testing

### Links

- **Upstream Repository:** [huggingface/tokenizers](https://github.com/huggingface/tokenizers)
- **C Bindings Documentation:** [bindings/c/README.md](bindings/c/README.md)
- **ErgoX VecraX (.NET):** [github.com/ergox/vecrax](https://github.com/ergox/vecrax)
- **HuggingFace Docs:** [huggingface.co/docs/tokenizers](https://huggingface.co/docs/tokenizers)
- **Issues (C bindings):** [GitHub Issues](https://github.com/ergosumx/vecrax-hf-tokenizers/issues)
- **Issues (core library):** [HuggingFace Issues](https://github.com/huggingface/tokenizers/issues)
