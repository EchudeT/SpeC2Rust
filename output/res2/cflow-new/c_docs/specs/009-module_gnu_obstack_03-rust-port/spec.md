# spec.md

## Overview

This module provides obstack-style memory management for variable-sized object construction within a linked sequence of allocation chunks. It initializes an obstack state, acquires and releases chunks through caller-supplied allocation hooks, grows storage when the current chunk is insufficient, checks whether a pointer belongs to the obstack, frees objects by discarding later chunk state, and reports total memory currently held.

The Rust rewrite must preserve the functional behavior evidenced by `gnu/obstack.c`, specifically the behavior represented by:

- obstack initialization with either plain or argument-carrying chunk alloc/free callbacks
- chunk replacement when additional space is needed for the current object
- membership testing for pointers within allocated chunks
- freeing back to a specific object or clearing all chunks
- reporting memory usage across all currently linked chunks

## Scope

In scope:

- Functional behavior corresponding to the obstack lifecycle managed in `gnu/obstack.c`
- Support for allocator/free callback forms represented by `_obstack_begin` and `_obstack_begin_1`
- State transitions affecting the current chunk chain and current object position
- Observable outcomes of the exported functions:
  - `_obstack_begin`
  - `_obstack_begin_1`
  - `_obstack_newchunk`
  - `_obstack_allocated_p`
  - `_obstack_free`
  - `_obstack_memory_used`

Out of scope:

- Any capability not evidenced by this module file
- New public APIs beyond what is needed to preserve the module behavior
- Concurrency guarantees
- Serialization, persistence, recovery, or benchmarking features

## Feature Specification

### Summary

The module manages a growable memory arena composed of linked chunks. Objects are built in the current chunk. When more space is required, a new chunk is allocated, the in-progress object data is preserved into the new chunk, and the obstack state advances to the new chunk. Existing chunks remain linked until explicitly freed. The module also supports checking whether a pointer lies within any currently allocated chunk and computing the total memory consumed by those chunks.

### Supported behaviors

1. **Obstack initialization**
   - The module must initialize an obstack with a first chunk and establish its operating parameters.
   - Initialization must support:
     - callbacks taking only `size_t` / `void *`
     - callbacks taking an additional user argument
   - The resulting obstack must have a valid current chunk and object-growth state if initialization succeeds.
   - Initialization reports success or failure as an integer result, matching the source behavior.

2. **Chunk acquisition through callbacks**
   - Chunk allocation must be performed through the callback configuration stored in the obstack.
   - Chunk release must likewise use the configured free callback form.
   - The internal distinction between callback signatures must be honored so the correct callback variant is used.

3. **New chunk creation for continued object growth**
   - When asked to provide additional space for an object, the module must allocate a new chunk if the current chunk cannot satisfy the requested continued growth.
   - The new chunk size must be sufficient for:
     - the existing partially built object content
     - the additional requested length
     - chunk overhead and alignment needs implied by obstack state
   - The object under construction must continue in the new chunk with preserved content.
   - The previous chunk remains part of the linked chain unless released by later freeing behavior.

4. **Pointer membership test**
   - The module must determine whether a supplied pointer lies within any chunk currently linked to the obstack.
   - The result is an integer truth value reflecting membership.

5. **Freeing to an object or clearing the obstack**
   - The module must support freeing back to a specific object pointer that belongs to the obstack.
   - Freeing to an object discards all newer chunks above the chunk containing that object and resets current state to that chunk.
   - The module must also support the null-pointer case used to release all chunks held by the obstack.
   - Free operations must release discarded chunks through the configured free callback.

6. **Memory usage reporting**
   - The module must report the total memory currently held across all linked chunks.
   - The total must decrease after chunk-discarding free operations and become zero after full release.

## User Scenarios & Testing

### Scenario 1: Initialize an obstack with simple callbacks

A caller provides chunk allocation and free callbacks without a user argument, along with requested chunk size and alignment. Initialization succeeds and the obstack becomes ready for object construction within its first chunk.

**Test expectations**
- Calling the simple initialization entry point returns success when the allocator provides a first chunk.
- The obstack has a current chunk after initialization.
- Memory usage is nonzero after successful initialization.

### Scenario 2: Initialize an obstack with callbacks that use a caller argument

A caller provides chunk allocation and free callbacks that require an additional context argument. Initialization uses those callbacks and stores the argument for later chunk allocation and release operations.

**Test expectations**
- Calling the argument-carrying initialization entry point returns success when the allocator provides a first chunk.
- Later chunk allocations and frees are performed through the callback form that receives the stored argument.
- Memory usage reflects chunks allocated through this path.

### Scenario 3: Grow an in-progress object into a new chunk

An object is being built in the current chunk. The caller requests additional length that does not fit. The module allocates a larger chunk, preserves the already-built object bytes into the new chunk, and continues the object there.

**Test expectations**
- After requesting a new chunk, the obstack's current chunk changes when the old chunk lacked sufficient space.
- The previously accumulated object content remains intact in the new current chunk.
- The old chunk remains linked and contributes to memory usage until later freed.

### Scenario 4: Check whether a pointer belongs to the obstack

A caller has a pointer and needs to know whether it lies within any chunk owned by the obstack.

**Test expectations**
- A pointer inside a currently linked chunk yields a true integer result.
- A pointer outside all currently linked chunks yields a false integer result.
- A pointer from a chunk that was discarded by freeing no longer tests as allocated.

### Scenario 5: Free back to a retained object

A caller frees the obstack back to an earlier object that is still within one of the linked chunks. All chunks newer than that object’s containing chunk are released.

**Test expectations**
- Chunks newer than the retained object's chunk are freed through the configured free callback.
- The current chunk becomes the chunk containing the retained object.
- Memory usage decreases by the total size of discarded chunks.

### Scenario 6: Release all memory

A caller frees the obstack with a null object pointer to release every chunk.

**Test expectations**
- All currently linked chunks are released through the configured free callback.
- Memory usage becomes zero.
- Subsequent membership checks for formerly valid pointers return false.

## Requirements

### Functional Requirements

#### FR-1: Initialize obstack state
The module shall initialize an obstack using caller-supplied chunk allocation and free callbacks, requested chunk size, and requested alignment, and shall return an integer success/failure result.
**Traceability:** `_obstack_begin_worker`, `_obstack_begin`, `_obstack_begin_1`

#### FR-2: Support both callback signatures
The module shall support both callback configurations evidenced by the source:
- allocation/free callbacks without an extra argument
- allocation/free callbacks with a stored extra argument used on each call
**Traceability:** `call_chunkfun`, `call_freefun`, `_obstack_begin`, `_obstack_begin_1`

#### FR-3: Acquire the initial chunk during initialization
Successful initialization shall include acquiring an initial chunk and establishing it as the current chunk for the obstack. If the initial allocation cannot be obtained, initialization shall fail.
**Traceability:** `_obstack_begin_worker`, `call_chunkfun`

#### FR-4: Create a new chunk when more space is required
The module shall provide behavior equivalent to allocating a new chunk when the current chunk cannot accommodate continued object growth by the requested length.
**Traceability:** `_obstack_newchunk`

#### FR-5: Preserve the in-progress object across chunk replacement
When a new chunk is allocated for continued object growth, the module shall preserve the current partially built object content so growth can continue in the new chunk.
**Traceability:** `_obstack_newchunk`

#### FR-6: Maintain linked chunk ownership
The module shall maintain ownership as a chain of currently allocated chunks, with newer chunks linked from prior state until explicitly freed.
**Traceability:** `_obstack_newchunk`, `_obstack_free`, `_obstack_memory_used`, key type `_obstack_chunk`

#### FR-7: Report whether a pointer belongs to the obstack
The module shall return an integer truth value indicating whether a supplied pointer lies within any chunk currently allocated to the obstack.
**Traceability:** `_obstack_allocated_p`

#### FR-8: Free back to a supplied object pointer
The module shall support freeing the obstack back to an object pointer by discarding all chunks newer than the chunk containing that object and making the containing chunk current.
**Traceability:** `_obstack_free`

#### FR-9: Free all chunks when given a null object pointer
The module shall support releasing all currently allocated chunks when the free operation is invoked with a null object pointer.
**Traceability:** `_obstack_free`

#### FR-10: Release chunks through the configured free callback
Any chunk discarded by free behavior shall be released through the free callback configuration stored in the obstack.
**Traceability:** `call_freefun`, `_obstack_free`

#### FR-11: Report total memory currently held
The module shall report the total amount of memory represented by all chunks currently linked to the obstack.
**Traceability:** `_obstack_memory_used`, key type `_obstack_chunk`

### Key Entities

#### `obstack`
The primary obstack state holder. It represents one managed memory arena and stores:
- callback configuration for chunk allocation and release
- sizing/alignment configuration established at initialization
- the current chunk used for object growth
- current object position/state needed to continue growth and to migrate an in-progress object to a new chunk

**Relationship to other entities**
- owns or references the head/current portion of a linked chain of `_obstack_chunk`
- uses callback configuration to allocate and free `_obstack_chunk` instances

#### `_obstack_chunk`
A chunk in the obstack’s linked storage chain. Each chunk represents one allocated memory block used for object storage and links to an earlier chunk.

**Relationship to other entities**
- belongs to exactly one obstack while allocated
- may be current or non-current within the obstack chain
- is traversed for pointer membership checks, freeing, and memory usage summation

## Success Criteria

1. The Rust module provides behavior equivalent to initialization through both exported initialization entry points, including success/failure signaling and acquisition of an initial chunk.
   **Traceability:** `_obstack_begin`, `_obstack_begin_1`, `_obstack_begin_worker`

2. The Rust module correctly dispatches chunk allocation and free operations through the callback form configured for the obstack instance.
   **Traceability:** `call_chunkfun`, `call_freefun`

3. When continued object growth exceeds current chunk capacity, the Rust module allocates a new chunk and preserves the already-built object content so growth can continue without data loss.
   **Traceability:** `_obstack_newchunk`

4. Pointer membership checks in the Rust module return true for pointers within currently linked chunks and false otherwise.
   **Traceability:** `_obstack_allocated_p`

5. Freeing to an object pointer in the Rust module releases all newer chunks, retains the chunk containing the object, and updates current state accordingly.
   **Traceability:** `_obstack_free`

6. Freeing with a null pointer in the Rust module releases all chunks held by the obstack.
   **Traceability:** `_obstack_free`

7. Memory usage reported by the Rust module equals the sum of memory represented by currently linked chunks, decreases after chunk-discarding frees, and reaches zero after full release.
   **Traceability:** `_obstack_memory_used`, `_obstack_free`

8. All scenario-based tests described in this document pass against the Rust rewrite.
   **Traceability:** all exported functions in `gnu/obstack.c`