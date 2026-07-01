# Implementation Plan

## Summary

Port `gnu/basename-lgpl.c` into a focused Rust module that preserves the existing path-component behavior of the C implementation without expanding scope. The Rust work should migrate the two functions `last_component` and `base_len` into a single module with equivalent byte-oriented path scanning logic.

The implementation approach should prefer `std` only. Because the original C code operates on raw path bytes and separator scanning, the Rust port should avoid introducing `std::path::Path`-based normalization semantics that could alter behavior. Instead, implement the logic over `&[u8]` / `&str` views with explicit indexing and boundary checks, keeping ownership simple and avoiding heap allocation where possible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time scanning behavior for both functions.
  - Avoid unnecessary allocation and copying; return slices or lengths derived from the input.
  - Keep branch structure close to the C implementation to reduce behavioral drift during migration.

## Module Mapping

- **C source**
  - `gnu/basename-lgpl.c`

- **Rust target**
  - `src/module_gnu_basename_lgpl.rs`

- **Function mapping**
  - `last_component` → `pub(crate) fn last_component(...)`
  - `base_len` → `pub(crate) fn base_len(...)`

The Rust file should contain only the migrated logic for this module. If the crate already has a module tree for cluster modules, expose this file through the existing `mod` declarations without introducing additional abstraction layers.

## Data Model

This module has no named C structs or persistent data objects.

### C to Rust mapping

- `char *` / `const char *`
  - Rust representation: `&[u8]` for internal scanning
  - Optional thin public wrapper: `&str` if current crate conventions require UTF-8 text inputs, but internal logic should still use `.as_bytes()` to preserve separator behavior exactly

- Pointer return representing a position within the original string
  - Rust representation: slice return such as `&[u8]` or string subslice `&str`, depending on the chosen function signature
  - For length-based logic, use `usize`

- C integer length/count values
  - Rust representation: `usize`

### Memory Management Notes

- No manual allocation or deallocation is needed.
- Returned values should borrow from the input rather than allocate new buffers.
- Index arithmetic must be bounds-checked through standard Rust slicing rules; avoid unchecked indexing unless a direct need is proven during implementation.

### Error Handling Notes

- These functions are pure path scanners and should not introduce `Result` unless required by surrounding crate APIs.
- Invalid UTF-8 should not be silently reinterpreted if the module is exposed as `&str`; prefer byte-slice internals to avoid encoding-dependent behavior.

## Implementation Phases

## Phase 1: Create Rust module skeleton and signatures

- Add `src/module_gnu_basename_lgpl.rs`.
- Define the two migrated functions with crate-appropriate visibility:
  - `last_component`
  - `base_len`
- Choose signatures that match existing project conventions while preserving C behavior:
  - Prefer byte-slice-based internals.
  - If external callers expect strings, add minimal wrappers in the same file rather than creating extra modules.
- Add the module to the crate’s existing `mod` tree.

### Deliverables

- New Rust source file present and wired into compilation.
- Function signatures established with placeholder or initial implementations.
- No new support infrastructure beyond what is needed to compile the module.

## Phase 2: Port scanning logic from C to Rust

- Translate `last_component` directly into index-based scanning over path bytes.
- Translate `base_len` directly into Rust using the same separator and trailing-component rules as the C source.
- Preserve edge-case handling from the original implementation, especially:
  - empty input
  - leading separators
  - repeated separators
  - root-only or separator-only paths
  - trailing separators
- Keep the logic local and explicit rather than replacing it with `Path` helpers, since standard path parsing may normalize differently from the original C behavior.

### Deliverables

- Working Rust implementations of both functions.
- Internal comments only where needed to clarify non-obvious C-to-Rust index translation.
- No behavioral expansion beyond the original file.

## Phase 3: Add focused unit tests for migrated behavior

- Add unit tests alongside the module or in the crate’s existing test layout.
- Cover representative cases tied to the migrated functions:
  - simple file name
  - nested path
  - path with trailing slash
  - multiple adjacent slashes
  - root path
  - empty string
- Verify both returned component selection and computed base length behavior.
- If project conventions allow, derive expected values from the original C semantics rather than from Rust `Path` behavior.

### Deliverables

- `cargo test` passes.
- Tests specifically protect the migrated edge cases of `last_component` and `base_len`.

## Phase 4: Final review and integration cleanup

- Review for exactness of boundary handling and borrowing lifetimes.
- Remove any temporary compatibility code not required by the final module interface.
- Ensure there are no unnecessary allocations, panics from invalid slicing, or API additions beyond the migrated functions.
- Confirm the final file mapping remains limited to the original module scope.

### Deliverables

- Clean Rust module ready on branch `027-module_gnu_basename_lgpl.c_21-rust-port`
- Final implementation aligned with the original C module’s scope and behavior