# spec.md

## Title

Functional Specification for `module_src_wordsplit_wordsplit_c_07` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_c_07`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `118-module_src_wordsplit_wordsplit_c_07-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides C-style string quoting support used by the wordsplitting subsystem. Its evidenced scope is limited to:

- computing the output length required to represent an input string in C-style quoted form,
- mapping between raw characters and their C escape-form equivalents for known single-character escapes,
- copying an input string into an output buffer using C-style quoting rules.

The Rust rewrite must preserve the observable behavior of these quoting helpers as used by the surrounding wordsplit logic. The specification does not require unrelated wordsplitting features beyond the entities and helper relationships evidenced in this source file.

## Feature Specification

### Feature: C-style quoted string sizing

The module must determine how many bytes are needed to encode a source C string using this module’s quoting rules.

Behavioral scope evidenced by `wordsplit_c_quoted_length`:

- Accept a null-terminated input string.
- Inspect the input sequentially and determine the encoded output length.
- Account for characters that are copied directly and characters that expand into escape sequences.
- Support a mode switch controlling whether hexadecimal-style escaping is used where applicable (`quote_hex`).
- Report, through the `quote` out-parameter when provided, whether quoting/escaping was required during the scan.

The Rust version must preserve the same sizing semantics as the C helper so that any encoded output produced by the quote-copy operation fits the predicted length.

### Feature: Single-character escape translation

The module must translate between raw control/special characters and their C escape notation equivalents for supported single-character escapes.

Behavioral scope evidenced by:

- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`

Required behavior:

- `quote_char` maps a raw character to its escape-letter form when the character has a supported short C escape.
- `unquote_char` performs the inverse mapping for supported escape letters.
- For unsupported inputs, behavior must remain compatible with the source module’s helper semantics.

This feature is limited to single-character escape translation and does not imply full general-purpose C literal parsing.

### Feature: C-style quoted string copy

The module must encode a source C string into a destination buffer using the same quoting rules used by the sizing helper.

Behavioral scope evidenced by `wordsplit_c_quote_copy`:

- Accept a destination buffer, a null-terminated source string, and the `quote_hex` mode flag.
- Write the quoted/escaped representation of the source into the destination buffer.
- Apply direct copying for characters that do not require escaping.
- Apply supported short escape forms and any hexadecimal escaping behavior controlled by `quote_hex`.
- Terminate the produced output as a C string.

The Rust rewrite must preserve the functional contract that the produced encoding matches the length predicted by the sizing helper for the same input and mode.

## User Scenarios & Testing

### Scenario 1: Precompute storage before quoting

A caller needs to emit a quoted representation of an input string and first determines how much storage is needed.

Expected support:

1. Provide an input string and the quoting mode.
2. Receive the encoded length.
3. Use that length to allocate output storage.
4. Encode the string into that storage.
5. Observe that the encoded output fits exactly within the predicted size plus terminator handling consistent with the source behavior.

Tests should verify that representative inputs produce matching results between:
- computed quoted length, and
- actual bytes written by the quote-copy routine.

### Scenario 2: Encode strings containing plain printable characters

A caller passes a string that contains only characters that do not require escaping under this module’s rules.

Expected support:

- The sizing helper reports a length equal to the copied output size for direct characters.
- The quote indicator reports that no quoting was needed when an indicator is requested.
- The copy helper reproduces the same visible text content in the output buffer.

Tests should cover empty strings and simple printable strings.

### Scenario 3: Encode strings containing escapable control or special characters

A caller passes a string containing characters such as control characters or characters with supported C short escapes.

Expected support:

- The sizing helper counts the escape expansion correctly.
- The quote indicator reports that quoting was needed.
- The copy helper emits the expected escaped representation.
- The single-character translation helpers agree with the corresponding short escapes used in the copied output.

Tests should include values that exercise supported short escape mappings.

### Scenario 4: Use hexadecimal quoting mode

A caller enables the hexadecimal quoting mode for inputs that require non-direct representation.

Expected support:

- The sizing helper reflects the length impact of hexadecimal escaping.
- The copy helper emits output consistent with that mode.
- The output for the same input can differ from non-hex mode where the source module’s rules distinguish them.

Tests should compare the same input under both `quote_hex` settings and verify behavior differences only where required by source semantics.

### Scenario 5: Translate a single escape character

A caller needs to convert between a raw character and its C escape-letter representation.

Expected support:

- `quote_char` returns the short-escape symbol for supported raw characters.
- `unquote_char` returns the corresponding raw character for supported escape letters.
- Round-trip behavior is correct for supported mappings.

Tests should verify each supported pair and confirm source-compatible handling of unsupported inputs.

## Requirements

### Functional Requirements

#### FR-1: Quoted-length computation
The module shall provide logic equivalent to `wordsplit_c_quoted_length` that computes the encoded length of a null-terminated input string under this module’s C-style quoting rules.
Traceability: `src/wordsplit/wordsplit.c:2392-2420`

#### FR-2: Quote-needed indication
When requested by the caller, the quoted-length computation shall indicate whether the inspected string required escaping/quoting under the module’s rules.
Traceability: `src/wordsplit/wordsplit.c:2392-2420`

#### FR-3: Mode-dependent sizing
The quoted-length computation shall vary its result according to the `quote_hex` mode exactly where the source function’s behavior does.
Traceability: `src/wordsplit/wordsplit.c:2392-2420`

#### FR-4: Escape-letter to raw-character translation
The module shall provide logic equivalent to `wordsplit_c_unquote_char` for converting supported C escape letters into their raw character values.
Traceability: `src/wordsplit/wordsplit.c:2445-2449`

#### FR-5: Raw-character to escape-letter translation
The module shall provide logic equivalent to `wordsplit_c_quote_char` for converting supported raw characters into their C escape-letter values.
Traceability: `src/wordsplit/wordsplit.c:2451-2455`

#### FR-6: Quoted copy generation
The module shall provide logic equivalent to `wordsplit_c_quote_copy` that writes the quoted representation of a null-terminated source string into caller-provided destination storage.
Traceability: `src/wordsplit/wordsplit.c:2535-2572`

#### FR-7: Consistency between sizing and copy
For any input and `quote_hex` mode accepted by the module, the encoded form produced by the quote-copy operation shall be consistent with the length predicted by the quoted-length operation.
Traceability: `src/wordsplit/wordsplit.c:2392-2420`, `src/wordsplit/wordsplit.c:2535-2572`

#### FR-8: C-string termination semantics
The quoted copy operation shall produce output with C-string termination semantics compatible with the source routine.
Traceability: `src/wordsplit/wordsplit.c:2535-2572`

### Key Entities

#### `wordsplit`
The source file defines a central `struct wordsplit` used by the larger subsystem. Within the evidenced scope of this module fragment, the quoting helpers operate as support logic for string handling associated with that subsystem rather than as independent domain objects.
Traceability: multiple `struct wordsplit` declarations in `src/wordsplit/wordsplit.c`

#### `wordsplit_node`
The source file also defines `struct wordsplit_node`, indicating node-based internal representation inside the broader wordsplit subsystem. No additional behavior for this spec is inferred beyond the relationship that the quoting helpers belong to the same source module and may support string processing used by that subsystem.
Traceability: `src/wordsplit/wordsplit.c:416-430` and related references

#### Quoted string helper operations
The concrete functional entities in scope for the Rust port are the helper operations for:
- quoted-length calculation,
- single-character quote/unquote translation,
- quoted string copying.

These operations are logically related and must use compatible quoting rules.
Traceability: `wordsplit_c_quoted_length`, `wordsplit_c_unquote_char`, `wordsplit_c_quote_char`, `wordsplit_c_quote_copy`

## Success Criteria

### SC-1: Behavioral equivalence for length computation
For a source-compatible test corpus of input strings and both `quote_hex` modes, the Rust implementation returns the same quoted lengths as the C module.
Traceability: `wordsplit_c_quoted_length`

### SC-2: Behavioral equivalence for quote-needed reporting
For test cases that do and do not require escaping, the Rust implementation reports quote-needed status identically to the C module when the caller requests it.
Traceability: `wordsplit_c_quoted_length`

### SC-3: Behavioral equivalence for escape translation
For every supported short escape mapping in the source module, the Rust implementation’s quote and unquote translation results match the C module, including round-trip correctness for supported pairs.
Traceability: `wordsplit_c_unquote_char`, `wordsplit_c_quote_char`

### SC-4: Behavioral equivalence for quoted output
For a source-compatible test corpus of input strings and both `quote_hex` modes, the Rust implementation produces the same quoted string bytes as the C module.
Traceability: `wordsplit_c_quote_copy`

### SC-5: Length/output consistency
For all verification cases, the number of bytes in the Rust-produced encoded output before the terminator matches the Rust quoted-length result for the same input and mode.
Traceability: `wordsplit_c_quoted_length`, `wordsplit_c_quote_copy`

### SC-6: Scope control
The Rust port implements the evidenced quoting-helper behavior in this module fragment without adding unrelated public functionality not supported by the cited source file and functions.
Traceability: source scope limited to `src/wordsplit/wordsplit.c` functions listed above