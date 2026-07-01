# spec.md

## Title

Functional Specification: `main_root_quotearg_style_14`

## Metadata

- Project: `pwd`
- Module: `main_root_quotearg_style_14`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Rust branch: `014-main_root_quotearg_style_14-rust-port`
- Generation date: 2026-06-09

## Overview

This module provides style-based argument quoting entry points for converting an input argument into a quoted string according to a selected quoting style. It exposes two public behaviors:

- quote a NUL-terminated string using a specified quoting style
- quote a byte sequence of explicit length using a specified quoting style

The Rust rewrite must preserve the observable behavior of these style-based quoting operations and their dependence on the module’s quoting option model.

## Feature Specification

### Purpose

The module supplies convenience functions that produce a quoted representation of input text based on an `enum quoting_style` selection. These functions act as style-selecting wrappers over the module’s broader quoting machinery and use the module’s quoting options model to determine output behavior.

### In-Scope Functionality

The Rust version must implement:

1. **Style-selected quoting for NUL-terminated input**
   - Accept a quoting style and a string argument.
   - Return the quoted form of that argument as determined by the selected style.

2. **Style-selected quoting for explicit-length input**
   - Accept a quoting style, a pointer to input bytes, and an explicit byte length.
   - Return the quoted form of exactly that byte range as determined by the selected style.

3. **Use of quoting option state derived from style**
   - The selected `quoting_style` must determine the quoting behavior through the module’s `quoting_options` model.

4. **Compatibility with the module’s reusable quoting storage behavior**
   - Since the public functions return a character pointer and are tied to `slotvec` storage in the analyzed source, the Rust rewrite must preserve equivalent observable semantics for repeated use through these entry points within the Rust design chosen for the port.

### Out of Scope

The specification does not require any capabilities not evidenced by this module slice, including:
- new public APIs beyond the two identified entry points
- guarantees about thread safety
- serialization or persistence
- FFI-specific interfaces
- performance or benchmark targets

## User Scenarios & Testing

### Scenario 1: Quote a normal string using a selected style

A caller has a standard string argument and wants a quoted representation using one of the supported quoting styles.

**Expected behavior**
- The module accepts the style and the string.
- The module returns a quoted string corresponding to that style.
- The returned result reflects the style-selected quoting rules rather than raw passthrough unless the selected style itself implies minimal or no added quoting.

**Test coverage**
- Call the Rust equivalent of `quotearg_style` with representative non-empty strings.
- Verify the output changes appropriately when the style value changes.
- Verify the result is a valid quoted rendering for the selected style.

### Scenario 2: Quote a byte range that may not be NUL-terminated

A caller has input data defined by pointer plus explicit size and needs only that exact byte range quoted.

**Expected behavior**
- The module reads exactly the provided byte count.
- Embedded or trailing bytes outside the specified range do not affect the result.
- The returned quoted output corresponds to the selected style.

**Test coverage**
- Call the Rust equivalent of `quotearg_style_mem` with explicit-length inputs.
- Include cases where the logical input length is shorter than any backing buffer.
- Include inputs containing interior NUL bytes if representable in the Rust test design, and verify the result reflects the specified length rather than C-string termination rules.

### Scenario 3: Same content through both entry points when lengths agree

A caller uses the string-based API for ordinary text in one place and the explicit-length API for the same text in another.

**Expected behavior**
- For data without interior NUL and with `argsize` equal to the string length, both entry points produce equivalent quoted output for the same style.

**Test coverage**
- For multiple styles, compare output from the Rust equivalents of:
  - string-based quoting on `"abc"`
  - explicit-length quoting on the bytes of `"abc"` with length `3`

### Scenario 4: Repeated calls with different styles

A caller repeatedly requests quoting of arguments under different styles.

**Expected behavior**
- Each call uses the style passed to that call.
- Earlier calls do not force later calls to reuse the wrong style.
- Returned results remain behaviorally consistent with the module’s managed quoting state model.

**Test coverage**
- Make consecutive calls with alternating style values and different inputs.
- Verify each result matches the style for its own call.

## Requirements

### Functional Requirements

#### FR-1: Style-based quoting for string input
The module shall provide behavior equivalent to `quotearg_style(enum quoting_style s, char const *arg)` from `quotearg.c:964-968`, producing a quoted result for a NUL-terminated input string according to the specified quoting style.

**Traceability**
- Function: `quotearg_style`
- File: `quotearg.c`

#### FR-2: Style-based quoting for explicit-length input
The module shall provide behavior equivalent to `quotearg_style_mem(enum quoting_style s, char const *arg, size_t argsize)` from `quotearg.c:970-974`, producing a quoted result for exactly the specified input byte range according to the specified quoting style.

**Traceability**
- Function: `quotearg_style_mem`
- File: `quotearg.c`

#### FR-3: Style selection shall govern quoting behavior through quoting options
The module shall map the supplied `quoting_style` to the quoting behavior represented by `struct quoting_options`, and the produced output shall reflect that selected style.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- Type: `struct quoting_options`
- File: `quotearg.c`

#### FR-4: String-input quoting and explicit-length quoting shall agree for equivalent inputs
When the explicit-length input corresponds exactly to a NUL-terminated string’s content and length, the two public entry points shall produce equivalent quoted output for the same quoting style.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- File: `quotearg.c`

#### FR-5: Explicit-length quoting shall not depend on NUL termination
For the explicit-length entry point, output generation shall be based on the provided `argsize`, not on scanning for a terminating NUL byte.

**Traceability**
- Function: `quotearg_style_mem`
- File: `quotearg.c`

#### FR-6: Returned quoted results shall follow the module’s managed result-storage model
Because the source module ties quoting results to `struct slotvec` and returns `char *`, the Rust rewrite shall preserve the externally observable semantics of obtaining a quoted result through module-managed storage or an equivalent Rust-owned result representation chosen for the port.

**Traceability**
- Functions: `quotearg_style`, `quotearg_style_mem`
- Type: `struct slotvec`
- File: `quotearg.c`

### Key Entities

#### `enum quoting_style`
A style selector that determines which quoting rules are applied to the input.

**Relationship**
- Consumed by both public functions.
- Drives configuration represented by `quoting_options`.

#### `struct quoting_options`
The module’s quoting configuration structure. The analysis shows multiple references to this structure throughout `quotearg.c`, indicating it is the central state model for how quoting is performed.

**Relationship**
- Derived from or configured by the selected `quoting_style`.
- Used as the behavioral basis for the quoting performed by the public entry points.

#### `struct slotvec`
A storage-management structure associated with quoted result handling in this module.

**Relationship**
- Supports the returned quoted result semantics for the public functions.
- Connects module quoting operations to reusable or managed output storage.

## Success Criteria

1. **Public behavior coverage**
   - The Rust module exposes equivalents for the two evidenced behaviors: style-based quoting of NUL-terminated input and style-based quoting of explicit-length input.
   - Traceability: `quotearg_style`, `quotearg_style_mem`

2. **Correct style sensitivity**
   - For at least two distinct `quoting_style` values and the same input, tests demonstrate that the Rust implementation selects behavior according to the requested style.
   - Traceability: `quotearg_style`, `quotearg_style_mem`, `struct quoting_options`

3. **Correct explicit-length handling**
   - Tests show that the explicit-length variant quotes exactly the specified byte range and does not rely on NUL termination.
   - Traceability: `quotearg_style_mem`

4. **API consistency across equivalent inputs**
   - Tests show that, for inputs without interior NUL and with matching length, the string-based and explicit-length variants produce equivalent output under the same style.

5. **Result delivery compatibility**
   - The Rust port returns quoted results in a way that preserves the observable contract of obtaining quoted text from each call, consistent with the source module’s use of managed result storage.
   - Traceability: `quotearg_style`, `quotearg_style_mem`, `struct slotvec`

6. **No unsupported feature expansion**
   - The Rust rewrite does not require new public functionality beyond the evidenced style-based quoting behaviors and their supporting option/storage relationships.
   - Traceability: `quotearg_style`, `quotearg_style_mem`, `struct quoting_options`, `struct slotvec`