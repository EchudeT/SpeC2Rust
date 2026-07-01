# Implementation Plan: module_src_yy_flex_16

## Summary

This module is a small utility port from `src/c.c` centered on two C helper functions:

- `yy_flex_strncpy`
- `yy_flex_strlen`

The Rust implementation should migrate these routines as narrow, local string/byte utilities without introducing broader abstraction layers. The technical approach is to preserve C-visible behavior where practical while expressing memory handling explicitly through Rust slices and owned buffers.

Because the source function set is limited and no stable named C structs are identified for this module, the Rust port should remain minimal:

- place the port in a single Rust module corresponding to `src/c.c`
- implement the two functions with standard-library byte/string operations
- model C string behavior carefully, especially around null termination, bounded copying, and length scanning
- keep unsafe code avoided unless exact C pointer semantics are required by the surrounding migrated code

The migration goal is behavioral equivalence for existing call sites in the current branch, not API redesign.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.76 or newer

### Primary Dependencies

- Rust standard library only

No third-party crates are recommended because the analyzed scope contains only basic C string helper functionality and does not justify external dependencies.

### Testing

- `cargo test`

Testing should focus on direct parity cases for the two migrated functions, especially edge conditions derived from C behavior.

### Performance Goals

- Preserve linear-time behavior equivalent to the C originals
- Avoid unnecessary heap allocation in the core helper functions
- Use byte-slice traversal and copying paths that compile to efficient standard-library primitives
- Maintain no worse asymptotic performance than the current C implementation for bounded copy and length scan operations

## Module Mapping

### C to Rust File Mapping

- `src/c.c` -> `src/module_src_yy_flex_16.rs`

If the destination crate already uses a `mod.rs` or existing source layout, this module should be added without expanding the structure beyond what is needed for the migrated functions.

### Function Mapping

- `yy_flex_strncpy` -> `pub(crate)` or private Rust function in `src/module_src_yy_flex_16.rs`
- `yy_flex_strlen` -> `pub(crate)` or private Rust function in `src/module_src_yy_flex_16.rs`

Visibility should be set to the minimum needed by migrated callers. Do not make functions public unless the existing Rust crate layout requires cross-module access.

### Behavioral Mapping Notes

- `yy_flex_strlen`
  - Map to length computation over C-style bytes until the first `0` byte
  - Prefer slice-based input if caller migration permits
  - If surrounding migrated code still uses raw pointers, isolate pointer traversal in a small, auditable function boundary

- `yy_flex_strncpy`
  - Map to bounded copy semantics over mutable destination and source bytes
  - Preserve C-style truncation and null-padding expectations as required by existing usage
  - Prefer `&mut [u8]` and `&[u8]` signatures internally, with adapter layers only if needed for call-site compatibility

## Data Model

The analysis lists only anonymous data structures and no named structs directly tied to this module’s exported functionality. For this module, the expected data-model work is therefore limited.

### Data-Structure Mapping

- anonymous -> no standalone Rust struct introduced unless a surrounding migrated caller requires one
- anonymous -> retain data in existing caller-owned buffers/slices
- anonymous -> use `&[u8]` for source C-string-like data where null scanning is needed
- anonymous -> use `&mut [u8]` for destination buffers in bounded copy operations
- anonymous -> use `usize` for lengths and indices
- anonymous -> use `u8` for raw C byte content
- anonymous -> use `Option<usize>` only if an internal helper needs to represent detected terminator position
- anonymous -> no enum needed based on current evidence
- anonymous -> no owned string type required in core logic
- anonymous -> no collection type beyond slices required
- anonymous -> no lifetime-carrying wrapper type required unless imposed by surrounding migration
- anonymous -> no heap-managed struct required
- anonymous -> no replacement record type should be added speculatively

### Memory Management Decisions

- Prefer borrowed slices over raw pointers in the Rust implementation
- Avoid allocation in both helper functions
- If adapting from raw pointers, confine any unsafe conversion or traversal to the smallest possible scope
- Validate destination bounds through slice length rather than manual pointer arithmetic wherever possible

### Error Handling Decisions

The original C helpers likely do not report recoverable errors explicitly. The Rust port should therefore:

- preserve infallible behavior for valid migrated call patterns
- encode bounds through slice sizes rather than runtime recovery mechanisms
- use debug assertions sparingly for internal invariants if helpful during migration
- avoid adding `Result` return types unless a concrete existing call pattern requires it

## Implementation Phases

### Phase 1: Establish Rust Module Skeleton and Signature Strategy

- Create `src/module_src_yy_flex_16.rs`
- Add the module to the crate with the smallest required visibility
- Inspect current and planned Rust call sites for `yy_flex_strncpy` and `yy_flex_strlen`
- Choose final Rust function signatures based on caller migration needs:
  - preferred internal form: slice-based
  - pointer-based wrapper only if existing migrated code cannot yet use slices
- Document expected null-termination assumptions directly in code comments near the function definitions

### Phase 2: Port Core Function Logic

- Implement `yy_flex_strlen`
  - scan bytes until the first `0`
  - return `usize`
  - preserve C-style behavior for empty strings and immediate terminators

- Implement `yy_flex_strncpy`
  - copy at most the destination/requested bound
  - preserve any required null-padding semantics from the C original
  - ensure no out-of-bounds writes through slice-based bounds

- Keep the implementation local and direct
- Avoid introducing utility layers beyond any small helper strictly necessary to share null-scan logic

### Phase 3: Integrate With Existing Module Callers

- Replace uses of the C implementations with the Rust module functions
- Adjust caller code from pointer arithmetic to slice usage where already feasible
- Where raw-pointer interoperability remains during staged migration, keep adaptation code minimal and localized
- Confirm that function visibility is no broader than required after integration

### Phase 4: Validate Behavior With Focused Tests

- Add unit tests for `yy_flex_strlen`
  - empty input
  - single-byte string
  - embedded null terminator
  - longer byte sequences

- Add unit tests for `yy_flex_strncpy`
  - source shorter than bound
  - source equal to bound
  - source longer than bound
  - zero-length destination or copy bound if applicable
  - null-padding behavior expected from the C original

- Run `cargo test`
- Fix any parity gaps found during integration without broadening module scope

## Migration Notes

### C-to-Rust Type Conversions

Expected local mappings for these functions:

- `char *` / `const char *` -> `&mut [u8]` / `&[u8]` where possible
- `int` or C length values -> `usize` after validating non-negative assumptions at the call boundary
- null terminator `'\0'` -> `0u8`

### Unsafe Code Policy

- Default to no unsafe code
- If raw-pointer compatibility is temporarily necessary, isolate unsafe blocks to thin adapters and keep the core algorithms safe
- Do not spread pointer arithmetic through the module

### Scope Control

This plan intentionally limits work to migrating the existing helper functionality from `src/c.c`. It does not include:
- new abstraction layers
- generalized string utility modules
- FFI surfaces
- concurrency-related changes
- broader refactors outside required caller adaptation