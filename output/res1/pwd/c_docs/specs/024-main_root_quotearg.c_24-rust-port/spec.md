# spec.md

## Title
Functional Specification for Rust Rewrite of `main_root_quotearg.c_24`

## Metadata
- Project: `pwd`
- Module: `main_root_quotearg.c_24`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Target branch: `024-main_root_quotearg.c_24-rust-port`
- Generation date: 2026-06-07

## Overview
This module provides argument quoting services for text intended for display or diagnostics. It converts input byte sequences into quoted, escaped, or locale-aware rendered strings according to selected quoting rules and options.

The Rust rewrite must preserve the module’s observable behavior as evidenced by `quotearg.c`, including:

- producing quoted representations of input strings and memory buffers,
- supporting configurable quoting styles and per-call quoting options,
- optionally forcing additional characters to be quoted,
- choosing locale-dependent quotation marks where applicable,
- exposing convenience entry points for common quoting use cases,
- managing internal reusable storage used by convenience APIs,
- freeing module-managed quote result storage.

This module is a formatting utility. It does not interpret shell commands, execute output, or perform unrelated string transformations beyond the quoting behavior evidenced by the source module.

## Scope
In scope for the Rust rewrite:

- Functional behavior represented by:
  - `gettext_quote`
  - `quotearg_buffer_restyled`
  - `quotearg_free`
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
- Data behavior represented by:
  - `struct quoting_options`
  - `struct slotvec`

Out of scope:

- New public APIs not evidenced by this module
- Concurrency guarantees not evidenced by this module
- Serialization, persistence, or external protocol behavior
- Performance targets beyond preserving functional behavior
- Any quoting modes or integrations not traceable to `quotearg.c`

## Feature Specification

### 1. Quoted rendering of strings and memory buffers
The module must accept either null-terminated string input or explicit-length memory input and produce a quoted textual representation.

The produced representation must reflect the selected quoting rules, including insertion of quote delimiters and escaping or protecting characters when required by the active quoting style and flags.

This behavior is evidenced by:
- `quotearg_buffer_restyled`
- `quotearg`
- `quotearg_mem`
- `quote_mem`
- `quote`

### 2. Configurable quoting behavior
The module must support quoting behavior controlled by a quoting options entity. The options determine how the input is rendered, including the selected quoting style and additional quoting controls represented in `struct quoting_options`.

The Rust rewrite must preserve the behavior of option-driven rendering as used by the module’s convenience entry points and internal formatting path.

This behavior is evidenced by:
- `struct quoting_options`
- `quotearg_buffer_restyled`

### 3. Style-sensitive quote delimiters
The module must support quoting styles that may use different left and right quote delimiters. For styles that rely on localized quotation marks, the module must obtain the appropriate quote strings via locale-sensitive lookup behavior.

This behavior is evidenced by:
- `gettext_quote`
- `quotearg_buffer_restyled`

### 4. Additional forced quoting for selected characters
The module must support a mode where specific characters are quoted even if they would not otherwise require quoting under the chosen style. This capability must be represented in the Rust rewrite because it is directly exercised by the API variant that quotes with respect to a single character.

This behavior is evidenced by:
- `quotearg_buffer_restyled`
- `quotearg_char`
- `struct quoting_options`

### 5. Convenience APIs for common quoting cases
The module must provide convenience operations matching the source module’s observable use cases:

- quote a null-terminated argument with default behavior,
- quote an explicit-length buffer,
- quote while forcing one specified character to be treated as quotable,
- produce a “quote” rendering for string or memory input through dedicated wrappers.

These behaviors are evidenced by:
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

### 6. Internal reusable result storage for convenience calls
The module must support convenience functions that return pointers/references to module-managed quoted results rather than requiring the caller to supply an output buffer. The module-managed storage must remain valid until reused by later convenience calls or released by the module cleanup function, consistent with the source module behavior.

This behavior is evidenced by:
- `struct slotvec`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`
- `quotearg_free`

### 7. Cleanup of module-managed quoting storage
The module must provide a cleanup operation that releases storage held for convenience-API results and restores the module to a post-cleanup state consistent with the source behavior.

This behavior is evidenced by:
- `quotearg_free`
- `struct slotvec`

## User Scenarios & Testing

### Scenario 1: Quote a normal string for display
A caller has a regular null-terminated argument and wants a quoted display form using default module behavior.

Expected support:
- The caller uses the equivalent of `quotearg(arg)`.
- The module returns a quoted string representation.
- The result is suitable for display and includes quoting behavior consistent with the module defaults.

Traceability:
- `quotearg`

Suggested tests:
- Quote an alphanumeric string and verify default delimiters/formatting are applied.
- Quote a string containing spaces or punctuation and verify output remains a valid quoted display form.

### Scenario 2: Quote a byte buffer with explicit length
A caller needs to quote data that may contain embedded null bytes or is not null-terminated.

Expected support:
- The caller uses the equivalent of `quotearg_mem(arg, argsize)` or `quote_mem(arg, argsize)`.
- The module processes exactly `argsize` bytes.
- The output reflects those bytes rather than stopping at the first null terminator.

Traceability:
- `quotearg_mem`
- `quote_mem`
- `quotearg_buffer_restyled`

Suggested tests:
- Quote a buffer containing embedded `\0` and verify all bytes up to the explicit length influence the result.
- Quote a non-null-terminated slice and verify no out-of-range read is required for correct behavior.

### Scenario 3: Force one character to be quoted
A caller wants output where a specific character receives quoting treatment even if the default style might otherwise leave it unquoted.

Expected support:
- The caller uses the equivalent of `quotearg_char(arg, ch)`.
- The returned quoted representation reflects forced quoting behavior for `ch`.

Traceability:
- `quotearg_char`
- `quotearg_buffer_restyled`
- `struct quoting_options`

Suggested tests:
- Input containing the forced character should produce different output than the default quoting path when the source module does so.
- Input not containing the forced character should match the default result for the same style and options.

### Scenario 4: Locale-dependent quotation marks
A caller uses a quoting style that depends on localized quotation marks.

Expected support:
- The module chooses left/right quote strings through the locale-sensitive quote lookup path.
- The rendering uses the corresponding localized delimiters when the selected style requires them.

Traceability:
- `gettext_quote`
- `quotearg_buffer_restyled`

Suggested tests:
- Under a locale/configuration where localized quotes differ from plain ASCII quotes, verify that the selected style uses the localized delimiters.
- Verify left and right delimiters are paired correctly.

### Scenario 5: Reuse convenience results and then free them
A caller uses convenience quoting functions multiple times and later invokes cleanup.

Expected support:
- Convenience calls return module-managed storage.
- Later convenience calls may reuse internal storage in a manner consistent with the source module.
- `quotearg_free()` releases internal storage and leaves the module able to operate correctly on subsequent quoting requests.

Traceability:
- `struct slotvec`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`
- `quotearg_free`

Suggested tests:
- Call convenience functions repeatedly and verify each returned value is a valid quoted result.
- Invoke cleanup and then call a convenience function again; verify correct behavior resumes.
- Verify cleanup is safe to call after prior convenience usage.

## Requirements

### Functional Requirements

#### FR-1: Quote input according to quoting style
The module shall render input into a quoted form according to a selected quoting style and related flags.

Traceability:
- `quotearg_buffer_restyled`
- `struct quoting_options`

#### FR-2: Support both string input and explicit-length memory input
The module shall support quoting of:
- null-terminated strings, and
- explicit-length memory buffers.

Traceability:
- `quotearg`
- `quotearg_mem`
- `quote_mem`
- `quote`
- `quotearg_buffer_restyled`

#### FR-3: Honor caller-visible convenience entry points
The Rust rewrite shall preserve the functional behavior of the source module’s convenience entry points for default quoting, memory quoting, forced-character quoting, and quote wrappers.

Traceability:
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

#### FR-4: Support locale-sensitive quote delimiter selection where required
When the active quoting style requires translated or locale-specific quote delimiters, the module shall obtain and use the corresponding left and right quote strings.

Traceability:
- `gettext_quote`
- `quotearg_buffer_restyled`

#### FR-5: Support explicit left and right quote delimiters in the formatting path
The formatting behavior shall support left and right quote delimiters as independent values where the selected style or option path provides them.

Traceability:
- `quotearg_buffer_restyled`

#### FR-6: Support forced quoting of selected characters
The module shall support marking additional characters as requiring quoting beyond the base behavior of the selected quoting style.

Traceability:
- `quotearg_buffer_restyled`
- `quotearg_char`
- `struct quoting_options`

#### FR-7: Produce output without requiring the caller to provide a buffer for convenience APIs
The convenience APIs shall return quoted results through module-managed storage.

Traceability:
- `struct slotvec`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

#### FR-8: Release module-managed quote result storage
The module shall provide a cleanup operation that frees module-managed storage used for convenience API results.

Traceability:
- `quotearg_free`
- `struct slotvec`

#### FR-9: Preserve behavior for repeated convenience calls
The module shall continue to produce correct quoted results across repeated convenience API calls, using internal storage management consistent with the source module’s slot-based result handling.

Traceability:
- `struct slotvec`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

### Key Entities

#### `quoting_options`
A configuration entity that represents quoting behavior. Based on source evidence, it is the central carrier of quoting style and related controls used by the formatting logic.

Role:
- defines how input should be quoted,
- participates in default and specialized quoting paths,
- supports additional character-quoting controls.

Relationships:
- consumed by `quotearg_buffer_restyled`,
- used by wrapper functions to select or derive quoting behavior.

Traceability:
- `struct quoting_options`
- `quotearg_buffer_restyled`
- wrapper call sites in `quotearg.c`

#### `slotvec`
A module-internal storage management entity used for convenience APIs that return quoted strings from reusable internal storage.

Role:
- tracks one or more result slots,
- backs return values of convenience quoting functions,
- is reset/freed by module cleanup.

Relationships:
- used by `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, and `quote`,
- released by `quotearg_free`.

Traceability:
- `struct slotvec`
- `quotearg_free`
- convenience API functions

## Success Criteria

### SC-1: Behavioral equivalence of quoting output
For representative inputs covering plain text, whitespace, punctuation, and explicit-length buffers, the Rust rewrite produces quoted output matching the source module’s behavior for the supported entry points.

Traceability:
- `quotearg_buffer_restyled`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

### SC-2: Correct handling of explicit-length input
Tests with embedded null bytes confirm that explicit-length quoting processes the full specified byte range rather than stopping at the first null byte.

Traceability:
- `quotearg_mem`
- `quote_mem`
- `quotearg_buffer_restyled`

### SC-3: Correct forced-character behavior
Tests confirm that the forced-character quoting entry point changes output when the specified character appears in input, consistent with the source module, and does not introduce unrelated changes when it does not appear.

Traceability:
- `quotearg_char`
- `quotearg_buffer_restyled`

### SC-4: Correct localized quote selection
For quoting styles that depend on locale-sensitive delimiters, tests confirm that the Rust rewrite selects and applies the same quote strings as the source behavior under equivalent locale conditions.

Traceability:
- `gettext_quote`
- `quotearg_buffer_restyled`

### SC-5: Correct lifecycle of convenience result storage
Tests confirm that:
- convenience APIs return valid quoted results across repeated calls, and
- after invoking cleanup, subsequent convenience calls still operate correctly.

Traceability:
- `struct slotvec`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`
- `quotearg_free`

### SC-6: No unsupported feature expansion
The Rust rewrite limits itself to the quoting and storage-management behavior evidenced by `quotearg.c`, with no newly invented public capabilities required by this specification.

Traceability:
- Entire module scope defined from `quotearg.c`