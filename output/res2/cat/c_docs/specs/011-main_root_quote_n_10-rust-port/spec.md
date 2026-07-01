# spec.md

## Title

Functional Specification: `main_root_quote_n_10`

## Metadata

- Project: `cat`
- Module: `main_root_quote_n_10`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary functions:
  - `quote_n_mem`
  - `quote_n`
- Rust target branch: `011-main_root_quote_n_10-rust-port`
- Generation date: 2026-06-07

## Overview

This module provides indexed quoting of argument data and returns a quoted string for later use by callers. It supports quoting either:

- a byte sequence with an explicit length, via `quote_n_mem`, or
- a NUL-terminated string, via `quote_n`.

The module is part of a larger quoting subsystem evidenced by use of `struct quoting_options` and `struct slotvec` in `quotearg.c`. Within this module boundary, the required behavior is limited to producing quoted text for a caller-specified slot index and returning a pointer/reference to the resulting quoted representation.

The Rust rewrite must preserve the observable behavior of these entry points as module-level quoting helpers that:
- accept an integer slot number,
- quote the supplied argument content,
- and return the quoted result associated with that slot.

## Feature Specification

### Supported functionality

The Rust version must implement the following module behavior evidenced by `quotearg.c`:

1. **Quote a memory region by slot index**
   - `quote_n_mem(n, arg, argsize)` must accept a slot index, a pointer/reference to argument bytes, and an explicit byte count.
   - It must produce and return the quoted representation of exactly the provided byte range.

2. **Quote a NUL-terminated string by slot index**
   - `quote_n(n, arg)` must accept a slot index and a string argument.
   - It must produce and return the quoted representation of the full input string up to its terminating NUL semantics as represented in Rust.

3. **Preserve slot-based result access**
   - Results are associated with the caller-provided index `n`, as evidenced by the `quote_n*` naming and the presence of `slotvec`.
   - Repeated use with different indices must allow callers to obtain quoted results for those indices without collapsing all calls into a single indistinguishable output location.

4. **Use the module’s quoting configuration domain**
   - The module participates in the existing quoting system evidenced by `struct quoting_options`.
   - The Rust rewrite must remain behaviorally compatible with the quoting behavior expected by these functions in the source module, rather than inventing a new quoting policy.

### Out of scope

The Rust version must not claim or introduce capabilities not evidenced here, including:
- new public quoting APIs beyond the two listed entry points,
- thread-safety guarantees,
- persistence or serialization of quoting state,
- FFI guarantees,
- custom recovery behavior.

## User Scenarios & Testing

### Scenario 1: Quote a known-length byte buffer

A caller has argument data that may include bytes not conveniently represented as a normal Rust string slice length-wise. The caller supplies:
- slot index `n`,
- byte content,
- exact byte length.

The module returns a quoted representation of exactly that byte span.

**Test expectations**
- The returned value is non-empty when quoting requires visible delimiters or escaping for the input under the module’s quoting rules.
- The result reflects only the supplied `argsize` bytes.
- Supplying the same bytes and size to the same slot produces the same textual quoted result.

### Scenario 2: Quote a normal string argument

A caller has a standard string argument and wants a quoted version for diagnostics or display. The caller supplies:
- slot index `n`,
- string content.

The module returns the quoted representation of that string.

**Test expectations**
- The result matches the quoting behavior of the source module for equivalent input.
- The returned text corresponds to the whole input string.
- For inputs that do not need special handling, the output remains consistent with the source function’s quoting style.

### Scenario 3: Maintain distinct results for different slot indices

A caller needs multiple quoted arguments available through separate indexed calls. The caller invokes the module with:
- one input under slot `0`,
- another input under slot `1`.

The module must keep these logically distinct by slot.

**Test expectations**
- Retrieving/observing the result of slot `0` after quoting slot `1` still yields the slot `0` quoted content, unless the source behavior for slot storage explicitly replaces that same slot.
- Different slot indices can represent different quoted inputs within the same calling context.

### Scenario 4: Reuse a slot with new input

A caller reuses the same slot index for a different argument.

**Test expectations**
- The quoted result for that slot reflects the most recent input for that same slot.
- No stale earlier content is returned for that slot after replacement.

### Scenario 5: Equivalence between string and explicit-length forms for plain text

A caller passes the same plain text through both entry points:
- as a Rust string through `quote_n`,
- as bytes plus exact length through `quote_n_mem`.

**Test expectations**
- For identical textual content without embedded truncation differences, both entry points produce equivalent quoted output under the module’s quoting rules.

## Requirements

### Functional Requirements

#### FR-1: Indexed quoting of explicit-length input
The module shall provide functionality equivalent to `quote_n_mem` from `quotearg.c`, accepting a slot index and explicit-length argument data, and returning that input’s quoted representation.

**Traceability:** `quotearg.c`, `quote_n_mem`

#### FR-2: Indexed quoting of string input
The module shall provide functionality equivalent to `quote_n` from `quotearg.c`, accepting a slot index and string input, and returning that input’s quoted representation.

**Traceability:** `quotearg.c`, `quote_n`

#### FR-3: Slot-associated result handling
The module shall preserve per-index result association consistent with the source module’s use of slot storage.

**Traceability:** `quotearg.c`, `quote_n_mem`, `quote_n`, `struct slotvec`

#### FR-4: Length-bounded processing for memory input
For the explicit-length entry point, the module shall quote exactly the provided input extent and shall not require NUL-termination semantics for that path.

**Traceability:** `quotearg.c`, `quote_n_mem`

#### FR-5: Compatibility with existing quoting behavior
The module shall remain aligned with the quoting behavior domain represented by `struct quoting_options` in the source file, so that outputs from these entry points are consistent with the original module’s quoting rules.

**Traceability:** `quotearg.c`, `struct quoting_options`, `quote_n_mem`, `quote_n`

### Key Entities

#### `quoting_options`
Represents the quoting configuration domain used by the surrounding quoting subsystem. Within this module’s functional boundary, it defines the quoting behavior that `quote_n_mem` and `quote_n` participate in and must remain compatible with.

**Traceability:** `struct quoting_options` entries in `quotearg.c`

#### `slotvec`
Represents indexed storage for quoted results. Its presence evidences that quoted output is managed per slot/index rather than as a single undifferentiated return buffer.

**Traceability:** `struct slotvec` entries in `quotearg.c`

#### Quoted result for slot `n`
The observable product of this module is the quoted textual representation returned for a given slot index. This result is produced from either:
- explicit-length bytes (`quote_n_mem`), or
- string input (`quote_n`).

**Traceability:** `quote_n_mem`, `quote_n`

## Success Criteria

1. **Entry-point coverage**
   - The Rust module exposes behaviorally equivalent functionality for both `quote_n_mem` and `quote_n`.
   - Traceability: `quotearg.c`, `quote_n_mem`, `quote_n`

2. **Correct input domain handling**
   - Tests show that explicit-length input is processed according to the supplied length, and string input is processed as a full string.
   - Traceability: `quote_n_mem`, `quote_n`

3. **Slot distinction**
   - Tests using at least two different slot indices demonstrate that outputs remain associated with their respective indices.
   - Traceability: `quote_n_mem`, `quote_n`, `struct slotvec`

4. **Slot replacement behavior**
   - Tests show that reusing a slot index updates that slot’s quoted result to match the latest input.

5. **Behavioral parity with source quoting**
   - For a representative set of inputs used by this module, Rust output matches the source module’s output for both entry points.
   - Traceability: `quotearg.c`, `quote_n_mem`, `quote_n`, `struct quoting_options`

6. **No unsupported feature expansion**
   - The Rust rewrite does not document or depend on new public capabilities beyond the evidenced module behavior.
   - Traceability: constrained by `quotearg.c` module evidence