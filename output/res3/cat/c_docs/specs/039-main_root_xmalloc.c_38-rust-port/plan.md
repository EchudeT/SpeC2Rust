# Implementation Plan: main_root_xmalloc.c_38

## Summary

This module ports the allocation-helper functionality from `xmalloc.c` into Rust, covering duplicated memory/string creation and zero-initialized allocation behavior for the listed functions: `xcalloc`, `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, and `xstrdup`.

The Rust implementation should translate these helpers into small, focused functions built on the Rust standard library’s owned buffer and string types. The technical approach is to replace raw heap allocation patterns with safe Rust allocations such as `Vec<u8>`, boxed slices, and `String`, while preserving the original module’s operational intent:

- zero-initialized allocation becomes standard zero-filled container creation,
- memory duplication becomes slice-to-owned-buffer copying,
- string duplication becomes owned `String` or byte buffer cloning,
- size/count arithmetic must remain explicit and checked to avoid overflow.

Where the C code would abort on allocation failure, the Rust implementation should align with the target crate’s existing error/termination conventions if already established elsewhere in the port; otherwise, it should prefer explicit checked construction and avoid introducing new recovery systems. The migration should stay local to this module and not expand the project with additional abstractions beyond what is required to replace the existing functions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C module behavior with linear-time copying for duplication helpers.
  - Avoid unnecessary intermediate allocations.
  - Use contiguous owned storage (`Vec<u8>`, `String`, boxed slices where useful).
  - Preserve explicit overflow checks for size calculations before allocation.
  - Keep helper functions small enough for inlining by the compiler where beneficial.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xmalloc.c` | `src/xmalloc.rs` | Direct port of allocation and duplication helpers. |
| `xmalloc.c` exported functions | `src/xmalloc.rs` public functions | Keep function surface limited to migrated helpers only. |

### Function Mapping

| C Function | Rust Function | Return Shape | Migration Notes |
|---|---|---|---|
| `xcalloc` | `pub fn xcalloc(...) -> ...` | Owned zeroed buffer/container | Implement with checked multiplication and zero-filled allocation. |
| `xicalloc` | `pub fn xicalloc(...) -> ...` | Owned zeroed buffer/container | Same migration pattern as `xcalloc`; preserve integer-size semantics through Rust `usize` validation. |
| `xmemdup` | `pub fn xmemdup(src: &[u8]) -> ...` | Owned byte buffer | Replace raw pointer copy with slice copy into owned storage. |
| `ximemdup` | `pub fn ximemdup(src: &[u8]) -> ...` | Owned byte buffer | Same as `xmemdup`, with naming retained only if required by call sites. |
| `ximemdup0` | `pub fn ximemdup0(src: &[u8]) -> ...` | Owned byte buffer with trailing zero | Allocate `len + 1`, copy bytes, append `0`. Use checked length increment. |
| `xstrdup` | `pub fn xstrdup(src: &str) -> String` | `String` | Direct string clone in Rust. If call sites operate on raw bytes, provide byte-oriented internal handling without widening scope. |

## Data Model

This module does not define standalone C structs in the provided input.

### Data-structure Mapping

| C Data Type / Pattern | Rust Type | Notes |
|---|---|---|
| Raw allocated memory block | `Vec<u8>` or `Box<[u8]>` | Prefer `Vec<u8>` during migration unless fixed-size ownership is clearly more natural at call sites. |
| NUL-terminated duplicated memory | `Vec<u8>` | Suitable for `ximemdup0`; explicit trailing zero retained in buffer. |
| Duplicated C string | `String` | Use when the source is valid UTF-8 at the Rust boundary. |
| Allocation size/count values | `usize` | Convert carefully from any narrower/signed C integer inputs. |
| Zero-initialized allocation | `vec![0; len]` | Standard-library replacement for `calloc` semantics. |

### Memory Management and Error Handling Decisions

- Eliminate manual free ownership patterns by returning owned Rust values.
- Replace pointer arithmetic and raw copy operations with slice-based copying.
- Use `checked_mul` and `checked_add` for all allocation-size calculations that correspond to C count/size arithmetic.
- Do not introduce custom allocators or wrapper layers unless already required by surrounding crate code.
- If the crate already centralizes fatal allocation handling, integrate with that existing convention; otherwise, keep behavior local and explicit in these functions without adding a broader error framework.

## Implementation Phases

### Phase 1: Create Rust Module Skeleton and Size-Safe Allocation Helpers

- Add `src/xmalloc.rs`.
- Define the Rust equivalents for the exported helper functions from `xmalloc.c`.
- Establish the minimal internal conventions for:
  - checked count × element-size arithmetic,
  - checked `len + 1` growth for zero-terminated duplication,
  - owned return types using standard library containers.
- Wire the module into the crate using standard Rust module declarations only where needed by existing project structure.

### Phase 2: Port Duplication Functions

- Implement `xmemdup` and `ximemdup` using slice cloning into owned byte storage.
- Implement `ximemdup0` by allocating one extra byte, copying the source bytes, and writing the trailing zero.
- Implement `xstrdup` as owned string duplication.
- Keep naming and visibility aligned with existing call sites to minimize churn outside the module.

### Phase 3: Port Zero-Initialized Allocation Functions

- Implement `xcalloc` with explicit checked multiplication and zero-filled allocation.
- Implement `xicalloc` with the same logic, preserving any distinct caller-facing signature required by the original interface.
- Confirm all conversions into `usize` are explicit and checked where needed to avoid silent truncation or sign issues from C-originated semantics.

### Phase 4: Validation and Replacement of C Usage

- Add targeted unit tests for:
  - empty input duplication,
  - non-empty byte duplication,
  - trailing-zero behavior for `ximemdup0`,
  - string duplication,
  - zero-filled allocation length correctness,
  - overflow-path handling for size arithmetic.
- Update internal call sites, if present in this branch scope, to use the Rust module instead of the C implementation.
- Run `cargo test` and verify the migrated module compiles cleanly without introducing extra support layers.