# spec.md

## Title

Rust Functional Specification for `main_root_xmalloc.c_29`

## Metadata

- Project: `pwd`
- Module: `main_root_xmalloc.c_29`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust branch: `029-main_root_xmalloc.c_29-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides allocation helper functions for callers that need to duplicate memory regions, duplicate strings, and allocate zero-initialized storage. The Rust rewrite must preserve the observable behavior of these helpers as allocation-oriented utility functions used by the rest of the program.

The evidenced public functional surface of this module consists of helpers for:

- zero-initialized allocation by element count and element size
- duplication of arbitrary memory ranges
- duplication of arbitrary memory ranges with an appended trailing zero byte
- duplication of NUL-terminated strings

No additional capabilities are specified beyond these behaviors.

## Feature Specification

### Summary

The Rust version must implement a utility module that supports safe rewriting of the allocation helper behavior exposed by the analyzed C module.

### Supported behaviors

1. **Zero-initialized allocation**
   - The module must support allocation of a memory region sized from a count and an element size.
   - The returned region must be initialized to zero.
   - This behavior is required for both `size_t`-based and `idx_t`-based entry points evidenced in the source module.

2. **Raw memory duplication**
   - The module must support creating a newly allocated copy of an existing memory region of a specified length.
   - The duplicate must contain the same byte content as the source region for exactly the requested length.

3. **Raw memory duplication with trailing zero**
   - The module must support creating a newly allocated copy of an existing memory region of a specified length and appending one additional zero byte after the copied region.
   - The copied portion must remain byte-for-byte identical to the source input.

4. **String duplication**
   - The module must support duplicating a NUL-terminated string into newly allocated storage.
   - The duplicate must preserve the source string content and terminator semantics expected from string duplication.

### Out of scope

The Rust version must not introduce or require:
- new public allocation APIs beyond the evidenced helper surface
- persistence, serialization, or recovery features
- thread-safety guarantees not evidenced by the source
- FFI requirements
- benchmarking-oriented behavior

## User Scenarios & Testing

### Scenario 1: Caller needs zero-filled storage for an array

A caller needs storage for `n` elements of size `s` and expects all bytes in the resulting region to be zero.

**Expected support**
- The module provides zero-initialized allocation for count/size inputs.
- The returned storage is sized for the requested multiplication and contains only zero bytes initially.

**Testing guidance**
- Request storage for a small count and size.
- Verify the returned region length matches the requested product.
- Verify all bytes are zero.

### Scenario 2: Caller needs a duplicate of arbitrary binary data

A caller has a memory buffer that may contain non-text bytes and needs an independent allocated copy.

**Expected support**
- The module duplicates exactly `s` bytes from the source region into newly allocated storage.
- The duplicate content matches the source content byte-for-byte.

**Testing guidance**
- Use a source buffer containing mixed byte values, including zero bytes.
- Duplicate the buffer.
- Verify equal contents for the requested length.
- Verify the destination is independently allocated by modifying the duplicate and confirming the source is unchanged.

### Scenario 3: Caller needs binary data copied and made zero-terminated

A caller has a memory region of known length and needs a duplicate with one additional trailing zero byte, suitable for consumers expecting a zero-terminated buffer.

**Expected support**
- The module duplicates exactly the specified input length.
- The result contains an extra zero byte immediately following the copied bytes.

**Testing guidance**
- Duplicate a buffer of known length without requiring the input to already end in zero.
- Verify the first `s` bytes equal the source.
- Verify byte `s` is zero.

### Scenario 4: Caller needs a duplicated string

A caller has a NUL-terminated string and needs an independently allocated copy.

**Expected support**
- The module duplicates the complete string content.
- The duplicate behaves as a standard string duplicate, preserving termination.

**Testing guidance**
- Duplicate a non-empty string and an empty string.
- Verify text equality with the source.
- Verify the duplicate can be modified independently without changing the source.

### Scenario 5: Callers using different size domains need equivalent behavior

Some callers provide sizes through `size_t`-based interfaces, while others use `idx_t`-based interfaces.

**Expected support**
- The Rust rewrite preserves the distinction in accepted size domains where required by the port design.
- Equivalent operations through the two evidenced variants produce equivalent allocation and duplication results for representable values.

**Testing guidance**
- For shared representable test values, compare outcomes of the `size_t`-style and `idx_t`-style allocation or duplication behaviors.
- Verify byte content and resulting lengths are equivalent.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide zero-initialized allocation behavior for a requested element count and element size, corresponding to `xcalloc` in `xmalloc.c`.
- **FR-2**: The module shall provide zero-initialized allocation behavior for `idx_t`-typed count and size inputs, corresponding to `xicalloc` in `xmalloc.c`.
- **FR-3**: The module shall provide duplication of an arbitrary memory region of `size_t` length into newly allocated storage, corresponding to `xmemdup` in `xmalloc.c`.
- **FR-4**: The module shall provide duplication of an arbitrary memory region of `idx_t` length into newly allocated storage, corresponding to `ximemdup` in `xmalloc.c`.
- **FR-5**: The module shall provide duplication of an arbitrary memory region of `idx_t` length with one appended trailing zero byte, corresponding to `ximemdup0` in `xmalloc.c`.
- **FR-6**: The module shall provide duplication of a NUL-terminated string into newly allocated storage, corresponding to `xstrdup` in `xmalloc.c`.
- **FR-7**: For all duplication behaviors, the content of the copied region shall exactly match the source bytes for the requested copied length.
- **FR-8**: For the zero-terminated duplication behavior, the appended extra byte after the copied region shall be zero.
- **FR-9**: For string duplication, the resulting duplicate shall preserve the source string content and termination semantics.
- **FR-10**: The module shall return newly allocated storage for successful allocation and duplication operations so that subsequent modification of the returned storage does not alter the source input.

### Key Entities

This module exposes behavior over memory regions and strings rather than defining its own complex data structures. The key entities evidenced by the source are:

- **Allocation request**
  - A pair of size values representing element count and element size.
  - Used by zero-initialized allocation helpers.

- **Memory region**
  - A source pointer plus a specified byte length.
  - Used by raw memory duplication helpers.

- **Zero-terminated duplicated region**
  - A duplicated memory region with one additional trailing zero byte.
  - Produced specifically by the zero-appending duplication helper.

- **NUL-terminated string**
  - A source character sequence ending in a NUL byte.
  - Used by the string duplication helper.

### Entity relationships

- An **allocation request** produces a newly allocated zero-filled region.
- A **memory region** produces a newly allocated copied region of the same specified length.
- A **memory region** may also produce a **zero-terminated duplicated region** when the caller requires one appended zero byte.
- A **NUL-terminated string** produces a newly allocated duplicated string.

## Success Criteria

- **SC-1**: A test covering `xcalloc`-equivalent behavior confirms that requested storage is zero-initialized for representative non-zero count/size inputs.
- **SC-2**: A test covering `xicalloc`-equivalent behavior confirms that requested storage is zero-initialized for representative `idx_t`-domain inputs.
- **SC-3**: A test covering `xmemdup`-equivalent behavior confirms that the output bytes match the source bytes exactly for the requested length.
- **SC-4**: A test covering `ximemdup`-equivalent behavior confirms that the output bytes match the source bytes exactly for the requested `idx_t` length.
- **SC-5**: A test covering `ximemdup0`-equivalent behavior confirms that bytes `0..s` match the source and that byte `s` is zero.
- **SC-6**: A test covering `xstrdup`-equivalent behavior confirms that duplicated strings are text-equal to their sources for both empty and non-empty inputs.
- **SC-7**: Independence tests confirm that modifying duplicated output storage does not change the original source buffer or string for duplication functions.
- **SC-8**: For values representable in both size domains, paired tests confirm equivalent observable results between the `size_t`-based and `idx_t`-based variants of the same operation class.

## Traceability

- `xcalloc` → FR-1, SC-1
- `xicalloc` → FR-2, SC-2
- `xmemdup` → FR-3, FR-7, FR-10, SC-3, SC-7
- `ximemdup` → FR-4, FR-7, FR-10, SC-4, SC-7
- `ximemdup0` → FR-5, FR-7, FR-8, FR-10, SC-5, SC-7
- `xstrdup` → FR-6, FR-9, FR-10, SC-6, SC-7