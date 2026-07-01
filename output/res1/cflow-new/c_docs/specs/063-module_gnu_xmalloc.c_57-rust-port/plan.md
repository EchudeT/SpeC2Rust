# Implementation Plan: module_gnu_xmalloc.c_57

## Summary

This module ports the allocation-helper logic from `gnu/xmalloc.c` into a focused Rust module that preserves the existing file/function scope without adding broader memory-management abstractions. The Rust implementation will replace C allocation patterns with standard-library owned containers and slices, while keeping function-level behavior aligned with the source module’s responsibilities:

- duplicate byte regions into owned memory,
- duplicate strings into owned `String`,
- provide zero-initialized allocation for element/count pairs,
- preserve failure behavior through explicit checked size calculations and panic-on-unrecoverable-allocation semantics where appropriate for `x*` helpers.

The technical approach is to migrate each C function into a Rust function with equivalent intent, using `Vec<u8>`, boxed slices, and `String` as the primary owned representations. Integer overflow checks that are implicit or manually guarded in C will be made explicit with `checked_mul` and length validation before allocation.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C-module expectations for linear-time duplication and initialization.
  - Avoid unnecessary intermediate buffers during memory duplication.
  - Keep allocation count minimal: one destination allocation per duplication operation.
  - Ensure zero-fill paths use standard-library initialization mechanisms with predictable O(n) cost.

## Module Mapping

### Source to Target

- **C source file**: `gnu/xmalloc.c`
- **Rust target file**: `src/module_gnu_xmalloc.rs`

### Function Mapping

- `xicalloc`
  → `pub fn xicalloc(count: usize, elem_size: usize) -> Box<[u8]>`
  Technical note: use checked multiplication to compute total byte length, then allocate zeroed storage via `vec![0u8; total].into_boxed_slice()`.

- `xmemdup`
  → `pub fn xmemdup(src: &[u8], size: usize) -> Box<[u8]>`
  Technical note: copy exactly `size` bytes from the provided source region after validating `size <= src.len()`.

- `ximemdup`
  → `pub fn ximemdup(src: &[u8]) -> Box<[u8]>`
  Technical note: provide the “duplicate full input region” variant using direct slice duplication.

- `ximemdup0`
  → `pub fn ximemdup0(src: &[u8]) -> Box<[u8]>`
  Technical note: allocate `src.len() + 1`, copy bytes, append trailing zero byte.

- `xstrdup`
  → `pub fn xstrdup(src: &str) -> String`
  Technical note: return an owned duplicate with exact textual content using `to_owned()`.

## Data Model

This module does not define C structs or persistent record types.

### Data-structure Mapping

- C raw memory region (`void *`, `char *`, byte buffers)
  → Rust `&[u8]`, `Box<[u8]>`, or `Vec<u8>` during construction

- C NUL-terminated string input/output (`char *`)
  → Rust `&str` for validated UTF-8 string inputs and `String` for owned duplicates

- C allocation size values (`size_t`)
  → Rust `usize`

### Memory Management Decisions

- Replace manual allocation/free ownership with Rust owned containers.
- Use `Box<[u8]>` for fixed-size returned byte buffers to reflect post-allocation immutability and simple ownership transfer.
- Use explicit overflow checks before capacity/length calculations.
- Treat impossible allocation/size conditions as immediate failures consistent with `x*` helper semantics, implemented via panic on violated invariants or allocation failure propagated by the standard allocator behavior.

## Implementation Phases

### Phase 1: Create Rust module skeleton and port straightforward duplication functions

- Add `src/module_gnu_xmalloc.rs`.
- Port `xstrdup` using `String::to_owned`.
- Port `ximemdup` as full-slice duplication into owned bytes.
- Port `xmemdup` with explicit size-bound validation against the input slice.
- Export only the migrated functions needed for this module; do not introduce extra helper APIs unless required to share exact overflow/copy logic.

### Phase 2: Port allocation-sensitive functions with explicit size checks

- Implement `xicalloc` using:
  - `checked_mul(count, elem_size)` for total-byte computation,
  - zero-initialized byte allocation,
  - owned boxed-slice return.
- Implement `ximemdup0` using:
  - `checked_add(src.len(), 1)` for terminator space,
  - one allocation,
  - copy plus appended zero byte.
- Keep failure handling local to these functions; do not add generalized allocator wrappers beyond what the file needs.

### Phase 3: Align behavior and edge-case handling with the C module

- Review zero-length inputs for all functions and preserve sensible compatibility:
  - zero-length duplication returns empty owned storage,
  - zero-initialized allocation with zero total size returns empty owned storage,
  - terminator-appending duplication of empty input returns a one-byte zeroed buffer.
- Verify exact returned lengths and trailing-zero behavior.
- Confirm no unnecessary reallocations or intermediate clones remain.

### Phase 4: Add focused unit tests for migrated functions

- Add `cargo test` coverage for:
  - normal byte duplication,
  - partial duplication via `xmemdup`,
  - full duplication via `ximemdup`,
  - zero-appended duplication via `ximemdup0`,
  - string duplication via `xstrdup`,
  - zero-sized and empty-input cases,
  - checked-overflow scenarios where size arithmetic must fail.
- Keep tests module-local and limited to the ported functions.