# spec.md

## Title

Rust Functional Specification for `module_gnu_is_infinite_18`

## Status

Draft

## Scope

This specification covers the Rust rewrite of the floating-point classification behavior represented by the `module_gnu_is_infinite_18` analysis result from `gnu/vasnprintf.c`.

The module scope is limited to behavior evidenced by these internal helpers:

- `is_infinite_or_zero`
- `is_infinite_or_zerol`

The Rust version must preserve the module’s observable functional role: classify floating-point inputs used by formatting logic to determine whether a value is either infinite or zero. No additional capabilities are in scope.

## Feature Specification

### Summary

This module provides boolean-like classification of floating-point values for two floating-point domains:

- `double`
- `long double`

The classification answers one question for a given input value:

- Is the value infinite or zero?

The Rust rewrite must implement equivalent behavior for both supported floating-point categories used by the source module’s formatting path.

### Functional Behavior

For each supported floating-point input type, the module shall:

1. Accept a floating-point value.
2. Evaluate whether the value is one of:
   - positive infinity
   - negative infinity
   - positive zero
   - negative zero, where the representation distinguishes signed zero
3. Return an integer-like truth result indicating:
   - true/nonzero when the value is infinite or zero
   - false/zero otherwise

### Out of Scope

The Rust rewrite must not introduce functionality not evidenced by the source analysis, including but not limited to:

- generalized floating-point classification beyond this predicate
- public formatting APIs
- parsing
- NaN payload inspection
- error recovery or fallback policies
- persistence, serialization, or FFI surfaces

## User Scenarios & Testing

### Scenario 1: Detect zero during numeric formatting preparation

A formatting path evaluates a floating-point value before selecting how to print it. When the value is exactly zero, the module reports a positive classification result so the caller can follow the zero-specific formatting path.

**Test expectations**
- Input `0.0` yields true/nonzero.
- Input `-0.0` yields true/nonzero if the Rust floating-point type preserves signed zero semantics.

### Scenario 2: Detect infinity during numeric formatting preparation

A formatting path evaluates a floating-point value before rendering it. When the value is positive or negative infinity, the module reports a positive classification result so the caller can follow the infinity-specific formatting path.

**Test expectations**
- Input positive infinity yields true/nonzero.
- Input negative infinity yields true/nonzero.

### Scenario 3: Reject ordinary finite nonzero values

A formatting path checks whether special handling is needed. For a regular finite nonzero number, the module reports a negative classification result so ordinary formatting continues.

**Test expectations**
- Input `1.0` yields false/zero.
- Input `-2.5` yields false/zero.
- Input smallest positive normal finite value yields false/zero.
- Input smallest positive subnormal finite value, if representable in the Rust type used, yields false/zero.

### Scenario 4: Reject NaN values

A formatting path distinguishes infinity/zero handling from other non-finite states. For NaN input, the module must not classify the value as infinite-or-zero.

**Test expectations**
- Quiet NaN yields false/zero.
- Signaling NaN, if representable and testable in the Rust environment, yields false/zero.

### Scenario 5: Support both source floating-point categories

The original module contains separate helpers for `double` and `long double`. The Rust rewrite must preserve equivalent classification coverage for both categories required by the rewritten formatting logic.

**Test expectations**
- The rewrite includes one classification path corresponding to `double`.
- The rewrite includes one classification path corresponding to `long double` behavior, mapped to an appropriate Rust representation used by the port.
- Both paths satisfy the same truth table for zero, infinity, finite nonzero, and NaN inputs.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide classification for the source `double`-based helper behavior represented by `is_infinite_or_zero`.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`

- **FR-2**: The module shall provide classification for the source `long double`-based helper behavior represented by `is_infinite_or_zerol`.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zerol`

- **FR-3**: For each supported floating-point category, the module shall return a positive result when the input is positive infinity or negative infinity.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

- **FR-4**: For each supported floating-point category, the module shall return a positive result when the input is positive zero or negative zero.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

- **FR-5**: For each supported floating-point category, the module shall return a negative result when the input is finite and nonzero.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

- **FR-6**: For each supported floating-point category, the module shall return a negative result when the input is NaN.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

- **FR-7**: The Rust rewrite shall preserve integer-like truth semantics compatible with the source helpers’ yes/no result contract.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`

### Key Entities

- **Floating-point input value**
  - A numeric value supplied for classification.
  - Exists in two source categories: one corresponding to `double`, one corresponding to `long double`.
  - Relationship: this is the sole input consumed by the classification helpers.

- **Classification result**
  - A binary truth result indicating whether the input belongs to the set `{infinity, zero}`.
  - Relationship: derived directly from one floating-point input value.

- **Local anonymous struct in `gnu/vasnprintf.c`**
  - Evidenced in the analyzed source region but not shown to define the primary behavior of the two classification helpers.
  - Relationship: no additional functional requirement is derived from this structure for this module specification due to insufficient evidence of behavioral responsibility.

## Success Criteria

- **SC-1**: For the Rust equivalent of `is_infinite_or_zero`, automated tests confirm true/nonzero results for `+0.0`, `-0.0`, `+∞`, and `-∞`.
  **Traceability:** `is_infinite_or_zero`

- **SC-2**: For the Rust equivalent of `is_infinite_or_zero`, automated tests confirm false/zero results for at least one positive finite nonzero value, one negative finite nonzero value, and one NaN value.
  **Traceability:** `is_infinite_or_zero`

- **SC-3**: For the Rust equivalent of `is_infinite_or_zerol`, automated tests confirm true/nonzero results for zero and infinity cases in the Rust representation chosen for the source `long double` behavior.
  **Traceability:** `is_infinite_or_zerol`

- **SC-4**: For the Rust equivalent of `is_infinite_or_zerol`, automated tests confirm false/zero results for finite nonzero and NaN cases in the Rust representation chosen for the source `long double` behavior.
  **Traceability:** `is_infinite_or_zerol`

- **SC-5**: No test demonstrates classification of NaN as infinite-or-zero.
  **Traceability:** `is_infinite_or_zero`, `is_infinite_or_zerol`

- **SC-6**: No requirement or implemented behavior extends beyond the evidenced predicate “is infinite or zero” for the two source floating-point categories.
  **Traceability:** `gnu/vasnprintf.c`, `is_infinite_or_zero`, `is_infinite_or_zerol`