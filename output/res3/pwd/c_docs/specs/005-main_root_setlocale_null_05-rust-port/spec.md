# spec.md

## Title

Functional Specification: `main_root_setlocale_null_05`

## Overview

This module provides locale-query helpers that obtain the current locale name for a requested locale category using `setlocale(category, NULL)` semantics, with reentrant and non-reentrant entry points.

The Rust rewrite must preserve the module’s observable behavior:

- support querying the current locale string for a specified category
- provide a form that writes the locale name into caller-provided storage
- provide a form that returns a pointer to module-managed storage for convenience
- distinguish success from failure through return values consistent with the C module’s contract
- preserve behavior across the locking variants present in the source, without introducing new public capabilities

This specification is derived from:

- `setlocale_null-unlocked.c`
- `setlocale_null.c`

and the functions:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

## Scope

In scope:

- querying the current locale name for a locale category
- copying the locale name into a caller buffer
- returning a stable result pointer for the immediate call from a convenience API
- handling invalid or failing locale queries through error returns

Out of scope:

- changing locale settings
- defining locale categories
- exposing new APIs beyond the module’s evidenced functionality
- adding thread-safety guarantees beyond those implied by the existing locked/unlocked function split

## Feature Specification

### Feature: Query current locale string for a category

The module must provide functionality equivalent to calling `setlocale(category, NULL)` for a specified locale category and exposing the resulting locale name to callers.

Behavior required by the Rust version:

- Accept a locale category value.
- Query the current locale string for that category.
- On success, provide the locale name exactly as a NUL-terminated string value suitable for C-compatible use.
- On failure, report failure through the corresponding function’s return contract.

This feature is evidenced by:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

### Feature: Reentrant copy-into-buffer API

The module must provide a reentrant interface that writes the queried locale name into caller-provided storage.

Behavior required by the Rust version:

- Accept a destination buffer and its size.
- If locale query succeeds and the locale string fits, copy the full locale string including terminating NUL into the provided buffer.
- Return success only when the buffer receives a complete valid string.
- If the category is invalid, the locale query fails, or the buffer is insufficient, return failure according to the module contract.

This feature is evidenced by:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### Feature: Convenience API returning module-managed storage

The module must provide a convenience interface that returns a pointer to a locale string without requiring the caller to supply a buffer.

Behavior required by the Rust version:

- Query the locale string for the given category.
- Return a pointer to a NUL-terminated string representing the current locale when successful.
- Return null on failure.
- Preserve the module behavior that this API is convenience-oriented and internally backed rather than caller-buffer-backed.

This feature is evidenced by:

- `setlocale_null_unlocked`
- `setlocale_null`

### Feature: Locked wrapper behavior

The module contains locked wrappers around the unlocked locale-query logic. The Rust rewrite must preserve the externally visible behavior of the public entry points that correspond to the locked path.

Behavior required by the Rust version:

- `setlocale_null_r` must provide the same functional result contract as the unlocked reentrant form.
- `setlocale_null` must provide the same functional result contract as the unlocked convenience form.
- The rewrite must not weaken correctness for callers using the public functions that rely on the locked path.

This feature is evidenced by:

- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`

## User Scenarios & Testing

### Scenario 1: Caller needs the current locale for a category in caller-owned storage

A caller has a locale category and a writable buffer. The caller requests the current locale name and expects the module to copy it into the buffer.

Expected result:

- success return when the category is valid, locale query succeeds, and the buffer is large enough
- buffer contains the full locale string with terminating NUL
- failure return when the query cannot be completed or the buffer is too small

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

Suggested tests:

- valid category with sufficiently large buffer
- valid category with exact-fit buffer
- valid category with undersized buffer
- invalid category
- null or unusable locale query result from the underlying system

### Scenario 2: Caller wants a simple pointer result for immediate use

A caller wants the current locale string and uses the convenience API instead of managing a buffer.

Expected result:

- non-null pointer on success
- returned string is NUL-terminated
- null on failure

Traceability:

- `setlocale_null_unlocked`
- `setlocale_null`

Suggested tests:

- successful query for a valid category
- failure for an invalid category
- repeated successful calls returning valid strings

### Scenario 3: Caller uses public API rather than unlocked helper

A caller uses the public functions exposed by the main module path and expects the same locale-query outcome as the unlocked helper path.

Expected result:

- for the same category and same underlying locale state, public APIs and unlocked APIs produce equivalent success/failure outcomes
- on success, returned/copied locale content matches

Traceability:

- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`
- `setlocale_null_r_unlocked`
- `setlocale_null_unlocked`

Suggested tests:

- compare `setlocale_null_r` with `setlocale_null_r_unlocked`
- compare `setlocale_null` with `setlocale_null_unlocked`
- verify matching behavior on both success and failure cases

### Scenario 4: Caller checks failure handling

A caller passes inputs that cannot produce a valid locale string and relies on the module to signal failure without returning a partial successful result.

Expected result:

- failure is indicated by the function’s documented return mode
- no successful pointer is returned on failure
- no success status is reported when the destination buffer cannot hold the result

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_unlocked`
- `setlocale_null`

Suggested tests:

- invalid category
- zero-sized buffer
- insufficient buffer for a non-empty locale string

## Requirements

### Functional Requirements

#### FR-1: Locale category query
The module shall accept a locale category parameter and attempt to obtain the current locale name for that category using `setlocale(category, NULL)` semantics.

Traceability:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

#### FR-2: Reentrant buffer output
The module shall provide a reentrant function that writes the locale name into caller-provided storage and reports success or failure as an integer status.

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-3: Complete-string copy contract
When the reentrant function succeeds, it shall copy the complete locale string including the terminating NUL into the supplied buffer.

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-4: Buffer-size failure
The reentrant function shall fail when the provided buffer is not large enough to hold the complete locale string and terminating NUL.

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-5: Convenience pointer result
The module shall provide a convenience function that returns a pointer to a NUL-terminated locale string on success and a null pointer on failure.

Traceability:

- `setlocale_null_unlocked`
- `setlocale_null`

#### FR-6: Public/wrapper behavioral equivalence
The public functions shall preserve the same observable success/failure and string-content behavior as their unlocked counterparts for the same inputs and locale state.

Traceability:

- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`
- `setlocale_null_r_unlocked`
- `setlocale_null_unlocked`

#### FR-7: Failure propagation
If the underlying locale query does not produce a valid locale string, the module shall report failure and shall not report a successful result.

Traceability:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

### Key Entities

#### Entity: Locale category
An integer-valued selector identifying which locale category is being queried.

Relationships:

- consumed by all module entry points
- determines which current locale string is requested

Traceability:

- parameter `category` in all listed functions

#### Entity: Destination buffer
Caller-provided writable character storage paired with a size value for receiving a copied locale string.

Relationships:

- used only by reentrant APIs
- must be large enough for the full locale string and terminating NUL

Traceability:

- parameters `buf` and `bufsize` in `setlocale_null_r_unlocked`, `setlocale_null_r_with_lock`, `setlocale_null_r`

#### Entity: Locale string result
A NUL-terminated string representing the current locale for the requested category.

Relationships:

- produced by the locale query
- copied into destination buffer by reentrant APIs
- returned by pointer from convenience APIs

Traceability:

- return values of `setlocale_null_unlocked` and `setlocale_null`
- copied output of `setlocale_null_r_unlocked` and `setlocale_null_r`

#### Entity: Status result
An integer success/failure indicator for reentrant calls.

Relationships:

- returned by reentrant APIs
- determines whether buffer content is a successful locale-string result

Traceability:

- return values of `setlocale_null_r_unlocked`, `setlocale_null_r_with_lock`, `setlocale_null_r`

## Success Criteria

### SC-1: Successful locale retrieval through reentrant API
For a valid category with an available locale string and sufficient buffer space, the Rust module returns success from the reentrant API and writes a NUL-terminated locale string equal to the underlying current locale value.

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### SC-2: Buffer-too-small detection
For a valid category where the destination buffer is too small for the full locale string plus terminating NUL, the Rust module returns failure from the reentrant API.

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### SC-3: Successful locale retrieval through convenience API
For a valid category with an available locale string, the Rust module returns a non-null pointer from the convenience API to a NUL-terminated string equal to the underlying current locale value.

Traceability:

- `setlocale_null_unlocked`
- `setlocale_null`

### SC-4: Failure result on invalid or failed locale query
For an invalid category or an underlying locale query failure, the Rust module returns failure from reentrant APIs and null from pointer-returning APIs.

Traceability:

- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_unlocked`
- `setlocale_null`

### SC-5: Public and unlocked path equivalence
For the same category and locale state, public APIs and unlocked APIs produce equivalent success/failure outcomes and identical locale-string content on success.

Traceability:

- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`
- `setlocale_null_r_unlocked`
- `setlocale_null_unlocked`

## Non-Goals

The Rust rewrite is not required by this specification to:

- change locale state
- define new locale-management abstractions
- expose additional public interfaces
- guarantee behavior beyond the source module’s evidenced contracts