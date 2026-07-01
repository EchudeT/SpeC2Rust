# Implementation Plan: module_gnu_xmalloc.c_57

## Summary

This module ports the allocation-helper routines from `gnu/xmalloc.c` into Rust with behavior aligned to the existing C call sites, while using Rust-owned memory types and explicit failure signaling instead of raw allocation APIs. The scope is limited to the functions present in this module:

- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

The Rust implementation should preserve the original operational intent of these helpers:

- zero-initialized allocation
- byte-slice duplication
- typed memory duplication where applicable
- null-terminated duplication where required
- string duplication

The technical approach is to replace C heap allocation and pointer-returning interfaces with narrow Rust functions built on `Vec<u8>`, `Box<[u8]>`, and `String`/`CString`-appropriate equivalents only where needed by actual migrated call sites. Allocation overflow and invalid size conversions must be checked explicitly. The module should not introduce broader allocation frameworks or generalized abstractions beyond what is necessary to migrate this file.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep allocation count equivalent to the C helpers for the same operation.
  - Avoid unnecessary intermediate buffers during duplication.
  - Preserve linear-time copying behavior for memory and string duplication.
  - Ensure overflow checks are constant-time and performed before allocation.

## Module Mapping

### Source Mapping

- **C source**: `gnu/xmalloc.c`
- **Rust target**: `src/gnu/xmalloc.rs`

### Function Mapping

- `xicalloc`
  - Migrate to a Rust helper that computes `count * size` with checked multiplication and returns a zero-initialized owned byte buffer.
  - Preferred Rust shape: internal function returning `Box<[u8]>` or `Vec<u8>` based on caller needs.
- `xmemdup`
  - Migrate to a byte-slice duplication helper returning owned copied bytes.
  - Preferred Rust shape: `fn xmemdup(src: &[u8]) -> Box<[u8]>` or `Vec<u8>`.
- `ximemdup`
  - Migrate as a size-checked duplication routine for raw byte content; if typed duplication is needed by migrated callers, keep it internal and minimal.
  - Avoid unsafe typed copying unless required by dependent ports.
- `ximemdup0`
  - Migrate to a duplication helper that appends one trailing zero byte after copying input bytes.
  - Preferred Rust shape: `fn ximemdup0(src: &[u8]) -> Box<[u8]>` or `Vec<u8>`.
- `xstrdup`
  - Migrate to Rust string duplication using `String` when input is valid UTF-8 in the translated code path.
  - If call sites operate on raw C-style bytes, use byte-preserving duplication with explicit trailing NUL only where required by those call sites, not as a general default.

### Module Placement

Keep the implementation in a single Rust module corresponding to the original file. Do not split helpers into extra allocation or utility modules unless required by the crate’s existing layout.

## Data Model

This module has no declared C structs to port.

### Data Mapping

- Raw allocated memory in C
  - Rust mapping: `Vec<u8>` or `Box<[u8]>`
- C string duplication result
  - Rust mapping: `String` for UTF-8 text paths
  - Alternative only if required by migrated call sites: `Vec<u8>` with trailing `0`
- Size/count parameters (`size_t`)
  - Rust mapping: `usize`
- Signed size-like parameters, if present in surrounding call sites
  - Rust mapping: checked conversion to `usize` before allocation or copy

### Memory Management Decisions

- Use Rust ownership to replace manual `malloc`/`calloc`/copy/free` patterns.
- Perform checked arithmetic before allocation:
  - `count.checked_mul(size)`
  - `len.checked_add(1)` for `ximemdup0`
- Prefer safe slice copying APIs:
  - `to_vec`
  - `copy_from_slice`
- Avoid exposing raw pointers in this module unless demanded by unchanged surrounding interfaces during incremental migration.

### Error Handling Decisions

Because the C module is allocation-oriented and traditionally abort-oriented in GNU style, the Rust port should choose one consistent internal policy based on surrounding crate conventions:

- Preferred for incremental migration: return `Result<_, AllocationError>` or crate-local equivalent if the Rust branch already uses fallible APIs.
- If the crate is being translated with infallible helper semantics, centralize failure through `panic!` only at the allocation boundary and keep the behavior consistent across all functions.

Do not introduce custom recovery layers or broad error hierarchies solely for this module.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and size-safe allocation helpers

### Goals
- Add `src/gnu/xmalloc.rs`
- Establish the minimal public/internal function signatures required by current migration order
- Implement checked size arithmetic before any allocation

### Tasks
- Create the Rust file matching `gnu/xmalloc.c`
- Define the narrow helper interface for:
  - zeroed allocation by total size
  - byte duplication
  - byte duplication with trailing zero
  - string duplication
- Implement shared checked helpers for:
  - multiplication overflow
  - addition overflow for terminator allocation
- Decide and apply one return-type convention consistently across the module:
  - `Vec<u8>`/`Box<[u8]>` for memory helpers
  - `String` for textual duplication where valid

### Completion Criteria
- Module compiles
- Overflow-safe allocation size computation is in place
- No unsafe code is introduced unless a migrated call site strictly requires it

## Phase 2: Port the duplication functions directly from the C behaviors

### Goals
- Implement the five functions in Rust
- Preserve per-function allocation and copy semantics without adding broader abstractions

### Tasks
- Implement `xicalloc`
  - allocate zeroed memory using `vec![0; total]` or equivalent
- Implement `xmemdup`
  - duplicate exact byte length from source slice
- Implement `ximemdup`
  - map to the same underlying byte-copy path with any required checked conversions
- Implement `ximemdup0`
  - allocate `len + 1`, copy bytes, set final byte to zero
- Implement `xstrdup`
  - duplicate owned string content from borrowed input
  - if non-UTF-8 paths are required by actual callers, keep handling byte-based and local to this function’s translated interface

### Completion Criteria
- All functions from `gnu/xmalloc.c` are represented in `src/gnu/xmalloc.rs`
- Copy behavior is covered without raw manual memory management
- Function behavior matches expected ownership and terminator semantics

## Phase 3: Integrate with callers and normalize interfaces

### Goals
- Connect the new module to the crate paths used by the translated code
- Adjust only the directly dependent call sites as needed to consume Rust-owned return values

### Tasks
- Export the module through the existing crate/module tree
- Update migrated callers to use slices, `Vec<u8>`, `Box<[u8]>`, or `String` instead of C pointers
- Remove any temporary interface mismatches introduced during porting
- Keep changes constrained to this module and immediate consumers

### Completion Criteria
- The module is reachable through the crate structure
- Immediate translated call sites compile against the Rust interfaces
- No extra compatibility layer is added beyond what those callers require

## Phase 4: Add focused tests for allocation, copy, and boundary conditions

### Goals
- Validate semantic equivalence for the migrated helpers
- Confirm safe behavior at edge conditions

### Tasks
- Add unit tests for:
  - `xicalloc` returns zero-filled memory of expected length
  - `xmemdup` returns an exact copied buffer
  - `ximemdup` duplicates correctly for representative sizes
  - `ximemdup0` appends exactly one trailing zero and preserves source bytes
  - `xstrdup` returns equal string content with independent ownership
- Add boundary tests for:
  - zero-length inputs
  - overflow in `count * size`
  - overflow in `len + 1`
- Run `cargo test` and fix interface or ownership issues revealed by tests

### Completion Criteria
- Unit tests pass under `cargo test`
- Boundary conditions are covered for all allocation-size computations
- The module’s memory ownership behavior is validated and stable