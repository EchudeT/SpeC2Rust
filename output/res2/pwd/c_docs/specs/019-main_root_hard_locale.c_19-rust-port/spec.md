# spec.md

## Title
Rust Functional Specification for `main_root_hard-locale.c_19`

## Document Metadata
- Project: `pwd`
- Module: `main_root_hard-locale.c_19`
- Category: `main_cluster`
- Source file: `hard-locale.c`
- Primary source function: `hard_locale(int category) -> bool`
- Target Rust branch: `019-main_root_hard_locale.c_19-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides a single decision function that determines whether the currently selected locale for a given locale category should be treated as a "hard" locale rather than a basic fallback locale.

The Rust rewrite must preserve this behavioral boundary: given a locale category identifier, it returns a boolean indicating whether the active locale for that category is meaningfully locale-specific, as opposed to the standard fallback locale values that should not be treated as hard locales.

## Feature Specification

### Feature Summary
The module exposes locale classification behavior for one locale category at a time.

Its responsibility is limited to:
- inspecting the active locale associated with the requested category, and
- classifying that locale as either:
  - not hard, when it corresponds to the standard fallback locale naming used by the C/POSIX environment, or
  - hard, for other locale selections.

### Rust Module Behavior
The Rust version must implement equivalent behavior to the source function:
- Accept a locale category input compatible with the source module's intended use.
- Determine the currently active locale name for that category.
- Return `false` when the locale is the default fallback locale represented by `"C"` or `"POSIX"`.
- Return `true` for other locale names.
- If locale lookup for the category does not yield a usable locale name, behavior must remain aligned with the source module's externally observable result for that condition.

### Functional Boundary
This module is a classifier only. It does not:
- set locale state,
- normalize locale names beyond what is required for classification,
- manage program configuration,
- expose additional public APIs beyond the documented behavior,
- store persistent module state.

## User Scenarios & Testing

### Scenario 1: Default C locale
A caller checks whether a locale category is using a locale that requires locale-sensitive behavior.

- Given the active locale name for the requested category is `"C"`
- When the caller invokes the module function
- Then the result must be `false`

### Scenario 2: POSIX fallback locale
A caller checks a locale category whose active locale name is `"POSIX"`.

- Given the active locale name for the requested category is `"POSIX"`
- When the caller invokes the module function
- Then the result must be `false`

### Scenario 3: Named non-fallback locale
A caller checks a locale category whose active locale name is a named locale other than the fallback values.

- Given the active locale name for the requested category is a non-fallback locale such as a language or region-specific locale string
- When the caller invokes the module function
- Then the result must be `true`

### Scenario 4: Category-specific query
A caller uses different locale categories and expects classification to reflect the active locale of the specific category requested.

- Given multiple locale categories may have different active locale values
- When the caller invokes the function with one category and then another
- Then each result must correspond to the active locale of the category passed for that invocation

### Testing Guidance
The Rust rewrite should be tested with category-based locale queries that verify:
- `false` for `"C"`
- `false` for `"POSIX"`
- `true` for at least one non-fallback locale name
- category-sensitive behavior when different categories resolve to different locale names

Tests must trace to the single source behavior in `hard_locale`.

## Requirements

### Functional Requirements

#### FR-1: Locale classification entry point
The module shall provide behavior equivalent to a function that accepts a locale category and returns a boolean classification result.

Traceability:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

#### FR-2: Fallback locale recognition
The module shall classify the locale as not hard when the active locale name for the requested category is `"C"`.

Traceability:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

#### FR-3: POSIX locale recognition
The module shall classify the locale as not hard when the active locale name for the requested category is `"POSIX"`.

Traceability:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

#### FR-4: Non-fallback locale recognition
The module shall classify the locale as hard when the active locale name for the requested category is not one of the fallback locale names recognized by the module.

Traceability:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

#### FR-5: Category-directed evaluation
The module shall base its classification on the locale associated with the specific category argument supplied by the caller.

Traceability:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

### Key Entities

#### Entity: Locale Category
An input value identifying which locale category is being queried.

Relationship:
- Supplied by the caller to the module's classification function.
- Determines which active locale setting is examined.

Traceability:
- `hard_locale(int category)`

#### Entity: Active Locale Name
The locale name resolved for the requested locale category at the time of the call.

Relationship:
- Derived from the locale category input.
- Compared against recognized fallback locale names to produce the boolean result.

Traceability:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

#### Entity: Hard-Locale Result
A boolean result indicating whether the resolved locale should be treated as hard.

Relationship:
- Produced by classifying the active locale name for the requested category.

Traceability:
- `hard_locale(int category) -> bool`

## Success Criteria

### SC-1: Correct fallback classification
For any supported invocation where the resolved locale name is `"C"`, the Rust version returns `false`.

Traceability:
- FR-2
- `hard_locale(int category) -> bool`

### SC-2: Correct POSIX classification
For any supported invocation where the resolved locale name is `"POSIX"`, the Rust version returns `false`.

Traceability:
- FR-3
- `hard_locale(int category) -> bool`

### SC-3: Correct non-fallback classification
For tested invocations where the resolved locale name is neither `"C"` nor `"POSIX"`, the Rust version returns `true`.

Traceability:
- FR-4
- `hard_locale(int category) -> bool`

### SC-4: Category-sensitive behavior preserved
When different locale categories resolve to different active locale names, the Rust version's result for each call matches the category passed to that call.

Traceability:
- FR-5
- `hard_locale(int category) -> bool`

### SC-5: Public behavior remains minimal and equivalent
The Rust rewrite preserves the module's functional scope as a locale hardness classifier and does not require additional caller-visible responsibilities beyond category-based boolean classification.

Traceability:
- FR-1 through FR-5
- `hard-locale.c`
- `hard_locale(int category) -> bool`