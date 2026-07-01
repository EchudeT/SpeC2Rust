# spec.md

## Title

Functional Specification: `main_root_hard-locale.c_28`

## Metadata

- **Project**: `cat`
- **Module**: `main_root_hard-locale.c_28`
- **Category**: `main_cluster`
- **Source file**: `hard-locale.c`
- **Primary function**: `hard_locale(int category) -> bool`
- **Rust branch target**: `029-main_root_hard_locale.c_28-rust-port`
- **Generation date**: `2026-06-06`

## Overview

This module provides a single locale-query function that determines whether the current locale for a specified locale category should be treated as a non-trivial locale rather than the default simple locale.

The Rust rewrite must preserve this behavioral role: given a locale category identifier, it must inspect the active locale name for that category and return a boolean indicating whether the locale is effectively a "hard" locale. In this module's context, the simple locale is the standard default locale represented by `"C"` and, where treated equivalently, `"POSIX"`.

The module is purely query-oriented. It does not define persistent state, configuration objects, or output formatting behavior.

## Feature Specification

### Feature: locale hardness detection

The module shall provide functionality to classify the active locale of a requested category as either:

- **not hard** when the locale is the default simple locale, or
- **hard** when the locale is some other active locale value.

This classification is intended to let higher-level program logic decide whether locale-sensitive behavior should follow simple default-locale assumptions or should acknowledge a locale-specific environment.

### Rust implementation scope

The Rust version must implement the same functional boundary as the C module:

- accept a locale category input;
- obtain the current locale setting associated with that category;
- compare that locale against the default simple-locale values used by the source behavior;
- return `false` for default simple locale;
- return `true` for other locale values.

No additional public capabilities are in scope.

## User Scenarios & Testing

### Scenario 1: Default `"C"` locale for a category

A caller checks whether a locale category is non-default while that category is set to `"C"`.

**Expected result**:
- The function returns `false`.

**Test guidance**:
- Set or simulate the target category's current locale as `"C"`.
- Call `hard_locale(category)`.
- Verify the result is `false`.

### Scenario 2: Default `"POSIX"` locale for a category

A caller checks whether a locale category is non-default while that category is set to `"POSIX"`.

**Expected result**:
- The function returns `false`.

**Test guidance**:
- Set or simulate the target category's current locale as `"POSIX"`.
- Call `hard_locale(category)`.
- Verify the result is `false`.

### Scenario 3: Non-default locale for a category

A caller checks a category whose current locale is a locale other than `"C"` and `"POSIX"`.

**Expected result**:
- The function returns `true`.

**Test guidance**:
- Set or simulate the target category's current locale as a non-default locale name.
- Call `hard_locale(category)`.
- Verify the result is `true`.

### Scenario 4: Category-sensitive query

A caller evaluates more than one locale category, and the categories may have different active locale values.

**Expected result**:
- The function result reflects the active locale of the specific category passed to the function.
- Different categories may produce different results if their locale settings differ.

**Test guidance**:
- Arrange distinct locale values for two categories.
- Call the function with each category.
- Verify each result matches that category's locale value.

## Requirements

### Functional Requirements

#### FR-1: Category-based locale query

The module shall accept a locale category identifier and evaluate the current locale associated with that category.

**Traceability**:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-2: Default locale classification

The module shall classify the locale values `"C"` and `"POSIX"` as not hard.

**Traceability**:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-3: Non-default locale classification

The module shall classify a locale value other than `"C"` and `"POSIX"` as hard.

**Traceability**:
- `hard-locale.c`
- `hard_locale(int category)`

#### FR-4: Boolean result contract

The module shall return a boolean result representing the hardness classification for the requested locale category.

**Traceability**:
- `hard-locale.c`
- `hard_locale(int category) -> bool`

### Key Entities

#### Entity: Locale category identifier

An input value identifying which locale category is being queried.

**Relationships**:
- Consumed by `hard_locale`.
- Determines which active locale setting is examined.

#### Entity: Active locale name

The current locale name associated with the requested locale category.

**Relationships**:
- Derived from the locale category input.
- Compared against the default simple locale names.
- Drives the boolean return value.

#### Entity: Hard-locale classification

A boolean classification result.

**Relationships**:
- Computed from the active locale name.
- Returned to callers of `hard_locale`.

## Success Criteria

### SC-1: Correct default-locale handling

For a tested category whose active locale is `"C"`, the Rust implementation returns `false`.

**Traceability**:
- FR-2
- `hard_locale(int category)`

### SC-2: Correct POSIX-locale handling

For a tested category whose active locale is `"POSIX"`, the Rust implementation returns `false`.

**Traceability**:
- FR-2
- `hard_locale(int category)`

### SC-3: Correct non-default handling

For a tested category whose active locale is neither `"C"` nor `"POSIX"`, the Rust implementation returns `true`.

**Traceability**:
- FR-3
- `hard_locale(int category)`

### SC-4: Category-specific behavior preservation

When different locale categories have different active locale values, the Rust implementation returns results corresponding to the specific category argument used in each call.

**Traceability**:
- FR-1
- `hard_locale(int category)`

### SC-5: Boolean API preservation

The Rust rewrite exposes behavior equivalent to a single boolean-returning locale-hardness query for a supplied locale category, with no additional required output to determine the classification.

**Traceability**:
- FR-4
- `hard_locale(int category) -> bool`