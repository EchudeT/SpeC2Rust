# spec.md

## Overview

This module provides two public quoting entry points from `quotearg.c`:

- `quotearg_colon`
- `quotearg_colon_mem`

Their role is to return a quoted representation of input text using the module’s quoting configuration machinery, with colon treated as a character that must be quoted/escaped. The Rust rewrite must preserve the externally visible behavior of these two entry points and their dependence on the module’s quoting option model.

## Feature Specification

### Purpose

The module converts input bytes or C strings into a quoted string suitable for contexts where the colon character must not appear unquoted in output. It does this by routing calls through the module’s shared quoting-options system.

### In Scope

The Rust version must implement:

- quoting of a NUL-terminated string input via `quotearg_colon`
- quoting of an arbitrary byte span with explicit length via `quotearg_colon_mem`
- use of the module’s quoting-options model to apply a “quote colon” behavior
- returning a quoted string result for each call

### Behavioral Summary

- `quotearg_colon` accepts a string input and produces its quoted form.
- `quotearg_colon_mem` accepts a pointer plus byte length and produces the quoted form of exactly that byte range.
- Both functions use quoting behavior associated with the module’s `quoting_options` data structure.
- The resulting output must reflect the module behavior that colon is treated as a character requiring quoting in these entry points.

### Out of Scope

The specification does not require introducing any new public APIs, new quoting styles beyond those evidenced through the existing option model, or any guarantees not evidenced by the source analysis.

## User Scenarios & Testing

### Scenario 1: Quote a normal path-like string containing a colon

A caller passes a string containing `:` to `quotearg_colon`.

Expected result:

- a quoted string is returned
- the returned representation reflects the module’s colon-quoting behavior
- the original input is not modified

### Scenario 2: Quote a string without a colon

A caller passes a string that contains no `:` to `quotearg_colon`.

Expected result:

- a quoted string is still returned
- output follows the same quoting rules used by this module
- absence of `:` does not cause failure

### Scenario 3: Quote a byte range with explicit size

A caller passes bytes and a length to `quotearg_colon_mem`.

Expected result:

- only the specified number of bytes are considered
- output is produced even if the byte range is not NUL-terminated
- any colon within the specified range is quoted according to module behavior

### Scenario 4: Quote data containing embedded NUL bytes

A caller uses `quotearg_colon_mem` with a byte sequence that includes `\0` before the specified end.

Expected result:

- processing continues through the full supplied length
- output corresponds to the entire byte range, not just bytes before the first NUL

### Scenario 5: Repeated use through shared quoting machinery

A caller invokes these functions multiple times.

Expected result:

- each invocation returns a valid quoted result
- results remain consistent with the same module-defined colon-quoting behavior

### Testing Guidance

The Rust port should be tested with:

- empty input
- input with no colon
- input with one colon
- input with multiple colons
- explicit-length byte input
- explicit-length input containing embedded NUL
- inputs whose quoted forms require use of the underlying quoting options behavior

Tests should compare Rust results to the C module behavior for the same inputs.

## Requirements

### Functional Requirements

#### FR-1: String-based colon quoting

The module shall provide behavior equivalent to `quotearg_colon(char const *arg)` from `quotearg.c`, accepting string input and returning its quoted form.

Traceability: `quotearg.c`, `quotearg_colon`

#### FR-2: Length-based colon quoting

The module shall provide behavior equivalent to `quotearg_colon_mem(char const *arg, size_t argsize)` from `quotearg.c`, accepting a byte pointer and explicit byte length and returning the quoted form of exactly that byte range.

Traceability: `quotearg.c`, `quotearg_colon_mem`

#### FR-3: Colon-sensitive quoting behavior

Both public entry points shall apply quoting behavior in which colon is treated as a character requiring quoting/escaping in the produced output.

Traceability: `quotearg.c`, `quotearg_colon`, `quotearg_colon_mem`, `struct quoting_options`

#### FR-4: Integration with quoting options

The Rust implementation shall preserve the use of the module’s quoting-options concept as the source of the quoting policy used by these entry points.

Traceability: `quotearg.c`, `struct quoting_options`

#### FR-5: Support for non-NUL-terminated data in explicit-length mode

For the explicit-length entry point, the implementation shall process input according to the supplied length rather than relying on NUL termination.

Traceability: `quotearg.c`, `quotearg_colon_mem`

#### FR-6: Returned quoted result

Each successful call shall produce a string result representing the quoted input.

Traceability: `quotearg.c`, `quotearg_colon`, `quotearg_colon_mem`

### Key Entities

#### `quoting_options`

The module’s core quoting policy structure. It represents the configuration used to determine how input is quoted, including the behavior used by the colon-specific entry points.

Traceability: `quotearg.c`, `struct quoting_options`

Relationship:

- `quotearg_colon` and `quotearg_colon_mem` depend on this quoting policy model to produce their output.

#### `slotvec`

A storage-related structure present in the module and associated with managing quoted results across calls.

Traceability: `quotearg.c`, `struct slotvec`

Relationship:

- public quoting entry points return quoted string data through the module’s result-management mechanism.
- this structure is part of that mechanism as evidenced by its presence in the same quoting module.

## Success Criteria

### SC-1: Public API behavior parity

For representative inputs, the Rust versions of `quotearg_colon` and `quotearg_colon_mem` produce the same quoted outputs as the C module.

Traceability: `quotearg.c`, `quotearg_colon`, `quotearg_colon_mem`

### SC-2: Colon handling correctness

For inputs containing one or more `:` characters, the Rust output demonstrates the same colon-quoting treatment as the C module.

Traceability: `quotearg.c`, `quotearg_colon`, `quotearg_colon_mem`, `struct quoting_options`

### SC-3: Explicit-length correctness

For byte inputs with a specified length, including those containing embedded NUL bytes, the Rust `quotearg_colon_mem` output matches C behavior for exactly the provided byte span.

Traceability: `quotearg.c`, `quotearg_colon_mem`

### SC-4: Option-model preservation

The Rust rewrite retains a quoting-options-driven behavior model sufficient to support the colon-specific quoting used by this module.

Traceability: `quotearg.c`, `struct quoting_options`

### SC-5: Scenario coverage

Automated tests cover all scenarios listed in this specification and pass against behavior expected from the analyzed C module.

Traceability: `quotearg.c`, `quotearg_colon`, `quotearg_colon_mem`, `struct quoting_options`