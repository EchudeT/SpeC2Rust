# Implementation Plan: main_root

## Summary

Port `sds.c` into a single Rust module that preserves the current SDS string behavior and migration scope without adding new capabilities. The Rust implementation should keep the same operational model as the C code: a heap-backed byte string with tracked length and spare capacity, plus helper operations for creation, duplication, resizing, clearing, concatenation, and copy.

The preferred technical approach is to represent SDS storage with a Rust-owned buffer (`Vec<u8>`) plus explicit metadata sufficient to mirror the C semantics exposed by the listed functions. The implementation should preserve byte-oriented behavior rather than assuming UTF-8 text. Functions that are header-specific in C, such as `sdsHdrSize` and `sdsReqType`, should be translated into internal helpers that compute equivalent sizing/classification decisions needed by the Rust port. Public API shape should stay narrowly aligned to the migrated C functions and their call patterns.

Memory handling should be moved from manual allocation and header-pointer arithmetic to safe ownership and capacity management through the standard library. Cases where the C code mutates length directly or relies on spare capacity should be implemented with explicit invariants around `len <= capacity` and zero-filling where required. Error handling should avoid panic-driven control flow for expected allocation/growth outcomes; use `Result` where growth or copy operations can fail under the chosen API surface, while keeping infallible helpers infallible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve amortized append/growth behavior comparable to the C implementation.
  - Keep byte storage contiguous.
  - Avoid unnecessary reallocations during repeated append/copy operations.
  - Maintain O(1) length/capacity queries and efficient clear/update operations.
  - Avoid extra copies beyond what is required by ownership conversion from C-style APIs.

## Module Mapping

### C to Rust File Mapping

- `sds.c` -> `src/main_root.rs`

If the crate already uses `lib.rs` or `main.rs`, expose the migrated module there with a minimal declaration only:
- `src/lib.rs` or `src/main.rs` -> `mod main_root;`

### Function Mapping

The Rust module should migrate the listed functions with close semantic correspondence:

- `sdsHdrSize` -> internal helper `fn hdr_size(...) -> usize`
- `sdsReqType` -> internal helper `fn req_type(...) -> SdsType`
- `sdsnewlen` -> constructor `fn sdsnewlen(init: Option<&[u8]>, len: usize) -> ...`
- `sdsempty` -> constructor `fn sdsempty() -> ...`
- `sdsnew` -> constructor from bytes/string slice `fn sdsnew(...) -> ...`
- `sdsdup` -> clone-style helper `fn sdsdup(s: &Sds) -> Sds`
- `sdsfree` -> ownership drop path; if retained for API parity, implement as consuming no-op wrapper
- `sdsupdatelen` -> method/helper to recompute logical length from content as needed
- `sdsclear` -> method/helper resetting length to zero without dropping allocation
- `sdsMakeRoomFor` -> reserve/grow helper
- `sdsRemoveFreeSpace` -> shrink-to-fit style helper preserving content
- `sdsAllocSize` -> total allocation size query helper
- `sdsAllocPtr` -> internal/raw allocation reference equivalent, only if required internally
- `sdsIncrLen` -> length-adjustment helper with invariant checks
- `sdsgrowzero` -> grow-and-zero-fill helper
- `sdscatlen` -> append bytes helper
- `sdscat` -> append C-string/byte-slice helper
- `sdscatsds` -> append another SDS helper
- `sdscpylen` -> overwrite from bytes helper
- `sdscpy` -> overwrite from C-string/byte-slice helper

Where direct one-to-one public exposure is unnatural in Rust, keep the names as thin wrappers around idiomatic internal methods so migration remains traceable.

## Data Model

The C module uses several anonymous header layouts to optimize metadata size. In Rust, preserve the functional distinction without reproducing pointer-prefix layout.

### C Struct to Rust Mapping

- `anonymous` header variants (`sdshdr5`, `sdshdr8`, `sdshdr16`, `sdshdr32`, `sdshdr64`-style layouts)
  -> `enum SdsType`
  - Purpose: retain the size-class decision logic currently represented by different C headers.
  - Use: internal classification for allocation/accounting helpers.

- C SDS allocation containing header + flexible char buffer
  -> `struct Sds`
  - Suggested fields:
    - `buf: Vec<u8>`
    - `len: usize`
    - `ty: SdsType` or compute-on-demand if not needed persistently
  - `buf.len()` should represent allocated initialized bytes, while logical string length should be tracked explicitly if spare initialized capacity must coexist with shorter content.
  - If implementation is simplified to use `Vec<u8>` where `buf.len()` equals logical length and `buf.capacity()` equals allocation, then `len` field may be omitted. This is acceptable only if all migrated functions can preserve semantics cleanly, especially `sdsIncrLen`, `sdsgrowzero`, and capacity-sensitive operations.

### Invariants

- Content is byte-oriented; no UTF-8 validity requirement.
- Logical length must never exceed allocated initialized region/capacity according to the chosen representation.
- Growth helpers must preserve existing bytes.
- Zero-growth operations must initialize newly exposed bytes to `0`.
- Clear operation must preserve reusable allocation.
- Copy/append helpers must maintain a trailing C-style nul byte only if the port explicitly chooses to preserve that internal invariant for compatibility; otherwise document that Rust storage is length-delimited and internal nul termination is not relied upon.

## Implementation Phases

## Phase 1: Establish Rust SDS Core

- Create `src/main_root.rs`.
- Define the core `Sds` type and internal `SdsType` classification enum.
- Implement the internal sizing/classification helpers corresponding to:
  - `sdsHdrSize`
  - `sdsReqType`
- Decide and document the internal storage invariant:
  - preferred: `Vec<u8>` with explicit logical length only if needed for parity
  - otherwise `Vec<u8>` using standard length/capacity if all listed functions remain faithful
- Implement basic constructors and ownership operations:
  - `sdsnewlen`
  - `sdsempty`
  - `sdsnew`
  - `sdsdup`
  - `sdsfree`
- Add unit tests for:
  - empty creation
  - creation from provided bytes
  - duplication independence
  - drop/free equivalence

## Phase 2: Length and Capacity Migration

- Implement length/capacity management functions:
  - `sdsupdatelen`
  - `sdsclear`
  - `sdsMakeRoomFor`
  - `sdsRemoveFreeSpace`
  - `sdsAllocSize`
  - `sdsAllocPtr` (internal only unless an exact external caller requires it)
  - `sdsIncrLen`
  - `sdsgrowzero`
- Translate C pointer arithmetic and realloc behavior into safe `Vec<u8>` reserve/shrink/resize operations.
- For any operation that can violate invariants in C if misused, enforce checked behavior in Rust:
  - bounds checks on length increments/decrements
  - explicit zero-fill on grow
  - no exposure of invalid raw pointers unless strictly internal
- Add unit tests for:
  - reserving extra capacity without content loss
  - shrinking excess capacity
  - clearing while retaining allocation
  - incrementing/decrementing logical length within valid range
  - grow-with-zero initialization behavior

## Phase 3: Append and Copy Operations

- Implement data mutation helpers:
  - `sdscatlen`
  - `sdscat`
  - `sdscatsds`
  - `sdscpylen`
  - `sdscpy`
- Preserve byte-level behavior for embedded nul bytes where the original length-based functions allow them.
- Ensure append/copy paths reuse the same internal reserve logic from Phase 2 rather than duplicating growth code.
- Where C distinguishes nul-terminated input from explicit-length input, represent this in Rust with `&[u8]` and thin wrappers for string-like inputs.
- Add unit tests for:
  - append by explicit length
  - append from another SDS
  - overwrite with shorter and longer inputs
  - embedded zero bytes in explicit-length operations
  - capacity reuse across repeated appends/copies

## Phase 4: Parity Review and Cleanup

- Review all migrated functions against the original `sds.c` ordering and behavior to ensure no listed function is omitted.
- Restrict visibility:
  - expose only what external callers need
  - keep header/accounting helpers private
- Remove any temporary abstractions not directly needed for the migrated C surface.
- Finalize module-level documentation describing:
  - byte-oriented semantics
  - ownership model differences from C
  - error/invariant handling choices
- Run full `cargo test` and stabilize naming/comments for maintainable one-file parity with the original module.