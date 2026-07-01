# Implementation Plan

## Summary

Port `gnu/basename-lgpl.c` into a focused Rust module that preserves the existing pathname component logic exposed by `last_component` and `base_len`. The Rust implementation should stay close to the C control flow and boundary handling, especially around repeated separators, root-like prefixes, and terminal separator trimming.

The technical approach is:

- migrate the two functions into a single Rust source module with minimal API surface,
- operate primarily on byte-oriented path data to match C string scanning behavior precisely,
- avoid introducing new abstractions beyond what is needed to represent the original function behavior,
- use safe Rust where practical, with careful indexing and slice handling instead of pointer arithmetic.

## Technical Context

- **Language/Version**: Rust stable, edition 2021
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates recommended based on current module scope
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time scans over the input path data
  - Avoid unnecessary allocation in core logic
  - Keep behavior close to C by using borrowed slices/indices rather than constructing intermediate owned strings
  - Maintain negligible overhead relative to the original small utility functions

## Module Mapping

### C to Rust File Mapping

- `gnu/basename-lgpl.c` → `src/module_gnu_basename_lgpl.rs`

### Function Mapping

- `last_component` → `pub(crate) fn last_component(...) -> ...`
- `base_len` → `pub(crate) fn base_len(...) -> usize`

### Rust Module Placement

- Expose the migrated logic through one internal module only
- Register the module from the crate root using standard Rust module declarations
- Keep helper logic local to this module unless direct reuse is required by existing project structure

## Data Model

This module has no dedicated C structs to translate.

### Function-Level Representation Mapping

- C `char *` / `const char *` path input
  - Rust borrowed path data:
    - prefer `&[u8]` for exact separator scanning and C-like byte semantics, or
    - `&str` only if the surrounding project already standardizes textual path helpers and byte indexing can still be handled safely
- C pointer return into the original string
  - Rust slice return such as `&[u8]`, or
  - Rust index/offset return if that better matches existing crate integration
- C `size_t`
  - Rust `usize`

### Memory Management Notes

- No heap ownership is required for the core migration
- Return borrowed views or offsets tied to the input lifetime
- Replace pointer traversal with explicit index-based scans to preserve safety and avoid lifetime ambiguity

### Error Handling Notes

- The original functions are utility-style path scanners and do not appear to use structured error returns
- Preserve this model in Rust by using total functions over borrowed input
- Handle edge cases through deterministic return values rather than `Result` unless an existing crate API forces a different signature

## Implementation Phases

### Phase 1: Module Skeleton and Signature Selection

- Create `src/module_gnu_basename_lgpl.rs`
- Add the Rust module declaration in the crate root
- Choose the narrowest Rust signatures that preserve C behavior:
  - decide whether project integration is better served by returning a borrowed slice or a start index for `last_component`
  - define `base_len` against the same input representation for consistency
- Document separator assumptions and any platform-specific behavior mirrored from the original C file

### Phase 2: Port Core Scanning Logic

- Port `last_component` first, translating pointer walks into bounded index scans
- Port `base_len` second, reusing the same separator and trailing-component rules
- Keep branch structure close to the C implementation to reduce behavioral drift
- Ensure no allocations are introduced in the core path-processing logic

### Phase 3: Edge-Case Validation Tests

- Add unit tests covering:
  - empty input
  - input with no separator
  - single separator and repeated separators
  - trailing separators
  - root-style paths and separator-only inputs
  - ordinary multi-component paths
- Add paired tests for `last_component` and `base_len` so both are validated against the same boundary cases
- Confirm all tests pass with `cargo test`

### Phase 4: Integration Cleanup

- Align naming, visibility, and documentation comments with the rest of the Rust crate
- Remove any temporary compatibility scaffolding used during translation
- Verify the module remains limited to the migrated file and functions without introducing extra facilities