# spec.md

## Title

Rust Functional Specification for `main_root_quoting_options_02`

## Document Metadata

- Project: `cat`
- Module: `main_root_quoting_options_02`
- Category: `main_cluster`
- Source basis: `quotearg.c`
- Primary analyzed function: `quotearg_n_custom_mem`
- Primary analyzed types: `quoting_options`, `slotvec`
- Target branch: `003-main_root_quoting_options_02-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides argument quoting behavior driven by quoting configuration. The analyzed entry point, `quotearg_n_custom_mem`, applies custom left and right quote delimiters to an input byte sequence of known length and returns a quoted representation using per-call quoting options derived from the module’s quoting configuration structure.

The Rust rewrite must preserve the functional role of this module as a quoting helper that:

- accepts an argument as memory plus explicit size,
- applies custom quote delimiters,
- routes behavior through a `quoting_options` configuration object,
- supports indexed/numbered quoting output selection via the `n` parameter,
- and produces quoted output consistent with the source module’s custom-quoting path.

No functionality beyond this evidenced scope is required by this specification.

## Feature Specification

### Feature Summary

The module implements configurable quoting of input data with custom quote markers. Its main supported operation is producing a quoted form of an input buffer using caller-supplied left and right quote strings and an indexed output slot.

### Functional Scope to Preserve

The Rust version must implement the following behavior evidenced by the analyzed source:

1. **Custom delimiter quoting**
   - The caller can provide a left quote string and a right quote string.
   - These custom delimiters are used to quote the provided input buffer.

2. **Length-based input handling**
   - The quoted operation must consume the input according to the explicit `argsize` length rather than requiring NUL-terminated text semantics.
   - The module must therefore support quoting byte sequences whose meaningful extent is defined by length.

3. **Indexed quoting call path**
   - The operation accepts an integer slot/index `n`.
   - The Rust implementation must preserve the functional meaning that quoting is performed for a caller-selected numbered quoting slot/path, as indicated by the source entry point name and signature.

4. **Options-driven behavior**
   - The operation is performed through a `quoting_options` configuration object.
   - For this call path, the options are set to use custom quoting delimiters.

5. **Returned quoted result**
   - The operation produces a quoted string result corresponding to the given input and configuration.

## User Scenarios & Testing

### Scenario 1: Quote a byte buffer with explicit custom delimiters

A caller has an argument buffer and knows its exact byte length. The caller wants the module to return the argument surrounded by a chosen left quote and right quote string.

**Expected support**
- Input is accepted together with its length.
- The output includes the provided left delimiter, the quoted content, and the provided right delimiter.

**Test examples**
- Input bytes representing `abc`, length `3`, left quote `<`, right quote `>` produces a result framed by `<` and `>`.
- Input bytes representing `name with spaces`, length equal to the provided byte count, left quote `[[`, right quote `]]` produces output framed by `[[` and `]]`.

### Scenario 2: Quote input containing embedded non-terminal bytes within a known-size buffer

A caller passes data where the intended content length is controlled by `argsize`, not by scanning for a terminator.

**Expected support**
- The module processes exactly the provided input extent.
- Behavior is defined by the explicit size argument.

**Test examples**
- A test buffer with extra trailing memory beyond `argsize` yields output derived only from the first `argsize` bytes.
- A buffer whose logical input is empty (`argsize == 0`) still yields correctly delimited quoted output.

### Scenario 3: Use different slot numbers for repeated quoting requests

A caller performs multiple quoting operations and passes different `n` values.

**Expected support**
- The Rust module accepts the slot/index argument and produces a valid quoted result for each call.
- The slot/index remains part of the externally visible behavior of the module’s main function.

**Test examples**
- Calls with `n = 0` and `n = 1` both return valid quoted outputs for the same source bytes and delimiters.
- Repeated calls with the same `n` and same inputs produce equivalent quoted content.

### Scenario 4: Use asymmetric delimiters

A caller wants different opening and closing quote strings.

**Expected support**
- The module must support distinct left and right delimiters rather than assuming a single symmetric quote character.

**Test examples**
- Left delimiter `(` and right delimiter `)` frame the result correctly.
- Left delimiter `<<` and right delimiter `>>` frame the result correctly.

## Requirements

### Functional Requirements

#### FR-1: Custom quote delimiters
The module shall accept caller-provided `left_quote` and `right_quote` values and quote the input using those delimiters.

**Traceability**
- Function: `quotearg_n_custom_mem`
- Type: `quoting_options`

#### FR-2: Explicit-length input processing
The module shall process the input argument according to the supplied `argsize` value.

**Traceability**
- Function: `quotearg_n_custom_mem`

#### FR-3: Indexed quoting entry point
The module shall support quoting through a numbered/indexed entry point parameter `n`.

**Traceability**
- Function: `quotearg_n_custom_mem`
- Type: `slotvec`

#### FR-4: Options-based configuration
The module shall represent quoting behavior through a `quoting_options` entity and use that configuration for the custom quoting operation.

**Traceability**
- Type: `quoting_options`
- Function: `quotearg_n_custom_mem`

#### FR-5: Quoted result production
The module shall return a quoted string result corresponding to the given input and quoting configuration.

**Traceability**
- Function: `quotearg_n_custom_mem`
- Type: `slotvec`

### Key Entities

#### `quoting_options`
Configuration entity that controls how an argument is quoted. In this module’s evidenced scope, it must be able to represent custom quoting behavior using caller-supplied left and right quote delimiters.

**Relationships**
- Consumed by the quoting operation.
- Determines how the input buffer is rendered into quoted output.

#### `slotvec`
Entity associated with numbered quoting result handling. In this module’s evidenced scope, it is related to the `n` parameter accepted by the main quoting function.

**Relationships**
- Connects indexed quoting requests to produced output storage/selection behavior.
- Used alongside quoting operations that return a quoted result for a given slot number.

#### Input buffer tuple
The effective input to the quoting operation is the combination of:
- argument pointer/reference,
- explicit byte length `argsize`.

**Relationships**
- Consumed by the quoting operation.
- Rendered according to `quoting_options`.
- Associated with a selected slot/index through `n`.

## Success Criteria

### Functional Correctness

1. **Custom delimiter correctness**
   - For inputs passed through the Rust equivalent of `quotearg_n_custom_mem`, the returned result is framed by the exact provided left and right quote strings.
   - Traceable to: `quotearg_n_custom_mem`, `quoting_options`

2. **Explicit-size correctness**
   - The Rust implementation uses the provided `argsize` as the authoritative input extent for quoting behavior.
   - Tests must demonstrate correct handling for non-empty and zero-length inputs.
   - Traceable to: `quotearg_n_custom_mem`

3. **Indexed call support**
   - The Rust implementation accepts the `n` parameter and successfully returns a quoted result for multiple valid test values.
   - Traceable to: `quotearg_n_custom_mem`, `slotvec`

4. **Asymmetric delimiter support**
   - Tests must verify that different opening and closing delimiters are preserved exactly in the returned quoted result.

5. **Repeatable output behavior**
   - Repeated calls with the same input bytes, length, delimiters, and `n` produce equivalent quoted content.

### Port Completion Criteria

6. **Source-traceable scope adherence**
   - The Rust module implements the custom-memory quoting behavior evidenced by `quotearg_n_custom_mem` and the associated configuration entities without requiring unsupported new module capabilities.
   - Traceable to: `quotearg.c`, `quotearg_n_custom_mem`, `quoting_options`, `slotvec`