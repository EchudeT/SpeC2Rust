# spec.md

## Title

Functional Specification: `main_root_quotearg.c_32` Rust Port

## Document Control

- Project: `cat`
- Module: `main_root_quotearg.c_32`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Target branch: `033-main_root_quotearg.c_32-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides argument-quoting functionality for strings and byte sequences intended for safe, readable presentation. It formats input according to configurable quoting rules, supports locale-sensitive quote selection, and exposes convenience entry points for common quoting cases.

The Rust rewrite must preserve the observable behavior evidenced by the source module functions and data structures in `quotearg.c`, including:

- selecting quote delimiters, including locale-aware variants;
- rendering quoted output into caller-provided storage;
- returning quoted strings through convenience wrappers;
- supporting both NUL-terminated strings and explicit-length byte ranges;
- applying configurable quoting options, including quoting style and character-specific treatment;
- releasing module-managed temporary quoting storage.

## Feature Specification

### Summary of supported functionality

The Rust version must implement a quoting subsystem that:

1. Accepts textual or raw byte input and produces a quoted representation.
2. Supports multiple quoting styles through a quoting-style selector used by the core formatter and related option structures.
3. Supports locale-sensitive quote delimiter selection through `gettext_quote`.
4. Supports formatting into a caller-supplied buffer through the core restyled quoting operation.
5. Supports convenience APIs that return quoted data for:
   - a full string,
   - an explicit-length memory region,
   - a string with a specific character forced into quoting treatment,
   - default "quote"/"quote_mem" wrappers.
6. Maintains module-managed storage for convenience-returned quoted strings and provides `quotearg_free` to release that storage.

### Functional boundaries

Included in scope for the Rust rewrite:

- quote delimiter selection based on message id and quoting style;
- quoted rendering of an input byte sequence under configured options;
- wrappers that build quoting options for common cases and return a quoted string;
- cleanup of internal storage used by the wrapper-returning functions.

Not in scope unless directly required to preserve the above behavior:

- any new public API surface beyond behavior evidenced by the listed functions and option/slot storage types;
- guarantees not evidenced by the source, such as thread safety, persistence, serialization, recovery behavior, or performance targets.

## User Scenarios & Testing

### Scenario 1: Quote a normal string with default behavior

A caller has a conventional C string and wants a quoted representation using the module's default quoting behavior.

- Input: a NUL-terminated string passed to `quotearg` or `quote`.
- Expected behavior: the module returns a pointer/reference to a quoted representation of that string.
- Test focus:
  - output is quoted rather than returned verbatim where the default behavior requires quoting;
  - returned content is stable for immediate caller use;
  - behavior matches the source module for representative plain strings, empty strings, and strings containing spaces or punctuation.

Traceability: `quotearg`, `quote`, `struct quoting_options`, `struct slotvec`.

### Scenario 2: Quote a byte sequence with explicit length

A caller needs to quote input that may contain embedded NUL bytes or is otherwise not represented as a conventional C string.

- Input: pointer plus explicit size passed to `quotearg_mem` or `quote_mem`.
- Expected behavior: the module processes exactly the specified number of bytes and returns a quoted representation of that byte range.
- Test focus:
  - embedded NUL does not truncate processing;
  - output corresponds to the full supplied length;
  - zero-length input is handled consistently with source behavior.

Traceability: `quotearg_mem`, `quote_mem`, `quotearg_buffer_restyled`.

### Scenario 3: Quote with character-specific treatment

A caller needs quoting behavior that specifically treats one chosen character as requiring quoting or escaping.

- Input: string plus character passed to `quotearg_char`.
- Expected behavior: output reflects the same quoting behavior as the base quoting logic, with the specified character specially included in the quoting set.
- Test focus:
  - when the target character appears, output differs from plain default quoting as required;
  - when the target character does not appear, behavior remains otherwise consistent with the base quoting path.

Traceability: `quotearg_char`, `struct quoting_options`, `quotearg_buffer_restyled`.

### Scenario 4: Render into caller-provided storage

A caller already owns an output buffer and needs the quoted form written there.

- Input: destination buffer and size, source pointer and explicit size, quoting style, flags, per-character quote mask, and quote delimiters passed to `quotearg_buffer_restyled`.
- Expected behavior: the module writes a quoted rendering into the supplied buffer according to the provided settings and reports the resulting size.
- Test focus:
  - returned size corresponds to the formatted result;
  - behavior is correct when the destination buffer is large enough;
  - behavior is defined and source-compatible when the destination buffer is smaller than the full quoted output;
  - custom left/right quote delimiters are honored when provided through the function contract.

Traceability: `quotearg_buffer_restyled`.

### Scenario 5: Locale-sensitive quote selection

A caller or wrapper path needs the correct textual quote delimiter for a selected quoting style.

- Input: quote-related message id and quoting style passed to `gettext_quote`.
- Expected behavior: the module returns the delimiter string appropriate for the requested style and translation-sensitive context.
- Test focus:
  - the returned delimiter varies appropriately by style where the source behavior does so;
  - locale/translation-driven choices match the source module behavior under equivalent conditions.

Traceability: `gettext_quote`.

### Scenario 6: Release temporary quoting storage

A caller has used convenience wrappers that return module-managed quoted strings and wants to release associated storage.

- Input: call to `quotearg_free`.
- Expected behavior: internal slot-based storage used for wrapper-returned quoted strings is released/reset so future use does not depend on stale allocations.
- Test focus:
  - calling cleanup after wrapper usage succeeds without leaving previously returned slot storage retained;
  - repeated cleanup calls remain source-compatible in effect.

Traceability: `quotearg_free`, `struct slotvec`.

## Requirements

### Functional Requirements

#### FR-1: Locale-sensitive quote delimiter selection

The module shall provide quote delimiter selection through `gettext_quote`, returning a delimiter string based on the supplied message identifier and quoting style.

Traceability: `quotearg.c:gettext_quote`.

#### FR-2: Core quoted rendering for explicit byte input

The module shall provide a core formatting operation equivalent in behavior to `quotearg_buffer_restyled` that accepts:

- output buffer and output buffer size,
- input pointer and explicit input size,
- quoting style,
- flags,
- an optional per-character quote mask,
- left and right quote delimiters.

It shall produce the quoted rendering for the specified byte range and report the resulting size.

Traceability: `quotearg.c:quotearg_buffer_restyled`.

#### FR-3: Support configurable quoting behavior

The module shall support configurable quoting behavior represented by `struct quoting_options`, including at minimum the quoting style and option state needed by the wrapper functions and core rendering path.

Traceability: `struct quoting_options`, `quotearg_buffer_restyled`, `quotearg_char`.

#### FR-4: Quote ordinary NUL-terminated strings

The module shall provide behavior equivalent to `quotearg` for quoting a NUL-terminated input string using the module's default or wrapper-selected options.

Traceability: `quotearg.c:quotearg`.

#### FR-5: Quote explicit-length memory regions

The module shall provide behavior equivalent to `quotearg_mem` for quoting an input byte range of caller-supplied length.

Traceability: `quotearg.c:quotearg_mem`.

#### FR-6: Quote with a designated extra quoted character

The module shall provide behavior equivalent to `quotearg_char`, allowing one specified character to be treated specially by the quoting logic for that call.

Traceability: `quotearg.c:quotearg_char`, `struct quoting_options`.

#### FR-7: Provide default quote wrappers

The module shall provide wrapper behavior equivalent to `quote_mem` and `quote`, exposing default quoting for explicit-length and NUL-terminated inputs respectively.

Traceability: `quotearg.c:quote_mem`, `quotearg.c:quote`.

#### FR-8: Maintain wrapper-return storage and cleanup

The module shall maintain internal storage for convenience functions that return quoted strings and shall provide cleanup behavior equivalent to `quotearg_free` to release or reset that storage.

Traceability: `quotearg.c:quotearg_free`, `struct slotvec`.

#### FR-9: Respect explicit input length

For APIs that accept an explicit size, the module shall process exactly the provided number of input bytes, including any embedded NUL bytes.

Traceability: `quotearg_buffer_restyled`, `quotearg_mem`, `quote_mem`.

#### FR-10: Support caller-specified quote delimiters in the core formatter

Where the core formatter is called with explicit left and right quote delimiter arguments, the module shall use those delimiters in the produced quoted output in accordance with source behavior.

Traceability: `quotearg_buffer_restyled`, `gettext_quote`.

### Key Entities

#### `quoting_options`

A configuration entity representing how quoting is to be performed. The source analysis shows this structure is central to the module and reused across multiple wrapper paths.

Observed role in module behavior:

- carries the selected quoting style;
- carries additional option state used to influence rendering;
- is created or adjusted by wrapper functions to obtain desired quoting behavior for a specific call.

Relationships:

- consumed by logic equivalent to `quotearg_buffer_restyled`;
- specialized by wrappers such as `quotearg_char`;
- underlies the behavior exposed by `quotearg`, `quotearg_mem`, `quote_mem`, and `quote`.

Traceability: multiple `struct quoting_options` references throughout `quotearg.c`; functions `quotearg_buffer_restyled`, `quotearg_char`, `quotearg`, `quotearg_mem`, `quote_mem`, `quote`.

#### `slotvec`

A storage-management entity used for module-managed return buffers associated with convenience quoting calls.

Observed role in module behavior:

- tracks one or more internal slots for returned quoted strings;
- supports reuse and cleanup of internal quoted-string storage;
- is directly relevant to `quotearg_free` and wrapper-return behavior.

Relationships:

- used by convenience APIs that return quoted strings without requiring caller-supplied output buffers;
- reset or released by `quotearg_free`.

Traceability: `struct slotvec` references in `quotearg.c`; `quotearg_free`, `quotearg`, `quotearg_mem`, `quote_mem`, `quote`.

## Success Criteria

### Behavioral equivalence

1. For representative inputs used with `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, and `quote`, the Rust port produces the same quoted textual result as the source module under the same quoting configuration.
   - Traceability: `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`.

2. For representative combinations of quoting style, flags, explicit quote delimiters, and per-character quote masks, the Rust equivalent of `quotearg_buffer_restyled` returns output content and result size matching the source module.
   - Traceability: `quotearg_buffer_restyled`.

3. For quote delimiter lookup cases exercised by the source behavior, the Rust equivalent of `gettext_quote` returns the same delimiter strings as the source module for the same quoting style and locale-sensitive context.
   - Traceability: `gettext_quote`.

### Input handling correctness

4. Explicit-length quoting APIs process the entire specified byte range, including embedded NUL bytes, without premature termination.
   - Traceability: `quotearg_buffer_restyled`, `quotearg_mem`, `quote_mem`.

5. Zero-length input and empty-string input are handled without divergence from source behavior.
   - Traceability: `quotearg_buffer_restyled`, `quotearg`, `quotearg_mem`, `quote_mem`, `quote`.

6. Character-specific quoting through the `quotearg_char` path affects output when the designated character is present and does not introduce unrelated behavioral changes.
   - Traceability: `quotearg_char`, `struct quoting_options`.

### Storage lifecycle correctness

7. Convenience wrapper calls return usable quoted string storage consistent with the source module's wrapper behavior.
   - Traceability: `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`, `struct slotvec`.

8. After using wrapper-return APIs, invoking the Rust equivalent of `quotearg_free` releases or resets internal slot-managed storage with source-compatible effect.
   - Traceability: `quotearg_free`, `struct slotvec`.

9. Repeated invocation patterns combining wrapper calls and cleanup remain behaviorally consistent with the source module and do not depend on stale internal quoted-string state.
   - Traceability: `quotearg_free`, `struct slotvec`, wrapper functions.

## Out of Scope

The Rust port specification does not require any capability not evidenced by `quotearg.c`, including:

- new public quoting APIs beyond the analyzed functions;
- thread-safety or synchronization guarantees;
- persistence or serialization of quoting configuration;
- error recovery features beyond source-observable behavior;
- FFI-specific interfaces;
- benchmark or performance commitments.

## Traceability Summary

- Source file: `quotearg.c`
- Primary behavior sources:
  - `gettext_quote`
  - `quotearg_buffer_restyled`
  - `quotearg_free`
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
- Primary data entities:
  - `struct quoting_options`
  - `struct slotvec`