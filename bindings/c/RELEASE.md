# Release Process for C Bindings

This document describes the release process for the HuggingFace Tokenizers C bindings.

## Table of Contents

- [Versioning](#versioning)
- [Release Types](#release-types)
- [Pre-Release Checklist](#pre-release-checklist)
- [Creating a Release](#creating-a-release)
- [Automated Build Process](#automated-build-process)
- [Manual Release (Fallback)](#manual-release-fallback)
- [Post-Release Tasks](#post-release-tasks)
- [Hotfix Process](#hotfix-process)
- [Platform-Specific Notes](#platform-specific-notes)

## Versioning

C bindings use **independent versioning** from the main HuggingFace Tokenizers library.

### Version Format

```
c-v{MAJOR}.{MINOR}.{PATCH}
```

**Examples:**
- `c-v0.1.0` - Initial release
- `c-v0.1.1` - Patch release
- `c-v0.2.0` - Minor release with new features
- `c-v1.0.0` - First stable release

### Versioning Rules

- **MAJOR**: Breaking changes to C API (function signatures, behavior changes)
- **MINOR**: New features, non-breaking additions
- **PATCH**: Bug fixes, performance improvements, documentation

**Prefix `c-`** distinguishes C bindings releases from:
- `v*.*.*` - Main Rust library
- `python-v*.*.*` - Python bindings
- `node-v*.*.*` - Node.js bindings

## Release Types

### Regular Release

Scheduled releases with new features and improvements.

**Timeline:**
- Feature freeze: 1 week before release
- Testing period: 3-5 days
- Release candidate: 2-3 days
- Final release: After RC approval

### Patch Release

Urgent bug fixes or security patches.

**Timeline:**
- Immediate after fix is merged
- Minimal testing (affected platforms only)
- No feature freeze required

### Pre-Release

Alpha/beta releases for testing.

**Format:**
- `c-v0.2.0-alpha.1`
- `c-v0.2.0-beta.1`
- `c-v0.2.0-rc.1`

## Pre-Release Checklist

### 1. Code Quality

- [ ] All tests pass locally (`cargo test --release`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No new compiler warnings

### 2. Cross-Platform Validation

Test builds on all supported platforms:

```bash
# Linux x64
cargo build --release --target x86_64-unknown-linux-gnu

# Windows x64
cargo build --release --target x86_64-pc-windows-msvc

# macOS x64 (Intel)
cargo build --release --target x86_64-apple-darwin

# macOS ARM64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# iOS ARM64
cargo build --release --target aarch64-apple-ios

# Android ARM64 (requires NDK)
cargo build --release --target aarch64-linux-android

# WebAssembly
cargo build --release --target wasm32-unknown-unknown
```

### 3. Documentation

- [ ] CHANGELOG.md updated with all changes
- [ ] README.md reflects current functionality
- [ ] API documentation (inline) is current
- [ ] Breaking changes are documented
- [ ] Migration guide added (if breaking changes)

### 4. Version Bump

Update version numbers in:
- [ ] `Cargo.toml` (version field)
- [ ] `CHANGELOG.md` (unreleased → version + date)
- [ ] `README.md` (example version numbers)

**Example:**
```toml
# Cargo.toml
[package]
name = "tokenizers-c"
version = "0.2.0"  # ← Update this
```

```markdown
# CHANGELOG.md
## [0.2.0] - 2024-01-15  # ← Change from [Unreleased]

### Added
- New function `tokenizers_batch_encode()`
...
```

### 5. Final Tests

```bash
# Clean build from scratch
cargo clean
cargo build --release
cargo test --release

# Verify binary outputs
ls -lh target/release/libtokenizers.*
```

## Creating a Release

### Automated Release (Recommended)

Releases are **fully automated** via GitHub Actions when you push a tag.

#### Step 1: Commit Version Changes

```bash
# Ensure you're on main branch
git checkout main
git pull origin main

# Commit version bumps
git add Cargo.toml CHANGELOG.md README.md
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

#### Step 2: Create and Push Tag

```bash
# Create annotated tag
git tag -a c-v0.2.0 -m "Release v0.2.0

## Changes
- Added batch encoding support
- Fixed memory leak in error paths
- Performance improvements for large vocabularies

## Platforms
- Linux x64, Windows x64, macOS x64/ARM64
- iOS ARM64, Android ARM64, WebAssembly
"

# Push tag (this triggers the workflow)
git push origin c-v0.2.0
```

#### Step 3: Monitor GitHub Actions

1. Go to: https://github.com/huggingface/tokenizers/actions
2. Find workflow: "Release C Bindings"
3. Monitor build progress (7 platform builds + release creation)
4. Estimated time: 15-30 minutes

#### Step 4: Verify Release

Once the workflow completes:

1. Go to: https://github.com/huggingface/tokenizers/releases
2. Find: "C Bindings v0.2.0"
3. Verify artifacts are attached:
   - `tokenizers-c-linux-x64.tar.gz`
   - `tokenizers-c-win-x64.zip`
   - `tokenizers-c-osx-x64.tar.gz`
   - `tokenizers-c-osx-arm64.tar.gz`
   - `tokenizers-c-ios-arm64.tar.gz`
   - `tokenizers-c-android-arm64.tar.gz`
   - `tokenizers-c-wasm.tar.gz`
   - `checksums.txt`

#### Step 5: Test Artifacts

Download and test each artifact:

```bash
# Linux example
wget https://github.com/huggingface/tokenizers/releases/download/c-v0.2.0/tokenizers-c-linux-x64.tar.gz
tar -xzf tokenizers-c-linux-x64.tar.gz
ls -lh native/libtokenizers.so

# Verify checksum
sha256sum native/libtokenizers.so
grep linux-x64 checksums.txt
```

### Manual Trigger

You can also trigger builds manually via GitHub UI:

1. Go to: https://github.com/huggingface/tokenizers/actions
2. Select: "Release C Bindings"
3. Click: "Run workflow"
4. Select branch: `main`
5. Click: "Run workflow"

This is useful for:
- Re-building a failed release
- Creating builds without a tag
- Testing the workflow

## Automated Build Process

The release workflow (`.github/workflows/release-c-bindings.yml`) performs:

### Build Jobs (7 platforms)

Each platform job:
1. **Checkout**: Repository with submodules
2. **Setup Rust**: Installs Rust toolchain + target
3. **Cache**: Restores cargo registry, git, build artifacts
4. **Build**: Compiles release binary
5. **Strip**: Removes debug symbols (Linux, macOS, Android)
6. **Package**: Creates platform-specific archive (.tar.gz or .zip)
7. **Upload**: Saves artifact for release job

**Platform-Specific:**

- **Linux**: Uses `ubuntu-latest`, gcc/g++
- **Windows**: Uses `windows-latest`, MSVC
- **macOS x64**: Uses `macos-13` (Intel runners)
- **macOS ARM64**: Uses `macos-14` (M1/M2 runners)
- **iOS**: Uses `macos-14`, Xcode toolchain
- **Android**: Uses `ubuntu-latest` + Android NDK r26d, custom cargo config
- **WASM**: Uses `ubuntu-latest` + wasm-pack

### Release Job

After all builds complete:
1. **Download**: All 7 platform artifacts
2. **Checksums**: Generates SHA-256 for all files
3. **Extract Version**: Parses tag (`c-v0.2.0` → `0.2.0`)
4. **Create Release**: GitHub Release with:
   - Name: "C Bindings v0.2.0"
   - Description: Platform list, installation instructions, changelog
   - Files: All 7 archives + checksums.txt
   - Tag: `c-v0.2.0`

## Manual Release (Fallback)

If GitHub Actions fails or is unavailable:

### Build Locally

```bash
# Linux (on Linux machine or WSL)
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/libtokenizers.so
tar -czf tokenizers-c-linux-x64.tar.gz \
    -C target/x86_64-unknown-linux-gnu/release \
    libtokenizers.so

# Windows (on Windows machine)
cargo build --release --target x86_64-pc-windows-msvc
Compress-Archive -Path target\x86_64-pc-windows-msvc\release\tokenizers.dll `
    -DestinationPath tokenizers-c-win-x64.zip

# macOS ARM64 (on M1/M2 Mac)
cargo build --release --target aarch64-apple-darwin
strip target/aarch64-apple-darwin/release/libtokenizers.dylib
tar -czf tokenizers-c-osx-arm64.tar.gz \
    -C target/aarch64-apple-darwin/release \
    libtokenizers.dylib

# ... (repeat for other platforms)
```

### Generate Checksums

```bash
# Linux/macOS
sha256sum tokenizers-c-*.tar.gz tokenizers-c-*.zip > checksums.txt

# Windows (PowerShell)
Get-ChildItem tokenizers-c-* | Get-FileHash -Algorithm SHA256 > checksums.txt
```

### Create GitHub Release Manually

1. Go to: https://github.com/huggingface/tokenizers/releases/new
2. Tag: `c-v0.2.0`
3. Title: `C Bindings v0.2.0`
4. Description: Copy from CHANGELOG.md + add installation instructions
5. Attach files: All archives + checksums.txt
6. Publish release

## Post-Release Tasks

### 1. Announcement

- [ ] Update project README with new version
- [ ] Post announcement in GitHub Discussions
- [ ] Notify ErgoX VecraX maintainers (if applicable)
- [ ] Update integration examples

### 2. Documentation

- [ ] Verify documentation site is updated
- [ ] Update platform support matrix
- [ ] Add release to version history

### 3. Monitoring

- [ ] Watch for bug reports (first 48 hours)
- [ ] Monitor download counts
- [ ] Check for platform-specific issues

### 4. Prepare Next Release

- [ ] Create `CHANGELOG.md` entry for next version
- [ ] Tag issues for next milestone
- [ ] Update project board

## Hotfix Process

For critical bugs in production:

### 1. Create Hotfix Branch

```bash
git checkout c-v0.2.0
git checkout -b hotfix/0.2.1
```

### 2. Apply Fix

```bash
# Make minimal changes
# Add test to prevent regression
cargo test
```

### 3. Version Bump

```bash
# Update Cargo.toml: 0.2.0 → 0.2.1
# Update CHANGELOG.md
git commit -am "fix: critical bug in tokenizers_decode"
```

### 4. Release

```bash
git tag -a c-v0.2.1 -m "Hotfix v0.2.1 - Fix critical decode bug"
git push origin c-v0.2.1
```

### 5. Backport to Main

```bash
git checkout main
git cherry-pick <hotfix-commit>
git push origin main
```

## Platform-Specific Notes

### Linux

- **Glibc compatibility**: Built on Ubuntu (recent glibc)
- **Distribution**: Works on most modern Linux distros
- **Testing**: Test on Ubuntu, Debian, Fedora, Alpine if possible

### Windows

- **MSVC runtime**: Ensure Visual C++ Redistributable is documented
- **Testing**: Test on Windows 10/11

### macOS

- **Universal Binary**: Build separate x64 and ARM64 binaries
- **Code Signing**: Not required for libraries (only apps)
- **Testing**: Test on both Intel and M1/M2 Macs

### iOS

- **Static Library**: iOS requires `.a` files (no dynamic linking)
- **Integration**: Users must link manually or via CocoaPods/SPM
- **Testing**: Test in Xcode project

### Android

- **NDK Version**: r26d (update in workflow if newer)
- **API Level**: Targets API 24+ (Android 7.0+)
- **ABIs**: Only ARM64 supported (add others if needed)

### WebAssembly

- **Targets**: `wasm32-unknown-unknown` for universal compatibility
- **Usage**: Works in browsers (via JS glue) and Node.js
- **Testing**: Test with wasm-pack and wasm-bindgen

## Troubleshooting

### Build Failures

**Symptom**: Platform build fails in CI

**Solutions:**
1. Check Rust version compatibility
2. Verify target is installed: `rustup target list --installed`
3. Check for platform-specific dependencies
4. Review build logs for linker errors

### Missing Artifacts

**Symptom**: Release created but some artifacts missing

**Solutions:**
1. Check individual job logs in GitHub Actions
2. Verify all jobs completed successfully
3. Re-run failed jobs
4. Manual upload if one platform fails

### Version Conflicts

**Symptom**: Tag already exists

**Solutions:**
```bash
# Delete local tag
git tag -d c-v0.2.0

# Delete remote tag
git push origin :refs/tags/c-v0.2.0

# Create new tag
git tag -a c-v0.2.0 -m "..."
git push origin c-v0.2.0
```

### Checksum Mismatches

**Symptom**: Downloaded file doesn't match checksum

**Solutions:**
1. Re-download artifact (network corruption)
2. Verify GitHub Actions didn't modify file
3. Check for platform-specific line ending issues
4. Rebuild that platform if necessary

## Rollback Process

If a release has critical issues:

### 1. Mark Release as Pre-Release

1. Go to release page
2. Edit release
3. Check "This is a pre-release"
4. Add warning message

### 2. Delete Broken Artifacts

Remove only the broken platform artifacts, keep working ones.

### 3. Issue Hotfix

Follow hotfix process above to create patch release.

### 4. Deprecation Notice

Add notice to CHANGELOG.md and release notes:

```markdown
## [0.2.0] - 2024-01-15 [DEPRECATED]

**⚠️ This release has been deprecated due to critical issue #123.**

**Use v0.2.1 instead.**
```

## FAQ

**Q: Can I release without all platforms?**
A: Yes, but document which platforms are missing in release notes.

**Q: How often should we release?**
A: Minor releases every 2-3 months, patch releases as needed.

**Q: Do we need to coordinate with main HuggingFace releases?**
A: No, C bindings have independent versioning.

**Q: Can I test the workflow without creating a release?**
A: Yes, use `workflow_dispatch` (manual trigger) without pushing a tag.

**Q: What if a platform is temporarily broken?**
A: Release other platforms, document the missing one, fix and re-release patch.

---

**Quick Links:**
- [GitHub Actions Workflow](../.github/workflows/release-c-bindings.yml)
- [Releases Page](https://github.com/huggingface/tokenizers/releases)
- [CHANGELOG](CHANGELOG.md)
- [Contributing Guide](CONTRIBUTING.md)
