# spec.md

## Title

Rust Functional Specification for `module_gnu_scale10_round_14`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_scale10_round_14`
- Category: `module_cluster`
- Source branch: `020-module_gnu_scale10_round_14-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides decimal scaling-and-rounding support for floating-point values during formatted numeric conversion. Its role is to produce a decimal character string representing a floating-point magnitude rounded to a requested number of decimal digits, including support for values supplied either as decoded mantissa/exponent components or directly as `double` and `long double`.

The Rust rewrite must preserve the observed functional boundary of the C module:

- accept a binary floating-point value or an already decoded value,
- scale the value to a base-10 decimal form,
- round to a caller-specified decimal digit count,
- return the resulting decimal digit string in the same effective cases supported by the C source.

This specification is limited to behavior evidenced by:

- `scale10_round_decimal_decoded`
- `scale10_round_decimal_long_double`
- `scale10_round_decimal_double`

## Scope

### In Scope

- Decimal digit string generation for a decoded floating-point value.
- Decimal digit string generation for `double`.
- Decimal digit string generation for `long double`.
- Correct decimal rounding to a requested precision `n`.
- Consistent behavior between direct floating-point entry points and the decoded-value path for equivalent inputs.

### Out of Scope

- General-purpose number formatting beyond this decimal scaling-and-rounding behavior.
- Parsing numeric strings.
- Locale-specific formatting behavior.
- Signed formatting, exponent-field formatting, prefix/suffix insertion, or width/alignment behavior unless already implied by the returned decimal digit string behavior of these functions.
- New public APIs or capabilities not evidenced by the source module.

## Feature Specification

### Feature: Decimal scaling and rounding for decoded floating-point values

The module must support conversion of a decoded floating-point magnitude into a decimal digit string rounded to a requested number of digits.

A decoded value consists of:

- a binary exponent-like integer input `e`,
- a mantissa-like multi-precision numeric input `m`,
- a requested decimal precision `n`,
- and caller-supplied working memory.

The Rust implementation must produce the decimal string that corresponds to scaling the decoded value into decimal form and rounding it according to the module's existing behavior.

Observed intent from the C module indicates that this decoded path is the core behavior, and the `double` and `long double` entry points are wrappers that route their values into this logic.

### Feature: Decimal scaling and rounding for `double`

The module must accept a `double` value and a requested digit count `n`, and return the decimal digit string corresponding to the module's decimal scaling-and-rounding behavior for that value.

This path must be behaviorally aligned with the decoded-value path.

### Feature: Decimal scaling and rounding for `long double`

The module must accept a `long double` value and a requested digit count `n`, and return the decimal digit string corresponding to the module's decimal scaling-and-rounding behavior for that value.

This path must be behaviorally aligned with the decoded-value path.

## User Scenarios & Testing

### Scenario 1: Rounded decimal digits are needed from a `double`

A formatting path has a `double` numeric value and needs decimal digits rounded to precision `n` before placing them into a larger formatted output.

Expected support:

- the module accepts the `double`,
- computes decimal scaling,
- rounds to the requested digit count,
- returns the decimal digit sequence required by the caller.

Testing guidance:

- compare Rust output with C output for representative finite `double` values over multiple precisions,
- include values that require no rounding, values that round within existing digits, and values that trigger carry propagation.

### Scenario 2: Rounded decimal digits are needed from a `long double`

A formatting path has a `long double` value and needs decimal digits rounded to precision `n`.

Expected support:

- the module accepts the `long double`,
- applies the same decimal rounding semantics used by the C module,
- returns the rounded decimal digit sequence.

Testing guidance:

- compare Rust output with C output for representative finite `long double` values and precisions,
- include values near decimal half-way cases if supported by the test environment.

### Scenario 3: A caller already has a decoded mantissa/exponent representation

A higher-level formatting path has already decoded a floating-point magnitude into exponent and mantissa components and wants to avoid re-decoding.

Expected support:

- the module accepts decoded inputs,
- uses caller-provided working memory as part of the conversion flow,
- returns the same rounded decimal digit result that would be obtained through the direct floating-point path for an equivalent numeric value.

Testing guidance:

- construct decoded inputs equivalent to known `double` or `long double` values,
- verify decoded-path output matches wrapper-path output for the same effective value and precision.

### Scenario 4: Precision-sensitive rounding at digit boundaries

A caller requests decimal output at a precision where discarded digits determine whether the retained part must be incremented.

Expected support:

- the module performs correct decimal rounding,
- carry propagation across multiple digits is handled correctly,
- the resulting decimal digit string reflects the same rounding decision as the C source.

Testing guidance:

- include cases where rounding leaves digits unchanged,
- include cases where one trailing digit increments,
- include cases where repeated `9` digits force propagation across several positions.

## Requirements

### Functional Requirements

#### FR-1: Decoded-value conversion

The Rust module shall provide functionality equivalent to `scale10_round_decimal_decoded`, converting a decoded floating-point magnitude into a rounded decimal digit string based on input exponent `e`, mantissa `m`, and precision `n`.

Traceability: `gnu/vasnprintf.c:1178-1389`, `scale10_round_decimal_decoded`.

#### FR-2: `double` conversion

The Rust module shall provide functionality equivalent to `scale10_round_decimal_double`, accepting a `double` value and precision `n` and producing the rounded decimal digit string for that value.

Traceability: `gnu/vasnprintf.c:1419-1429`, `scale10_round_decimal_double`.

#### FR-3: `long double` conversion

The Rust module shall provide functionality equivalent to `scale10_round_decimal_long_double`, accepting a `long double` value and precision `n` and producing the rounded decimal digit string for that value.

Traceability: `gnu/vasnprintf.c:1398-1408`, `scale10_round_decimal_long_double`.

#### FR-4: Behavioral consistency between entry paths

For numerically equivalent inputs, the direct floating-point entry paths and the decoded-value path shall produce the same decimal digit string result, subject to the representational limits of the source type.

Traceability: `scale10_round_decimal_decoded`, `scale10_round_decimal_double`, `scale10_round_decimal_long_double` in `gnu/vasnprintf.c`.

#### FR-5: Precision-controlled decimal rounding

The Rust implementation shall round output according to the requested decimal digit count `n`, preserving the same effective rounding behavior as the C module for supported inputs.

Traceability: module purpose evidenced by `scale10_round_decimal_decoded` and its wrappers in `gnu/vasnprintf.c:1178-1429`.

#### FR-6: Decimal digit string result

The Rust implementation shall return the conversion result as a character string of decimal digits suitable for use by higher-level formatting logic, matching the C module's effective output form for the covered functions.

Traceability: return type and role of all three functions in `gnu/vasnprintf.c:1178-1429`.

### Key Entities

#### Entity: Decoded floating-point input

A decoded floating-point input is the internal representation accepted by the core conversion path. It combines:

- an integer exponent parameter `e`,
- a multi-precision mantissa parameter `m`,
- and a requested output precision `n`.

Relationship:
- This entity is consumed by the core decoded conversion function.
- The `double` and `long double` paths are functionally dependent on this same conversion behavior.

Traceability: `scale10_round_decimal_decoded (int e, mpn_t m, void *memory, int n)`.

#### Entity: Floating-point source value

A floating-point source value is either a `double` or `long double` provided directly by a caller.

Relationship:
- These values are adapted into the module's decimal scaling-and-rounding flow by the wrapper functions.
- Their resulting output must align with the decoded representation path for equivalent values.

Traceability:
- `scale10_round_decimal_double (double x, int n)`
- `scale10_round_decimal_long_double (long double x, int n)`

#### Entity: Decimal digit string

The decimal digit string is the module's output product.

Relationship:
- It is generated by the decoded conversion logic.
- It is the result returned by all three functions.
- It is intended for use by surrounding formatted-output code.

Traceability: `char *` return type of all three functions in `gnu/vasnprintf.c:1178-1429`.

#### Entity: Working memory

The decoded conversion path receives caller-provided working memory.

Relationship:
- It supports the decoded conversion process.
- It is part of the core function boundary and must be accounted for in the Rust rewrite's internal design, without expanding the module's public capability set.

Traceability: `void *memory` parameter of `scale10_round_decimal_decoded`.

## Success Criteria

### SC-1: Output equivalence for `double`

For a regression suite of representative finite `double` inputs and multiple precision values `n`, the Rust implementation produces the same decimal digit string as the C implementation.

Traceability: `scale10_round_decimal_double`, `gnu/vasnprintf.c:1419-1429`.

### SC-2: Output equivalence for `long double`

For a regression suite of representative finite `long double` inputs and multiple precision values `n`, the Rust implementation produces the same decimal digit string as the C implementation on the target platform.

Traceability: `scale10_round_decimal_long_double`, `gnu/vasnprintf.c:1398-1408`.

### SC-3: Output equivalence for decoded inputs

For a regression suite of decoded mantissa/exponent inputs corresponding to known numeric values, the Rust implementation produces the same decimal digit string as the C implementation for the same `e`, `m`, and `n`.

Traceability: `scale10_round_decimal_decoded`, `gnu/vasnprintf.c:1178-1389`.

### SC-4: Consistent cross-entry-path behavior

For test cases where a numeric value can be exercised both through a direct floating-point entry point and through an equivalent decoded representation, the Rust implementation returns matching decimal digit strings.

Traceability: relationship among all three functions in `gnu/vasnprintf.c:1178-1429`.

### SC-5: Correct boundary rounding behavior

In regression cases that exercise rounding boundaries, including carry propagation across one or more decimal digits, the Rust implementation matches the C module's output exactly.

Traceability: decimal rounding role of `scale10_round_decimal_decoded` and wrappers in `gnu/vasnprintf.c:1178-1429`.

## Notes for Porting Validation

- Validation should prioritize observable string output rather than internal algorithm parity.
- Where `long double` semantics differ by platform, equivalence should be measured against the C behavior on the same target environment.
- This specification intentionally does not require functionality beyond the evidenced decimal scaling-and-rounding boundary.