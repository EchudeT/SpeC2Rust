# spec.md

## Title

Functional Specification: `module_gnu_memchr.c_35` Rust Port

## Document Control

- **Project:** `cflow-new`
- **Module:** `module_gnu_memchr.c_35`
- **Category:** `module_cluster`
- **Source file:** `gnu/memchr.c`
- **Primary function:** `__memchr`
- **Rust branch:** `041-module_gnu_memchr.c_35-rust-port`
- **Generation date:** 2026-06-17

## Overview

This module provides byte-search functionality over a bounded memory region. Its purpose is to examine at most a caller-specified number of bytes starting from a given memory address and locate the first byte equal to a target value.

The Rust rewrite must preserve the observable behavior of the source module: given an input memory region, a target byte value, and a maximum search length, it returns a result corresponding to the first matching byte within the permitted range, or indicates that no such byte exists within that range.

This specification covers only the behavior evidenced by `gnu/memchr.c` and its main function `__memchr`.

## Feature Specification

### Feature: Bounded byte search in memory

The module shall provide the behavior of a memory search routine that:

- accepts a starting memory location,
- accepts a byte value to search for,
- accepts a maximum number of bytes to inspect,
- scans the memory region from the beginning toward higher addresses,
- returns the location of the first matching byte if one is found within the permitted range,
- returns no match if the byte does not occur within the permitted range.

### Functional behavior to preserve in Rust

The Rust version must implement the following behavior exposed by the source module:

1. **First-match search**
   - Search begins at the first byte of the provided memory region.
   - If multiple matching bytes exist within the searched region, the earliest one is the result.

2. **Bounded inspection**
   - No bytes beyond the specified length are considered part of the search.
   - A match occurring after the specified bound must not be reported.

3. **Byte-value comparison**
   - The search compares memory contents as byte values.
   - The target value is treated as a byte-sized comparison value for matching purposes.

4. **Empty-range handling**
   - When the length is zero, the function reports no match.

5. **Result form**
   - If a match is found, the result identifies the matching position within the original memory region.
   - If no match is found, the result indicates absence of a match.

## User Scenarios & Testing

### Scenario 1: Match at the beginning of the region

A caller searches a memory region whose first byte equals the target value.

**Expected behavior**
- The module reports a match at the starting position.

**Test focus**
- Input length greater than zero.
- First byte equal to target.
- Returned result corresponds to offset 0.

### Scenario 2: Match in the middle of the region

A caller searches a region where the target byte first appears after one or more non-matching bytes.

**Expected behavior**
- The module reports the first occurrence only.
- The returned position corresponds to the earliest matching offset.

**Test focus**
- Verify exact offset of first match.
- Verify later matches do not affect the result.

### Scenario 3: Match at the last permitted byte

A caller searches a region where the target occurs exactly at the final byte allowed by the provided length.

**Expected behavior**
- The module reports that final in-range occurrence as a valid match.

**Test focus**
- Boundary correctness at offset `n - 1`.

### Scenario 4: Target absent from the searched range

A caller searches a region of length `n` and the target byte does not occur within those `n` bytes.

**Expected behavior**
- The module reports no match.

**Test focus**
- Verify absence result when all inspected bytes differ.

### Scenario 5: Target present after the permitted range

A caller searches only the first `n` bytes of a larger region, and the target occurs only beyond that bound.

**Expected behavior**
- The module reports no match.

**Test focus**
- Confirm the search is strictly limited to the given length.

### Scenario 6: Zero-length search

A caller requests a search length of zero.

**Expected behavior**
- The module reports no match.

**Test focus**
- No in-range bytes exist to inspect.
- Result is absence regardless of target value.

### Scenario 7: Search over arbitrary byte values

A caller searches data containing byte values across the full byte range, including values that may correspond to non-printable bytes.

**Expected behavior**
- Matching is based on byte equality, not text interpretation.

**Test focus**
- Cases using values such as `0x00`, `0x7F`, `0x80`, and `0xFF`.

## Requirements

### Functional Requirements

- **FR-1:** The Rust module shall provide the functional behavior of `__memchr` from `gnu/memchr.c`: search a memory region for a target byte and identify the first matching byte within a caller-specified maximum length.
- **FR-2:** The search shall examine bytes in forward order starting from the initial address of the provided region.
- **FR-3:** The search shall return the first in-range occurrence of the target byte when one exists.
- **FR-4:** The search shall not report matches outside the first `n` bytes of the provided region.
- **FR-5:** When `n` is zero, the search shall report no match.
- **FR-6:** If the target byte is absent from the searched range, the search shall report no match.
- **FR-7:** Byte comparison shall be based on the byte value derived from the target argument and the byte values stored in the searched memory.
- **FR-8:** When a match is reported, the result shall correspond to the location of that byte within the original searched region.

### Key Entities

- **Memory region**
  - A caller-provided contiguous sequence of bytes beginning at a specified start location.
  - This is the data being searched.

- **Target byte**
  - The byte value the search attempts to locate within the memory region.

- **Search length**
  - The maximum number of bytes in the memory region that are considered searchable for a given call.

- **Search result**
  - Either:
    - a location identifying the first matching byte within the region, or
    - an absence indication when no matching byte exists within the allowed range.

### Entity Relationships

- A **memory region** is searched for a **target byte**.
- A **search length** limits how much of the **memory region** is eligible for comparison.
- A **search result** depends on whether the **target byte** appears within the first `n` bytes of the **memory region**, and if so, at its earliest occurrence.

## Success Criteria

- **SC-1:** For any test case where the target byte occurs at offset `k` and no earlier matching byte exists within the first `n` bytes, the Rust implementation returns a result corresponding to offset `k`.
  **Traceability:** `__memchr` in `gnu/memchr.c`.

- **SC-2:** For any test case where the target byte does not occur within the first `n` bytes, the Rust implementation reports no match.
  **Traceability:** `__memchr` in `gnu/memchr.c`.

- **SC-3:** For any test case where `n = 0`, the Rust implementation reports no match.
  **Traceability:** `__memchr` in `gnu/memchr.c`.

- **SC-4:** For test cases where the target exists both within and after the searched bound, the Rust implementation returns the first in-range occurrence and never a later out-of-range occurrence.
  **Traceability:** `__memchr` in `gnu/memchr.c`.

- **SC-5:** For test cases covering representative byte values including `0x00`, `0x7F`, `0x80`, and `0xFF`, the Rust implementation matches by byte value and produces correct presence/absence results.
  **Traceability:** `__memchr` in `gnu/memchr.c`.

- **SC-6:** For test cases with multiple in-range matches, the Rust implementation always reports the earliest matching position.
  **Traceability:** `__memchr` in `gnu/memchr.c`.

## Out of Scope

The Rust port specification does not require any behavior not evidenced by the source module, including but not limited to:

- new public APIs beyond the behavior corresponding to `__memchr`,
- text or string-specific semantics,
- persistence, serialization, or recovery behavior,
- concurrency guarantees,
- benchmark targets,
- compatibility layers beyond preserving the module’s functional search behavior.