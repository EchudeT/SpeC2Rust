# spec.md

## Title

**Functional Specification: `main_root_quoting_options_02`**

- Project: `cat`
- Module category: `main_cluster`
- Source module: `quotearg.c`
- Primary analyzed entry point: `quotearg_n_custom_mem`
- Rust target branch: `003-main_root_quoting_options_02-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides argument quoting behavior driven by quoting configuration. In the analyzed scope, the module constructs a quoted representation of an input byte sequence using caller-supplied left and right quote delimiters, and does so through the module’s configurable quoting options type.

The Rust rewrite must preserve the functional role evidenced here:

- accept an input argument as memory plus explicit length,
- apply a custom quoting style defined by caller-provided quote strings,
- use module quoting options as the controlling configuration entity,
- return the quoted result in the same functional sense as the C module entry point.

This specification is limited to functionality directly evidenced by the analyzed file, function, and referenced data structures. It does not define additional public capabilities beyond that scope.

## Feature Specification

### Feature: Custom-memory argument quoting

The module supports quoting an input argument provided as a byte buffer with an explicit size, using custom left and right quote delimiters supplied by the caller.

Behaviorally, the Rust version must implement:

- creation or use of quoting options representing a custom quoting style,
- association of the supplied left and right quote delimiters with those options,
- quoting of the provided memory region according to that custom style,
- indexed or slot-based result selection corresponding to the `n` argument, as evidenced by the entry point name and the presence of `slotvec`,
- production of a quoted character result suitable for caller consumption.

### Functional boundary

Within this analyzed module scope, the functional boundary is:

- input: slot index, left quote string, right quote string, input pointer, input size,
- internal control entity: `quoting_options`,
- output: quoted string result.

No broader formatting, escaping policy matrix, or unrelated command behavior is specified here unless directly required to support the custom quoting operation.

## User Scenarios & Testing

### Scenario 1: Quote a bounded input with custom delimiters

A caller has an argument in memory whose length is known independently of null termination. The caller requests quoting with specific delimiters, such as an opening marker and a closing marker.

The Rust module must support:

- receiving the input as bytes plus length,
- not requiring the input to be null-terminated,
- wrapping or otherwise quoting the exact provided memory extent using the caller’s custom delimiters,
- returning the quoted result.

**Test focus**
- Input buffer contains exactly `argsize` bytes.
- Output includes the requested left and right quote strings in the expected positions around the quoted argument.
- Only the provided byte range is consumed.

### Scenario 2: Use different slot indices for repeated quoting calls

A caller performs multiple quoting operations and passes an index `n` to identify the result slot to be used.

The Rust module must support:

- accepting the slot index,
- producing a valid quoted result for each requested index,
- preserving the observable behavior that the indexed API variant addresses per-slot result handling, as evidenced by `quotearg_n_custom_mem` and `slotvec`.

**Test focus**
- Calls with different `n` values each return valid quoted strings.
- Repeated calls do not collapse all results into a single indistinguishable storage slot if indexed behavior is externally observable.

### Scenario 3: Apply caller-selected quote strings

A caller needs non-default delimiters and supplies both the left and right quote strings.

The Rust module must support:

- distinct left and right quote strings,
- use of exactly those caller-provided delimiters for the custom quoting style,
- correct handling when the delimiters differ from each other.

**Test focus**
- Left and right delimiters are both reflected in output.
- Different delimiter pairs produce correspondingly different quoted results for the same input.

### Scenario 4: Quote arbitrary memory content by explicit size

A caller passes data that may contain embedded null bytes or may not be terminated.

The Rust module must support quoting based on explicit byte length rather than C-string termination semantics.

**Test focus**
- Inputs with embedded `\0` are accepted as bounded memory.
- The quoted result corresponds to the full specified byte span, not only the prefix before a null byte.

## Requirements

### Functional Requirements

#### FR-1: Custom quoting entry behavior
The Rust module shall provide functionality equivalent to the analyzed entry point `quotearg_n_custom_mem`, accepting:

- a slot/index parameter `n`,
- a left quote string,
- a right quote string,
- an input memory region,
- the size of that memory region.

**Traceability:** `quotearg.c:1020-1028`, function `quotearg_n_custom_mem`

#### FR-2: Explicit-size input handling
The module shall process the input argument as a bounded memory region defined by the supplied size, rather than relying solely on terminator-based string length detection.

**Traceability:** `quotearg.c:1020-1028`, function signature includes `char const *arg, size_t argsize`

#### FR-3: Custom delimiter application
The module shall support caller-defined left and right quote delimiters and apply them through quoting configuration for the produced result.

**Traceability:** `quotearg.c:1020-1028`, parameters `left_quote`, `right_quote`; `quoting_options` references in analyzed file

#### FR-4: Quoting options as controlling configuration
The module shall represent quoting behavior through a quoting-options entity corresponding to `struct quoting_options`, and custom-memory quoting shall be driven by that configuration.

**Traceability:** `struct quoting_options` references throughout `quotearg.c`, including around `1025`

#### FR-5: Indexed result behavior
The module shall preserve the functional meaning of the `n`-indexed quoting API variant, consistent with the analyzed presence of slot-based storage represented by `slotvec`.

**Traceability:** function name/parameter in `quotearg.c:1020-1028`; `struct slotvec` at `quotearg.c:829-833` and related references

#### FR-6: Returned quoted string result
The module shall produce a quoted string result corresponding to the requested input and custom quoting configuration.

**Traceability:** `quotearg.c:1020-1028`, return type is character-pointer string result

### Key Entities

#### `quoting_options`
The core configuration entity controlling how quoting is performed. In this module scope, it is the mechanism by which custom left and right quote delimiters are associated with a quoting operation.

**Role**
- carries quoting style/configuration,
- enables custom quote selection for the active operation.

**Traceability:** multiple `struct quoting_options` references in `quotearg.c`, including around `57-74` and near the analyzed function region

#### `slotvec`
A slot-oriented storage entity associated with indexed quoting results.

**Role**
- supports the `n`-indexed API behavior,
- represents per-slot result management implied by the `quotearg_n_*` family behavior.

**Traceability:** `quotearg.c:829-833` and related `slotvec` references

#### Input memory region
The argument data plus explicit byte count supplied by the caller.

**Role**
- defines the exact bytes to be quoted,
- decouples content length from null termination.

**Traceability:** `quotearg.c:1020-1028`, parameters `arg` and `argsize`

#### Custom quote delimiters
Caller-provided left and right quote strings.

**Role**
- define the opening and closing quotation markers for the operation,
- parameterize the quoting style used for the result.

**Traceability:** `quotearg.c:1020-1028`, parameters `left_quote`, `right_quote`

### Entity Relationships

- The input memory region is quoted under rules determined by `quoting_options`.
- The custom left and right quote delimiters are bound into the quoting configuration for the operation.
- The `n` parameter selects indexed result behavior associated with `slotvec`.
- The final quoted string result is the output of applying the custom quoting configuration to the specified byte range.

## Success Criteria

### SC-1: Functional equivalence for custom-memory quoting
For representative inputs, the Rust implementation produces quoted results functionally equivalent to the C module’s `quotearg_n_custom_mem` for the same:

- slot index,
- left quote string,
- right quote string,
- input bytes,
- input size.

**Traceability:** `quotearg.c:1020-1028`

### SC-2: Exact bounded-input handling
Tests demonstrate that the Rust module processes exactly the specified number of input bytes, including cases with embedded null bytes.

**Traceability:** `quotearg.c:1020-1028`, explicit `argsize` parameter

### SC-3: Custom delimiter correctness
Tests demonstrate that changing the supplied left and right quote strings changes the produced quoted result accordingly, with both delimiters reflected in output.

**Traceability:** `quotearg.c:1020-1028`, `left_quote` and `right_quote` parameters

### SC-4: Indexed API support
Tests demonstrate that the Rust module accepts varying `n` values and preserves the externally relevant indexed behavior implied by the `quotearg_n_*` entry point and slot-based storage model.

**Traceability:** `quotearg.c:1020-1028`; `slotvec` references at `829-833` and related lines

### SC-5: Configuration-centered design fidelity
The Rust rewrite uses a quoting-options concept corresponding to `quoting_options` as the controlling functional entity for custom quoting behavior.

**Traceability:** `quoting_options` references throughout `quotearg.c`

## Out of Scope

The following are not required by this specification unless needed solely to preserve the evidenced behavior of the analyzed entry point:

- new public APIs not evidenced by the analyzed module input,
- thread-safety guarantees,
- serialization or persistence of quoting configuration,
- recovery or retry mechanisms,
- FFI requirements,
- performance or benchmark targets,
- broader command-line behavior of `cat` unrelated to the analyzed quoting function.