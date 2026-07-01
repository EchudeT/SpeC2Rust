# Implementation Plan: main_root_quote_n_11

## Summary

This module ports the `quote_n_mem` and `quote_n` logic from `quotearg.c` into Rust, preserving the existing quoting behavior and call patterns used by the main cluster. The Rust implementation should stay narrowly focused on migrating the current functions and any directly required internal state from the C source, without introducing broader quoting abstractions beyond what these entry points require.

The technical approach is to translate the C routines into a Rust module that operates on byte slices for memory-based input and provides a string-oriented wrapper for the simpler entry point. Any C global or static quote-slot behavior used by numbered quoting should be represented with Rust-owned storage scoped to the module, using safe standard-library containers where possible. Where the original C logic depends on nullable pointers, explicit lengths, and reusable buffers, the Rust version should map these to `&[u8]`, `&str`, `Vec<u8>`, and `String` as appropriate, while keeping byte-level behavior intact.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time behavior relative to input length for quoting operations.
  - Avoid unnecessary intermediate allocations beyond what is needed to produce the quoted result.
  - Reuse internal per-slot storage where the C implementation reuses buffers, if required by observed behavior.
  - Maintain byte-accurate handling for memory input paths.

## Module Mapping

- **C source file**: `quotearg.c`
- **Rust target module**: `src/main_root_quote_n_11.rs`

### Function Mapping

- `quote_n_mem`
  - **Rust target**: `pub(crate) fn quote_n_mem(n: usize, arg: &[u8]) -> Vec<u8>` or equivalent internal-return type based on surrounding project conventions
  - **Notes**:
    - Preserve explicit numbered-slot semantics.
    - Keep byte-oriented processing to match C memory-input behavior.
    - Return an owned Rust buffer instead of exposing raw internal pointers.

- `quote_n`
  - **Rust target**: `pub(crate) fn quote_n(n: usize, arg: &str) -> String`
    - Implement as a thin wrapper over the memory-based function where practical.
    - Preserve behavior for NUL-free string input expected from the original string entry point.

### Internal Mapping

Only internal helpers directly required by these two functions should be migrated into this module. Do not split out extra utility modules unless a helper is already clearly separated in the source and needed for faithful translation.

## Data Model

The analysis reports unnamed/anonymous data structures only. For this migration, data modeling should be driven strictly by the structures actually touched by `quote_n_mem` and `quote_n`.

### C-to-Rust Mapping Strategy

- **Anonymous structs/unions used only locally in `quotearg.c`**
  - **Rust target**: private `struct` or `enum` definitions inside `src/main_root_quote_n_11.rs`
  - **Notes**:
    - Name them by role from the source usage, not by inventing new conceptual layers.
    - Only migrate fields referenced by the target functions or their direct helpers.
    - Replace union-like state with enums where the distinction is semantic and local.

- **C static/global slot arrays for numbered quote results**
  - **Rust target**: private module-level storage, likely `Vec<Option<...>>` or similar
    - Expand on demand when `n` exceeds current capacity.
    - Use owned Rust buffers rather than raw heap pointers.
    - Keep mutability localized.

- **C `char *` / `unsigned char *` buffers**
  - **Rust target**: `Vec<u8>` for byte-preserving quoted output
    - Do not assume UTF-8 for memory-based paths.
    - Convert to `String` only in the string-oriented wrapper after validating or preserving known text semantics.

- **C string inputs with implicit termination**
  - **Rust target**: `&str` for `quote_n`, `&[u8]` for `quote_n_mem`
    - This separates textual and raw-memory entry points cleanly and matches the original API distinction.

- **C size/count fields**
  - **Rust target**: `usize`

- **C status/error signaling via null pointers or allocation failure paths**
  - **Rust target**: ordinary Rust allocation semantics and, if needed internally, `Option`/`Result`
    - Keep public signatures simple unless surrounding code requires explicit error propagation.
    - Avoid introducing custom error types unless the migrated code path already depends on distinguishable failures.

## Implementation Phases

### Phase 1: Establish direct module skeleton and signatures

- Create `src/main_root_quote_n_11.rs`.
- Add Rust equivalents for `quote_n_mem` and `quote_n` with signatures aligned to the project’s internal calling style.
- Identify the minimum internal state from `quotearg.c` needed for numbered quote slots.
- Define private Rust representations for any anonymous C data touched by these functions.
- Wire the module into the crate without introducing unrelated exports.

### Phase 2: Port core quoting logic and slot storage

- Translate the byte-processing logic used by `quote_n_mem` into safe Rust.
- Implement numbered-slot management with Rust-owned storage that mirrors the C reuse model closely enough for callers that expect stable per-index results.
- Port `quote_n` as the string-entry wrapper over the memory-based implementation.
- Preserve edge-case behavior from C around empty input, length-sensitive quoting, and repeated calls with the same or different slot numbers.

### Phase 3: Memory, ownership, and behavior reconciliation

- Review all pointer-based C behavior and replace it with explicit ownership and borrowing rules.
- Ensure there are no references to temporary buffers escaping function scope.
- Confirm that buffer growth and replacement behavior for slot `n` matches the original intent.
- Reduce unsafe code to zero unless a specific source construct makes it unavoidable; if unavoidable, isolate it and document the invariant narrowly.

### Phase 4: Validation and cleanup

- Add unit tests covering:
  - basic quoting through `quote_n`
  - raw byte input through `quote_n_mem`
  - repeated use of the same slot index
  - use of multiple slot indices
  - empty and special-character inputs
- Compare outputs against the C implementation behavior for representative cases drawn from the migrated functions.
- Remove dead translation artifacts and keep only helpers directly supporting these entry points.