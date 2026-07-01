# Implementation Plan

## Summary

This module migrates the C implementation in `gnu/hash.c` for `hash_string` into a Rust module with equivalent behavior and a minimal surface area. The Rust port should preserve the original hashing algorithm, integer width behavior, and input handling expectations as closely as possible, while replacing raw-pointer-oriented C implementation details with safe Rust string or byte-slice processing.

The implementation approach is to create a single Rust module dedicated to this file-level port, expose one canonical `hash_string` function matching the intended call pattern in the Rust codebase, and keep all logic local to that module. Any duplicated function listing from the source analysis should be treated as a single migration target unless the source file contains distinct variants that require separate private helpers.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s algorithmic complexity and practical throughput.
  - Avoid unnecessary allocation; compute hashes directly from `&str` or `&[u8]`.
  - Preserve deterministic output for identical input across test runs and platforms, subject to the original C integer semantics.
  - Keep the implementation lightweight and inlinable where appropriate.

## Module Mapping

- **C source file**: `gnu/hash.c`
- **Rust target module**: `src/gnu/hash.rs`

Suggested Rust module layout:

- `src/gnu/mod.rs`
  - `pub mod hash;`
- `src/gnu/hash.rs`
  - `pub fn hash_string(...) -> ...`
  - private helper(s) only if needed to preserve exact arithmetic or input normalization

If the current Rust project already has a `gnu` namespace, place the migrated file into the existing structure rather than introducing a new parallel layout.

## Data Model

The analysis lists only repeated `anonymous` data structures and provides no named structs tied to `hash_string`. This suggests the migration target is function-centric and likely does not require durable type definitions beyond standard Rust primitives.

### Data-structure mapping

| C construct | Rust mapping | Notes |
|---|---|---|
| `char *` / `const char *` input | `&str` or `&[u8]` | Prefer `&str` if the source usage is textual and byte iteration over UTF-8 bytes matches the C logic; use `&[u8]` if the original function operates on raw bytes. |
| `unsigned int` / `unsigned long` return or accumulator | `u32` or `u64` | Select exact width based on the C source definition in `gnu/hash.c`; avoid architecture-dependent `usize` unless the original type is truly pointer-sized. |
| anonymous local structs/unions | no direct Rust type unless source proves necessity | Local temporary C layouts should be eliminated if they are not part of the hash interface. |
| null-terminated string traversal | slice iteration | Replace sentinel-based traversal with explicit byte iteration; preserve stopping conditions equivalent to C input contract. |

### Memory management

- No heap allocation should be introduced for the core hashing path.
- Borrowed inputs should be used instead of owned buffers.
- Unsafe code should be avoided unless exact C-compatible pointer behavior is required by surrounding migrated code; if unavoidable, isolate it in the smallest possible helper.

### Error handling

- If the C function assumes valid non-null string input, the Rust public API should encode this through references rather than runtime null checks.
- Do not add new error enums or recovery paths unless the original source exposes multiple failure outcomes. For a pure hash function, a total function signature is preferred.

## Implementation Phases

### Phase 1: Source validation and API definition

- Inspect `gnu/hash.c` and confirm:
  - the exact signature of `hash_string`
  - the hash accumulator type and return type
  - whether the duplicate function listing reflects duplicate declarations, overload-like variants, or analysis duplication
  - whether the function hashes bytes until `'\0'` or consumes a known-length buffer
- Define the Rust function signature to mirror the C behavior with the smallest safe adaptation:
  - `pub fn hash_string(input: &str) -> u32/u64`, or
  - `pub fn hash_string(input: &[u8]) -> u32/u64`
- Settle integer types explicitly to preserve overflow and wraparound semantics.

### Phase 2: Core function migration

- Port the hashing loop from `gnu/hash.c` into `src/gnu/hash.rs`.
- Preserve:
  - operation order
  - seed/initial value
  - multiplication, shifts, xor/add behavior
  - wrapping arithmetic semantics using `wrapping_*` operations where needed
- Replace C character iteration with Rust byte iteration.
- Keep helper usage minimal and local to this file.
- Avoid introducing generalized hashing abstractions or traits.

### Phase 3: Behavioral verification

- Add unit tests in the Rust module or adjacent test module covering:
  - empty input
  - short ASCII strings
  - representative longer strings
  - boundary-sensitive inputs if the C algorithm depends on signed/unsigned byte interpretation
- Where practical, derive expected values from the original C implementation and lock them into tests.
- Confirm duplicate `hash_string` analysis entries do not require multiple exported Rust functions.

### Phase 4: Integration and cleanup

- Wire the module into the project’s existing module tree.
- Ensure any prior call sites are updated to the Rust function signature without adding compatibility layers beyond what is necessary.
- Remove ambiguity around integer widths and document the chosen mapping in code comments only where needed for maintenance.
- Run `cargo test` and fix any behavioral mismatches caused by C-to-Rust conversion details such as byte signedness or overflow handling.