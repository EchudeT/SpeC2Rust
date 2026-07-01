# Implementation Plan

## Summary

This module ports the `quotearg.c` functionality needed for `quotearg_colon` and `quotearg_colon_mem` into Rust for the `pwd` project branch `012-main_root_quotearg_colon_12-rust-port`.

The Rust implementation should remain narrowly scoped to the existing C behavior represented by these two functions. The work should migrate the relevant quoting logic from `quotearg.c` into a Rust module that exposes equivalent internal functions for colon-aware argument quoting, including the length-bounded variant.

The technical approach is:

- translate only the code paths required by `quotearg_colon` and `quotearg_colon_mem`
- preserve byte-oriented behavior where the C code operates on raw memory and explicit lengths
- use safe Rust APIs where possible, with careful handling of non-UTF-8 input via byte slices
- keep allocation ownership explicit using `String` or `Vec<u8>` depending on the final quoting path needs
- avoid introducing generalized quoting infrastructure unless it is directly required to support these two functions

## Technical Context

- **Language/Version**: Rust 1.75 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on current evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - preserve linear-time processing over the input buffer
  - avoid unnecessary intermediate allocations beyond what is needed for the returned quoted value
  - support non-NUL-terminated and non-UTF-8 inputs without conversion overhead when possible
  - remain comparable to the C implementation for typical short command-line argument sizes

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` -> `src/quotearg.rs`

### Function Mapping

- `quotearg_colon` -> `quotearg_colon`
- `quotearg_colon_mem` -> `quotearg_colon_mem`

### Rust Module Placement

Keep both migrated functions in a single Rust module corresponding to the source C file:

- `src/quotearg.rs`

If the crate already has a central module file, expose only the minimum needed item declarations there, without splitting the implementation into additional helper modules unless required by compilation constraints.

## Data Model

The analysis reports only anonymous C data structures and does not identify named structs directly tied to these two functions. The plan should therefore map only the data forms actually needed by the migrated functions.

### Data-Structure Mapping

- C `char *` input -> Rust `&str` only if the call site guarantees valid UTF-8
- C `char *` / raw byte buffer -> Rust `&[u8]`
- C `(char *, size_t)` pair -> Rust `&[u8]` or `(&[u8], usize)` during transitional migration
- C returned quoted string buffer -> Rust `String` if output is valid text, otherwise `Vec<u8>` with final conversion only where required by callers
- C `size_t` -> Rust `usize`
- C integer flags or literal punctuation handling -> Rust `u8`, `char`, or small internal enum as needed

### Anonymous Struct Handling

Since the source analysis lists only anonymous structures and gives no evidence that `quotearg_colon` or `quotearg_colon_mem` require public struct migration:

- do not create Rust equivalents for anonymous structs unless they are directly referenced by the extracted implementation
- if internal quote options or state from `quotearg.c` are required, define a minimal private Rust struct limited to the fields actually consumed by these two functions
- prefer private enums for quoting mode selection only if the original C logic cannot be expressed cleanly with constants and helper functions

### Memory Management Notes

- Replace C-managed output buffers with owned Rust return values
- Use borrowed byte slices for inputs to eliminate manual lifetime and deallocation concerns
- Preserve exact byte processing semantics for bounded-memory input
- Avoid unchecked indexing where possible; rely on iterator-based traversal or explicit bounds-checked indexing

### Error Handling Notes

The original C functions are likely pure transformation helpers rather than fallible APIs. The Rust port should therefore:

- return plain owned output values where no explicit failure mode exists
- avoid introducing `Result` unless a true conversion boundary requires it
- keep behavior deterministic for invalid UTF-8 by operating on bytes instead of assuming text input

## Implementation Phases

## Phase 1: Isolate Required Quoting Logic

- Inspect `quotearg.c` and identify the exact internal helpers, constants, and option state used by:
  - `quotearg_colon`
  - `quotearg_colon_mem`
- Extract only the minimum dependency set necessary for these functions
- Create `src/quotearg.rs`
- Define Rust function signatures based on actual caller needs:
  - one convenience entry point corresponding to NUL-terminated/string input
  - one explicit length/byte-slice entry point corresponding to `_mem`
- Decide output type based on call graph expectations, preferring a single owned type across both functions

**Exit criteria**:
- The Rust module compiles with stubbed or partially implemented function bodies
- All direct dependencies from the C functions are identified and listed for migration

## Phase 2: Port Core Byte-Oriented Quoting Behavior

- Implement the quoting transformation for colon-aware escaping/quoting in Rust
- Port the byte-scanning logic needed for bounded-memory input exactly, including handling of:
  - embedded non-UTF-8 bytes
  - explicit input length
  - colon-specific quoting behavior
- Introduce only minimal private helpers needed to avoid duplicated scan/append logic between the two functions
- Ensure allocation growth follows straightforward append semantics using `String` or `Vec<u8>`

**Exit criteria**:
- `quotearg_colon_mem` is fully implemented and drives the core behavior
- `quotearg_colon` is implemented as a thin wrapper over the shared logic
- No unnecessary quoting API surface has been added beyond what this module requires

## Phase 3: Integrate With Existing Callers

- Replace references to the C implementation with the Rust module on this branch
- Adjust call sites to pass byte slices or strings according to the chosen Rust signatures
- Keep changes localized to the migrated file and immediate callers
- Remove any temporary compatibility code introduced during migration if it is no longer needed

**Exit criteria**:
- Existing module call sites compile against the Rust implementation
- The port remains constrained to the `quotearg.c` functionality required for these two functions

## Phase 4: Validate Behavior and Clean Up

- Add unit tests in the Rust module covering:
  - empty input
  - input containing `:`
  - input without special characters
  - bounded-memory input with explicit length shorter than the underlying buffer
  - non-UTF-8 byte input where relevant
- Compare outputs against expected behavior derived from the C implementation for representative cases
- Remove dead code and unused helper paths not required by `quotearg_colon` or `quotearg_colon_mem`
- Run `cargo test`

**Exit criteria**:
- Tests pass under `cargo test`
- The implementation is limited to migrated behavior and does not include unrelated quoting features
- Memory ownership and byte handling are explicit and reviewable