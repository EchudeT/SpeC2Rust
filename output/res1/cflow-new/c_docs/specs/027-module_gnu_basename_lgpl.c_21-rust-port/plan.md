# Implementation Plan: module_gnu_basename-lgpl.c_21

## Summary

Port `gnu/basename-lgpl.c` into a Rust module that preserves the current file-scoped functionality of `last_component` and `base_len` without adding new behavior. The Rust implementation should mirror the C control flow closely enough to retain path-component parsing semantics, while replacing pointer arithmetic with safe slice- and index-based string/byte traversal.

The implementation approach should prefer the Rust standard library and operate on path data as byte slices where needed, since the original C code is character-oriented and may rely on exact separator handling rather than higher-level path normalization. The migration should focus on preserving existing basename-related behavior, especially around trailing separators and component boundary detection, while keeping ownership simple and avoiding heap allocation unless required by the surrounding crate API.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s linear-time path scanning behavior.
  - Avoid unnecessary allocation and copying during basename/component analysis.
  - Use borrowed slices (`&str` or `&[u8]`) where possible.
  - Keep branch structure straightforward so the Rust port remains close to the original implementation and easy to verify.

## Module Mapping

### Source File Mapping
- `gnu/basename-lgpl.c` → `src/module_gnu_basename_lgpl.rs`

### Function Mapping
- `last_component` → `pub(crate) fn last_component(...) -> ...`
- `base_len` → `pub(crate) fn base_len(...) -> usize`

### Rust Module Placement
- Declare the migrated file as a single Rust module within the existing crate structure.
- Export visibility should remain crate-local unless existing callers require wider exposure.
- Keep both migrated functions together in the same Rust source file to reflect the original C file boundary.

## Data Model

This module does not define dedicated C structs.

### Data-Structure Mapping
- **C pointer-based string access** → Rust borrowed string/byte slice access
- **`char *` / `const char *` inputs** → `&str` when UTF-8 is already guaranteed by surrounding code, otherwise `&[u8]`
- **Pointer offsets / returned interior pointers** → slice indices or subslices referencing the original input
- **C size/count results** → `usize`

### Representation Decision
Because the original implementation is low-level pathname scanning code, the Rust port should internally prefer byte-oriented processing if exact C-like separator traversal is needed. If the surrounding crate already standardizes on `&str`, use `as_bytes()` internally and return either:
- a subslice range/index for internal composition, or
- an `&str` subslice only when boundaries are guaranteed to lie on valid UTF-8 boundaries.

### Memory Management
- No manual memory management is needed.
- Returned values should borrow from the input rather than allocate.
- Avoid constructing intermediate owned `String` values.

### Error Handling
- These functions are expected to be total for the accepted input domain and should not introduce `Result` unless required by the crate’s existing API.
- Handle empty inputs and separator-only inputs explicitly through deterministic return values matching the C behavior.

## Implementation Phases

### Phase 1: Establish Rust Module Skeleton
- Create `src/module_gnu_basename_lgpl.rs`.
- Add the direct Rust equivalents of the two C functions in the same file.
- Choose final function signatures based on current crate usage:
  - prefer borrowed inputs,
  - prefer index/subslice returns over allocation.
- Add the module declaration to the crate root or existing module tree with minimal visibility.

### Phase 2: Port Core Parsing Logic
- Port `last_component` first, preserving the original scan order and separator handling.
- Replace C pointer walking with explicit byte-index traversal.
- Port `base_len` next, reusing the same separator assumptions and component-boundary rules as the C source.
- Keep logic local and direct; do not introduce generalized path utilities beyond what these two functions need.

### Phase 3: Validate Edge Cases with Unit Tests
- Add focused unit tests covering:
  - empty path input,
  - single-component paths,
  - paths with trailing separators,
  - separator-only paths,
  - multi-component paths,
  - cases where `last_component` and `base_len` interact on the same input.
- Use tests to confirm the Rust outputs match the current C semantics rather than platform-native `Path` normalization behavior.

### Phase 4: Integration Review and Cleanup
- Confirm the Rust file fully replaces the migrated C module’s responsibilities in this branch.
- Verify that function visibility and signatures fit existing call sites without introducing wrappers or auxiliary modules.
- Remove any temporary translation artifacts and ensure the code is documented only where needed to clarify C-to-Rust boundary decisions.