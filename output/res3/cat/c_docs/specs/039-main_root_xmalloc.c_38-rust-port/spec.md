# spec.md

## Title
Rust Functional Specification for `main_root_xmalloc.c_38`

## Metadata
- Project: `cat`
- Module: `main_root_xmalloc.c_38`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust branch: `039-main_root_xmalloc.c_38-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides allocation helper functions for creating zero-initialized memory blocks and duplicated memory/string buffers. The Rust rewrite must preserve the observable behavior of these helpers as a small utility module used by higher-level code that needs:

- allocation of arrays or buffers from element counts and element sizes,
- duplication of arbitrary byte regions,
- duplication of arbitrary byte regions with an added trailing NUL byte,
- duplication of C strings.

The module specification is limited to the functionality evidenced by the analyzed source functions:
`xcalloc`, `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, and `xstrdup`.

## Feature Specification

### Feature: Zero-initialized allocation helpers
The module shall provide allocation operations that return newly allocated memory sized from a count and an element size, with the allocated region initialized to zero.

This feature covers:
- `xcalloc(size_t n, size_t s)`
- `xicalloc(idx_t n, idx_t s)`

Required behavior:
- Accept two size operands representing element count and element size.
- Produce a memory region whose total requested size is the product of those operands.
- Ensure the returned region is zero-initialized across its full allocated extent.
- Support both the `size_t`-based variant and the `idx_t`-based variant as distinct entry points because both are present in the source module.

### Feature: Raw memory duplication
The module shall provide allocation-backed duplication of an existing memory region of a caller-specified length.

This feature covers:
- `xmemdup(void const *p, size_t s)`
- `ximemdup(void const *p, idx_t s)`

Required behavior:
- Accept a source memory region pointer/reference plus an explicit byte length.
- Allocate a new region of exactly the requested length.
- Copy the specified bytes from the source region into the newly allocated region.
- Return the duplicated region independently from the source region.
- Support both the `size_t`-length and `idx_t`-length variants.

### Feature: Memory duplication with appended NUL terminator
The module shall provide duplication of an arbitrary memory region into a newly allocated character buffer with one additional trailing zero byte.

This feature covers:
- `ximemdup0(void const *p, idx_t s)`

Required behavior:
- Accept a source memory region and explicit byte length.
- Allocate enough space for the copied bytes plus one extra byte.
- Copy exactly the requested source bytes.
- Set the extra final byte to `'\0'`.
- Return a character buffer suitable for consumers that require a NUL-terminated byte sequence, while preserving the original bytes unchanged before the terminator.

### Feature: C-string duplication
The module shall provide duplication of an input NUL-terminated string into newly allocated storage.

This feature covers:
- `xstrdup(char const *string)`

Required behavior:
- Accept a NUL-terminated string.
- Allocate a new character buffer large enough to hold the string content and its terminating NUL byte.
- Copy the full string including termination semantics.
- Return a duplicated string buffer independent from the source buffer.

## User Scenarios & Testing

### Scenario 1: Allocate a cleared buffer for later writes
A caller needs a new buffer for computed output and requires all bytes to start as zero.

Expected support:
- Calling the zero-initialized allocation helper with a count and element size returns newly allocated cleared memory.
- A test shall verify that each byte in the allocated region is zero before any caller writes occur.

Traceability:
- `xcalloc`
- `xicalloc`

### Scenario 2: Duplicate binary data that may contain zero bytes
A caller has a non-string memory region, possibly containing embedded `0x00` bytes, and needs an owned copy.

Expected support:
- Calling the raw memory duplication helper with a source pointer/reference and explicit byte length returns a new region with identical byte contents.
- A test shall verify byte-for-byte equality for the requested length.
- A test shall verify the returned region is distinct from the source storage.

Traceability:
- `xmemdup`
- `ximemdup`

### Scenario 3: Convert a fixed-length byte region into a NUL-terminated character buffer
A caller has a byte span that is not terminated and needs a newly allocated buffer that can be consumed as a C-style string-compatible buffer.

Expected support:
- Calling the NUL-appending duplication helper returns a buffer containing the original bytes followed by a trailing zero byte.
- A test shall verify that the first `s` bytes equal the source data and byte `s` is `'\0'`.

Traceability:
- `ximemdup0`

### Scenario 4: Duplicate an existing C string
A caller has a NUL-terminated input string and needs an owned duplicate.

Expected support:
- Calling the string duplication helper returns a new string buffer with the same textual content.
- A test shall verify content equality and independent storage from the source buffer.

Traceability:
- `xstrdup`

### Scenario 5: Support both native-size and index-size entry points
Higher-level code uses both `size_t`-based and `idx_t`-based interfaces depending on the size representation already in use.

Expected support:
- The Rust rewrite shall preserve the functional distinction of the two API families where present in the source module.
- Tests shall exercise both families with representative nonzero lengths.

Traceability:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`

## Requirements

### Functional Requirements

#### FR-1: Zero-initialized allocation
The module shall provide a function corresponding to `xcalloc` that allocates a new memory region from `(n, s)` and returns it zero-initialized.

Traceability:
- `xmalloc.c`
- `xcalloc`

#### FR-2: Index-sized zero-initialized allocation
The module shall provide a function corresponding to `xicalloc` that allocates a new memory region from `(n, s)` using the module’s index-sized inputs and returns it zero-initialized.

Traceability:
- `xmalloc.c`
- `xicalloc`

#### FR-3: Byte-counted memory duplication
The module shall provide a function corresponding to `xmemdup` that duplicates exactly `s` bytes from an input memory region into newly allocated storage.

Traceability:
- `xmalloc.c`
- `xmemdup`

#### FR-4: Index-sized memory duplication
The module shall provide a function corresponding to `ximemdup` that duplicates exactly `s` bytes from an input memory region into newly allocated storage using the module’s index-sized length input.

Traceability:
- `xmalloc.c`
- `ximemdup`

#### FR-5: Memory duplication with trailing terminator
The module shall provide a function corresponding to `ximemdup0` that duplicates exactly `s` bytes from an input memory region, allocates one additional byte, and stores a trailing `'\0'` byte after the copied region.

Traceability:
- `xmalloc.c`
- `ximemdup0`

#### FR-6: NUL-terminated string duplication
The module shall provide a function corresponding to `xstrdup` that duplicates an input NUL-terminated string into newly allocated character storage.

Traceability:
- `xmalloc.c`
- `xstrdup`

#### FR-7: Returned storage independence
For each duplication function in this module, the returned storage shall be newly allocated and independent of the source storage.

Traceability:
- `xmalloc.c`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### Key Entities

#### Entity: Allocated memory region
A newly created owned memory block returned by the allocation and duplication helpers.

Relationships:
- Produced by all functions in this module.
- For duplication helpers, its initial contents are derived from a caller-provided source region or source string.
- For zero-initialized allocation helpers, its initial contents are all zeros.

Traceability:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

#### Entity: Source memory region
A caller-supplied byte region paired with an explicit length for raw duplication operations.

Relationships:
- Consumed by `xmemdup`, `ximemdup`, and `ximemdup0`.
- Copied into an allocated memory region.
- Not modified by module behavior described here.

Traceability:
- `xmemdup`
- `ximemdup`
- `ximemdup0`

#### Entity: Source C string
A caller-supplied NUL-terminated string used as input to string duplication.

Relationships:
- Consumed by `xstrdup`.
- Copied into newly allocated character storage including string termination semantics.

Traceability:
- `xstrdup`

#### Entity: Size operands
Count, element size, or byte length values supplied by the caller.

Relationships:
- `(n, s)` determines requested allocation extent for `xcalloc` and `xicalloc`.
- `s` determines copied byte count for `xmemdup`, `ximemdup`, and `ximemdup0`.
- String length is derived from the source C string for `xstrdup`.

Traceability:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

## Success Criteria

### SC-1: Correct zero initialization
Tests for the Rust module shall show that memory returned by the `xcalloc`-equivalent and `xicalloc`-equivalent functions is zeroed over the full requested extent for representative input sizes.

Traceability:
- `xcalloc`
- `xicalloc`

### SC-2: Correct raw duplication
Tests for the Rust module shall show that the `xmemdup`-equivalent and `ximemdup`-equivalent functions return newly allocated regions whose first `s` bytes exactly match the source bytes for representative binary inputs, including inputs with embedded zero bytes.

Traceability:
- `xmemdup`
- `ximemdup`

### SC-3: Correct NUL-appending duplication
Tests for the Rust module shall show that the `ximemdup0`-equivalent function returns a region where bytes `0..s-1` equal the source bytes and byte `s` is zero.

Traceability:
- `ximemdup0`

### SC-4: Correct string duplication
Tests for the Rust module shall show that the `xstrdup`-equivalent function returns a new string buffer with content equal to the input C string.

Traceability:
- `xstrdup`

### SC-5: Source and result are distinct storage
Tests for all duplication helpers in the Rust module shall confirm that the returned allocation does not alias the source storage.

Traceability:
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### SC-6: Coverage of both size families
Tests shall exercise both native-size and index-size API variants present in the source module and confirm equivalent functional outcomes for matching practical input values.

Traceability:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`