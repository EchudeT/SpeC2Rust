# spec.md

## Overview

This module provides locale classification logic for the program's main execution path. Its purpose is to determine whether a given locale category should be treated as a "hard" locale rather than the default simple locale. The Rust rewrite must preserve this decision behavior for the same locale-category inputs.

## Scope

In scope for this module:

- Determining whether a locale category currently resolves to a non-default locale that should be treated as hard.
- Returning the result as a boolean decision for the requested locale category.

Out of scope for this module:

- Changing locale settings.
- Formatting, translation, collation, or other locale-dependent operations beyond this boolean classification.
- Defining new locale categories or new public APIs.

## Feature Specification

### Feature: hard locale detection

The module exposes a single functional capability: given a locale category identifier, determine whether the active locale for that category is a hard locale.

Behavior the Rust version must implement:

- Accept a locale category identifier as input.
- Inspect the currently active locale name for that category.
- Return `false` when the locale corresponds to the default simple locale forms that are not considered hard.
- Return `true` for other locale values.
- Produce a stable boolean result for the current process locale state at the time of the call.

This behavior is traceable to:

- File: `hard-locale.c`
- Function: `hard_locale`

## User Scenarios & Testing

### Scenario 1: Default POSIX/C locale

A caller checks whether a locale category is hard while the process is using the standard simple locale.

Expected result:

- The function returns `false`.

Test guidance:

- Set the process locale for the tested category to a default simple locale form.
- Call the Rust equivalent of `hard_locale`.
- Verify the result is `false`.

### Scenario 2: Explicit non-default locale

A caller checks whether a locale category is hard while the process is using a locale other than the simple default locale.

Expected result:

- The function returns `true`.

Test guidance:

- Set the process locale for the tested category to a known non-default locale available in the test environment.
- Call the Rust equivalent.
- Verify the result is `true`.

### Scenario 3: Locale category-sensitive use

A caller evaluates more than one locale category and relies on the decision being based on the specific category passed in.

Expected result:

- The return value reflects the active locale of the requested category, not some other category.

Test guidance:

- Configure different locale values for different categories where supported.
- Call the function with each category.
- Verify each result matches the locale assigned to that category.

### Scenario 4: Alias handling for simple locale names

A caller checks a locale category whose active locale name is represented by a simple-locale alias form.

Expected result:

- Alias forms representing the default simple locale are treated as not hard and return `false`.

Test guidance:

- Use a locale environment that yields a recognized default simple locale alias.
- Call the function.
- Verify the result is `false`.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a function that accepts a locale category identifier and returns a boolean classification result.
  Traceability: `hard-locale.c`, `hard_locale`

- **FR-2**: The module shall determine the classification using the currently active locale associated with the provided category.
  Traceability: `hard-locale.c`, `hard_locale`

- **FR-3**: The module shall return `false` when the active locale name for the category denotes the standard simple locale rather than a hard locale.
  Traceability: `hard-locale.c`, `hard_locale`

- **FR-4**: The module shall return `true` when the active locale name for the category is not one of the simple default locale forms handled by the module.
  Traceability: `hard-locale.c`, `hard_locale`

- **FR-5**: The module shall make its decision independently for each invocation based on the category argument supplied to that invocation.
  Traceability: `hard-locale.c`, `hard_locale`

### Key Entities

- **Locale category identifier**: An integer input selecting which locale category to inspect. It is the sole input to the module's function.
  Traceability: `hard-locale.c`, `hard_locale`

- **Locale name**: The active locale string associated with the selected category and used as the basis for classification.
  Traceability: `hard-locale.c`, `hard_locale`

- **Hard-locale decision**: The boolean result returned by the function, indicating whether the selected category is treated as hard.
  Traceability: `hard-locale.c`, `hard_locale`

Relationship:

- A locale category identifier selects a locale name, and that locale name is classified into the hard-locale decision returned by the function.

## Success Criteria

- **SC-1**: For a tested category set to the standard simple locale, the Rust implementation returns `false`.
  Traceability: `hard-locale.c`, `hard_locale`

- **SC-2**: For a tested category set to a non-default locale, the Rust implementation returns `true`.
  Traceability: `hard-locale.c`, `hard_locale`

- **SC-3**: When different locale categories have different active locale values, calls using different category arguments return results that correspond to the locale of each specific category.
  Traceability: `hard-locale.c`, `hard_locale`

- **SC-4**: For recognized alias forms of the standard simple locale, the Rust implementation returns `false`.
  Traceability: `hard-locale.c`, `hard_locale`

- **SC-5**: The Rust module exposes no additional functional behavior beyond the boolean hard-locale classification evidenced by the source module.
  Traceability: `hard-locale.c`, `hard_locale`