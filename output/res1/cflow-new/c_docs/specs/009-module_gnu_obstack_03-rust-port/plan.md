# Implementation Plan: module_gnu_obstack_03

## Summary

This module ports the obstack allocator logic from `gnu/obstack.c` into Rust while preserving the existing allocation model, chunk growth behavior, and object-lifetime semantics defined by the current C implementation.

The Rust implementation should focus on a direct migration of the existing file and functions into a single Rust module that manages:
- chunk-based allocation,
- configurable chunk allocation and free callbacks,
- object growth via chunk replacement,
- membership checks for allocated memory,
- freeing back to an earlier object boundary,
- reporting total memory usage.

The technical approach should remain close to the C layout:
- represent the obstack state as a Rust struct mirroring the C fields,
- model chunk links explicitly,
- preserve callback-driven chunk allocation/free behavior through typed function pointers or closures stored in the obstack state,
- use raw pointers internally where necessary to match the original memory model,
- isolate unsafe memory operations inside narrowly scoped helper routines,
- expose Rust methods corresponding to the C entry points rather than introducing additional abstractions.

The implementation should not expand the feature set beyond the listed functions and data structures.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.75+`

### Primary Dependencies
- Rust standard library only

Recommended standard-library components:
- `std::ptr`
- `std::mem`
- `std::alloc` only if needed for internal raw allocation behavior
- `std::ffi::c_void` for C-like pointer representation where required

No third-party crates are recommended because the input provides no evidence requiring them.

### Testing
- `cargo test`

Testing scope should cover:
- initialization through both begin variants,
- chunk expansion through `_obstack_newchunk`,
- pointer containment checks through `_obstack_allocated_p`,
- free-to-object behavior in `_obstack_free`,
- total usage accounting in `_obstack_memory_used`,
- callback invocation paths for chunk allocation and free.

### Performance Goals
- Preserve the amortized chunk-allocation behavior of the C implementation.
- Avoid per-object heap allocation beyond the chunk model already present in the source.
- Keep pointer arithmetic and copy operations close to the original algorithmic cost.
- Maintain low overhead in hot paths by using direct field access and limited abstraction layers.
- Avoid unnecessary initialization or extra ownership layers that would alter chunk growth cost.

## Module Mapping

### Source Mapping
- C source: `gnu/obstack.c`
- Rust target: `src/module_gnu_obstack_03.rs`

If the crate already uses a module tree, expose it from the existing root with a single module declaration only, without splitting the port into additional submodules unless the existing project layout already requires it.

### Function Mapping
Map each C function to a Rust function or inherent method with the same operational role:

- `call_chunkfun`
  - Rust: private helper function
  - Responsibility: invoke the configured chunk-allocation callback and normalize pointer handling

- `call_freefun`
  - Responsibility: invoke the configured chunk-free callback safely for a chunk pointer

- `_obstack_begin_worker`
  - Rust: internal initialization method/helper
  - Responsibility: shared initialization logic used by public begin entry points

- `_obstack_begin`
  - Rust: public constructor-style function/method
  - Responsibility: initialize obstack with default free signature path

- `_obstack_begin_1`
  - Responsibility: initialize obstack with alternate callback configuration matching the C entry point

- `_obstack_newchunk`
  - Rust: internal/public method matching current visibility needs
  - Responsibility: allocate a new chunk, copy active object data, relink chunk chain, and update pointers

- `_obstack_allocated_p`
  - Rust: public query method
  - Responsibility: test whether a pointer belongs to any current chunk in the obstack chain

- `_obstack_free`
  - Rust: public method
  - Responsibility: free chunks back to the chunk containing the provided object, or reset/free all as defined by current behavior

- `_obstack_memory_used`
  - Responsibility: sum chunk sizes currently owned by the obstack

## Data Model

The analysis lists only anonymous data structures, but `gnu/obstack.c` conventionally operates around the obstack state object and linked chunk headers. The Rust port should reconstruct only the structures required by the listed functions.

### Data-Structure Mapping

| C structure | Rust representation | Notes |
|---|---|---|
| obstack state structure | `struct Obstack` | Primary allocator state; mirrors C fields as closely as practical |
| chunk header structure | `struct ObstackChunk` | Linked chunk metadata plus owned allocation buffer metadata |
| chunk allocation callback | function pointer field in `Obstack` | Use typed function pointer matching required signature |
| chunk free callback | function pointer field in `Obstack` | Use typed function pointer matching required signature |
| object/data pointers within chunk | `*mut u8` or `NonNull<u8>` | Raw pointers are appropriate for pointer arithmetic and C parity |
| extra argument / callback context if present | stored scalar/pointer field in `Obstack` | Preserve exact role without adding ownership semantics |
| bit/flag fields from C anonymous structures | plain Rust integer/bool fields | Keep direct field mapping; avoid introducing enums unless clearly stable and local |

### Suggested Rust Types

```rust
use std::ffi::c_void;
use std::ptr::NonNull;

type ChunkAllocFn = unsafe fn(extra_arg: *mut c_void, size: usize) -> *mut c_void;
type ChunkFreeFn = unsafe fn(extra_arg: *mut c_void, ptr: *mut c_void);

struct ObstackChunk {
    prev: Option<NonNull<ObstackChunk>>,
    limit: *mut u8,
    block: NonNull<u8>,
    size: usize,
}

struct Obstack {
    chunk_size: usize,
    alignment_mask: usize,
    chunk: Option<NonNull<ObstackChunk>>,
    object_base: *mut u8,
    next_free: *mut u8,
    chunk_limit: *mut u8,
    maybe_empty_object: bool,
    alloc_failed: bool,
    chunkfun: ChunkAllocFn,
    freefun: ChunkFreeFn,
    extra_arg: *mut c_void,
}
```

This is a planning shape, not a fixed API. Final field names and exact pointer wrappers should be chosen to match the semantics of the source file.

### Memory Management Decisions
- Chunk ownership remains explicit and manual, because the original module manages linked raw allocations and supports freeing back through chunk history.
- Internally, the port may store chunk allocations in raw memory blocks and free them through the configured callback rather than through Rust-owned containers, to preserve behavior.
- Unsafe code is expected for:
  - pointer arithmetic,
  - copying in-progress object bytes to a new chunk,
  - linked-chunk traversal through raw pointers,
  - callback invocation over raw memory addresses.
- All unsafe operations should be localized inside helper functions and small method bodies with documented invariants.

### Error Handling Decisions
- Initialization and chunk growth should reflect the original failure behavior.
- If the C logic records allocation failure in state rather than returning rich errors, the Rust port should preserve that pattern.
- Do not introduce broader error enums unless required to represent existing branch behavior from the C implementation.
- Null callback returns and invalid pointer conditions should be handled in a way that matches current semantics rather than converting the API into idiomatic `Result`-based allocation APIs.

## Implementation Phases

### Phase 1: Establish module skeleton and state mapping
Goals:
- Create the Rust module file for the port.
- Define Rust structs for obstack state and chunk metadata.
- Define callback type aliases matching the C allocation/free call patterns.
- Implement `call_chunkfun`, `call_freefun`, `_obstack_begin_worker`, `_obstack_begin`, and `_obstack_begin_1`.

Tasks:
- Inspect `gnu/obstack.c` field usage and map every required state field directly into `Obstack`.
- Recreate initialization defaults, alignment handling, and initial chunk setup.
- Preserve dual initialization entry points without combining them into a more abstract constructor API.
- Add focused unit tests for initialization state and callback wiring.

Exit criteria:
- The obstack can be initialized through both begin functions.
- Initial chunk allocation and internal pointer setup match expected invariants.
- Tests confirm callback dispatch and initial field values.

### Phase 2: Port chunk growth and memory traversal logic
Goals:
- Implement `_obstack_newchunk`.
- Implement `_obstack_allocated_p`.
- Preserve object-copy and chunk-link behavior exactly enough to support later free logic.

Tasks:
- Port chunk sizing and growth calculations from C with attention to overflow-sensitive arithmetic.
- Implement active-object byte copying from the old chunk to the new chunk using raw pointer copy.
- Update chunk-chain links and object pointers after growth.
- Implement pointer-range scanning across chunk links for allocation membership tests.

Exit criteria:
- Growing an obstack moves in-progress object bytes correctly.
- Membership checks work for pointers inside current and previous chunks and reject unrelated pointers.
- Tests cover chunk growth, copied content continuity, and containment checks.

### Phase 3: Port free-back and memory accounting behavior
Goals:
- Implement `_obstack_free`.
- Implement `_obstack_memory_used`.
- Validate chunk-release ordering and memory totals.

Tasks:
- Port the logic that walks backward through chunk links to locate the target object.
- Free newer chunks through the configured free callback until the containing chunk is reached.
- Restore object and free pointers consistent with the located chunk and object address.
- Sum all currently linked chunk sizes for `_obstack_memory_used`.

Exit criteria:
- Free-back behavior matches the source semantics for:
  - freeing to an object in the current chunk,
  - freeing to an object in an older chunk,
  - freeing/reset behavior when applicable.
- Memory-used reporting matches the set of chunks still linked.
- Tests validate chunk release count, pointer restoration, and accounting.

### Phase 4: Safety review and parity cleanup
Goals:
- Reduce risk in unsafe sections without changing behavior.
- Finalize API visibility and documentation comments.
- Ensure the Rust file is a faithful, contained migration of the original C file.

Tasks:
- Audit every unsafe block and document required invariants.
- Check null-pointer handling, alignment assumptions, and integer conversions.
- Remove any accidental abstraction layers or helper code not needed by the migrated functions.
- Expand tests for edge conditions already implied by the C implementation, such as minimal chunk sizes and repeated growth/free cycles.

Exit criteria:
- Unsafe sections are narrowly scoped and justified.
- Public/private boundaries reflect actual use by the project.
- `cargo test` passes with behavior centered on parity with `gnu/obstack.c`.

## Notes and Constraints

- Keep the port confined to the behavior and structures required by `gnu/obstack.c`.
- Do not introduce thread-safe wrappers, allocator trait integrations, serialization, or broader memory-management frameworks.
- Prefer one Rust module corresponding to the single C file.
- Match C semantics first; only apply Rust-specific refinements where they do not alter behavior or migration scope.