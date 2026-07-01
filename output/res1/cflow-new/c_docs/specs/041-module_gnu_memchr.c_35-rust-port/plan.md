# Implementation Plan

## Summary

Port `gnu/memchr.c` into an equivalent Rust module that preserves the current single-function scope around `__memchr`. The Rust implementation should mirror the original low-level byte-search behavior closely, using pointer-oriented code where needed to keep semantics and performance aligned with the C source.

The implementation should stay narrowly focused on migrating the existing file and function, without introducing broader utility layers or unrelated abstractions. The preferred approach is a small Rust module exposing a direct equivalent of `__memchr`, implemented with standard library raw-pointer operations and explicit safety boundaries. Any `unsafe` usage should be minimized and documented around pointer traversal, byte comparison, and null/length handling assumptions inherited from the C behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time byte search behavior equivalent to the C implementation.
  - Avoid unnecessary allocation or copying.
  - Keep the implementation suitable for compiler optimization, especially for tight pointer-walking loops.
  - Match C-style memory access patterns closely enough that the Rust port does not introduce avoidable overhead.

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/memchr.c` | `src/gnu/memchr.rs` | Direct migration target for the single-function module. |
| `__memchr` | `gnu::memchr::__memchr` | Keep the function scope and behavior aligned with the original implementation. |

If the crate already organizes GNU-derived code differently, place the Rust file within the existing equivalent module tree rather than introducing a new subsystem.

## Data Model

This module does not define dedicated C structs or custom data containers.

### Type Mapping

| C Type/Concept | Rust Mapping | Notes |
|---|---|---|
| `void *` / `const void *` | `*mut c_void` / `*const c_void` or byte pointers internally | Use raw pointers for interface compatibility; convert internally to `*const u8` / `*mut u8` as needed. |
| `int` search byte | `c_int` at boundary, narrowed to `u8` for comparison | Preserve C-compatible parameter type at the function boundary. |
| `size_t` length | `usize` | Direct Rust equivalent. |
| returned pointer or null | raw pointer with null return | Preserve C null-pointer semantics exactly. |

### Memory and Safety Notes

- The function should not allocate or own memory.
- The Rust port should treat the input region as externally managed memory, matching C semantics.
- Pointer dereferencing must remain inside an `unsafe` block and only occur while iterating within the provided byte count.
- Null handling should follow the original function contract rather than adding Rust-specific recovery paths.

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/gnu/memchr.rs` as the migration target for `gnu/memchr.c`.
- Wire the module into the existing crate module tree using standard Rust module declarations only as needed for this file to compile.
- Define the Rust signature for `__memchr` using C-compatible primitive types from `core::ffi` or `std::ffi` as appropriate.
- Document the intended safety contract for callers, especially around pointer validity and readable range length.

### Phase 2: Port the byte-search logic directly

- Translate the existing `__memchr` implementation into a direct Rust equivalent.
- Keep the control flow close to the C source:
  - convert the searched byte to `u8`,
  - iterate over the memory region up to `n`,
  - compare each byte,
  - return the matching pointer or null if not found.
- Use raw byte pointers internally rather than slices unless slice creation can be proven to preserve the original preconditions without changing behavior.
- Keep all unsafe operations localized to the smallest practical region.
- Avoid adding helper layers unless required to keep the port readable and compilable.

### Phase 3: Validate behavior with focused tests

- Add unit tests covering the migrated behavior of `__memchr`.
- Test the core cases only:
  - match at the beginning,
  - match in the middle,
  - match at the end,
  - no match within length,
  - zero-length search.
- Verify returned pointer position by comparing offsets from the original buffer base.
- Keep tests narrowly tied to the migrated function rather than building a broader memory-utility test framework.

### Phase 4: Review semantic and safety parity

- Confirm the Rust function preserves:
  - null return behavior,
  - byte-wise comparison semantics,
  - no out-of-bounds reads beyond the provided length,
  - no allocation or ownership changes.
- Review the final implementation for unnecessary abstractions or deviations from the original file’s scope.
- Ensure the module is ready on branch `041-module_gnu_memchr.c_35-rust-port` with only the required file/function migration completed.