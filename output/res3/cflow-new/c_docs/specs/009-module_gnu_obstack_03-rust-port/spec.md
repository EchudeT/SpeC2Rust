# spec.md

## Title

Functional Specification: `module_gnu_obstack_03` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_obstack_03`
- Category: `module_cluster`
- Source basis: `gnu/obstack.c`
- Rust branch: `009-module_gnu_obstack_03-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides obstack-style chunked object storage management. It initializes an obstack, obtains and releases backing chunks through configured allocation callbacks, grows storage when the current chunk is insufficient, determines whether a pointer belongs to the obstack, frees allocations back to a specified object boundary, and reports total backing memory currently held.

The Rust rewrite must preserve the observable behavior of this module as a stateful chunk manager centered on an `obstack` and its linked backing chunks. The required scope is limited to the functionality evidenced by `gnu/obstack.c`.

## Feature Specification

### Summary

The Rust version must implement the following functional behavior:

- Initialize an obstack with configured chunk size, alignment, and chunk allocation/free callbacks.
- Support both callback forms used by the source module:
  - allocation/free functions without an extra user argument
  - allocation/free functions with an extra user argument
- Create an initial backing chunk during initialization.
- Expand storage by allocating a new chunk when additional object data does not fit in the current chunk.
- Preserve existing in-progress object contents when moving to a new chunk.
- Track chunks as a linked sequence associated with a single obstack.
- Test whether a pointer is located within any chunk currently owned by the obstack.
- Free storage back to a given object pointer by releasing newer chunks until the containing chunk is reached.
- Free the entire obstack backing storage when requested with a null object pointer.
- Report the total memory currently held across all chunks.

### Functional Boundary

This module is a storage-management component only. It is not specified here to provide higher-level object construction helpers beyond the chunk-management behavior directly evidenced by the source functions. The Rust port must therefore preserve the allocator/chunk lifecycle semantics of the C module, not extend them with unrelated capabilities.

## User Scenarios & Testing

### Scenario 1: Initialize an obstack with simple callbacks

A caller provides an obstack instance, a preferred chunk size, an alignment value, and chunk allocation/free callbacks that do not take an extra argument. Initialization succeeds only if the initial chunk can be obtained and the obstack enters a usable state.

The Rust version must support tests that verify:

- initialization returns success when the initial chunk allocation succeeds
- initialization returns failure when the initial chunk allocation fails
- after successful initialization, the obstack has a current chunk and valid object-growth region boundaries

### Scenario 2: Initialize an obstack with callbacks that use a caller-supplied argument

A caller provides allocation/free callbacks that require an additional context argument. The module must store and use that argument for subsequent chunk allocation and release operations.

The Rust version must support tests that verify:

- the argument-bearing callback path is accepted
- later chunk allocation uses the stored argument
- later chunk release uses the stored argument

### Scenario 3: Grow storage when the current chunk is too small

While an object is being built in the current chunk, the caller requires additional space that exceeds the remaining capacity. The module allocates a new chunk large enough for the existing partial object data plus the requested extension, moves the partial object data into the new chunk, updates obstack state to point into the new chunk, and releases the old chunk when it no longer contains other retained objects.

The Rust version must support tests that verify:

- requesting a new chunk increases available capacity for continued object growth
- existing partial object bytes are preserved across chunk replacement
- chunk links and current-chunk state are updated consistently
- the old chunk is released only in the cases evidenced by module behavior

### Scenario 4: Check whether a pointer belongs to the obstack

A caller needs to know whether a given pointer lies within memory currently owned by the obstack. The module scans the chunk chain and returns a positive result only if the pointer falls within one of the currently held chunks.

The Rust version must support tests that verify:

- a pointer inside the current chunk is reported as allocated by the obstack
- a pointer inside an older retained chunk is reported as allocated by the obstack
- a pointer outside all obstack chunks is not reported as allocated by the obstack

### Scenario 5: Free back to a specific object

A caller frees the obstack back to an object pointer that lies within one of its chunks. The module releases all newer chunks and resets the current state to the chunk containing that object.

The Rust version must support tests that verify:

- freeing to an object in the current chunk keeps that chunk and updates state appropriately
- freeing to an object in an older chunk releases all newer chunks
- after the operation, the current chunk is the one containing the target object

### Scenario 6: Free the entire obstack

A caller passes a null object pointer to free all backing storage held by the obstack.

The Rust version must support tests that verify:

- all chunks are released
- each released chunk is passed through the configured free callback path
- no chunks remain owned by the obstack afterward

### Scenario 7: Reject invalid free targets

A caller attempts to free to a pointer not contained within the obstack. The source module treats this as an error condition rather than silently accepting it.

The Rust version must support tests that verify:

- freeing to a non-member pointer does not behave as a valid in-obstack free
- the resulting behavior matches the source module’s contract for this invalid case

### Scenario 8: Report memory usage

A caller queries total memory held by the obstack. The module sums the sizes of all currently owned chunks.

The Rust version must support tests that verify:

- memory usage after initialization includes the initial chunk
- memory usage increases after additional chunks are acquired
- memory usage decreases after freeing newer chunks or the entire obstack

## Requirements

### Functional Requirements

#### FR-1: Obstack initialization

The module shall initialize an obstack using a requested chunk size and alignment, and shall acquire an initial chunk before reporting success.

Traceability: `_obstack_begin_worker`, `_obstack_begin`, `_obstack_begin_1`

#### FR-2: Dual callback configuration forms

The module shall support both callback configurations evidenced by the source:
- chunk allocation/free callbacks without an extra argument
- chunk allocation/free callbacks with an extra stored argument

Traceability: `call_chunkfun`, `call_freefun`, `_obstack_begin`, `_obstack_begin_1`

#### FR-3: Callback-mediated chunk lifecycle

The module shall obtain backing chunks only through the configured allocation callback path and shall release backing chunks only through the configured free callback path.

Traceability: `call_chunkfun`, `call_freefun`, `_obstack_begin_worker`, `_obstack_newchunk`, `_obstack_free`

#### FR-4: Alignment-aware object area establishment

On successful initialization and after chunk replacement, the module shall establish object-growth positions consistent with the configured alignment and chunk layout rules used by the obstack.

Traceability: `_obstack_begin_worker`, `_obstack_newchunk`

#### FR-5: Chunk growth for insufficient space

When the current chunk lacks sufficient room for requested additional object length, the module shall allocate a replacement chunk large enough for retained object contents plus requested growth and continue object storage there.

Traceability: `_obstack_newchunk`

#### FR-6: Preservation of in-progress object contents during growth

When a new chunk is acquired for continued object growth, the module shall preserve the bytes of the partially built object from the previous current chunk in the new current chunk.

Traceability: `_obstack_newchunk`

#### FR-7: Linked ownership of chunks

The module shall maintain chunk ownership as a chain associated with an obstack so that current and older retained chunks can be traversed for membership checks, freeing, and memory accounting.

Traceability: `_obstack_newchunk`, `_obstack_allocated_p`, `_obstack_free`, `_obstack_memory_used`, key types `obstack`, `_obstack_chunk`

#### FR-8: Membership testing

The module shall report whether a supplied pointer lies within any chunk currently owned by the obstack.

Traceability: `_obstack_allocated_p`

#### FR-9: Free-to-object semantics

The module shall support freeing storage back to a supplied object pointer by releasing newer chunks until the chunk containing that pointer becomes the current chunk.

Traceability: `_obstack_free`

#### FR-10: Full release semantics

The module shall support releasing all backing chunks when asked to free to a null object pointer.

Traceability: `_obstack_free`

#### FR-11: Invalid free target handling

The module shall preserve the source module’s error behavior when a free request specifies a pointer not contained in the obstack.

Traceability: `_obstack_free`, `_obstack_allocated_p`

#### FR-12: Memory usage reporting

The module shall return the total size of all chunks currently owned by the obstack.

Traceability: `_obstack_memory_used`

### Key Entities

#### `obstack`

The primary state holder for obstack-managed storage. It stores the current chunk context, object-growth boundaries, configuration such as chunk sizing and alignment, and the callback configuration used to allocate and free chunks. It owns a chain of backing chunks.

Traceability: all listed functions reference `struct obstack`

#### `_obstack_chunk`

A backing storage chunk linked to older chunks. Each chunk contributes to total owned memory, may contain retained objects, and participates in pointer membership checks and free-back traversal.

Traceability: `_obstack_begin_worker`, `_obstack_newchunk`, `_obstack_allocated_p`, `_obstack_free`, `_obstack_memory_used`

#### Allocation/free callback configuration

The configured callback pair, with or without an extra caller-supplied argument, defines how chunk memory is acquired and released for a given obstack instance.

Traceability: `call_chunkfun`, `call_freefun`, `_obstack_begin`, `_obstack_begin_1`

## Success Criteria

### SC-1: Initialization correctness

For successful initialization paths, the Rust module creates an initial owned chunk and establishes valid current object-growth state. For failed initial chunk allocation, initialization reports failure and does not present a usable initialized obstack state.

Traceability: `_obstack_begin_worker`, `_obstack_begin`, `_obstack_begin_1`

### SC-2: Callback-path fidelity

Tests demonstrate that chunk allocation and release occur through the callback form configured at initialization, including correct use of the stored extra argument when that form is selected.

Traceability: `call_chunkfun`, `call_freefun`, `_obstack_begin`, `_obstack_begin_1`

### SC-3: Growth correctness

Tests demonstrate that requesting additional space beyond current capacity results in acquisition of a suitable new chunk, preservation of in-progress object data, and correct update of the current chunk state.

Traceability: `_obstack_newchunk`

### SC-4: Membership correctness

Tests demonstrate that pointers inside any currently owned chunk are accepted by membership checks and pointers outside all owned chunks are rejected.

Traceability: `_obstack_allocated_p`

### SC-5: Free-back correctness

Tests demonstrate that freeing to an object in an owned chunk releases all newer chunks and leaves the chunk containing that object as current.

Traceability: `_obstack_free`

### SC-6: Full-release correctness

Tests demonstrate that freeing with a null object pointer releases all owned chunks through the configured free callback path.

Traceability: `_obstack_free`

### SC-7: Invalid free behavior preservation

Tests demonstrate that attempting to free to a pointer not owned by the obstack follows the same error contract as the source module.

Traceability: `_obstack_free`

### SC-8: Memory accounting correctness

Tests demonstrate that reported memory usage equals the sum of sizes of all currently owned chunks across initialization, growth, partial free, and full release cases.

Traceability: `_obstack_memory_used`