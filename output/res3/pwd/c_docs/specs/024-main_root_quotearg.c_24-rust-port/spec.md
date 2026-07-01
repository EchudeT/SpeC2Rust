# spec.md

## Title

Functional Specification for `main_root_quotearg.c_24` Rust Port

## Document Control

- Project: `pwd`
- Module: `main_root_quotearg.c_24`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Rust branch: `024-main_root_quotearg.c_24-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides argument quoting services for user-visible text. Its purpose is to transform input strings or byte sequences into quoted representations suitable for display, with behavior controlled by quoting style and option state.

The Rust rewrite must preserve the module’s observable quoting behavior exposed by the identified entry points in `quotearg.c`, including:

- choosing quote delimiters, including locale-sensitive delimiters,
- escaping or protecting characters according to the selected quoting style,
- supporting both NUL-terminated strings and explicit-length byte input,
- returning quoted results through convenience interfaces, and
- releasing module-managed temporary quoted-string storage.

The specification is limited to behavior evidenced by the following functions and data structures from `quotearg.c`:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`
- `struct quoting_options`
- `struct slotvec`

## Feature Specification

### Summary

The module formats input text into quoted output according to a configurable quoting policy. It supports default quoting, quoting of explicit-length memory regions, quoting with an extra protected character, and quote-oriented convenience wrappers. It also maintains reusable internal result storage for interfaces that return pointers to quoted strings, and provides an explicit cleanup operation for that storage.

### In-Scope Functionality

1. **Quoting according to quoting options and style**
   - The module must quote input text according to a selected quoting style and related option state.
   - The produced output must reflect the chosen delimiters and escaping rules represented by `struct quoting_options` and consumed by `quotearg_buffer_restyled`.

2. **Locale-sensitive quote delimiter selection**
   - The module must support obtaining quote delimiters via `gettext_quote`.
   - Delimiter selection must depend on the requested message identifier and quoting style, so that styles requiring translated or locale-aware quote marks can use the appropriate left/right quote strings.

3. **Quoting into a caller-provided buffer**
   - The module must support producing quoted output into a caller-supplied buffer via the core quoting routine represented by `quotearg_buffer_restyled`.
   - The operation must be defined for both sufficient and insufficient buffer space by reporting the full output size needed while writing at most the provided capacity.

4. **Quoting of explicit-length input**
   - The module must support quoting byte sequences whose length is supplied explicitly, without requiring the input to be NUL-terminated.
   - This behavior is required for `quotearg_mem` and `quote_mem`, and is driven by the `arg` plus `argsize` interface.

5. **Default quoting convenience**
   - The module must provide a default convenience operation for quoting a NUL-terminated string through `quotearg`.
   - The module must also provide a quote-oriented convenience wrapper through `quote`.

6. **Quoting with one additionally protected character**
   - The module must provide a convenience operation that quotes a string while ensuring one specified character is treated as requiring quoting/escaping behavior, as exposed by `quotearg_char`.

7. **Module-managed returned string storage**
   - Functions that return quoted string pointers rather than writing into caller buffers must provide valid returned storage suitable for subsequent use by the caller until overwritten by later module activity or released.
   - The internal storage model is evidenced by `struct slotvec` and by the presence of `quotearg_free`.

8. **Cleanup of internal quote result storage**
   - The module must provide `quotearg_free` to release internal storage associated with returned quoted strings.

### Out of Scope

The Rust rewrite must not introduce capabilities not evidenced by this module analysis, including:

- new public quoting APIs beyond those listed in this specification,
- thread-safety guarantees,
- serialization formats,
- recovery or persistence behavior,
- FFI requirements,
- benchmark or performance commitments beyond preserving functionality.

## User Scenarios & Testing

### Scenario 1: Quote a simple command-line argument for display

A caller has a regular NUL-terminated string and needs a quoted representation for diagnostics or output formatting.

- Entry point: `quotearg` or `quote`
- Expected behavior:
  - returns a pointer/reference to a quoted form of the input,
  - output includes quoting appropriate to the module’s default behavior,
  - repeated use remains valid according to the module-managed storage model.

#### Test expectations
- Given an input string with plain printable characters, the module returns a non-empty quoted representation.
- Given an input string containing characters that require protection under the default behavior, the output reflects quoting/escaping rather than passing those characters through unchanged.
- The result is stable long enough for immediate caller use.

### Scenario 2: Quote a memory region that is not NUL-terminated

A caller has bytes plus an explicit length and must quote exactly that region, even if embedded NULs or trailing uninitialized data exist beyond the specified length.

- Entry point: `quotearg_mem` or `quote_mem`
- Expected behavior:
  - only the specified number of bytes is consumed,
  - the returned output represents that exact byte region,
  - no dependency on finding a terminating NUL exists.

#### Test expectations
- Supplying a buffer with a valid prefix and unrelated bytes after `argsize` affects output only for the specified prefix.
- Supplying data containing an embedded NUL does not truncate processing before `argsize`.
- Returned text is quoted according to the same policy family used for string input.

### Scenario 3: Quote text while forcing one specific character to be protected

A caller needs a result where a particular character receives quoting treatment even if it might otherwise appear unescaped.

- Entry point: `quotearg_char`
- Expected behavior:
  - the selected character is treated as one that must be quoted/escaped,
  - otherwise the result follows the module’s standard quoting rules.

#### Test expectations
- If the chosen character appears in the input, the output differs from default quoting in a way that protects that character.
- If the chosen character does not appear, output matches ordinary quoting for the same input.

### Scenario 4: Produce quoted output into a caller-owned buffer

A caller needs direct control over output storage and uses the core quoting operation.

- Entry point: `quotearg_buffer_restyled`
- Expected behavior:
  - writes quoted output into the supplied buffer up to available capacity,
  - reports the full size of the quoted result,
  - obeys the requested quoting style, flags, per-character quote mask, and quote delimiters.

#### Test expectations
- With a sufficiently large buffer, the entire quoted output is written and the reported size matches the produced result length.
- With an undersized buffer, no out-of-bounds write occurs and the reported size still reflects the full output length needed.
- Changing style or quote delimiters changes output accordingly.

### Scenario 5: Use locale-sensitive quote marks

A caller or internal wrapper needs quote marks appropriate to a translation/localization context.

- Entry point: `gettext_quote`
- Expected behavior:
  - returns quote strings corresponding to the requested message identifier and quoting style,
  - supports left/right quote selection where relevant.

#### Test expectations
- For styles that use localized delimiters, left and right quote requests return the expected delimiter strings for the active locale or translation domain behavior represented by the source module.
- For non-localized cases, returned delimiters match the module’s defined defaults.

### Scenario 6: Release internal quoting storage

A caller has used pointer-returning quoting functions and wants to release temporary storage held by the module.

- Entry point: `quotearg_free`
- Expected behavior:
  - internal slot-based quoted-string storage is released,
  - subsequent module use can re-establish storage as needed.

#### Test expectations
- After using pointer-returning functions, calling `quotearg_free` succeeds without requiring caller knowledge of internal slots.
- After cleanup, calling quoting functions again still produces valid results.

## Requirements

### Functional Requirements

#### FR-1: Quoted formatting of input data
The Rust module shall transform input strings or byte sequences into quoted output according to quoting behavior represented by `struct quoting_options` and exercised through `quotearg_buffer_restyled`.

**Traceability:** `quotearg_buffer_restyled`, `struct quoting_options`

#### FR-2: Support for explicit-length input
The Rust module shall support quoting input provided as `(pointer/reference, length)` without requiring NUL termination.

**Traceability:** `quotearg_buffer_restyled`, `quotearg_mem`, `quote_mem`

#### FR-3: Support for default string quoting
The Rust module shall provide default quoting of NUL-terminated strings through the behavior represented by `quotearg` and `quote`.

**Traceability:** `quotearg`, `quote`

#### FR-4: Support for quoting with an additionally protected character
The Rust module shall provide quoting behavior in which one caller-specified character is included in the set of characters that must receive quoting treatment.

**Traceability:** `quotearg_char`, `struct quoting_options`

#### FR-5: Buffer-oriented quoting result sizing
The Rust module shall support writing quoted output into caller-provided storage while also reporting the total quoted output size required, including when the supplied storage is too small to hold the complete result.

**Traceability:** `quotearg_buffer_restyled`

#### FR-6: Quote delimiter selection
The Rust module shall support selection of quote delimiter strings, including style-dependent and locale-sensitive delimiters, via the behavior represented by `gettext_quote`.

**Traceability:** `gettext_quote`

#### FR-7: Convenience wrappers over core quoting behavior
The Rust module shall provide the convenience behaviors corresponding to:
- default string quoting,
- explicit-length quoting,
- explicit-length quote wrapper behavior, and
- single-extra-character quoting.

**Traceability:** `quotearg`, `quotearg_mem`, `quote_mem`, `quote`, `quotearg_char`

#### FR-8: Internal storage for pointer-returning results
The Rust module shall preserve the observable behavior that pointer-returning quoting interfaces provide module-managed result storage rather than requiring caller-owned output buffers.

**Traceability:** `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`, `struct slotvec`

#### FR-9: Cleanup of internal result storage
The Rust module shall provide an operation equivalent to `quotearg_free` that releases internal storage used for pointer-returning quoted results.

**Traceability:** `quotearg_free`, `struct slotvec`

### Key Entities

#### `quoting_options`
A quoting configuration object that defines how input is transformed into quoted output. It represents the quoting style and related controls used by the core quoting routine and by convenience wrappers.

**Role in the module:**
- carries quoting style selection,
- carries flags affecting quoting behavior,
- carries per-character quoting customization,
- may carry or determine quote delimiter choices.

**Traceability:** `struct quoting_options`, `quotearg_buffer_restyled`, `quotearg_char`

#### `slotvec`
An internal storage entity used to hold reusable quoted-string result slots for interfaces that return pointers to quoted strings.

**Role in the module:**
- stores module-managed result buffers,
- supports repeated calls to pointer-returning quoting helpers,
- is released by the module cleanup function.

**Traceability:** `struct slotvec`, `quotearg_free`, `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`

#### Quoted result
The produced textual representation of input data after applying quoting rules.

**Role in the module:**
- may be written into caller-owned buffers,
- may be returned from module-managed storage,
- reflects style, flags, delimiter choice, and explicit character protection.

**Traceability:** `quotearg_buffer_restyled` and all wrapper functions listed above

## Success Criteria

### SC-1: Wrapper behavior equivalence
For representative inputs, the Rust implementations corresponding to `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, and `quote` produce the same quoted text as the source module for the same effective quoting conditions.

**Traceability:** `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`

### SC-2: Explicit-length correctness
Tests demonstrate that explicit-length quoting processes exactly `argsize` bytes, including inputs with embedded NUL bytes and inputs followed by extra data beyond the specified length.

**Traceability:** `quotearg_mem`, `quote_mem`, `quotearg_buffer_restyled`

### SC-3: Buffer sizing correctness
Tests demonstrate that the Rust core quoting routine reports the full required output length and does not write beyond the supplied buffer capacity for both sufficient and insufficient buffer sizes.

**Traceability:** `quotearg_buffer_restyled`

### SC-4: Style and delimiter sensitivity
Tests demonstrate that changing quoting style and/or quote delimiters changes the produced output consistently with the source behavior, including cases using locale-sensitive delimiter lookup.

**Traceability:** `gettext_quote`, `quotearg_buffer_restyled`, `struct quoting_options`

### SC-5: Extra-character protection correctness
Tests demonstrate that the Rust behavior corresponding to `quotearg_char` protects the specified character when present and does not alter unrelated inputs beyond the source module’s established quoting behavior.

**Traceability:** `quotearg_char`, `struct quoting_options`

### SC-6: Internal storage lifecycle preservation
Tests demonstrate that pointer-returning quoting operations provide usable returned results across ordinary use, and that cleanup through the Rust equivalent of `quotearg_free` releases internal storage without preventing later successful quoting calls.

**Traceability:** `quotearg_free`, `struct slotvec`, `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`

## Acceptance Notes

- Conformance is defined by observable behavior of the source module interfaces named in this specification.
- The Rust rewrite may change internal implementation structure, but it must not change the module’s functional boundaries established here.
- Any behavior not evidenced by the listed functions and data structures is outside the required scope for this module port.