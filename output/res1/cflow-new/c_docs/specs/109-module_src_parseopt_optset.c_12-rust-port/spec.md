# Functional Specification: `module_src_parseopt_optset.c_12`

- **Project**: `cflow-new`
- **Module**: `module_src_parseopt_optset.c_12`
- **Category**: `module_cluster`
- **Source Basis**: `src/parseopt/optset.c`
- **Rust Port Target**: `109-module_src_parseopt_optset.c_12-rust-port`
- **Generation Date**: `2026-06-11`

## 1. Overview

This module provides option-value parsing and assignment behavior for option definitions used by the parseopt subsystem. Its evidenced functionality is the interpretation of textual option arguments as bounded integer values and the application of those values to option-associated storage according to option definition metadata.

The Rust rewrite must preserve the module’s observable behavior as an option-setting component within the parser subsystem, specifically:

- converting signed textual numeric arguments into integer values within caller-provided bounds,
- converting unsigned textual numeric arguments into integer values within caller-provided bounds,
- rejecting invalid or out-of-range numeric inputs,
- supporting option definitions that map parsed values into parser-managed destination fields.

This specification is limited to behavior evidenced by `src/parseopt/optset.c` and the referenced functions and structures.

## 2. Feature Specification

### 2.1 Numeric option argument parsing

The module parses string arguments intended for numeric options.

Supported evidenced behaviors:

- Parse a textual argument as a signed integer.
- Enforce both minimum and maximum bounds for signed results.
- Parse a textual argument as an unsigned integer.
- Enforce an upper bound for unsigned results.
- Return failure when the input cannot be interpreted as a valid number for the expected signedness and bounds.
- On success, provide the parsed integer value to the caller through output storage.

Traceability:
- `get_signed_int` in `src/parseopt/optset.c:27-41`
- `get_unsigned_int` in `src/parseopt/optset.c:43-57`

### 2.2 Option-setting integration

The module participates in setting values for options described by parseopt option definitions. The repeated local references to `struct parseopt` and `struct optdef` in this file indicate that the module operates on parser state plus option definition metadata to assign option values.

The Rust rewrite must preserve the ability for option-setting logic in this module to:

- receive parser context and an option definition,
- interpret the option argument according to the option definition’s expected value category,
- update the target value associated with that option when parsing succeeds,
- reject the setting operation when argument parsing fails.

This requirement is constrained to option-setting behavior evidenced by the file’s dependency on `struct parseopt` and `struct optdef`; it does not assume any new option kinds beyond those supported by the original file.

Traceability:
- `src/parseopt/optset.c`
- local use sites involving `struct parseopt` and `struct optdef` at lines 95, 104, 112, 128, 145, 152, 159

## 3. User Scenarios & Testing

## 3.1 Scenario: Set an option from a valid signed integer argument

A caller in the parseopt subsystem processes an option whose definition expects a signed integer value and provides a textual argument such as `-3` or `42`.

Expected behavior:

- the module accepts the string,
- converts it to a signed integer,
- verifies it lies within the definition’s allowed minimum and maximum,
- stores the parsed result into the option’s destination,
- reports success.

Test coverage:
- valid negative signed value within bounds,
- valid positive signed value within bounds,
- boundary values exactly equal to minimum and maximum.

Traceability:
- `get_signed_int`

## 3.2 Scenario: Reject a signed integer argument outside bounds

A caller provides a textual argument that is numerically valid but smaller than the allowed minimum or larger than the allowed maximum for the option.

Expected behavior:

- the module does not update the target destination with an accepted value,
- the setting operation reports failure.

Test coverage:
- one value below minimum,
- one value above maximum.

Traceability:
- `get_signed_int`

## 3.3 Scenario: Reject a malformed signed integer argument

A caller provides a non-numeric or otherwise invalid textual argument for a signed integer option.

Expected behavior:

- parsing fails,
- the setting operation reports failure.

Test coverage:
- alphabetic string,
- empty string if accepted by surrounding API surface as input,
- mixed numeric/non-numeric text.

Traceability:
- `get_signed_int`

## 3.4 Scenario: Set an option from a valid unsigned integer argument

A caller processes an option whose definition expects an unsigned integer and provides a textual argument such as `0` or `25`.

Expected behavior:

- the module accepts the string,
- converts it to an unsigned integer,
- verifies it does not exceed the allowed maximum,
- stores the parsed result into the option’s destination,
- reports success.

Test coverage:
- zero,
- a normal interior value,
- the exact maximum value.

Traceability:
- `get_unsigned_int`

## 3.5 Scenario: Reject an unsigned integer argument that exceeds the maximum

A caller provides a numerically valid unsigned textual argument greater than the option’s allowed maximum.

Expected behavior:

- parsing is treated as unsuccessful for the option-setting operation,
- the destination is not accepted as updated from that value.

Test coverage:
- one value just above maximum,
- one much larger value.

Traceability:
- `get_unsigned_int`

## 3.6 Scenario: Reject an invalid unsigned integer argument

A caller provides an argument inappropriate for an unsigned integer option.

Expected behavior:

- parsing fails,
- the setting operation reports failure.

Test coverage:
- negative textual value,
- non-numeric string,
- mixed numeric/non-numeric text.

Traceability:
- `get_unsigned_int`

## 3.7 Scenario: Option definition drives assignment behavior

A caller invokes module logic using parser state plus an option definition entry. Different option definitions may target different stored values or expected numeric categories.

Expected behavior:

- the module consults the option definition being processed,
- chooses the correct parsing/assignment behavior for that definition,
- applies the parsed value to the intended parser-managed target only on success.

Test coverage:
- two distinct option definitions with different destinations,
- one signed-valued option and one unsigned-valued option,
- failure in one option does not count as a successful set.

Traceability:
- `src/parseopt/optset.c`
- use of `struct parseopt` and `struct optdef`

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Signed integer parsing with lower and upper bounds

The module shall accept a textual argument and attempt to parse it as a signed integer, succeeding only when the text represents a valid signed integer and the result lies inclusively within the caller-supplied minimum and maximum bounds.

Traceability:
- `get_signed_int` (`src/parseopt/optset.c:27-41`)

#### FR-2: Unsigned integer parsing with upper bound

The module shall accept a textual argument and attempt to parse it as an unsigned integer, succeeding only when the text represents a valid unsigned integer and the result does not exceed the caller-supplied maximum bound.

Traceability:
- `get_unsigned_int` (`src/parseopt/optset.c:43-57`)

#### FR-3: Failure signaling for invalid numeric input

The module shall report failure when numeric parsing cannot be completed because the argument text is not valid for the expected numeric type.

Traceability:
- `get_signed_int` (`src/parseopt/optset.c:27-41`)
- `get_unsigned_int` (`src/parseopt/optset.c:43-57`)

#### FR-4: Failure signaling for out-of-range numeric input

The module shall report failure when a parsed numeric value violates the applicable bounds for the option being set.

Traceability:
- `get_signed_int` (`src/parseopt/optset.c:27-41`)
- `get_unsigned_int` (`src/parseopt/optset.c:43-57`)

#### FR-5: Output of parsed numeric result on success

When numeric parsing succeeds, the module shall provide the parsed value through caller-supplied output storage so that option-setting logic can apply it to the option destination.

Traceability:
- `get_signed_int` (`src/parseopt/optset.c:27-41`)
- `get_unsigned_int` (`src/parseopt/optset.c:43-57`)

#### FR-6: Option-setting behavior driven by option definitions

The module shall perform option-setting in the context of parser state and an option definition, using the option definition to determine how an argument value is interpreted and where the resulting value is applied.

Traceability:
- `src/parseopt/optset.c`
- references to `struct parseopt` and `struct optdef` in file-local logic

#### FR-7: No successful assignment from failed parse

The module shall not treat an option as successfully set when numeric parsing or bound validation fails for the provided argument.

Traceability:
- `get_signed_int` (`src/parseopt/optset.c:27-41`)
- `get_unsigned_int` (`src/parseopt/optset.c:43-57`)
- option-setting logic in `src/parseopt/optset.c`

### 4.2 Key Entities

#### Entity: Parse option context (`struct parseopt`)

Represents parser state supplied to the option-setting logic. This context is used together with an option definition when applying an option value.

Relationships:
- cooperates with `struct optdef` during setting of an option value,
- provides the owning context in which option destinations are updated.

Traceability:
- `src/parseopt/optset.c` local references at lines 95, 104, 112, 128, 145, 152, 159

#### Entity: Option definition (`struct optdef`)

Represents metadata for an individual option handled by the parser subsystem. In this module, it determines the expected value interpretation and the destination to which a successfully parsed value is assigned.

Relationships:
- is processed within a `struct parseopt` context,
- drives which parsing path and assignment behavior the module uses.

Traceability:
- `src/parseopt/optset.c` local references at lines 95, 104, 112, 128, 145, 152, 159

#### Entity: Signed numeric argument value

Represents a parsed integer result produced from textual input when a signed option argument is valid and within bounds.

Relationships:
- originates from a string argument,
- is bounded by minimum and maximum constraints,
- is written to output storage on success.

Traceability:
- `get_signed_int`

#### Entity: Unsigned numeric argument value

Represents a parsed unsigned integer result produced from textual input when an unsigned option argument is valid and within bounds.

Relationships:
- originates from a string argument,
- is bounded by a maximum constraint,
- is written to output storage on success.

Traceability:
- `get_unsigned_int`

## 5. Success Criteria

### 5.1 Behavioral parity

The Rust module reproduces the original module’s evidenced behavior for signed and unsigned numeric option argument parsing and option-setting outcomes in all scenarios described in this specification.

Measured by:
- passing scenario-based tests in Sections 3.1 through 3.7.

### 5.2 Signed parsing correctness

For signed integer parsing, the Rust module:

- accepts valid signed textual numbers within inclusive bounds,
- rejects malformed input,
- rejects values below minimum,
- rejects values above maximum.

Measured by:
- tests covering in-range interior values, exact boundary values, malformed text, below-minimum values, and above-maximum values.

Traceability:
- `get_signed_int`

### 5.3 Unsigned parsing correctness

For unsigned integer parsing, the Rust module:

- accepts valid unsigned textual numbers not exceeding the maximum,
- rejects malformed input,
- rejects negative textual input,
- rejects values greater than the maximum.

Measured by:
- tests covering zero, interior values, exact maximum, negative input, malformed text, and above-maximum values.

Traceability:
- `get_unsigned_int`

### 5.4 Option-definition-driven application

The Rust module correctly applies parsed values according to the supplied option definition and parser context, updating the intended destination only when parsing and validation succeed.

Measured by:
- tests using multiple option definitions with distinct destinations and value categories,
- verification that failed parses do not count as successful sets.

Traceability:
- `src/parseopt/optset.c`
- references to `struct parseopt` and `struct optdef`

### 5.5 Traceable module scope compliance

The Rust rewrite stays within the evidenced functional scope of this module: numeric parsing, bound checking, failure reporting, and option-definition-driven value application.

Measured by:
- review of the Rust port against this specification and the cited source basis,
- absence of required behaviors not evidenced by `src/parseopt/optset.c`.