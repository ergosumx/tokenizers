## Description

**Brief description of your changes:**

<!-- Explain WHAT you changed and WHY -->

Fixes # (issue number)
Relates to # (issue number)

## Type of Change

<!-- Mark relevant options with [x] -->

- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature causing existing functionality to break)
- [ ] Performance improvement
- [ ] Documentation update
- [ ] Build/CI improvement
- [ ] Refactoring (no functional changes)

## Breaking Changes

<!-- If this is a breaking change, describe the impact and migration path -->

**What breaks:**

**Migration guide:**

## Testing

### Test Coverage

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All existing tests pass
- [ ] Manual testing completed

### Platform Testing

<!-- Check all platforms you've tested on -->

- [ ] Linux x64
- [ ] Windows x64
- [ ] macOS x64 (Intel)
- [ ] macOS ARM64 (Apple Silicon)
- [ ] iOS ARM64
- [ ] Android ARM64
- [ ] WebAssembly
- [ ] Not platform-specific

**How to test:**

```bash
# Commands to verify your changes
cargo build --release
cargo test
```

## Code Quality

- [ ] Code compiles without warnings (`cargo build --release`)
- [ ] All tests pass (`cargo test --release`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Linter passes (`cargo clippy -- -D warnings`)
- [ ] No new memory leaks (Valgrind/AddressSanitizer checked)
- [ ] API documentation updated (inline doc comments)

## Documentation

- [ ] README.md updated (if API changed)
- [ ] CHANGELOG.md updated
- [ ] Inline code documentation added/updated
- [ ] Examples added/updated (if applicable)
- [ ] Migration guide added (if breaking change)

## Performance Impact

<!-- If this change affects performance, provide benchmarks -->

**Before:**
```
# Benchmark results before your changes
```

**After:**
```
# Benchmark results after your changes
```

**Analysis:**
<!-- Explain performance impact (positive, negative, or neutral) -->

## Memory Safety

<!-- Describe any unsafe code additions or changes -->

**Unsafe code:**
- [ ] No unsafe code added
- [ ] Unsafe code added with justification below

**If unsafe code added:**
- Why is unsafe necessary?
- What safety invariants must hold?
- How are safety invariants enforced?

```rust
// Example:
unsafe {
    // Why: Direct memory access for FFI
    // Invariant: ptr must be non-null and aligned
    // Enforcement: Null check before this block
    *ptr = value;
}
```

## API Changes

<!-- If you changed the C API, document it here -->

**New functions:**
```c
// Function signatures
```

**Modified functions:**
```c
// Before:
void old_function(int param);

// After:
void new_function(int param, int new_param);
```

**Deprecated functions:**
```c
// List functions that should be deprecated
```

**Removed functions:**
```c
// List functions removed (breaking change)
```

## Dependencies

- [ ] No new dependencies added
- [ ] New dependencies added (listed below)

**New dependencies:**
- `crate-name = "version"` - Reason for adding

## Backward Compatibility

- [ ] Fully backward compatible
- [ ] Backward compatible with deprecation warnings
- [ ] Breaking change (requires major version bump)

## Security Considerations

<!-- Does this change have security implications? -->

- [ ] No security impact
- [ ] Fixes security vulnerability
- [ ] Introduces new security considerations (described below)

**Security notes:**

## Checklist

- [ ] I have read the [Contributing Guidelines](CONTRIBUTING.md)
- [ ] I have performed a self-review of my code
- [ ] I have commented my code where necessary
- [ ] I have updated the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] All tests pass locally
- [ ] I have checked for memory leaks
- [ ] I have considered cross-platform compatibility
- [ ] I have updated CHANGELOG.md

## Additional Context

<!-- Any other information reviewers should know -->

**Screenshots/Logs:**
<!-- If applicable, add screenshots or log output -->

**Related Work:**
<!-- Links to related PRs, issues, or external resources -->

**Reviewer Notes:**
<!-- Specific areas you'd like reviewers to focus on -->

---

## For Reviewers

**Review focus areas:**
- [ ] Code correctness
- [ ] Memory safety
- [ ] Platform compatibility
- [ ] API design
- [ ] Performance
- [ ] Documentation
- [ ] Test coverage

**Estimated review time:** <!-- e.g., 15 minutes, 1 hour -->
