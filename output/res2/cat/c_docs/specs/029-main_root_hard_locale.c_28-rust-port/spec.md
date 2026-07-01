# spec.md

## Overview

- **Project**: `cat`
- **Module**: `main_root_hard-locale.c_28`
- **Category**: `main_cluster`
- **Source file**: `hard-locale.c`
- **Primary function**: `hard_locale(int category) -> bool`
- **Rust target branch**: `029-main_root_hard_locale.c_28-rust-port`
- **Generation date**: 2026-06-07

## Feature Specification

This module provides a single locale-querying capability: determine whether the current process locale for a specified locale category should be treated as a "hard" locale rather than the default simple locale.

The Rust rewrite must implement equivalent behavior for the module’s exposed functionality:

- Accept a locale category identifier as input.
- Inspect the current locale setting for that category.
- Return `false` when the active locale is the default non-hard locale represented by the standard `"C"` locale.
- Return `false` when the active locale is the equivalent default POSIX locale represented by `"POSIX"`.
- Return `true` for other locale names, indicating that locale-sensitive behavior should be treated as active.

This module is a pure query helper. Its role is to support higher-level program logic that needs to distinguish between default byte-oriented behavior and locale-dependent behavior.

## User Scenarios & Testing

### Scenario 1: Default C locale
A caller checks whether locale-sensitive behavior is needed for a locale category while the process is running under the `"C"` locale.

**Expected result**:
- `hard_locale(category)` returns `false`.

**Test guidance**:
- Set the relevant locale category to `"C"`.
- Call the function with that category.
- Verify the result is `false`.

### Scenario 2: POSIX locale
A caller checks the locale category while the process locale is `"POSIX"`.

**Expected result**:
- `hard_locale(category)` returns `false`.

**Test guidance**:
- Set the relevant locale category to `"POSIX"`.
- Call the function with that category.
- Verify the result is `false`.

### Scenario 3: Named non-default locale
A caller checks whether locale handling should be considered active for a category set to a named locale other than `"C"` or `"POSIX"`.

**Expected result**:
- `hard_locale(category)` returns `true`.

**Test guidance**:
- Set the relevant locale category to an available locale such as `en_US.UTF-8` or any other installed non-default locale.
- Call the function with that category.
- Verify the result is `true`.

### Scenario 4: Category-specific query
A caller uses the helper with a specific locale category rather than assuming a global locale state.

**Expected result**:
- The return value reflects the current locale assigned to the provided category.

**Test guidance**:
- Configure different locale categories independently where supported.
- Query at least one category explicitly.
- Verify the result matches that category’s current locale name.

## Requirements

### Functional Requirements

#### FR-1: Locale category input
The module shall accept a locale category identifier and evaluate the current locale associated with that category.

**Traceability**: `hard-locale.c`, `hard_locale(int category)`

#### FR-2: Default C locale classification
The module shall classify the locale name `"C"` as not hard and return `false`.

**Traceability**: `hard-locale.c`, `hard_locale(int category)`

#### FR-3: Default POSIX locale classification
The module shall classify the locale name `"POSIX"` as not hard and return `false`.

**Traceability**: `hard-locale.c`, `hard_locale(int category)`

#### FR-4: Non-default locale classification
The module shall return `true` when the queried locale category resolves to a locale name other than `"C"` and `"POSIX"`.

**Traceability**: `hard-locale.c`, `hard_locale(int category)`

#### FR-5: Boolean query behavior
The module shall expose this determination as a boolean result only.

**Traceability**: `hard-locale.c`, `hard_locale(int category)`

### Key Entities

#### Entity: Locale category
An integer locale category identifier supplied by the caller to specify which current locale setting should be queried.

**Relationship**:
- Consumed by `hard_locale` as the sole input that selects the locale category to inspect.

#### Entity: Locale name
The current locale designation associated with the supplied category.

**Relationship**:
- Evaluated by `hard_locale` against the distinguished default values `"C"` and `"POSIX"` to determine the boolean result.

#### Entity: Hard-locale result
A boolean classification indicating whether the locale category should be treated as locale-sensitive (`true`) or as default simple locale behavior (`false`).

**Relationship**:
- Produced by `hard_locale` from the locale name of the selected locale category.

## Success Criteria

1. The Rust module provides a functionally equivalent boolean query for a supplied locale category.
   - **Traceability**: `hard_locale(int category)`

2. When the queried category is set to `"C"`, the Rust version returns `false`.
   - **Traceability**: `hard-locale.c`, `hard_locale(int category)`

3. When the queried category is set to `"POSIX"`, the Rust version returns `false`.

4. When the queried category is set to a locale name other than `"C"` or `"POSIX"`, the Rust version returns `true`.

5. Tests cover category-based invocation and verify the above classifications using locale configurations available in the test environment.