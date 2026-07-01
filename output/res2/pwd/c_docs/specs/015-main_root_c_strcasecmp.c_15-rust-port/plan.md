# Implementation Plan

## Summary
Port the `c-strcasecmp.c` module into an idiomatic Rust implementation that preserves the existing module scope: a case-insensitive string comparison routine equivalent to `c_strcasecmp`. The Rust version should stay narrowly focused on reproducing the current behavior and call shape needed by the project, using standard-library byte/string handling rather than introducing broader text-processing abstractions.

The implementation approach is to migrate the comparison logic into a small Rust module under the main crate, using explicit ASCII-oriented case folding if the original C behavior is bytewise and locale-independent, which is the typical intent of such helper routines. The port should avoid heap allocation, operate over borrowed inputs, and return an integer ordering value compatible with the C routine’s comparison semantics.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74` or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended, since the module scope is limited to a single string-comparison helper and the input provides no evidence requiring external libraries

### Testing
- `cargo test`
- Unit tests should cover:
  - equal strings
  - unequal strings differing only by ASCII case
  - unequal strings with differing lengths
  - empty-string handling
  - first-different-byte ordering behavior
  - non-alphabetic ASCII bytes remaining unchanged during comparison

### Performance Goals
- Match the C helper’s expected lightweight behavior
- No heap allocation during comparison
- Linear time in the shorter input length, with early exit on first difference
- Keep implementation simple enough for inlining by the compiler where appropriate

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `c-strcasecmp.c` | `c_strcasecmp` | `src/main_root/c_strcasecmp.rs` or nearest existing main-cluster module file | `pub(crate) fn c_strcasecmp(...) -> i32` |

### Notes
- Keep the Rust file/module narrowly aligned with the original source file rather than merging into unrelated utilities.
- If the crate already has a central module for the main cluster, expose this function there without creating extra abstraction layers.

## Data Model

This module does not define custom C data structures.

### Type Mapping

| C Type/Concept | Rust Mapping |
|---|---|
| `char *` / `const char *` string input | `&str` if callers already provide valid UTF-8 text; otherwise `&[u8]` if bytewise C semantics must be preserved |
| C integer comparison result | `i32` |

### Representation Decision
- Prefer `&[u8]` internally if the original C function compares raw bytes and applies ASCII case folding manually.
- If the surrounding Rust port already normalizes inputs as valid UTF-8 strings, accept `&str` and compare via `.as_bytes()` internally.
- Do not introduce owned string types (`String`, `CString`) unless required by existing caller migration constraints.

## Implementation Phases

### Phase 1: Inspect and map C behavior
- Review `c-strcasecmp.c` and identify the exact comparison semantics:
  - whether comparison is strictly ASCII-based
  - whether return value is normalized to `-1/0/1` or is the raw byte difference
  - how null terminators and differing lengths affect the result
- Determine the closest Rust function signature based on migrated callers:
  - `&[u8]` for byte-preserving behavior
  - or `&str` with byte-level comparison if all inputs are textual
- Create the destination Rust module file matching the original module boundary.

### Phase 2: Implement the Rust comparison routine
- Port the loop structure directly, preserving comparison order and early-return behavior.
- Implement case folding using standard-library ASCII operations only if the C logic is ASCII-only.
- Ensure no allocation and no unnecessary iterator layering that could obscure exact semantics.
- Return `i32` in a way that matches the C function’s ordering behavior.

### Phase 3: Migrate tests for behavior parity
- Add focused unit tests derived from the observed C behavior.
- Include edge cases for:
  - empty inputs
  - equal inputs with mixed case
  - prefixes and length mismatches
  - punctuation/digits
  - differing alphabetic bytes near the end of input
- Confirm `cargo test` passes and adjust implementation only for semantic parity, not for expanded behavior.

### Phase 4: Integrate with the main cluster
- Update the relevant module declarations so the new Rust item is available where the C helper was used.
- Migrate existing call sites in the same branch/module scope to the Rust function signature with minimal adaptation.
- Remove or isolate the original C implementation from the active build path once Rust usage is complete.