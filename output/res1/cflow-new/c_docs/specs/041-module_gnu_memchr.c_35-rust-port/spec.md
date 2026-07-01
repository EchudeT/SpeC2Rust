# spec.md

## Title

Functional Specification for `module_gnu_memchr.c_35` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_memchr.c_35`
- Category: `module_cluster`
- Source file: `gnu/memchr.c`
- Primary function: `__memchr`
- Rust branch: `041-module_gnu_memchr.c_35-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides byte-search functionality over a bounded memory region. Its purpose is to scan at most `n` bytes starting at a caller-provided memory address and return the address of the first byte equal to the requested search value, or indicate that no such byte exists within the specified range.

The Rust rewrite must preserve the observable behavior of the source module’s exported functionality: bounded search over arbitrary memory treated as a byte sequence, matching on the low 8 bits of the input character value, and returning either the location of the first match or no result.

## Feature Specification

### Supported functionality

The Rust version must implement the behavior of the source module’s memory search routine:

- Accept a pointer/reference to the start of a memory region.
- Accept an integer search value.
- Accept a byte count limiting the search range.
- Search only within the first `n` bytes of the provided region.
- Compare each byte in the region against the target byte value derived from the input integer.
- Return the position of the first matching byte if one exists within the bounded range.
- Return no match if the target byte does not occur within that range.

### Functional boundaries

The Rust version must preserve the following boundaries evidenced by the source module:

- The module performs search only; it does not modify the input memory.
- The search is byte-oriented rather than character-encoding-aware.
- The result identifies the first matching location only.
- The search does not continue beyond the provided length bound.
- Zero-length searches produce no match.

No additional public functionality beyond this bounded byte search is required by this module specification.

## User Scenarios & Testing

### Scenario 1: Match at the beginning of the region

A caller searches a memory region whose first byte equals the target value.

Expected behavior:
- The module reports a match.
- The returned location corresponds to the first byte of the region.

Test coverage:
- Input region begins with the target byte.
- Length is greater than zero.
- Result points to offset `0`.

### Scenario 2: Match in the middle of the region

A caller searches a buffer where the target byte appears after one or more non-matching bytes.

Expected behavior:
- The module reports a match.
- The returned location corresponds to the earliest matching byte within the searched range.

Test coverage:
- Input contains multiple bytes before the first match.
- Result points to the first matching offset, not a later one.

### Scenario 3: Match at the last searchable byte

A caller searches a region where the target byte appears exactly at offset `n - 1`.

Expected behavior:
- The module reports a match.
- The returned location corresponds to the last byte still inside the allowed search range.

Test coverage:
- Match placed at final in-range position.
- No earlier matches exist.

### Scenario 4: No match within range

A caller searches a region in which the target byte does not appear in the first `n` bytes.

Expected behavior:
- The module reports no match.

Test coverage:
- Buffer contains no matching byte in the bounded region.
- Result is the no-match outcome.

### Scenario 5: Match exists outside the allowed range

A caller provides a region where the target byte appears after the first `n` bytes but not within them.

Expected behavior:
- The module reports no match.
- Bytes beyond the provided bound do not affect the result.

Test coverage:
- Match exists only at offset `>= n`.
- Result is the no-match outcome.

### Scenario 6: Zero-length search

A caller requests a search with `n = 0`.

Expected behavior:
- The module reports no match.
- No bytes are considered part of the search.

Test coverage:
- Any input region with zero length.
- Result is the no-match outcome.

### Scenario 7: Integer search value uses only byte-equivalent value

A caller passes an integer search value outside the range `0..=255`.

Expected behavior:
- The module matches bytes using the byte value represented by the low 8 bits of the integer input.

Test coverage:
- Search with values such as `256 + x`, `-1`, or other non-byte integers.
- Behavior matches search using the corresponding byte value.

## Requirements

### Functional Requirements

#### FR-1: Bounded byte search
Traceability: `gnu/memchr.c`, `__memchr`

The module shall search a caller-supplied memory region as a sequence of bytes, beginning at the provided start location and considering at most `n` bytes.

#### FR-2: First-match result
Traceability: `gnu/memchr.c`, `__memchr`

The module shall return the location of the first byte within the searched range whose value equals the target byte.

#### FR-3: No-match result
Traceability: `gnu/memchr.c`, `__memchr`

If no byte equal to the target value exists within the first `n` bytes, the module shall return a no-match result.

#### FR-4: Byte-value interpretation of search argument
Traceability: `gnu/memchr.c`, `__memchr`

The module shall interpret the integer search argument as a byte comparison value for matching memory contents.

#### FR-5: Range limit enforcement
Traceability: `gnu/memchr.c`, `__memchr`

The module shall not report matches outside the first `n` bytes of the supplied memory region.

#### FR-6: Zero-length behavior
Traceability: `gnu/memchr.c`, `__memchr`

When the supplied length is zero, the module shall return a no-match result.

#### FR-7: Non-mutating behavior
Traceability: `gnu/memchr.c`, `__memchr`

The module shall not alter the contents of the searched memory region.

### Key Entities

#### Memory region
Traceability: `gnu/memchr.c`, `__memchr`

An input memory area provided by the caller, treated as a contiguous sequence of bytes for search purposes.

#### Search byte
Traceability: `gnu/memchr.c`, `__memchr`

The byte value derived from the routine’s integer search argument and used as the equality target during scanning.

#### Search length
Traceability: `gnu/memchr.c`, `__memchr`

The maximum number of bytes from the start of the memory region that are included in the search.

#### Match result
Traceability: `gnu/memchr.c`, `__memchr`

The outcome of the search: either the location of the first matching byte within the permitted range or a no-match result.

#### Entity relationships

- A memory region is searched using one search byte and one search length.
- Search length bounds which portion of the memory region is considered.
- Match result, if present, refers to a location within the bounded portion of the memory region.
- If no matching byte exists within that bounded portion, the match result is absent.

## Success Criteria

### Behavioral correctness

- For any input region and positive length, the Rust port returns a match if and only if the target byte occurs within the first `n` bytes.
  Traceability: FR-1, FR-3, FR-5

- When a match exists, the Rust port returns the earliest matching location within the searched range.
  Traceability: FR-2

- When `n = 0`, the Rust port returns no match.
  Traceability: FR-6

- Search behavior for integer inputs outside `0..=255` is equivalent to searching for the corresponding byte value.
  Traceability: FR-4

- The Rust port does not modify the searched memory contents.
  Traceability: FR-7

### Testability criteria

The Rust port shall be considered complete for this module when automated tests demonstrate all of the following:

- Match at offset `0`.
- Match at a nonzero interior offset.
- Match at offset `n - 1`.
- No match in the searched range.
- Match only beyond the searched range yields no match.
- Zero-length search yields no match.
- Integer search arguments outside the byte range behave consistently with byte-value comparison.
- For cases with multiple matches, the first match is returned.

## Out of Scope

The Rust port specification does not require any functionality not evidenced by the source module, including:

- Additional search variants.
- Reverse search behavior.
- Pattern or substring search.
- Encoding-aware text processing.
- Any public API beyond the behavior corresponding to the module’s existing memory search routine.