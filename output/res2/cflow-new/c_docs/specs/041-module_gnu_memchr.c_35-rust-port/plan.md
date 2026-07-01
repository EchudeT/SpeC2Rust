# Implementation Plan: module_gnu_memchr.c_35

## Summary

This module ports the behavior of `gnu/memchr.c` into Rust, covering the `__memchr` routine only. The implementation should stay narrowly aligned with the existing C file and avoid introducing broader abstractions or additional utilities.

The Rust approach should implement byte-search logic over raw memory-compatible inputs using standard-library facilities where possible, while preserving C-like semantics for pointer traversal, length-bounded search, and null-result handling. Because the original routine operates on arbitrary memory regions rather than UTF-8 strings or owned containers, the Rust port will likely require a small unsafe boundary for raw pointer access, with internal logic kept minimal and directly traceable to the C function.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates recommended, since the input provides no evidence requiring external dependencies
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve linear scan behavior equivalent to the C implementation
  - Avoid unnecessary allocations and copying
  - Keep the search on raw bytes with minimal abstraction overhead
  - Match C behavior for bounded memory scanning and early return on first match

## Module Mapping

| C Source File | C Function | Rust Target |
|---|---|---|
| `gnu/memchr.c` | `__memchr` | `src/module_gnu_memchr_c_35.rs` with a direct Rust function implementing the same search logic |

### Rust Module Layout

| Rust File | Contents |
|---|---|
| `src/module_gnu_memchr_c_35.rs` | Port of `__memchr` and module-local tests |
| `src/lib.rs` | Exposes the migrated module if the crate is library-based |

### Function Mapping

| C Function | Rust Function Shape | Notes |
|---|---|---|
| `__memchr` | `pub(crate) unsafe fn __memchr(...) -> ...` or equivalent internal function | Use a signature reflecting raw memory access and nullable result semantics |

## Data Model

This module contains no named C structs in the provided analysis.

### Data-structure Mapping

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| Raw memory buffer pointer | `*const u8` / `*mut u8` | Choose mutability to match required return semantics |
| Search byte / character value | `i32` or `u8` internally normalized to `u8` | C `memchr`-style APIs typically accept an integer byte value |
| Length parameter | `usize` | Natural Rust mapping for byte counts |
| Nullable pointer result | Raw pointer with null allowed | Preserve C-style "found/not found" behavior without wrapping in broader abstractions |

### Memory Management Notes

- No ownership transfer should be introduced.
- The Rust implementation must treat the input region as borrowed raw memory only.
- Pointer arithmetic, dereference, and bounds progression should be confined to a small unsafe section.
- The function must not read past the provided byte count.

### Error Handling Notes

- No new error type is needed.
- Preserve the original success/failure signaling model through pointer return values.
- Invalid caller-provided pointers remain a caller contract issue, matching C expectations.

## Implementation Phases

## Phase 1: Create the Rust module skeleton

- Add `src/module_gnu_memchr_c_35.rs`.
- Add the module declaration in `src/lib.rs` only as needed by the existing crate layout.
- Define the Rust function corresponding to `__memchr` with a C-compatible raw-memory-oriented signature.
- Keep the public/internal visibility limited to what the existing project structure requires.

### Deliverables

- Rust module file created
- Function stub for `__memchr`
- Basic crate wiring completed

## Phase 2: Port the search logic directly

- Translate the byte-wise scan from `gnu/memchr.c` into Rust with close structural correspondence.
- Normalize the search value to a byte before comparison, matching C behavior.
- Implement bounded iteration over the memory region without allocation or slice copying.
- Return the pointer to the first matching byte, or null if no match is found.
- Keep unsafe operations localized to pointer reads and pointer offset computation.

### Deliverables

- Complete functional Rust implementation of `__memchr`
- Unsafe boundary minimized and documented in code comments

## Phase 3: Add correctness tests for C-equivalent behavior

- Add unit tests in the same module or standard Rust test layout.
- Cover:
  - match at beginning
  - match in middle
  - match at end
  - no match
  - zero-length search
  - first-match behavior when multiple bytes match
  - non-text binary byte values
- Verify returned positions through pointer offset comparison or equivalent raw-memory checks.

### Deliverables

- `cargo test` coverage for main search cases
- Validation of null result semantics and bounded scanning behavior

## Phase 4: Review for migration fidelity and safety boundaries

- Confirm the final implementation stays limited to the original file and function scope.
- Review the Rust code for any accidental expansion beyond the C module behavior.
- Ensure no unnecessary wrappers, helper subsystems, or extra modules were added.
- Verify the implementation uses standard library only and keeps memory access assumptions explicit.

### Deliverables

- Finalized Rust port aligned to `gnu/memchr.c`
- Clean module ready on branch `041-module_gnu_memchr.c_35-rust-port`