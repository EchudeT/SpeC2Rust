# spec.md

## Title

Functional Specification: `main_root_quotearg_n_07`

## Metadata

- **Project**: `cat`
- **Module**: `main_root_quotearg_n_07`
- **Category**: `main_cluster`
- **Source file**: `quotearg.c`
- **Rust target branch**: `008-main_root_quotearg_n_07-rust-port`
- **Generation date**: `2026-06-07`

## 1. Feature Specification

### Overview

This module provides indexed argument-quoting entry points that return a quoted representation of input text. The exposed functionality is centered on three public behaviors:

- quote a NUL-terminated argument using a selected slot index,
- quote a byte sequence with explicit length using a selected slot index,
- quote a NUL-terminated argument using caller-supplied left and right quote delimiters.

The source evidence for this functionality is the public functions:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

The Rust rewrite must preserve the module’s role as a convenience layer for producing quoted argument text while selecting the destination/result context by numeric index.

### Functional Boundary

The Rust version must implement only the functionality evidenced by this module:

1. Accept an integer slot index `n` and input text.
2. Produce quoted output for the provided input.
3. Support both NUL-terminated string input and explicit-length memory input.
4. Support a custom quoting mode where the caller provides left and right quote strings.
5. Use the module’s quoting-option model and slot-based result model as the core behavioral entities because both are directly evidenced by `struct quoting_options` and `struct slotvec`.

No additional public capabilities are required beyond these behaviors.

### Supported Behaviors

#### Indexed quoting of a C-style string

`quotearg_n` quotes an argument given as a NUL-terminated string and associates the produced result with slot index `n`.

Rust behavior requirement:
- The Rust port must support quoting a string-like input under the module’s default option path for a specified slot index.

#### Indexed quoting of explicit-length memory

`quotearg_n_mem` quotes an argument provided as a pointer plus byte length and associates the produced result with slot index `n`.

Rust behavior requirement:
- The Rust port must support quoting byte input where the byte count is explicitly supplied, including cases where the input cannot be modeled solely as a NUL-terminated string.

#### Indexed quoting with caller-provided delimiters

`quotearg_n_custom` quotes a NUL-terminated argument using custom left and right quote delimiters and associates the produced result with slot index `n`.

Rust behavior requirement:
- The Rust port must support a quoting path that uses caller-provided opening and closing quote text rather than only module-default quoting delimiters.

## 2. User Scenarios & Testing

### Scenario 1: Quote a regular argument by slot index

A caller needs a quoted form of a normal argument string and selects slot `n` for the result.

- **Input**: slot index, NUL-terminated argument text
- **Operation**: call the equivalent of `quotearg_n`
- **Expected outcome**: a quoted string is returned for that input, associated with the requested slot index

#### Test focus
- Returned text is not identical to raw unquoted input when quoting rules require delimiters/escaping.
- Different slot indices are accepted and produce valid quoted results.

### Scenario 2: Quote binary or length-delimited input

A caller has input represented by bytes plus an explicit length and needs the same quoting service without depending on NUL termination.

- **Input**: slot index, byte buffer, explicit size
- **Operation**: call the equivalent of `quotearg_n_mem`
- **Expected outcome**: the quoted result is based on exactly the provided byte length

#### Test focus
- The function processes the exact supplied length.
- Input handling does not rely on a trailing NUL byte.
- The returned quoted output is stable for repeated calls with the same bytes and size.

### Scenario 3: Quote using custom delimiters

A caller wants output surrounded by specific left and right quote strings instead of default quoting delimiters.

- **Input**: slot index, custom left quote, custom right quote, NUL-terminated argument text
- **Operation**: call the equivalent of `quotearg_n_custom`
- **Expected outcome**: the returned quoted output uses the caller-provided delimiters

#### Test focus
- The output begins with the provided left delimiter.
- The output ends with the provided right delimiter.
- The input content is quoted under the custom-delimiter path rather than the default-delimiter path.

### Scenario 4: Use multiple indexed results within one execution

A caller performs quoting for more than one argument and uses different slot indices to retrieve independent results.

- **Input**: multiple calls with differing `n` values
- **Operation**: invoke indexed quoting functions repeatedly
- **Expected outcome**: each call yields the quoted form for its own input without collapsing all results into a single indistinguishable slot

#### Test focus
- Calls for slot `0` and slot `1` both succeed.
- Results correspond to the arguments passed for those slots.
- Reusing one slot updates that slot’s observed result for subsequent use.

## 3. Requirements

### Functional Requirements

#### FR-1: Indexed quoting entry point for NUL-terminated input
The module shall provide functionality equivalent to `quotearg_n(int n, char const *arg)` for quoting NUL-terminated argument text by slot index.

**Traceability**: `quotearg.c`, function `quotearg_n`.

#### FR-2: Indexed quoting entry point for explicit-length input
The module shall provide functionality equivalent to `quotearg_n_mem(int n, char const *arg, size_t argsize)` for quoting input defined by byte pointer plus explicit size.

**Traceability**: `quotearg.c`, function `quotearg_n_mem`.

#### FR-3: Indexed custom-delimiter quoting entry point
The module shall provide functionality equivalent to `quotearg_n_custom(int n, char const *left_quote, char const *right_quote, char const *arg)` for quoting NUL-terminated input with caller-specified left and right quote strings.

**Traceability**: `quotearg.c`, function `quotearg_n_custom`.

#### FR-4: Quoting behavior shall be governed by quoting options
The module shall represent and apply quoting configuration through the quoting-options entity used by these entry points.

**Traceability**: `quotearg.c`, `struct quoting_options` references associated with the public entry-point region.

#### FR-5: Result management shall support slot-indexed usage
The module shall maintain the slot-oriented behavior implied by the public `quotearg_n*` API family, using the slot-vector entity as the result organization model.

**Traceability**: `quotearg.c`, `struct slotvec`; functions `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`.

#### FR-6: Custom quoting shall incorporate the supplied delimiters into output formation
When the custom quoting entry point is used, the module shall form output according to the supplied left and right quote strings rather than using only default quoting delimiters.

**Traceability**: `quotearg.c`, function `quotearg_n_custom`; `struct quoting_options`.

#### FR-7: Explicit-length quoting shall honor the supplied size as the input boundary
When the explicit-length entry point is used, the module shall operate on the input extent specified by `argsize`.

**Traceability**: `quotearg.c`, function `quotearg_n_mem`.

### Key Entities

#### `quoting_options`
A configuration entity that defines how quoting is to be performed. This entity is central to all three public entry points and is the behavioral source for default versus customized quoting behavior.

**Role in module**:
- carries quoting configuration,
- enables selection of quoting behavior,
- supports the custom-quote path used by `quotearg_n_custom`.

**Traceability**: `quotearg.c`, `struct quoting_options`.

#### `slotvec`
A slot-oriented storage/organization entity used to support index-based quoting results.

**Role in module**:
- associates result handling with numeric slot indices,
- supports repeated calls through `quotearg_n*` APIs where the caller selects a slot.

**Traceability**: `quotearg.c`, `struct slotvec`.

#### Relationship between entities
`quoting_options` determines how the argument is quoted, while `slotvec` supports where the indexed result is managed for the `quotearg_n*` family.

**Traceability**: `quotearg.c`, public functions and related struct references.

## 4. Success Criteria

### Behavioral Success Criteria

1. **String input support**: The Rust module exposes behavior equivalent to quoting NUL-terminated argument input by slot index, matching the functional role of `quotearg_n`.
   - **Traceability**: `quotearg_n`

2. **Explicit-length input support**: The Rust module exposes behavior equivalent to quoting explicit-length input by slot index, matching the functional role of `quotearg_n_mem`.
   - **Traceability**: `quotearg_n_mem`

3. **Custom delimiter support**: The Rust module exposes behavior equivalent to quoting with caller-provided left and right delimiters, matching the functional role of `quotearg_n_custom`.
   - **Traceability**: `quotearg_n_custom`

4. **Slot-indexed operation preserved**: Calls using different slot indices are accepted and yield valid quoted results consistent with per-slot usage.
   - **Traceability**: `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`, `struct slotvec`

5. **Quoting-options model preserved**: The Rust rewrite includes a quoting-configuration model fulfilling the role evidenced by `struct quoting_options` for the supported entry points.
   - **Traceability**: `struct quoting_options`

6. **Custom delimiter correctness**: In the custom-quote path, output uses the provided left and right quote strings.

7. **Explicit-size correctness**: In the explicit-length path, processing is bounded by the provided input size rather than requiring NUL termination.

### Test Completion Criteria

The rewrite is complete for this module when all of the following are demonstrated:

- Tests cover the three public entry-point behaviors.
- Tests cover at least two distinct slot indices.
- Tests verify custom left/right delimiter use.
- Tests verify explicit-length handling independent of NUL termination.
- No required behavior in this specification depends on undocumented capabilities outside the evidenced module boundary.