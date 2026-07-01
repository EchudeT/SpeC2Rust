# Implementation Plan

## Summary

Port the C module `c-strcasecmp.c` into a focused Rust implementation that preserves the original case-insensitive string comparison behavior without broadening scope. The Rust version should provide a single module-level function corresponding to `c_strcasecmp`, implemented with explicit byte-wise comparison logic over string data.

The technical approach is to migrate the existing comparison routine directly into safe Rust where possible, using standard-library primitives for byte access and ASCII case folding. Because the source function name and file indicate `strcasecmp`-style semantics, the implementation should stay narrowly aligned with C behavior: compare two strings lexicographically after ASCII lowercasing each byte, and return an integer ordering result compatible with the original call pattern.

No new abstraction layers or auxiliary subsystems should be introduced. The work should focus on a minimal module port, a stable public function signature suitable for the crate’s internal use, and tests that cover equality, ordering, prefix differences, and ASCII case handling.

## Technical Context

- **Language/Version**: Rust 1.75 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear-time comparison behavior, matching the original C routine’s asymptotic cost.
  - Avoid heap allocation during comparison.
  - Compare bytes directly and stop at the first non-matching folded byte.
  - Keep implementation close to C control flow to reduce migration risk.

## Module Mapping

- **C source file**: `c-strcasecmp.c`
- **Rust module/file**: `src/c_strcasecmp.rs`

### Function Mapping

- `c_strcasecmp` -> `pub(crate)` or `pub` function `c_strcasecmp`

Recommended Rust signature:

```rust
pub fn c_strcasecmp(a: &str, b: &str) -> i32
```

Implementation notes:
- Operate on `a.as_bytes()` and `b.as_bytes()`.
- Apply ASCII-only case folding per compared byte.
- Return:
  - `0` when equal under ASCII case-insensitive comparison,
  - negative value when `a < b`,
  - positive value when `a > b`.
- Prefer returning the arithmetic difference between the first differing folded bytes to remain close to C ordering behavior.

If surrounding project call sites require C-like byte-string handling rather than UTF-8 `&str`, adjust only as needed to:

```rust
pub fn c_strcasecmp(a: &[u8], b: &[u8]) -> i32
```

This should be decided from actual integration needs, not preemptively.

## Data Model

This module has no dedicated C structs or persistent data types.

### Data-structure Mapping

- No C struct/enum mappings required.
- C string inputs (`const char *` style) map to Rust borrowed inputs:
  - preferred: `&str` if callers already operate on valid text
  - fallback: `&[u8]` if byte-oriented semantics are needed to stay closer to C

### Memory Management

- Use borrowed inputs only; no owned buffers should be introduced.
- No manual allocation or deallocation is required.
- Comparison should be performed through indexed or iterator-based byte access.

### Error Handling

- The comparison function should remain infallible.
- Do not introduce `Result` unless integration constraints force input validation beyond the original C behavior.
- Null-pointer concerns from C are handled by Rust type guarantees at the API boundary; if a lower-level byte API is needed, represent absence outside this function rather than embedding nullable behavior here.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton

- Create `src/c_strcasecmp.rs`.
- Add the Rust function corresponding to `c_strcasecmp`.
- Select the narrowest practical input type based on current crate usage:
  - start with `&str` unless existing interfaces require raw bytes.
- Keep naming close to the original C module/function to simplify traceability during review.
- Expose the module through the existing crate structure only as needed for current call sites.

### Deliverables

- Rust source file for the ported module
- Function stub with final chosen signature
- Basic module wiring in `src/lib.rs` or `src/main.rs` as appropriate

## Phase 2: Port Core Comparison Logic

- Translate the C routine into straightforward Rust control flow.
- Implement byte-wise iteration over both inputs.
- Fold each byte using ASCII lowercase conversion before comparison.
- Preserve C-like lexicographic termination behavior:
  - stop on first differing folded byte,
  - otherwise continue until one or both inputs end,
  - account for differing lengths/prefix relationships in the return value.
- Avoid introducing locale-dependent or Unicode case-folding behavior.

### Deliverables

- Completed `c_strcasecmp` implementation
- Inline comments only where needed to document C-behavior preservation
- No extra helper layers unless required to keep logic readable

## Phase 3: Add Focused Tests

- Add unit tests alongside the module or in the crate’s standard test layout.
- Cover:
  - exact equality
  - equality under ASCII case differences
  - less-than and greater-than ordering
  - prefix comparisons
  - empty-string handling
  - non-alphabetic ASCII bytes remaining unchanged in comparison
- Keep expected assertions aligned to sign behavior (`< 0`, `== 0`, `> 0`) unless the exact numeric difference is intentionally preserved and relied upon.

### Deliverables

- Unit tests runnable with `cargo test`
- Verification that the function behaves consistently across representative ASCII inputs

## Phase 4: Integrate and Finalize Migration

- Replace or connect existing internal usage to the Rust `c_strcasecmp` function.
- Confirm that no residual C-specific assumptions remain at the Rust boundary for this module.
- Run formatting and tests.
- Keep the final module limited to the original responsibility of case-insensitive comparison.

### Deliverables

- Integrated Rust module in the project branch
- Passing `cargo test`
- Final review for scope containment and behavioral parity