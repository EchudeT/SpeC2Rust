# spec.md

## Title

Functional Specification: `main_root_quoting_options_01`

## Metadata

- Project: `cat`
- Module: `main_root_quoting_options_01`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Rust branch target: `002-main_root_quoting_options_01-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides configurable argument quoting services centered on a `quoting_options` configuration object and a set of quoting entry points that produce quoted text in caller-provided buffers, newly allocated memory, or reusable numbered slots.

The Rust rewrite must preserve the module’s externally visible behavior evidenced by the analyzed functions:

- creating and copying quoting option sets
- reading and updating quoting style and flags
- enabling per-character quoting
- configuring custom quote delimiters
- deriving option sets from a quoting style
- producing quoted output into a buffer
- producing quoted output via allocation
- producing quoted output through reusable numbered quoting slots
- providing style-based and character-based convenience quoting helpers

## Feature Specification

### Feature: Configurable quoting behavior

The module defines a configurable quoting policy through `quoting_options`.

The Rust version must implement behavior equivalent to the C module’s supported option manipulation operations:

- clone an existing quoting option set
- retrieve the current quoting style from an option set
- update the quoting style in an option set
- enable or disable quoting treatment for a specific character
- update quoting flags on an option set
- configure custom left and right quote strings

A null or omitted options argument is treated by the C interface as using default behavior where applicable; the Rust rewrite must preserve the effective behavior of default-option use at all public quoting entry points that accept an optional or nullable options input in C.

### Feature: Quoting style selection

The module supports quoting according to a selected `quoting_style` value.

The Rust version must support:

- constructing effective quoting behavior from a style alone
- quoting using explicit options
- quoting using style-based convenience functions

Where the C module exposes helpers that build option state from style and immediately quote, the Rust rewrite must preserve the same observable quoting result for the same style and input.

### Feature: Buffer-based quoting

The module can write quoted text into a caller-provided output buffer.

The Rust version must support the equivalent of `quotearg_buffer` behavior:

- consume an input byte sequence and a quoting configuration
- write a quoted representation into provided output storage
- report the resulting quoted length as a signed size result
- operate correctly when the input size is explicitly provided, including for data that is not limited to NUL-terminated strings

### Feature: Allocating quoting APIs

The module can return newly allocated quoted text.

The Rust version must support equivalent behavior for:

- allocation of a quoted string from a C-style string input
- allocation of a quoted string from an explicit byte length input
- reporting the output size when requested by the caller

The allocated result must contain the quoted representation dictated by the chosen options or style.

### Feature: Reusable slot-based quoting

The module maintains reusable numbered storage slots for quoted results through `quotearg_n_options` and style helpers layered on it.

The Rust version must preserve the functional behavior that:

- a caller can request quoted output associated with a numeric slot index
- repeated use of a slot yields a valid quoted result for the latest input passed to that slot
- style-based helpers using slot numbers behave consistently with quoting through explicit options derived from that style

This requirement is limited to preserving externally visible slot-based result behavior evidenced by the functions provided; it does not require reproducing internal C memory layout.

### Feature: Convenience helpers

The module includes convenience operations that specialize the base quoting machinery:

- quote using a slot number plus style
- quote using a slot number plus style and explicit input length
- quote while forcing quoting treatment for one specific character
- quote using a style and additionally force quoting of `:`

The Rust version must preserve the observable output behavior of these helpers by applying the corresponding option changes before quoting.

## User Scenarios & Testing

### Scenario 1: Copy and modify quoting options

A caller starts from an existing option set, clones it, changes the style, and updates flags or character quoting on the clone without altering the original.

The Rust version must support tests that verify:

- cloning creates an independent option set
- reading style from the original and clone matches initially
- changing style, flags, or per-character quoting on the clone changes quoting output produced with the clone
- the original option set’s quoting behavior remains unchanged

Traceability:
- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`

### Scenario 2: Use custom quote delimiters

A caller configures custom left and right quote strings and quotes an argument using those settings.

The Rust version must support tests that verify:

- custom delimiters are accepted as part of the option set
- output reflects the configured delimiters
- using custom delimiters is tied to the option set on which they were configured

Traceability:
- `set_custom_quoting`
- `quotearg_buffer`
- `quotearg_alloc`
- `quotearg_alloc_mem`

### Scenario 3: Quote into a caller-owned buffer

A caller provides a buffer and explicit input length to receive a quoted form of input data.

The Rust version must support tests that verify:

- quoted output is written according to the supplied options
- the function returns the quoted result length
- explicit input lengths are honored, including inputs containing embedded NUL bytes

Traceability:
- `quotearg_buffer`

### Scenario 4: Quote by allocating a new result

A caller requests a newly allocated quoted representation of an input string or byte sequence.

The Rust version must support tests that verify:

- the returned result matches the same quoting policy as buffer-based quoting
- the memory-returning variant with size reporting returns the quoted size consistent with the produced output
- explicit-length input produces the same result as buffer-based quoting for the same bytes and options

Traceability:
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quotearg_buffer`

### Scenario 5: Quote using style-only helpers

A caller does not manually construct options, but instead requests quoting by providing a style and slot number.

The Rust version must support tests that verify:

- style-based helpers produce the same quoted result as using options derived from the same style
- the explicit-length style helper honors the provided input length

Traceability:
- `quoting_options_from_style`
- `quotearg_n_style`
- `quotearg_n_style_mem`

### Scenario 6: Quote while forcing one character to be quoted

A caller needs a normally valid character to be treated as requiring quoting, either for an arbitrary chosen character or specifically for `:`.

The Rust version must support tests that verify:

- forcing a target character changes output when that character appears in the input
- the colon-specialized helper behaves like applying the corresponding per-character quoting option and then quoting with the selected style

Traceability:
- `set_char_quoting`
- `quotearg_char_mem`
- `quotearg_n_style_colon`

### Scenario 7: Reuse numbered quoting slots

A caller requests quoted results through the same numeric slot across multiple calls.

The Rust version must support tests that verify:

- each slot request returns a valid quoted result
- later quoting through the same slot reflects the latest input for that slot
- use of one slot does not corrupt the visible result associated with another slot in the same test flow

Traceability:
- `quotearg_n_options`
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `slotvec`

## Requirements

### Functional Requirements

#### FR-1: Option set cloning
The module shall provide an operation that creates a copy of an existing `quoting_options` configuration such that subsequent changes to the copy do not alter the source configuration.

Traceability:
- `clone_quoting_options`
- `quoting_options`

#### FR-2: Style inspection and update
The module shall provide operations to read the current `quoting_style` from a `quoting_options` object and to replace that style with another one.

Traceability:
- `get_quoting_style`
- `set_quoting_style`
- `quoting_options`

#### FR-3: Per-character quoting control
The module shall provide an operation to enable or disable special quoting treatment for an individual character within a `quoting_options` object and to return the previous setting for that character.

Traceability:
- `set_char_quoting`
- `quoting_options`

#### FR-4: Quoting flag update
The module shall provide an operation to replace or update the option set’s quoting flags and to return the previous flags value.

Traceability:
- `set_quoting_flags`
- `quoting_options`

#### FR-5: Custom delimiter configuration
The module shall provide an operation to configure custom left and right quote delimiters on a `quoting_options` object for use by quoting operations that honor custom quoting.

Traceability:
- `set_custom_quoting`
- `quoting_options`

#### FR-6: Style-derived option behavior
The module shall support deriving effective quoting behavior from a `quoting_style` value for use by style-oriented helper entry points.

Traceability:
- `quoting_options_from_style`
- `quoting_style`
- `quoting_options`

#### FR-7: Buffer output quoting
The module shall provide quoting into caller-supplied output storage from input data supplied with an explicit size and a quoting configuration, returning the resulting quoted length.

Traceability:
- `quotearg_buffer`

#### FR-8: Allocated output quoting
The module shall provide functions that return newly allocated quoted output for:
- string-like input
- explicit-length input
- explicit-length input with output size reporting

Traceability:
- `quotearg_alloc`
- `quotearg_alloc_mem`

#### FR-9: Slot-indexed quoted result reuse
The module shall provide slot-indexed quoting that associates quoted output with an integer slot number and returns the quoted result for that slot using specified options.

Traceability:
- `quotearg_n_options`
- `slotvec`

#### FR-10: Style-based slot helpers
The module shall provide convenience helpers that perform slot-indexed quoting based on a style alone, including a variant that accepts explicit input length.

Traceability:
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quoting_options_from_style`

#### FR-11: Character-forcing convenience helper
The module shall provide a helper that quotes explicit-length input while forcing one specified character to be quoted.

Traceability:
- `quotearg_char_mem`
- `set_char_quoting`

#### FR-12: Colon-forcing style helper
The module shall provide a helper that performs style-based slot quoting while forcing quoting treatment for the `:` character.

Traceability:
- `quotearg_n_style_colon`
- `set_char_quoting`

### Key Entities

#### `quoting_options`
Configuration object that determines how an input argument is transformed into a quoted representation.

Observed responsibilities include:
- storing the selected quoting style
- storing quoting flags
- storing per-character quoting overrides
- storing custom left and right quote delimiters

Relationships:
- consumed by buffer-based and allocation-based quoting functions
- produced by cloning and style-derived construction
- modified by style, flag, character, and custom delimiter setters

Traceability:
- `quoting_options`
- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quoting_options_from_style`

#### `quoting_style`
An enumeration selecting a quoting mode used directly or embedded within `quoting_options`.

Relationships:
- read from and written to `quoting_options`
- used to derive effective option sets for style-based quoting helpers

Traceability:
- `get_quoting_style`
- `set_quoting_style`
- `quoting_options_from_style`
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quotearg_n_style_colon`

#### `slotvec`
Internal slot-tracking entity used to support numbered reusable quoted result storage.

Relationships:
- used by slot-indexed quoting entry points
- associates an integer slot identifier with stored quoted output state

Traceability:
- `slotvec`
- `quotearg_n_options`

## Success Criteria

1. The Rust module exposes functionality equivalent to all analyzed public behaviors of this module: option cloning, option mutation, buffer quoting, allocation quoting, slot-based quoting, and the documented convenience helpers.
   - Traceability: all listed functions in the analyzed interface

2. For the same input bytes, explicit input length, and equivalent quoting configuration, buffer-based quoting and allocation-based quoting produce matching quoted content.
   - Traceability: `quotearg_buffer`, `quotearg_alloc`, `quotearg_alloc_mem`

3. A cloned option set can be modified without changing the quoting behavior produced from the original option set in comparative tests.
   - Traceability: `clone_quoting_options`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, `set_custom_quoting`

4. Style-based helper functions produce the same observable quoted result as quoting with an option set derived from the same style.
   - Traceability: `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`

5. The explicit-length quoting paths correctly process inputs containing embedded NUL bytes, as demonstrated by tests comparing expected quoted content and reported length.
   - Traceability: `quotearg_buffer`, `quotearg_alloc_mem`, `quotearg_n_style_mem`, `quotearg_char_mem`

6. Per-character quoting overrides measurably affect output when the target character occurs in the input, including the dedicated colon helper.
   - Traceability: `set_char_quoting`, `quotearg_char_mem`, `quotearg_n_style_colon`

7. Slot-indexed quoting supports repeated reuse of slot numbers in tests without losing correctness of the latest result for each exercised slot.
   - Traceability: `quotearg_n_options`, `quotearg_n_style`, `quotearg_n_style_mem`, `slotvec`

8. Operations that report prior state when changing character quoting or quoting flags return values consistent with the immediately preceding configuration in tests.
   - Traceability: `set_char_quoting`, `set_quoting_flags`