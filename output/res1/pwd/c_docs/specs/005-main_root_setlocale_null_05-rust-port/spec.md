# spec.md

## Title

Rust Functional Specification for `main_root_setlocale_null_05`

## Summary

This module provides read-only access to the current locale name for a requested locale category, with both direct-return and caller-buffer forms. Its purpose is to obtain the locale string corresponding to `setlocale(category, NULL)` semantics while handling environments where direct access may require synchronization or copying into caller-provided storage.

The Rust rewrite must preserve the observable behavior of the C module functions represented by:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

## Scope

In scope:

- Querying the current locale name for a specified locale category.
- Returning that locale either as a direct string result or by copying it into a caller-supplied buffer.
- Supporting both unlocked and wrapped/locking-aware variants as represented by the module.
- Reporting failure when the locale cannot be obtained or cannot fit in the provided buffer.

Out of scope:

- Changing locale settings.
- Defining new locale categories.
- Providing locale parsing, normalization, or translation.
- Adding new public APIs beyond the behaviors evidenced by the source module.

## Feature Specification

### Feature: Read current locale name for a category

The module shall provide functionality equivalent to querying the current locale name for a given locale category without modifying process locale state.

Two usage forms are required:

1. A form that returns a locale string pointer/reference result for the category.
2. A form that writes the locale string into a caller-provided buffer and reports success or failure as an integer status.

### Feature: Buffer-based locale retrieval

The buffer-based behavior shall copy the full locale name for the requested category into caller storage when possible.

The Rust version must preserve these functional boundaries:

- Accept a locale category identifier.
- Accept a mutable output buffer and its capacity for the reentrant/buffered form.
- Succeed only when a complete locale string can be represented in the output buffer.
- Fail when the category is invalid, locale retrieval fails, or the provided storage is insufficient.

### Feature: Wrapped retrieval matching top-level API behavior

The module includes wrapped forms that mediate access to the unlocked retrieval behavior. The Rust rewrite must preserve the externally visible distinction that:

- `setlocale_null_r` is the public buffered retrieval entry point.
- `setlocale_null` is the public direct-return retrieval entry point.

These entry points must behave consistently with the underlying locale query semantics of the C module.

## User Scenarios & Testing

### Scenario 1: Caller needs the current locale string for one category

A caller requests the locale name for a valid locale category using the direct-return API.

Expected behavior:

- The module returns a string representing the current locale for that category when available.
- The operation does not change locale configuration.

Test coverage:

- Invoke the direct-return entry point with a valid category.
- Verify that a non-failure result is produced when the environment provides a locale string.
- Verify that repeated calls remain observationally read-only.

### Scenario 2: Caller needs locale data copied into owned storage

A caller provides a buffer and capacity, then requests the locale name for a valid locale category using the buffered API.

Expected behavior:

- On success, the buffer contains the complete locale string.
- The function returns a success status.
- The copied result corresponds to the same category-specific locale value that the direct query would expose.

Test coverage:

- Provide a sufficiently large buffer.
- Verify success status.
- Verify the buffer content matches the locale name for the selected category.

### Scenario 3: Caller provides insufficient buffer space

A caller requests buffered locale retrieval but the provided buffer is too small to hold the full locale string result.

Expected behavior:

- The function reports failure.
- No partial-success outcome is reported as success.

Test coverage:

- Provide a buffer smaller than the locale string length plus terminator needs implied by C-string behavior.
- Verify failure status.

### Scenario 4: Caller uses an invalid or unsupported locale category

A caller passes a category value for which locale lookup does not produce a valid result.

Expected behavior:

- The direct-return form reports failure through its null/failure result convention.
- The buffered form reports failure through its integer status convention.

Test coverage:

- Invoke both public APIs with an invalid category.
- Verify failure is reported.

### Scenario 5: Caller uses public buffered wrapper path

A caller uses the public buffered API rather than the unlocked helper.

Expected behavior:

- Observable success/failure and copied output match module-defined locale retrieval behavior.
- The wrapper does not alter the returned locale content.

Test coverage:

- Compare buffered public entry-point behavior against underlying retrieval expectations for the same category and buffer conditions.

## Requirements

### Functional Requirements

#### FR-1: Category-based locale query
The module shall accept a locale category identifier and attempt to obtain the current locale name for that category.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

#### FR-2: Direct-return locale access
The module shall provide a direct-return operation that yields the current locale string for a category on success and a failure result on error.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null`

#### FR-3: Buffered locale access
The module shall provide a buffered operation that writes the current locale string for a category into caller-provided storage and returns an integer success/failure status.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

#### FR-4: Full-string copy semantics
The buffered operation shall succeed only if the complete locale string can be stored in the supplied buffer.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

#### FR-5: Failure reporting on unavailable locale result
If locale retrieval for the requested category does not produce a valid string result, both API forms shall report failure using their respective conventions.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

#### FR-6: Public entry-point preservation
The Rust rewrite shall preserve the two public functional entry points represented by the module: direct-return retrieval and buffered retrieval.

**Traceability:** `setlocale_null_r`, `setlocale_null`

#### FR-7: Read-only locale observation
The module shall observe the current locale state for a category without changing locale configuration.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null`

### Key Entities

#### Locale category
An integer selector identifying which locale category is being queried.

**Relationship:** Consumed by all module entry points to determine which locale string to retrieve.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

#### Locale string result
A null-terminated locale name in C behavior terms, exposed either as a direct returned string or copied into caller-owned storage.

**Relationship:** Produced from the locale category query; consumed by callers directly or through copied buffer contents.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null`

#### Caller-provided output buffer
Mutable storage supplied by the caller for reentrant/buffered retrieval.

**Relationship:** Used by buffered retrieval functions to receive the locale string if capacity is sufficient.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

#### Buffer size
The capacity associated with the caller-provided output buffer.

**Relationship:** Determines whether buffered retrieval can complete successfully.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

## Success Criteria

### SC-1: Correct success behavior for valid category
For a valid locale category with an available locale string, the Rust implementation returns success from the buffered API and a non-failure string result from the direct API.

**Traceability:** `setlocale_null_r`, `setlocale_null`

### SC-2: Correct copied output
When buffered retrieval succeeds, the output buffer contains the complete locale string corresponding to the requested category.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

### SC-3: Correct failure on insufficient buffer
When the provided output buffer cannot hold the complete locale string, the buffered API reports failure.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

### SC-4: Correct failure on invalid retrieval
When locale lookup fails for a requested category, the direct API returns its failure result and the buffered API returns failure status.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

### SC-5: Public API behavioral consistency
For the same valid category under the same process locale state, the direct-return and buffered public APIs represent the same locale value, subject to buffer-capacity constraints in the buffered form.

**Traceability:** `setlocale_null_r`, `setlocale_null`

### SC-6: No locale mutation as part of query
Tests exercising module APIs do not require or observe locale-setting side effects caused by this module.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null`