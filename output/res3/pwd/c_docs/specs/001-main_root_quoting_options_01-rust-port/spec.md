# spec.md

## Overview

- **Project**: `pwd`
- **Module**: `main_root_quoting_options_01`
- **Category**: `main_cluster`
- **Source basis**: `quotearg.c`
- **Rust branch target**: `001-main_root_quoting_options_01-rust-port`
- **Generation date**: `2026-06-09`

This module provides argument-quoting behavior driven by configurable quoting options. It supports constructing and modifying quoting option sets, producing quoted output into caller-provided buffers or newly allocated strings, and using reusable numbered slots for repeated quoted-string generation. It also supports style-based convenience entry points and limited per-character quoting customization.

The Rust rewrite must preserve the functional behavior evidenced by the analyzed C module: configuration of quoting behavior, conversion of raw byte/string input into quoted form under those options, style-derived option creation, custom quote delimiter configuration, and slot-indexed repeated quoting operations.

---

## Feature Specification

### Summary

The module is responsible for transforming an input argument into a quoted representation according to a `quoting_options` configuration. The configuration may be cloned, queried, and modified. Quoted output may be written into a caller-supplied buffer, returned as newly allocated storage, or returned through a reusable numbered-slot mechanism. Convenience entry points derive quoting behavior from a specified quoting style and from specific character-quoting adjustments.

### In-scope functionality

1. **Quoting option lifecycle**
   - Create an independent copy of an existing quoting options object.
   - Derive a quoting options object from a quoting style.
   - Read and update the quoting style associated with an options object.

2. **Quoting option customization**
   - Enable or disable quoting behavior for an individual character.
   - Replace quoting flags as a whole.
   - Configure custom left and right quote delimiters for custom quoting behavior.

3. **Quoted output generation**
   - Quote an input argument into a caller-provided buffer and report the resulting size.
   - Quote an input argument into newly allocated memory.
   - Quote an input argument into newly allocated memory while also reporting the produced size.

4. **Slot-based repeated quoting**
   - Produce quoted output associated with a caller-selected numeric slot index.
   - Reuse and resize per-slot storage as needed across repeated calls.

5. **Style-based convenience quoting**
   - Quote by specifying only a style and input.
   - Quote by specifying style plus explicit input length.
   - Quote while forcing quoting treatment for a specific character.
   - Quote using a style with colon-sensitive behavior via per-character quoting configuration.

### Behavioral boundaries

- The module’s purpose is quoting and quoting-option management only.
- The module exposes configuration and quoting operations evidenced by the listed functions and data structures.
- The Rust rewrite must not introduce unrelated capabilities such as persistence, serialization, concurrency guarantees, or external interface layers not evidenced in the source analysis.

---

## User Scenarios & Testing

### Scenario 1: Clone and adjust a quoting configuration

A caller starts from an existing quoting configuration, clones it, changes the style in the clone, and uses the clone without altering the original configuration.

**Expected support**
- The clone behaves as an independent options object.
- Reading the style from the original and clone reflects their respective current values.
- Updating the clone’s style does not mutate the source options object.

**Testing focus**
- Clone an options object.
- Change style on the clone.
- Verify original style remains unchanged.
- Verify quoted output differs when styles differ.

### Scenario 2: Toggle quoting for a specific character

A caller needs a certain character to be forcibly quoted or not quoted, independent of the broader style defaults.

**Expected support**
- The module accepts a character and a requested state.
- The module reports the previous state for that character.
- Subsequent quoting honors the updated per-character setting.

**Testing focus**
- Set per-character quoting on a chosen character.
- Confirm previous setting is returned.
- Quote input containing that character before and after the change.
- Confirm output changes accordingly.

### Scenario 3: Replace option flags

A caller wants to replace the active quoting flags for an options object.

**Expected support**
- The module updates the options object’s flags.
- The previous flag value is returned.
- Quoted output reflects the new flag configuration where relevant.

**Testing focus**
- Set flags on an options object.
- Verify previous flags are returned.
- Compare quoting output under different flag values.

### Scenario 4: Use custom quote delimiters

A caller requires custom left and right quote strings around quoted output.

**Expected support**
- Custom delimiters can be assigned to an options object.
- Quoting with that options object uses the configured delimiters.
- The operation requires both left and right quote strings to be supplied.

**Testing focus**
- Configure custom left/right quote strings.
- Quote representative input.
- Verify output contains the configured delimiters.

### Scenario 5: Write quoted output into a caller buffer

A caller provides a destination buffer and needs the module to render quoted output into it while reporting the total output size.

**Expected support**
- The module processes an input argument with explicit size.
- It writes the quoted representation into the buffer subject to buffer capacity.
- It returns the size associated with the quoted result.

**Testing focus**
- Call with sufficient buffer capacity and verify exact content.
- Call with smaller buffer capacity and verify returned size still reflects the full quoted result semantics expected by the C behavior.
- Exercise empty input and non-empty input.

### Scenario 6: Allocate a quoted string

A caller wants a standalone allocated quoted result for an input argument.

**Expected support**
- The module allocates storage for the quoted output and returns it.
- A size-reporting variant also returns the produced size.
- Explicit input length is supported.

**Testing focus**
- Quote a normal string.
- Quote an input containing bytes or characters requiring quoting treatment.
- Verify size-reporting variant matches the actual produced output size.

### Scenario 7: Reuse slot-indexed quoted results

A caller repeatedly requests quoted output using a numeric slot index and expects the module to manage reusable storage for that slot.

**Expected support**
- Quoted results are available per slot index.
- Different slot indices can hold independent results.
- Repeated use of the same slot can replace prior contents.
- Storage expands as needed for larger quoted results.

**Testing focus**
- Request quoted output in slot 0, then again in slot 0 with different input.
- Request quoted output in another slot and verify independence.
- Use a larger input later and verify correctness after storage growth.

### Scenario 8: Quote directly from a style

A caller does not want to manually build an options object and instead quotes directly using a specified style.

**Expected support**
- Style-based convenience entry points work for null-terminated and explicit-length inputs.
- Colon-sensitive convenience behavior is supported through the dedicated style-plus-colon entry point.
- Character-specific convenience quoting is supported through the dedicated character-oriented entry point.

**Testing focus**
- Compare style-derived quoting against quoting using explicitly constructed options from the same style.
- Verify colon-sensitive entry point affects colon-containing input.
- Verify character-oriented convenience function affects the specified character.

---

## Requirements

### Functional Requirements

#### FR-1: Quoting options cloning
The Rust module shall provide behavior equivalent to cloning a `quoting_options` object so that the resulting options object can be modified independently of the source.

**Traceability**: `clone_quoting_options`

#### FR-2: Quoting style inspection and update
The Rust module shall support reading the quoting style from a `quoting_options` object and updating that style on a mutable options object.

**Traceability**: `get_quoting_style`, `set_quoting_style`

#### FR-3: Per-character quoting control
The Rust module shall support changing whether a specific character is specially quoted within a `quoting_options` object, and shall report the previous setting for that character.

**Traceability**: `set_char_quoting`

#### FR-4: Quoting flag replacement
The Rust module shall support replacing the active quoting flags in a `quoting_options` object and shall report the previous flag value.

**Traceability**: `set_quoting_flags`

#### FR-5: Custom quote delimiter configuration
The Rust module shall support assigning custom left and right quote delimiters to a `quoting_options` object for use by custom quoting behavior.

**Traceability**: `set_custom_quoting`

#### FR-6: Style-derived option construction
The Rust module shall support producing quoting behavior from a quoting style, including internal style-to-options derivation used by convenience APIs.

**Traceability**: `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_n_style_colon`

#### FR-7: Buffer-based quoting
The Rust module shall support quoting an input argument of explicit length into caller-provided storage and returning the resulting quoted size.

**Traceability**: `quotearg_buffer`

#### FR-8: Allocating quoted output
The Rust module shall support returning quoted output in newly allocated storage, both with and without reporting the resulting size.

**Traceability**: `quotearg_alloc`, `quotearg_alloc_mem`

#### FR-9: Slot-indexed quoting
The Rust module shall support slot-based quoted output generation indexed by a caller-provided integer, with each slot retaining reusable storage for subsequent calls.

**Traceability**: `quotearg_n_options`, `slotvec`

#### FR-10: Style-based convenience quoting APIs
The Rust module shall support convenience quoting operations that:
- quote by slot and style,
- quote by slot, style, and explicit input length,
- quote while marking a specified character for quoting,
- quote by slot and style with colon-sensitive behavior.

**Traceability**: `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_char_mem`, `quotearg_n_style_colon`

### Key Entities

#### `quoting_options`
Configuration object controlling how an argument is transformed into quoted output.

Observed responsibilities include:
- storing the selected quoting style,
- storing quoting flags,
- storing per-character quoting adjustments,
- storing custom left/right quote delimiters.

Relationship to functionality:
- modified by configuration functions,
- consumed by buffer-based and allocation-based quoting functions,
- created or derived from a quoting style for convenience operations.

**Traceability**: `quoting_options` structure references; `clone_quoting_options`, `get_quoting_style`, `set_quoting_style`, `set_char_quoting`, `set_quoting_flags`, `set_custom_quoting`, `quoting_options_from_style`, `quotearg_buffer`, `quotearg_alloc`, `quotearg_alloc_mem`, `quotearg_n_options`

#### Quoting style
Enumeration-like selector determining the general quoting strategy applied to input.

Relationship to functionality:
- stored in `quoting_options`,
- read and written directly,
- used to derive temporary or convenience quoting configurations.

**Traceability**: `get_quoting_style`, `set_quoting_style`, `quoting_options_from_style`, `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_n_style_colon`

#### `slotvec`
Internal slot-associated storage used by slot-indexed quoting operations.

Observed responsibilities include:
- associating reusable storage with numeric slot identifiers,
- allowing replacement or growth of per-slot quoted results across calls.

Relationship to functionality:
- used by slot-based quoting entry points to return stable per-slot results.

**Traceability**: `slotvec`, `quotearg_n_options`

---

## Success Criteria

1. **Configuration independence**
   - Cloning a quoting-options object and mutating the clone does not alter the original object’s style, flags, per-character settings, or custom delimiters as observed through subsequent quoting behavior.
   - **Traceability**: `clone_quoting_options`

2. **Style access correctness**
   - Reading a style after setting it yields the updated style value.
   - Quoted output produced with different styles can differ in accordance with the source behavior.
   - **Traceability**: `get_quoting_style`, `set_quoting_style`

3. **Per-character setting correctness**
   - Updating a character’s quoting state returns the prior state.
   - Quoting input containing that character reflects the updated setting.
   - **Traceability**: `set_char_quoting`

4. **Flag replacement correctness**
   - Replacing quoting flags returns the prior flag value.
   - Quoted output under the updated flags matches the source module behavior for the same input and options.
   - **Traceability**: `set_quoting_flags`

5. **Custom delimiter correctness**
   - After setting custom left and right quote delimiters, quoted output uses those delimiters where custom quoting behavior applies.
   - **Traceability**: `set_custom_quoting`

6. **Buffer quoting equivalence**
   - For representative inputs and option combinations, the buffer-based quoting API returns the same quoted size and byte content as the source module, subject to the provided buffer capacity semantics.
   - **Traceability**: `quotearg_buffer`

7. **Allocated output equivalence**
   - For representative inputs and option combinations, allocated-output APIs produce the same quoted content as the source module.
   - The size-reporting variant reports the produced size consistently with the returned output.
   - **Traceability**: `quotearg_alloc`, `quotearg_alloc_mem`

8. **Slot behavior equivalence**
   - Repeated quoting through the same numeric slot replaces prior slot content correctly.
   - Distinct slot numbers can hold independent quoted results.
   - Larger later results are handled correctly through slot storage growth.
   - **Traceability**: `quotearg_n_options`, `slotvec`

9. **Convenience API equivalence**
   - Style-based, length-aware, colon-sensitive, and character-oriented convenience quoting APIs produce the same results as equivalent behavior assembled through underlying options configuration.
   - **Traceability**: `quotearg_n_style`, `quotearg_n_style_mem`, `quotearg_char_mem`, `quotearg_n_style_colon`, `quoting_options_from_style`

10. **Source-compatibility coverage**
    - The Rust rewrite implements all evidenced functional areas of this module: quoting-option management, style-derived configuration, buffer and allocated quoting, and slot-based convenience quoting.
    - **Traceability**: all listed module functions and referenced core structures