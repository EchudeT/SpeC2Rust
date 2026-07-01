# spec.md

## Title
Rust Functional Specification for `module_gnu_xmalloc.c_57`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_xmalloc.c_57`
- Category: `module_cluster`
- Source file: `gnu/xmalloc.c`
- Rust branch: `063-module_gnu_xmalloc.c_57-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides allocation and duplication helpers for sized memory regions and C-style strings. Its role is to create newly allocated memory blocks derived either from element counts and element sizes, from arbitrary byte regions, or from NUL-terminated strings.

The Rust rewrite must preserve the functional behavior evidenced by the analyzed functions:
- zero-initialized allocation for a count-and-size pair
- duplication of a byte region using `size_t` and `idx_t` sized lengths
- duplication of a byte region with an added trailing NUL byte
- duplication of a NUL-terminated string

## Scope
In scope:
- behavior corresponding to `xicalloc`
- behavior corresponding to `xmemdup`
- behavior corresponding to `ximemdup`
- behavior corresponding to `ximemdup0`
- behavior corresponding to `xstrdup`

Out of scope:
- defining new public APIs beyond the evidenced module behavior
- guarantees not evidenced by the source summary, such as thread-safety, serialization, recovery behavior, or FFI surface design
- allocator customization or performance commitments

## Feature Specification

### Feature 1: Zero-initialized allocation from element count and element size
The module shall provide functionality equivalent to allocating memory for `n` elements of size `s`, returning a newly allocated region whose contents are initialized to zero.

Traceability:
- `xicalloc` in `gnu/xmalloc.c`

Behavioral notes:
- The allocated region size is derived from the multiplication of element count and element size.
- The returned region is distinct newly allocated storage.
- The resulting bytes are zero-filled.

### Feature 2: Duplication of an arbitrary memory region using byte length
The module shall provide functionality equivalent to duplicating `s` bytes from an existing memory region `p` into newly allocated memory and returning the duplicate region.

Traceability:
- `xmemdup` in `gnu/xmalloc.c`

Behavioral notes:
- Exactly the requested number of bytes is duplicated.
- The source region is treated as raw memory, not as a string.
- The destination region is newly allocated and independent from the source.

### Feature 3: Duplication of an arbitrary memory region using `idx_t` length
The module shall provide the same duplication behavior as Feature 2, with the input length expressed in the module’s index-sized type.

Traceability:
- `ximemdup` in `gnu/xmalloc.c`

Behavioral notes:
- Exactly the requested number of bytes is duplicated.
- The destination region is newly allocated and independent from the source.

### Feature 4: Duplication of a memory region with an added trailing NUL
The module shall provide functionality equivalent to duplicating `s` bytes from an existing memory region and allocating one additional byte set to `'\0'`.

Traceability:
- `ximemdup0` in `gnu/xmalloc.c`

Behavioral notes:
- The first `s` bytes of the result match the input region.
- The result includes one additional trailing NUL byte after the duplicated payload.
- The result is suitable for consumers expecting a NUL-terminated byte sequence, without changing the copied payload length.

### Feature 5: Duplication of a NUL-terminated string
The module shall provide functionality equivalent to duplicating a NUL-terminated input string into newly allocated storage and returning the duplicate string.

Traceability:
- `xstrdup` in `gnu/xmalloc.c`

Behavioral notes:
- The duplicate preserves the source string contents up to and including the terminating NUL.
- The destination string is newly allocated and independent from the source.

## User Scenarios & Testing

### Scenario 1: Allocate zeroed storage for an array-like region
A caller needs storage for `n` fixed-size elements and expects all bytes to start as zero. The module is used to obtain a new zero-filled region sized from element count and element width.

Required test coverage:
- allocating a non-zero region yields a region whose total bytes are all zero
- the total capacity corresponds to `n * s`
- modifying the returned region does not affect any unrelated memory owned by the caller

Traceability:
- `xicalloc`

### Scenario 2: Copy opaque bytes from an existing buffer
A caller has binary data that may contain zero bytes and must preserve exactly `s` bytes in a newly owned duplicate. The module is used to duplicate the region without treating it as text.

Required test coverage:
- duplicated bytes exactly match the source bytes for a representative binary buffer
- embedded zero bytes are preserved
- changing the source after duplication does not change the copy

Traceability:
- `xmemdup`
- `ximemdup`

### Scenario 3: Produce a NUL-terminated duplicate from non-string bytes
A caller has a byte sequence of known length that is not guaranteed to be terminated. The module is used to create a duplicate with one additional trailing NUL byte so that string-oriented consumers can safely stop at the terminator.

Required test coverage:
- the first `s` bytes of the result equal the source bytes
- byte `s` is NUL
- embedded zero bytes within the first `s` bytes remain unchanged

Traceability:
- `ximemdup0`

### Scenario 4: Duplicate a C-style string
A caller needs an owned copy of a NUL-terminated string. The module is used to produce a distinct duplicate string preserving the original text and terminator.

Required test coverage:
- ordinary non-empty strings are duplicated correctly
- the empty string is duplicated correctly
- mutating the duplicate does not mutate the original source storage

Traceability:
- `xstrdup`

## Requirements

### Functional Requirements

#### FR-1: Zeroed allocation
The Rust module shall allocate a new memory region sized from an element count and element size and initialize the allocated bytes to zero.

Traceability:
- `xicalloc`

#### FR-2: Raw memory duplication with `size_t` length
The Rust module shall duplicate exactly the requested number of bytes from a source memory region into newly allocated memory when given a byte count expressed as `size_t`.

Traceability:
- `xmemdup`

#### FR-3: Raw memory duplication with `idx_t` length
The Rust module shall duplicate exactly the requested number of bytes from a source memory region into newly allocated memory when given a byte count expressed as `idx_t`.

Traceability:
- `ximemdup`

#### FR-4: Raw memory duplication with appended terminator
The Rust module shall duplicate exactly `s` source bytes and append one trailing NUL byte in the allocated result.

Traceability:
- `ximemdup0`

#### FR-5: C-string duplication
The Rust module shall duplicate a NUL-terminated string into newly allocated storage, preserving the full source string contents and terminator.

Traceability:
- `xstrdup`

#### FR-6: Independent ownership of duplicated results
For each duplication behavior in this module, the Rust result shall be stored in newly allocated memory independent from the source region.

Traceability:
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### Key Entities

#### Entity 1: Source memory region
An input byte-addressable region provided by the caller for duplication operations. It is used as the read-only source for copying a specified number of bytes.

Relationships:
- consumed by raw memory duplication
- consumed by terminator-appending duplication

Traceability:
- `xmemdup`
- `ximemdup`
- `ximemdup0`

#### Entity 2: Source C string
An input NUL-terminated character sequence provided by the caller for string duplication.

Relationships:
- consumed by string duplication
- length is determined by the terminating NUL rather than a separate explicit size input

Traceability:
- `xstrdup`

#### Entity 3: Newly allocated output region
A newly allocated destination buffer produced by allocation or duplication operations.

Relationships:
- produced as zeroed storage by count-and-size allocation
- produced as a byte-for-byte copy of a source region
- produced as a copied region plus a trailing NUL
- produced as a duplicated C string

Traceability:
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

#### Entity 4: Size inputs
Numeric size values describing either element count and element size or the number of bytes to duplicate.

Relationships:
- `n` and `s` determine zeroed allocation extent
- `s` determines the number of copied bytes
- `idx_t` and `size_t` forms select the corresponding duplication entry point

Traceability:
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`

## Success Criteria

### SC-1: Zero-fill correctness
For representative valid inputs to zeroed allocation, every byte in the returned region is zero before caller modification.

Traceability:
- `xicalloc`
- FR-1

### SC-2: Exact byte preservation for raw duplication
For representative valid inputs, raw duplication returns a region whose first `s` bytes exactly match the source bytes, including any embedded zero bytes.

Traceability:
- `xmemdup`
- `ximemdup`
- FR-2
- FR-3

### SC-3: Appended terminator correctness
For representative valid inputs, terminator-appending duplication returns a region where bytes `0..s` match the source and byte `s` is NUL.

Traceability:
- `ximemdup0`
- FR-4

### SC-4: String duplication correctness
For representative valid C-string inputs, the duplicated string content matches the source string exactly through its terminating NUL.

Traceability:
- `xstrdup`
- FR-5

### SC-5: Destination independence
For representative valid duplication cases, modifying the source after duplication does not alter the duplicate, and modifying the duplicate does not alter the source.

Traceability:
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`
- FR-6

### SC-6: API coverage parity
The Rust rewrite exposes functionality covering all five evidenced behaviors from the analyzed module and no required behavior in this specification is left unimplemented.

Traceability:
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`