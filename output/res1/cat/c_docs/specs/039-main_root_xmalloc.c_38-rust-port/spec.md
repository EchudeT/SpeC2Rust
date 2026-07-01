# spec.md

## Title
Rust Functional Specification for `main_root_xmalloc.c_38`

## Metadata
- Project: `cat`
- Module: `main_root_xmalloc.c_38`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust branch: `039-main_root_xmalloc.c_38-rust-port`
- Generation date: `2026-06-06`

## Overview
This module provides duplication and zero-initialized allocation helpers for callers that need dynamically allocated memory derived from element counts, byte sizes, or existing byte/string content.

The Rust rewrite must preserve the module’s functional role as a small allocation-support boundary with these behaviors:
- allocate zero-filled memory for a requested count and element size;
- duplicate an arbitrary memory region into newly allocated storage;
- duplicate an arbitrary memory region and append a trailing NUL byte;
- duplicate a C string into newly allocated storage.

The source evidence for this module consists of the following functions in `xmalloc.c`:
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

## Feature Specification

### Summary
The module supplies allocation-oriented helper operations that create new owned memory blocks from caller-provided size information or source buffers. It is a support module for code that wants a single call to both allocate and initialize or copy memory content.

### In-Scope Functionality
The Rust version must implement the following functional behaviors evidenced by the source module:

1. **Zero-initialized allocation by `size_t` dimensions**
   - Given an element count and element size, return a newly allocated memory region large enough for `count * size` bytes.
   - The returned region must be zero-initialized.

2. **Zero-initialized allocation by `idx_t` dimensions**
   - Given an element count and element size expressed with the module’s index-sized integer domain, return a newly allocated zero-initialized memory region large enough for the requested total byte count.

3. **Raw memory duplication by byte length**
   - Given a pointer to existing memory and a byte length, allocate new storage of that length and copy the source bytes into it.

4. **Raw memory duplication by index-sized length**
   - Given a pointer to existing memory and a length in the module’s index-sized integer domain, allocate new storage of that length and copy the source bytes into it.

5. **Memory duplication with trailing terminator**
   - Given a pointer to existing memory and a byte count, allocate new storage large enough for the copied bytes plus one additional trailing byte.
   - Copy the source bytes unchanged.
   - Set the extra trailing byte to `'\0'`.

6. **C-string duplication**
   - Given a NUL-terminated input string, allocate new storage, copy the string contents, and include the trailing NUL terminator in the duplicate.

### Out of Scope
The specification does not require any capabilities not directly evidenced by the source functions, including:
- new public allocation APIs beyond the six evidenced functions;
- custom allocator configuration;
- ownership-sharing facilities;
- resizing/reallocation interfaces;
- persistence, serialization, or recovery behavior;
- concurrency guarantees.

## User Scenarios & Testing

### Scenario 1: Caller needs a zeroed array buffer
A caller needs storage for `n` elements of size `s` and expects all bytes to be initially zero.

**Expected behavior**
- The module returns newly allocated memory covering the requested total byte count.
- Every byte in the allocated region is zero.

**Test coverage**
- Call the `size_t`-based zero-allocation entry point with a small nonzero product and verify all bytes are zero.
- Call the `idx_t`-based zero-allocation entry point with a small nonzero product and verify all bytes are zero.

### Scenario 2: Caller duplicates binary data
A caller has an existing memory region that may contain arbitrary bytes, including zero bytes, and wants an independent copy.

**Expected behavior**
- The module allocates a new region of exactly the requested payload length.
- The copied bytes in the destination match the source bytes exactly.

**Test coverage**
- Duplicate a byte slice containing mixed values including `0x00`.
- Verify content equality and independence from the original buffer.
- Exercise both the `size_t`-length and `idx_t`-length duplication entry points.

### Scenario 3: Caller needs a byte buffer usable as a C string
A caller has non-NUL-terminated byte content of known length and needs a duplicate with an added terminating zero byte.

**Expected behavior**
- The module allocates space for the original byte count plus one byte.
- The leading bytes equal the source payload.
- The final byte is `'\0'`.

**Test coverage**
- Duplicate a byte sequence of known length with no terminator in the source.
- Verify the destination payload matches the source and that exactly one appended trailing zero is present at the next byte.

### Scenario 4: Caller duplicates an existing C string
A caller has a valid NUL-terminated string and needs an owned duplicate.

**Expected behavior**
- The module returns a newly allocated string containing the same bytes up to and including the first NUL terminator.

**Test coverage**
- Duplicate a normal nonempty string and compare string content.
- Duplicate an empty string and verify the result contains only the terminator.

### Scenario 5: Caller uses the returned memory as independent owned storage
A caller relies on the helper not aliasing the original source buffer.

**Expected behavior**
- Returned duplicated memory is distinct from the source region.
- Subsequent modification of one region does not retroactively change the other.

**Test coverage**
- Duplicate mutable source data.
- Modify the source after duplication and verify the duplicate is unchanged, or modify the duplicate and verify the source is unchanged.

## Requirements

### Functional Requirements

#### FR-1: Zeroed allocation from element count and size
The Rust module shall provide functionality corresponding to `xcalloc` in `xmalloc.c` that returns newly allocated zero-initialized storage for a requested element count and element size.

**Traceability**
- `xmalloc.c`: `xcalloc`

#### FR-2: Zeroed allocation using index-sized parameters
The Rust module shall provide functionality corresponding to `xicalloc` in `xmalloc.c` that returns newly allocated zero-initialized storage for a requested element count and element size expressed in the module’s index-sized integer domain.

**Traceability**
- `xmalloc.c`: `xicalloc`

#### FR-3: Byte-for-byte memory duplication by `size_t` length
The Rust module shall provide functionality corresponding to `xmemdup` in `xmalloc.c` that allocates a new memory region of the requested byte length and copies the source memory contents into it unchanged.

**Traceability**
- `xmalloc.c`: `xmemdup`

#### FR-4: Byte-for-byte memory duplication by `idx_t` length
The Rust module shall provide functionality corresponding to `ximemdup` in `xmalloc.c` that allocates a new memory region of the requested length in the index-sized integer domain and copies the source memory contents into it unchanged.

**Traceability**
- `xmalloc.c`: `ximemdup`

#### FR-5: Memory duplication with appended NUL byte
The Rust module shall provide functionality corresponding to `ximemdup0` in `xmalloc.c` that allocates new memory for the source bytes plus one additional byte, copies the source bytes, and writes a trailing NUL byte after the copied region.

**Traceability**
- `xmalloc.c`: `ximemdup0`

#### FR-6: NUL-terminated string duplication
The Rust module shall provide functionality corresponding to `xstrdup` in `xmalloc.c` that duplicates a NUL-terminated input string into newly allocated memory, preserving the string contents and terminator.

**Traceability**
- `xmalloc.c`: `xstrdup`

### Key Entities

#### Allocated Memory Region
A newly created owned storage region returned by the module’s allocation and duplication operations.

**Relationships**
- Produced by all six evidenced functions.
- For duplication functions, it is initialized from caller-provided source content.
- For zero-allocation functions, it is initialized to zero bytes.

#### Source Memory Buffer
An input memory region supplied by the caller for raw duplication operations.

**Relationships**
- Consumed by `xmemdup`, `ximemdup`, and `ximemdup0`.
- Its bytes define the initial contents of the produced allocated memory region.

#### Source C String
A NUL-terminated input string supplied by the caller.

**Relationships**
- Consumed by `xstrdup`.
- Its bytes through the first terminator define the contents of the produced allocated string.

#### Size Parameters
Caller-supplied count and length values expressed either as `size_t` or `idx_t`.

**Relationships**
- Determine the amount of storage to allocate.
- Used by `xcalloc`, `xicalloc`, `xmemdup`, `ximemdup`, and `ximemdup0`.

## Success Criteria

### SC-1: Zero initialization correctness
For representative valid inputs to the Rust equivalents of `xcalloc` and `xicalloc`, inspection of the returned regions shall show that all allocated bytes are zero.

**Traceability**
- `xmalloc.c`: `xcalloc`, `xicalloc`

### SC-2: Raw duplication correctness
For representative valid inputs to the Rust equivalents of `xmemdup` and `ximemdup`, the returned memory regions shall contain exactly the same byte sequence as the source over the requested length.

**Traceability**
- `xmalloc.c`: `xmemdup`, `ximemdup`

### SC-3: Terminator-appending duplication correctness
For representative valid inputs to the Rust equivalent of `ximemdup0`, the returned region shall contain the source bytes unchanged in the first `s` positions and a NUL byte at position `s`.

**Traceability**
- `xmalloc.c`: `ximemdup0`

### SC-4: String duplication correctness
For representative valid NUL-terminated string inputs to the Rust equivalent of `xstrdup`, the returned string shall compare equal in content to the source string and shall include a terminating NUL in storage.

**Traceability**
- `xmalloc.c`: `xstrdup`

### SC-5: Non-aliasing of duplicates
For representative valid duplication calls, the returned storage shall be independent from the source region, demonstrated by modifying one region without changing the other.

**Traceability**
- `xmalloc.c`: `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`

### SC-6: Coverage of both size domains
The test suite for the Rust port shall include exercised paths corresponding to both `size_t`-based and `idx_t`-based entry points where both exist in the source module.

**Traceability**
- `xmalloc.c`: `xcalloc`, `xicalloc`, `xmemdup`, `ximemdup`