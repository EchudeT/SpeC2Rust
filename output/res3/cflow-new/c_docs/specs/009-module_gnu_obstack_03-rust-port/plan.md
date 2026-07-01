# Implementation Plan

## Summary

Port `gnu/obstack.c` into a single Rust module that preserves the existing obstack allocation behavior and migration boundaries of the C source. The Rust implementation should mirror the current chunk-based allocation model, retain the same function-level responsibilities, and translate pointer-oriented state updates into explicit Rust structures plus narrowly scoped unsafe code where raw memory manipulation is unavoidable.

The implementation approach is to:
- keep the port centered on one Rust source module corresponding to `gnu/obstack.c`,
- migrate the current allocator/chunk lifecycle functions in place,
- represent obstack state and chunks with Rust structs that preserve C layout intent where needed,
- use standard library allocation primitives and raw pointers for chunk management,
- keep error handling aligned with the existing low-level behavior rather than introducing new abstractions or capabilities.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve amortized chunk-allocation behavior of the C implementation,
  - avoid extra per-allocation indirection beyond what the C code requires,
  - keep memory copies and chunk transitions equivalent to the original logic,
  - maintain constant-time checks for ownership/allocation-region queries where the original design allows it.

## Module Mapping

### C to Rust File Mapping

- `gnu/obstack.c` → `src/module_gnu_obstack_03.rs`

### Function Mapping

- `call_chunkfun` → `call_chunkfun`
- `call_freefun` → `call_freefun`
- `_obstack_begin_worker` → `obstack_begin_worker`
- `_obstack_begin` → `obstack_begin`
- `_obstack_begin_1` → `obstack_begin_1`
- `_obstack_newchunk` → `obstack_newchunk`
- `_obstack_allocated_p` → `obstack_allocated_p`
- `_obstack_free` → `obstack_free`
- `_obstack_memory_used` → `obstack_memory_used`

### Mapping Notes

- Remove the leading underscore from exported/internal Rust function names unless an existing project API requires exact preservation.
- Keep all migrated functions in the same Rust module initially; do not split allocator helpers into extra files.
- Preserve the original call order and helper relationships from the C file so review can be done function-by-function against the source.

## Data Model

Because the analysis only reports anonymous C data structures, the Rust port should derive its data model directly from the concrete structs/unions/macros present in `gnu/obstack.c` and keep the mapping minimal.

### Planned Structure Mapping

- **C obstack state struct** → `struct Obstack`
  - stores chunk pointers, object base/next-free pointers, chunk size, alignment mask, and allocator/deallocator callbacks or equivalents.
  - fields that are manipulated as raw addresses in C should remain raw pointers in Rust where required for faithful behavior.

- **C chunk/header struct** → `struct ObstackChunk`
  - stores previous-chunk link and chunk limit metadata.
  - payload area should be treated as trailing allocated memory managed through pointer arithmetic rather than modeled as a Rust slice field.

- **C allocator function pointer forms** → function pointer fields in `Obstack`
  - one form for chunk allocation,
  - one form for chunk release,
  - if the C source includes “extra arg” variants, represent them explicitly rather than generalizing beyond the source.

- **C integer flags / bitfield-like state** → plain integer or boolean fields in `Obstack`
  - preserve width/sign intent only where it affects pointer math or layout-sensitive behavior.

### Rust Representation Guidance

- Use `*mut u8` / `*const u8` for byte-level memory positions corresponding to:
  - current object base,
  - next free byte,
  - chunk limit,
  - raw chunk payload traversal.
- Use `NonNull<T>` only if it does not complicate direct translation of nullable chunk links; nullable raw pointers are acceptable here because the C model depends on them.
- For chunk allocation sizes and memory accounting, use `usize`.
- For alignment masks and offset computations, use `usize` with checked or saturating arithmetic where needed to avoid unintended overflow differences.

### Memory Management Decisions

- Allocate chunk blocks with the Rust standard allocator (`std::alloc::{alloc, dealloc, Layout}`) unless the C code requires preserving configurable external chunk/free callbacks; if callbacks are part of the source state, keep them as first-class fields and route allocation through them.
- Any unsafe block should be limited to:
  - chunk allocation and deallocation,
  - pointer offset computation,
  - copying object bytes when growing into a new chunk,
  - walking chunk links during free/accounting operations.
- Centralize layout computation for chunk size plus header size so all chunk allocations use one path.

### Error Handling Decisions

- Initialization and chunk-growth paths that can fail allocation should return a `Result` only if the surrounding Rust project API already uses `Result` for low-level modules.
- If preserving C semantics more closely is required, keep fallible internals and expose failure in the narrowest way compatible with existing branch conventions.
- Do not introduce recovery layers or alternative allocation policies.

## Implementation Phases

## Phase 1: Skeleton Port and State Definitions

- Create `src/module_gnu_obstack_03.rs`.
- Define Rust equivalents for the obstack state and chunk header structures from `gnu/obstack.c`.
- Add the function signatures for:
  - `call_chunkfun`
  - `call_freefun`
  - `obstack_begin_worker`
  - `obstack_begin`
  - `obstack_begin_1`
  - `obstack_newchunk`
  - `obstack_allocated_p`
  - `obstack_free`
  - `obstack_memory_used`
- Translate constants, alignment helpers, and size calculations used by the C file into local Rust helpers.
- Establish the minimal unsafe boundaries needed for pointer-based state updates.

## Phase 2: Initialization and Chunk Lifecycle Migration

- Implement `call_chunkfun` and `call_freefun` according to the callback forms used in the C source.
- Port `_obstack_begin_worker`, `_obstack_begin`, and `_obstack_begin_1` first, since they establish all allocator state and callback wiring.
- Port `_obstack_newchunk` next, preserving:
  - chunk size calculation,
  - allocation of a replacement chunk,
  - copy/move of the in-progress object content,
  - previous chunk linking,
  - next-free/base/limit pointer updates.
- Validate alignment handling and header-size calculations with focused unit tests.

## Phase 3: Ownership, Freeing, and Accounting

- Port `_obstack_allocated_p` with direct chunk-chain traversal matching the C logic.
- Port `_obstack_free`, preserving:
  - freeing back to a specific object boundary,
  - chunk walk and release behavior,
  - current-chunk reset rules when freeing into an older chunk.
- Port `_obstack_memory_used` by summing all chunk sizes through the linked chunk list.
- Add tests covering:
  - initialization,
  - chunk growth,
  - freeing within the current chunk,
  - freeing across prior chunks,
  - allocated-pointer detection,
  - memory usage reporting.

## Phase 4: Validation and C-Behavior Parity Review

- Review every Rust function against its C counterpart in source order and confirm one-to-one state transitions.
- Remove any temporary abstractions added during porting that are not required by the original module.
- Tighten unsafe blocks to the smallest possible regions and document invariants at each unsafe boundary.
- Run `cargo test` and finalize the module on branch `009-module_gnu_obstack_03-rust-port`.