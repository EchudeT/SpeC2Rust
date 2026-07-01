# spec.md

## Title

Rust Functional Specification for `module_gnu_obstack_03`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_obstack_03`
- Category: `module_cluster`
- Source basis: `gnu/obstack.c`
- Rust branch: `009-module_gnu_obstack_03-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides obstack chunk-management behavior: initialization of an obstack, growth by allocating replacement chunks when the current chunk is insufficient, testing whether a pointer belongs to the obstack, freeing objects by rewinding to a given object or clearing the entire obstack, and reporting total memory currently held by the obstack.

The Rust rewrite must preserve the observable behavior defined by the source module for these responsibilities. The specification is limited to functionality evidenced by `gnu/obstack.c` and its referenced `struct obstack` and `struct _obstack_chunk` state.

## Scope

### In Scope

- Initializing an obstack with allocator and deallocator callbacks.
- Supporting both callback styles:
  - callbacks taking only allocation/free parameters
  - callbacks taking an extra user argument
- Creating an initial chunk during initialization.
- Allocating a new chunk when additional room is needed for object growth.
- Preserving the partially built object contents when moving to a new chunk.
- Determining whether a given object pointer lies within any chunk currently owned by the obstack.
- Freeing chunks above a target object and resetting the obstack state to that object.
- Freeing the full obstack when requested with a null object pointer.
- Reporting aggregate memory used across all currently retained chunks.

### Out of Scope

- Any API or behavior not evidenced by `gnu/obstack.c`.
- New public APIs beyond the behavior represented by the analyzed functions.
- Thread-safety guarantees.
- Serialization, persistence, recovery, diagnostics, or benchmarking features.
- Changes to allocator semantics beyond what the source module requires.

## Feature Specification

### 1. Obstack Initialization

The module must initialize an obstack instance so it can begin allocating from an initial chunk.

Initialization behavior must support two forms:

- a form using allocation and free callbacks without an extra argument
- a form using allocation and free callbacks with an extra stored argument passed back into both callbacks

Initialization must:

- accept requested chunk size and alignment inputs
- normalize zero-valued size and alignment inputs into usable values
- allocate an initial chunk through the configured allocation callback
- establish the obstack’s current chunk and object-position state within that chunk
- return success or failure status according to whether the initial chunk allocation succeeds

The Rust version must preserve the distinction between the two callback forms because both are directly supported by the source module.

### 2. Callback-Driven Chunk Allocation and Release

The module must obtain and release chunk memory only through the callback functions configured on the obstack.

Behavior must include:

- invoking the correct allocation callback variant for the current obstack configuration
- invoking the correct free callback variant for the current obstack configuration
- passing the configured extra argument when the obstack uses argument-bearing callbacks

This is a functional boundary of the module because callback dispatch is explicitly handled by dedicated helper logic and is required for all chunk lifecycle operations.

### 3. New-Chunk Growth for Object Completion

When the current chunk does not have enough room for continued object growth, the module must allocate a replacement chunk and move the in-progress object into it.

The Rust version must implement behavior equivalent to the source module:

- compute a new chunk size large enough for:
  - the already accumulated object data
  - the additional requested growth length
  - chunk overhead
- link the new chunk into the obstack’s chunk chain
- copy the in-progress object bytes from the previous chunk into the new chunk
- update object base, next-free position, chunk limit, and current chunk references to point into the new chunk
- release the previous chunk if it held no completed object other than the in-progress one being moved
- otherwise retain the previous chunk in the chain

This feature is central to obstack behavior and must remain semantically equivalent in the Rust rewrite.

### 4. Membership Test for Allocated Objects

The module must be able to test whether a supplied object pointer lies within any chunk currently owned by the obstack.

The check must:

- traverse the current chunk chain
- treat a pointer as allocated by the obstack if it falls within the address range covered by one of the retained chunks
- return a boolean-like success/failure result

The Rust rewrite must preserve this chunk-membership semantics rather than introducing stronger provenance or object-identity rules not evidenced by the source.

### 5. Freeing to an Object or Clearing the Obstack

The module must support freeing retained chunks back to a specified object pointer, or freeing the entire obstack when given a null object pointer.

Required behavior:

- when the target pointer is null:
  - release all current chunks through the configured free callback
  - leave the obstack cleared of retained chunk storage
- when the target pointer lies within a retained chunk:
  - free all newer chunks above the chunk containing that pointer
  - set the obstack’s current chunk to the containing chunk
  - reset the obstack’s object base and next-free position to the target pointer
  - set the chunk limit to the end of the retained chunk
- when the target pointer is not found in any retained chunk:
  - invoke the module’s invalid-free failure behavior as defined by the source environment

The Rust version must preserve the success-path semantics exactly and must not silently reinterpret an invalid target pointer as a no-op.

### 6. Memory Usage Reporting

The module must report the total memory currently held by the obstack across all retained chunks.

The reported total must be computed by summing the size of each currently linked chunk. The total must reflect current state after initialization, growth, and free operations.

## User Scenarios & Testing

### Scenario 1: Initialize an Obstack with Simple Callbacks

A caller creates an obstack using callbacks that allocate and free chunks without an extra argument.

Expected result:

- initialization succeeds when the allocator returns a chunk
- the obstack owns one initial chunk
- memory-used reporting returns a positive value reflecting that chunk

Test coverage:

- initialize with nonzero size/alignment
- initialize with zero size and/or zero alignment
- verify failure result when initial chunk allocation fails

### Scenario 2: Initialize an Obstack with Argument-Bearing Callbacks

A caller creates an obstack using callbacks that receive a stored user argument.

Expected result:

- allocation and free operations are dispatched through the argument-bearing callbacks
- the configured argument is passed unchanged to both callback types
- initialization success and failure follow allocator outcome

Test coverage:

- use instrumentation callbacks to record received argument values
- verify both allocation and subsequent free use the same argument

### Scenario 3: Grow an In-Progress Object into a New Chunk

A caller has an obstack with an in-progress object in the current chunk and requests additional space beyond the chunk’s remaining capacity.

Expected result:

- a new chunk is allocated
- the partial object content is copied into the new chunk
- obstack state now points into the new chunk
- the object continues at the same logical content and length as before growth, plus requested additional capacity

Test coverage:

- growth where previous chunk remains retained
- growth where previous chunk is released because it contains no earlier completed object
- resulting current chunk differs from the original one
- object base and next-free offsets are updated consistently

### Scenario 4: Check Whether a Pointer Belongs to the Obstack

A caller asks whether a pointer is allocated within the obstack.

Expected result:

- returns true for pointers within currently retained chunk ranges
- returns false for pointers outside all retained chunks
- after chunks are freed, pointers from released chunks are no longer reported as allocated

Test coverage:

- pointer inside current chunk
- pointer inside an older retained chunk
- pointer outside all chunks
- pointer from a chunk freed by rewind/clear

### Scenario 5: Rewind to a Previously Allocated Object

A caller frees the obstack back to an object pointer located inside an older retained chunk.

Expected result:

- all chunks newer than the target chunk are freed
- the containing chunk becomes current
- object base and next-free are reset to the target pointer
- subsequent memory-used reporting reflects only retained chunks

Test coverage:

- rewind to an object in the current chunk
- rewind to an object in an older chunk
- verify newer chunks are released in order through the configured free callback

### Scenario 6: Clear the Entire Obstack

A caller frees with a null object pointer.

Expected result:

- all retained chunks are released
- no previous chunk remains reported as allocated
- memory-used reporting returns zero

Test coverage:

- clear immediately after initialization
- clear after one or more growth operations

### Scenario 7: Report Memory Usage

A caller queries memory usage across state transitions.

Expected result:

- after initialization, usage equals the initial retained chunk total
- after growth, usage increases to reflect all retained chunks
- after rewind or clear, usage decreases to reflect only currently retained chunks

Test coverage:

- compare usage before and after growth
- compare usage before and after rewind
- compare usage before and after clear

## Requirements

### Functional Requirements

#### FR-1: Initialization with Basic Callbacks
The Rust module shall initialize an obstack using allocation and free callbacks of the form `(size) -> chunk` and `(chunk) -> void`, allocate an initial chunk, set the obstack’s active state to that chunk, and report success or failure based on initial allocation outcome.

Traceability: `_obstack_begin`, `_obstack_begin_worker`, `call_chunkfun`, `call_freefun`, `struct obstack`, `struct _obstack_chunk`.

#### FR-2: Initialization with Argument-Bearing Callbacks
The Rust module shall initialize an obstack using allocation and free callbacks of the form `(arg, size) -> chunk` and `(arg, chunk) -> void`, preserving and reusing the provided argument for later chunk allocation and release.

Traceability: `_obstack_begin_1`, `_obstack_begin_worker`, `call_chunkfun`, `call_freefun`, `struct obstack`.

#### FR-3: Defaulting of Size and Alignment Inputs
The Rust module shall accept requested chunk size and alignment values during initialization and shall convert zero-valued inputs into usable operational values before allocating the initial chunk.

Traceability: `_obstack_begin_worker`, `struct obstack`.

#### FR-4: Callback-Based Chunk Lifecycle
The Rust module shall perform chunk allocation and chunk release only through the callbacks configured on the obstack and shall dispatch to the correct callback signature according to obstack configuration.

Traceability: `call_chunkfun`, `call_freefun`, `_obstack_begin`, `_obstack_begin_1`, `_obstack_newchunk`, `_obstack_free`, `struct obstack`.

#### FR-5: Growth by Allocating a New Chunk
The Rust module shall support growing an in-progress object by allocating a new chunk when the current chunk lacks sufficient room for an additional requested length.

Traceability: `_obstack_newchunk`, `struct obstack`, `struct _obstack_chunk`.

#### FR-6: Preservation of In-Progress Object Data
When growth moves an in-progress object to a new chunk, the Rust module shall preserve the accumulated object bytes by copying them into the new chunk and updating obstack state to reference the copied object.

Traceability: `_obstack_newchunk`, `struct obstack`.

#### FR-7: Conditional Release of Previous Chunk During Growth
When a new chunk is allocated for growth, the Rust module shall release the prior current chunk if that chunk contained no earlier completed object that must remain retained; otherwise it shall keep that chunk linked as part of the obstack.

Traceability: `_obstack_newchunk`, `call_freefun`, `struct obstack`, `struct _obstack_chunk`.

#### FR-8: Membership Testing
The Rust module shall determine whether a supplied pointer lies within the address range of any currently retained chunk in the obstack and shall return a boolean-like result.

Traceability: `_obstack_allocated_p`, `struct obstack`, `struct _obstack_chunk`.

#### FR-9: Free-to-Object Rewind
The Rust module shall free all chunks newer than the chunk containing a supplied target object pointer, then reset the obstack’s current position so that both object base and next-free refer to that target pointer within the retained chunk.

Traceability: `_obstack_free`, `call_freefun`, `struct obstack`, `struct _obstack_chunk`.

#### FR-10: Full Clear on Null Target
The Rust module shall free all retained chunks when the free operation is invoked with a null target pointer.

Traceability: `_obstack_free`, `call_freefun`, `struct obstack`, `struct _obstack_chunk`.

#### FR-11: Invalid Free Detection
The Rust module shall preserve the source module’s invalid-target behavior for free operations when the supplied non-null target pointer is not contained in any currently retained chunk, and shall not treat such input as successful rewind or silent no-op.

Traceability: `_obstack_free`, `struct obstack`, `struct _obstack_chunk`.

#### FR-12: Memory Usage Reporting
The Rust module shall report total current memory usage by summing the sizes of all retained chunks linked in the obstack.

Traceability: `_obstack_memory_used`, `struct obstack`, `struct _obstack_chunk`.

### Key Entities

#### `obstack`
Primary state holder for the chunked object-building arena.

Functional role evidenced by the module:

- stores allocator/free callback configuration
- stores optional extra callback argument
- identifies the current active chunk
- tracks the current object’s base position
- tracks the next free position for continued growth
- tracks the limit of writable space in the current chunk
- stores chunk size and alignment-related initialization state needed for operation

Relationship to other entities:

- owns a chain of `_obstack_chunk` instances
- uses callback configuration to allocate and free those chunks
- exposes behavior through initialization, growth, membership test, free, and memory-usage operations

Traceability: all analyzed functions reference `struct obstack`.

#### `_obstack_chunk`
Represents one retained memory chunk belonging to an obstack.

Functional role evidenced by the module:

- has a size used by memory accounting
- participates in a linked chain of retained chunks
- provides an address range used for membership checks
- serves as storage for current or older object data
- can be released through the obstack’s configured free callback

Relationship to other entities:

- linked into the chunk chain owned by an `obstack`
- may be current or older relative to the obstack’s active position
- may be freed during growth replacement or rewind/clear operations

Traceability: `_obstack_begin_worker`, `_obstack_newchunk`, `_obstack_allocated_p`, `_obstack_free`, `_obstack_memory_used`.

## Success Criteria

### SC-1: Initialization Outcome
For both initialization forms, when the configured allocator returns a valid initial chunk, initialization succeeds and leaves the obstack with one current retained chunk; when the allocator fails, initialization reports failure.

Traceability: `_obstack_begin`, `_obstack_begin_1`, `_obstack_begin_worker`.

### SC-2: Correct Callback Dispatch
Tests using instrumented callbacks confirm that allocation and free operations call the callback variant matching the obstack configuration and, for argument-bearing callbacks, pass the same stored argument value on every invocation.

Traceability: `call_chunkfun`, `call_freefun`, `_obstack_begin`, `_obstack_begin_1`, `_obstack_newchunk`, `_obstack_free`.

### SC-3: Growth Preserves Object State
When growth requires a new chunk, the resulting obstack state references the new chunk, the in-progress object bytes are preserved exactly, and the new writable capacity is sufficient for the previous object content plus requested additional length.

Traceability: `_obstack_newchunk`.

### SC-4: Membership Semantics
For test pointers placed inside retained chunks, membership testing returns true; for pointers outside all retained chunks or belonging only to chunks already freed by rewind/clear, membership testing returns false.

Traceability: `_obstack_allocated_p`, `_obstack_free`.

### SC-5: Rewind Behavior
Freeing to a valid object pointer releases all newer chunks, retains the containing chunk, and resets object base and next-free to the target pointer.

Traceability: `_obstack_free`.

### SC-6: Full Clear Behavior
Freeing with a null target releases all retained chunks, after which memory-used reporting returns zero and membership testing no longer reports pointers from formerly retained chunks as allocated.

Traceability: `_obstack_free`, `_obstack_memory_used`, `_obstack_allocated_p`.

### SC-7: Memory Accounting
Memory-used reporting equals the sum of all currently retained chunk sizes after initialization, after one or more growth operations, and after rewind or clear operations.

Traceability: `_obstack_memory_used`, `_obstack_newchunk`, `_obstack_free`.

### SC-8: Invalid Free Is Not Accepted as Success
A free operation with a non-null pointer not contained in any retained chunk does not produce successful rewind semantics or silent success; tests must observe the preserved invalid-target behavior defined by the source-compatible design.

Traceability: `_obstack_free`.