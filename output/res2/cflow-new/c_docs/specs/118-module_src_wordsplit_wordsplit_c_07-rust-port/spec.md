# spec.md

## Title

Functional Specification for `module_src_wordsplit_wordsplit_c_07` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_c_07`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `118-module_src_wordsplit_wordsplit_c_07-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides C-style string quoting and unquoting support used by the broader word-splitting subsystem. Its evidenced scope is limited to:

- determining the output length required to C-quote a source string,
- classifying characters for C-style quote/unquote handling,
- copying a source string into a destination buffer using C-style escape sequences.

The Rust rewrite must preserve the observable behavior of these operations as used by the surrounding `wordsplit` subsystem. The specification covers only the functionality evidenced by the identified functions and their relationship to the `wordsplit` parsing context.

## Feature Specification

### Feature: C-style quoted string sizing

The module shall provide behavior equivalent to `wordsplit_c_quoted_length` for computing how many output characters are required to represent an input C string in quoted form.

This behavior includes:

- scanning a NUL-terminated input string,
- determining the length of the escaped representation,
- accounting for characters that require escaping,
- optionally using hexadecimal escape forms when the `quote_hex` mode is enabled,
- reporting through the `quote` output parameter whether quoting is needed for the input.

The Rust version must preserve the sizing behavior needed so that a destination buffer sized from this result is sufficient for the corresponding quote-copy operation.

### Feature: Character classification for C-style escapes

The module shall provide behavior equivalent to:

- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`

These operations classify or translate a single character according to the module’s C-style escape rules. The Rust rewrite must preserve the same per-character decisions used by the quoting logic.

### Feature: C-style quoted copy

The module shall provide behavior equivalent to `wordsplit_c_quote_copy` for copying a NUL-terminated input string into a destination buffer while applying C-style escape sequences.

This behavior includes:

- reading the source as a byte-oriented C string,
- writing escaped output into the caller-provided destination,
- escaping characters according to the same rules used by quoted-length calculation,
- supporting the `quote_hex` mode consistently with length calculation,
- producing output suitable for downstream word-splitting or display logic that expects C-style quoted content.

## User Scenarios & Testing

### Scenario 1: Determine whether a string needs quoting

A caller in the word-splitting subsystem has an input string and needs to know whether plain copying is sufficient or whether C-style quoting is required.

The Rust version must support:

- computing the quoted output length from the input,
- indicating through a quote-status result whether quoting is required,
- returning a length consistent with the eventual quoted-copy result.

#### Test expectations

- For an input containing only characters that do not require escaping, the quote-status result indicates no quoting needed.
- For an input containing at least one escapable character, the quote-status result indicates quoting needed.
- The reported length matches the number of non-NUL output characters produced by quote-copy for the same input and `quote_hex` setting.

### Scenario 2: Copy a string into escaped form

A caller has already allocated destination space and needs a C-style escaped representation of a source string.

The Rust version must support:

- copying the complete input into the destination representation,
- escaping special characters consistently,
- honoring `quote_hex` when enabled.

#### Test expectations

- Characters classified as requiring C-style escapes are emitted in escaped form.
- Characters not requiring escapes are copied as-is.
- The produced output length matches the sizing function’s result.

### Scenario 3: Process individual escaped characters consistently

A caller or internal quoting routine needs the character-level mapping used by the quoting subsystem.

The Rust version must support:

- consistent single-character handling for quote and unquote classification,
- agreement between character-level classification and whole-string quoting behavior.

#### Test expectations

- The character-level quote decision aligns with whether the same character is escaped by quote-copy.
- The character-level unquote handling recognizes the escape forms supported by the original module logic.

### Scenario 4: Integrate with `wordsplit` context-driven processing

The quoting helpers are used within a larger `wordsplit` module that manages parsing state and node-based word construction.

The Rust version must support:

- use from code that operates on `wordsplit` state,
- behavior that does not require capabilities outside the evidenced quoting/unquoting scope,
- compatibility with node- or buffer-oriented assembly of transformed text.

#### Test expectations

- The quoting helpers can be applied to strings originating from the `wordsplit` processing flow.
- Their results are stable and deterministic for repeated calls with the same input.

## Requirements

### Functional Requirements

#### FR-1: Quoted length computation

The module shall compute the length of the C-style quoted representation of a NUL-terminated input string.

**Traceability:** `wordsplit_c_quoted_length` in `src/wordsplit/wordsplit.c`

#### FR-2: Quote-needed indication

The module shall provide a quote-needed indication associated with quoted-length computation, reflecting whether the input contains any characters that require quoting.

**Traceability:** `wordsplit_c_quoted_length` in `src/wordsplit/wordsplit.c`

#### FR-3: Hex-escape mode support

The module shall support a mode that affects quoting behavior through the `quote_hex` parameter, and this mode shall be applied consistently in both size computation and quote-copy behavior.

**Traceability:** `wordsplit_c_quoted_length`, `wordsplit_c_quote_copy` in `src/wordsplit/wordsplit.c`

#### FR-4: Character-level unquote handling

The module shall provide single-character handling equivalent to the original C-style unquote helper.

**Traceability:** `wordsplit_c_unquote_char` in `src/wordsplit/wordsplit.c`

#### FR-5: Character-level quote handling

The module shall provide single-character handling equivalent to the original C-style quote helper.

**Traceability:** `wordsplit_c_quote_char` in `src/wordsplit/wordsplit.c`

#### FR-6: Quoted copy generation

The module shall generate a destination string containing the C-style quoted representation of a source NUL-terminated string.

**Traceability:** `wordsplit_c_quote_copy` in `src/wordsplit/wordsplit.c`

#### FR-7: Length/copy consistency

For the same input and `quote_hex` setting, the quoted-length operation and the quoted-copy operation shall be behaviorally consistent: the computed length must equal the number of output characters written, excluding the terminating NUL if represented internally.

**Traceability:** `wordsplit_c_quoted_length`, `wordsplit_c_quote_copy` in `src/wordsplit/wordsplit.c`

#### FR-8: Compatibility with `wordsplit` subsystem use

The Rust port shall preserve the quoting helper behavior needed by the surrounding `wordsplit` module state and node processing context, without expanding beyond the evidenced quoting-related role.

**Traceability:** `struct wordsplit`, `struct wordsplit_node`; quoting functions in `src/wordsplit/wordsplit.c`

### Key Entities

#### `wordsplit`

The `wordsplit` structure is the broader parsing and transformation context in which this module’s quoting helpers operate. It represents the subsystem state that may consume or produce strings requiring C-style quoting behavior.

**Relationship to this module:** The quoted-length and quote-copy helpers are part of the `wordsplit.c` functionality and must remain usable from logic organized around `wordsplit`.

**Traceability:** multiple `struct wordsplit` declarations/usages in `src/wordsplit/wordsplit.c`

#### `wordsplit_node`

The `wordsplit_node` structure represents node-based intermediate content used by the broader word-splitting subsystem.

**Relationship to this module:** Quoted string output may be incorporated into node-managed text assembly or transformations within the surrounding subsystem.

**Traceability:** `struct wordsplit_node` in `src/wordsplit/wordsplit.c:416-430` and related usages

#### Input string

The input string is a NUL-terminated C string consumed by the quoted-length and quote-copy operations.

**Relationship to this module:** It is the source material scanned for escapable characters and transformed into quoted output.

**Traceability:** function signatures of `wordsplit_c_quoted_length` and `wordsplit_c_quote_copy`

#### Destination string/buffer

The destination buffer is caller-provided storage receiving the quoted representation.

**Relationship to this module:** Its required size is determined by the quoted-length computation, and it is populated by the quote-copy operation.

**Traceability:** function signatures of `wordsplit_c_quoted_length` and `wordsplit_c_quote_copy`

#### Quote mode and quote-status values

The module uses:

- a `quote_hex` input mode that influences escape style,
- a `quote` output indicator reporting whether quoting is needed.

**Relationship to this module:** These values control and report quoting behavior across the sizing and copying operations.

**Traceability:** `wordsplit_c_quoted_length`, `wordsplit_c_quote_copy`

## Success Criteria

### SC-1: Behavioral equivalence for quoted sizing

For a representative test set of input strings, the Rust implementation returns the same quoted length as the C module for each input and each tested `quote_hex` mode.

**Traceability:** `wordsplit_c_quoted_length`

### SC-2: Behavioral equivalence for quote-needed reporting

For the same test set, the Rust implementation reports the same quote-needed outcome as the C module.

**Traceability:** `wordsplit_c_quoted_length`

### SC-3: Behavioral equivalence for quoted output

For a representative test set of input strings, the Rust implementation produces the same quoted output content as the C module for each input and each tested `quote_hex` mode.

**Traceability:** `wordsplit_c_quote_copy`

### SC-4: Character-level equivalence

For the set of characters handled by the original helpers, the Rust implementation returns the same results as the C module for both quote and unquote character operations.

**Traceability:** `wordsplit_c_unquote_char`, `wordsplit_c_quote_char`

### SC-5: Length/output consistency

For every tested input, the Rust quoted-length result equals the length of the Rust quoted output produced under the same mode.

**Traceability:** `wordsplit_c_quoted_length`, `wordsplit_c_quote_copy`

### SC-6: No unsupported feature expansion

The Rust port remains limited to the evidenced module responsibilities: quoted sizing, character-level quote/unquote handling, and quoted copy behavior used by the `wordsplit` subsystem.

**Traceability:** identified functions and `wordsplit`/`wordsplit_node` context in `src/wordsplit/wordsplit.c`