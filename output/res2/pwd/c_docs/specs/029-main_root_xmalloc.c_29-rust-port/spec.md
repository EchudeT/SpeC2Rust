# spec.md

## Title

Rust Functional Specification for `main_root_xmalloc.c_29`

## Summary

This module provides allocation-and-copy helper functions for callers that need dynamically allocated memory initialized or populated from existing data. The supported behaviors evidenced by `xmalloc.c` are:

- zero-initialized allocation for element-count and element-size inputs
- duplication of raw memory regions
- duplication of raw memory regions with an added trailing NUL byte
- duplication of C strings

The Rust rewrite for branch `029-main_root_xmalloc.c_29-rust-port` must preserve these observable behaviors for the functions analyzed from `xmalloc.c`.

## Scope

### In Scope

Behavior corresponding to the following functions in `xmalloc.c`:

- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### Out of Scope

The following are not evidenced by the provided module analysis and are therefore not part of this specification:

- additional allocation APIs beyond the listed functions
- custom allocator configuration
- thread-safety guarantees
- serialization or persistence behavior
- FFI design requirements
- recovery policies beyond the function-level behavior evidenced by the source module

## Feature Specification

This module acts as a small utility layer for obtaining newly allocated memory buffers or strings derived from caller-provided sizes or source data.

### Feature 1: Zero-initialized allocation

The module shall provide allocation helpers that accept a count and an element size and return newly allocated storage whose contents are zero-initialized.

This behavior is evidenced by:

- `xcalloc(size_t n, size_t s)`
- `xicalloc(idx_t n, idx_t s)`

The Rust version must implement equivalent behavior for both size domains represented by these two entry points.

### Feature 2: Raw memory duplication

The module shall provide helpers that allocate new storage and copy exactly the requested number of bytes from a source memory region into the new allocation.

This behavior is evidenced by:

- `xmemdup(void const *p, size_t s)`
- `ximemdup(void const *p, idx_t s)`

The Rust version must preserve the distinction that these functions duplicate arbitrary memory bytes, not necessarily text and not necessarily NUL-terminated data.

### Feature 3: Raw memory duplication with trailing NUL

The module shall provide a helper that duplicates a specified memory region and appends one trailing zero byte in the returned allocation.

This behavior is evidenced by:

- `ximemdup0(void const *p, idx_t s)`

The Rust version must ensure that the returned allocation contains the original `s` bytes followed by a trailing NUL byte, making the result suitable for call sites that require copied bytes plus terminator.

### Feature 4: C-string duplication

The module shall provide a helper that duplicates a NUL-terminated string into newly allocated storage.

This behavior is evidenced by:

- `xstrdup(char const *string)`

The Rust version must preserve the behavior that the returned string storage is a separate allocation containing the source string content and terminating NUL.

## User Scenarios & Testing

### Scenario 1: Allocate a zeroed buffer for fixed-size elements

A caller needs storage for `n` elements of size `s`, and requires the initial contents to be all zero.

Expected behavior:

- the module returns newly allocated storage sized for `n * s`
- all bytes in the returned region are zeroed before the caller uses it

Relevant functions:

- `xcalloc`
- `xicalloc`

Suggested tests:

- request a small allocation and verify returned bytes are all zero
- request allocation with `n > 1` and `s > 1` and verify the total length matches the expected product
- verify the `size_t`-based and `idx_t`-based entry points both exhibit the same zero-initialization behavior for equivalent representable sizes

### Scenario 2: Duplicate a binary memory region

A caller has a source buffer containing arbitrary bytes, including possible zero bytes, and needs an independent copy.

Expected behavior:

- the module returns newly allocated storage
- the destination bytes exactly match the source bytes for the requested length
- the copy length is controlled by the explicit size argument, not by scanning for a terminator

Relevant functions:

- `xmemdup`
- `ximemdup`

Suggested tests:

- duplicate a buffer containing embedded zero bytes and verify byte-for-byte equality
- mutate the source after duplication and verify the duplicated buffer remains unchanged
- verify the duplicate resides in separate storage from the source

### Scenario 3: Duplicate bytes and make them NUL-terminated

A caller has a non-NUL-terminated memory region and needs a copied buffer with one extra trailing zero byte.

Expected behavior:

- the first `s` bytes of the result match the source region
- byte `s` of the result is zero
- the result is independently allocated

Relevant function:

- `ximemdup0`

Suggested tests:

- duplicate a byte slice with no trailing zero and verify the extra terminator exists
- duplicate data containing embedded zero bytes before the end and verify the appended terminator is still placed exactly after `s` bytes
- for `s == 0`, verify the result contains a valid trailing zero byte

### Scenario 4: Duplicate a C string

A caller has a NUL-terminated string and needs an independent duplicate.

Expected behavior:

- the returned allocation contains the same characters as the source string
- the returned allocation is separately owned from the source
- the duplicate includes a proper terminating NUL

Relevant function:

- `xstrdup`

Suggested tests:

- duplicate a normal ASCII string and verify textual equality and termination
- duplicate an empty string and verify the result is a valid empty NUL-terminated string
- mutate the original backing storage where possible and verify the duplicate remains unchanged

## Requirements

### Functional Requirements

#### FR-1 Zeroed allocation by `size_t` dimensions

The module shall provide behavior equivalent to `xcalloc` that creates newly allocated storage for `n` elements of size `s`, with the returned memory initialized to zero.

Traceability:

- `xmalloc.c` — `xcalloc`

#### FR-2 Zeroed allocation by `idx_t` dimensions

The module shall provide behavior equivalent to `xicalloc` that creates newly allocated storage for `n` elements of size `s`, with the returned memory initialized to zero, using the `idx_t` size domain.

Traceability:

- `xmalloc.c` — `xicalloc`

#### FR-3 Raw memory duplication by `size_t` length

The module shall provide behavior equivalent to `xmemdup` that allocates new storage and copies exactly `s` bytes from source pointer `p`.

Traceability:

- `xmalloc.c` — `xmemdup`

#### FR-4 Raw memory duplication by `idx_t` length

The module shall provide behavior equivalent to `ximemdup` that allocates new storage and copies exactly `s` bytes from source pointer `p`, using the `idx_t` size domain.

Traceability:

- `xmalloc.c` — `ximemdup`

#### FR-5 Raw memory duplication with appended terminator

The module shall provide behavior equivalent to `ximemdup0` that allocates new storage for `s + 1` bytes, copies exactly `s` bytes from source pointer `p`, and sets the final byte to zero.

Traceability:

- `xmalloc.c` — `ximemdup0`

#### FR-6 NUL-terminated string duplication

The module shall provide behavior equivalent to `xstrdup` that allocates new storage and copies a source NUL-terminated string, including its terminator.

Traceability:

- `xmalloc.c` — `xstrdup`

### Key Entities

This module does not define standalone structs or container types in the provided analysis. Its key entities are function-level data concepts:

#### Entity 1: Source memory region

A caller-provided readable memory range identified by a pointer and explicit byte length.

Used by:

- `xmemdup`
- `ximemdup`
- `ximemdup0`

Relationship:

- copied into a newly allocated destination region

#### Entity 2: Source C string

A caller-provided NUL-terminated character sequence.

Used by:

- `xstrdup`

Relationship:

- measured as a string and copied into a newly allocated destination string including terminator

#### Entity 3: Newly allocated destination region

A fresh memory allocation returned to the caller.

Used by:

- all listed functions

Relationship:

- may be zero-initialized (`xcalloc`, `xicalloc`)
- may contain copied bytes from a source region (`xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`)
- may include an appended trailing zero byte (`ximemdup0`, `xstrdup`)

#### Entity 4: Allocation dimensions

The caller-supplied size values that determine destination capacity.

Used by:

- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`

Relationship:

- define how many bytes are allocated and, where applicable, how many source bytes are copied

## Success Criteria

### SC-1 Zero-initialization correctness

For representative valid inputs to `xcalloc` and `xicalloc`, automated tests confirm that every byte in the returned allocation is zero before any caller modification.

Traceability:

- `xmalloc.c` — `xcalloc`, `xicalloc`

### SC-2 Exact copy correctness for raw memory duplication

For representative valid inputs to `xmemdup` and `ximemdup`, automated tests confirm that the returned allocation matches the source bytes exactly for the requested length, including cases with embedded zero bytes.

Traceability:

- `xmalloc.c` — `xmemdup`, `ximemdup`

### SC-3 Appended NUL correctness

For representative valid inputs to `ximemdup0`, automated tests confirm that:
- the first `s` bytes equal the source bytes, and
- the byte immediately following them is zero.

Traceability:

- `xmalloc.c` — `ximemdup0`

### SC-4 String duplication correctness

For representative valid inputs to `xstrdup`, automated tests confirm that the returned string content equals the source string content and that the duplicate is NUL-terminated.

Traceability:

- `xmalloc.c` — `xstrdup`

### SC-5 Independent allocation behavior

For duplication functions in this module, automated tests confirm that the returned storage is independent from the source storage, demonstrated by lack of aliasing effects after post-copy source mutation where such mutation is testable.

Traceability:

- `xmalloc.c` — `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`