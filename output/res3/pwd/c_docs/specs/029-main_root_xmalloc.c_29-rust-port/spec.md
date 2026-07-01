# spec.md

## Title
Rust Functional Specification for `main_root_xmalloc.c_29`

## Document Metadata
- Project: `pwd`
- Module: `main_root_xmalloc.c_29`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust branch: `029-main_root_xmalloc.c_29-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides allocation helper functionality for creating zero-initialized memory blocks and duplicating existing memory regions or C strings. The Rust rewrite must preserve the observable behavior of these helpers as exposed by the source module.

The scope evidenced by this module is limited to:
- zero-initialized allocation for element-count/element-size pairs
- zero-initialized allocation for `idx_t`-sized counts and widths
- duplication of arbitrary memory regions by size
- duplication of arbitrary memory regions with an added trailing zero byte
- duplication of NUL-terminated C strings

No additional capabilities are specified beyond these behaviors.

## Feature Specification

### Summary
The Rust version must implement helper functionality equivalent to the allocation and duplication behaviors provided by:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### Functional Behavior
1. The module shall support allocating a new memory region sized as a product of element count and element size, with the returned region initialized to zero.
2. The module shall support the same zero-initialized allocation behavior when the size inputs are expressed with `idx_t`.
3. The module shall support duplicating an existing memory region of a specified byte length into newly allocated storage.
4. The module shall support duplicating an existing memory region of `idx_t` length into newly allocated storage.
5. The module shall support duplicating an existing memory region of `idx_t` length into newly allocated storage and appending one extra trailing zero byte.
6. The module shall support duplicating a NUL-terminated C string into newly allocated storage, including the terminating NUL.

### Behavioral Boundaries
- This module is an allocation/duplication helper module only.
- It does not define higher-level ownership models, container abstractions, or persistence behavior.
- It does not expose functionality beyond allocation and duplication of raw memory or C strings.

## User Scenarios & Testing

### Scenario 1: Zero-initialized allocation by element count and element width
A caller needs storage for `n` elements of size `s`, and requires all bytes to begin as zero.

Expected support:
- requesting allocation by `(n, s)`
- receiving a distinct writable memory region sized for `n * s`
- all bytes in the returned region are zero-initialized

Suggested tests:
- allocate a small block and verify all bytes are zero
- allocate with `n = 0` or `s = 0` and verify behavior is well-defined and does not produce copied payload content
- verify the returned region is independent from any other allocation

Traceability:
- `xcalloc`
- `xicalloc`

### Scenario 2: Duplicate a binary memory region
A caller has a pointer to `s` bytes of existing data and needs a separate allocated copy with identical byte contents.

Expected support:
- supplying a source memory region and size
- receiving a newly allocated destination region
- destination bytes exactly match source bytes for the requested length

Suggested tests:
- duplicate a fixed byte array and verify byte-for-byte equality
- modify the duplicate and verify the source is unchanged
- duplicate an empty region length and verify the operation completes consistently

Traceability:
- `xmemdup`
- `ximemdup`

### Scenario 3: Duplicate data and append a trailing zero byte
A caller has non-NUL-terminated data of length `s` and needs a copy that is safe to treat as zero-terminated after the copied payload.

Expected support:
- copying exactly `s` source bytes
- allocating space for one additional byte
- setting the extra byte after the copied payload to zero

Suggested tests:
- duplicate a byte sequence without any final zero and verify:
  - first `s` bytes match source
  - byte at offset `s` is zero
- verify the added zero byte does not alter copied payload bytes

Traceability:
- `ximemdup0`

### Scenario 4: Duplicate a C string
A caller has a NUL-terminated string and needs an allocated copy.

Expected support:
- reading source content until the terminating NUL
- allocating sufficient storage for all characters plus the terminating NUL
- returning a copy with identical string contents and terminator

Suggested tests:
- duplicate a normal ASCII C string and verify content equality
- duplicate an empty string and verify the result contains only the terminating NUL
- modify the duplicate and verify the source string is unchanged

Traceability:
- `xstrdup`

## Requirements

### Functional Requirements

#### FR-1 Zero-initialized allocation for `size_t`
The Rust module shall provide functionality equivalent to allocating memory for `n * s` bytes from `size_t` inputs and initializing the allocated region to zero.

Traceability:
- `xcalloc` in `xmalloc.c`

#### FR-2 Zero-initialized allocation for `idx_t`
The Rust module shall provide functionality equivalent to allocating memory for `n * s` bytes from `idx_t` inputs and initializing the allocated region to zero.

Traceability:
- `xicalloc` in `xmalloc.c`

#### FR-3 Raw memory duplication for `size_t` length
The Rust module shall provide functionality equivalent to allocating a new region of `s` bytes and copying the `s` bytes from the source region into it.

Traceability:
- `xmemdup` in `xmalloc.c`

#### FR-4 Raw memory duplication for `idx_t` length
The Rust module shall provide functionality equivalent to allocating a new region of `s` bytes and copying the `s` bytes from the source region into it when `s` is expressed as `idx_t`.

Traceability:
- `ximemdup` in `xmalloc.c`

#### FR-5 Raw memory duplication with added trailing zero
The Rust module shall provide functionality equivalent to allocating a new region large enough for `s` copied bytes plus one additional trailing zero byte, copying the first `s` bytes from the source region, and storing zero in the extra byte.

Traceability:
- `ximemdup0` in `xmalloc.c`

#### FR-6 C-string duplication
The Rust module shall provide functionality equivalent to duplicating a NUL-terminated C string into newly allocated storage, preserving all source characters and the terminating NUL.

Traceability:
- `xstrdup` in `xmalloc.c`

### Key Entities

#### Allocated memory region
A newly created writable memory block returned by the module. Depending on the operation, the block contains:
- all-zero bytes
- an exact byte copy of a source region
- an exact byte copy of a source region followed by one zero byte
- a duplicated C string including its terminating NUL

Relationships:
- produced by all functions in this module
- may be based on a source memory region or source C string

#### Source memory region
A caller-provided input region interpreted as a sequence of bytes with a caller-specified length.

Relationships:
- consumed by raw-memory duplication operations
- copied into an allocated memory region

Traceability:
- `xmemdup`
- `ximemdup`
- `ximemdup0`

#### Source C string
A caller-provided NUL-terminated character sequence.

Relationships:
- consumed by C-string duplication
- copied into an allocated memory region including terminator

Traceability:
- `xstrdup`

#### Size inputs
Numeric inputs describing allocation extent or duplication length, represented in this module as either `size_t` or `idx_t`.

Relationships:
- determine the number of bytes allocated or copied
- bind operation variants to standard-size or index-sized interfaces

Traceability:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`

## Success Criteria

### SC-1 Zeroed allocation correctness
For inputs requesting zero-initialized allocation, the Rust implementation returns a newly allocated region whose observable byte contents are zero across the requested extent.

Traceability:
- FR-1
- FR-2

### SC-2 Exact duplication correctness
For raw-memory duplication operations, the Rust implementation returns a newly allocated region whose first `s` bytes exactly equal the source region's first `s` bytes.

Traceability:
- FR-3
- FR-4

### SC-3 Added terminator correctness
For the zero-appending duplication operation, the Rust implementation returns a newly allocated region where:
- bytes `0..s` equal the source bytes
- byte `s` equals zero

Traceability:
- FR-5

### SC-4 C-string duplication correctness
For C-string duplication, the Rust implementation returns a newly allocated region containing the same character sequence as the source string and ending with a terminating NUL.

Traceability:
- FR-6

### SC-5 Independence of returned storage
For every duplication or allocation operation in this module, writes to the returned region after creation do not alter the caller's source region or any separately returned allocation from another call.

Traceability:
- FR-1
- FR-2
- FR-3
- FR-4
- FR-5
- FR-6

### SC-6 API-scope conformance
The Rust rewrite implements only the allocation and duplication behaviors evidenced by this module and does not require unrelated higher-level behaviors to satisfy this specification.

Traceability:
- module scope evidenced by `xmalloc.c`