# Release Process

> **⚠️ ErgoX Fork Notice**
>
> This is an ErgoX-maintained fork focused on **C bindings development**.
>
> - **For C bindings releases:** See [C Bindings Release Process](#c-bindings-release-process-ergox) below
> - **For upstream releases (Rust/Python/Node):** This follows the original HuggingFace process

---

## C Bindings Release Process (ErgoX)

**Status:** Experimental - API may change between releases

### Version Format

```
c-v{MAJOR}.{MINOR}.{PATCH}
```

**Example:** `c-v0.1.0`, `c-v0.2.0`, `c-v1.0.0`

**Versioning is independent** from upstream HuggingFace Tokenizers releases.

### Release Workflow

```
1. HuggingFace releases new version (e.g., v0.20.5)
   ↓
2. Sync this fork with upstream
   git fetch upstream
   git merge upstream/main
   ↓
3. Test C bindings compatibility
   cd bindings/c
   cargo test --release
   ↓
4. Update C bindings if needed
   - Fix breaking changes
   - Add new FFI functions (if desired)
   - Update documentation
   ↓
5. Release C bindings
   git tag c-v0.2.0
   git push origin c-v0.2.0
   ↓
6. GitHub Actions builds 7 platforms automatically
   ↓
7. ErgoX VecraX .NET library updates
```

### Quick Release Steps

**Prerequisites:**
- All tests pass: `cd bindings/c && cargo test --release`
- Documentation updated: `README.md`, `CHANGELOG.md`
- Version bumped in `Cargo.toml`

**Release:**
```bash
# Tag and push (triggers automated build)
git tag -a c-v0.2.0 -m "C Bindings v0.2.0

- Based on HuggingFace Tokenizers v0.20.5
- Added batch encoding support
- Fixed memory leak in error paths
"
git push origin c-v0.2.0
```

**Automated builds create:**
- Linux x64 binaries (`.so`)
- Windows x64 binaries (`.dll`)
- macOS x64/ARM64 binaries (`.dylib`)
- iOS ARM64 static library (`.a`)
- Android ARM64 binaries (`.so`)
- WebAssembly binaries (`.wasm`)
- SHA-256 checksums

**See detailed process:** [bindings/c/RELEASE.md](bindings/c/RELEASE.md)

### Upstream Sync Process

When HuggingFace releases a new version:

```bash
# Add upstream remote (one-time)
git remote add upstream https://github.com/huggingface/tokenizers.git

# Fetch and merge
git checkout main
git fetch upstream
git merge upstream/main

# Resolve conflicts (if any) in C bindings
# Test thoroughly
cd bindings/c
cargo clean
cargo build --release
cargo test --release

# If compatible, release new C bindings version
git tag c-v0.X.0
git push origin c-v0.X.0
```

### Development Status

**Current versions:**
- **Upstream base:** HuggingFace Tokenizers v0.20.4
- **C bindings:** c-v0.1.0 (experimental)
- **API stability:** ⚠️ May change - experimental phase

**Platform support:**
- ✅ Linux x64
- ✅ Windows x64
- ✅ macOS x64 (Intel)
- ✅ macOS ARM64 (Apple Silicon)
- ✅ iOS ARM64
- ✅ Android ARM64
- ✅ WebAssembly

**Quality metrics:**
- 179/180 tests passing (99.4%)
- 0 compiler warnings
- 0 SonarQube issues
- 80 FFI functions

---

## Upstream Release Process (HuggingFace)

> **Note:** The following sections document the **original HuggingFace release process** for Rust, Python, and Node.js bindings.
>
> **ErgoX does not modify these.** We only maintain the C bindings.

## How to release

# Before the release

Simple checklist on how to make releases for `tokenizers`.

- Freeze `master` branch.
- Run all tests (Check CI has properly run)
- If any significant work, check benchmarks:
  - `cd tokenizers && cargo bench` (needs to be run on latest release tag to measure difference if it's your first time)
- Run all `transformers` tests. (`transformers` is a big user of `tokenizers` we need
  to make sure we don't break it, testing is one way to make sure nothing unforeseen
  has been done.)
  - Run all fast tests at the VERY least (not just the tokenization tests). (`RUN_PIPELINE_TESTS=1 CUDA_VISIBLE_DEVICES=-1 pytest -sv tests/`)
  - When all *fast*  tests work, then we can also (it's recommended) run the whole `transformers`
  test suite. 
    - Rebase this [PR](https://github.com/huggingface/transformers/pull/16708).
        This will create new docker images ready to run the tests suites with `tokenizers` from the main branch.
    - Wait for actions to finish
    - Rebase this [PR](https://github.com/huggingface/transformers/pull/16712)
        This will run the actual full test suite.
    - Check the results.
- **If any breaking change has been done**, make sure the version can safely be increased for transformers users (`tokenizers` version need to make sure users don't upgrade before `transformers` has). [link](https://github.com/huggingface/transformers/blob/main/setup.py#L154)
  For instance `tokenizers>=0.10,<0.11` so we can safely upgrade to `0.11` without impacting
  current users
- Then start a new PR containing all desired code changes from the following steps.
- You will `Create release` after the code modifications are on `master`.

# Rust

- `tokenizers` (rust, python & node) versions don't have to be in sync but it's
  very common to release for all versions at once for new features.
- Edit `Cargo.toml` to reflect new version
- Edit `CHANGELOG.md`:
    - Add relevant PRs that were added (python PRs do not belong for instance).
    - Add links at the end of the files.
- Go to [Releases](https://github.com/huggingface/tokenizers/releases)
- Create new Release:
    - Mark it as pre-release
    - Use new version name with a new tag (create on publish) `vX.X.X`.
    - Copy paste the new part of the `CHANGELOG.md`
- ⚠️  Click on `Publish release`. This will start the whole process of building a uploading
  the new version on `crates.io`, there's no going back after this
- Go to the [Actions](https://github.com/huggingface/tokenizers/actions) tab and check everything works smoothly.
- If anything fails, you need to fix the CI/CD to make it work again. Since your package was not uploaded to the repository properly, you can try again.


# Python

- Edit `bindings/python/setup.py` to reflect new version.
- Edit `bindings/python/py_src/tokenizers/__init__.py` to reflect new version.
- Edit `CHANGELOG.md`:
    - Add relevant PRs that were added (node PRs do not belong for instance).
    - Add links at the end of the files.
- Go to [Releases](https://github.com/huggingface/tokenizers/releases)
- Create new Release:
    - Mark it as pre-release
    - Use new version name with a new tag (create on publish) `python-vX.X.X`.
    - Copy paste the new part of the `CHANGELOG.md`
- ⚠️  Click on `Publish release`. This will start the whole process of building a uploading
  the new version on `pypi`, there's no going back after this
- Go to the [Actions](https://github.com/huggingface/tokenizers/actions) tab and check everything works smoothly.
- If anything fails, you need to fix the CI/CD to make it work again. Since your package was not uploaded to the repository properly, you can try again.
- This CI/CD has 3 distinct builds, `Pypi`(normal), `conda` and `extra`. `Extra` is REALLY slow (~4h), this is normal since it has to rebuild many things, but enables the wheel to be available for old Linuxes

# Node

- Edit `bindings/node/package.json` to reflect new version.
- Edit `CHANGELOG.md`:
    - Add relevant PRs that were added (python PRs do not belong for instance).
    - Add links at the end of the files.
- Go to [Releases](https://github.com/huggingface/tokenizers/releases)
- Create new Release:
    - Mark it as pre-release
    - Use new version name with a new tag (create on publish) `node-vX.X.X`.
    - Copy paste the new part of the `CHANGELOG.md`
- ⚠️  Click on `Publish release`. This will start the whole process of building a uploading
  the new version on `npm`, there's no going back after this
- Go to the [Actions](https://github.com/huggingface/tokenizers/actions) tab and check everything works smoothly.
- If anything fails, you need to fix the CI/CD to make it work again. Since your package was not uploaded to the repository properly, you can try again.


# Testing the CI/CD for release


If you want to make modifications to the CI/CD of the release GH actions, you need
to : 
- **Comment the part that uploads the artifacts** to `crates.io`, `PyPi` or `npm`.
- Change the trigger mechanism so it can trigger every time you push to your branch.
- Keep pushing your changes until the artifacts are properly created.

---

## ErgoX Fork: Additional Information

### Why This Fork Exists

1. **C bindings are not part of upstream** - HuggingFace maintains Rust, Python, and Node.js bindings only
2. **ErgoX VecraX needs native libraries** - Our .NET tokenization library requires cross-platform C FFI
3. **Independent release cycle** - C bindings can be updated/fixed without waiting for upstream releases
4. **Platform-specific testing** - We test on 7 platforms including mobile and WASM

### What We Don't Do

- ❌ Fork the core tokenization algorithms
- ❌ Modify Python, Node.js, or Rust bindings
- ❌ Create competing implementations
- ❌ Diverge from upstream functionality

### What We Do

- ✅ Maintain C bindings in `bindings/c/`
- ✅ Sync with upstream HuggingFace releases
- ✅ Test C bindings against new versions
- ✅ Release cross-platform native libraries
- ✅ Support ErgoX VecraX .NET library

### Release Tag Format

- `v*.*.*` - Upstream Rust library (not released from this fork)
- `python-v*.*.*` - Upstream Python bindings (not released from this fork)
- `node-v*.*.*` - Upstream Node.js bindings (not released from this fork)
- `c-v*.*.*` - **ErgoX C bindings** (released from this fork) ✅

### Contributing

**C bindings contributions:** Submit PRs to this repository
- See [bindings/c/CONTRIBUTING.md](bindings/c/CONTRIBUTING.md)

**Core library contributions:** Submit to upstream
- Go to [huggingface/tokenizers](https://github.com/huggingface/tokenizers)

### Questions?

- **C bindings:** [GitHub Issues](https://github.com/ergosumx/vecrax-hf-tokenizers/issues)
- **Core library:** [HuggingFace Issues](https://github.com/huggingface/tokenizers/issues)
- **.NET wrapper:** [ErgoX VecraX Issues](https://github.com/ergox/vecrax/issues)

### Links

- **Upstream:** [github.com/huggingface/tokenizers](https://github.com/huggingface/tokenizers)
- **C Bindings Docs:** [bindings/c/README.md](bindings/c/README.md)
- **C Bindings Release Docs:** [bindings/c/RELEASE.md](bindings/c/RELEASE.md)
- **ErgoX VecraX:** [github.com/ergox/vecrax](https://github.com/ergox/vecrax)
