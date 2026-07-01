# Implementation Plan: module_src_yy_flex_16

## Summary

This module is a small utility port from `src/c.c` centered on two C helper functions:

- `yy_flex_strncpy`
- `yy_flex_strlen`

The Rust implementation should keep the scope narrow and migrate only the behavior required by these functions. The main technical approach is to replace manual C string traversal and copying with safe Rust slice and byte-oriented operations where possible, while preserving C-compatible semantics relevant to the original callers.

Because these functions are string utilities originating from C-style usage, the Rust port should explicitly decide whether the surrounding code expects:

- nul-terminated byte strings, or
- bounded byte buffers with explicit lengths.

The implementation should prefer `&[u8]` / `&mut [u8]` and explicit length handling over introducing broader abstractions. Any required nul-termination behavior should be preserved in the translated function logic rather than hidden behind unrelated helper layers.

## Technical Context

- **Language/Version**: Rust stable, edition 2021
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the original C helper behavior without adding allocation-heavy paths.
  - Keep string length and copy operations linear in input size.
  - Avoid unnecessary UTF-8 validation if the source behavior is byte-string based.
  - Use slice-based copying where semantics allow, while preserving original bounded-copy behavior.

## Module Mapping

### Source File Mapping

- C: `src/c.c`
- Rust: `src/module_src_yy_flex_16.rs`

If the project already centralizes ports differently, this module should still remain a single Rust source file corresponding to this C translation unit fragment, rather than being split further.

### Function Mapping

- `yy_flex_strncpy`
  - Port as a Rust function in `src/module_src_yy_flex_16.rs`
  - Use explicit byte-slice parameters and bounded copy semantics
  - Preserve any destination padding or termination behavior only if present in the current C implementation

- `yy_flex_strlen`
  - Implement as byte-length scan up to the first nul byte if the C function operates on C strings
  - Return a Rust integer type appropriate for indexing and lengths, normally `usize`

## Data Model

The analysis lists only anonymous data structures and no named structs directly tied to the target functions. Since this module appears limited to utility functions, no standalone Rust data model should be introduced unless inspection of `src/c.c` shows these functions directly depend on local struct layouts.

### Data-Structure Mapping

- C `anonymous` -> No Rust type introduced unless required by direct function signature dependencies

### C-to-Rust Representation Notes

- C `char *` / `const char *`
  - Prefer `&mut [u8]` / `&[u8]` for internal Rust APIs when the caller can provide bounded buffers.
  - If the surrounding port layer still uses C-like pointers internally, isolate raw-pointer handling to a minimal boundary and convert immediately to slices under checked bounds.

- C length/count types
  - Map to `usize` unless exact signed behavior is required by existing call sites.

- Memory management
  - No heap ownership model should be added for these helpers.
  - Operate on caller-provided buffers only.

- Error handling
  - If original C functions do not report errors and assume valid inputs, keep Rust APIs narrow and deterministic.
  - For unsafe boundary cases, prefer internal assertions or documented preconditions rather than inventing new recovery behavior.

## Implementation Phases

### Phase 1: Inspect and Freeze C Semantics

- Read the exact implementations of `yy_flex_strncpy` and `yy_flex_strlen` in `src/c.c`.
- Confirm:
  - whether `yy_flex_strlen` scans to the first nul byte,
  - whether `yy_flex_strncpy` matches `strncpy` semantics exactly or only copies until nul,
  - whether destination buffer padding with zero bytes occurs,
  - whether either function relies on signed `char` behavior.
- Identify all local call sites that use these helpers so the Rust signatures match actual usage rather than generic assumptions.
- Record preconditions about buffer sizes and termination expectations.

### Phase 2: Port Functions into a Single Rust Module

- Create `src/module_src_yy_flex_16.rs`.
- Implement `yy_flex_strlen` first as a direct translation using byte iteration.
- Implement `yy_flex_strncpy` second using:
  - indexed loops for exact C behavior when needed, or
  - slice copying only if it does not change padding/termination semantics.
- Keep the implementation self-contained in this file.
- Do not introduce extra utility modules, wrapper types, or generic string frameworks.

### Phase 3: Integrate With Existing Rust Project Structure

- Expose the module from the crate root or existing module tree using standard Rust module declarations only as needed.
- Replace or wire up existing references so the translated functions are called from the Rust port path corresponding to `src/c.c`.
- If raw pointers exist at call boundaries, contain unsafe code to the smallest possible conversion points and keep the helper implementations themselves safe.

### Phase 4: Validate Behavior With Focused Tests

- Add unit tests covering the observed C semantics for both functions.
- Test cases should include:
  - empty input,
  - input containing an early nul byte,
  - exact-fit copy lengths,
  - truncation cases,
  - zero-length copy,
  - any required zero-padding behavior.
- Run `cargo test` and fix mismatches against the original C behavior.
- Keep tests scoped to these two migrated helpers and their direct edge cases only.