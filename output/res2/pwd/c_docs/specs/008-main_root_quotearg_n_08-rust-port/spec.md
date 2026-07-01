# spec.md

## Title

Functional Specification: `main_root_quotearg_n_08`

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_quotearg_n_08`
- **Category**: `main_cluster`
- **Source file**: `quotearg.c`
- **Rust branch target**: `008-main_root_quotearg_n_08-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides indexed argument quoting services built on the quoting facilities defined in `quotearg.c`. Its exposed behavior centers on returning quoted forms of input arguments through three entry points:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

The Rust rewrite must preserve the observable behavior of these entry points as a module that:

- quotes a provided argument string,
- supports quoting either NUL-terminated text or text with an explicit byte length,
- supports caller-specified left and right quote delimiters for the custom form,
- associates returned quoted output with a caller-selected slot index `n`, matching the indexed usage model evidenced by the module.

The specification is limited to behavior directly evidenced by the listed functions and the related internal entities in `quotearg.c`, especially `struct quoting_options` and `struct slotvec`.

## Feature Specification

### Summary

The module formats input arguments into quoted output strings. It supports three usage forms:

1. **Default indexed quoting** for a conventional C string.
2. **Indexed quoting with explicit input size** for data where the relevant length is provided separately.
3. **Indexed quoting with custom quote delimiters** for callers that need non-default surrounding quotes.

### In-scope functionality

The Rust version must implement the following module behavior:

- Accept a slot index `n` and produce quoted output associated with that slot.
- Accept plain input text via a NUL-terminated argument for default quoting.
- Accept input text plus explicit byte length for memory-bounded quoting.
- Accept custom left and right quote strings and apply them to the quoted result in the custom variant.
- Use quoting option state sufficient to represent the quoting style used by these entry points.
- Maintain per-slot output behavior consistent with indexed quoting semantics.

### Out-of-scope functionality

The Rust version must not claim or introduce capabilities not evidenced by this module analysis, including:

- new public APIs beyond the identified module entry points,
- serialization or persistence features,
- thread-safety guarantees,
- FFI commitments,
- recovery or retry mechanisms,
- benchmark or performance guarantees.

## User Scenarios & Testing

### Scenario 1: Quote a standard argument by slot

A caller has a normal argument string and needs its quoted representation for display or diagnostics. The caller selects a slot index and invokes the default indexed quoting function.

**Expected behavior**

- The module returns a quoted string for the provided argument.
- The result corresponds to the requested slot index.
- Repeating calls with the same or different slot indices remains valid under the module’s indexed model.

**Testing focus**

- Verify that a non-empty input string produces a quoted output string.
- Verify that different slot indices are accepted.
- Verify that the returned value is usable as the quoted representation for that input.

### Scenario 2: Quote an argument using an explicit byte length

A caller needs quoting for input where the relevant content length is specified independently of NUL termination.

**Expected behavior**

- The module quotes exactly the input region designated by the provided pointer and size.
- The operation does not require the input to be interpreted solely as an ordinary NUL-terminated string.

**Testing focus**

- Verify that the function accepts an explicit size.
- Verify that the resulting quoted content reflects the specified input extent.
- Verify behavior for zero-length and non-zero-length inputs.

### Scenario 3: Quote an argument with custom left and right delimiters

A caller wants the quoted result to use caller-supplied opening and closing quote strings instead of the module’s default quoting delimiters.

**Expected behavior**

- The module applies the provided left and right quote strings to the quoted output.
- The quoted result reflects both the input argument and the caller-selected delimiters.
- The result remains integrated with the slot-indexed behavior.

**Testing focus**

- Verify that custom delimiters appear in the output.
- Verify that distinct left and right delimiters are both supported.
- Verify that custom quoting works for empty and non-empty arguments.

### Scenario 4: Independent use of multiple slots

A caller uses more than one slot index to obtain quoted results for different arguments.

**Expected behavior**

- The module supports multiple slot indices as part of its public indexed interface.
- Use of one slot does not invalidate the module’s ability to produce a quoted result for another slot.

**Testing focus**

- Invoke quoting through at least two distinct slot indices.
- Confirm that each call yields a valid quoted result for its corresponding input.
- Confirm continued correctness across interleaved calls.

## Requirements

### Functional Requirements

#### FR-1: Indexed quoting entry points

The module shall provide behavior equivalent to the indexed quoting entry points `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom` defined in `quotearg.c`.

**Traceability**: `quotearg.c:925-935`, `quotearg.c:1012-1018`

#### FR-2: Default quoting of NUL-terminated input

The module shall accept a slot index `n` and a character argument and return a quoted representation of that argument through the default indexed quoting path.

**Traceability**: `quotearg_n` in `quotearg.c:925-929`

#### FR-3: Quoting with explicit input length

The module shall accept a slot index `n`, an input pointer, and an explicit byte length, and return a quoted representation of the specified input extent.

**Traceability**: `quotearg_n_mem` in `quotearg.c:931-935`

#### FR-4: Quoting with caller-provided quote delimiters

The module shall accept a slot index `n`, a left quote string, a right quote string, and an input argument, and return a quoted representation using those custom delimiters.

**Traceability**: `quotearg_n_custom` in `quotearg.c:1012-1018`

#### FR-5: Quoting options support for exposed behaviors

The module shall maintain quoting configuration sufficient to support default and custom quoting behaviors exercised by the exposed entry points.

**Traceability**: `struct quoting_options` references in `quotearg.c`, including `57-74`, `782`, `784`, `795`, `808`, `810`, `874`, `952`, `960`, `979`, `1006`, `1025`, `1047`

#### FR-6: Slot-based result management

The module shall support a slot-based model for quoted results, as evidenced by the use of slot index `n` and the `slotvec` entity.

**Traceability**: `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`; `struct slotvec` in `quotearg.c:829-833`, `839`, `840`, `845`, `878`

#### FR-7: Result form

The module shall produce quoted output in string form consistent with the original module’s public behavior.

**Traceability**: return type usage of `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom` in `quotearg.c:925-935`, `1012-1018`

### Key Entities

#### `quoting_options`

A configuration entity representing the quoting behavior applied when formatting arguments. Within this module’s scope, it is the options-bearing structure that enables the default and custom quoting behaviors used by the exposed entry points.

**Relationship to module behavior**

- Supplies quoting configuration for argument formatting.
- Is involved when custom delimiter behavior is selected.
- Connects caller intent to the produced quoted output.

**Traceability**: `struct quoting_options` references throughout `quotearg.c`, including `57-74` and later use sites tied to exposed functions

#### `slotvec`

A slot management entity used to support indexed quoted results. Within this module’s scope, it represents the per-slot storage model that underlies the `n` parameter accepted by the public functions.

**Relationship to module behavior**

- Associates a quoted result with a selected slot index.
- Supports repeated and multi-slot use of the quoting interface.

**Traceability**: `struct slotvec` in `quotearg.c:829-833`, `839`, `840`, `845`, `878`

#### Input argument data

The module accepts two input forms:

- NUL-terminated argument text for the default and custom functions.
- Pointer-plus-length argument data for the memory-bounded function.

**Relationship to module behavior**

- Serves as the source content to be quoted.
- Determines whether the quoting operation is string-based or explicit-length-based.

**Traceability**: function signatures in `quotearg.c:925-935`, `1012-1018`

#### Custom quote delimiters

The custom quoting entry point accepts left and right quote strings as caller-provided delimiter data.

**Relationship to module behavior**

- Alters the delimiters used around the quoted output.
- Applies only to the custom quoting path.

**Traceability**: `quotearg_n_custom` in `quotearg.c:1012-1018`

## Success Criteria

### SC-1: Entry point coverage

The Rust module exposes behaviorally equivalent implementations for all three evidenced entry points:

- default indexed quoting,
- explicit-length indexed quoting,
- custom-delimiter indexed quoting.

**Measured by**

- Existence of Rust functionality mapped to each source entry point.
- Tests covering each entry point.

**Traceability**: `quotearg.c:925-935`, `1012-1018`

### SC-2: Correct handling of indexed usage

The Rust module correctly supports use with multiple slot indices in accordance with the indexed interface model.

**Measured by**

- Tests that call the module with at least two distinct slot indices.
- Each call returns a valid quoted result for its input.

**Traceability**: public parameter `n` in all three functions; `struct slotvec`

### SC-3: Correct handling of explicit-length input

The Rust module correctly quotes input supplied with an explicit byte length.

**Measured by**

- Tests for zero-length input.
- Tests for non-zero-length input.
- Tests demonstrating that the explicit-size path is independently exercised.

**Traceability**: `quotearg_n_mem` in `quotearg.c:931-935`

### SC-4: Correct application of custom delimiters

The Rust module correctly applies caller-provided left and right quote strings in the custom quoting path.

**Measured by**

- Tests that verify the presence of the provided left and right delimiters in the result.
- Tests using distinct delimiter values.

**Traceability**: `quotearg_n_custom` in `quotearg.c:1012-1018`

### SC-5: Preservation of quoted string output role

The Rust module returns quoted output in a string form usable by callers in the same role as the original API.

**Measured by**

- Tests confirming that each public path yields a string result representing the quoted input.
- No substitution of unrelated output forms for the module’s public behavior.

**Traceability**: return behavior of `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`