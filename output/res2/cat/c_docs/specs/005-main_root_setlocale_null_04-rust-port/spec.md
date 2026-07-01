# spec.md

## Title
Functional Specification for `main_root_setlocale_null_04`

## Summary
This module provides read-only access to the process locale name for a requested locale category, specifically the behavior equivalent to querying `setlocale(category, NULL)`, plus a buffer-based variant that writes the locale name into caller-provided storage.

The Rust rewrite must preserve the module’s externally observable behavior:
- return the current locale name for a category without changing locale state,
- support a reentrant/buffer-writing form,
- handle invalid categories and insufficient buffer capacity through failure signaling,
- provide a stable result contract equivalent to the C module’s public entry points.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`.

---

## Scope
Included in scope:
- Querying the current locale string for a specified locale category.
- Copying the locale string into a caller-supplied buffer with explicit size handling.
- Public behavior for success and failure cases of the exported entry points.

Out of scope:
- Changing locale settings.
- Defining new locale categories.
- Providing capabilities beyond the query behavior evidenced by this module.

---

## Feature Specification

### Feature: Query current locale string by category
The module shall provide functionality to obtain the current locale name associated with a requested locale category, without modifying locale configuration.

This feature exists in two behavioral forms:
1. a form returning a locale string pointer/result,
2. a form writing the locale string into a caller-provided buffer and reporting success or failure.

The Rust version must implement both forms of behavior represented by the module’s public functions.

**Traceability:**
- Pointer/result form: `setlocale_null_unlocked`, `setlocale_null`
- Buffer/result-code form: `setlocale_null_r_unlocked`, `setlocale_null_r`

### Feature: Buffer-based locale query
The module shall support writing the locale name for a category into caller-provided storage when enough space is available.

The Rust version must:
- accept a locale category and writable destination storage,
- copy the complete locale name on success,
- fail when the category is invalid or destination capacity is not sufficient,
- avoid partial-success semantics being treated as success.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

### Feature: Locked/public query wrapper behavior
The module shall expose public behavior that wraps the underlying locale query in the module-defined “with lock”/public path, preserving the same query semantics for callers of the public API.

The Rust rewrite must preserve the public observable contract of:
- `setlocale_null_r`
- `setlocale_null`

without requiring callers to use lower-level unlocked variants.

**Traceability:** `setlocale_null_r_with_lock`, `setlocale_null_r`, `setlocale_null`

---

## User Scenarios & Testing

### Scenario 1: Caller needs the current locale name for a valid category
A caller asks for the current locale name of a valid locale category and receives a successful result representing that locale.

Expected support in Rust:
- the query succeeds for valid categories recognized by the platform/module,
- the returned locale string/result corresponds to the current locale for that category,
- the query does not change locale settings.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null`

### Scenario 2: Caller needs a copy of the locale name in its own buffer
A caller provides a destination buffer and requests the locale name for a valid category.

Expected support in Rust:
- success is reported when the buffer is large enough,
- the destination contains the full locale string on success.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

### Scenario 3: Caller provides too small a buffer
A caller requests the locale name into a destination buffer that cannot hold the entire result.

Expected support in Rust:
- the operation reports failure,
- success is not reported for truncated output.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

### Scenario 4: Caller passes an invalid locale category
A caller provides a category value not accepted by the underlying locale query.

Expected support in Rust:
- the operation reports failure or returns no result, matching the corresponding API form,
- no locale change occurs.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

### Scenario 5: Caller uses the public wrapper rather than an unlocked form
A caller uses the module’s public entry point to query locale state.

Expected support in Rust:
- the public API produces the same logical locale-query result as the lower-level behavior for the same category,
- wrapper use is sufficient; callers do not need an internal helper path.

**Traceability:** `setlocale_null_r_with_lock`, `setlocale_null_r`, `setlocale_null`

### Testing Guidance
The Rust rewrite must be testable for:
- valid category success cases,
- invalid category failure cases,
- exact-fit buffer cases,
- too-small buffer failure cases,
- consistency between buffer-based and pointer/result-based query forms for the same category.

---

## Requirements

### Functional Requirements

#### FR-1: Locale query without locale modification
The module shall provide a query operation that obtains the current locale name for a specified category without setting or changing the locale.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null`

#### FR-2: Public locale query result form
The module shall provide a public result-returning operation for locale query by category. On success it returns the locale name; on failure it returns no successful result.

**Traceability:** `setlocale_null`

#### FR-3: Reentrant/buffer-writing locale query
The module shall provide a buffer-writing query operation that writes the locale name for a category into caller-provided storage and reports success or failure with an integer-style status.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

#### FR-4: Invalid category handling
For both query forms, the module shall detect or propagate failure for unsupported or invalid locale categories.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

#### FR-5: Buffer capacity enforcement
The buffer-writing query shall succeed only when the provided storage capacity is sufficient for the complete locale string required by the module’s contract.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

#### FR-6: Public wrapper semantics
The public APIs shall preserve the query semantics of the module’s lower-level locale retrieval behavior.

**Traceability:** `setlocale_null_r_with_lock`, `setlocale_null_r`, `setlocale_null`

### Key Entities

#### Entity: Locale category
An integer category selector identifying which locale class is being queried.

Relationship:
- used as input by all module entry points,
- determines which current locale string is returned or copied.

**Traceability:** all listed functions

#### Entity: Locale name result
A string result representing the current locale associated with the requested category.

Relationship:
- returned directly by the result-returning query form,
- copied into caller-provided storage by the buffer-based form.

**Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

#### Entity: Caller-provided buffer
Writable storage supplied by the caller for the buffer-based query.

Relationship:
- receives the locale name on success,
- must be large enough for the operation to succeed.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

#### Entity: Status result
An integer-style success/failure result for buffer-based query operations.

Relationship:
- indicates whether locale retrieval and copying succeeded.

**Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

---

## Success Criteria

1. The Rust module exposes behavior equivalent to the C module’s public query operations for:
   - returning the current locale name by category,
   - writing the current locale name into caller-provided storage.

   **Traceability:** `setlocale_null`, `setlocale_null_r`

2. For a valid locale category, the Rust implementation returns a successful locale query result and the buffer-based form reproduces the same locale string content as the result-returning form.

   **Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

3. For an invalid locale category, the Rust implementation reports failure consistently with the corresponding API form.

   **Traceability:** `setlocale_null_unlocked`, `setlocale_null_r_unlocked`, `setlocale_null_r`, `setlocale_null`

4. For the buffer-based query, an exact-size-or-larger destination succeeds and a too-small destination fails.

   **Traceability:** `setlocale_null_r_unlocked`, `setlocale_null_r`

5. The Rust implementation does not require callers of the public API to use internal helper paths to obtain correct locale-query behavior.

   **Traceability:** `setlocale_null_r_with_lock`, `setlocale_null_r`, `setlocale_null`

6. Tests demonstrate all required scenarios in this specification against the Rust port on branch `005-main_root_setlocale_null_04-rust-port`.

   **Traceability:** all public module behaviors above