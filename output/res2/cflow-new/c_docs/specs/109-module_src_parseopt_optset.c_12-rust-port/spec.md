# spec.md

## Overview

This module defines option-value parsing behavior for a subset of command-line option setters in `cflow-new`. The analyzed functionality is limited to converting textual option arguments into validated integer values and applying those conversions within option-definition handling represented by `parseopt` and `optdef` structures in `src/parseopt/optset.c`.

The Rust rewrite must preserve the observed functional boundary:

- parse a signed integer argument string within caller-supplied bounds;
- parse an unsigned integer argument string within a caller-supplied maximum;
- report success or failure of conversion;
- return the parsed numeric value only when the input is valid for the requested numeric domain and range;
- support use by option-setting logic associated with `parseopt` / `optdef` records.

No broader option-parser behavior is specified here beyond what is evidenced by this module analysis.

---

## Feature Specification

### Feature Summary

The module provides validated integer conversion for option arguments used by option-setting logic.

Two conversion behaviors are evidenced:

1. **Signed integer option value parsing**
   - Accept a textual argument.
   - Convert it to a signed integer value.
   - Enforce both lower and upper bounds supplied by the caller.
   - Return a status indicating whether conversion succeeded.
   - On success, provide the parsed value to the caller.

2. **Unsigned integer option value parsing**
   - Convert it to an unsigned integer value.
   - Enforce a caller-supplied maximum.

### In-Scope Behavior

The Rust version must implement behavior equivalent to the module’s evidenced role in `src/parseopt/optset.c`:

- validation of numeric option arguments before they are applied;
- distinction between signed and unsigned numeric domains;
- range checking using limits supplied by the option-setting caller;
- compatibility with option-definition driven use through `parseopt` and `optdef`.

### Out-of-Scope

The following are not evidenced by the provided analysis and must not be added to the specification:

- definition of a new public CLI API;
- support for non-integer numeric formats;
- persistence, serialization, or config-file parsing;
- recovery or correction of invalid values beyond reporting failure;
- thread-safety guarantees;
- extended diagnostics beyond success/failure behavior.

---

## User Scenarios & Testing

### Scenario 1: Set an option that requires a bounded signed integer

A caller handling an option definition needs to accept a numeric argument such as a level, offset, or limit that may be negative or positive. The caller passes the argument text and an allowed minimum and maximum into the module.

**Expected behavior**
- If the text represents a valid signed integer within the allowed range, parsing succeeds and returns that value.
- If the text is not a valid signed integer, parsing fails.
- If the value is below the minimum or above the maximum, parsing fails.

**Test coverage**
- valid negative value within bounds;
- valid zero value within bounds;
- valid positive value within bounds;
- value just below minimum;
- value just above maximum;
- malformed text;
- empty text, if reachable through caller input.

### Scenario 2: Set an option that requires a bounded unsigned integer

A caller handling an option definition needs to accept a non-negative numeric argument. The caller passes the argument text and the maximum allowed value into the module.

**Expected behavior**
- If the text represents a valid unsigned integer not exceeding the maximum, parsing succeeds and returns that value.
- If the text is malformed, negative, or exceeds the maximum, parsing fails.

**Test coverage**
- zero;
- positive value below maximum;
- value exactly equal to maximum;
- value greater than maximum;
- negative textual input;
- malformed text.

### Scenario 3: Option-setting logic stores the parsed result only on success

An option handler associated with `parseopt` / `optdef` uses this module to validate an argument before applying it to the option target.

**Expected behavior**
- On successful parsing, the handler can consume the returned numeric value.
- On failed parsing, the handler receives a failure status and must not treat the parse result as valid.

**Test coverage**
- verify success status and returned value on valid input;
- verify failure status on invalid input;
- verify no false success on out-of-range input.

### Scenario 4: Different option definitions can enforce different numeric ranges

Multiple option definitions may rely on the same conversion behavior but with different accepted ranges.

**Expected behavior**
- The same signed parser respects per-call minimum and maximum values.
- The same unsigned parser respects per-call maximum values.

**Test coverage**
- same numeric string accepted for one range and rejected for another;
- same unsigned numeric string accepted under one maximum and rejected under a smaller maximum.

---

## Requirements

### Functional Requirements

#### FR-1 Signed integer parsing
The module shall accept an argument string and attempt to parse it as a signed integer value.
**Traceability:** `get_signed_int` in `src/parseopt/optset.c:27-41`

#### FR-2 Signed integer bounds enforcement
The module shall treat signed parsing as successful only when the parsed value is within the caller-provided inclusive minimum and maximum bounds.
**Traceability:** `get_signed_int` in `src/parseopt/optset.c:27-41`

#### FR-3 Signed parse result reporting
The module shall return a success/failure status for signed integer parsing and provide the parsed value only for successful conversions.
**Traceability:** `get_signed_int` in `src/parseopt/optset.c:27-41`

#### FR-4 Unsigned integer parsing
The module shall accept an argument string and attempt to parse it as an unsigned integer value.
**Traceability:** `get_unsigned_int` in `src/parseopt/optset.c:43-57`

#### FR-5 Unsigned maximum enforcement
The module shall treat unsigned parsing as successful only when the parsed value does not exceed the caller-provided maximum.
**Traceability:** `get_unsigned_int` in `src/parseopt/optset.c:43-57`

#### FR-6 Unsigned parse result reporting
The module shall return a success/failure status for unsigned integer parsing and provide the parsed value only for successful conversions.
**Traceability:** `get_unsigned_int` in `src/parseopt/optset.c:43-57`

#### FR-7 Option-setting integration
The module shall support use by option-setting logic driven by `parseopt` and `optdef` records in the same source module, so that option definitions can rely on validated integer argument conversion.
**Traceability:** `struct parseopt` / `struct optdef` usages at `src/parseopt/optset.c:95, 104, 112, 128, 145, 152, 159`

### Key Entities

#### `parseopt`
Represents parser state or parser context used when processing option definitions in this module. Its exact fields are not part of the evidenced interface here, but it participates in the option-setting flows that depend on validated numeric argument conversion.
**Traceability:** anonymous `struct parseopt` references in `src/parseopt/optset.c`

#### `optdef`
Represents an option definition associated with the parser context. Option-setting logic in this module uses such definitions to determine how an incoming option argument should be interpreted and validated.
**Traceability:** anonymous `struct optdef` references in `src/parseopt/optset.c`

#### Relationship between `parseopt` and `optdef`
`optdef` records are processed in the context of `parseopt`, and some option definitions use the module’s signed or unsigned integer conversion behavior to validate argument strings before applying them.
**Traceability:** combined `parseopt` / `optdef` occurrences at `src/parseopt/optset.c:95, 104, 112, 128, 145, 152, 159`

---

## Success Criteria

### Behavioral Correctness

1. **Signed conversion acceptance**
   - For representative valid signed integer strings within supplied bounds, the Rust module returns success and the expected numeric value.
   - **Traceability:** `get_signed_int`

2. **Signed conversion rejection**
   - For malformed signed input or values outside supplied bounds, the Rust module returns failure.

3. **Unsigned conversion acceptance**
   - For representative valid unsigned integer strings at or below the supplied maximum, the Rust module returns success and the expected numeric value.
   - **Traceability:** `get_unsigned_int`

4. **Unsigned conversion rejection**
   - For malformed input, negative textual input, or values above the supplied maximum, the Rust module returns failure.

### Integration Correctness

5. **Per-option range enforcement**
   - Tests demonstrate that different caller-supplied bounds produce different acceptance/rejection outcomes for the same input string where appropriate.
   - **Traceability:** `get_signed_int`, `get_unsigned_int`, `parseopt` / `optdef` integration role

6. **No false valid result on failure**
   - In all tested invalid-input cases, the Rust rewrite does not report success.
   - **Traceability:** `get_signed_int`, `get_unsigned_int`

7. **Support for option-setting usage**
   - The Rust rewrite exposes or preserves module behavior sufficient for option-setting flows backed by `parseopt` / `optdef` logic to obtain validated integer values from argument strings.
   - **Traceability:** `src/parseopt/optset.c`, `parseopt`, `optdef`