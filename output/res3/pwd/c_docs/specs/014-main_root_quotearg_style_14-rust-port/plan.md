# Implementation Plan

## Summary

Port the `quotearg.c` portion of the main cluster into Rust, focusing only on the functionality needed by:

- `quotearg_style`
- `quotearg_style_mem`

The Rust implementation should preserve the existing behavior shape of style-based quoting and memory-slice-based quoting without adding new quoting modes or broader formatting utilities. The recommended approach is to migrate the required quoting logic into a single Rust module with a narrow public API matching the current call patterns:

- one function accepting a quoting style and string-like input
- one function accepting a quoting style and explicit byte slice length semantics

Implementation should prefer safe Rust with `&str` and `&[u8]` where possible, using owned `String` output. Any C patterns involving temporary buffers, implicit null termination, or shared static state should be converted into explicit ownership and return values. Error handling should remain minimal and local to invalid UTF-8 or byte-processing boundaries, using standard library types and avoiding global mutable state.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s practical runtime characteristics for typical command-line sized inputs.
  - Avoid unnecessary intermediate allocations beyond the final output buffer.
  - Support direct processing of byte slices for `_mem` behavior.
  - Keep complexity linear in input length.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` -> `src/quotearg.rs`

### Function Mapping

- `quotearg_style` -> `pub fn quotearg_style(style: QuotingStyle, arg: &str) -> String`
- `quotearg_style_mem` -> `pub fn quotearg_style_mem(style: QuotingStyle, arg: &[u8]) -> String`

### Integration Mapping

If these functions are currently used from the crate root or main execution path, expose the module through the existing crate layout only as needed:

- `src/quotearg.rs`
- `src/lib.rs` or `src/main.rs`: `mod quotearg;` and minimal re-export/import required by current callers

No extra helper modules should be introduced unless directly required to keep the migrated code compileable.

## Data Model

The analysis lists only anonymous C data structures, so the Rust mapping should stay conservative and only introduce named Rust types required by these two functions.

### C Anonymous Structures -> Rust Representation

Because the relevant exported functions are style-oriented, the likely required structure set should be collapsed into a minimal Rust model:

- anonymous quoting style identifiers -> `enum QuotingStyle`
- anonymous option/state records used internally by quoting logic -> private `struct QuotingOptions` only if needed
- anonymous lookup/config fragments -> private constants, match arms, or small private enums

### Recommended Rust Types

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotingStyle {
    // exact variants to be added only as required by the C logic used here
}
```

If the C implementation uses an options object implicitly or shares behavior between the two functions:

```rust
#[derive(Clone, Debug)]
struct QuotingOptions {
    style: QuotingStyle,
}
```

### Memory and Ownership Mapping

- C `char *` input -> Rust `&str` for `quotearg_style`
- C pointer + length input -> Rust `&[u8]` for `quotearg_style_mem`
- C returned quoted buffer/static buffer -> Rust `String`
- C mutable temporary buffers -> local `String` with reserved capacity
- C enum/integer style selectors -> Rust `enum QuotingStyle`, converting integer-style call sites only where required by existing code

### Error Handling Mapping

The original C API likely does not expose rich errors. The Rust port should therefore:

- keep `quotearg_style` infallible for valid `&str` input
- keep `quotearg_style_mem` infallible by operating on bytes directly
- avoid `unwrap()` in core logic
- treat non-UTF-8 bytes via escaping/quoting logic rather than conversion failure where the C behavior is byte-oriented

## Implementation Phases

### Phase 1: Establish Rust module and public API surface

Create the Rust module file and define the minimum public interface required to replace the C functions.

Tasks:
- Add `src/quotearg.rs`
- Define `QuotingStyle` with only the variants actually referenced by current call sites or required by the migrated logic
- Implement function signatures:
  - `quotearg_style`
  - `quotearg_style_mem`
- Wire the module into the current crate entry points with minimal exposure

Technical decisions:
- Use `String` as the return type instead of borrowed/static buffers
- Keep helper functions private to `src/quotearg.rs`
- Do not introduce generic quoting frameworks or configuration builders

Exit criteria:
- Module compiles
- Existing Rust code can call the new functions through stable signatures

### Phase 2: Port core quoting logic from `quotearg.c`

Translate the logic needed for style dispatch and byte-wise quoting into safe Rust.

Tasks:
- Port the style selection logic used by `quotearg_style`
- Port the explicit-length byte handling used by `quotearg_style_mem`
- Replace pointer arithmetic with indexed or iterator-based byte traversal
- Recreate escaping and delimiter insertion behavior exactly as needed by the C paths exercised by these two functions

Technical decisions:
- Operate on `&[u8]` internally for shared logic
- Use `match` on `QuotingStyle` instead of integer switches
- Pre-allocate output with a conservative capacity estimate when possible
- Keep all state local to the function call

Memory management notes:
- No static mutable buffers
- No manual frees
- Avoid repeated `push_str` from temporary strings when a direct `push`/byte-to-fragment path is available

Exit criteria:
- Behaviorally complete implementation of the two target functions
- No unsafe code unless a specific C behavior cannot be represented safely, which is unlikely here

### Phase 3: Align call-site compatibility and edge-case behavior

Adapt the Rust implementation to match the actual calling patterns and edge semantics used by the surrounding main cluster.

Tasks:
- Verify style enum mapping against existing callers
- Confirm handling of empty input, embedded NUL bytes, and non-printable bytes for `_mem`
- Ensure `quotearg_style` delegates through the same core path as `_mem` where appropriate
- Remove any placeholder variants or logic not needed by the existing codebase

Technical decisions:
- Preserve byte-oriented semantics for `_mem`
- Keep UTF-8 assumptions limited to the `&str` entry point only
- Prefer exact behavior parity over API generalization

Exit criteria:
- Surrounding main-cluster code builds against the Rust port
- Edge cases observed in the original C paths are represented in tests

### Phase 4: Add focused tests and finalize migration

Create tests targeted strictly at the migrated behavior and complete replacement of the C file responsibilities for this module.

Tasks:
- Add unit tests in `src/quotearg.rs` or `tests/` for:
  - representative quoting style outputs
  - empty input
  - inputs requiring escaping
  - byte-slice inputs with explicit non-text content
- Compare expected outputs against known C behavior for the relevant styles
- Remove or disable the old C implementation from the Rust build path if applicable

Testing scope:
- deterministic output tests only
- no benchmark or fuzzing plan
- no extra integration harness beyond `cargo test`

Exit criteria:
- `cargo test` passes
- The Rust module fully covers the responsibilities of `main_root_quotearg_style_14` within the current branch