# spec.md

## Title

Rust Functional Specification for `main_root_quoting_options_01`

## Metadata

- Project: `cat`
- Module: `main_root_quoting_options_01`
- Category: `main_cluster`
- Source basis: `quotearg.c`
- Rust branch: `002-main_root_quoting_options_01-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides configurable argument quoting services centered on a `quoting_options` configuration object and a family of APIs that produce quoted representations of input text.

The Rust rewrite must preserve the functional role evidenced by `quotearg.c`:

- create and clone quoting option sets,
- inspect and modify quoting style and flags,
- configure per-character quoting behavior,
- configure custom quote delimiters,
- render quoted text into a caller-provided buffer,
- allocate quoted results as returned strings/buffers,
- support slot-based repeated quoting calls indexed by `n`,
- provide convenience entry points that quote using a chosen style or force quoting of a particular character.

This specification covers only functionality evidenced by the listed functions and referenced data structures.

## Feature Specification

### Feature: Configurable quoting options

The module must provide a quoting options entity equivalent in role to `struct quoting_options`.

Supported configuration behavior must include:

- obtaining a new options value from a quoting style,
- cloning an existing options object,
- reading the current quoting style,
- changing the quoting style,
- changing quoting flags,
- enabling or disabling special quoting treatment for a specific character,
- installing custom left and right quote strings.

Where an API accepts a nullable/omitted options argument in the C source, the Rust rewrite must preserve the observable behavior of using the module’s default quoting behavior for that operation.

### Feature: Quoted output generation

The module must transform an input argument into a quoted representation according to supplied or default quoting options.

Supported output forms must include:

- writing into a caller-provided buffer and reporting the produced/required size,
- allocating and returning a quoted result,
- allocating and returning a quoted result while also reporting output size.

The module must support input passed with an explicit byte length, not only NUL-terminated text, because multiple entry points accept `arg` together with `argsize`.

### Feature: Indexed reusable quoting results

The module must support the `quotearg_n_options` family behavior evidenced by use of `slotvec`: callers can request quoted output associated with an integer slot index `n`, and repeated calls may reuse storage associated with that slot while returning the quoted content for the latest input/options.

The Rust rewrite must preserve the functional semantics of indexed quoting entry points:

- `quotearg_n_options`
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quotearg_n_style_colon`

This requirement is about externally observable behavior of per-index result access, not about preserving C memory layout.

### Feature: Style-based convenience quoting

The module must provide convenience operations that derive quoting options from a requested quoting style and then quote text using those derived options.

This includes:

- quoting by style for a specific slot,
- quoting by style with explicit input size,
- quoting by style while forcing additional quoting of `:` as evidenced by the colon-specific helper.

### Feature: Character-forcing convenience quoting

The module must provide a convenience operation equivalent in behavior to `quotearg_char_mem`, which quotes an input with explicit size while forcing quoting treatment for one caller-specified character.

## User Scenarios & Testing

### Scenario 1: Clone and adjust quoting configuration

A caller starts from an existing quoting configuration, clones it, changes the style on the clone, and uses both original and clone independently.

Expected behavior:

- cloning produces a distinct options object with equivalent initial settings,
- changing style on the clone does not alter the original,
- subsequent quoting reflects each object’s own settings.

### Scenario 2: Toggle quoting for a specific character

A caller enables quoting treatment for a chosen character using the per-character configuration API, then quotes text containing that character.

Expected behavior:

- the setter reports the previous state for that character,
- quoted output changes in a way consistent with that character now being specially quoted under the current options,
- disabling the same character again restores the prior setting behavior.

### Scenario 3: Install custom delimiters

A caller configures custom left and right quote strings and quotes an input.

Expected behavior:

- the module accepts both delimiters as configuration input,
- output uses the configured custom delimiters rather than the default delimiters for the custom quoting mode/path,
- invalid custom configuration is not silently accepted if the source behavior requires valid non-null delimiters.

### Scenario 4: Render into a caller buffer

A caller provides an output buffer and an input with explicit size.

Expected behavior:

- the module writes the quoted representation into the provided buffer up to buffer capacity,
- the returned size accurately indicates the quoted result length as defined by the source behavior,
- explicit-size input is handled correctly even if the data would not be representable as a simple C string.

### Scenario 5: Allocate quoted output

A caller requests an allocated quoted result using default or supplied options.

Expected behavior:

- the module returns a newly allocated quoted result containing the full quoted text,
- the size-reporting variant also reports the output size consistently with the returned content,
- quoting respects the supplied options or default behavior when options are not supplied.

### Scenario 6: Use slot-based quoting APIs

A caller uses slot `n` to request quoted output multiple times, then uses a different slot.

Expected behavior:

- each call returns the quoted representation for that slot request,
- calls using a different slot do not overwrite the logical latest result associated with another slot,
- style-based slot helpers behave consistently with first constructing options from the requested style and then using slot-based quoting.

### Scenario 7: Quote by style with explicit additional colon handling

A caller uses the colon-specific style helper on input containing `:`.

Expected behavior:

- the helper behaves like style-based quoting plus forced quoting treatment for colon,
- the effect is observable in output for inputs containing `:`.

### Scenario 8: Quote with explicit input length

A caller passes input bytes with an explicit size through memory-oriented APIs.

Expected behavior:

- only the specified input length participates in quoting,
- the operation does not depend on encountering a terminating NUL in the source data,
- convenience helpers that accept `argsize` behave consistently with the general memory-oriented quoting functions.

## Requirements

### Functional Requirements

#### FR-1: Options lifecycle and inspection
The Rust module shall provide functionality corresponding to `clone_quoting_options`, `get_quoting_style`, `set_quoting_style`, and `quoting_options_from_style` for creating, duplicating, reading, and updating quoting configuration.

**Traceability:** `quotearg.c:113-192`

#### FR-2: Per-character quoting configuration
The Rust module shall allow callers to enable or disable special quoting treatment for an individual character and shall return the previous setting for that character, matching the role of `set_char_quoting`.

**Traceability:** `quotearg.c:143-153`

#### FR-3: Quoting flags configuration
The Rust module shall allow callers to replace quoting flags on an options object and shall report the previous flags value, matching the role of `set_quoting_flags`.

**Traceability:** `quotearg.c:159-168`

#### FR-4: Custom quote delimiter configuration
The Rust module shall allow callers to configure custom left and right quote delimiters on an options object, matching the role of `set_custom_quoting`.

**Traceability:** `quotearg.c:170-181`

#### FR-5: Buffer-based quoting
The Rust module shall provide a buffer-oriented quoting operation equivalent in behavior to `quotearg_buffer`, accepting input data, explicit input size, output buffer capacity, and optional quoting options.

**Traceability:** `quotearg.c:779-791`

#### FR-6: Allocating quoting operations
The Rust module shall provide allocating quoting operations equivalent in role to `quotearg_alloc` and `quotearg_alloc_mem`, including the variant that reports output size.

**Traceability:** `quotearg.c:793-826`

#### FR-7: Slot-indexed quoting
The Rust module shall provide functionality equivalent to `quotearg_n_options` in which quoting results are requested by integer slot index and are returned according to the latest content for that slot.

**Traceability:** `quotearg.c:872-923`, `slotvec` references at `quotearg.c:829-845`, `878`

#### FR-8: Style-derived convenience operations
The Rust module shall provide convenience functionality equivalent to `quotearg_n_style` and `quotearg_n_style_mem`, deriving quoting behavior from a requested style and then performing slot-indexed quoting.

**Traceability:** `quotearg.c:949-962`

#### FR-9: Forced-character convenience quoting
The Rust module shall provide convenience functionality equivalent to `quotearg_char_mem`, quoting explicit-size input while forcing quoting behavior for a specified character.

**Traceability:** `quotearg.c:976-983`

#### FR-10: Colon-specific convenience quoting
The Rust module shall provide convenience functionality equivalent to `quotearg_n_style_colon`, applying style-derived quoting with additional colon quoting behavior.

**Traceability:** `quotearg.c:1003-1010`

### Key Entities

#### `quoting_options`
Primary configuration entity controlling quoting behavior.

Observed responsibilities include:

- storing the active quoting style,
- storing quoting flags,
- storing per-character quoting overrides,
- storing custom quote delimiters when configured.

Relationships:

- consumed by buffer-oriented and allocation-oriented quoting functions,
- created directly by cloning or style-derived construction,
- modified by style, flags, character, and custom delimiter setters.

**Traceability:** `quotearg.c:57-74`, functions at `113-192`, uses at `782-810`, `874`, `952`, `960`, `979`, `1006`, `1025`, `1047`

#### `slotvec`
Internal or module-scoped storage entity associated with slot-indexed quoting results.

Observed responsibilities include:

- maintaining per-slot storage for quoted results,
- supporting repeated retrieval/update by integer slot index for `quotearg_n_options` and related helpers.

Relationships:

- used by slot-indexed quoting APIs,
- tied to the logical behavior of per-index reusable result storage.

**Traceability:** `quotearg.c:829-845`, `878`

## Success Criteria

1. The Rust module can create a quoting configuration from a style, clone it, and independently modify clone and original while preserving separate observable quoting behavior.
   - **Traceability:** `clone_quoting_options`, `get_quoting_style`, `set_quoting_style`, `quoting_options_from_style`

2. The Rust module returns previous values when changing per-character quoting and quoting flags, and subsequent quoting reflects the updated settings.
   - **Traceability:** `set_char_quoting`, `set_quoting_flags`

3. The Rust module applies configured custom quote delimiters in produced quoted output where the source behavior requires custom quoting.
   - **Traceability:** `set_custom_quoting`

4. For explicit-size input, buffer-based and allocating APIs produce consistent quoted content for the same input and options.
   - **Traceability:** `quotearg_buffer`, `quotearg_alloc_mem`

5. The size-reporting quoting operations report lengths consistent with the actual quoted output returned or written.

6. Slot-indexed quoting requests distinguish slot indices and preserve the externally observable semantics of per-slot latest quoted result.
   - **Traceability:** `quotearg_n_options`, `slotvec`

7. Style-based convenience APIs produce the same quoting result as constructing options from the same style and using the corresponding general quoting path.
   - **Traceability:** `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`

8. The forced-character and colon-specific convenience APIs produce observable output differences when the specified character appears in the input and would otherwise not be quoted the same way.
   - **Traceability:** `quotearg_char_mem`, `quotearg_n_style_colon`

9. All scenarios in this document are covered by Rust tests using inputs with explicit byte lengths as well as ordinary string inputs where applicable.
   - **Traceability:** memory-oriented quoting functions and style/slot convenience functions listed above