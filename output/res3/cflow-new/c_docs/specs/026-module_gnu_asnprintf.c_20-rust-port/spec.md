# spec.md

## Title

Functional Specification for `module_gnu_asnprintf.c_20` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_asnprintf.c_20`
- Category: `module_cluster`
- Source file: `gnu/asnprintf.c`
- Primary source function: `asnprintf`
- Rust branch: `026-module_gnu_asnprintf.c_20-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides a formatted string construction entry point that accepts a destination buffer pointer, a pointer to a length value, and a printf-style format string with variadic arguments, and returns a character buffer containing the formatted result.

The Rust rewrite must preserve the observable behavior of this formatting entry point as a module-level capability: accepting caller-supplied formatting input, producing a NUL-terminated byte string result, updating the caller-visible output length, and returning the resulting buffer pointer or failure indication consistent with the source behavior evidenced by the module interface.

## Scope

In scope:

- The functionality represented by the exported `asnprintf` entry point in `gnu/asnprintf.c`.
- Behavior visible from its signature:
  - formatted output generation from a format string and variadic arguments,
  - use of an optional caller-provided result buffer,
  - use of a caller-provided length pointer as output state,
  - returning a character pointer to the produced string.

Out of scope:

- Any additional formatting APIs not evidenced in this module.
- Any guarantees about internals, allocation strategy, thread behavior, locale behavior, or interoperability beyond what is required by the exposed function contract.
- Any public APIs beyond the behavior corresponding to `asnprintf`.

## Feature Specification

### Feature: formatted string creation with caller-visible length

The module exposes a single formatting operation equivalent to `asnprintf`.

The Rust version must implement a functionally equivalent capability that:

- accepts formatting input consisting of:
  - a possibly caller-supplied output buffer,
  - a mutable output length location,
  - a format string,
  - formatting arguments;
- produces formatted textual output based on the format string and arguments;
- returns a pointer/reference/ownership result representing the produced character buffer in the Rust port’s compatibility boundary;
- ensures the produced string is suitable for C-style string use, including terminating NUL semantics at the compatibility boundary;
- updates the caller-visible length output to reflect the resulting formatted string length, excluding the terminating NUL unless source-compatible evidence requires otherwise.

### Feature boundaries

This module is responsible only for the top-level “allocate or use buffer and format into it” operation exposed as `asnprintf`.

This module is not specified to:

- define new formatting syntax;
- expose parsing utilities;
- expose buffer management as a separate public feature;
- provide multiple independent formatting modes.

## User Scenarios & Testing

### Scenario 1: Create a formatted string without reusing an existing buffer

A caller needs a newly produced formatted string for a format such as a literal plus substituted values.

Expected behavior:

- The caller provides formatting input and a valid length location.
- The module returns a pointer/result identifying a formatted C-style string.
- The length output is updated to the produced text length.
- The returned string content matches the format and arguments.

Testing focus:

- verify exact formatted content;
- verify terminating NUL presence at the compatibility boundary;
- verify reported length matches produced content length.

### Scenario 2: Use a caller-provided buffer as the initial destination

A caller passes an existing buffer pointer together with length state and requests formatted output.

Expected behavior:

- The module accepts the caller-provided destination input.
- The formatted result is produced and returned.
- The caller-visible length is updated to the resulting string length.
- If the initial buffer is insufficient, behavior must remain source-compatible at the functional level: the result still represents the full formatted output or failure as defined by the original module behavior.

Testing focus:

- verify success when the supplied buffer is large enough;
- verify correct result content and length;
- verify source-compatible handling when the supplied buffer is too small.

### Scenario 3: Produce an empty formatted result

A caller uses formatting that results in an empty string.

Expected behavior:

- The module returns a valid string result representing empty content.
- The string is NUL-terminated.
- The output length is updated to zero.

Testing focus:

- verify zero-length reporting;
- verify returned string starts with NUL;
- verify no extra visible characters are present.

### Scenario 4: Handle formatting failure

A caller provides input that causes formatting to fail, or the module cannot produce the required result.

Expected behavior:

- The module indicates failure through its return value in a source-compatible manner.
- The Rust port must not report successful content when formatting failed.

Testing focus:

- verify failure indication is observable;
- verify no false success result is returned;
- verify any caller-visible length behavior matches the original module contract as determined during port validation.

## Requirements

### Functional Requirements

#### FR-1: Formatted output operation

The Rust port shall provide the functionality of the source module’s `asnprintf` operation from `gnu/asnprintf.c`.

Traceability:
- Source file: `gnu/asnprintf.c`
- Source function: `asnprintf`

#### FR-2: Format-driven content generation

The module shall generate output text according to the provided format string and accompanying arguments.

Traceability:
- Source function signature: `asnprintf(char *resultbuf, size_t *lengthp, const char *format, ...)`

#### FR-3: Caller-visible result buffer return

The module shall return a character-buffer result representing the formatted string, with success or failure indicated through the returned result in a source-compatible manner.

Traceability:
- Source function: `asnprintf`
- Return type evidence: `char *`

#### FR-4: Caller-visible length update

The module shall use the caller-provided length location as output state and update it to describe the produced formatted string length on successful formatting.

Traceability:
- Source function signature parameter: `size_t *lengthp`

#### FR-5: Support for caller-provided initial buffer input

The module shall accept a caller-provided buffer input and use it as part of the formatting operation contract.

Traceability:
- Source function signature parameter: `char *resultbuf`

#### FR-6: C-string compatible output

The module shall produce output suitable for C-string consumption at the module compatibility boundary, including terminating NUL semantics.

Traceability:
- Source function return type and purpose: `char *` formatted string result

#### FR-7: Empty-output correctness

When formatting yields no visible characters, the module shall return an empty string result and report zero output length.

Traceability:
- Source function purpose: formatted string production through `asnprintf`

#### FR-8: Failure propagation

If the formatted result cannot be produced, the module shall expose failure through the result contract rather than returning incorrect formatted content.

Traceability:
- Source function: `asnprintf`
- Return type evidence: pointer-based success/failure signaling

### Key Entities

#### Entity: Result buffer

A character buffer representing the formatted output string.

Relationship to other entities:

- receives content derived from the format string and arguments;
- may originate from caller-supplied buffer input;
- is the object identified by the function return value.

Traceability:
- Source signature elements: `char *resultbuf`, return type `char *`

#### Entity: Output length location

A mutable length value supplied by the caller and updated by the formatting operation.

Relationship to other entities:

- describes the length of the result buffer’s formatted content;
- is written by the module as part of successful operation.

Traceability:
- Source signature element: `size_t *lengthp`

#### Entity: Format string

A C string that defines the textual template for the resulting output.

Relationship to other entities:

- combines with variadic arguments to determine result buffer content.

Traceability:
- Source signature element: `const char *format`

#### Entity: Formatting arguments

The variadic argument list consumed according to the format string.

Relationship to other entities:

- supplies values inserted into the output text defined by the format string.

Traceability:
- Source signature variadic portion: `...`

## Success Criteria

### SC-1: Correct formatted content

For representative valid format inputs, the Rust port produces byte-for-byte formatted output equivalent to the source module behavior.

Traceability:
- `asnprintf`

### SC-2: Correct length reporting

For successful formatting operations, the caller-visible length output matches the produced string length.

Traceability:
- `size_t *lengthp` parameter in `asnprintf`

### SC-3: C-string termination compatibility

For successful operations, the result is consumable as a C-style string with correct terminating NUL semantics.

Traceability:
- `char *` result contract of `asnprintf`

### SC-4: Caller buffer contract preservation

Tests covering caller-supplied destination buffer usage confirm source-compatible behavior when the initial buffer is sufficient and when it is insufficient.

Traceability:
- `char *resultbuf` parameter in `asnprintf`

### SC-5: Empty string handling

A formatting case that yields empty output returns an empty result and reports length zero.

Traceability:
- `asnprintf` formatted string production behavior

### SC-6: Failure signaling correctness

For failure cases identified during source-compatibility validation, the Rust port signals failure through the result contract and does not present invalid success data.

Traceability:
- `asnprintf`

## Validation Notes

Because the available module evidence consists of a single exported function signature and source location, the Rust port validation must prioritize observable contract compatibility at the function boundary:

- returned result behavior,
- updated length behavior,
- formatted content behavior,
- C-string compatibility behavior.

No additional public behavior shall be assumed beyond what is necessary to preserve the functionality evidenced by `gnu/asnprintf.c`.