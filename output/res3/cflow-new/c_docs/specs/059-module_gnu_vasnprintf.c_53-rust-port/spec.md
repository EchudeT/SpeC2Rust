# spec.md

## Title

Functional Specification for `module_gnu_vasnprintf.c_53` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_53`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `059-module_gnu_vasnprintf.c_53-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides internal numeric support used by formatted output logic for floating-point values. Its responsibility is to derive decimal-formatting inputs from binary floating-point numbers and to support correct rounding-sensitive decisions near decimal boundaries.

The Rust rewrite must preserve the functional behavior evidenced by the source module for:

- decoding `double` and `long double` inputs into an internal multi-precision representation with an associated exponent,
- computing the decimal order of magnitude for `double` and `long double` values,
- detecting whether a generated decimal digit sequence lies on a rounding borderline for a requested precision.

This specification covers only the behavior evidenced by the analyzed functions and associated internal data representation.

## Scope

### In Scope

- Internal handling of finite floating-point values as required by decimal formatting support.
- Separate behavior for `double` and `long double`.
- Production of exponent and multi-precision numeric state suitable for downstream formatting logic.
- Decimal magnitude estimation for floating-point inputs.
- Borderline-digit detection used to inform rounding behavior.

### Out of Scope

- General formatted output API design.
- Parsing of format strings.
- Memory model or allocation strategy beyond preserving behavior.
- Public API expansion beyond what is required to support the evidenced module behavior.
- Capabilities not evidenced by the analyzed source, including serialization, FFI, concurrency guarantees, or recovery mechanisms.

## Feature Specification

### Feature 1: Decode binary floating-point into internal decimal-conversion inputs

The module must accept a `double` or `long double` value and derive an internal representation consisting of:

- a returned internal storage object for the numeric payload,
- an output exponent value,
- an output multi-precision descriptor/value holder.

The behavior must preserve the role of the C functions `decode_double` and `decode_long_double`: they prepare the significant numeric content of the input value for later decimal formatting logic. The Rust version must support both input widths separately and preserve their distinct decoding behavior.

The resulting decoded state must be usable by downstream formatting steps to reason about magnitude and decimal digit production without loss of correctness attributable to the port.

### Feature 2: Compute floor base-10 logarithm for floating-point values

The module must compute the decimal order of magnitude for both `double` and `long double` inputs, matching the behavior of `floorlog10` and `floorlog10l`.

For supported inputs, the returned value must equal the integer `k` such that:

- `10^k <= x < 10^(k+1)`

subject to the exact boundary handling required by the source behavior. The Rust port must preserve correct results for values near powers of ten, where implementation mistakes commonly occur.

### Feature 3: Detect decimal rounding borderlines

The module must determine whether a digit sequence at a given precision lies on a rounding borderline, matching the role of `is_borderline`.

Given:

- a decimal digit sequence,
- a precision value,

the module must report whether the represented tail corresponds to the specific borderline condition required by downstream rounding logic. The Rust rewrite must preserve this decision behavior exactly for the inputs supported by the source logic.

## User Scenarios & Testing

### Scenario 1: Formatting logic needs a decoded `double`

A caller in formatted-output processing has a finite `double` value and needs internal state suitable for decimal digit generation.

Expected support:

- the Rust module accepts the value,
- produces decoded numeric state and exponent outputs,
- returns state usable by subsequent formatting logic without changing the represented numeric value.

Testing focus:

- normal positive values,
- negative values if sign is handled externally, ensuring magnitude decoding remains correct,
- exact powers of two,
- values with fractional binary representation,
- smallest and largest finite `double` values relevant to the source behavior.

### Scenario 2: Formatting logic needs a decoded `long double`

A caller processes a `long double` and requires the same kind of internal representation, but with `long double`-specific precision and range behavior.

Expected support:

- decoding remains correct for `long double`,
- exponent and multi-precision outputs reflect the higher-precision source type,
- results remain suitable for downstream decimal formatting.

Testing focus:

- representative `long double` values across its supported range,
- values near type-specific precision limits,
- values whose decimal conversion is sensitive to mantissa width.

### Scenario 3: Determining the decimal exponent for scientific or fixed formatting

A caller needs the base-10 order of magnitude for a floating-point input in order to choose formatting layout and digit placement.

Expected support:

- `double` inputs use `floorlog10`,
- `long double` inputs use `floorlog10l`,
- values near powers of ten return the mathematically correct lower exponent.

Testing focus:

- `1.0`, `10.0`, `100.0`,
- `0.1`, `0.01`, `0.001`,
- values just below and just above powers of ten,
- large and small finite magnitudes.

### Scenario 4: Rounding decision at precision boundary

A caller has produced decimal digits and must determine whether the remainder lies exactly on the borderline that affects rounding.

Expected support:

- the Rust module evaluates the digit string and precision,
- returns the same borderline/non-borderline decision as the C source logic,
- enables downstream formatting to preserve source rounding behavior.

Testing focus:

- digit strings that are clearly non-borderline,
- digit strings that are exact halfway-style borderlines,
- differing precision values over the same digit buffer,
- short and long digit sequences.

## Requirements

### Functional Requirements

#### FR-1: Double decode support
The module shall decode a `double` input into internal numeric state consisting of returned storage plus output exponent and multi-precision data, preserving the behavior of `decode_double` in `gnu/vasnprintf.c`.

Traceability: `decode_double` in `gnu/vasnprintf.c:1094-1169`.

#### FR-2: Long double decode support
The module shall decode a `long double` input into internal numeric state consisting of returned storage plus output exponent and multi-precision data, preserving the behavior of `decode_long_double` in `gnu/vasnprintf.c`.

Traceability: `decode_long_double` in `gnu/vasnprintf.c:1006-1084`.

#### FR-3: Decimal magnitude for `double`
The module shall compute the floor base-10 logarithm for `double` values with correct power-of-ten boundary behavior.

Traceability: `floorlog10` in `gnu/vasnprintf.c:1529-1611`.

#### FR-4: Decimal magnitude for `long double`
The module shall compute the floor base-10 logarithm for `long double` values with correct power-of-ten boundary behavior.

Traceability: `floorlog10l` in `gnu/vasnprintf.c:1438-1520`.

#### FR-5: Borderline detection
The module shall evaluate whether a decimal digit sequence at a given precision satisfies the rounding-borderline condition used by the source formatting logic.

Traceability: `is_borderline` in `gnu/vasnprintf.c:1617-1627`.

#### FR-6: Behavior preservation across represented ranges
For supported finite input values, the module shall preserve source-equivalent decisions and outputs for decode, decimal magnitude, and borderline evaluation so that downstream formatting behavior is not observably changed by the Rust port.

Traceability: `decode_long_double`, `decode_double`, `floorlog10l`, `floorlog10`, `is_borderline` in `gnu/vasnprintf.c`.

### Key Entities

#### Entity 1: Decoded floating-point state
An internal decoded state represents the significant numeric content of a floating-point input together with an associated exponent and multi-precision form. It is the output relationship created by the decode functions and consumed by later formatting logic.

Traceability: `decode_long_double`, `decode_double` in `gnu/vasnprintf.c`.

#### Entity 2: Multi-precision numeric holder
The module uses an internal multi-precision numeric holder (`mpn_t`) as part of decoded floating-point state. It carries the significant value needed for precise decimal conversion steps.

Traceability: `decode_long_double`, `decode_double` in `gnu/vasnprintf.c`.

#### Entity 3: Exponent output
The decode process produces an integer exponent output associated with the decoded numeric content. This exponent participates in later decimal conversion and magnitude handling.

Traceability: `decode_long_double`, `decode_double` in `gnu/vasnprintf.c`.

#### Entity 4: Borderline digit view
A decimal digit sequence plus a precision value forms the input entity for rounding-borderline evaluation.

Traceability: `is_borderline` in `gnu/vasnprintf.c`.

#### Entity 5: Internal anonymous struct
The source file contains an internal anonymous struct used within the module. The Rust port must preserve any behaviorally relevant role it serves, but does not need to preserve C layout unless required by the module’s own logic.

Traceability: anonymous struct in `gnu/vasnprintf.c:426-430`.

## Success Criteria

### SC-1: Decode equivalence for `double`
For a representative conformance test set of finite `double` values, the Rust port produces decoded state that leads to the same downstream decimal-formatting-relevant value and exponent behavior as the C source.

Traceability: FR-1.

### SC-2: Decode equivalence for `long double`
For a representative conformance test set of finite `long double` values, the Rust port produces decoded state that leads to the same downstream decimal-formatting-relevant value and exponent behavior as the C source.

Traceability: FR-2.

### SC-3: Correct decimal exponent results
For test inputs spanning magnitudes above, below, and exactly at powers of ten, the Rust port returns the same floor base-10 logarithm results as the C source for both `double` and `long double`.

Traceability: FR-3, FR-4.

### SC-4: Correct borderline classification
For test cases covering borderline and non-borderline digit sequences at varying precisions, the Rust port returns the same classification as the C source.

Traceability: FR-5.

### SC-5: No observable behavior drift in covered module responsibilities
When integrated into the surrounding formatted-output logic that depends on these functions, the Rust port preserves source-equivalent decisions for decimal magnitude estimation and rounding-sensitive boundary handling for the covered functionality.

Traceability: FR-6.

## Acceptance Notes

- Source equivalence is defined behaviorally, not by reproducing C memory layout or implementation technique.
- Tests should emphasize boundary values, especially around powers of ten and rounding half-boundaries, because those are the evidenced responsibilities of this module.
- Any Rust-internal refactoring is acceptable only if all requirements and success criteria above remain satisfied.