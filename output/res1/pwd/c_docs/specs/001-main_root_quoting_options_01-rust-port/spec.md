# spec.md

## Title
Rust Functional Specification for `main_root_quoting_options_01`

## Metadata
- Project: `pwd`
- Module: `main_root_quoting_options_01`
- Category: `main_cluster`
- Source basis: `quotearg.c`
- Target branch: `001-main_root_quoting_options_01-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides configurable argument-quoting behavior centered on a `quoting_options` configuration object and a family of quoting entry points. Its role is to transform input byte strings into quoted output according to a selected quoting style, flag set, optional per-character quoting overrides, and optional custom left/right quote delimiters.

The Rust rewrite must preserve the functional boundary evidenced by `quotearg.c`:

- creation and mutation of quoting configuration,
- quoting into caller-provided buffers,
- quoting into newly allocated output,
- style-based convenience quoting helpers,
- slot-indexed quoting helpers that return stable per-slot results across calls within the module’s quoting interface.

This specification covers only the behavior evidenced by the listed functions and data structures.

## Feature Specification

### 1. Configurable quoting options
The module must support a quoting-options object that governs how an input argument is quoted.

Supported configuration behaviors evidenced by the source functions:
- clone an existing quoting-options object,
- get the current quoting style,
- set the quoting style,
- enable or disable quoting treatment for an individual character,
- set module-defined quoting flags as a whole,
- set custom quote delimiters.

A style-to-options conversion capability must exist internally because style-based helper functions rely on obtaining an options object from a style value.

### 2. Quoting of byte strings
The module must quote an input argument provided as:
- a byte sequence with explicit length, or
- a C-style string in helper forms that imply string termination.

The module must support:
- writing quoted output into a caller-supplied buffer while reporting the produced or required quoted length,
- returning newly allocated quoted output,
- returning newly allocated quoted output together with its resulting size.

The quoting behavior must be controlled by the effective `quoting_options` value supplied to the call.

### 3. Style-based helper quoting
The module must provide helper operations that quote using a quoting style directly, without requiring the caller to manually construct a full options object first.

These helpers must support:
- slot-indexed quoting by style,
- slot-indexed quoting by style with explicit input length,
- style-based quoting with additional colon-character quoting behavior.

### 4. Character-specific quoting customization
The module must support a usage mode where a specific character is forced into quoting treatment through the options path, as evidenced by the character-specific helper and the per-character option setter.

### 5. Custom quote delimiters
The module must support custom left and right quote strings as part of options. When custom quoting is configured, both delimiters are required for valid configuration.

### 6. Slot-indexed result reuse behavior
The module must support quoting operations addressed by an integer slot index. The interface behavior evidenced by `quotearg_n_options` and style-based `quotearg_n_*` wrappers requires that:
- a caller can request a quoted result associated with slot `n`,
- the module manages storage for per-slot quoted strings returned by these helpers,
- repeated use of the same slot is supported by the module interface.

This requirement is limited to preserving externally visible behavior of slot-based quoting results; it does not prescribe any Rust-internal storage design.

## User Scenarios & Testing

### Scenario 1: Clone and adjust quoting configuration
A caller starts from an existing quoting configuration, clones it, changes the style, and uses the clone for quoting without changing the original configuration.

The Rust version must support tests that verify:
- cloned options preserve the original option values,
- later mutation of the clone changes clone behavior only,
- style getter reflects the current style after mutation.

Traceability: `clone_quoting_options`, `get_quoting_style`, `set_quoting_style`, `quoting_options`.

### Scenario 2: Toggle quoting for a specific character
A caller wants an otherwise style-driven quoting operation to additionally treat one chosen character specially.

The Rust version must support tests that verify:
- setting quoting behavior for a character returns the previous state for that character,
- subsequent quoting reflects the updated per-character behavior,
- using the helper dedicated to a single character produces behavior consistent with options configured through the setter.

Traceability: `set_char_quoting`, `quotearg_char_mem`, `quotearg_buffer`, `quoting_options`.

### Scenario 3: Apply a complete flags value
A caller changes the flags field of a quoting configuration and expects the previous flags value to be returned.

The Rust version must support tests that verify:
- setting flags returns the prior flags value,
- quoting with the updated options uses the new flags state,
- replacing flags is whole-value replacement, not additive mutation.

Traceability: `set_quoting_flags`, `quotearg_buffer`, `quoting_options`.

### Scenario 4: Use custom quote delimiters
A caller configures custom left and right delimiters and quotes an argument expecting those delimiters to be used by the custom quoting mode.

The Rust version must support tests that verify:
- custom delimiters can be set on an options object,
- quoting with those options uses the configured delimiter pair,
- invalid custom configuration is rejected in the same situations evidenced by the C API contract that requires both delimiter pointers.

Traceability: `set_custom_quoting`, `quotearg_buffer`, `quoting_options`.

### Scenario 5: Quote into a provided buffer
A caller has a fixed output buffer and needs the quoted form of a given byte sequence written into it.

The Rust version must support tests that verify:
- quoting accepts an explicit input length,
- the function reports the resulting quoted size,
- behavior is correct when the provided buffer size is smaller than the full quoted output,
- behavior is correct when the buffer is large enough for the complete result.

Traceability: `quotearg_buffer`.

### Scenario 6: Quote into owned allocated output
A caller needs a newly allocated quoted string, sometimes also requiring the output size.

The Rust version must support tests that verify:
- allocated-output quoting returns the quoted result,
- the explicit-size variant also returns the output length,
- the two allocation-based entry points produce consistent quoted content for the same input and options.

Traceability: `quotearg_alloc`, `quotearg_alloc_mem`.

### Scenario 7: Use style-based convenience helpers
A caller wants to quote using a style enum directly instead of explicitly building options.

The Rust version must support tests that verify:
- style-based helpers behave consistently with quoting performed using options derived from that same style,
- explicit-length style helpers honor embedded NULs or non-terminated byte input,
- the style-plus-colon helper applies both the selected style and the extra colon treatment.

Traceability: `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_n_style_colon`.

### Scenario 8: Reuse slot-indexed quoting outputs
A caller uses slot-based quoting helpers with multiple slot numbers and expects the module to manage outputs separately by slot.

The Rust version must support tests that verify:
- distinct slot numbers can hold distinct quoted results,
- reusing the same slot replaces or updates that slot’s current result for later return,
- slot-based wrappers delegate consistently to the options-based slot behavior.

Traceability: `quotearg_n_options`, `quotearg_n_style`, `quotearg_n_style_mem`, `slotvec`.

## Requirements

### Functional Requirements

#### FR-1: Quoting options object
The module shall provide a quoting-options abstraction that contains the quoting style and additional quoting controls used by quoting operations.

Traceability: `quoting_options` structure references; `get_quoting_style`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, `set_custom_quoting`.

#### FR-2: Clone behavior
The module shall support creating an independent copy of a quoting-options object.

Traceability: `clone_quoting_options`.

#### FR-3: Style access and mutation
The module shall support reading and replacing the quoting style stored in a quoting-options object.

Traceability: `get_quoting_style`, `set_quoting_style`.

#### FR-4: Per-character quoting control
The module shall support enabling or disabling quoting treatment for an individual character within a quoting-options object and shall report the previous state for that character.

Traceability: `set_char_quoting`.

#### FR-5: Flags replacement
The module shall support replacing the quoting flags value within a quoting-options object and shall report the previous flags value.

Traceability: `set_quoting_flags`.

#### FR-6: Custom quoting delimiters
The module shall support configuring custom left and right quote delimiters within a quoting-options object. The operation shall require both delimiter values to be provided for valid use.

Traceability: `set_custom_quoting`.

#### FR-7: Style-derived options
The module shall support deriving an effective quoting-options value from a quoting style for use by helper quoting entry points.

Traceability: `quoting_options_from_style`.

#### FR-8: Buffer-based quoting
The module shall support quoting an input byte sequence into a caller-provided output buffer using a specified quoting-options value and explicit input length.

Traceability: `quotearg_buffer`.

#### FR-9: Owned-output quoting
The module shall support quoting an input byte sequence into newly allocated owned output using a specified quoting-options value.

Traceability: `quotearg_alloc`, `quotearg_alloc_mem`.

#### FR-10: Report output size
For allocation-based quoting with size reporting, the module shall provide the resulting quoted output size together with the quoted output.

Traceability: `quotearg_alloc_mem`.

#### FR-11: Slot-indexed quoting
The module shall support quoting addressed by integer slot number, using a specified quoting-options value and explicit input length, returning the slot-associated quoted result.

Traceability: `quotearg_n_options`, `slotvec`.

#### FR-12: Style-based slot helpers
The module shall provide slot-based convenience helpers that accept a quoting style directly, including a string-terminated form and an explicit-length form.

Traceability: `quotearg_n_style`, `quotearg_n_style_mem`.

#### FR-13: Character-helper quoting
The module shall provide a helper that quotes an input byte sequence while applying special quoting treatment for a caller-specified character.

Traceability: `quotearg_char_mem`.

#### FR-14: Colon-helper quoting
The module shall provide a style-based helper that applies additional colon quoting behavior.

Traceability: `quotearg_n_style_colon`.

### Key Entities

#### `quoting_options`
The core configuration entity for quoting behavior.

Observed responsibilities:
- stores the selected quoting style,
- stores module-defined flags,
- stores per-character quoting overrides,
- stores custom left/right quote delimiters.

Relationships:
- consumed by buffer-based and allocation-based quoting functions,
- copied by the clone operation,
- synthesized from a style by the style-derived-options helper,
- adjusted by style, flag, character, and custom-delimiter setters.

Traceability: `quoting_options` references across `quotearg.c`; `clone_quoting_options`, `get_quoting_style`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, `set_custom_quoting`, `quotearg_buffer`, `quotearg_alloc`, `quotearg_alloc_mem`, `quotearg_n_options`.

#### Quoting style
An enum-valued selector that determines the base quoting mode.

Relationships:
- stored in `quoting_options`,
- readable and writable through style accessors,
- accepted directly by style-based helpers,
- converted into an effective options object for helper execution.

Traceability: `get_quoting_style`, `set_quoting_style`, `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_n_style_colon`.

#### Slot storage (`slotvec`)
An internal slot-associated storage concept used by slot-indexed quoting functions.

Observed responsibility:
- supports retaining module-managed quoted results by slot number for `quotearg_n_*` interfaces.

Relationships:
- used by slot-indexed options-based quoting,
- indirectly used by style-based slot helpers that route through slot behavior.

Traceability: `slotvec` references; `quotearg_n_options`, `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_n_style_colon`.

## Success Criteria

1. The Rust module exposes behaviorally equivalent support for configurable quoting options, including style access, style mutation, flag replacement, per-character quoting control, and custom quote delimiter configuration.
   - Traceability: `get_quoting_style`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, `set_custom_quoting`.

2. Cloning a quoting-options value yields an independent copy whose subsequent mutation does not alter the source value.
   - Traceability: `clone_quoting_options`.

3. Buffer-based quoting correctly processes explicit-length input and reports quoted size for both sufficient and insufficient destination-buffer capacities.
   - Traceability: `quotearg_buffer`.

4. Allocation-based quoting returns quoted output equivalent in content to buffer-based quoting for the same input and options; the size-reporting variant also returns the output length.
   - Traceability: `quotearg_alloc`, `quotearg_alloc_mem`, `quotearg_buffer`.

5. Style-based convenience helpers produce results consistent with quoting performed through options derived from the same style.
   - Traceability: `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`.

6. The character-specific helper and colon-style helper apply their additional quoting behavior on top of the selected base style as evidenced by the source entry points.
   - Traceability: `quotearg_char_mem`, `quotearg_n_style_colon`.

7. Slot-indexed quoting behavior is preserved such that distinct slots can be used independently and repeated calls with the same slot continue to return the current slot-associated quoted result.
   - Traceability: `quotearg_n_options`, `slotvec`.

8. The Rust rewrite remains within the evidenced module boundary and does not require capabilities beyond those represented by the listed functions and data structures.
   - Traceability: `quotearg.c` module scope.