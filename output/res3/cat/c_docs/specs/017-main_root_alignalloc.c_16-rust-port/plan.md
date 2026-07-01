# Implementation Plan

## Summary

This module ports the C allocation helper in `alignalloc.c` into a Rust module that preserves the existing allocation behavior and call structure with minimal expansion. The Rust implementation should focus on reproducing the current aligned-allocation workflow:

- compute alignment-adjusted addresses,
- keep access to the original malloc-like base pointer needed for deallocation,
- expose allocation and free operations with equivalent semantics.

Because the source module is fundamentally about raw memory allocation and pointer arithmetic, the Rust port will likely require a small, isolated unsafe core built on `std::alloc` and raw pointers. The implementation should keep the unsafe surface narrow and place all address math in dedicated helper functions corresponding directly to the C functions:

- `align_down`
- `address_of_pointer_to_malloced`
- `alignalloc`
- `alignfree`

The technical approach is to migrate the existing logic into one Rust source file with function-level correspondence to the C module, preserving behavior rather than redesigning the allocator interface.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only, especially:
  - `std::alloc`
  - `std::mem`
  - `std::ptr`
  - `std::num` only if needed for checked arithmetic
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain constant-time alignment calculations.
  - Avoid extra heap allocations beyond the underlying aligned allocation strategy.
  - Preserve low-overhead pointer arithmetic comparable to the C implementation.
  - Minimize abstraction overhead by keeping the implementation close to the original control flow.

## Module Mapping

### Source File Mapping

- `alignalloc.c` -> `src/main_root_alignalloc.rs`

If the project already keeps module code under a different existing file path, this module should be placed in the closest matching existing Rust location without introducing extra helper modules. The migration should remain a one-file port unless the surrounding crate structure already dictates another file name.

### Function Mapping

- `align_down` -> `fn align_down(...) -> ...`
- `address_of_pointer_to_malloced` -> `fn address_of_pointer_to_malloced(...) -> ...`
- `alignalloc` -> `pub(crate) unsafe fn alignalloc(...) -> ...`
- `alignfree` -> `pub(crate) unsafe fn alignfree(...)`

Function names may remain close to the C names to simplify review and traceability during migration. Signature refinement should be limited to Rust type correctness and safety requirements.

## Data Model

This module analysis shows no named C structs or custom data structures. The port should therefore remain pointer-oriented.

### Data-Structure Mapping

- No C struct/union definitions -> no Rust struct/enum required

### Primitive and Pointer Mapping

Expected low-level mappings:

- C integer types used for sizes/alignments -> `usize`
- C raw addresses/pointers -> `*mut u8`, `*const u8`, or `*mut core::ffi::c_void` only where required by surrounding interfaces
- Stored original allocation pointer adjacent to aligned memory -> raw pointer slot represented through pointer casts and reads/writes

### Memory Layout Notes

The C code likely stores the original allocated pointer in bytes immediately before or near the aligned pointer returned to callers. The Rust port should preserve that layout exactly if that is how `alignfree` recovers the base pointer. This means:

- layout math must use checked size computations where possible,
- pointer offset calculations must be centralized,
- reads and writes of the saved base pointer must use correct alignment assumptions or explicitly unaligned operations if the C layout requires them.

No additional Rust-owned container type should be introduced unless strictly necessary to express the existing layout.

## Implementation Phases

### Phase 1: Port helper address calculations

Goals:

- Create the Rust module file for the migrated code.
- Implement direct Rust equivalents for:
  - `align_down`
  - `address_of_pointer_to_malloced`
- Convert all arithmetic to `usize`-based operations.
- Use explicit checked arithmetic for size additions and pointer-offset calculations where overflow is possible.

Key decisions:

- Keep helper functions private unless external use already exists in the crate.
- Document all invariants for unsafe blocks, especially:
  - alignment values expected to be nonzero,
  - pointer provenance assumptions,
  - valid address range assumptions.

Exit criteria:

- Helper functions compile.
- Unit tests cover representative alignment computations and pointer-slot address derivation logic.

### Phase 2: Implement allocation path

Goals:

- Port `alignalloc` using `std::alloc` or the nearest existing project allocation pattern if the crate already wraps allocation centrally.
- Reproduce the C allocation formula exactly:
  - request enough space for payload,
  - include extra bytes for alignment adjustment,
  - include storage for the original allocated pointer.
- Store the original allocation pointer in the reserved metadata location used by the C design.
- Return the aligned payload pointer with equivalent null/error behavior.

Key decisions:

- Preserve C-like failure behavior by returning a null raw pointer or equivalent existing internal convention, rather than introducing new error abstractions.
- Keep the function unsafe if it returns or manipulates raw memory directly.
- Avoid replacing the design with `Layout`-only aligned allocation if doing so would remove the original-pointer bookkeeping semantics present in the C implementation.

Exit criteria:

- Allocation path compiles and passes tests for:
  - valid alignments,
  - returned pointer alignment,
  - zero-size behavior matching intended C semantics,
  - null/failure path handling as far as can be tested safely.

### Phase 3: Implement deallocation path

Goals:

- Port `alignfree`.
- Recover the original allocation pointer from the metadata location defined by `address_of_pointer_to_malloced`.
- Free using the same allocator family used by `alignalloc`.
- Ensure null-pointer handling matches the C behavior.

Key decisions:

- Deallocation must use the exact base pointer and layout assumptions established in Phase 2.
- Keep all metadata recovery logic localized so allocation/deallocation invariants are easy to audit together.

Exit criteria:

- Round-trip tests validate:
  - allocate then free for several sizes and alignments,
  - repeated usage does not corrupt metadata,
  - null free behavior is tolerated if the C implementation permits it.

### Phase 4: Validation and integration cleanup

Goals:

- Review all unsafe blocks for minimum scope and explicit invariants.
- Verify function naming, visibility, and placement match the existing crate structure.
- Add focused tests for edge conditions that are directly implied by the C implementation:
  - small alignments,
  - larger power-of-two alignments,
  - metadata placement around alignment boundaries.

Key decisions:

- Do not add extra allocation APIs, wrappers, or broader abstractions.
- Do not generalize beyond the original module’s role.

Exit criteria:

- `cargo test` passes.
- The Rust module cleanly replaces the C file’s responsibilities within the branch `017-main_root_alignalloc.c_16-rust-port`.
- The implementation remains limited to the original file’s functions and required memory-management behavior.