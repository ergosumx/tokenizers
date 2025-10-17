# Tokenizers C Bindings - CI/CD Setup

## Overview

This repository contains the Rust C bindings for HuggingFace Tokenizers with automated testing and release workflows.

## Workflows

### 1. Rust C Bindings Tests (`.github/workflows/test-c-bindings.yml`)

**Purpose**: Automated testing of Rust C bindings on every push and PR

**Triggers**:
- Push to `main`, `master`, or `develop` branches
- Pull requests
- Manual dispatch

**Jobs**:

#### Test Job
- Runs on: Linux, Windows, macOS
- Executes: 16 Rust decoder tests
- Builds: Release binaries for all platforms

#### Coverage Job
- Uses: cargo-tarpaulin
- Generates: HTML and XML coverage reports
- Uploads to: Codecov

#### Test Summary Job
- Aggregates results
- Posts PR comments
- Generates GitHub step summary

**Badge**:
```markdown
[![Rust Tests](https://github.com/ergosumx/tokenizers/actions/workflows/test-c-bindings.yml/badge.svg)](https://github.com/ergosumx/tokenizers/actions/workflows/test-c-bindings.yml)
```

---

### 2. Release C Bindings (`.github/workflows/release-c-bindings.yml`)

**Purpose**: Build multi-platform binaries and create GitHub releases

**Triggers**:
- Tags matching `c-v*.*.*` (e.g., `c-v0.22.1`)
- Manual dispatch

**Build Targets**:
1. Linux x64
2. Windows x64
3. macOS x64
4. macOS ARM64
5. iOS ARM64
6. Android ARM64
7. WebAssembly

**Release Assets**:
- Platform-specific binary archives (`.tar.gz`, `.zip`)
- SHA-256 checksums file
- Release notes with platform support

---

## Running Tests Locally

### Rust Tests
```bash
cd bindings/c
cargo test --release
```

### With Coverage
```bash
cd bindings/c
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features --workspace \
  --timeout 120 --out Html --out Xml --output-dir coverage
```

---

## Creating a Release

1. **Tag the release**:
   ```bash
   git tag c-v0.22.2
   git push origin c-v0.22.2
   ```

2. **Workflow automatically**:
   - Builds for 7 platforms
   - Generates checksums
   - Creates GitHub Release

3. **Verify release**:
   - Check Actions tab
   - Review release notes
   - Download and test binaries

---

## Integration with ErgoX VecraX

These C bindings are consumed by the [ErgoX VecraX ML.NLP.Tokenizers](https://github.com/ergosumx/vecrax-hf-tokenizers) .NET library.

The .NET integration tests are maintained in the parent repository.

---

**Last Updated**: October 17, 2025  
**Repository**: https://github.com/ergosumx/tokenizers
