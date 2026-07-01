# Implementation Plan: module_gnu_xmalloc.c_57

## Summary

This module migration covers the allocation-helper routines currently implemented in `gnu/xmalloc.c`: `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, and `xstrdup`.

The Rust implementation should preserve the module’s role as a small set of memory-duplication and zero-initialized allocation helpers, while adapting the API to Rust ownership and allocation semantics. The implementation should rely primarily on `Vec`, `Box<[T]>`, and `String`/`CString`-adjacent byte handling where applicable, avoiding manual heap management unless required by exact call-site compatibility.

The technical approach is to port these functions into a focused Rust module that:
- maps raw allocation behavior to safe standard-library containers,
- handles size computations explicitly to avoid overflow,
- preserves failure behavior through explicit panic/abort-style expectations only if required by surrounding project conventions, otherwise by returning owned values directly where infallible under normal Rust allocation semantics,
- keeps scope limited to the existing functions and their direct migration needs.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C routines’ linear-time copy behavior.
  - Avoid unnecessary intermediate allocations.
  - Preserve zero-initialization only where the original function semantics require it.
  - Keep memory overhead limited to Rust container metadata and required terminator bytes for `ximemdup0`-style behavior.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/xmalloc.c` | `src/gnu/xmalloc.rs` | Direct migration target for the listed helper functions only. |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `xicalloc` | `xicalloc` | Implement as zero-initialized owned allocation using Rust containers; include checked multiplication for element count × element size semantics. |
| `xmemdup` | `xmemdup` | Copy byte slice/input region into newly owned allocation without extra terminator. |
| `ximemdup` | `ximemdup` | Preserve input-size-based duplication semantics for raw memory content; likely shares core logic with `xmemdup`. |
| `ximemdup0` | `ximemdup0` | Duplicate bytes and append one zero byte/terminator-equivalent allocation unit. |
| `xstrdup` | `xstrdup` | Duplicate string data into owned Rust string/byte representation, depending on call-site needs. |

## Data Model

This module does not define standalone C structs in the provided input.

### Data-structure Mapping

| C Data Type / Pattern | Rust Mapping | Notes |
|---|---|---|
| Raw allocated memory block | `Vec<u8>` or `Box<[u8]>` | Use `Vec<u8>` during construction; convert to boxed slice if fixed-size ownership is preferable. |
| Zero-initialized allocation | `vec![0u8; len]` or `Vec<T>::with_capacity` + fill | Prefer direct zero-filled initialization for exact semantics. |
| NUL-terminated duplicated buffer | `Vec<u8>` | Appropriate for `ximemdup0` when preserving explicit terminator byte. |
| Duplicated C string / string bytes | `String` or `Vec<u8>` | Choose the narrowest safe representation compatible with migrated call sites; avoid assuming UTF-8 unless validated by usage. |

## Implementation Phases

### Phase 1: Establish module skeleton and ownership model

- Create `src/gnu/xmalloc.rs`.
- Wire the module into the existing Rust crate structure using standard `mod` declarations only as needed for this file.
- Decide per function whether the Rust signature should operate on:
  - `&[u8]` / `Vec<u8>` for memory buffers,
  - `&str` / `String` for textual duplication,
  - or generic byte-oriented helpers where direct C semantics must be preserved.
- Document and implement a consistent allocation-failure and overflow policy aligned with the port’s existing conventions.

### Phase 2: Port allocation and duplication helpers

- Implement `xicalloc` with explicit checked multiplication before allocation.
- Implement `xmemdup` as owned duplication of a provided byte region.
- Implement `ximemdup` by reusing the same internal copy path where semantics are identical.
- Implement `ximemdup0` by allocating one additional byte and setting the appended byte to zero.
- Implement `xstrdup` using owned duplication of the source string/byte content, selecting `String` only if the source is already valid Rust text.

### Phase 3: Validate semantics and edge cases

- Add unit tests for:
  - empty input,
  - non-empty input,
  - `ximemdup0` terminator presence,
  - zero-sized allocation cases,
  - overflow detection in multiplication-based allocation sizing.
- Verify no function performs uninitialized reads or exposes aliased mutable memory.
- Confirm that each helper returns independently owned memory equivalent to the original C behavior.

### Phase 4: Final integration cleanup

- Replace or connect any existing call sites that still expect the C-style helper behavior.
- Remove duplicated internal logic by consolidating shared copy/allocation code within `src/gnu/xmalloc.rs` only.
- Run `cargo test` and fix signature mismatches or ownership issues introduced by the migration.