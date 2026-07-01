# Functional Specification: `module_gnu_xmalloc.c_57`

## 1. Overview

This module provides allocation helper functions that duplicate memory or strings and perform zero-initialized allocation using size/count inputs. The Rust rewrite must preserve the observable behavior of the C module functions found in `gnu/xmalloc.c`:

- `xicalloc`
- `xmemdup`
- `ximemdup`
- `ximemdup0`
- `xstrdup`

The module’s role is limited to producing newly allocated buffers derived from caller-provided sizes or source data. It does not define complex domain objects or persistent state.

## 2. Feature Specification

### 2.1 Supported Features

The Rust version must implement the following functional behaviors evidenced by the source module:

1. **Zero-initialized allocation from element count and element size**
   - Provide functionality equivalent to `xicalloc(n, s)`.
   - Allocate enough space for `n * s` bytes.
   - Return a newly allocated region whose contents are zero-initialized.

2. **Raw memory duplication using byte size**
   - Provide functionality equivalent to `xmemdup(p, s)`.
   - Allocate a new buffer of `s` bytes.
   - Copy exactly `s` bytes from source memory `p` into the new buffer.

3. **Raw memory duplication using `idx_t` size**
   - Provide functionality equivalent to `ximemdup(p, s)`.
   - Same behavioral contract as raw memory duplication, but with the size supplied as `idx_t`.

4. **Raw memory duplication with trailing zero byte**
   - Provide functionality equivalent to `ximemdup0(p, s)`.
   - Allocate a new character buffer of `s + 1` bytes.
   - Copy exactly `s` bytes from source memory `p`.
   - Set the final byte to `'\0'`.

5. **String duplication**
   - Provide functionality equivalent to `xstrdup(string)`.
   - Allocate a new NUL-terminated string buffer.
   - Copy the full input string including its terminating NUL byte.

### 2.2 Functional Boundary

This module is responsible only for:
- creating newly allocated memory from caller-provided size information or source content,
- copying source bytes into newly allocated storage where applicable,
- appending a terminating zero byte for the dedicated zero-terminated duplication case.

This module is not responsible for:
- parsing or interpreting buffer contents,
- ownership tracking beyond returning newly allocated memory,
- defining higher-level container abstractions.

## 3. User Scenarios & Testing

### 3.1 Scenario: Allocate a zeroed buffer for counted items
A caller needs storage for `n` items of size `s` and expects all bytes to begin as zero.

**Expected behavior**
- The module returns a newly allocated buffer sized for `n * s`.
- Every byte in the returned allocation is zero.

**Test coverage**
- Allocate a nonzero-sized region and verify all bytes are zero.
- Verify that the returned region is distinct from any source input pointer.
- Verify size computation behavior for representative `idx_t` values supported by the Rust port.

### 3.2 Scenario: Duplicate an arbitrary memory block
A caller has a pointer to `s` bytes of data and needs an independent copy.

**Expected behavior**
- The module returns a newly allocated buffer of exactly `s` bytes.
- The returned buffer contains byte-for-byte identical contents to the source region.

**Test coverage**
- Duplicate a buffer containing mixed byte values, including zero bytes.
- Verify equality of contents across the full requested length.
- Verify the destination storage is separate from the source.

### 3.3 Scenario: Duplicate an arbitrary memory block where the size is expressed as `idx_t`
A caller uses the project’s index-sized type for lengths and needs the same duplication behavior.

**Expected behavior**
- The module duplicates exactly `s` bytes from the source.
- The result matches the source contents exactly.

**Test coverage**
- Use representative `idx_t` sizes and verify exact copying.
- Confirm parity of behavior with the byte-size duplication path for equivalent sizes.

### 3.4 Scenario: Duplicate non-NUL-terminated bytes into a NUL-terminated character buffer
A caller has `s` bytes of character data that may not end with `'\0'`, and needs a newly allocated buffer suitable for C-style string use.

**Expected behavior**
- The module returns a new buffer of length `s + 1`.
- The first `s` bytes match the source.
- The last byte is `'\0'`.

**Test coverage**
- Duplicate a byte sequence with no trailing zero and verify termination is added.
- Duplicate a sequence containing embedded zero bytes and verify only the appended final byte is guaranteed as terminator.
- Verify source bytes are preserved exactly in the first `s` positions.

### 3.5 Scenario: Duplicate an existing C string
A caller has a NUL-terminated string and needs an owned copy.

**Expected behavior**
- The module returns a new NUL-terminated string with the same byte sequence as the input string.
- The copied string compares equal to the source string content.

**Test coverage**
- Duplicate an empty string.
- Duplicate a typical ASCII string.
- Verify the returned string is independently allocated and NUL-terminated.

## 4. Requirements

### 4.1 Functional Requirements

- **FR-1:** The module shall provide zero-initialized allocation based on an element count and element size, corresponding to `xicalloc` in `gnu/xmalloc.c`.
- **FR-2:** The module shall provide duplication of an arbitrary memory region given a source pointer and a byte count of type `size_t`, corresponding to `xmemdup` in `gnu/xmalloc.c`.
- **FR-3:** The module shall provide duplication of an arbitrary memory region given a source pointer and a length of type `idx_t`, corresponding to `ximemdup` in `gnu/xmalloc.c`.
- **FR-4:** The module shall provide duplication of `idx_t` bytes into a newly allocated character buffer with one appended trailing zero byte, corresponding to `ximemdup0` in `gnu/xmalloc.c`.
- **FR-5:** The module shall provide duplication of a NUL-terminated input string into newly allocated storage, corresponding to `xstrdup` in `gnu/xmalloc.c`.
- **FR-6:** For all duplication operations in this module, the returned storage shall be newly allocated and shall contain copied data rather than aliasing the source region. Traceable to `xmemdup`, `ximemdup`, `ximemdup0`, and `xstrdup`.
- **FR-7:** For the zero-terminated duplication operation, the appended trailing byte shall be zero and shall follow the copied `s` source bytes exactly. Traceable to `ximemdup0`.

### 4.2 Key Entities

This module has no named internal struct or container types in the provided analysis. Its key entities are functional data inputs and outputs:

- **Source memory region**
  - An input pointer plus a length.
  - Used by `xmemdup`, `ximemdup`, and `ximemdup0`.

- **Source C string**
  - An input pointer to a NUL-terminated byte string.
  - Used by `xstrdup`.

- **Allocated output buffer**
  - A newly created memory region returned by each function.
  - Its size and content are determined by the associated source bytes or requested allocation dimensions.

- **Size values**
  - `size_t` and `idx_t` lengths/counts that determine allocation and copy extent.
  - `xicalloc` uses count and element size.
  - `xmemdup` uses `size_t`.
  - `ximemdup` and `ximemdup0` use `idx_t`.

### 4.3 Entity Relationships

- A **source memory region** and a **size value** determine the contents and length of an **allocated output buffer** for memory duplication functions.
- A **source C string** determines the copied contents and terminating layout of the **allocated output buffer** for string duplication.
- In `ximemdup0`, the **allocated output buffer** is one byte longer than the copied source region to hold the required trailing zero byte.

## 5. Success Criteria

- **SC-1:** A Rust implementation of the `xicalloc` behavior returns a newly allocated buffer whose byte length matches the requested allocation extent and whose contents are all zero for tested nonzero allocation cases.
- **SC-2:** A Rust implementation of the `xmemdup` behavior reproduces the exact `s` source bytes in a separate allocation for all covered test inputs.
- **SC-3:** A Rust implementation of the `ximemdup` behavior reproduces the exact `s` source bytes in a separate allocation for all covered `idx_t`-based test inputs.
- **SC-4:** A Rust implementation of the `ximemdup0` behavior returns a buffer where bytes `0..s` equal the source data and byte `s` equals zero in all covered tests.
- **SC-5:** A Rust implementation of the `xstrdup` behavior returns a NUL-terminated string equal in content to the input string for empty and non-empty tested strings.
- **SC-6:** For each duplication function (`xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`), tests demonstrate that modifying the returned allocation does not modify the original source data, confirming non-aliasing copied storage.
- **SC-7:** The Rust module surface remains limited to the functional scope evidenced by `gnu/xmalloc.c` for this module and does not require additional behavior beyond the five traced functions above.