# spec.md

## Title

Functional Specification: `main_root_quote_n_11`

## Overview

This module provides slot-indexed quoting entry points for string arguments. Its exposed behavior is centered on two functions from `quotearg.c`:

- `quote_n_mem(int n, char const *arg, size_t argsize)`
- `quote_n(int n, char const *arg)`

The Rust rewrite must preserve the functional role of these entry points: produce and return a quoted representation of input text using the module’s quoting machinery, with output selected by a caller-provided slot index `n`.

The available analysis also shows that these functions operate in the context of:

- `struct quoting_options`
- `struct slotvec`

Therefore, the Rust version must preserve the observable behavior that these APIs depend on: quoting according to module-defined/default quoting options and managing per-slot returned quoted results.

## Feature Specification

### Feature: Slot-indexed quoting of byte sequences

The module must accept an input character buffer plus an explicit byte length and return a quoted form of that input for a specified slot index.

Observed source evidence:
- `quote_n_mem` in `quotearg.c:1055-1059`
- `struct quoting_options`
- `struct slotvec`

Required behavior:
- The caller supplies:
  - a slot number `n`
  - an input pointer/reference to character data
  - an explicit input size
- The module returns a quoted string result associated with slot `n`.
- The quoting behavior must be consistent with the module’s quoting configuration model evidenced by `struct quoting_options`.
- The operation must support inputs whose length is determined by the explicit size parameter rather than C string termination.

### Feature: Slot-indexed quoting of NUL-terminated strings

The module must accept a conventional string input and return a quoted form of that input for a specified slot index.

Observed source evidence:
- `quote_n` in `quotearg.c:1067-1071`
- `struct quoting_options`
- `struct slotvec`

Required behavior:
- The caller supplies:
  - a slot number `n`
  - an input string
- The module returns a quoted string result associated with slot `n`.
- The input is interpreted as a NUL-terminated string.
- The result must be functionally consistent with quoting the same bytes through the explicit-length path.

### Feature: Shared quoting option model

The module must use the project’s quoting option model as part of how quoted output is determined.

Observed source evidence:
- repeated `struct quoting_options` references throughout `quotearg.c`

Required behavior:
- The Rust rewrite must preserve compatibility with the quoting option semantics used by these entry points.
- The exposed behavior of `quote_n_mem` and `quote_n` must remain governed by the same quoting rules/defaults used by this module family.
- No new externally visible quoting modes or configuration surfaces may be invented in this rewrite unless separately evidenced elsewhere.

### Feature: Per-slot result storage behavior

The module must preserve the slot-based output selection implied by `quote_n_mem`, `quote_n`, and `struct slotvec`.

Observed source evidence:
- `quote_n_mem`
- `quote_n`
- `struct slotvec` entries around `quotearg.c:829-845`, `878`

Required behavior:
- Different slot indices are treated as distinct result channels.
- The returned quoted result is tied to the specified slot number.
- Repeated calls may reuse or update the result for a given slot, as implied by slot-backed storage.
- The Rust rewrite must preserve externally visible slot semantics rather than collapsing all calls into one undifferentiated shared return buffer.

## User Scenarios & Testing

### Scenario 1: Quote a standard pathname-like string

A caller has a normal NUL-terminated string and needs its quoted representation.

Example usage intent:
- Call `quote_n(0, arg)`
- Receive a quoted string for slot `0`

What must be testable:
- The function accepts a string input.
- It returns a non-empty quoted form when quoting is required by the module’s rules.
- The returned value is stable enough for immediate caller use as the module’s quoted result for that slot.

### Scenario 2: Quote a byte sequence with explicit length

A caller has data that should be quoted using a byte count instead of relying on the first NUL byte.

Example usage intent:
- Call `quote_n_mem(0, arg, argsize)`
- Receive quoted output based on exactly `argsize` bytes

What must be testable:
- Inputs containing embedded NUL bytes are processed according to explicit length.
- Truncation at the first NUL must not occur merely because the input contains a NUL before `argsize`.
- The quoted result reflects the provided byte span.

### Scenario 3: Use multiple slots independently

A caller needs more than one quoted result available from the module without all results referring to the same active slot.

Example usage intent:
- Call `quote_n(0, a)`
- Call `quote_n(1, b)`

What must be testable:
- The result for slot `0` corresponds to `a`.
- The result for slot `1` corresponds to `b`.
- Using one slot does not make another slot’s returned content become semantically indistinguishable from the newer call’s content.

### Scenario 4: Reuse the same slot for a new argument

A caller uses the same slot repeatedly for successive quoting operations.

Example usage intent:
- Call `quote_n(0, first)`
- Later call `quote_n(0, second)`

What must be testable:
- The later result for slot `0` corresponds to `second`.
- Slot reuse updates that slot’s visible quoted result.

### Scenario 5: Consistent behavior between string and explicit-length entry points

A caller uses either API depending on whether the input length is already known.

Example usage intent:
- Call `quote_n(0, "abc")`
- Call `quote_n_mem(0, "abc", 3)`

What must be testable:
- For equivalent input bytes, both entry points produce equivalent quoted content under the same module quoting rules.

## Requirements

### Functional Requirements

#### FR-1: Provide slot-indexed quoting for explicit-length input
The Rust module shall implement the behavior of `quote_n_mem` from `quotearg.c`, accepting a slot index, input bytes, and explicit size, and returning the quoted representation for that slot.

Traceability:
- `quote_n_mem` (`quotearg.c:1055-1059`)

#### FR-2: Provide slot-indexed quoting for NUL-terminated string input
The Rust module shall implement the behavior of `quote_n` from `quotearg.c`, accepting a slot index and string input, and returning the quoted representation for that slot.

Traceability:
- `quote_n` (`quotearg.c:1067-1071`)

#### FR-3: Respect the module’s quoting option model
The Rust module shall preserve the quoting behavior dependency on the module’s quoting configuration model represented by `struct quoting_options`.

Traceability:
- `struct quoting_options` references throughout `quotearg.c`
- `quote_n_mem`
- `quote_n`

#### FR-4: Preserve distinct slot semantics
The Rust module shall preserve behavior in which quoted results are selected and managed by slot index, consistent with the presence and use of `struct slotvec`.

Traceability:
- `struct slotvec` (`quotearg.c:829-845`, `839`, `840`, `845`, `878`)
- `quote_n_mem`
- `quote_n`

#### FR-5: Support explicit-length processing independent of C string termination
For the explicit-length entry point, the Rust module shall determine the quoted input span from the provided size parameter rather than solely from NUL termination.

Traceability:
- `quote_n_mem` signature includes `size_t argsize`

#### FR-6: Produce equivalent quoted content for equivalent inputs across the two entry points
When `quote_n` and `quote_n_mem` are given equivalent input bytes under equivalent conditions, the Rust module shall produce equivalent quoted output content.

Traceability:
- `quote_n_mem`
- `quote_n`

### Key Entities

#### `quoting_options`
A module-level quoting configuration structure that defines the rules governing how input text is transformed into quoted output.

Role:
- Governs quoting behavior used by the exposed quoting entry points.

Relationship to other entities:
- Applied by `quote_n_mem` and `quote_n`
- Determines how the final result stored or returned through slot handling is formed

Traceability:
- `struct quoting_options` references across `quotearg.c`

#### `slotvec`
A slot-oriented storage structure associated with indexed quoted results.

Role:
- Supports per-slot result handling for quoted strings.

Relationship to other entities:
- Used by slot-indexed APIs `quote_n_mem` and `quote_n`
- Holds or tracks state for the quoted result corresponding to each slot index

Traceability:
- `struct slotvec` references in `quotearg.c`

## Success Criteria

### SC-1: API-equivalent functional coverage
The Rust rewrite exposes behaviorally equivalent implementations of both analyzed entry points:
- explicit-length quoting via `quote_n_mem`
- NUL-terminated string quoting via `quote_n`

Measured by:
- Tests covering both call patterns and verifying returned quoted content is produced.

Traceability:
- `quote_n_mem`
- `quote_n`

### SC-2: Correct explicit-length handling
Tests demonstrate that the explicit-length path uses the provided byte length, including cases where embedded NUL bytes occur before the specified end of input.

Traceability:
- `quote_n_mem`

### SC-3: Distinct slot behavior is preserved
Tests demonstrate that using two different slot indices yields independently addressable quoted results corresponding to their respective inputs.

Traceability:
- `quote_n_mem`
- `quote_n`
- `struct slotvec`

### SC-4: Slot reuse updates the selected slot
Tests demonstrate that repeated use of the same slot index causes the later call’s quoted content to become the current result for that slot.

Traceability:
- `quote_n_mem`
- `quote_n`
- `struct slotvec`

### SC-5: Entry-point consistency
Tests demonstrate that `quote_n(arg)` and `quote_n_mem(arg, strlen(arg))` produce equivalent quoted content for the same textual input.

Traceability:
- `quote_n_mem`
- `quote_n`

### SC-6: Quoting behavior remains governed by the module’s quoting model
Validation against the original module behavior demonstrates that returned quoted content follows the same quoting rules/defaults relied upon by these entry points.

Traceability:
- `struct quoting_options`
- `quote_n_mem`
- `quote_n`