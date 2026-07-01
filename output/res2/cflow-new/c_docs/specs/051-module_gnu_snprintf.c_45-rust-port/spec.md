# spec.md

## Overview

This module provides a single formatting function equivalent in role to `snprintf`, implemented in `gnu/snprintf.c`. Its responsibility is to accept a destination character buffer, a buffer size, a format string, and variadic formatting arguments, then produce formatted text into the provided buffer while reporting the formatting result as an integer.

The Rust rewrite for branch `051-module_gnu_snprintf.c_45-rust-port` must preserve the observable behavior and functional boundary of this module as a bounded string-formatting entry point. The specification is limited to behavior evidenced by the source analysis: one public formatting function with no standalone module-owned data structures.

## Scope

### In Scope
- Bounded formatted output into a caller-provided character buffer.
- Acceptance of a format string plus variadic arguments.
- Returning an integer result for the formatting operation.
- Behavior corresponding to the module’s single exported function.

### Out of Scope
- Introducing additional public APIs.
- Defining new formatting syntax beyond what the module already accepts through its existing interface.
- Module-specific persistent state, configuration, or custom data structures.
- Guarantees not evidenced by the source analysis, including thread-safety, serialization, recovery behavior, FFI design, or performance targets.

## Feature Specification

### Feature: Bounded formatted string production

The module shall provide a functionally equivalent Rust implementation of the module’s existing `snprintf` behavior:

- It accepts:
  - a writable output buffer,
  - the size bound for that buffer,
  - a format string,
  - formatting arguments associated with that format string.
- It formats text according to the supplied format and arguments.
- It writes formatted output into the caller-supplied buffer subject to the provided size bound.
- It returns an integer status/result associated with the formatting operation.

### Required Rust Behavior
The Rust version must implement the same functional contract at the module boundary:

1. Use caller-supplied storage as the output destination.
2. Respect the supplied output size limit when producing output.
3. Consume a format string and matching formatting arguments as the source of the produced text.
4. Produce an integer return value representing the formatting call’s result.
5. Operate without requiring module-owned global or persistent data structures, since none are evidenced for this module.

## User Scenarios & Testing

### Scenario 1: Format text into a sufficiently large buffer
A caller provides a destination buffer large enough to hold the produced formatted text and terminator requirements of the underlying contract. The module formats the requested content into that buffer and returns the corresponding integer result.

**Test focus**
- Output content matches the requested format and arguments.
- The destination buffer contains the expected formatted string.
- The return value matches the formatting result defined by the original module behavior.

### Scenario 2: Format text into a size-limited buffer
A caller provides a destination buffer smaller than the full formatted output. The module still performs bounded formatting using the given size limit and returns the corresponding integer result.

**Test focus**
- The module does not write beyond the caller-provided size bound.
- The produced buffer content reflects bounded formatting behavior.
- The return value remains consistent with the original function’s behavior for size-limited output.

### Scenario 3: Use a zero or minimal size bound
A caller invokes formatting with a very small output size, including zero where permitted by the original contract. The module must handle the call according to the existing function’s observable behavior.

**Test focus**
- No out-of-bounds write occurs relative to the provided size.
- The return value matches original behavior for this boundary case.
- Any written output, if permitted by the size, remains within bounds.

### Scenario 4: Format different argument values through one entry point
A caller uses the same function with different format strings and corresponding arguments to produce different textual results.

**Test focus**
- The module accepts multiple valid format/argument combinations through the same interface.
- Each call produces output and a return value consistent with the original module behavior.

## Requirements

### Functional Requirements

#### FR-1: Provide bounded formatting entry point
The Rust module shall provide the functionality of the existing `snprintf` entry point from `gnu/snprintf.c`, accepting a destination buffer, a size bound, a format string, and formatting arguments.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

#### FR-2: Write formatted output to caller-provided buffer
The module shall write the formatted character output into the buffer provided by the caller rather than allocating a required module-owned output object.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

#### FR-3: Enforce caller-provided size bound on output
The module shall constrain output writes according to the `size` argument supplied by the caller.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

#### FR-4: Return integer formatting result
The module shall return an integer result for each formatting call, preserving the observable return-value role of the original function.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

#### FR-5: Accept formatting instructions and arguments per call
The module shall accept a format string and per-call formatting arguments and use them as the source of the produced output.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

### Key Entities

#### Entity: Output buffer
A caller-provided writable character storage region that receives formatted output.

**Relationship:** Supplied as input to the formatting function and bounded by the size argument.

**Traceability:** `gnu/snprintf.c`, function `snprintf`, parameter `char *str`

#### Entity: Output size bound
A caller-provided size value that limits how much output may be written into the destination buffer.

**Relationship:** Applies directly to the output buffer for each formatting call.

**Traceability:** `gnu/snprintf.c`, function `snprintf`, parameter `size_t size`

#### Entity: Format string
A caller-provided string describing the text to be produced and how arguments are incorporated into it.

**Relationship:** Interpreted together with variadic arguments to generate output.

**Traceability:** `gnu/snprintf.c`, function `snprintf`, parameter `const char *format`

#### Entity: Formatting arguments
Per-call variadic inputs associated with the format string.

**Relationship:** Consumed according to the format string to produce the final output text.

**Traceability:** `gnu/snprintf.c`, function `snprintf`, variadic parameter `...`

#### Entity: Integer result
The function’s return value representing the formatting operation’s result.

**Relationship:** Produced once per formatting call after bounded output handling.

**Traceability:** `gnu/snprintf.c`, function `snprintf`, return type `int`

## Success Criteria

### SC-1: Boundary compatibility
For representative valid calls equivalent to the original module interface, the Rust rewrite accepts the same categories of inputs:
- destination buffer,
- size bound,
- format string,
- formatting arguments.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

### SC-2: Correct bounded-write behavior
Tests covering sufficiently large, small, and zero/minimal buffer sizes demonstrate that output remains within the caller-provided size bound for every call.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

### SC-3: Output compatibility
For a defined set of formatting scenarios exercised against both the original C module and the Rust rewrite, the resulting buffer content is equivalent for the same inputs within the provided size bound.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

### SC-4: Return-value compatibility
For the same comparison scenarios, the Rust rewrite returns the same integer result as the original module.

**Traceability:** `gnu/snprintf.c`, function `snprintf`

### SC-5: No extra functional surface
The Rust rewrite exposes no additional module-level functionality beyond the behavior required to replace the original module’s single formatting entry point.

**Traceability:** `gnu/snprintf.c`, function `snprintf`