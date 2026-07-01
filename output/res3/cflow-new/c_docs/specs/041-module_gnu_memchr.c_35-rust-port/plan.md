# Implementation Plan: module_gnu_memchr.c_35

## Summary

This module ports `gnu/memchr.c` into Rust with a narrow scope: preserve the existing byte-search behavior of `__memchr` while adapting pointer-based C logic into idiomatic, low-level Rust.

The Rust implementation should:
- keep the functionality confined to searching for a target byte within a bounded memory region,
- use standard-library byte-slice operations where they preserve semantics and performance,
- isolate any unavoidable `unsafe` logic to thin boundaries where raw pointers are converted to slices,
- maintain C-like null/absence behavior through pointer- or index-oriented return handling as required by the surrounding port.

The technical approach is to migrate the single function into a focused Rust module, represent the input memory as `&[u8]` whenever possible, and return either an optional location or raw pointer equivalent depending on the integration contract used elsewhere in the port branch.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s linear scan behavior and bounded-memory semantics.
  - Avoid heap allocation.
  - Prefer zero-copy slice-based scanning.
  - Keep overhead from pointer-to-slice conversion minimal.
  - Remain suitable for optimized builds where slice search compiles to efficient byte scanning.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `gnu/memchr.c` | `src/gnu/memchr.rs` | Direct module migration for the single function in this source file. |
| `__memchr` | `pub(crate)` Rust function in `src/gnu/memchr.rs` | Preserve narrow behavior and naming correspondence as closely as project conventions allow. |

If the branch already centralizes GNU-origin ports under a different path, the file should still map one-to-one with the source module and avoid introducing extra abstraction layers.

## Data Model

This module has no named C structs or persistent data structures.

### C-to-Rust Type Mapping

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| `const void *` memory input | `*const u8` at boundary, `&[u8]` internally | Use raw pointer only at the API boundary if required by surrounding compatibility expectations. |
| byte value argument (`int` in C memchr-style APIs) | `u8` after narrowing | Normalize to byte semantics explicitly. |
| length (`size_t`) | `usize` | Direct mapping. |
| found pointer result | `Option<usize>` internally or `Option<*const u8>` / `*const u8` at boundary | Choose the external form based on how adjacent migrated modules represent C search results. |
| null result / not found | `None` internally, null pointer if raw-pointer API is required | Keep nullability localized at the boundary. |

### Memory Management Notes

- No ownership transfer is involved.
- The implementation must treat the input region as borrowed, read-only memory.
- Any conversion from raw pointer plus length into a slice must be done in a tightly scoped `unsafe` block with explicit assumptions:
  - pointer is valid for reads of `len` bytes,
  - memory region is properly aligned for `u8` access,
  - region does not outlive the call.

### Error Handling Notes

- This function does not introduce Rust error types.
- “Not found” should remain the only absence case.
- Invalid raw pointers are not converted into recoverable errors; they remain caller-contract issues, matching the original C-level expectations.

## Implementation Phases

### Phase 1: Establish module skeleton and signature mapping

- Create the Rust target file for `gnu/memchr.c`.
- Define the Rust function corresponding to `__memchr`.
- Decide the exact boundary signature based on the existing port pattern in this branch:
  - raw-pointer-compatible signature if neighboring translated modules expose C-like APIs,
  - otherwise a crate-internal slice-based helper plus a thin compatibility wrapper.
- Document the safety contract directly on the function if `unsafe` is exposed or internally required.

**Deliverable**:
- Compilable module file with function stub and finalized signature.

### Phase 2: Port core search logic

- Implement the bounded byte search for `__memchr`.
- Convert the search region to `&[u8]` internally when entering from raw pointer and length.
- Use standard-library iteration or slice search mechanics to locate the first matching byte.
- Convert the located position back into the module’s chosen return form:
  - offset/index internally,
  - adjusted pointer or null-equivalent externally if needed.
- Keep the implementation single-purpose and avoid introducing generalized search helpers unless they are required to preserve the one-to-one migration cleanly.

**Deliverable**:
- Working Rust implementation of the original search behavior with scoped `unsafe` only at raw-memory boundaries.

### Phase 3: Add correctness tests

- Add unit tests covering:
  - match at start,
  - match in middle,
  - match at end,
  - no match,
  - zero-length input,
  - repeated target bytes returning the first occurrence.
- Where the boundary API is raw-pointer-based, test via stable byte arrays and compare returned location offsets rather than dereferencing arbitrary pointers.
- Ensure tests validate bounded-search behavior only within the provided length.

**Deliverable**:
- `cargo test` passing for normal and edge cases tied directly to `__memchr`.

### Phase 4: Review semantics and finalize integration

- Verify that the Rust implementation’s byte narrowing and first-match semantics align with the C source behavior.
- Check that null/absence handling is consistent with the surrounding migrated code.
- Confirm there are no unnecessary allocations, copies, or module additions.
- Keep visibility restricted to the minimum needed by the crate.

**Deliverable**:
- Finalized Rust port of `gnu/memchr.c` ready on branch `041-module_gnu_memchr.c_35-rust-port`.