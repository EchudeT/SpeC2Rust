# Implementation Plan

## Summary

Port `xmalloc.c` into a single Rust module that preserves the existing allocation-helper role of the C code without adding broader memory APIs. The Rust implementation should replace manual heap allocation and raw memory duplication logic with standard-library owned containers and explicit checked size handling.

The migrated module should cover these functions only:

- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

Technical approach:

- Use `Vec<u8>`, boxed slices, and `String`/`CString`-adjacent standard representations as the primary ownership model, depending on actual caller needs inside the Rust port.
- Recreate the C semantics of “allocate or terminate/panic on impossible allocation conditions” using explicit overflow checks and standard allocation paths.
- Keep the implementation localized to one Rust source file corresponding to the original C module.
- Migrate callers to typed Rust return values rather than preserving raw pointer interfaces unless other already-ported code in this branch still requires internal raw-byte buffers.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C helper module’s role as thin allocation/duplication wrappers.
  - Avoid unnecessary copies beyond the duplication already required by the original functions.
  - Use contiguous owned buffers (`Vec<u8>`, `Box<[u8]>`, `String>`) with predictable allocation behavior.
  - Preserve linear-time duplication and zero-fill behavior.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xmalloc.c` | `src/main_root_xmalloc.rs` | Single-module port of the allocation helpers. Keep scope limited to the existing exported functions. |

### Function Mapping

| C Function | Rust Function | Return Strategy |
|---|---|---|
| `xcalloc` | `pub(crate) fn xcalloc(count: usize, elem_size: usize) -> Box<[u8]>` | Allocate zeroed buffer of `count * elem_size` bytes after checked multiplication. |
| `xicalloc` | `pub(crate) fn xicalloc(count: usize, elem_size: usize) -> Box<[u8]>` | Same checked/zeroed allocation behavior as `xcalloc`; keep separate function if call sites distinguish interfaces. |
| `xmemdup` | `pub(crate) fn xmemdup(src: &[u8]) -> Box<[u8]>` | Duplicate exact byte slice. |
| `ximemdup` | `pub(crate) fn ximemdup(src: &[u8]) -> Box<[u8]>` | Same duplication semantics retained as a separate wrapper only if needed by callers. |
| `ximemdup0` | `pub(crate) fn ximemdup0(src: &[u8]) -> Box<[u8]>` | Duplicate with one extra trailing zero byte. |
| `xstrdup` | `pub(crate) fn xstrdup(src: &str) -> String` | Owned string duplication via `to_owned()`. |

### Notes on Signature Adaptation

- The Rust port should prefer safe slice and string inputs over raw pointers.
- If immediate caller migration is incomplete, temporary internal helper forms may accept `*const u8` plus length, but that should remain private and be removed once all call sites are converted.
- Keep function naming close to the C names to simplify migration review.

## Data Model

This module does not define C structs, so no struct-for-struct migration is required.

### Type Mapping

| C Concept | Rust Type | Notes |
|---|---|---|
| `void *` allocated buffer | `Box<[u8]>` | Owned heap buffer with fixed length. |
| zero-initialized allocation | `Box<[u8]>` from `vec![0; len].into_boxed_slice()` | Safe zero-fill replacement. |
| memory region + explicit length | `&[u8]` / `Box<[u8]>` | Preferred safe representation. |
| NUL-terminated duplicated memory | `Box<[u8]>` | For `ximemdup0`, final byte explicitly set to `0`. |
| `char *` string duplicate | `String` | Use only for UTF-8 text call sites. |
| C string bytes including terminator, if needed by callers | `Vec<u8>` or `Box<[u8]>` | Only if existing surrounding code still works at byte-buffer level. |

## Implementation Phases

## Phase 1: Create the Rust module skeleton and define allocation semantics

- Add `src/main_root_xmalloc.rs`.
- Implement the checked-size core used by `xcalloc` and `xicalloc`:
  - multiply `count * elem_size` with `checked_mul`
  - treat overflow as fatal to match non-recoverable C helper behavior
- Implement zeroed allocation using standard library containers only.
- Decide module visibility based on current crate usage, defaulting to `pub(crate)`.

### Deliverables

- Rust module file exists.
- `xcalloc` and `xicalloc` compile and use shared checked-allocation logic.
- No third-party allocator or utility crate introduced.

## Phase 2: Port duplication helpers with safe ownership types

- Implement `xmemdup` as exact byte-slice duplication.
- Implement `ximemdup` as a distinct wrapper or alias preserving the original call surface.
- Implement `ximemdup0` by allocating `len + 1` bytes with checked growth, copying source bytes, and appending a trailing zero.
- Implement `xstrdup` using `to_owned()` for UTF-8 string call paths.

### Deliverables

- All six functions from `xmalloc.c` are present in Rust.
- All duplication logic is expressed without unsafe code unless a caller-compatibility bridge temporarily requires it.
- Overflow handling is explicit for both multiplication and `len + 1` extension.

## Phase 3: Migrate call sites in the main cluster to the Rust interfaces

- Update imports and references from the C module usage to `main_root_xmalloc`.
- Replace raw buffer expectations with `Box<[u8]>`, `&[u8]`, or `String` as appropriate.
- Where a caller expects writable allocated memory, convert from boxed slice ownership to `Vec<u8>` only at the call site if mutation is required, rather than broadening this module API.
- Remove any temporary private raw-pointer adapters once all direct users are safe Rust callers.

### Deliverables

- Existing users of `xmalloc.c` helpers compile against the Rust module.
- Interface mismatches are resolved locally at callers, not by expanding this module’s scope.

## Phase 4: Add focused tests and finalize behavior parity

- Add unit tests for:
  - zero-length allocation behavior
  - normal zeroed allocation size
  - overflow detection in `count * elem_size`
  - exact duplication for `xmemdup` / `ximemdup`
  - trailing zero insertion for `ximemdup0`
  - independent owned copy for `xstrdup`
- Verify that returned buffers have the expected length and contents.
- Run `cargo test` and fix any migration regressions.

### Deliverables

- Module-level tests cover each exported function.
- Behavior-important edge cases are locked down.
- Rust implementation is ready on branch `029-main_root_xmalloc.c_29-rust-port`.