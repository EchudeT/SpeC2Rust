# Implementation Plan

## Summary

Port `xmalloc.c` into a single Rust module that preserves the existing allocation-helper surface without adding new behaviors. The Rust implementation should replace C heap-management patterns with safe standard-library allocation APIs wherever possible, while keeping the same operational intent for the existing functions:

- zero-initialized allocation helpers (`xcalloc`, `xicalloc`)
- byte-copy duplication helpers (`xmemdup`, `ximemdup`, `ximemdup0`)
- string duplication helper (`xstrdup`)

The technical approach is to migrate these functions into one Rust source file under the main crate, using owned Rust types instead of raw heap pointers as the primary representation:

- allocation of zeroed buffers via `Vec` initialization
- memory duplication via slice copying into owned buffers
- NUL-terminated duplication needs represented explicitly with byte vectors where trailing `0` matters
- string duplication via `String`/`Box<str>` depending on the surrounding crate conventions

Where the original C code likely assumes infallible “x*” allocation semantics, the Rust port should make that behavior explicit at module boundaries. Unless the broader project already defines a shared fatal-allocation path, use direct standard allocation operations and preserve panic-on-OOM behavior from Rust’s global allocator rather than introducing custom recovery layers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time copy behavior for duplication helpers.
  - Avoid extra allocations beyond the single destination buffer/string required per helper.
  - Use contiguous owned buffers (`Vec<u8>`, `String`, `Box<[u8]>`) with no intermediary copies.
  - Keep zero-fill operations proportional only to requested size, matching the C helper intent.

## Module Mapping

### C to Rust File Mapping

- `xmalloc.c` -> `src/main_root_xmalloc.rs` or the nearest existing main-cluster Rust file location used by this branch

If the branch already has a central `main`-cluster module tree, expose the migrated functions from that existing module root rather than creating extra abstraction layers.

### Function Mapping

- `xcalloc`
  - **C role**: allocate zeroed memory for `n * size`
  - **Rust mapping**: function returning an owned zero-initialized byte buffer or typed container appropriate to call sites
  - **Primary implementation**: checked size multiplication, then `vec![0u8; total]`

- `xicalloc`
  - **C role**: integer-count zeroed allocation variant
  - **Rust mapping**: same allocation strategy as `xcalloc`, with integer parameters mapped to Rust `usize`
  - **Primary implementation**: checked multiplication and zeroed `Vec<u8>`

- `xmemdup`
  - **C role**: duplicate raw memory region
  - **Rust mapping**: duplicate `&[u8]` into `Vec<u8>` or `Box<[u8]>`
  - **Primary implementation**: `src.to_vec()` or equivalent

- `ximemdup`
  - **C role**: duplicate raw memory region using integer-sized length
  - **Rust mapping**: same as `xmemdup`, consolidating onto one internal implementation where signatures permit
  - **Primary implementation**: slice copy into owned buffer

- `ximemdup0`
  - **C role**: duplicate memory and append trailing NUL byte
  - **Rust mapping**: duplicate `&[u8]` into `Vec<u8>` with one additional `0` byte
  - **Primary implementation**: allocate `len + 1`, copy bytes, append `0`

- `xstrdup`
  - **C role**: duplicate C string / string bytes
  - **Rust mapping**: duplicate Rust string input into `String` when UTF-8 text is intended; if the surrounding port still operates on byte strings with NUL semantics, use a byte-oriented representation local to the module
  - **Primary implementation**: `to_owned()` for `&str`, with call-site review to confirm whether any NUL-terminated byte behavior must instead be handled by `ximemdup0`

## Data Model

This module does not define C structs in the provided analysis, so the data-model work is limited to replacing pointer-based return values with owned Rust containers.

### Data Structure Mapping

- `void *` allocated buffer -> `Vec<u8>` or `Box<[u8]>`
- `char *` duplicated string -> `String` when valid UTF-8 text is expected
- `char *` with explicit trailing NUL requirement -> `Vec<u8>`
- C size/count integers -> `usize`

### Memory Management Decisions

- Eliminate manual `malloc`/`calloc`/`free` ownership tracking in favor of Rust owned values with automatic drop.
- Use checked arithmetic (`checked_mul`, `checked_add`) before allocation sizing to prevent silent wraparound that would be possible in C.
- Do not introduce custom allocator wrappers unless already required by adjacent migrated code.
- Keep function outputs ownership-bearing so callers naturally receive exclusive ownership equivalent to newly allocated C memory.

### Error Handling Decisions

- Overflow in requested allocation size should be handled explicitly at the Rust boundary:
  - either panic with a narrow message consistent with infallible `x*` semantics, or
  - route through an already-existing project fatal helper if one exists in this branch
- Do not convert these helpers to `Result`-returning APIs unless existing migrated callers already require that signature change.
- Rely on standard Rust allocation behavior for OOM rather than designing custom recovery.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Core Allocation Semantics

- Create the Rust destination file for the `xmalloc.c` migration in the existing main-cluster layout.
- Define the direct Rust equivalents for:
  - `xcalloc`
  - `xicalloc`
- Standardize parameter types to `usize`.
- Implement checked multiplication before zeroed allocation.
- Decide the concrete return type based on existing caller expectations in the branch:
  - prefer `Vec<u8>` for byte-oriented buffers
  - use `Box<[u8]>` only if ownership without growth better matches current call sites
- Add unit tests for:
  - zero-length allocation
  - normal allocation size
  - overflow detection behavior

## Phase 2: Port Raw Memory Duplication Helpers

- Implement:
  - `xmemdup`
  - `ximemdup`
  - `ximemdup0`
- Internally consolidate duplicate copy logic so `xmemdup` and `ximemdup` share a single byte-slice duplication path.
- For `ximemdup0`, perform checked `len + 1` sizing before allocation and ensure the last byte is `0`.
- Keep APIs byte-oriented rather than introducing typed generic duplication not evidenced by the C source.
- Add unit tests for:
  - exact byte preservation
  - empty input duplication
  - NUL-appended duplication with correct final length
  - embedded zero bytes remaining unchanged except for the appended terminator

## Phase 3: Port String Duplication and Align Call Sites

- Implement `xstrdup` using the narrowest string ownership type that matches existing Rust-side callers:
  - `String` for text usage
  - if callers still require byte-plus-NUL behavior, keep that responsibility in `ximemdup0` rather than broadening `xstrdup`
- Update any immediate call sites in the branch that still assume raw pointer ownership from the C version.
- Ensure no redundant conversions are introduced between `String`, `Vec<u8>`, and slices.
- Add unit tests for:
  - empty string duplication
  - normal ASCII/UTF-8 string duplication
  - independence of returned owned value from source input

## Phase 4: Final Integration Review

- Verify the module remains limited to the migrated `xmalloc.c` responsibilities only.
- Confirm exported function names and visibility align with existing crate usage.
- Run `cargo test` and fix any ownership/signature mismatches in dependent code.
- Review for:
  - unnecessary unsafe code; remove it unless a specific caller contract forces it
  - consistent overflow checks on all computed allocation sizes
  - no extra helper modules or support infrastructure beyond this file’s migration scope