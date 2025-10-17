---
name: Platform Support Request
about: Request support for a new platform or architecture
title: '[PLATFORM] '
labels: 'platform, enhancement, c-bindings'
assignees: ''
---

## Platform Details

**Which platform would you like to see supported?**

- **Operating System**: (e.g., FreeBSD, Raspberry Pi OS, embedded Linux)
- **Architecture**: (e.g., ARM32, RISC-V, x86, PowerPC)
- **Target Triple**: (e.g., `armv7-unknown-linux-gnueabihf`, `riscv64gc-unknown-linux-gnu`)

## Use Case

**Why is this platform important?**
- What are you building?
- How many users would benefit?
- Is this for production use or experimentation?

## Rust Support

**Does Rust support this platform?**

Check: https://doc.rust-lang.org/nightly/rustc/platform-support.html

- [ ] Tier 1 (guaranteed to work)
- [ ] Tier 2 (guaranteed to build)
- [ ] Tier 3 (no guarantees)
- [ ] Not officially supported

**Rust target name:** (e.g., `aarch64-unknown-linux-musl`)

## Toolchain Availability

**Is a C/C++ toolchain available?**
- [ ] Yes, widely available
- [ ] Yes, but requires special setup
- [ ] No, needs cross-compilation

**Toolchain details:**
- Compiler: (e.g., gcc, clang, custom)
- Linker: (e.g., ld, lld, custom)
- Standard library: (e.g., glibc, musl, bionic)

## CI/CD Feasibility

**Can we build this platform in GitHub Actions?**
- [ ] Yes, there are existing GitHub Actions runners
- [ ] Yes, via cross-compilation
- [ ] No, requires special hardware
- [ ] Not sure

**Suggested CI approach:**
```yaml
# Example GitHub Actions configuration
- name: Build for new platform
  run: |
    # Commands to build
```

## Testing Capability

**How can we test builds for this platform?**
- [ ] I can test builds on real hardware
- [ ] Emulator available (e.g., QEMU)
- [ ] Cloud provider offers instances
- [ ] Community can help test
- [ ] No testing capability

## Complexity Assessment

**What challenges might this platform introduce?**
- [ ] Standard cross-compilation (low complexity)
- [ ] Requires custom build scripts (medium complexity)
- [ ] Requires platform-specific code (high complexity)
- [ ] Unknown

**Known issues or blockers:**
- Specific dependencies not available?
- Platform-specific quirks?
- Performance concerns?

## Priority

**How urgent is this request?**
- [ ] Critical (blocking production deployment)
- [ ] High (would unlock significant use cases)
- [ ] Medium (nice to have)
- [ ] Low (experimental interest)

## Contribution Offer

**Can you help with implementation?**
- [ ] I can provide testing hardware
- [ ] I can test builds
- [ ] I can contribute build scripts
- [ ] I can maintain the platform long-term
- [ ] I need the community to implement this

## Additional Context

**Any other relevant information:**
- Similar projects supporting this platform?
- Documentation or guides available?
- Community size for this platform?
- Expected user demand?

## Checklist

- [ ] I have checked if Rust supports this platform
- [ ] I have searched existing issues
- [ ] I have described testing capabilities
- [ ] I have provided target triple (if known)
- [ ] I am willing to help test (if possible)
