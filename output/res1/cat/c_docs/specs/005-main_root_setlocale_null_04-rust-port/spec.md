# spec.md

## Title

Functional Specification: `main_root_setlocale_null_04`

## Metadata

- **Project**: `cat`
- **Module**: `main_root_setlocale_null_04`
- **Category**: `main_cluster`
- **Target Rust Branch**: `005-main_root_setlocale_null_04-rust-port`
- **Source Basis**:
  - `setlocale_null-unlocked.c`
  - `setlocale_null.c`
- **Generation Date**: 2026-06-06

## Overview

This module provides read-only access to the process locale name for a requested locale category by querying the current locale with a null locale argument behavior equivalent to `setlocale(category, NULL)`.

The module exposes:
- an unlocked path for obtaining the current locale name directly,
- a reentrant path that copies the locale name into caller-provided storage and reports success or failure,
- a public wrapper that returns a stable string result for the requested category.

The Rust rewrite must preserve the module’s externally visible behavior: obtain the current locale name for a category, support copying that result into a caller buffer, and distinguish successful retrieval from error cases such as invalid category or insufficient output storage.

## Feature Specification

### Feature: Query current locale name for a category

The module shall provide functionality equivalent to requesting the current locale name for a locale category without changing locale state.

This includes:
- retrieving the locale name as a string result,
- supporting a category parameter supplied by the caller,
- treating the operation as a query of existing process locale state rather than a locale update.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null`

### Feature: Reentrant retrieval into caller-provided buffer

The module shall support a reentrant retrieval mode where the caller supplies:
- a locale category,
- a destination buffer,
- the destination buffer size.

The operation shall:
- copy the current locale name into the provided buffer when possible,
- include string termination in the copied result,
- report failure when the locale name cannot be retrieved or does not fit.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock`
- `setlocale_null_r`

### Feature: Locked/public retrieval path

The module shall provide a public retrieval path that uses the module’s lock-aware wrapper behavior when obtaining locale data through the reentrant interface and when returning a pointer-style string result.

The Rust rewrite must preserve the behavioral distinction that:
- internal unlocked retrieval exists,
- public entry points use the wrapper path intended for safe public querying behavior.

No stronger concurrency guarantees than this wrapper behavior are specified.

**Traceability**:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`

### Feature: Error-sensitive return behavior

The module shall preserve observable success/failure behavior for both interface styles:

- For buffer-copying calls:
  - success is reported when a valid locale string is obtained and copied fully,
  - failure is reported for invalid query or insufficient buffer capacity.

- For string-returning calls:
  - a locale string is returned on success,
  - failure is represented by the function’s failure return behavior rather than by partial output.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_unlocked`
- `setlocale_null`

## User Scenarios & Testing

### Scenario 1: Read current locale name for a valid category

A caller needs the current locale name for a locale category already configured in the process.

Expected behavior:
- the query succeeds,
- the returned locale name reflects the current setting for that category,
- no locale change is performed by the module.

**Relevant functions**:
- `setlocale_null`
- `setlocale_null_unlocked`

### Scenario 2: Copy locale name into caller-owned storage

A caller needs the locale name in its own writable storage.

Expected behavior:
- the caller provides category, buffer, and buffer size,
- the module copies the locale name including termination when it fits,
- the call reports success.

**Relevant functions**:
- `setlocale_null_r`
- `setlocale_null_r_unlocked`

### Scenario 3: Buffer too small for locale name

A caller provides a destination buffer that is smaller than required for the locale name string and terminator.

Expected behavior:
- the module does not report success,
- the module does not treat truncated output as a successful result.

**Relevant functions**:
- `setlocale_null_r`
- `setlocale_null_r_unlocked`

### Scenario 4: Invalid or unsupported locale category

A caller passes a category that does not yield a valid locale string.

Expected behavior:
- the module reports failure through the respective API style,
- no fabricated locale name is returned.

**Relevant functions**:
- `setlocale_null`
- `setlocale_null_r`
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`

### Scenario 5: Public query path delegates through lock-aware wrapper behavior

A caller uses the public reentrant or string-returning entry point rather than the unlocked helper.

Expected behavior:
- the public call follows the wrapper behavior represented by the lock-aware path,
- the public result is behaviorally consistent with unlocked retrieval for the same locale category when retrieval succeeds.

**Relevant functions**:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`

### Testing expectations

The Rust version must be testable for at least the following:
- valid category returns a non-failure result,
- reentrant copy succeeds when buffer size is sufficient,
- reentrant copy fails when buffer size is insufficient,
- invalid category produces failure behavior,
- public and unlocked retrieval paths agree on successful locale content for the same category.

## Requirements

### Functional Requirements

#### FR-1: Locale query without mutation
The module shall query the current locale name for a provided locale category without changing locale configuration.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null`

#### FR-2: Unlocked direct string retrieval
The module shall provide an internal/direct retrieval mode that returns the current locale name as a string result for a category on success.

**Traceability**:
- `setlocale_null_unlocked`

#### FR-3: Reentrant buffer-based retrieval
The module shall provide a retrieval mode that writes the locale name for a category into caller-provided storage and reports status as an integer success/failure result.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-4: Null-terminated copied result
When buffer-based retrieval succeeds, the copied locale string shall be complete and null-terminated within the provided buffer.

**Traceability**:
- `setlocale_null_r_unlocked`

#### FR-5: Insufficient-capacity detection
Buffer-based retrieval shall fail if the provided buffer size is not large enough to hold the full locale string and terminating null byte.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

#### FR-6: Invalid-category failure propagation
If the requested category does not produce a valid current locale string, the module shall report failure through the corresponding API result form.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

#### FR-7: Public reentrant wrapper behavior
The public reentrant entry point shall obtain locale data through the module’s wrapper path corresponding to `setlocale_null_r_with_lock`.

**Traceability**:
- `setlocale_null_r_with_lock`
- `setlocale_null_r`

#### FR-8: Public string-returning wrapper behavior
The public string-returning entry point shall provide a stable successful result for the requested category using the module’s public wrapper behavior rather than exposing only the unlocked helper.

**Traceability**:
- `setlocale_null`
- `setlocale_null_r_with_lock`

### Key Entities

#### Entity: Locale category
An input value identifying which locale category is being queried.

Relationship:
- supplied by the caller to all module entry points,
- determines which current locale name is returned or copied.

**Traceability**:
- all listed functions with `int category`

#### Entity: Locale name string
The current locale identifier associated with the requested category.

Relationship:
- produced by direct retrieval,
- copied into caller storage by reentrant retrieval,
- returned on success or absent on failure.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null`

#### Entity: Caller-provided output buffer
Writable storage supplied by the caller for reentrant retrieval.

Relationship:
- receives the locale name on success,
- must be large enough for full content plus terminator,
- paired with an explicit size parameter.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_r_with_lock`

#### Entity: Status result
An integer success/failure result for reentrant APIs and a pointer-style success/failure result for string-returning APIs.

Relationship:
- communicates whether locale retrieval and output production succeeded,
- reflects invalid category and insufficient-capacity cases.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null_unlocked`
- `setlocale_null`

## Success Criteria

### SC-1: Successful locale query
For a valid locale category configured in the process, the Rust module returns a successful locale name result through the public query interface.

**Traceability**:
- `setlocale_null`

### SC-2: Successful reentrant copy
For a valid locale category and sufficiently large output buffer, the Rust module copies the full locale name and terminator into caller-provided storage and reports success.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### SC-3: Small-buffer rejection
For a valid locale category and an output buffer smaller than required, the Rust module reports failure and does not treat the operation as a successful truncated copy.

**Traceability**:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`

### SC-4: Invalid-category rejection
For an invalid or unsupported locale category, the Rust module returns failure behavior matching the corresponding API style.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

### SC-5: Public/unlocked content consistency
When both public and unlocked retrieval paths succeed for the same locale category, they produce the same locale name content.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

### SC-6: No locale mutation during query
Tests that query locale state through this module do not require any locale change as part of successful operation; the module behaves as a read-only locale query interface.

**Traceability**:
- `setlocale_null_unlocked`
- `setlocale_null`