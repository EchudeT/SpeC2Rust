# spec.md

## Title
Rust Functional Specification for `main_root_quoting_options_01`

## Metadata
- Project: `cat`
- Module: `main_root_quoting_options_01`
- Category: `main_cluster`
- Source basis: `quotearg.c`
- Target branch: `002-main_root_quoting_options_01-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides configurable argument quoting behavior centered on a `quoting_options` configuration object and a set of formatting entry points that produce quoted text from byte or string input.

The Rust rewrite must preserve the module’s observable behavior as evidenced by the analyzed source:
- creating and cloning quoting option sets,
- reading and mutating quoting configuration,
- constructing configuration from a quoting style,
- producing quoted output into caller-provided buffers,
- producing allocated quoted strings,
- producing quoted strings through numbered reusable slots,
- supporting style-based, custom-quote-based, and character-specific quoting adjustments.

This specification covers only the functional boundaries evidenced by `quotearg.c`.

## Feature Specification

### Feature: Configurable quoting options
The module shall provide a quoting configuration entity equivalent in role to `struct quoting_options`.

Supported configuration operations shall include:
- cloning an existing option set,
- reading the active quoting style,
- changing the active quoting style,
- changing quoting behavior for a specific character,
- changing quoting flags,
- setting custom left and right quote delimiters.

The module shall also support creating an option set derived from a specified quoting style.

### Feature: Quoted output generation
The module shall generate quoted output for input text using a supplied or derived quoting configuration.

Supported output modes shall include:
- writing quoted output into a caller-provided buffer while reporting the resulting size,
- allocating and returning a quoted string,
- allocating and returning a quoted string while also reporting the output size.

The module shall accept both:
- input treated as a C-style string in APIs that do not take an explicit length, and
- input treated as memory with an explicit byte length in APIs that do.

### Feature: Reusable numbered quoting slots
The module shall support generation of quoted strings through numbered slots, where a slot index identifies reusable storage for returned quoted output.

The Rust rewrite must preserve the externally visible behavior of slot-based quoting APIs:
- obtaining quoted output for a specified slot number,
- obtaining quoted output for a slot number using a specified quoting style,
- obtaining quoted output for a slot number using a specified quoting style and explicit input length.

### Feature: Convenience quoting variants
The module shall support convenience entry points that derive quoting behavior from narrower inputs:
- quoting with a given style,
- quoting with a given style and explicit input length,
- quoting with an additional character-specific quoting rule,
- quoting with style plus colon-sensitive behavior as defined by the source module’s option composition.

## User Scenarios & Testing

### Scenario 1: Clone and adjust a quoting configuration
A caller starts from an existing quoting configuration, clones it, changes the style, adjusts flags, and changes whether a specific character is quoted.

The Rust module must support:
- creating an independent cloned configuration,
- reading back the changed style,
- observing that character-specific and flag changes apply to output generated with that modified configuration.

### Scenario 2: Apply custom quote delimiters
A caller configures custom left and right quote strings, then quotes an argument using that configuration.

The Rust module must support:
- storing custom quote delimiters in the quoting configuration,
- using those delimiters in generated quoted output.

### Scenario 3: Quote into a caller-owned buffer
A caller has a destination buffer and wants quoted output without requesting module-owned allocation.

The Rust module must support:
- producing quoted output into the supplied buffer,
- reporting the total quoted size,
- handling explicit input lengths for memory-based input.

### Scenario 4: Quote by allocation
A caller provides an input argument and receives an allocated quoted string, optionally with the resulting size.

The Rust module must support:
- returning the quoted output as newly allocated storage,
- returning the output size when requested,
- handling both string-style and explicit-length input forms.

### Scenario 5: Use a numbered reusable slot
A caller repeatedly quotes different arguments into the same slot number and expects the API to return the current quoted string for that slot.

The Rust module must support:
- addressing output storage by slot number,
- updating the slot’s content when called again for the same slot,
- supporting slot-based quoting with explicit options and with style-derived options.

### Scenario 6: Use style-based convenience entry points
A caller wants quoting behavior without manually constructing a full option object.

The Rust module must support:
- quoting by style alone,
- quoting by style with explicit input length,
- quoting by style with colon-related behavior where provided by the source API.

### Scenario 7: Force quoting for one character
A caller wants a specific character to be quoted even when using otherwise standard quoting behavior.

The Rust module must support:
- deriving behavior from a default or style-based option set,
- enabling quoting for one designated character,
- producing output that reflects that character-specific rule.

### Testing expectations
The Rust rewrite should be tested with scenarios covering:
- option cloning independence,
- style get/set round trips,
- flag replacement behavior,
- character-specific quoting rule replacement behavior,
- custom delimiter application,
- buffer output size reporting,
- allocated output size reporting,
- slot reuse by index,
- style-derived convenience APIs,
- explicit-length handling for inputs containing non-terminal bytes before the specified length limit.

## Requirements

### Functional Requirements

#### FR-1: Quoting configuration lifecycle
The module shall provide a quoting configuration object corresponding to `quoting_options` and support cloning of an existing configuration into a distinct object.

Traceability:
- `clone_quoting_options`
- `quoting_options`

#### FR-2: Quoting style inspection and mutation
The module shall allow callers to read the current quoting style from a configuration and replace that style with another supported style value.

Traceability:
- `get_quoting_style`
- `set_quoting_style`
- `quoting_options`

#### FR-3: Character-specific quoting control
The module shall allow callers to set quoting behavior for an individual character within a configuration and shall report the previous setting for that character.

Traceability:
- `set_char_quoting`
- `quoting_options`

#### FR-4: Quoting flag replacement
The module shall allow callers to replace the configuration’s quoting flags and shall report the previous flag value.

Traceability:
- `set_quoting_flags`
- `quoting_options`

#### FR-5: Custom quote delimiter configuration
The module shall allow callers to set custom left and right quote delimiters in a configuration for use by generated output.

Traceability:
- `set_custom_quoting`
- `quoting_options`

#### FR-6: Style-derived configuration creation
The module shall support producing a quoting configuration from a specified quoting style for use by higher-level quoting operations.

Traceability:
- `quoting_options_from_style`

#### FR-7: Buffer-based quoted output
The module shall generate quoted output into a caller-provided buffer using a supplied quoting configuration and shall return the size of the quoted result.

Traceability:
- `quotearg_buffer`
- `quoting_options`

#### FR-8: Allocating quoted output
The module shall allocate and return quoted output for an input argument using a supplied quoting configuration.

Traceability:
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quoting_options`

#### FR-9: Size-reporting allocated output
For the explicit-length allocating variant, the module shall report the size of the generated quoted output through the provided size output parameter.

Traceability:
- `quotearg_alloc_mem`

#### FR-10: Slot-indexed quoted output
The module shall support quoting into reusable storage selected by a numeric slot index and return the quoted string associated with that slot.

Traceability:
- `quotearg_n_options`
- `slotvec`

#### FR-11: Style-based slot quoting
The module shall support slot-indexed quoting where the quoting configuration is derived from a provided style value.

Traceability:
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quoting_options_from_style`

#### FR-12: Character-specific convenience quoting
The module shall support a convenience API that quotes input using behavior modified to force quoting treatment for a specified character.

Traceability:
- `quotearg_char_mem`
- `set_char_quoting`

#### FR-13: Colon-related style convenience quoting
The module shall support a convenience API combining a provided style with additional colon-related option composition as represented by the source API.

Traceability:
- `quotearg_n_style_colon`
- `set_char_quoting`
- `quoting_options_from_style`

#### FR-14: Explicit-length input handling
APIs that accept an input length shall process the input according to that explicit length rather than requiring NUL-terminated input.

Traceability:
- `quotearg_buffer`
- `quotearg_alloc_mem`
- `quotearg_n_options`
- `quotearg_n_style_mem`
- `quotearg_char_mem`

### Key Entities

#### `quoting_options`
The core configuration entity controlling how quoted output is produced.

Observed responsibilities:
- stores the active quoting style,
- stores quoting flags,
- stores character-specific quoting state,
- stores custom left and right quote delimiters when configured.

Relationships:
- consumed by buffer-based and allocation-based quoting functions,
- created directly or derived from a style,
- mutated by style, flag, character, and custom-delimiter setters.

Traceability:
- `quoting_options`
- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quotearg_buffer`
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quotearg_n_options`

#### `slotvec`
An internal storage concept used to associate reusable quoted output buffers with slot indices.

Observed responsibilities:
- backs slot-indexed quoting results,
- allows repeated use of a slot number across calls.

Relationships:
- used by slot-based quoting operations,
- holds per-slot output storage rather than quoting rules.

Traceability:
- `slotvec`
- `quotearg_n_options`

## Success Criteria

1. A cloned quoting configuration can be modified without changing the original configuration’s style, flags, character-specific settings, or custom delimiters.
   - Traceability: `clone_quoting_options`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, `set_custom_quoting`

2. Reading a configuration’s style after setting it returns the value most recently assigned.
   - Traceability: `get_quoting_style`, `set_quoting_style`

3. Setting character-specific quoting returns the previous setting for that character and changes subsequent quoted output accordingly.
   - Traceability: `set_char_quoting`, `quotearg_buffer`, `quotearg_char_mem`

4. Setting quoting flags returns the previous flags and changes subsequent quoted output accordingly where flags affect formatting.
   - Traceability: `set_quoting_flags`, `quotearg_buffer`

5. Setting custom left and right quote delimiters causes quoted output generated with that configuration to use those delimiters.
   - Traceability: `set_custom_quoting`, `quotearg_buffer`, `quotearg_alloc`, `quotearg_alloc_mem`

6. Buffer-based quoting returns the quoted result size and supports explicit-length input.
   - Traceability: `quotearg_buffer`

7. Allocating quoting APIs return quoted output representing the same formatting behavior as the buffer-based API for equivalent input and options.
   - Traceability: `quotearg_alloc`, `quotearg_alloc_mem`, `quotearg_buffer`

8. The size-reporting allocation API writes the generated output size to the caller-provided size location.
   - Traceability: `quotearg_alloc_mem`

9. Slot-based quoting returns a quoted string for the requested slot index and reuses that slot across repeated calls.
   - Traceability: `quotearg_n_options`, `slotvec`

10. Style-based convenience APIs produce output consistent with first deriving options from the specified style and then quoting the same input.
    - Traceability: `quotearg_n_style`, `quotearg_n_style_mem`, `quoting_options_from_style`

11. The character-specific convenience API produces output consistent with applying a character-specific quoting rule before quoting the explicit-length input.
    - Traceability: `quotearg_char_mem`, `set_char_quoting`

12. The colon-related convenience API produces output consistent with the source module’s style-plus-colon option composition.
    - Traceability: `quotearg_n_style_colon`, `set_char_quoting`, `quoting_options_from_style`

13. All APIs with explicit input-length parameters correctly process inputs containing embedded NUL bytes within the specified length.
    - Traceability: `quotearg_buffer`, `quotearg_alloc_mem`, `quotearg_n_options`, `quotearg_n_style_mem`, `quotearg_char_mem`

## Out of Scope
The Rust rewrite specification does not require any capability not evidenced by the analyzed module, including:
- new public APIs,
- serialization or persistence,
- thread-safety guarantees,
- FFI guarantees,
- recovery or transactional behavior,
- performance or benchmark targets.