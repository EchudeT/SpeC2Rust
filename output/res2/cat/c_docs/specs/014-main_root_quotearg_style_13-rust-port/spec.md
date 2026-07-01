# spec.md

## Title

Rust Functional Specification for `main_root_quotearg_style_13`

## Status

Draft

## Scope

This specification covers the Rust rewrite of the `main_root_quotearg_style_13` module from `quotearg.c`. The analyzed module surface is limited to two entry points:

- `quotearg_style`
- `quotearg_style_mem`

These functions operate in the context of the module’s quoting configuration model represented by `struct quoting_options`, and the module-managed storage represented by `struct slotvec`.

This specification defines only the behavior evidenced by the analyzed module inputs.

## Feature Specification

### Summary

The module provides quoting of an input argument using a caller-selected quoting style.

It supports:

- quoting a NUL-terminated string using a specified quoting style
- quoting a memory region of explicit length using a specified quoting style
- producing the quoted result as returned character data
- applying the selected style through the module’s quoting-options model

### In-Scope Behavior

The Rust version must implement behavior equivalent to the analyzed C module for the following functional boundary:

1. Accept a quoting style selector and input data.
2. Apply that style to the input using the module’s quoting behavior.
3. Return the resulting quoted character sequence.
4. Support both:
   - string input terminated by NUL
   - byte sequence input with explicit size

### Out of Scope

The Rust rewrite spec does not require any capability not evidenced by the analyzed module surface, including but not limited to:

- defining new public quoting APIs beyond the evidenced functions
- adding style kinds not present in the source module’s quoting model
- promising thread-safety or reentrancy properties
- serialization, persistence, or IPC behavior
- FFI compatibility guarantees
- performance or benchmark targets

## User Scenarios & Testing

### Scenario 1: Quote a regular string with a chosen style

A caller has a normal argument string and wants a quoted version formatted according to a selected quoting style.

**Expected behavior**
- The module accepts the style and the string.
- The module returns a quoted character result for that string.
- The result reflects the selected style rather than an implicit default unrelated to the call.

**Primary entry point**
- `quotearg_style`

**Test focus**
- Call with a non-empty string and each supported style value from the source module.
- Verify the output matches the C module’s behavior for the same style and input.

### Scenario 2: Quote a memory buffer that may include embedded NUL bytes

A caller has argument data represented as memory plus length and cannot rely on NUL termination.

**Expected behavior**
- The module accepts the style, buffer pointer/data, and explicit size.
- The module processes exactly the specified number of input bytes.
- Embedded NUL bytes are treated as part of the input region rather than as an early terminator.

**Primary entry point**
- `quotearg_style_mem`

**Test focus**
- Use buffers containing embedded NUL and trailing bytes after the first NUL.
- Verify output matches the C module for identical style, bytes, and length.

### Scenario 3: Distinguish string-based and length-based input handling

A caller needs different behavior depending on whether input is passed as a C string or as explicit-length memory.

**Expected behavior**
- The string variant stops according to string semantics.
- The memory variant respects the provided byte count.
- The two calls may produce different results when the memory contains bytes beyond the first NUL.

**Primary entry points**
- `quotearg_style`
- `quotearg_style_mem`

**Test focus**
- For input bytes like `b"a\0b"`, compare:
  - string-style call using `"a"`
  - memory-style call using all 3 bytes
- Confirm results differ only in ways consistent with the C implementation.

### Scenario 4: Quoting style selection affects output form

A caller uses different quoting styles for the same input and expects style-dependent output formatting.

**Expected behavior**
- The same input quoted under different style values yields outputs consistent with those distinct style definitions in the original module.
- Style selection is applied through the quoting options model, not ignored.

**Test focus**
- For a representative set of inputs, compare outputs across multiple style values.
- Confirm the Rust outputs match the C outputs style-by-style.

## Requirements

### Functional Requirements

#### FR-1: Style-based string quoting

The module shall provide a function equivalent to `quotearg_style` that accepts:

- a quoting style value
- a string argument

and returns the quoted result for that string using the specified style.

**Traceability**
- `quotearg.c`
- `quotearg_style` at lines 964-968
- `struct quoting_options`

#### FR-2: Style-based explicit-length quoting

The module shall provide a function equivalent to `quotearg_style_mem` that accepts:

- a quoting style value
- an argument buffer
- an explicit buffer size

and returns the quoted result for exactly that input region using the specified style.

**Traceability**
- `quotearg.c`
- `quotearg_style_mem` at lines 970-974
- `struct quoting_options`

#### FR-3: Shared quoting semantics across the two entry points

The module shall apply the same quoting-style model to both entry points, differing only in input-length interpretation:

- string semantics for the string variant
- explicit-size semantics for the memory variant

**Traceability**
- `quotearg_style`
- `quotearg_style_mem`
- `struct quoting_options`

#### FR-4: Style selection must influence output

The module shall honor the caller-provided quoting style when producing output.

Different quoting style values defined by the source module’s quoting model shall be observable through corresponding output behavior, matching the original module.

**Traceability**
- `quotearg_style`
- `quotearg_style_mem`
- `struct quoting_options`

#### FR-5: Returned output must be usable as character data

Each entry point shall return the quoted result as character data corresponding to the source module’s returned `char *` behavior.

The Rust rewrite may represent this with Rust-owned or borrowed output internally, but externally the functional result must be equivalent to the C module’s returned quoted text.

**Traceability**
- `quotearg_style` return type
- `quotearg_style_mem` return type
- `struct slotvec`

### Key Entities

#### `quoting_options`

This entity represents the quoting configuration model used by the module to determine how an argument is rendered under a selected quoting style.

**Role**
- Carries the quoting-style choice and related option state used during quoting.
- Provides the configuration basis shared by both public entry points.

**Relationships**
- `quotearg_style` uses it to quote string input.
- `quotearg_style_mem` uses it to quote explicit-length input.
- It governs the style-dependent behavior observable in output.

**Traceability**
- `struct quoting_options` occurrences throughout `quotearg.c`
- `quotearg_style`
- `quotearg_style_mem`

#### `slotvec`

This entity represents module-managed storage associated with returned quoted results.

**Role**
- Holds or tracks storage used for produced quoted character data.

**Relationships**
- Supports the result-returning behavior of the quoting entry points.
- Exists alongside the quoting-options model rather than replacing it.

**Traceability**
- `struct slotvec` occurrences in `quotearg.c`
- proximity to the public quoting entry points and returned-character behavior

## Success Criteria

### Behavioral Equivalence

1. For every quoting style supported by the original module and for representative string inputs, the Rust `quotearg_style` behavior matches the C module’s output.
2. For every quoting style supported by the original module and for representative byte buffers with explicit lengths, the Rust `quotearg_style_mem` behavior matches the C module’s output.
3. For inputs containing embedded NUL bytes, the Rust memory-length variant matches the C module by processing the full specified length.
4. For cases where string and explicit-length semantics differ, the Rust rewrite preserves the same distinction as the C module.

### Interface-Level Completeness

5. The Rust rewrite exposes functional equivalents of the two analyzed entry points and no additional behavior is required by this specification.

### Entity Fidelity

6. The Rust rewrite preserves the functional roles of:
   - quoting configuration equivalent to `quoting_options`
   - result-storage behavior corresponding to the module’s returned quoted data and associated `slotvec` role

### Testability

7. A conformance test set can be written that compares Rust and C outputs for:
   - multiple quoting styles
   - empty input
   - ordinary ASCII input
   - input containing characters that require quoting under at least one style
   - explicit-length input containing embedded NUL bytes

8. All such conformance tests pass against the Rust rewrite for the analyzed module surface.