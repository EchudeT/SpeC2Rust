# spec.md

## Title

Functional Specification: `module_src_parseopt_optset.c_12` Rust Port

## Overview

This module provides option-value parsing and assignment support for the parse-option subsystem in `cflow-new`. Its evidenced scope is conversion of textual option arguments into bounded integer values and integration of those conversions with option-definition-driven setters represented through `parseopt` and `optdef` relationships in `src/parseopt/optset.c`.

The Rust rewrite must preserve the module’s observable behavior within that scope:

- parse signed integer option arguments from strings,
- parse unsigned integer option arguments from strings,
- reject values outside declared bounds,
- report conversion failure versus success in a way equivalent to the C module’s current role in option setting,
- support use from option-definition-based parsing flows tied to `parseopt` and `optdef`.

This specification is intentionally limited to functionality directly evidenced by:

- `src/parseopt/optset.c`
- `get_signed_int`
- `get_unsigned_int`
- the `parseopt` / `optdef` pairings referenced in that file

## Feature Specification

### Summary

The module is responsible for validating and converting option argument text into integer values suitable for assignment by the parse-option system. It acts as a boundary between raw command-line argument text and typed option values.

### Functional Scope

The Rust version must implement the following module behavior:

1. **Signed integer parsing**
   - Accept a textual argument intended to represent a signed integer.
   - Convert it to an integer value.
   - Enforce caller-supplied inclusive minimum and maximum bounds.
   - Indicate failure when the text is not a valid integer representation or when the converted value is outside bounds.
   - Provide the converted value on success.

2. **Unsigned integer parsing**
   - Accept a textual argument intended to represent an unsigned integer.
   - Enforce a caller-supplied inclusive maximum bound.
   - Indicate failure when the text is not a valid unsigned integer representation or when the converted value exceeds the bound.

3. **Option-setting support**
   - Support use of the above conversions in the option-setting logic associated with option definitions in the parse-option subsystem.
   - Preserve the distinction between:
     - option argument accepted and converted,
     - option argument rejected due to invalid numeric form,
     - option argument rejected due to range violation.

### Out of Scope

The Rust port specification does not require any functionality not evidenced in the provided module analysis, including:

- new option kinds,
- new public APIs beyond what is needed to preserve module behavior,
- formatting or localization behavior not evidenced,
- thread-safety guarantees,
- persistence, serialization, or FFI behavior.

## User Scenarios & Testing

### Scenario 1: Signed numeric option within range

A caller in the parse-option subsystem processes an option definition whose argument must be a signed integer with known bounds. The user supplies a valid numeric string within those bounds.

Expected behavior:

- the module accepts the string,
- converts it to the signed integer value,
- returns success to the option-setting flow,
- makes the converted value available for assignment.

Test coverage:

- lower bound accepted,
- upper bound accepted,
- ordinary in-range value accepted,
- negative in-range value accepted when the minimum allows it.

### Scenario 2: Signed numeric option outside range

A caller processes a signed integer option, but the user supplies a value below the minimum or above the maximum.

Expected behavior:

- the module rejects the value,
- no successful converted result is produced,
- the option-setting flow can treat the argument as invalid.

Test coverage:

- one case below minimum,
- one case above maximum,
- exact just-outside-boundary values.

### Scenario 3: Signed numeric option with invalid text

A caller processes a signed integer option, but the user supplies text that is not a valid integer.

Expected behavior:

- the module rejects the argument as invalid,
- no successful converted result is produced.

Test coverage:

- non-numeric text,
- mixed numeric and non-numeric text,
- empty string if accepted as input by the surrounding parser.

### Scenario 4: Unsigned numeric option within range

A caller processes an option definition whose argument must be an unsigned integer with a defined maximum. The user supplies a valid unsigned numeric string not exceeding that maximum.

Expected behavior:

- the module accepts the string,
- converts it to the unsigned integer value,
- returns success to the option-setting flow.

Test coverage:

- zero accepted,
- maximum accepted,
- ordinary in-range value accepted.

### Scenario 5: Unsigned numeric option exceeding range

A caller processes an unsigned integer option, but the user supplies a value greater than the allowed maximum.

Expected behavior:

- the module rejects the value,
- the option-setting flow can treat the argument as invalid.

Test coverage:

- exact maximum plus one,
- much larger out-of-range value.

### Scenario 6: Unsigned numeric option with invalid text

A caller processes an unsigned integer option, but the user supplies invalid numeric text.

Expected behavior:

- the module rejects the argument,
- no successful converted result is produced.

Test coverage:

- alphabetic text,
- signed-form text if not valid for the unsigned conversion behavior,
- mixed text.

### Scenario 7: Integration with option definitions

The parse-option subsystem invokes the module as part of applying an `optdef` to parser state represented by `parseopt`.

Expected behavior:

- conversion behavior is usable from option-definition-driven setting paths in this module,
- accepted values and rejected values produce outcomes consistent with the signed/unsigned conversion rules above.

Test coverage:

- at least one signed-setting path tied to an option definition,
- at least one unsigned-setting path tied to an option definition,
- failure propagation from conversion logic to the setting result.

## Requirements

### Functional Requirements

#### FR-1: Signed integer conversion
Traceability: `src/parseopt/optset.c`, `get_signed_int`

The module shall convert a textual argument to a signed integer result when the text is a valid signed integer representation and the converted value lies within the caller-provided inclusive minimum and maximum bounds.

#### FR-2: Signed integer rejection on invalid format
Traceability: `src/parseopt/optset.c`, `get_signed_int`

The module shall reject a signed integer argument when the text is not a valid signed integer representation.

#### FR-3: Signed integer rejection on range violation
Traceability: `src/parseopt/optset.c`, `get_signed_int`

The module shall reject a signed integer argument when the converted value is less than the provided minimum or greater than the provided maximum.

#### FR-4: Unsigned integer conversion
Traceability: `src/parseopt/optset.c`, `get_unsigned_int`

The module shall convert a textual argument to an unsigned integer result when the text is a valid unsigned integer representation and the converted value does not exceed the caller-provided inclusive maximum.

#### FR-5: Unsigned integer rejection on invalid format
Traceability: `src/parseopt/optset.c`, `get_unsigned_int`

The module shall reject an unsigned integer argument when the text is not a valid unsigned integer representation.

#### FR-6: Unsigned integer rejection on range violation
Traceability: `src/parseopt/optset.c`, `get_unsigned_int`

The module shall reject an unsigned integer argument when the converted value exceeds the provided maximum.

#### FR-7: Success/failure signaling for option-setting use
Traceability: `src/parseopt/optset.c`, `get_signed_int`, `get_unsigned_int`

The module shall expose conversion outcomes in a form that distinguishes success from failure so the surrounding option-setting logic can determine whether an option argument may be applied.

#### FR-8: Compatibility with parse-option definition flows
Traceability: `src/parseopt/optset.c`, `struct parseopt`, `struct optdef`

The module shall preserve behavior needed for conversion and setting logic that operates in the context of parser state (`parseopt`) and option definitions (`optdef`) within this file.

### Key Entities

#### `parseopt`
Traceability: `src/parseopt/optset.c`

Represents parser state in the parse-option subsystem. In this module, it participates as the contextual state within which option-setting actions occur.

#### `optdef`
Traceability: `src/parseopt/optset.c`

Represents an option definition that determines how an option argument should be interpreted and applied.

#### Relationship: `parseopt` and `optdef`
Traceability: `src/parseopt/optset.c`

This module operates where option definitions are applied against parser state. Integer conversion supports those applications by validating and translating raw argument text into typed numeric values suitable for the option definition being processed.

#### Numeric conversion result
Traceability: `get_signed_int`, `get_unsigned_int`

A successful conversion yields a typed integer value. An unsuccessful conversion yields a failure outcome caused by invalid text or a range violation.

## Success Criteria

1. **Signed in-range acceptance**
   - For test inputs representing valid signed integers at the minimum, maximum, and interior of the allowed range, the Rust module returns success and the same numeric value expected by the C behavior.
   - Traceability: `get_signed_int`

2. **Signed invalid-input rejection**
   - For invalid signed integer text inputs, the Rust module returns failure and does not produce a successful result.

3. **Signed out-of-range rejection**
   - For signed integer values below minimum or above maximum, the Rust module returns failure and does not accept the argument.

4. **Unsigned in-range acceptance**
   - For test inputs representing valid unsigned integers from zero through the allowed maximum, the Rust module returns success and the same numeric value expected by the C behavior.
   - Traceability: `get_unsigned_int`

5. **Unsigned invalid-input rejection**
   - For invalid unsigned integer text inputs, the Rust module returns failure and does not produce a successful result.

6. **Unsigned out-of-range rejection**
   - For unsigned integer values greater than the allowed maximum, the Rust module returns failure and does not accept the argument.

7. **Option-setting integration preservation**
   - Module-level tests or integration tests demonstrate that conversion outcomes can be consumed by option-setting paths associated with `parseopt` and `optdef` without changing the accepted/rejected behavior of numeric option arguments.
   - Traceability: `src/parseopt/optset.c`, `struct parseopt`, `struct optdef`