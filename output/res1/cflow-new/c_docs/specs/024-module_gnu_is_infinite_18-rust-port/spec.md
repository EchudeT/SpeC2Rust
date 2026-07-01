# spec.md

## Title

Rust Port Functional Specification: `module_gnu_is_infinite_18`

## Summary

This module provides floating-point classification support used by formatted output logic in `gnu/vasnprintf.c`. Its evidenced behavior is limited to detecting whether a floating-point input is either infinite or zero, for both `double` and `long double` inputs.

The Rust rewrite must preserve this classification behavior so that higher-level formatting code can make the same decisions for special numeric values.

## Scope

In scope:

- Classification of `double`-equivalent values as infinite-or-zero.
- Classification of `long double`-equivalent values as infinite-or-zero.
- Support for use from surrounding numeric formatting logic in the same module context.

Out of scope:

- General-purpose floating-point formatting.
- Classification beyond the evidenced infinite-or-zero behavior.
- Any new public API, portability layer, or extended numeric utilities not evidenced by the source module analysis.

## Source Basis

This specification is derived from the following evidenced module elements:

- File: `gnu/vasnprintf.c`
- Functions:
  - `is_infinite_or_zero`
  - `is_infinite_or_zerol`
- Data structure:
  - anonymous struct at `gnu/vasnprintf.c:426-430`

## Feature Specification

### Feature: Detect infinite-or-zero floating-point values

The module must provide behavior equivalent to the C module’s internal helpers that determine whether a floating-point value belongs to the special set:

- positive infinity
- negative infinity
- positive zero
- negative zero

This behavior must exist for:

- one path corresponding to C `double`
- one path corresponding to C `long double`

The result must be usable by surrounding formatting logic as a boolean-like classification outcome indicating whether the input is in that special set.

### Behavioral Notes

- Finite nonzero values must not be classified as infinite-or-zero.
- NaN values must not be classified as infinite-or-zero unless directly evidenced otherwise; no such evidence is present in the analyzed module.
- Signed zero must be accepted as zero.
- Both positive and negative infinity must be accepted as infinite.

## User Scenarios & Testing

### Scenario 1: Formatting logic checks a `double` before numeric processing

A caller in formatted output processing has a `double` value and needs to know whether it is a special case requiring alternate handling.

Expected support:

- When the value is `0.0`, the classification reports true.
- When the value is `-0.0`, the classification reports true.
- When the value is `+∞`, the classification reports true.
- When the value is `-∞`, the classification reports true.
- When the value is a finite nonzero number such as `1.0` or `-3.5`, the classification reports false.
- When the value is NaN, the classification reports false.

### Scenario 2: Formatting logic checks a `long double` before numeric processing

A caller in formatted output processing has a `long double`-class value and needs the same special-case decision.

Expected support:

- Zero values, including signed zero, classify as true.
- Positive and negative infinity classify as true.
- Finite nonzero values classify as false.
- NaN classifies as false.

### Scenario 3: Consistent decision behavior across supported floating-point kinds

Higher-level formatting code depends on equivalent semantics for special-value detection across both supported floating-point categories.

Expected support:

- The `double` and `long double` classification paths apply the same logical rule: return true exactly for zero or infinity, otherwise false.
- Inputs representing the same mathematical special cases produce equivalent classification outcomes in each supported type domain.

### Testing

The Rust version must include tests covering, at minimum:

- `double`-equivalent:
  - `0.0`
  - `-0.0`
  - `f64::INFINITY`
  - `f64::NEG_INFINITY`
  - representative finite nonzero positive value
  - representative finite nonzero negative value
  - `f64::NAN`
- `long double`-equivalent port path:
  - zero
  - signed zero if representable by the chosen Rust representation
  - positive infinity
  - negative infinity
  - representative finite nonzero values
  - NaN

If the Rust port maps C `long double` behavior onto a Rust type with platform constraints, tests must still verify the specified logical classification behavior for the chosen representation.

## Requirements

### Functional Requirements

#### FR-1: `double` special-set classification

The module shall provide behavior equivalent to `is_infinite_or_zero` from `gnu/vasnprintf.c` for `double`-class inputs.

Traceability:

- `gnu/vasnprintf.c:392-396`
- Function: `is_infinite_or_zero`

#### FR-2: `double` true cases

For the `double` classification behavior, the module shall report true for:

- positive zero
- negative zero
- positive infinity
- negative infinity

Traceability:

- `gnu/vasnprintf.c:392-396`
- Function: `is_infinite_or_zero`

#### FR-3: `double` false cases

For the `double` classification behavior, the module shall report false for finite nonzero values.

Traceability:

- `gnu/vasnprintf.c:392-396`
- Function: `is_infinite_or_zero`

#### FR-4: `long double` special-set classification

The module shall provide behavior equivalent to `is_infinite_or_zerol` from `gnu/vasnprintf.c` for `long double`-class inputs.

Traceability:

- `gnu/vasnprintf.c:403-407`
- Function: `is_infinite_or_zerol`

#### FR-5: `long double` true cases

For the `long double` classification behavior, the module shall report true for:

- positive zero
- negative zero
- positive infinity
- negative infinity

Traceability:

- `gnu/vasnprintf.c:403-407`
- Function: `is_infinite_or_zerol`

#### FR-6: `long double` false cases

For the `long double` classification behavior, the module shall report false for finite nonzero values.

Traceability:

- `gnu/vasnprintf.c:403-407`
- Function: `is_infinite_or_zerol`

#### FR-7: Internal consumption by formatting logic

The Rust port shall make these classification results available for use by surrounding formatted-output logic within the rewritten module context.

Traceability:

- File context: `gnu/vasnprintf.c`
- Functions: `is_infinite_or_zero`, `is_infinite_or_zerol`

### Key Entities

#### Entity: `double`-class floating-point input

A numeric input corresponding to the C `double` domain is consumed by the `double` classification behavior and yields a boolean-like decision indicating membership in the infinite-or-zero set.

Relationship:

- Evaluated by the behavior traced to `is_infinite_or_zero`.

Traceability:

- `gnu/vasnprintf.c:392-396`

#### Entity: `long double`-class floating-point input

A numeric input corresponding to the C `long double` domain is consumed by the `long double` classification behavior and yields a boolean-like decision indicating membership in the infinite-or-zero set.

Relationship:

- Evaluated by the behavior traced to `is_infinite_or_zerol`.

Traceability:

- `gnu/vasnprintf.c:403-407`

#### Entity: anonymous struct in `gnu/vasnprintf.c`

An anonymous struct is present in the analyzed source region. The provided evidence does not establish it as part of the functional boundary of this classification module beyond co-location in the file.

Relationship:

- No normative functional requirement in this specification depends on this struct, because no direct behavioral role is evidenced by the supplied analysis.

Traceability:

- `gnu/vasnprintf.c:426-430`

## Success Criteria

### SC-1: Correct `double` classification

Automated tests demonstrate that the Rust port’s `double`-class behavior returns true for `+0.0`, `-0.0`, positive infinity, and negative infinity, and false for representative finite nonzero values.

Traceability:

- FR-1, FR-2, FR-3
- `is_infinite_or_zero`

### SC-2: Correct `long double` classification

Automated tests demonstrate that the Rust port’s `long double`-class behavior returns true for zero and infinities and false for representative finite nonzero values, using the Rust representation chosen for the port.

Traceability:

- FR-4, FR-5, FR-6
- `is_infinite_or_zerol`

### SC-3: NaN exclusion

Automated tests demonstrate that NaN is not classified as infinite-or-zero in both supported floating-point classification paths.

Traceability:

- Feature behavior constrained by the evidenced function purpose in:
  - `is_infinite_or_zero`
  - `is_infinite_or_zerol`

### SC-4: Semantic consistency across both paths

Tests demonstrate that the `double` and `long double` classification paths apply the same rule: true exactly for zero or infinity, false otherwise.

Traceability:

- FR-1 through FR-6
- `gnu/vasnprintf.c:392-407`