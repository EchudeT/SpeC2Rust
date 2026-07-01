# spec.md

## Title

Functional Specification: `main_root_setlocale_null_05`

## Metadata

- Project: `pwd`
- Module: `main_root_setlocale_null_05`
- Category: `main_cluster`
- Rust branch target: `005-main_root_setlocale_null_05-rust-port`
- Source basis:
  - `setlocale_null-unlocked.c`
  - `setlocale_null.c`
- Generation date: 2026-06-07

## Overview

This module provides locale-name query helpers for a requested locale category, specifically for the equivalent of querying the current locale without changing it. It offers:

- an unlocked query path,
- a buffer-writing query path that reports whether the provided storage is sufficient,
- and a higher-level query path returning a pointer-like string result.

The Rust rewrite must preserve the observable behavior of these query operations: obtaining the current locale name for a category, copying it into caller-provided storage when requested, and handling failure cases through return status and null-like results as appropriate.

## Feature Specification

### Feature Summary

The module exposes functionality to retrieve the current locale name associated with a locale category, without requesting a locale change. It supports two usage forms:

1. **Direct string query**: obtain a string representing the current locale for a category.
2. **Reentrant/buffered query**: copy that locale string into a caller-provided buffer and report success or failure.

A locked wrapper layer exists around the unlocked retrieval logic in order to provide the externally used behavior of the module. The Rust version must preserve the same functional boundary: locale-name retrieval for a category, with both pointer-style and buffer-style results.

### In-Scope Behavior

The Rust version must implement the following evidenced behaviors:

- Accept a locale category selector as input.
- Query the current locale value for that category in the “null locale argument” sense.
- Return the locale name as text when available.
- Support writing the locale name into caller-provided storage.
- Detect when the provided output buffer is too small and report failure.
- Propagate failure when the locale cannot be obtained for the requested category.
- Preserve the distinction between:
  - a function returning a string result, and
  - a function returning an integer status while filling a buffer.

### Out of Scope

The Rust version must not add capabilities not evidenced by the source basis, including:

- changing locale settings,
- introducing new public APIs beyond the module-equivalent surface,
- promising stronger synchronization semantics than required by observed behavior,
- persistence, serialization, recovery, or benchmarking features.

## User Scenarios & Testing

### Scenario 1: Query current locale name for a valid category

A caller needs the current locale string for a specific locale category and uses the direct query function.

Expected behavior:

- The module returns a non-null string result when the locale can be determined.
- The returned text corresponds to the current locale for the requested category.

Test coverage:

- Call the direct query path with a valid locale category.
- Verify a string result is returned.
- Verify the string is non-empty when the underlying locale system reports a named locale.

### Scenario 2: Copy current locale name into caller buffer

A caller needs a stable copy of the locale name and provides its own buffer.

Expected behavior:

- On sufficient capacity, the module copies the full locale string into the buffer, including string termination as required by C-compatible semantics.
- The status result indicates success.

Test coverage:

- Provide a buffer larger than the locale string.
- Verify success status.
- Verify the buffer content matches the locale name returned by the direct query path for the same category.

### Scenario 3: Buffer too small

A caller provides a buffer that cannot hold the locale string.

Expected behavior:

- The buffered query reports failure.
- It does not report success for truncated storage.

Test coverage:

- Provide a deliberately undersized buffer.
- Verify the function returns failure status.
- Verify the test does not accept partial-copy success.

### Scenario 4: Invalid or unsupported category

A caller passes a category for which the locale cannot be queried.

Expected behavior:

- The direct query returns a null-like failure result.
- The buffered query returns failure status.

Test coverage:

- Invoke both query forms with an invalid category value if the platform permits such testing.
- Verify failure is surfaced consistently.

### Scenario 5: Higher-level wrapper behavior matches underlying query behavior

A caller uses the public wrapper path rather than the unlocked helper.

Expected behavior:

- For successful queries, wrapper-visible results are equivalent to the underlying locale value for the category.
- For failures, wrapper-visible failure status or null result is preserved.

Test coverage:

- Compare buffered and direct results through the public entry points.
- Verify wrapper paths do not alter successful locale text.
- Verify wrapper paths preserve failure outcomes.

## Requirements

### Functional Requirements

#### FR-1: Locale query by category

The module shall accept a locale category input and query the current locale name for that category using the semantics of a null-locale query.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-2: Direct string-returning query

The module shall provide a functionally equivalent operation that returns the current locale name as a string result for the requested category, or a null-like result on failure.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null`

#### FR-3: Buffered/reentrant query

The module shall provide a functionally equivalent operation that writes the locale name into caller-provided storage and returns an integer success/failure status.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-4: Successful copy requires adequate buffer capacity

When the locale name is available and the provided buffer is large enough to hold the full result, the buffered query shall copy the locale string into that buffer and report success.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock`
- `setlocale_null_r`

#### FR-5: Insufficient buffer is reported as failure

When the locale name is available but the provided buffer is too small for the full string result, the buffered query shall report failure rather than success.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock`
- `setlocale_null_r`

#### FR-6: Failure propagation from locale lookup

When the underlying locale lookup for the requested category fails, the direct query shall return a null-like result and the buffered query shall return failure status.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null`
- `setlocale_null_r`

#### FR-7: Wrapper-visible behavior preservation

The externally used wrapper functions shall preserve the success and failure outcomes of the underlying locale query helpers.

Traceability:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`

### Key Entities

#### Locale category selector

An integer input identifying which locale category is being queried.

Relationships:
- Consumed by all query functions.
- Determines which current locale value is requested.

Traceability:
- Parameters named `category` in all listed functions

#### Locale name string

The textual result representing the current locale for a category.

Relationships:
- Produced by direct query functions.
- Used as the source text for buffer-copying functions.

Traceability:
- Return value of `setlocale_null_unlocked`
- Return value of `setlocale_null`

#### Caller-provided output buffer

Writable storage supplied by the caller for buffered locale retrieval.

Relationships:
- Filled by buffered query functions on success.
- Validity of the operation depends on available capacity.

Traceability:
- Parameters `buf` and `bufsize` in `setlocale_null_r_unlocked`
- Parameters `buf` and `bufsize` in `setlocale_null_r_with_lock`
- Parameters `buf` and `bufsize` in `setlocale_null_r`

#### Integer status result

A success/failure result for buffered query operations.

Relationships:
- Reports whether locale retrieval and copying completed successfully.
- Reflects both lookup failure and insufficient-capacity failure.

Traceability:
- Return value of `setlocale_null_r_unlocked`
- Return value of `setlocale_null_r_with_lock`
- Return value of `setlocale_null_r`

## Success Criteria

### SC-1: Direct query parity

For valid locale categories under test, the Rust rewrite returns the same locale text as the module-equivalent null-locale query behavior, or a failure result when the source query fails.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null`

### SC-2: Buffered query correctness

For valid locale categories and adequately sized buffers, the Rust rewrite’s buffered query returns success and writes the full locale string matching the direct query result.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### SC-3: Small-buffer failure handling

For buffers smaller than required for the locale string, the Rust rewrite’s buffered query returns failure and does not claim success.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock`
- `setlocale_null_r`

### SC-4: Invalid-category failure handling

For invalid or unsupported locale categories reachable in tests, the Rust rewrite returns null-like failure from the direct query path and failure status from the buffered query path.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null`
- `setlocale_null_r`

### SC-5: Wrapper outcome preservation

Observed results from the Rust rewrite’s public wrapper-equivalent entry points match the success/failure behavior of the underlying query logic for the same category and buffer conditions.

Traceability:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`