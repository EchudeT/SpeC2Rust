# spec.md

## Overview

This module defines argument-quoting behavior used by the `pwd` project through configurable quoting options and convenience entry points that produce quoted text from input strings. The Rust rewrite must preserve the module’s observable behavior as evidenced by `quotearg.c`, including:

- creation and copying of quoting option sets,
- inspection and mutation of quoting style and quoting-related flags,
- per-character quoting configuration,
- custom quote delimiter configuration,
- quoting into a caller-provided buffer,
- allocation of newly quoted strings,
- indexed reusable quoting slots for repeated calls,
- style-based convenience wrappers, including variants that force quoting of a specific character.

The module’s functional boundary is limited to quoting configuration and production of quoted string results. It does not define broader command behavior.

## Feature Specification

### Feature: Configurable quoting options

The module provides a configurable `quoting_options` entity that controls how an input argument is quoted.

The Rust version must implement behavior equivalent to the C module for:

- cloning an existing option set,
- reading the currently configured quoting style,
- changing the quoting style,
- enabling or disabling special treatment for an individual character,
- replacing the current quoting flags and reporting the previous value,
- configuring custom left and right quote delimiters.

A quoting option set created from a style must behave as a valid configuration for subsequent quoting operations.

### Feature: Quoting into caller-managed or newly allocated output

The module supports producing quoted output from an input byte sequence or string-like argument by either:

- writing into a caller-provided buffer and returning the required/resulting size, or
- allocating memory for the quoted result and returning that allocated string.

The Rust version must preserve the distinction between:

- quoting with an explicit input length,
- quoting with an output buffer size limit,
- quoting that reports the produced size for allocated results.

### Feature: Reusable indexed quoting results

The module provides indexed quoting operations that return quoted results associated with an integer slot index. These functions are used to obtain quoted strings repeatedly without the caller providing the destination storage each time.

The Rust version must implement the same externally visible slot-based behavior supported by `quotearg_n_options` and the style-based wrappers that build on it.

### Feature: Style-based convenience entry points

The module provides convenience functions that derive quoting behavior directly from a quoting style or from a style plus a forced quoted character.

The Rust version must support the evidenced convenience behaviors:

- quote using a specified style,
- quote using a specified style with explicit input length,
- quote while forcing a given character to be quoted,
- quote using a specified style while forcing `:` to be quoted.

## User Scenarios & Testing

### Scenario 1: A caller clones and adjusts quoting options

A caller starts from an existing quoting option set, clones it, changes the style, enables quoting for a specific character, and uses the modified options to quote an argument.

The Rust version must support tests that verify:

- the clone is independent from the original for subsequent mutations,
- `get_quoting_style` reflects the current style,
- `set_char_quoting` changes whether a chosen character is specially quoted,
- quoting with the modified options produces a result consistent with those changes.

### Scenario 2: A caller uses custom quote delimiters

A caller configures custom left and right quote strings and then quotes an argument through the configured options.

The Rust version must support tests that verify:

- custom delimiters can be applied to an option set,
- quoting with those options uses the configured delimiters,
- invalid custom delimiter input is rejected or otherwise handled consistently with the C module’s contract as evidenced by `set_custom_quoting`.

### Scenario 3: A caller writes quoted output into a fixed buffer

A caller has a preallocated destination buffer and invokes buffer-based quoting for an input with an explicit length.

The Rust version must support tests that verify:

- quoting can be requested with a finite output capacity,
- the returned size matches the module’s quoted output size semantics,
- the function behaves correctly when the buffer is large enough,
- the function behaves correctly when the buffer is smaller than the full quoted output.

### Scenario 4: A caller requests an allocated quoted string

A caller passes an argument and receives a newly allocated quoted result, optionally along with the produced size.

The Rust version must support tests that verify:

- allocated quoting returns the same quoted content as buffer-based quoting for the same input and options,
- the size-reporting variant reports the produced quoted length consistently,
- explicit input lengths are honored, including inputs containing embedded non-terminating bytes if supported by the source interface.

### Scenario 5: A caller reuses indexed quote slots

A caller quotes multiple arguments through indexed APIs, reusing a slot index across calls.

The Rust version must support tests that verify:

- a quoted result can be requested for a specific slot index,
- repeated calls with the same index continue to produce correct current results,
- different slot indices can be used independently from the caller’s perspective.

### Scenario 6: A caller uses style-based wrappers

A caller does not manually build an option set and instead uses convenience wrappers for style-based quoting, including the colon-forcing variant.

The Rust version must support tests that verify:

- style-based wrappers produce results consistent with equivalent manually configured options,
- the explicit-length style wrapper honors the provided input size,
- the single-character-forcing wrapper forces quoting for the requested character,
- the colon wrapper behaves like the single-character-forcing wrapper with `:`.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a quoting-options configuration object equivalent in role to `struct quoting_options` and usable by all quoting entry points.
  **Traceability:** `quotearg.c`, `struct quoting_options`, all listed functions accepting quoting options.

- **FR-2**: The module shall support cloning an existing quoting-options object and returning an independent copy of its configuration state.
  **Traceability:** `clone_quoting_options`.

- **FR-3**: The module shall support reading and updating the quoting style stored in a quoting-options object.
  **Traceability:** `get_quoting_style`, `set_quoting_style`, `quoting_options_from_style`.

- **FR-4**: The module shall support enabling or disabling special quoting treatment for an individual character and shall report the previous setting for that character.
  **Traceability:** `set_char_quoting`.

- **FR-5**: The module shall support replacing the quoting flags associated with a quoting-options object and shall report the previous flags value.
  **Traceability:** `set_quoting_flags`.

- **FR-6**: The module shall support configuring custom left and right quote delimiters on a quoting-options object for subsequent quoting operations.
  **Traceability:** `set_custom_quoting`.

- **FR-7**: The module shall support deriving a valid quoting-options configuration from a quoting style for use by style-based quoting operations.
  **Traceability:** `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_n_style_colon`.

- **FR-8**: The module shall support quoting an input argument into a caller-provided output buffer using a specified quoting-options configuration and an explicit input length.
  **Traceability:** `quotearg_buffer`.

- **FR-9**: The module shall support returning a newly allocated quoted string for a given input argument and quoting-options configuration.
  **Traceability:** `quotearg_alloc`.

- **FR-10**: The module shall support returning a newly allocated quoted string together with the produced quoted size.
  **Traceability:** `quotearg_alloc_mem`.

- **FR-11**: The module shall support indexed quoting operations in which the caller selects a numeric slot and receives the quoted result associated with that slot for the current call.
  **Traceability:** `quotearg_n_options`, `struct slotvec`.

- **FR-12**: The module shall support style-based indexed quoting without requiring the caller to construct a quoting-options object directly.
  **Traceability:** `quotearg_n_style`, `quotearg_n_style_mem`.

- **FR-13**: The module shall support quoting with a configuration that forces a specified character to be quoted.
  **Traceability:** `quotearg_char_mem`.

- **FR-14**: The module shall support quoting with a configuration that forces `:` to be quoted while also applying a specified quoting style.
  **Traceability:** `quotearg_n_style_colon`.

### Key Entities

- **QuotingOptions**
  Functional equivalent of `struct quoting_options`. This entity stores the quoting configuration used by the module, including quoting style, flag state, per-character quoting decisions, and custom quote delimiter configuration. It is consumed by buffer-based, allocated, and indexed quoting operations.

- **QuotingStyle**
  Functional equivalent of `enum quoting_style`. This value selects the general quoting mode applied when converting an input argument into quoted output. It can be read from or written to a quoting-options object and can also be used to derive a fresh options configuration for convenience APIs.

- **Slot Vector / Indexed Quote Storage**
  Functional equivalent of `struct slotvec`. This entity represents the module’s internal association between integer slot indices and reusable quoted results. It underpins `quotearg_n_options` and the wrapper functions built on that entry point.

- **Quoted Result**
  The produced quoted text returned either through a caller-provided buffer, a newly allocated string, or an indexed slot result. Its content is determined by the input argument, explicit input length where provided, and the selected quoting configuration.

Relationships:

- A `QuotingOptions` instance determines how a `Quoted Result` is produced.
- A `QuotingStyle` may be stored inside `QuotingOptions` or converted into a new `QuotingOptions` configuration for convenience calls.
- Indexed quote storage uses a slot index plus a quoting configuration to provide a `Quoted Result` without caller-managed destination storage.

## Success Criteria

- **SC-1**: All public behaviors evidenced by `clone_quoting_options`, `get_quoting_style`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, and `set_custom_quoting` are implemented and covered by automated tests.
  **Traceability:** those functions in `quotearg.c`.

- **SC-2**: For the same input, explicit input length, and quoting options, Rust buffer-based quoting and Rust allocated quoting produce equivalent quoted content.
  **Traceability:** `quotearg_buffer`, `quotearg_alloc`, `quotearg_alloc_mem`.

- **SC-3**: The Rust implementation correctly preserves the size-reporting contract for buffer-based and allocated-memory quoting paths in tests covering both sufficient and insufficient output-buffer capacity.
  **Traceability:** `quotearg_buffer`, `quotearg_alloc_mem`.

- **SC-4**: Style-based wrapper functions produce results equivalent to using an options object derived from the same style.
  **Traceability:** `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`.

- **SC-5**: Character-forcing wrappers correctly alter quoting behavior for the targeted character, including the special colon wrapper.
  **Traceability:** `set_char_quoting`, `quotearg_char_mem`, `quotearg_n_style_colon`.

- **SC-6**: Indexed quoting behavior is validated by tests that reuse at least one slot index across multiple calls and verify correct current results for reused and distinct indices.
  **Traceability:** `quotearg_n_options`, `struct slotvec`.

- **SC-7**: Custom quote delimiter behavior is validated by tests showing that configured delimiters are reflected in produced quoted output.
  **Traceability:** `set_custom_quoting`.