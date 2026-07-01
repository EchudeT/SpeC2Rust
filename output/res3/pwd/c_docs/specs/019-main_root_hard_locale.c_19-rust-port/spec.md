# spec.md

## Title

Rust Functional Specification for `main_root_hard-locale.c_19`

## Metadata

- Project: `pwd`
- Module: `main_root_hard-locale.c_19`
- Category: `main_cluster`
- Source file: `hard-locale.c`
- Primary function: `hard_locale(int category) -> bool`
- Rust branch: `019-main_root_hard_locale.c_19-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides a single locale-classification capability: determining whether the current locale for a requested locale category should be treated as a "hard" locale rather than a basic default locale.

For the Rust rewrite, the module must preserve this behavioral role: given a locale category identifier, it must evaluate the active locale name for that category and return a boolean indicating whether the locale is non-default in the sense recognized by this module.

The scope of this specification is limited to the behavior evidenced by the source module and its exported function. No additional public capabilities are in scope.

## Feature Specification

### Feature: Hard-locale detection

The module shall provide functionality that answers whether the current locale for a specified locale category is a hard locale.

A hard locale, for purposes of this module, is any locale setting that is not one of the basic default locale forms recognized by the original behavior. The decision is based on the locale name returned for the requested category.

The Rust version must implement the same observable behavior:

- Accept a locale category selector as input.
- Inspect the current locale associated with that category.
- Return `false` when the locale corresponds to the default simple locale forms recognized by the module.
- Return `true` otherwise.

This functionality is classification-only. The module does not change locale state, does not own locale configuration, and does not define locale categories itself.

### Behavioral boundary

The Rust rewrite must stay within these boundaries:

- It must classify the current locale for the requested category.
- It must return a boolean result only.
- It must not introduce new policy beyond the source behavior.
- It must not require callers to provide locale strings directly if the source module determines the locale from process locale state.

## User Scenarios & Testing

### Scenario 1: Default POSIX-style locale

A caller checks whether a category such as character classification or message formatting is using a basic default locale.

Expected behavior:

- If the active locale name for the category is one of the default locale forms recognized by the module, the function returns `false`.

Testing guidance:

- Set the process locale category under test to a default locale form recognized by the source behavior.
- Call the Rust function with that category.
- Verify that the result is `false`.

### Scenario 2: Explicit non-default locale

A caller checks whether locale-sensitive behavior should be treated as non-default because the environment is configured for a specific language or regional locale.

Expected behavior:

- If the active locale name for the category is not one of the default locale forms recognized by the module, the function returns `true`.

Testing guidance:

- Set the process locale category under test to a named non-default locale.
- Call the Rust function with that category.
- Verify that the result is `true`.

### Scenario 3: Category-specific evaluation

A caller evaluates more than one locale category and expects the result to reflect the specific category requested, not a global answer detached from the category parameter.

Expected behavior:

- The result reflects the locale currently active for the provided category.

Testing guidance:

- Configure different categories to different locale values where supported.
- Call the Rust function separately for each category.
- Verify that each result matches the locale of the requested category.

### Scenario 4: Integration use by higher-level program logic

A higher-level command uses this module to decide whether locale-aware behavior should follow a default path or a locale-specific path.

Expected behavior:

- The function provides a stable boolean classification suitable for branching logic.
- The module itself performs no output and no side effects visible to the caller other than the return value.

Testing guidance:

- Invoke the function from integration code that branches on the boolean result.
- Confirm that only the return value influences downstream logic from this module.

## Requirements

### Functional Requirements

#### FR-1: Locale category input
The module shall accept a locale category identifier and evaluate the current locale for that category.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-2: Boolean classification result
The module shall return a boolean result indicating whether the locale for the requested category is hard.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-3: Default-locale recognition
The module shall recognize the basic default locale forms treated by the original module as non-hard and return `false` for those forms.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-4: Non-default locale recognition
The module shall return `true` when the locale for the requested category is not one of the basic default locale forms treated as non-hard.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-5: No locale mutation
The module shall not modify locale settings; it shall only inspect and classify the current locale state for the requested category.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-6: Category-dependent behavior
The module shall base its result on the category argument supplied by the caller rather than on unrelated categories.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

### Key Entities

#### Entity: Locale category
An integer-valued selector identifying which locale category is being queried.

Relationship:
- Supplied by the caller to the module's classification function.

Traceability:
- `hard_locale(int category)`

#### Entity: Current locale name
The locale identifier currently associated with the requested category in process locale state.

Relationship:
- Read by the module for the specified locale category.
- Interpreted to determine whether the locale is default or non-default.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### Entity: Hard-locale classification
A boolean outcome representing whether the requested category's current locale is considered hard.

Relationship:
- Produced from evaluating the current locale name for the requested category.

Traceability:
- `hard_locale(int category)`

## Success Criteria

### SC-1: Correct default-locale result
For every locale category value supported by the surrounding platform interface and exercised in tests, when that category is set to a default locale form recognized by the source behavior, the Rust implementation returns `false`.

Traceability:
- FR-1
- FR-3

### SC-2: Correct non-default-locale result
For every tested locale category, when that category is set to a non-default locale name, the Rust implementation returns `true`.

Traceability:
- FR-1
- FR-4

### SC-3: Category-sensitive correctness
When two locale categories are configured differently in the same process, querying each category returns the classification corresponding to that category's own current locale.

Traceability:
- FR-1
- FR-6

### SC-4: No observable side effects
Calling the Rust implementation does not alter process locale configuration and produces no module-owned output; its only observable module result is the returned boolean value.

Traceability:
- FR-2
- FR-5

### SC-5: API-equivalent functional role
The Rust rewrite preserves the source module's single functional role as a hard-locale detector for a supplied locale category, without requiring additional caller inputs beyond the category and without adding unrelated public behavior.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

## Out of Scope

The following are not required by this module specification because they are not evidenced by the source module input:

- Defining or managing locale categories
- Providing locale-setting APIs
- Returning locale names or metadata beyond the boolean classification
- Thread-safety guarantees beyond those implied by the surrounding runtime
- Serialization, persistence, caching, recovery, or benchmarking features
- Additional public interfaces beyond the functional equivalent of the source behavior