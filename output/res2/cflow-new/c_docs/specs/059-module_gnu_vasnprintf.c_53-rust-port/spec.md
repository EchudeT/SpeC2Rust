# spec.md

## Title

Functional Specification for `module_gnu_vasnprintf.c_53` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_53`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Target Rust branch: `059-module_gnu_vasnprintf.c_53-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides internal floating-point support behavior used by `vasnprintf` formatting logic in `gnu/vasnprintf.c`. Its evidenced responsibilities are:

- decoding `double` and `long double` values into an internal multi-precision-compatible representation,
- computing the base-10 order of magnitude for finite floating-point values,
- detecting a borderline decimal-digit condition used by later formatting decisions.

The Rust rewrite must preserve the same observable behavior for these responsibilities so that higher-level numeric formatting in the surrounding module continues to behave equivalently.

## Scope

### In Scope

The Rust version must implement the functional behavior evidenced by the analyzed source for:

- conversion of `double` and `long double` inputs into decoded internal form with an associated exponent,
- support for downstream decimal formatting through a produced multi-precision numeric payload,
- calculation of floor base-10 logarithms for `double` and `long double`,
- detection of whether a decimal digit sequence is in the module’s borderline condition for a given precision.

### Out of Scope

The following are not required by this specification because they are not evidenced by the analyzed module slice:

- introduction of new public APIs beyond what is needed to preserve module behavior,
- changes to formatting semantics outside the responsibilities listed above,
- concurrency guarantees,
- serialization or persistence,
- FFI design,
- error recovery features beyond source-equivalent behavior.

## Feature Specification

### Feature 1: Floating-Point Decoding for Formatting

The module must accept binary floating-point inputs of type `double` and `long double` and transform them into an internal decoded representation suitable for later decimal formatting steps.

This decoding behavior must:

- derive a normalized numeric payload compatible with the module’s multi-precision number handling,
- produce an exponent value associated with the decoded payload,
- preserve distinctions required by the formatting logic for `double` versus `long double`,
- behave consistently for the range of finite values accepted by the source module.

The Rust version must implement separate decoding behavior for `double` and `long double`-equivalent inputs when the source semantics distinguish them.

### Feature 2: Base-10 Magnitude Estimation

The module must compute the integer floor of the base-10 logarithm of a floating-point value for both `double` and `long double` cases.

This behavior must support formatting logic that needs to determine decimal magnitude, including values:

- greater than or equal to 1,
- between 0 and 1,
- near powers of ten where exact magnitude classification affects formatted output.

The Rust version must preserve source-equivalent magnitude results for the same finite inputs.

### Feature 3: Borderline Decimal Digit Detection

The module must determine whether a decimal digit string is in a specific borderline condition relative to a requested precision.

This behavior is used to support later formatting decisions where digit sequences at or near a rounding boundary require special treatment.

The Rust version must:

- inspect a digit sequence represented as characters,
- evaluate it relative to a provided precision,
- return a boolean-equivalent result matching source behavior.

## User Scenarios & Testing

### Scenario 1: Formatting logic decodes a `double` before decimal conversion

A caller within the formatting subsystem receives a finite `double` and needs its internal decoded form to continue decimal digit generation.

Expected support:

- the value is decoded,
- an exponent is produced,
- the internal numeric payload is made available for subsequent formatting steps.

#### Acceptance tests

- For representative positive finite `double` values, decoding yields a usable payload and exponent.
- For representative negative finite `double` values, decoding preserves the numeric content needed by later formatting logic.
- For values near binary exponent boundaries, the produced decoded form remains suitable for consistent downstream formatting.

### Scenario 2: Formatting logic decodes a `long double`

A caller within the formatting subsystem processes a finite `long double` that requires the module’s higher-precision decoding path.

Expected support:

- decoding follows the `long double` path rather than collapsing behavior to `double`,
- the resulting payload and exponent reflect the source module’s `long double` handling.

#### Acceptance tests

- For representative finite `long double` inputs, decoding succeeds with source-equivalent exponent classification.
- For `long double` values whose behavior differs from `double` precision, the Rust version preserves the source distinction.

### Scenario 3: Decimal magnitude selection during formatting

The formatting subsystem must choose formatting behavior based on the decimal order of magnitude of a finite floating-point number.

Expected support:

- the module returns the floor base-10 logarithm for `double` and `long double`,
- powers of ten and values adjacent to them are classified consistently with the source behavior.

#### Acceptance tests

- Exact decimal powers such as `1`, `10`, `100`, `0.1`, and `0.01` produce the expected floor base-10 logarithm.
- Values just below and just above powers of ten are classified correctly.
- The `double` and `long double` paths each match source behavior for their respective type.

### Scenario 4: Borderline digit handling during rounding-sensitive formatting

The formatting subsystem has a decimal digit sequence and precision and needs to know whether the sequence lies in the module’s borderline condition.

Expected support:

- the digit sequence is examined relative to the precision,
- the result correctly signals whether special downstream handling is required.

#### Acceptance tests

- Inputs that are clearly non-borderline return false.
- Inputs matching the source module’s borderline pattern return true.
- Precision-dependent cases change result consistently when precision changes.

## Requirements

### Functional Requirements

#### FR-1: Decode `double` values into internal formatting representation

The module shall decode a finite `double` input into:

- an internal numeric payload compatible with the module’s multi-precision formatting flow, and
- an associated exponent output.

**Traceability:** `decode_double` in `gnu/vasnprintf.c`

#### FR-2: Decode `long double` values into internal formatting representation

The module shall decode a finite `long double` input into:

- an internal numeric payload compatible with the module’s multi-precision formatting flow, and
- an associated exponent output.

**Traceability:** `decode_long_double` in `gnu/vasnprintf.c`

#### FR-3: Preserve type-specific decoding behavior

Where source behavior distinguishes `double` decoding from `long double` decoding, the Rust version shall preserve that distinction.

**Traceability:** `decode_double`, `decode_long_double` in `gnu/vasnprintf.c`

#### FR-4: Compute floor base-10 logarithm for `double`

The module shall compute the integer `floor(log10(x))` classification used by formatting logic for finite `double` inputs.

**Traceability:** `floorlog10` in `gnu/vasnprintf.c`

#### FR-5: Compute floor base-10 logarithm for `long double`

The module shall compute the integer `floor(log10(x))` classification used by formatting logic for finite `long double` inputs.

**Traceability:** `floorlog10l` in `gnu/vasnprintf.c`

#### FR-6: Classify values consistently around decimal power boundaries

The module shall return source-equivalent decimal magnitude results for values at, below, and above powers of ten, including subunit positive values.

**Traceability:** `floorlog10`, `floorlog10l` in `gnu/vasnprintf.c`

#### FR-7: Detect borderline decimal digit sequences

The module shall evaluate a decimal digit character sequence against a requested precision and return whether it is in the source-defined borderline condition.

**Traceability:** `is_borderline` in `gnu/vasnprintf.c`

#### FR-8: Support downstream formatting decisions with source-equivalent behavior

The module’s decoding, magnitude classification, and borderline detection shall together provide behavior compatible with the surrounding `vasnprintf` floating-point formatting flow.

**Traceability:** `decode_double`, `decode_long_double`, `floorlog10`, `floorlog10l`, `is_borderline` in `gnu/vasnprintf.c`

### Key Entities

#### Entity 1: Decoded floating-point payload

A decoded floating-point payload represents the internal numeric form produced from a binary floating-point input for use by later decimal-formatting logic. It is paired with an exponent output and is compatible with the module’s multi-precision number handling.

**Relationships:**

- produced by `double` and `long double` decoding,
- consumed by later formatting stages in the surrounding module.

**Traceability:** `decode_double`, `decode_long_double` in `gnu/vasnprintf.c`

#### Entity 2: Decimal exponent classification

A decimal exponent classification is the integer floor base-10 logarithm associated with a finite floating-point input. It informs formatting decisions about decimal magnitude.

**Relationships:**

- computed directly from floating-point inputs,
- used by formatting logic to choose or guide decimal presentation.

**Traceability:** `floorlog10`, `floorlog10l` in `gnu/vasnprintf.c`

#### Entity 3: Borderline digit condition

A borderline digit condition is a boolean-equivalent classification of a decimal digit string relative to a precision threshold.

**Relationships:**

- derived from a digit sequence and precision,
- influences later rounding-sensitive formatting decisions.

**Traceability:** `is_borderline` in `gnu/vasnprintf.c`

#### Entity 4: Internal helper state structure

The source module contains an internal anonymous structure used within the `gnu/vasnprintf.c` implementation context. The Rust port must preserve its functional role only to the extent necessary to maintain source-equivalent module behavior.

**Relationships:**

- internal to the module implementation,
- not specified as a public interface.

**Traceability:** anonymous struct at `gnu/vasnprintf.c:426-430`

## Success Criteria

### Behavioral Equivalence

- The Rust port produces source-equivalent decoded payload/exponent behavior for representative finite `double` inputs.
- The Rust port produces source-equivalent decoded payload/exponent behavior for representative finite `long double` inputs.
- The Rust port returns source-equivalent `floor(log10(x))` results for representative finite inputs across:
  - values greater than 1,
  - values between 0 and 1,
  - exact powers of ten,
  - values immediately adjacent to powers of ten.
- The Rust port returns source-equivalent borderline detection results for representative digit-string and precision combinations.

### Integration Fitness

- The rewritten module can support the surrounding `vasnprintf` floating-point formatting flow without requiring behavioral expansion beyond the source module responsibilities.
- No evidenced responsibility from the analyzed functions is omitted in the Rust port.

### Testability

The Rust implementation is acceptable when automated tests demonstrate all of the following:

1. `double` decoding cases execute successfully and preserve source-equivalent exponent classification.
2. `long double` decoding cases execute successfully and preserve source-equivalent exponent classification.
3. `floorlog10` and `floorlog10l` behavior matches the source on a boundary-focused test set.
4. borderline detection matches the source on precision-sensitive test cases.

## Assumptions and Constraints

- This specification covers only the evidenced functional slice of `gnu/vasnprintf.c` named in the analysis results.
- The module is specified as an internal formatting support component, not as a standalone user-facing API.
- The Rust port may adapt internal representation details, but it must not change the functional behavior defined in this document.