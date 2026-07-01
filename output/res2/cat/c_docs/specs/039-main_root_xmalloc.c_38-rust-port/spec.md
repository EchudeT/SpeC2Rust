# spec.md

## Title
Rust Functional Specification for `main_root_xmalloc.c_38`

## Document Control
- Project: `cat`
- Module: `main_root_xmalloc.c_38`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust branch: `039-main_root_xmalloc.c_38-rust-port`
- Generation date: 2026-06-07

## Overview
This module provides allocation-adjacent helper behavior for callers that need duplicated memory blocks, duplicated strings, and zero-initialized newly allocated regions. The Rust rewrite must preserve the module’s externally observable behavior for the functions evidenced in `xmalloc.c`:

- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

The module’s responsibility is limited to creating owned copies of caller-supplied data or creating zeroed storage of a requested size. No additional capabilities are required by this specification.

## Feature Specification

### Summary
The Rust version must implement a small allocation utility surface that supports:
- allocation of zero-initialized memory for element count and element size inputs,
- duplication of arbitrary byte regions into newly allocated owned memory,
- duplication of arbitrary byte regions with an added trailing zero byte,
- duplication of NUL-terminated strings into newly allocated owned memory.

### In-Scope Behavior
The Rust rewrite must support the following functional behaviors evidenced by the source module:

1. **Zero-initialized allocation by `size_t`-sized inputs**
   - Accept a count and element size.
   - Produce newly allocated storage sized for `count * size`.
   - The allocated region must be zero-initialized.

2. **Zero-initialized allocation by `idx_t`-sized inputs**
   - Accept a count and element size in the module’s index-sized domain.

3. **Raw memory duplication by byte length**
   - Accept a source memory region and a length.
   - Produce a newly allocated region of exactly that length.
   - Copy the source bytes into the new region.

4. **Raw memory duplication by index-sized length**
   - Accept a source memory region and a length in the module’s index-sized domain.

5. **Raw memory duplication with trailing zero byte**
   - Produce a newly allocated character buffer whose content begins with the copied source bytes and ends with a single zero byte.
   - The added zero byte is part of the result contract and follows the copied region.

6. **String duplication**
   - Accept a NUL-terminated string.
   - Produce a newly allocated string with the same byte content up to and including the terminating zero.

### Out of Scope
The Rust rewrite must not claim or introduce functionality not evidenced by this module, including:
- new public allocation APIs beyond the listed functions,
- persistence, serialization, or file I/O,
- thread-safety guarantees,
- recovery/fallback allocation policies,
- benchmarking or allocator selection interfaces,
- foreign-function integration requirements.

## User Scenarios & Testing

### Scenario 1: Allocate zeroed storage for a counted collection
A caller needs space for `n` elements of size `s` and expects every byte in the new region to be zero before first use.

**Expected result**
- The returned allocation has length `n * s`.
- Every byte in the region is zero.

**Test coverage**
- Allocate a small region where `n * s > 0`; verify all bytes are zero.
- Allocate with one dimension equal to zero; verify behavior remains well-defined in the Rust API shape chosen for the port and does not fabricate non-zero data.

### Scenario 2: Duplicate a binary memory block
A caller has a buffer containing arbitrary bytes, including zero bytes, and needs an owned copy independent of the source.

**Expected result**
- The duplicate has exactly the requested length.
- The duplicate’s bytes match the source byte-for-byte.
- Mutating the duplicate does not mutate the source.

**Test coverage**
- Duplicate a byte slice containing non-text binary data.
- Duplicate a slice containing embedded zero bytes.
- Verify destination content equality and ownership independence.

### Scenario 3: Duplicate a memory block and append a trailing zero
A caller has a byte sequence that is not necessarily NUL-terminated but needs a newly allocated character buffer with an added terminating zero.

**Expected result**
- The first `s` bytes equal the source bytes.
- The byte at index `s` is zero.

**Test coverage**
- Use a source slice without trailing zero; verify one zero byte is appended.
- Use a source slice containing internal zero bytes; verify only the contractually appended byte is required at the end.

### Scenario 4: Duplicate a C-style string
A caller has a NUL-terminated string and needs an owned duplicate.

**Expected result**
- The result contains the same string data as the source.
- The duplicate is independently owned from the source.

**Test coverage**
- Duplicate an ordinary non-empty string.
- Duplicate an empty string.
- Verify content equality.

### Scenario 5: Use index-sized length variants consistently
A caller working in the module’s `idx_t` size domain needs the same behaviors as the `size_t` variants.

**Expected result**
- `xicalloc` behaves like `xcalloc` for representable equivalent values.
- `ximemdup` behaves like `xmemdup` for representable equivalent values.

**Test coverage**
- For small values representable in both domains, compare observed results of paired functions for equivalent inputs.

## Requirements

### Functional Requirements

#### FR-1: Zero-initialized counted allocation
The module shall provide zero-initialized allocation from `(n, s)` inputs for the behavior represented by `xcalloc` in `xmalloc.c`.

**Traceability**
- `xmalloc.c`: `xcalloc`

#### FR-2: Zero-initialized counted allocation in index-sized domain
The module shall provide zero-initialized allocation from index-sized `(n, s)` inputs for the behavior represented by `xicalloc` in `xmalloc.c`.

**Traceability**
- `xmalloc.c`: `xicalloc`

#### FR-3: Memory duplication by explicit byte length
The module shall provide duplication of a caller-supplied memory region into newly allocated storage of the requested length for the behavior represented by `xmemdup` in `xmalloc.c`.

**Traceability**
- `xmalloc.c`: `xmemdup`

#### FR-4: Memory duplication by explicit index-sized length
The module shall provide duplication of a caller-supplied memory region into newly allocated storage of the requested index-sized length for the behavior represented by `ximemdup` in `xmalloc.c`.

**Traceability**
- `xmalloc.c`: `ximemdup`

#### FR-5: Memory duplication with appended zero byte
The module shall provide duplication of a caller-supplied memory region into newly allocated character storage with one additional trailing zero byte for the behavior represented by `ximemdup0` in `xmalloc.c`.

**Traceability**
- `xmalloc.c`: `ximemdup0`

#### FR-6: NUL-terminated string duplication
The module shall provide duplication of a NUL-terminated input string into newly allocated character storage for the behavior represented by `xstrdup` in `xmalloc.c`.

**Traceability**
- `xmalloc.c`: `xstrdup`

### Key Entities

#### Allocated memory region
A newly created owned memory block returned by the allocation and duplication functions. Depending on the function, the region is either:
- zero-initialized on creation, or
- initialized by copying caller-provided bytes, or
- initialized by copying caller-provided bytes plus one appended zero byte.

**Relationships**
- Produced by `xcalloc` and `xicalloc`.
- Produced and populated from a source region by `xmemdup` and `ximemdup`.
- Produced and populated from a source region with an extra trailing zero by `ximemdup0`.

#### Source memory region
A caller-provided input region whose bytes are read and copied into a newly allocated result.

**Relationships**
- Consumed by `xmemdup`, `ximemdup`, and `ximemdup0`.

#### NUL-terminated source string
A caller-provided C-style string used as the source for string duplication.

**Relationships**
- Consumed by `xstrdup`.
- The resulting allocated string is a newly owned copy of the source string content.

#### Size inputs
The count and length values used to determine allocation and copy extent. This module uses both `size_t`-sized and `idx_t`-sized input domains according to function signature.

**Relationships**
- `xcalloc` and `xmemdup` use `size_t`-domain sizing.
- `xicalloc`, `ximemdup`, and `ximemdup0` use `idx_t`-domain sizing.

## Success Criteria

### SC-1: Zeroed allocation correctness
For representative positive input sizes, results corresponding to `xcalloc` and `xicalloc` contain only zero bytes across the full returned extent.

**Traceability**
- `xcalloc`
- `xicalloc`

### SC-2: Exact copy correctness for raw memory duplication
For representative byte sequences, including embedded zero bytes, results corresponding to `xmemdup` and `ximemdup` match the source bytes exactly for the requested length.

**Traceability**
- `xmemdup`
- `ximemdup`

### SC-3: Trailing-zero duplication correctness
For representative inputs, results corresponding to `ximemdup0` preserve the source bytes in the first `s` positions and place a zero byte at position `s`.

**Traceability**
- `ximemdup0`

### SC-4: String duplication correctness
For representative NUL-terminated strings, including the empty string, results corresponding to `xstrdup` contain the same string content as the source.

**Traceability**
- `xstrdup`

### SC-5: Ownership independence of duplicates
For duplication functions, modifying the returned owned result in tests does not change the original source data used to create it.

**Traceability**
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

### SC-6: Cross-variant behavioral consistency for equivalent small values
Where small test values are representable in both size domains, the `size_t` and `idx_t` variants exhibit equivalent observable results for the same logical allocation or duplication request.

**Traceability**
- `xcalloc`
- `xicalloc`
- `xmemdup`
- `ximemdup`