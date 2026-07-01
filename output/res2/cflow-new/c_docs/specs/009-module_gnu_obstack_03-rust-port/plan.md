# Implementation Plan: module_gnu_obstack_03

## Summary

This module ports the allocation logic from `gnu/obstack.c` into Rust, preserving the original chunk-based object stack behavior and migration order around the existing C functions. The Rust implementation should focus on reproducing the current allocator semantics: chunk acquisition, chunk release, object growth support through chunk replacement, membership checks, freeing back to a prior object boundary, and reporting total memory usage.

The technical approach is to implement a single Rust module that models the obstack state explicitly, including current chunk pointers, chunk limits, object base, next free position, alignment mask, and allocator callbacks. Since the C code depends on manual pointer arithmetic and chunk headers embedded in allocated memory, the Rust port should use carefully bounded unsafe code only where raw pointer operations are required. Allocation and free callback behavior should remain configurable so the Rust code can preserve the same chunk sourcing model as the C implementation rather than replacing it with a different allocator abstraction.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve amortized allocation behavior of chunk-based growth.
  - Avoid per-object heap allocations beyond the chunk strategy already present in the C implementation.
  - Keep pointer adjustment and chunk traversal costs comparable to the C code.
  - Minimize additional runtime checks outside those needed for Rust safety boundaries.

## Module Mapping

### Source Mapping

- **C source file**: `gnu/obstack.c`
- **Rust target file**: `src/module_gnu_obstack_03.rs`

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `call_chunkfun` | `fn call_chunkfun(...)` | Internal helper wrapping configured chunk allocator callback |
| `call_freefun` | `fn call_freefun(...)` | Internal helper wrapping configured chunk free callback |
| `_obstack_begin_worker` | `fn obstack_begin_worker(...) -> Result<..., ...>` | Core initializer used by public constructors |
| `_obstack_begin` | `fn obstack_begin(...) -> Result<..., ...>` | Constructor path without extra allocator argument |
| `_obstack_begin_1` | `fn obstack_begin_1(...) -> Result<..., ...>` | Constructor path with extended allocator configuration |
| `_obstack_newchunk` | `fn obstack_newchunk(...) -> Result<(), ...>` | Internal chunk growth and object relocation routine |
| `_obstack_allocated_p` | `fn obstack_allocated_p(...) -> bool` | Chunk membership check |
| `_obstack_free` | `fn obstack_free(...)` | Free-to-object/chunk rewind logic |
| `_obstack_memory_used` | `fn obstack_memory_used(...) -> usize` | Total allocated chunk memory accounting |

### Rust Module Scope

The Rust module should remain narrowly scoped to the migrated obstack implementation from `gnu/obstack.c`. Do not split functionality into extra allocator layers or utility modules unless directly required by borrow-checking or unsafe isolation inside the same source file.

## Data Model

The C analysis only exposes anonymous data structures, so the Rust plan should reconstruct only the structures implied by the listed functions and file responsibilities.

### Data Structure Mapping

| C Structure | Rust Representation | Purpose |
|---|---|---|
| anonymous chunk header structure | `struct ObstackChunk` | Stores chunk linkage and allocation extent metadata |
| anonymous obstack state structure | `struct Obstack` | Stores current chunk, object base, next free pointer, chunk limit, alignment, chunk size, and allocator callbacks |
| anonymous allocator callback fields | `type ChunkAllocFn` / `type ChunkFreeFn` | Represents chunk allocation and release entry points |
| anonymous optional extra-argument callback variant | `enum AllocatorMode` or embedded callback fields | Preserves distinction between basic and extended initializer forms |
| anonymous flag/integer configuration fields | primitive Rust fields (`usize`, `u8`, `bool`) | Mirrors alignment masks, sizes, and mode flags |

### Planned Rust Structures

```rust
type ChunkAllocFn = unsafe fn(size: usize) -> *mut u8;
type ChunkFreeFn = unsafe fn(ptr: *mut u8);

struct ObstackChunk {
    prev: *mut ObstackChunk,
    limit: *mut u8,
}

struct Obstack {
    chunk_size: usize,
    alignment_mask: usize,
    chunk: *mut ObstackChunk,
    object_base: *mut u8,
    next_free: *mut u8,
    chunk_limit: *mut u8,
    alloc_fn: ChunkAllocFn,
    free_fn: ChunkFreeFn,
    use_extra_arg: bool,
    // extra allocator argument field only if required by _obstack_begin_1 semantics
}
```

### Ownership and Memory Strategy

- Chunk memory will remain manually managed because the original implementation depends on allocator callback pairs and raw region traversal.
- The top-level `Obstack` state should own only the allocator configuration and current chunk chain head logically; physical chunk lifetime is managed through the configured free callback.
- Raw pointers are appropriate for:
  - navigating linked chunk headers,
  - computing aligned object boundaries,
  - relocating partially built objects during chunk growth,
  - checking whether a candidate object address lies inside a known chunk.
- Unsafe code should be concentrated around:
  - converting allocation results into chunk headers,
  - pointer arithmetic for base/limit calculations,
  - copying partially built object bytes into a new chunk,
  - iterating chunk links during free and memory usage traversal.

### Error Handling Mapping

| C Style | Rust Plan |
|---|---|
| null return / implicit allocation failure | `Result<..., ObstackError>` for initialization and new-chunk creation |
| procedures mutating state in place | keep mutating methods, return `Result` only when allocation can fail |
| unchecked callback failure conventions | treat null allocation pointer as allocation failure |
| free operations with no explicit status | preserve infallible API unless invalid pointer handling in C requires a defined Rust error or panic policy |

### Error Type

Use a small module-local error enum, for example:

```rust
enum ObstackError {
    AllocationFailed,
    InvalidConfiguration,
}
```

This should remain minimal and only cover states directly exercised by the migrated functions.

## Implementation Phases

### Phase 1: Establish Core State and Initializers

- Create `src/module_gnu_obstack_03.rs`.
- Define the Rust representations for obstack state, chunk headers, allocator callback types, and minimal error enum.
- Implement:
  - `call_chunkfun`
  - `call_freefun`
  - `obstack_begin_worker`
  - `obstack_begin`
  - `obstack_begin_1`
- Port initialization logic in the same order as the C code so field dependencies remain easy to verify.
- Ensure alignment mask handling, chunk size setup, and initial chunk allocation match the original behavior.
- Add unit tests for:
  - successful initialization,
  - allocation callback failure at startup,
  - configuration edge cases such as invalid size/alignment inputs if the C logic distinguishes them.

### Phase 2: Port Chunk Growth Logic

- Implement `obstack_newchunk` using the same data movement model as the C version.
- Preserve object-in-progress relocation behavior when the current chunk cannot satisfy additional space requirements.
- Use `ptr::copy_nonoverlapping` or `ptr::copy` only where the C code semantics require it.
- Verify that old chunk retention/free decisions match the original logic during growth.
- Add unit tests for:
  - growing from one chunk to another,
  - preserving existing partial object bytes,
  - correct updating of `object_base`, `next_free`, `chunk`, and `chunk_limit`,
  - allocation failure during growth leaving state in a defined condition.

### Phase 3: Port Membership, Rewind, and Accounting Operations

- Implement:
  - `obstack_allocated_p`
  - `obstack_free`
  - `obstack_memory_used`
- Mirror C traversal order through chunk links for membership detection and memory accounting.
- Preserve free-to-pointer semantics, including releasing later chunks until the target object location is reached.
- Keep invalid target-pointer handling aligned with the source behavior rather than introducing new policy layers.
- Add unit tests for:
  - membership checks across multiple chunks,
  - freeing to null/current/earlier object positions as supported by the C implementation,
  - memory usage totals before and after chunk release,
  - chunk chain integrity after repeated growth and rewind cycles.

### Phase 4: Final Safety Review and Integration Cleanup

- Reduce unsafe scope by moving raw pointer operations into narrowly bounded helper blocks while keeping the file single-module.
- Validate that all callback invocations, chunk header writes, and pointer range checks are documented with invariants in code comments.
- Confirm no extra abstractions were introduced beyond what is necessary for the direct port.
- Run `cargo test` and fix any semantic mismatches revealed by migration tests.
- Perform a final API review to keep exported items limited to what the ported module actually requires.

## Validation Notes

- Test coverage should focus on observable allocator behavior, not redesign.
- Prefer deterministic tests with simple custom allocator/free stubs backed by raw buffers or boxed allocations.
- Verify that memory accounting and chunk traversal logic operate correctly across chained chunks and after partial unwinding.
- Ensure the Rust implementation does not silently change semantics around pointer containment or chunk release ordering.