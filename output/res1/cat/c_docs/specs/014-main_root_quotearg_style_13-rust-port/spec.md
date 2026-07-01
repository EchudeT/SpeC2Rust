# spec.md

## Title

Functional Specification: `main_root_quotearg_style_13`

## Document Control

- Project: `cat`
- Module: `main_root_quotearg_style_13`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Rust branch: `014-main_root_quotearg_style_13-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides style-driven argument quoting entry points for callers that need a quoted string representation of input text. It exposes two public functions:

- `quotearg_style`
- `quotearg_style_mem`

These functions select a quoting style, apply that style to input text, and return the resulting quoted argument text. The module is part of a broader quoting subsystem centered on `struct quoting_options`, with the public behavior here being the creation and use of style-specific quoting options for string and sized-memory inputs.

The Rust rewrite must preserve the observable behavior of these style-based quoting entry points, including support for both NUL-terminated string input and explicit-length memory input.

## Feature Specification

### Summary

The module supplies convenience interfaces for quoting an argument using a caller-selected `quoting_style`. It must:

- accept a quoting style value,
- apply that style to the provided input,
- support both ordinary C-string style input and explicit byte-length input,
- produce quoted output consistent with the selected style and the underlying quoting options model.

### In-Scope Functionality

1. **Style-based quoting for string input**
   - `quotearg_style` accepts a `quoting_style` and a pointer to character data interpreted as a standard string input.
   - The function returns a quoted representation of the argument using the requested style.

2. **Style-based quoting for memory input**
   - `quotearg_style_mem` accepts a `quoting_style`, a pointer to character data, and an explicit size.
   - The function returns a quoted representation of exactly the provided memory extent, rather than depending on NUL termination.

3. **Use of quoting options as the style carrier**
   - The module behavior is defined in terms of `struct quoting_options`.
   - The selected `quoting_style` must be reflected in the options used for the quoting operation.

4. **Consistency between string and memory entry points**
   - For inputs where the memory form represents the same logical text as the string form, both entry points must yield equivalent quoted output under the same style.

### Out of Scope

The Rust rewrite must not add capabilities not evidenced by this module input, including:

- new public quoting APIs,
- additional style configuration interfaces,
- thread-safety guarantees,
- serialization or persistence behavior,
- FFI requirements,
- performance or benchmark commitments.

## User Scenarios & Testing

### Scenario 1: Quote a standard argument using a selected style

A caller has a normal string argument and needs a quoted form suitable for presentation or further processing under a specific quoting style.

- Input: a `quoting_style` value and a string argument
- Action: call `quotearg_style`
- Expected result: returned text reflects the selected style applied to the full string argument

**Test focus**
- Verify that each supported style value accepted by the module can be passed through this entry point.
- Verify that the output changes according to the chosen style.
- Verify that the full input string is represented in the quoted result.

### Scenario 2: Quote non-NUL-terminated or length-bounded data

A caller has character data that must be quoted using an explicit byte count, such as a buffer slice or text containing embedded terminators outside the requested extent.

- Input: a `quoting_style`, a pointer to data, and a size
- Action: call `quotearg_style_mem`
- Expected result: only the specified memory range is quoted, using the requested style

**Test focus**
- Verify that quoting respects the provided length.
- Verify that data beyond the specified size does not affect output.
- Verify operation on zero-length input.

### Scenario 3: Equivalent string and memory usage

A caller uses either API depending on context but expects identical results when both represent the same text.

- Input: the same textual content passed once as a standard string and once as memory with matching length
- Action: call both `quotearg_style` and `quotearg_style_mem` with the same style
- Expected result: outputs are equivalent

**Test focus**
- Compare output from both functions for matching content and style.
- Include representative inputs containing ordinary characters and characters that require quoting behavior under at least one style.

### Scenario 4: Style selection drives quoting behavior

A caller changes only the quoting style while keeping the input constant.

- Input: one argument quoted under multiple style values
- Action: invoke the style-based API repeatedly with different styles
- Expected result: observable output follows the selected style each time

**Test focus**
- Verify that style is not ignored or fixed.
- Verify that the options backing the operation reflect the requested style.

## Requirements

### Functional Requirements

#### FR-1: Provide a style-based string quoting entry point
The Rust module shall provide behavior equivalent to `quotearg_style` from `quotearg.c:964-968`, accepting a `quoting_style` and a string input and returning the quoted representation of that input.

**Traceability**
- Function: `quotearg_style`
- File: `quotearg.c`

#### FR-2: Provide a style-based memory quoting entry point
The Rust module shall provide behavior equivalent to `quotearg_style_mem` from `quotearg.c:970-974`, accepting a `quoting_style`, a character buffer, and an explicit size, and returning the quoted representation of exactly that input extent.

**Traceability**
- Function: `quotearg_style_mem`
- File: `quotearg.c`

#### FR-3: Apply the caller-specified quoting style
For both public entry points, the Rust module shall ensure that the supplied `quoting_style` determines the quoting behavior used for the returned output.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- Entity: `struct quoting_options`
- File: `quotearg.c`

#### FR-4: Preserve explicit-length semantics for memory input
For the memory-based entry point, the Rust module shall quote based on the provided `argsize` rather than requiring NUL termination.

**Traceability**
- Function: `quotearg_style_mem`
- File: `quotearg.c`

#### FR-5: Preserve equivalence for matching logical inputs
When `quotearg_style` and `quotearg_style_mem` are given the same logical text under the same style, the Rust module shall produce equivalent quoted output.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- File: `quotearg.c`

### Key Entities

#### `struct quoting_options`
This is the module’s core quoting-configuration entity. In this module’s functional boundary, it represents the quoting settings used to perform a quoting operation, including the selected quoting style.

Relationship to module behavior:
- public entry points derive or populate quoting behavior through this options structure,
- the selected `quoting_style` is carried through this entity to control output behavior.

**Traceability**
- Type references: multiple `struct quoting_options` occurrences in `quotearg.c`
- Public functions: `quotearg_style`, `quotearg_style_mem`

#### `enum quoting_style`
This is the style selector consumed by both public functions. It identifies which quoting behavior is requested by the caller.

Relationship to module behavior:
- supplied by the caller,
- transferred into effective quoting options,
- determines the form of returned quoted output.

**Traceability**
- Function signatures: `quotearg_style`, `quotearg_style_mem`
- File: `quotearg.c`

#### `struct slotvec`
This structure appears in the same source file as part of the broader quoting subsystem. Within this module’s boundary, it is only relevant insofar as returned quoted strings belong to the subsystem that manages quoted result storage.

Relationship to module behavior:
- supports the broader returned-string model used by the quoting subsystem,
- not a caller-configured entity in this module’s public interface.

**Traceability**
- Type references: `struct slotvec` occurrences in `quotearg.c`
- Public functions return quoted string pointers from the same subsystem context

## Success Criteria

### SC-1: Public API behavior parity
For representative inputs and style values supported by the source module, the Rust rewrite produces quoted outputs equivalent to the C module for both `quotearg_style` and `quotearg_style_mem`.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`

### SC-2: Correct handling of explicit-length input
Tests demonstrate that the Rust version of the memory entry point quotes exactly the specified number of bytes, including cases where:
- the input length is zero,
- the buffer contains additional data beyond the specified size,
- the logical content would differ if NUL termination were assumed.

**Traceability**
- Function: `quotearg_style_mem`

### SC-3: Style-sensitive output
Tests demonstrate that changing the `quoting_style` for the same input can change the output, confirming that the requested style is actually applied.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- Entity: `struct quoting_options`

### SC-4: Cross-entry-point consistency
Tests demonstrate that when a string input and a memory input represent the same text under the same style, the Rust rewrite returns equivalent quoted output from both entry points.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`

### SC-5: No unsupported feature expansion
The Rust rewrite exposes only the functionality evidenced for this module boundary: style-based quoting of string and explicit-length inputs through the existing quoting-options model.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- Entity: `struct quoting_options`