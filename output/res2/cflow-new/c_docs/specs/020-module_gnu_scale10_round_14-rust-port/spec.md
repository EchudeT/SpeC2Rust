# spec.md

## Title

Rust Functional Specification for `module_gnu_scale10_round_14`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_scale10_round_14`
- **Category**: `module_cluster`
- **Source basis**: `gnu/vasnprintf.c`
- **Rust branch**: `020-module_gnu_scale10_round_14-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides decimal scaling and rounding support for floating-point values during formatted output generation. Its functional role is to convert a finite floating-point input into a decimal character sequence that reflects the value rounded to a requested decimal precision.

The analyzed functionality is centered on three routines:

- conversion and rounding from a decoded decimal representation,
- adaptation of that behavior for `long double`,
- adaptation of that behavior for `double`.

The Rust rewrite must preserve the observed behavior boundaries of this module: producing a decimal string result from `double` and `long double` inputs by scaling in base 10 and rounding to a caller-specified count.

## Scope

### In Scope

- Decimal scaling and rounding of floating-point values for formatted output.
- Support for inputs represented as:
  - a decoded mantissa-plus-exponent form,
  - `double`,
  - `long double`.
- Returning a character string containing the rounded decimal result.
- Behavior driven by a requested decimal count parameter `n`.

### Out of Scope

- General-purpose number parsing.
- Format-string parsing or full `printf` behavior.
- Locale-specific formatting rules.
- Public API design beyond what is needed to preserve the module’s existing functional role.
- Non-decimal radix formatting.
- New capabilities not evidenced by the source module.

## Feature Specification

### Feature: Decimal scaling and rounding for formatted numeric output

The module shall produce a decimal character representation of a floating-point value after applying base-10 scaling and rounding according to a requested decimal precision/count.

This feature exists in three functional layers:

1. **Decoded-decimal rounding layer**
   Accepts:
   - a decimal exponent-like scale input,
   - a decoded mantissa representation,
   - a precision/count parameter,
   - caller-provided memory context.

   Produces:
   - a newly formed decimal character sequence representing the scaled and rounded value.

2. **`long double` adaptation layer**
   Accepts a `long double` value and requested decimal count, converts or decodes the value into the internal form required by the rounding layer, and returns the rounded decimal character sequence.

3. **`double` adaptation layer**
   Accepts a `double` value and requested decimal count, converts or decodes the value into the internal form required by the rounding layer, and returns the rounded decimal character sequence.

### Behavioral Notes

- The result is a string result suitable for use in formatted numeric output assembly.
- The decimal precision/count argument controls the rounding boundary and the shape of the returned decimal digits.
- The `double` and `long double` entry points must behave consistently with the decoded-decimal rounding logic they delegate to.
- The module’s responsibility is limited to decimal conversion/rounding behavior evidenced by the listed functions; it does not define complete surrounding formatting semantics.

## User Scenarios & Testing

### Scenario 1: Format a `double` with a requested decimal count

A formatting component needs a decimal string for a `double` value rounded to a specified count. It invokes the Rust replacement for the `double` entry point and incorporates the returned string into a larger formatted output buffer.

**Expected support**
- Accept a `double` input and integer count.
- Return a decimal string reflecting base-10 rounding to that count.
- Produce output usable as a formatting intermediate.

**Testing focus**
- Representative positive and negative finite values.
- Values requiring no rounding change.
- Values where rounding changes one or more trailing digits.
- Values where rounding propagates across multiple digits.

### Scenario 2: Format a `long double` with the same decimal-rounding rules

A formatting component needs equivalent decimal rounding behavior for a `long double` value. It invokes the Rust replacement for the `long double` entry point and uses the returned string in the same formatting pipeline.

**Expected support**
- Accept a `long double`-equivalent Rust representation used by the port.
- Apply the same decimal rounding model as the decoded-decimal path.
- Return a decimal string result.

**Testing focus**
- Values that are representable in both `double` and `long double`, to verify consistent results when precision permits.
- Values whose decimal expansion exercises the rounding boundary.
- Values near decimal carry boundaries.

### Scenario 3: Internal formatting path uses decoded mantissa/exponent input

An internal formatting path has already decoded a floating-point value into mantissa-plus-scale form and needs only the scaling/rounding step. It invokes the Rust equivalent of the decoded-decimal routine to obtain the final decimal string.

**Expected support**
- Accept decoded numeric components and requested count.
- Produce the same decimal string that the floating-point wrapper paths would produce for equivalent values.
- Preserve correct rounding behavior independent of whether the source was `double`, `long double`, or pre-decoded form.

**Testing focus**
- Known decoded inputs paired with expected decimal strings.
- Inputs whose rounding result changes the effective leading digits.
- Inputs requiring carry propagation into a new most-significant digit.

### Scenario 4: Integration into formatted output construction

The surrounding formatted-output subsystem combines this module’s returned decimal string with signs, prefixes, padding, or width handling managed elsewhere.

**Expected support**
- Return plain decimal content that can be integrated without further reinterpretation by this module.
- Avoid requiring the caller to reconstruct rounding decisions already made here.

**Testing focus**
- Verify that the returned string is directly consumable by the surrounding formatter.
- Verify that no additional digit correction is needed downstream.

## Requirements

### Functional Requirements

#### FR-1: Decimal rounding from decoded representation
The Rust module shall implement the functional behavior of converting a decoded decimal representation into a decimal character string rounded according to the requested count parameter.

**Traceability**: `scale10_round_decimal_decoded` in `gnu/vasnprintf.c`

#### FR-2: `double` input support
The Rust module shall implement a `double`-input path that produces a decimal character string by applying the module’s decimal scaling and rounding behavior to the supplied `double` value and requested count.

**Traceability**: `scale10_round_decimal_double` in `gnu/vasnprintf.c`

#### FR-3: `long double` input support
The Rust module shall implement a `long double`-input path that produces a decimal character string by applying the module’s decimal scaling and rounding behavior to the supplied `long double` value and requested count.

**Traceability**: `scale10_round_decimal_long_double` in `gnu/vasnprintf.c`

#### FR-4: Consistent rounding model across entry paths
The Rust module shall ensure that the `double` and `long double` paths use the same effective decimal rounding behavior as the decoded-decimal path for equivalent numeric inputs.

**Traceability**: relationship among `scale10_round_decimal_decoded`, `scale10_round_decimal_long_double`, and `scale10_round_decimal_double` in `gnu/vasnprintf.c`

#### FR-5: Precision/count-driven output
The Rust module shall make the returned decimal string depend on the caller-supplied count parameter `n`, such that changing `n` changes the rounding target and may change the output digits.

**Traceability**: parameter `n` in all three listed functions in `gnu/vasnprintf.c`

#### FR-6: Returned result as decimal character sequence
The Rust module shall return the conversion result as a character string representing the rounded decimal output, suitable for use by a higher-level formatted-output routine.

**Traceability**: `char *` return type of all three listed functions in `gnu/vasnprintf.c`

### Key Entities

#### Decimal rounded output string
A character sequence containing the module’s final decimal result after base-10 scaling and rounding.

**Role**
- Primary output of all three functional entry points.

**Traceability**
- `char *` return values of:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_long_double`
  - `scale10_round_decimal_double`

#### Decoded decimal input
An internal numeric representation consisting of:
- an integer scale/exponent input,
- a mantissa representation (`mpn_t` in the C source),
- a requested count.

This entity supplies the numeric state consumed by the core rounding routine.

**Role**
- Canonical internal form for the core scaling/rounding behavior.

**Traceability**
- Parameters of `scale10_round_decimal_decoded` in `gnu/vasnprintf.c`

#### Floating-point source values
Binary floating-point inputs supplied as `double` or `long double`.

**Role**
- External numeric source forms that must be adapted to the decoded-decimal rounding flow.

**Traceability**
- Parameters of:
  - `scale10_round_decimal_double`
  - `scale10_round_decimal_long_double`

#### Anonymous internal struct
An internal structure exists in the source module and may participate in surrounding formatting logic. For this module specification, it is only recognized as an existing source-level internal type and not as a required public Rust entity unless needed to preserve the listed functions’ behavior.

**Role**
- Internal supporting type only, with no evidenced standalone behavior requirement in this module slice.

**Traceability**
- Anonymous `struct` at `gnu/vasnprintf.c:426-430`

### Entity Relationships

- The `double` and `long double` source values are adapted into the decoded-decimal processing flow.
- The decoded decimal input is transformed into the decimal rounded output string.
- The output string is then consumed by surrounding formatted-output logic outside this module’s scope.

## Success Criteria

### SC-1: Correct decoded-input rounding behavior
For a maintained set of decoded mantissa/scale test cases derived from source-compatible behavior, the Rust implementation returns the same decimal strings as the C module for the same count parameter.

**Traceability**: `scale10_round_decimal_decoded`

### SC-2: Correct `double` path behavior
For representative finite `double` test inputs and count values, the Rust implementation returns the same decimal strings as the C module.

**Traceability**: `scale10_round_decimal_double`

### SC-3: Correct `long double` path behavior
For representative finite `long double` test inputs and count values, the Rust implementation returns the same decimal strings as the C module.

**Traceability**: `scale10_round_decimal_long_double`

### SC-4: Cross-path consistency
For values that can be exercised through both a floating-point entry path and an equivalent decoded-input path, the Rust implementation produces matching decimal strings.

**Traceability**: interaction of all three listed functions

### SC-5: Precision sensitivity
Tests that vary only the count parameter `n` shall demonstrate that the Rust implementation changes output exactly where the C module’s rounding behavior changes output.

**Traceability**: parameter `n` in all three listed functions

### SC-6: Formatter-ready output
Integration tests with the surrounding formatting pipeline shall confirm that the returned Rust string output can replace the C module’s result without requiring downstream correction of decimal rounding decisions.

**Traceability**: all three listed functions as formatted-output helpers in `gnu/vasnprintf.c`

## Non-Goals

The Rust rewrite of this module is not required by this specification to provide:

- a new public formatting API,
- locale-aware decimal formatting,
- arbitrary-precision decimal arithmetic beyond preserving the source behavior,
- handling guarantees for cases not evidenced by the analyzed functions,
- independent exposure of internal helper types.

## Acceptance Notes

- Behavioral equivalence to the source module is the acceptance standard.
- Where exact behavior is ambiguous from signature-only evidence, the Rust implementation must follow the observable behavior of the source file in tests rather than introducing new semantics.
- Any Rust-facing interface adjustments needed by the port are acceptable only if they preserve the functional behavior specified above.