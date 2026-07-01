# spec.md

## Title

Rust Functional Specification for `module_gnu_is_infinite_18`

## Summary

This module provides floating-point classification support used by formatted output logic in `gnu/vasnprintf.c`. Its evidenced behavior is limited to detecting whether a floating-point input is either infinite or zero, for both `double` and `long double` inputs.

The Rust rewrite must preserve this classification behavior so that surrounding formatting logic can make the same decisions for special floating-point values.

## Scope

### In Scope

- Classification of `double`-equivalent values as either:
  - infinite, or
  - zero
- Classification of `long double`-equivalent values as either:
- Returning an integer-style truth value consistent with the source module’s behavior

### Out of Scope

- General-purpose floating-point formatting
- NaN-specific reporting beyond what is implied by “not infinite and not zero”
- Parsing numeric strings
- Public API expansion beyond the evidenced module behavior
- Any functionality not traceable to `gnu/vasnprintf.c` and the listed functions

## Feature Specification

### Feature: Special-value classification for floating-point formatting decisions

The module supplies helper behavior for checking whether a floating-point input belongs to a narrow set of special cases relevant to formatting: infinity or zero.

Two classifications are required:

1. A classification for values corresponding to C `double`
2. A classification for values corresponding to C `long double`

For each supported floating-point category, the Rust version must evaluate an input and report whether the input is:
- positive infinity,
- negative infinity,
- positive zero, or
- negative zero

Inputs outside those cases must be reported as not matching.

### Required Rust Behavior

The Rust rewrite must implement behavior equivalent to the following source responsibilities:

- For `double`-precision inputs, determine whether the value is infinite or zero.
- For extended-precision inputs corresponding to `long double`, determine whether the value is infinite or zero.
- Produce a result usable as a boolean/int predicate by surrounding formatting logic.

No additional classifications are required by this module specification.

## User Scenarios & Testing

### Scenario 1: Formatting logic checks a `double` special case

A caller in formatted-output processing needs to know whether a `double` value should be treated as a special case before normal numeric formatting.

- Input: `+0.0`
- Expected result: classified as matching

Test expectation:
- The Rust implementation returns a truthy/int-equivalent result for `+0.0`.

### Scenario 2: Formatting logic checks negative zero for `double`

A caller evaluates a `double` value that is numerically zero but may carry a sign.

- Input: `-0.0`
- Expected result: classified as matching

Test expectation:
- The Rust implementation returns a truthy/int-equivalent result for `-0.0`.

### Scenario 3: Formatting logic checks infinity for `double`

A caller receives a `double` that is not finite.

- Input: `+∞` or `-∞`
- Expected result: classified as matching

Test expectation:
- The Rust implementation returns a truthy/int-equivalent result for both positive and negative infinity.

### Scenario 4: Formatting logic checks an ordinary finite `double`

A caller evaluates a normal finite nonzero `double` during formatted output.

- Input: `1.0`
- Expected result: classified as not matching

Test expectation:
- The Rust implementation returns a falsy/int-equivalent result for finite nonzero values.

### Scenario 5: Formatting logic checks a `long double` special case

A caller performs the same special-case check for a `long double`-class value.

- Input: zero or infinity in the `long double` domain
- Expected result: classified as matching

Test expectation:
- The Rust implementation returns a truthy/int-equivalent result for zero and infinity cases represented in the supported `long double` mapping.

### Scenario 6: Formatting logic checks an ordinary finite `long double`

A caller evaluates a finite nonzero `long double`-class value.

- Input: finite nonzero value
- Expected result: classified as not matching

Test expectation:
- The Rust implementation returns a falsy/int-equivalent result.

### Scenario 7: NaN does not match the predicate

A caller passes a NaN value into the classifier.

- Input: NaN
- Expected result: not classified as infinite-or-zero

Test expectation:
- The Rust implementation returns a falsy/int-equivalent result for NaN inputs.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a predicate corresponding to `is_infinite_or_zero` that reports whether a `double`-class input is infinite or zero.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`

- **FR-2**: The module shall provide a predicate corresponding to `is_infinite_or_zerol` that reports whether a `long double`-class input is infinite or zero.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zerol`

- **FR-3**: The `double`-class predicate shall treat both positive and negative infinity as matching inputs.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`

- **FR-4**: The `double`-class predicate shall treat both positive zero and negative zero as matching inputs.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`

- **FR-5**: The `double`-class predicate shall treat finite nonzero values as non-matching inputs.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`

- **FR-6**: The `long double`-class predicate shall treat both positive and negative infinity as matching inputs within the Rust representation chosen for the source `long double` behavior.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zerol`

- **FR-7**: The `long double`-class predicate shall treat both positive zero and negative zero as matching inputs within the Rust representation chosen for the source `long double` behavior.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zerol`

- **FR-8**: The `long double`-class predicate shall treat finite nonzero values as non-matching inputs.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zerol`

- **FR-9**: For both predicates, NaN inputs shall be reported as non-matching. This is required because the module’s evidenced responsibility is only “infinite or zero,” and NaN is outside that set.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

- **FR-10**: Each predicate shall return a result compatible with predicate-style use by surrounding formatting logic, preserving the original integer truth-value behavior at the module boundary.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

### Key Entities

- **Floating-point input (`double` class)**
  The input examined by the predicate corresponding to `is_infinite_or_zero`. Its relationship to the module is as the value being classified into matching or non-matching special cases.

- **Floating-point input (`long double` class)**
  The input examined by the predicate corresponding to `is_infinite_or_zerol`. Its relationship to the module is the same classification role for the extended-precision path used by surrounding formatting logic.

- **Predicate result**
  The integer-style truth value returned by each classifier. It expresses whether the input belongs to the module’s supported special cases.

- **Anonymous struct in `gnu/vasnprintf.c`**
  A local data structure exists in the file, but based on the supplied evidence it is not a core functional entity for this module’s special-value classification responsibility. The Rust rewrite does not need to assign additional behavior to it unless required by direct integration with the surrounding file logic.
  **Traceability:** `gnu/vasnprintf.c:426-430`

## Success Criteria

- **SC-1**: For `double`-class inputs, tests show that `+0.0`, `-0.0`, `+∞`, and `-∞` all produce a matching result.
  **Traceability:** `is_infinite_or_zero`

- **SC-2**: For `double`-class inputs, tests show that at least one ordinary finite nonzero value produces a non-matching result.
  **Traceability:** `is_infinite_or_zero`

- **SC-3**: For `double`-class inputs, tests show that NaN produces a non-matching result.
  **Traceability:** `is_infinite_or_zero`

- **SC-4**: For the Rust representation used for `long double` behavior, tests show that zero and infinity cases produce a matching result.
  **Traceability:** `is_infinite_or_zerol`

- **SC-5**: For the Rust representation used for `long double` behavior, tests show that a finite nonzero value produces a non-matching result.
  **Traceability:** `is_infinite_or_zerol`

- **SC-6**: The Rust module exposes no required functionality beyond the two evidenced classification responsibilities.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`