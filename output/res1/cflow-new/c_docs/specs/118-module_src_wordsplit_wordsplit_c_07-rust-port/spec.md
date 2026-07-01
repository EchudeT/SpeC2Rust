# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_c_07`

## Overview

This module covers the C-style quoting support implemented in `src/wordsplit/wordsplit.c` for the following functions:

- `wordsplit_c_quoted_length`
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`
- `wordsplit_c_quote_copy`

The Rust rewrite must preserve the observable behavior of this quoting submodule as used by the surrounding `wordsplit` logic. Its responsibility is limited to:

- recognizing characters that have C-style quoted or unquoted forms,
- computing the output length needed to represent a source string in C-style quoted form,
- copying a source string into a destination buffer using C-style quoting rules.

The specification is limited to functionality evidenced by these functions and the surrounding `wordsplit` data model in `src/wordsplit/wordsplit.c`. It does not define unrelated parsing, tokenization, shell expansion, or new public capabilities.

## Feature Specification

### Summary

The module provides C-style quote transformation support for strings and characters within the broader `wordsplit` subsystem.

It must support:

- mapping selected character values to their C escape-code form,
- mapping selected escape-code letters back to their represented character values,
- determining the number of bytes required to encode a NUL-terminated input string using the module’s C-style quoting rules,
- emitting the quoted representation of a NUL-terminated input string into a caller-provided destination buffer.

### Functional Scope

#### 1. Character escape recognition

The module must recognize a defined set of C-style single-character escapes. This includes behavior represented by:

- `wordsplit_c_unquote_char`: converts an escape-code character to the corresponding character value when the escape is one of the supported C-style forms,
- `wordsplit_c_quote_char`: converts a character value to the corresponding escape-code character when that character has a supported short C-style escape.

This behavior is limited to the escape mapping supported by the source module. The Rust version must preserve the same mapping set and fallback behavior.

#### 2. Quoted-length calculation

The module must compute the number of output bytes needed to represent an input string under the same quoting rules used by quote copying.

`wordsplit_c_quoted_length` must:

- inspect a NUL-terminated source string,
- account for characters that can be emitted directly,
- account for characters that must be emitted as escape sequences,
- account for cases where hexadecimal quoting is selected through the `quote_hex` input,
- optionally report quoting state through the `quote` output parameter behavior reflected in the source function.

The Rust version must preserve the length result for all inputs covered by the C implementation’s behavior.

#### 3. Quoted string emission

The module must copy a source string into a destination buffer using the same C-style quoting rules measured by the quoted-length function.

`wordsplit_c_quote_copy` must:

- read a NUL-terminated source string,
- write the quoted form into a caller-provided destination region,
- use the same escape selection rules as the original module,
- respect the `quote_hex` mode in the same way as the C implementation,
- produce output whose length matches the corresponding quoted-length calculation for the same input and mode.

The Rust rewrite may use safe internal representations, but the observable emitted byte sequence must match the source module’s behavior.

## User Scenarios & Testing

### Scenario 1: Quote a plain string with no characters requiring escaping

A caller passes a string containing only characters that the module leaves unescaped.

Expected support:

- quoted-length calculation returns the number of directly copied output bytes,
- quote copying emits the same visible characters,
- output length and emitted content are consistent with each other.

### Scenario 2: Quote a string containing characters with standard short C escapes

A caller passes a string containing characters such as control characters or other characters that the source module represents using standard C-style backslash escapes.

Expected support:

- the module recognizes that these characters require escaping,
- quoted-length calculation includes the escape sequence width rather than the raw character width,
- quote copying emits the corresponding backslash escape form,
- for characters that have a short named escape, the short form is used consistently with the source implementation.

### Scenario 3: Quote a string in hexadecimal-escape mode

A caller enables `quote_hex` and passes a string containing bytes that require escaped representation.

Expected support:

- the module selects hexadecimal quoting where the source implementation does so,
- quoted-length calculation reflects the hexadecimal form width,
- quote copying emits hexadecimal escape sequences exactly where required by the source behavior.

### Scenario 4: Convert escape-code letters back to character values

A caller needs to interpret a recognized C escape-code character.

Expected support:

- `wordsplit_c_unquote_char` returns the represented character value for supported escape-code letters,
- unsupported inputs retain the same fallback behavior as the source function.

### Scenario 5: Convert character values to escape-code letters

A caller needs to determine whether a character has a short C escape form.

Expected support:

- `wordsplit_c_quote_char` returns the correct escape-code letter for supported character values,
- unsupported inputs retain the same fallback behavior as the source function.

### Scenario 6: Length computation and copy remain aligned

A caller first computes required space, then allocates a destination buffer, then copies.

Expected support:

- for the same source string and `quote_hex` mode, quoted-length calculation and quote copying agree,
- the emitted quoted string occupies exactly the predicted number of non-NUL output bytes under the source module’s conventions.

## Requirements

### Functional Requirements

#### FR-1: Supported C-style escape mapping
The Rust module shall implement the same character-to-escape and escape-to-character mappings as provided by `wordsplit_c_quote_char` and `wordsplit_c_unquote_char` in `src/wordsplit/wordsplit.c`.

Traceability:
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`

#### FR-2: Preserve fallback behavior for unsupported mappings
The Rust module shall preserve the source module’s behavior for inputs that do not correspond to a supported short C escape mapping, both for quote and unquote operations.

Traceability:
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`

#### FR-3: Compute quoted length for NUL-terminated input
The Rust module shall provide behavior equivalent to `wordsplit_c_quoted_length` for determining the number of output bytes required to represent a NUL-terminated source string using the module’s quoting rules.

Traceability:
- `wordsplit_c_quoted_length`

#### FR-4: Respect hexadecimal quoting mode
The Rust module shall preserve the effect of the `quote_hex` argument in both quoted-length calculation and quoted-string emission.

Traceability:
- `wordsplit_c_quoted_length`
- `wordsplit_c_quote_copy`

#### FR-5: Preserve quote-state reporting behavior
Where the source function updates the optional quote-state output parameter, the Rust rewrite shall preserve the same externally observable state semantics.

Traceability:
- `wordsplit_c_quoted_length`

#### FR-6: Emit quoted output equivalent to the C module
The Rust module shall provide behavior equivalent to `wordsplit_c_quote_copy`, producing the same quoted byte sequence for the same input string and quoting mode.

Traceability:
- `wordsplit_c_quote_copy`

#### FR-7: Length and emission consistency
For any input supported by the source module, the Rust module’s quoted-length result shall match the number of emitted output bytes produced by the corresponding quote-copy behavior.

Traceability:
- `wordsplit_c_quoted_length`
- `wordsplit_c_quote_copy`

#### FR-8: Operate within the `wordsplit` quoting subsystem only
The Rust rewrite shall remain limited to the quoting functionality evidenced by the identified functions and shall not require unrelated tokenization or expansion behavior from the broader `wordsplit` module.

Traceability:
- `src/wordsplit/wordsplit.c`
- `struct wordsplit`
- `struct wordsplit_node`

### Key Entities

#### `wordsplit` context
The source file defines `struct wordsplit` in multiple related declarations and uses it as the main state carrier for the broader subsystem. For this module specification, it establishes that the quoting logic belongs to a larger word-splitting context, even though the listed quoting functions operate primarily on strings and character values rather than directly mutating the full context.

Relationship to this module:
- provides subsystem context only,
- not all `wordsplit` fields are in scope for this rewrite spec,
- quoting behavior must remain compatible with its use inside the larger subsystem.

Traceability:
- `struct wordsplit` declarations in `src/wordsplit/wordsplit.c`

#### `wordsplit_node`
The source file defines `struct wordsplit_node` as part of the broader internal representation used by `wordsplit`. Its presence shows that quoted string processing participates in a larger staged parsing and node-processing flow.

Relationship to this module:
- indirect structural relationship only,
- no additional node behavior is specified beyond compatibility with the surrounding subsystem’s quoting needs.

Traceability:
- `struct wordsplit_node` in `src/wordsplit/wordsplit.c:416-430`

#### Input string
A NUL-terminated character sequence supplied to quoted-length and quote-copy operations.

Relationship to this module:
- source data inspected and transformed by quoting functions,
- may contain characters requiring direct copy, short escape forms, or hexadecimal escape forms depending on mode.

Traceability:
- `wordsplit_c_quoted_length`
- `wordsplit_c_quote_copy`

#### Quote mode / quote state
A small control/state concept represented by the `quote_hex` argument and the optional `quote` output parameter in `wordsplit_c_quoted_length`.

Relationship to this module:
- influences which escape representation is selected,
- may report state needed by the surrounding caller logic.

Traceability:
- `wordsplit_c_quoted_length`
- `wordsplit_c_quote_copy`

## Success Criteria

### SC-1: Escape mapping parity
For every input value covered by the source implementations of `wordsplit_c_quote_char` and `wordsplit_c_unquote_char`, the Rust rewrite returns the same result as the C module.

### SC-2: Quoted-length parity
For representative and edge-case NUL-terminated inputs used by the source module, including strings with no escapes, strings with short escapes, and strings affected by `quote_hex`, the Rust rewrite returns the same quoted length as `wordsplit_c_quoted_length`.

### SC-3: Quote-copy parity
For the same test inputs and modes, the Rust rewrite emits the same quoted byte sequence as `wordsplit_c_quote_copy`.

### SC-4: Length/copy agreement
For every tested input and `quote_hex` mode, the Rust rewrite’s quoted-length result equals the number of emitted non-NUL output bytes from its quote-copy behavior.

### SC-5: Quote-state parity
For cases where the source `wordsplit_c_quoted_length` updates the optional quote-state output, the Rust rewrite yields the same observable state result.

### SC-6: Scope containment
The Rust rewrite implements only the quoting behavior evidenced by:
- `wordsplit_c_quoted_length`
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`
- `wordsplit_c_quote_copy`

and does not require or expose unrelated new functionality beyond compatibility with the existing `wordsplit` subsystem.