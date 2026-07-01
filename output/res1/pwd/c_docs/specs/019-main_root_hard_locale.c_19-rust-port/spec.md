# spec.md

## Title
Functional Specification for `main_root_hard-locale.c_19`

## Metadata
- Project: `pwd`
- Module: `main_root_hard-locale.c_19`
- Category: `main_cluster`
- Source file: `hard-locale.c`
- Primary function: `hard_locale(int category) -> bool`
- Rust branch target: `019-main_root_hard_locale.c_19-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides a single locale-classification function that determines whether the current locale setting for a specified locale category should be treated as a non-trivial locale rather than the default simple locale.

The Rust rewrite must preserve the observable behavior of this classification function: given a locale category identifier, it inspects the active locale name for that category and returns whether it represents a "hard" locale, meaning a locale other than the default `"C"` or `"POSIX"` locale.

This module is functional and narrowly scoped. No broader locale management behavior is evidenced by the source analysis and must not be added.

## Feature Specification

### Feature: Hard-locale classification
The module shall expose functionality equivalent to `hard_locale(int category) -> bool`.

Behavior required from the Rust version:
- Accept a locale category identifier corresponding to a standard locale category accepted by the underlying locale query mechanism.
- Query the currently active locale name for that category.
- Return `false` when the locale name is the default locale `"C"`.
- Return `false` when the locale name is the default locale `"POSIX"`.
- Return `true` for other locale names.
- Operate as a pure classification step from the caller's perspective: the function determines locale hardness and reports it as a boolean result.

### Functional boundary
This module's responsibility ends at determining whether the locale for a category is hard or default-like. It does not:
- set or modify locale state,
- normalize locale names beyond the evidenced `"C"` and `"POSIX"` checks,
- provide formatting, translation, or parsing behavior,
- expose additional locale utilities.

## User Scenarios & Testing

### Scenario 1: Caller checks whether a locale category is default
A caller needs to know whether behavior tied to locale should be treated as default/simple for a given category.

Expected outcome:
- If the current locale name for that category is `"C"` or `"POSIX"`, the function returns `false`.

Test cases:
- With the queried category set to `"C"`, result is `false`.
- With the queried category set to `"POSIX"`, result is `false`.

### Scenario 2: Caller checks whether a locale category is non-default
A caller needs to detect whether locale-sensitive behavior should be considered active for a given category.

Expected outcome:
- If the current locale name for that category is any locale name other than `"C"` or `"POSIX"`, the function returns `true`.

Test cases:
- With the queried category set to a named locale such as `en_US.UTF-8`, result is `true`.
- With the queried category set to another non-default locale name, result is `true`.

### Scenario 3: Caller passes a supported locale category identifier
A caller invokes the function with a locale category constant supported by the platform C locale interface.

Expected outcome:
- The function performs classification for that category and returns a boolean outcome without introducing additional side effects in module behavior.

Test cases:
- Invoke using a standard category such as `LC_CTYPE` and verify the returned value reflects the active locale name for that category.

## Requirements

### Functional Requirements

#### FR-1: Locale category input
The Rust module shall provide a function equivalent in purpose to `hard_locale` that accepts a locale category identifier as input.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-2: Current locale inspection
The function shall determine its result by inspecting the current locale associated with the provided category.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-3: Default locale classification
The function shall classify the locale as not hard when the locale name for the requested category is exactly `"C"`.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-4: POSIX locale classification
The function shall classify the locale as not hard when the locale name for the requested category is exactly `"POSIX"`.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-5: Non-default locale classification
The function shall classify the locale as hard when the locale name for the requested category is not `"C"` and not `"POSIX"`.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-6: Boolean result
The function shall report the classification result as a boolean value.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

### Key Entities

#### Entity: Locale category identifier
- Represents the caller-supplied locale category selection.
- Serves as the input used to identify which locale setting is inspected.

Traceability:
- Function parameter: `int category`

#### Entity: Locale name
- Represents the currently active locale name corresponding to the requested category.
- Is compared against the recognized default names `"C"` and `"POSIX"`.

Traceability:
- `hard-locale.c`
- `hard_locale(int category)`

#### Entity: Hard-locale boolean classification
- Represents the function output.
- `false` means default/simple locale (`"C"` or `"POSIX"`).
- `true` means a non-default locale name.

Traceability:
- Function return type: `bool`
- `hard_locale(int category)`

## Success Criteria

### SC-1: Correct classification of `"C"`
When the active locale name for a tested category is `"C"`, the Rust implementation returns `false`.

Traceability:
- FR-3
- `hard_locale(int category)`

### SC-2: Correct classification of `"POSIX"`
When the active locale name for a tested category is `"POSIX"`, the Rust implementation returns `false`.

Traceability:
- FR-4
- `hard_locale(int category)`

### SC-3: Correct classification of non-default locale names
When the active locale name for a tested category is a name other than `"C"` or `"POSIX"`, the Rust implementation returns `true`.

Traceability:
- FR-5
- `hard_locale(int category)`

### SC-4: Category-based evaluation
For at least one standard locale category supplied by the caller, the Rust implementation evaluates the locale associated with that category rather than returning a fixed constant.

Traceability:
- FR-1
- FR-2
- `hard_locale(int category)`

### SC-5: Boolean interface preservation
The Rust implementation exposes this module behavior as a boolean-returning classification function matching the source module's functional contract.

Traceability:
- FR-6
- `hard_locale(int category)`