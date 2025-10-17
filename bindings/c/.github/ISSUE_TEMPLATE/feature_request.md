---
name: Feature Request
about: Suggest a new feature or enhancement for the C bindings
title: '[FEATURE] '
labels: 'enhancement, c-bindings'
assignees: ''
---

## Feature Description

**Clear and concise description of the feature you'd like to see.**

## Use Case

**Describe the problem this feature would solve:**
- What are you trying to accomplish?
- Why is the current API insufficient?
- How would this feature improve your workflow?

## Proposed Solution

**Describe how you envision this feature working:**

### API Design

```c
// Example of proposed C function signature(s)
void* tokenizers_new_feature(void* tokenizer, /* parameters */, int* status);
```

### Usage Example

```c
// How would users call this feature?
int status = 0;
void* result = tokenizers_new_feature(tokenizer, param1, param2, &status);
if (status == 0) {
    // Use result...
    tokenizers_free_result(result);
}
```

## Alternatives Considered

**What other approaches have you considered?**
- Alternative API designs?
- Workarounds you're currently using?
- Similar features in other libraries?

## Scope

**Does this feature belong in the C bindings?**

- [ ] Yes, this is specific to the C FFI layer
- [ ] No, this should be added to the core Rust library first
- [ ] Not sure

**Note:** If this requires changes to the core tokenization logic, it may need to be implemented in the [main HuggingFace Tokenizers repository](https://github.com/huggingface/tokenizers) first.

## Platform Considerations

**Would this feature be platform-specific or cross-platform?**
- [ ] Cross-platform (all platforms)
- [ ] Platform-specific: ___________

**Special considerations:**
- Memory management implications?
- Thread safety requirements?
- Performance impact?

## Additional Context

**Any other relevant information:**
- References to similar implementations?
- Links to documentation?
- Related issues or PRs?
- Real-world examples where this is needed?

## Willingness to Contribute

- [ ] I am willing to implement this feature and submit a PR
- [ ] I can help with testing
- [ ] I can help with documentation
- [ ] I need someone else to implement this

## Checklist

- [ ] I have searched existing issues to ensure this is not a duplicate
- [ ] I have described a clear use case
- [ ] I have proposed a concrete solution
- [ ] This belongs in the C bindings (not core library)
