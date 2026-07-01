# Implementation Plan

## Summary

Port the `quotearg.c` subset for `quotearg_colon` and `quotearg_colon_mem` into a Rust module that preserves existing calling behavior needed by `pwd` main-cluster usage. The Rust implementation should focus on reproducing the current colon-oriented quoting path only, rather than generalizing the full quoting subsystem.

The implementation approach is:

- migrate the logic in `quotearg_colon` and `quotearg_colon_mem` into a single Rust source module;
- represent C string/byte inputs with Rust `&str` or `&[u8]` at internal boundaries, selecting `&[u8]` where exact byte preservation is required;
- return owned Rust strings/byte buffers rather than relying on C-style static buffers;
- keep memory ownership explicit and local, using standard library containers;
- preserve error-free behavior for arbitrary input bytes by avoiding unnecessary UTF-8 assumptions in the lower-level memory-oriented path.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - match the C implementation’s practical runtime characteristics for short path-like inputs;
  - avoid repeated reallocations where input length is known or can guide capacity reservation;
  - preserve linear-time processing over the input buffer;
  - avoid unnecessary UTF-8 validation in the `_mem`-style path when byte-wise handling is sufficient.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `quotearg_colon` → `pub fn quotearg_colon(input: &str) -> String`
- `quotearg_colon_mem` → `pub fn quotearg_colon_mem(input: &[u8]) -> Vec<u8>` or `String`, depending on final verified output requirements from the existing call sites

### Integration Boundary

- Export only the migrated functions needed by this module plan.
- If `pwd` currently consumes quoted text as UTF-8-visible output, provide a thin `String`-returning wrapper around a byte-preserving internal routine.
- Do not port unrelated quoting entry points from `quotearg.c`.

## Data Model

The analysis reports only anonymous C data structures and does not identify named structs directly used by the target functions. For this migration, the plan should avoid inventing unnecessary Rust types and only introduce concrete Rust structures if the function bodies require retained configuration state.

### Data-Structure Mapping

- `anonymous` C structs not directly referenced by `quotearg_colon` / `quotearg_colon_mem`
  - **Rust mapping:** no direct public equivalent unless required during code extraction
  - **Handling:** inline constants, local variables, or private helper enums/functions

### Expected Rust Internal Representations

If the extracted logic shows quoting mode flags or option tables are needed:

- C flag fields / mode selectors → private Rust `enum` or `struct`
- C pointer + length pairs → `&[u8]`
- C NUL-terminated input strings → `&str` at public API boundary when valid text is expected
- C heap/static output buffers → `String` or `Vec<u8>`

### Memory Management Notes

- Eliminate static shared output buffers common in C quoting helpers.
- Use owned return values to make lifetime and ownership explicit.
- Reserve output capacity conservatively based on input size and escaping rules.
- Keep byte-oriented processing in `_mem` to avoid lossy conversion.

### Error Handling Notes

- The original C-style functions are typically non-fallible for normal quoting transformations.
- Rust APIs should remain non-fallible unless the extracted implementation reveals formatting assumptions that can fail.
- Any internal UTF-8 conversion should be limited to wrapper layers and should avoid `unwrap()` on externally sourced bytes.

## Implementation Phases

## Phase 1: Extract and Map the Target Logic

- Inspect `quotearg.c` and isolate the exact logic dependency chain for:
  - `quotearg_colon`
  - `quotearg_colon_mem`
- Determine whether these functions:
  - directly implement colon escaping; or
  - delegate into shared quoting helpers/options that must be minimally migrated.
- Create `src/quotearg.rs`.
- Add only the smallest set of private helpers/constants required to support these two functions.
- Define the Rust public signatures based on actual `pwd` call-site needs.

## Phase 2: Implement Byte-Preserving Quoting Behavior

- Port the core transformation logic from C into Rust with byte-wise iteration.
- Implement the `_mem` variant first as the canonical internal path.
- Build the non-`_mem` wrapper on top of that implementation.
- Replace C pointer arithmetic and manual buffer writes with:
  - indexed or iterator-based traversal;
  - `Vec<u8>` or `String` accumulation.
- Preserve delimiter and colon-specific escaping behavior exactly as in the C logic.
- Keep helper scope private to the module.

## Phase 3: Integrate and Verify Call-Site Compatibility

- Update the Rust `pwd` main-cluster code on branch `012-main_root_quotearg_colon_12-rust-port` to use the new module path.
- Confirm that returned types fit existing output formatting without adding extra abstraction layers.
- Add unit tests covering:
  - empty input;
  - input without colons;
  - input containing one or more colons;
  - embedded special characters handled by the migrated logic;
  - `_mem` behavior on non-UTF-8 byte input if byte-oriented output is retained.
- Add comparison-oriented tests against expected outputs derived from the current C behavior.

## Phase 4: Clean Up and Finalize Migration Scope

- Remove any temporary compatibility code introduced during extraction.
- Ensure no unused generalized quoting APIs were ported.
- Verify the module remains limited to the existing file/function scope.
- Run `cargo test` and fix any ownership, conversion, or boundary issues discovered during integration.