# spec.md

## Title

Functional Specification: `main_root_setlocale_null_04`

## Metadata

- Project: `cat`
- Module: `main_root_setlocale_null_04`
- Category: `main_cluster`
- Rust branch: `005-main_root_setlocale_null_04-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides locale-query helpers that obtain the current locale name for a requested locale category without changing locale state. It supports two result forms:

- a function that returns a locale name as a string result, and
- a function that writes the locale name into caller-provided storage and reports success or failure.

The Rust rewrite must preserve the observable behavior of these helpers, including:

- querying the current locale for a specified category,
- supporting both direct string-return and caller-buffer output styles,
- handling categories for which the locale query is invalid or unavailable,
- producing a stable result for the public wrapper that remains usable after the call returns,
- and preserving the distinction between unlocked/internal querying behavior and the public entry points that ensure correct use around locale access.

This specification is derived from the module functions:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

and the lock-protected helper variant(s) used internally by `setlocale_null.c`.

## Feature Specification

### Feature: Query current locale name without modifying locale configuration

The module shall provide functionality equivalent to calling locale query operations with a null locale-setting argument: it reads the currently active locale name for a specified category and does not request a locale change.

Behavior that the Rust version must implement:

1. Accept a locale category selector as input.
2. Query the current locale name for that category.
3. Return the locale name in one of two supported forms:
   - as a string result, or
   - by copying it into a caller-provided buffer.
4. Report failure when the category is invalid, the locale query cannot produce a name, or the provided output buffer cannot hold the complete result.
5. Ensure the public string-returning function provides a usable returned string beyond transient internal query storage.

### Feature: Reentrant buffer-based locale query

The module shall provide a buffer-based operation that writes the current locale name into caller-provided storage.

Behavior that the Rust version must implement:

1. Accept:
   - locale category,
   - output buffer,
   - output buffer capacity.
2. On success:
   - copy the complete locale name into the output buffer,
   - include terminating null semantics in the copied representation if modeling C-compatible behavior internally,
   - return a success status.
3. On failure:
   - do not report success,
   - preserve failure signaling for insufficient buffer space and unsuccessful locale lookup.

### Feature: Public wrapper around internal locale query handling

The module includes internal unlocked helpers and public wrappers. The Rust rewrite must preserve the external functional boundary:

- public APIs behave correctly for callers of this module,
- internal helper distinctions remain internal to the Rust module design,
- behavior visible to callers matches the current module’s success/failure and result semantics.

No additional public capabilities are required beyond the evidenced query behavior.

## User Scenarios & Testing

### Scenario 1: Caller needs the current locale name as a string

A caller requests the current locale for a valid category and needs a string result.

Expected behavior:

- the module returns the current locale name for that category;
- the result reflects the current locale state at the time of the call;
- the call does not change locale settings.

Test coverage:

- invoke the string-returning API with a valid category;
- verify a non-failure string result is returned when the underlying locale query succeeds;
- verify the returned string matches the active locale name for that category.

### Scenario 2: Caller needs the locale name copied into its own buffer

A caller provides a writable buffer and asks the module to place the current locale name into that buffer.

Expected behavior:

- if the buffer is large enough, the complete locale name is written successfully;
- success status is returned;
- no locale state is modified.

Test coverage:

- provide a valid category and sufficiently large buffer;
- verify success status;
- verify the copied content equals the current locale name.

### Scenario 3: Buffer is too small

A caller provides an output buffer that cannot hold the locale name.

Expected behavior:

- the module reports failure;
- it does not falsely report success for truncated output.

Test coverage:

- use a valid category with a deliberately undersized buffer;
- verify failure status is returned.

### Scenario 4: Invalid or unsupported locale category

A caller passes a category that does not yield a valid locale query result.

Expected behavior:

- the module reports failure in the buffer-based API;
- the string-returning API reports no valid string result.

Test coverage:

- call each public API with an invalid category value;
- verify failure/null-style result behavior.

### Scenario 5: Public result remains usable after the call returns

A caller uses the string-returning public API and expects the returned value to remain usable after the function exits, rather than referring only to transient query memory.

Expected behavior:

- the public API returns a stable string result consistent with module behavior;
- subsequent caller use of the returned value after function return remains valid according to the module contract.

Test coverage:

- call the public string-returning API;
- retain the returned value and inspect it after the call boundary;
- verify it still contains the expected locale name.

## Requirements

### Functional Requirements

#### FR-1: Locale query by category
The module shall accept a locale category argument and query the current locale name for that category without requesting a locale change.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

#### FR-2: String-returning query interface
The module shall provide a functionally equivalent interface that returns the current locale name as a string result for a requested category.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null`

#### FR-3: Buffer-output query interface
The module shall provide a functionally equivalent interface that writes the current locale name into caller-provided storage and returns an integer success/failure status.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-4: Failure on invalid locale query result
If the current locale name cannot be obtained for the requested category, the module shall report failure rather than fabricate a locale name.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

#### FR-5: Failure on insufficient output capacity
For the buffer-output interface, if the provided buffer is too small to hold the complete locale name representation, the module shall return failure.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-6: Full copy of successful buffer result
For the buffer-output interface, on success the module shall copy the complete locale name into the caller-provided buffer.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-7: Public wrapper preserves stable returned string semantics
For the public string-returning interface, the module shall return a usable string result that is not limited to transient internal query storage at the instant of return.

Traceability:
- `setlocale_null`

#### FR-8: Public wrappers preserve externally visible query behavior
The Rust rewrite shall preserve the externally visible behavior of the public wrappers relative to the internal unlocked helpers, including equivalent success/failure outcomes and locale-name results for the same category and active locale state.

Traceability:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`

### Key Entities

#### Entity: Locale category input
An integer-like locale category selector identifies which locale category is queried.

Relationships:

- consumed by all module entry points;
- determines which current locale name is requested.

Traceability:
- all listed functions

#### Entity: Locale name result
The locale name is the core output value of the module.

Relationships:

- produced directly by the string-returning interfaces;
- copied into caller-owned storage by the buffer-output interfaces;
- absent on failure.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null`
- `setlocale_null_r`

#### Entity: Caller-provided output buffer
Writable storage supplied by the caller receives the locale name for the reentrant/buffer-based interface.

Relationships:

- used only by buffer-output functions;
- capacity determines success or failure for copy-out behavior.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_r_with_lock`

#### Entity: Status result
An integer success/failure result communicates whether the locale name was successfully obtained and copied for buffer-based operations.

Relationships:

- returned by buffer-output functions;
- reflects validity of category, success of locale query, and adequacy of buffer capacity.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_r_with_lock`

## Success Criteria

### SC-1: Correct successful query behavior
For a valid locale category with an obtainable current locale name, the Rust module returns the same locale name content through both the string-returning and buffer-output public interfaces.

Traceability:
- `setlocale_null`
- `setlocale_null_r`

### SC-2: No locale mutation
Calling the Rust module’s query functions does not change the active locale configuration.

Traceability:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null`
- `setlocale_null_r`

### SC-3: Proper failure for invalid category or unavailable locale name
For invalid category input or unsuccessful locale lookup, the Rust module returns failure-equivalent results matching the source module’s public behavior.

Traceability:
- `setlocale_null`
- `setlocale_null_r`

### SC-4: Proper failure for undersized buffer
For the buffer-output public interface, when the caller-provided storage is too small for the complete locale name, the Rust module reports failure.

Traceability:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### SC-5: Stable public string result
The Rust implementation of the public string-returning interface provides a returned string value that remains usable after the function returns, consistent with the source module’s public contract.

Traceability:
- `setlocale_null`

### SC-6: Public wrapper behavior matches internal query outcomes
Where the source module distinguishes internal unlocked querying from public wrapper entry points, the Rust rewrite preserves the same externally observable outcomes for success, failure, and returned locale-name content.

Traceability:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`