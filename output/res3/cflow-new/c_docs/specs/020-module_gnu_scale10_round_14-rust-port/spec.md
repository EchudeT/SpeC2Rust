# spec.md

## Title

Rust Functional Specification for `module_gnu_scale10_round_14`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_scale10_round_14`
- Category: `module_cluster`
- Source basis: `gnu/vasnprintf.c`
- Rust branch target: `020-module_gnu_scale10_round_14-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides decimal scaling-and-rounding support for floating-point values as character output. Its role is to produce a decimal string rounded to a requested precision after applying base-10 scaling logic, for inputs represented either as decoded significand/exponent data or as native `double` / `long double` values.

The Rust rewrite must preserve the observed functional boundary of the C module:

- accept a binary floating-point value or an already-decoded numeric representation,
- produce a newly created decimal character string,
- apply decimal rounding according to the requested digit count,
- provide equivalent behavior across the three source entry points that serve decoded values, `double`, and `long double`.

This specification covers only the functionality evidenced by:

- `scale10_round_decimal_decoded`
- `scale10_round_decimal_long_double`
- `scale10_round_decimal_double`

## Feature Specification

### Feature Summary

The module converts finite floating-point numeric input into a rounded decimal string using decimal scaling by powers of 10. It supports three closely related use paths:

1. formatting from a decoded mantissa/exponent representation,
2. formatting from a `double`,
3. formatting from a `long double`.

The decoded-value path is the functional core. The native floating-point paths must preserve the same externally visible decimal-rounding behavior after converting their inputs into the internal numeric form needed by the module.

### Functional Scope

The Rust version must implement:

- decimal string generation for a supplied numeric value,
- rounding to a caller-specified decimal precision parameter `n`,
- behavior for both `double` and `long double` inputs,
- equivalent output generation when the same mathematical value is provided via the decoded-input path.

### Explicit Non-Scope

The Rust version must not claim or introduce capabilities not evidenced here, including:

- a new public formatting API unrelated to these functions,
- generalized locale-aware formatting,
- parsing of decimal strings back into numbers,
- exponent-notation output as a separate feature unless required to preserve observed output behavior of these functions,
- thread-safety or synchronization guarantees,
- persistence, serialization, or FFI promises.

## User Scenarios & Testing

### Scenario 1: Round a decoded floating-point value to decimal digits

A caller already has a value represented as a binary exponent and multiprecision significand and needs a decimal string rounded to `n` digits.

Expected support:

- the Rust module accepts the decoded numeric components corresponding to the C function boundary,
- returns a character string representing the rounded decimal result,
- the result is stable for repeated calls with the same numeric input and `n`.

Testing guidance:

- prepare decoded inputs representing exact powers of 10, midpoints, and non-terminating decimal expansions,
- verify returned strings match the C module behavior for the same inputs,
- verify different `n` values change only the requested precision/rounding result.

### Scenario 2: Round a `double` value for decimal output

A caller has a `double` and needs the same decimal scaling-and-rounding behavior without manually decoding the value first.

Expected support:

- the Rust module accepts a `double`-equivalent Rust floating-point value,
- produces the same rounded decimal string that the decoded path would produce for the same numeric value,
- preserves sign and decimal rounding behavior.

Testing guidance:

- compare output for representative positive and negative values,
- compare exact integers, small fractions, and values near decimal rounding boundaries,
- compare the Rust result to the existing C implementation.

### Scenario 3: Round a `long double` value for decimal output

A caller has an extended-precision floating-point value and requires decimal output rounded with the same functional semantics.

Expected support:

- the Rust module accepts a value equivalent to the source module’s `long double` path,
- produces a rounded decimal string according to the requested precision `n`,
- preserves the higher-precision path’s behavior rather than collapsing it into unrelated `double` semantics.

Testing guidance:

- test values that differ only in extended precision beyond `double`,
- verify outputs match the C implementation on the target platform/profile used for port validation,
- verify precision-sensitive rounding cases.

### Scenario 4: Precision-driven rounding changes output at decimal boundaries

A caller relies on correct decimal rounding when the next digit is exactly at or near the rounding threshold.

Expected support:

- the Rust module rounds correctly when discarded digits are below, equal to, or above the threshold that changes the retained decimal result,
- carry propagation across one or more digits is reflected in the returned string.

Testing guidance:

- use values whose decimal expansions trigger no carry, single-digit carry, and multi-digit carry,
- include cases such as rounding that changes trailing `9` sequences,
- compare each output directly with the C implementation.

### Scenario 5: Returned output is an owned string result

A caller needs a standalone string result that can outlive temporary computation state.

Expected support:

- the Rust module returns an owned string value corresponding to the C function’s allocated character buffer result,
- the returned content is null-free textual decimal data suitable for subsequent formatting assembly.

Testing guidance:

- verify returned strings remain valid after internal temporaries are dropped,
- verify no mutation of caller-provided numeric inputs occurs.

## Requirements

### Functional Requirements

#### FR-1: Decimal rounded string generation
The module shall generate a decimal character string for the supplied numeric input, as evidenced by the return type and purpose of `scale10_round_decimal_decoded`, `scale10_round_decimal_double`, and `scale10_round_decimal_long_double`.

Traceability:
- `gnu/vasnprintf.c` functions:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_double`
  - `scale10_round_decimal_long_double`

#### FR-2: Precision-controlled rounding
The module shall apply rounding based on the supplied integer precision parameter `n`, and different valid `n` inputs shall be able to change the decimal output accordingly.

Traceability:
- parameter `int n` in:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_double`
  - `scale10_round_decimal_long_double`

#### FR-3: Decoded numeric input support
The module shall support formatting from a decoded numeric representation consisting of a decimal-scaling exponent argument and a multiprecision significand input, preserving the functional role of the decoded-input path.

Traceability:
- `scale10_round_decimal_decoded (int e, mpn_t m, void *memory, int n)`

#### FR-4: Native double input support
The module shall support formatting directly from a `double` input and produce a decimal rounded string consistent with the module’s decoded-value behavior for the same mathematical value.

Traceability:
- `scale10_round_decimal_double (double x, int n)`

#### FR-5: Native long double input support
The module shall support formatting directly from a `long double` input and produce a decimal rounded string consistent with the module’s decoded-value behavior for the same mathematical value, subject to the source precision of `long double`.

Traceability:
- `scale10_round_decimal_long_double (long double x, int n)`

#### FR-6: Sign-preserving decimal output
The module shall preserve the sign of the numeric input in the produced decimal string where the underlying value is negative.

Traceability:
- implied by decimal formatting responsibility of all three functions producing character output from signed floating-point inputs:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_double`
  - `scale10_round_decimal_long_double`

#### FR-7: Carry-aware rounding behavior
The module shall correctly propagate rounding carry through the decimal digit sequence when rounding changes one or more more-significant digits.

Traceability:
- functional responsibility of `scale10_round_decimal_decoded`, which performs the core decimal scaling and rounding operation used by the native floating-point wrappers.

#### FR-8: Owned result production
The module shall produce an owned string result rather than requiring the caller to supply the final output buffer content in place of the result.

Traceability:
- `char *` return type of:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_double`
  - `scale10_round_decimal_long_double`

### Key Entities

#### Entity 1: Decoded numeric value
A numeric value represented by:

- a scaling exponent `e`,
- a multiprecision significand `m`,
- an associated temporary memory/context input.

This entity is the core input to the module’s fundamental rounding path. It represents the number to be scaled and converted into rounded decimal text.

Traceability:
- `scale10_round_decimal_decoded (int e, mpn_t m, void *memory, int n)`

#### Entity 2: Precision selector
An integer parameter `n` that determines the decimal rounding target.

Relationship:
- applies uniformly to decoded, `double`, and `long double` input paths.

Traceability:
- parameter `n` in all three functions

#### Entity 3: Native floating-point input
A source numeric value provided as either:

- `double`
- `long double`

Relationship:
- each native value path is a convenience entry that feeds the core decimal scaling-and-rounding behavior.

Traceability:
- `scale10_round_decimal_double`
- `scale10_round_decimal_long_double`

#### Entity 4: Decimal output string
The final product of the module: an allocated character string containing the rounded decimal representation.

Relationship:
- produced by all three functions from either decoded or native floating-point input.

Traceability:
- `char *` return type in all three functions

#### Entity 5: Local helper struct in `gnu/vasnprintf.c`
The source file contains an anonymous struct at lines 426-430. The input evidence does not establish this struct as part of the functional boundary of this module subset. The Rust rewrite may model any necessary internal state, but no externally visible behavior shall depend on exposing this struct as a public entity.

Traceability:
- anonymous `struct` at `gnu/vasnprintf.c:426-430`

## Success Criteria

### SC-1: Output equivalence for decoded inputs
For a conformance test set of decoded numeric inputs and precision values `n`, the Rust implementation shall produce the same decimal strings as the C implementation.

Traceability:
- `scale10_round_decimal_decoded`

### SC-2: Output equivalence for `double`
For a conformance test set of `double` values and precision values `n`, the Rust implementation shall produce the same decimal strings as the C implementation.

Traceability:
- `scale10_round_decimal_double`

### SC-3: Output equivalence for `long double`
For a conformance test set of `long double` values and precision values `n`, the Rust implementation shall produce the same decimal strings as the C implementation on the validation target used for the port.

Traceability:
- `scale10_round_decimal_long_double`

### SC-4: Boundary-rounding correctness
Test cases that exercise decimal rounding boundaries shall match the C implementation, including:
- no-round-up cases,
- round-up cases,
- carry propagation across trailing `9` digits,
- cases where changing `n` changes the result.

Traceability:
- core rounding behavior of `scale10_round_decimal_decoded`
- shared behavior surfaced through `scale10_round_decimal_double`
- shared behavior surfaced through `scale10_round_decimal_long_double`

### SC-5: Sign correctness
For positive and negative nonzero test values supported by the source functions, the sign reflected in the Rust output shall match the C output.

Traceability:
- all three formatting functions

### SC-6: Owned result behavior
The Rust implementation shall return owned string data whose contents remain valid independently of temporary internal computation state.

Traceability:
- `char *` result contract of all three functions

## Assumptions and Porting Notes

- This specification is limited to behavior evidenced by the cited functions and does not define broader formatting semantics used elsewhere in `vasnprintf.c`.
- Where platform-dependent `long double` behavior exists, validation must be performed against the chosen target environment corresponding to the Rust port effort.
- Internal algorithms, temporary storage strategies, and exact Rust type choices are intentionally unspecified unless required to preserve the functional behavior above.