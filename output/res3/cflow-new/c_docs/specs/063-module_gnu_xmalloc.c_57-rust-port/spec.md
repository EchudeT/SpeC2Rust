# spec.md

## Title

Rust Functional Specification for `module_gnu_xmalloc.c_57`

## Metadata

- Project: `cflow-new`
- Source module: `gnu/xmalloc.c`
- Module name: `module_gnu_xmalloc.c_57`
- Category: `module_cluster`
- Target branch: `063-module_gnu_xmalloc.c_57-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides allocation helper behavior centered on duplicating memory regions and strings, plus zero-initialized allocation using size parameters that include `idx_t`-based variants. The Rust rewrite must preserve the observable behavior of the source module functions identified for this module boundary.

The covered behavior is limited to:

- allocating zero-initialized storage for a count-and-element-size pair,
- duplicating an arbitrary memory region by size,
- duplicating an arbitrary memory region where size is expressed as `idx_t`,
- duplicating a memory region and appending a trailing zero byte,
- duplicating a NUL-terminated string.

No additional functionality beyond these evidenced behaviors is part of this specification.

## Scope

### In Scope

The Rust module must implement the functional behavior corresponding to these source functions:

- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### Out of Scope

The following are out of scope unless required solely to preserve the observable behavior of the listed functions:

- unrelated allocation helpers from other parts of `gnu/xmalloc.c`,
- new public APIs,
- custom recovery mechanisms,
- thread-safety guarantees,
- serialization or persistence behavior,
- FFI surface design,
- performance targets beyond functional equivalence.

## Feature Specification

### Feature: Zero-initialized allocation with indexed size parameters

The module must provide behavior equivalent to allocating storage for `n` elements of size `s`, where both values are expressed using the module’s indexed size domain (`idx_t`). The resulting storage must be zero-initialized across the full allocated region.

This behavior exists to support callers that need a newly allocated block with all bytes initially set to zero, without requiring the caller to clear the memory after allocation.

### Feature: Exact-size memory duplication

The module must provide behavior equivalent to creating a newly allocated copy of an existing memory region of a caller-specified byte length.

Two forms are required:

- one where the size is expressed as `size_t`,
- one where the size is expressed as `idx_t`.

In both forms, the copied region must preserve the original byte content for exactly the requested number of bytes.

### Feature: Memory duplication with appended trailing zero byte

The module must provide behavior equivalent to copying an arbitrary memory region of length `s` bytes into newly allocated storage and then appending one additional trailing zero byte.

The source input is treated as a raw byte region, not as a required NUL-terminated string. The returned result must therefore contain:

- the original `s` bytes unchanged, and
- a zero byte immediately after them.

This supports callers that need a byte-for-byte copy but also require C-string-compatible termination in the destination.

### Feature: String duplication

The module must provide behavior equivalent to duplicating a NUL-terminated string into newly allocated storage.

The duplicated string must preserve the source character sequence up to and including its terminating NUL byte, yielding an independent allocated copy.

## User Scenarios & Testing

### Scenario 1: Allocate cleared storage for indexed counts

A caller needs storage for `n` fixed-size elements and requires all bytes to begin as zero.

Expected support:

- the caller requests zero-initialized allocation using indexed size values,
- the returned block has capacity for the full requested byte count,
- every byte in the allocated region is zero before caller writes to it.

Suggested tests:

- allocate a non-zero region and verify all bytes are zero,
- allocate a region where either dimension is zero and verify behavior remains valid for the module’s supported semantics,
- verify the returned storage is distinct from any existing buffer.

### Scenario 2: Duplicate a binary memory block by exact size

A caller has a memory buffer that may contain arbitrary bytes, including zero bytes, and needs an independent copy of exactly `s` bytes.

Expected support:

- the module copies exactly `s` bytes from the source region,
- embedded zero bytes do not truncate the copy,
- modifying the original after duplication does not change the duplicate.

Suggested tests:

- duplicate a buffer containing non-text binary data,
- duplicate a buffer containing embedded zero bytes,
- verify content equality for the full requested length,
- verify source and destination occupy different storage.

### Scenario 3: Duplicate a memory block when length is expressed in `idx_t`

A caller tracks sizes in the indexed size type used by the surrounding project and needs the same duplication behavior without converting semantics.

Expected support:

- the module accepts an indexed-length argument,
- the returned duplicate matches the source byte-for-byte for the requested length.

Suggested tests:

- repeat the exact-size duplication tests using the indexed-size entry point,
- verify behavior on representative small and larger lengths supported by the environment.

### Scenario 4: Duplicate bytes and ensure trailing zero termination

A caller has a byte sequence of known length that may not be NUL-terminated and wants a duplicated buffer that is safe to treat as zero-terminated afterward.

Expected support:

- the first `s` bytes of the result equal the source bytes,
- byte `s` of the result is zero,
- the original bytes are preserved even if they include embedded zero values.

Suggested tests:

- duplicate a non-terminated byte slice and verify a trailing zero is added,
- duplicate data already ending in zero and verify one trailing zero still exists at the appended position,
- verify the copied prefix remains unchanged.

### Scenario 5: Duplicate a conventional C string

A caller has a NUL-terminated string and needs an allocated copy with the same textual content.

Expected support:

- the entire string content is copied,
- the terminating NUL is preserved in the duplicate,
- changes to the source storage after duplication do not affect the copy.

Suggested tests:

- duplicate an empty string,
- duplicate a short ASCII string,
- duplicate a longer string,
- verify the duplicate compares equal as a C string and resides in separate storage.

## Requirements

### Functional Requirements

#### FR-1: Zero-initialized indexed allocation

The Rust module shall provide behavior equivalent to `xicalloc`, producing newly allocated storage for `n * s` bytes with all bytes initialized to zero.

Traceability: `gnu/xmalloc.c`, `xicalloc`.

#### FR-2: Memory duplication by `size_t` length

The Rust module shall provide behavior equivalent to `xmemdup`, producing a newly allocated copy of exactly `s` bytes from the source memory region.

Traceability: `gnu/xmalloc.c`, `xmemdup`.

#### FR-3: Memory duplication by `idx_t` length

The Rust module shall provide behavior equivalent to `ximemdup`, producing a newly allocated copy of exactly `s` bytes from the source memory region when the length is expressed as `idx_t`.

Traceability: `gnu/xmalloc.c`, `ximemdup`.

#### FR-4: Memory duplication with trailing zero byte

The Rust module shall provide behavior equivalent to `ximemdup0`, producing newly allocated storage containing the original `s` bytes followed by one zero byte.

Traceability: `gnu/xmalloc.c`, `ximemdup0`.

#### FR-5: NUL-terminated string duplication

The Rust module shall provide behavior equivalent to `xstrdup`, producing a newly allocated duplicate of the input NUL-terminated string.

Traceability: `gnu/xmalloc.c`, `xstrdup`.

#### FR-6: Independent destination storage

For each duplication or allocation behavior in scope, the Rust module shall return newly allocated destination storage that is independent from the source region or from any prior caller-provided storage.

Traceability: `gnu/xmalloc.c`, `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`.

### Key Entities

This module does not define standalone project-specific structs or container types within the analyzed boundary. Its functional entities are allocation and copied byte/string regions.

#### Entity: Indexed size value

A size or element-count value expressed in `idx_t`, used by:

- zero-initialized indexed allocation,
- indexed-length memory duplication,
- zero-terminated indexed-length duplication.

Relationship:
- determines the byte extent to allocate or copy in the `idx_t`-based entry points.

Traceability: `xicalloc`, `ximemdup`, `ximemdup0`.

#### Entity: Raw source memory region

A caller-provided contiguous byte region used as input for memory duplication operations.

Relationship:
- serves as the source for exact-size duplication,
- serves as the source prefix for duplication with appended zero termination.

Traceability: `xmemdup`, `ximemdup`, `ximemdup0`.

#### Entity: NUL-terminated source string

A caller-provided C string terminated by a zero byte.

Relationship:
- serves as the source for string duplication,
- determines copied content through its terminating NUL.

Traceability: `xstrdup`.

#### Entity: Newly allocated destination region

A fresh allocated region returned by each function in scope.

Relationship:
- contains zero-initialized bytes for indexed allocation,
- contains copied bytes for memory duplication,
- contains copied bytes plus appended zero for zero-terminated duplication,
- contains a duplicated C string for string duplication.

Traceability: `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`.

## Success Criteria

### Behavioral Correctness

- For `xicalloc`, test cases confirm that every byte in the returned region is zero for non-zero allocation sizes.
  Traceability: `xicalloc`.

- For `xmemdup`, test cases confirm that the returned region matches the source for exactly the requested byte length, including when the source contains embedded zero bytes.
  Traceability: `xmemdup`.

- For `ximemdup`, test cases confirm the same byte-exact duplication behavior as `xmemdup` for lengths expressed via `idx_t`.
  Traceability: `ximemdup`.

- For `ximemdup0`, test cases confirm that bytes `0..s-1` match the source exactly and byte `s` is zero.
  Traceability: `ximemdup0`.

- For `xstrdup`, test cases confirm that the duplicated string content matches the source string and includes a terminating zero byte.
  Traceability: `xstrdup`.

### Allocation Independence

- For each in-scope duplication function, tests confirm that modifying the source after duplication does not modify the returned destination content.
  Traceability: `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`.

- For zero-initialized allocation, tests confirm that the returned region is newly allocated caller-usable storage rather than an alias of preexisting input memory.
  Traceability: `xicalloc`.

### Interface Coverage

- The Rust rewrite exposes functional coverage for all five in-scope behaviors identified in this module specification, with no omitted source behavior among the listed functions.
  Traceability: `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`.

## Acceptance Notes

Conformance is satisfied when the Rust port reproduces the observable functional behavior of the in-scope source functions and all success criteria above are demonstrated by tests derived from the listed scenarios. No acceptance requirement is implied for behavior outside this module boundary.