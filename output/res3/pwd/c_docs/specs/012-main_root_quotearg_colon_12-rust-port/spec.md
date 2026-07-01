# spec.md

## Title

Functional Specification: `main_root_quotearg_colon_12`

## Overview

This module provides colon-oriented argument quoting wrappers from `quotearg.c` for the `pwd` project rewrite in Rust.

The analyzed module exposes two entry points:

- `quotearg_colon(const char *arg)`
- `quotearg_colon_mem(const char *arg, size_t argsize)`

Its functional role is to produce a quoted representation of input text using quoting behavior configured for the colon character. The Rust rewrite must preserve the observable behavior of these wrapper functions, including support for both NUL-terminated string input and explicit-length byte input.

This specification is limited to the functionality evidenced by the provided functions and the associated quoting configuration data structures in `quotearg.c`.

## Feature Specification

### Summary

The Rust module must provide the same functional behavior as the C module’s colon-specific quoting wrappers:

- Quote an input argument using quoting rules specialized for colon handling.
- Support both:
  - a conventional C-string style input path, and
  - an explicit-size input path that can process data by length.
- Apply quoting through quoting options represented by the module’s quoting configuration structures.

### Supported behavior

1. **Colon-specific quoting**
   - The module must format input according to the quoting mode used by `quotearg_colon` and `quotearg_colon_mem`.
   - The behavior must reflect use of quoting options that treat `:` as a character requiring quoting/escaping under this wrapper.

2. **String-input quoting**
   - A caller can pass a NUL-terminated argument and receive its quoted form.
   - The resulting output must match the C module’s observable output for the same input.

3. **Explicit-length quoting**
   - A caller can pass a pointer plus byte length and receive the quoted form of exactly that byte sequence.
   - The operation must not depend on an interior NUL byte to terminate processing when length is explicitly supplied.

4. **Use of module-defined quoting configuration**
   - The wrapper behavior must be driven by the module’s quoting configuration entity rather than ad hoc formatting.
   - The Rust rewrite must preserve the relationship between the colon-specific wrapper and quoting options evidenced by `struct quoting_options`.

### Out of scope

The following are not required unless already necessary to preserve the observable behavior of these two functions:

- New public APIs beyond the Rust equivalents of the analyzed wrappers
- Additional quoting styles not evidenced by these wrappers
- Thread-safety guarantees
- Serialization or persistence of quoting options
- FFI-facing compatibility promises beyond functional equivalence

## User Scenarios & Testing

### Scenario 1: Quote a simple path-like argument containing a colon

A caller has an argument whose content includes `:` and needs the same quoted rendering as the original C wrapper.

**Expected support**
- Passing the argument through the Rust equivalent of `quotearg_colon` returns a quoted string.
- The output reflects colon-specific quoting behavior, not generic pass-through output.

**Test approach**
- Compare Rust output against C output for representative inputs such as:
  - `a:b`
  - `:`
  - `prefix:suffix`

### Scenario 2: Quote an argument without explicit length

A caller has ordinary text available as a NUL-terminated string and uses the convenience wrapper.

**Expected support**
- The Rust equivalent of `quotearg_colon` accepts ordinary string input and returns the quoted representation corresponding to the full string up to termination.

**Test approach**
- Compare Rust output against C output for:
  - empty string
  - plain alphanumeric text
  - text containing colon and shell-relevant punctuation if such punctuation affects the underlying quoting logic

### Scenario 3: Quote data with explicit byte length

A caller has data where processing must use a provided size rather than stop at the first NUL.

**Expected support**
- The Rust equivalent of `quotearg_colon_mem` quotes exactly the provided byte span.
- Interior NUL bytes are handled as data within the quoted result according to the underlying quoting behavior.

**Test approach**
- Compare Rust output against C output for byte slices containing:
  - interior NULs
  - trailing bytes after an interior NUL
  - colon bytes in combination with nonprintable bytes

### Scenario 4: Consistent behavior between convenience and explicit-length forms

A caller uses either wrapper for the same text content and expects the same result when the explicit length equals the full string length.

**Expected support**
- For inputs without interior NULs, the outputs of the Rust equivalents of `quotearg_colon` and `quotearg_colon_mem` are identical when applied to the same byte content.

**Test approach**
- For multiple strings, assert equality between:
  - `quotearg_colon(s)`
  - `quotearg_colon_mem(s.as_bytes(), s.len())`

### Scenario 5: Repeated use of the wrappers

A caller invokes the quoting wrappers multiple times during command execution.

**Expected support**
- Each call produces the correct quoted representation for its input.
- Repeated calls do not change the quoting rules for later calls.

**Test approach**
- Run sequential quoting calls on varied inputs and compare each result to the C implementation’s output.

## Requirements

### Functional Requirements

#### FR-1: Colon-wrapper quoting for NUL-terminated input
The module shall provide a Rust implementation corresponding to `quotearg_colon` that returns the quoted representation of a NUL-terminated input argument using the module’s colon-specific quoting behavior.

**Traceability:** `quotearg.c`, `quotearg_colon` [991-995], `struct quoting_options`

#### FR-2: Colon-wrapper quoting for explicit-length input
The module shall provide a Rust implementation corresponding to `quotearg_colon_mem` that returns the quoted representation of an input byte sequence using exactly the supplied length and the module’s colon-specific quoting behavior.

**Traceability:** `quotearg.c`, `quotearg_colon_mem` [997-1001], `struct quoting_options`

#### FR-3: Colon must be treated according to the wrapper’s configured quoting behavior
The module shall preserve the wrapper-specific treatment of the `:` character that distinguishes these functions from generic quoting entry points.

**Traceability:** `quotearg.c`, `quotearg_colon` [991-995], `quotearg_colon_mem` [997-1001], `struct quoting_options`

#### FR-4: Explicit-length processing must not rely on C-string termination
For the explicit-length variant, the module shall process the full provided byte range even when the data contains interior NUL bytes.

**Traceability:** `quotearg.c`, `quotearg_colon_mem` [997-1001]

#### FR-5: Equivalent results for equivalent content
When given the same byte content without interior NUL bytes, the string-input and explicit-length wrappers shall produce equivalent quoted output.

**Traceability:** `quotearg.c`, `quotearg_colon` [991-995], `quotearg_colon_mem` [997-1001]

#### FR-6: Wrapper behavior shall be derived from quoting configuration entities
The Rust rewrite shall preserve the functional dependency on the module’s quoting configuration structures used to define quoting behavior, rather than implementing unrelated custom formatting.

**Traceability:** `quotearg.c`, `struct quoting_options` occurrences including [57-74], [952], [960], [979], [1006], [1025], [1047]

### Key Entities

#### `quoting_options`
This structure family represents the quoting configuration used by the module. For this analyzed wrapper module, it is the key behavioral entity that determines how input is quoted, including the colon-specific behavior used by the two exported wrappers.

**Role**
- Defines quoting behavior applied by the wrapper functions.
- Encodes the rule set that the Rust rewrite must preserve at the observable-output level.

**Traceability:** `quotearg.c`, `struct quoting_options`

#### `slotvec`
This structure appears in the module as storage associated with quoted results. It indicates that quoted output is managed through module-level result slots in the original implementation.

For the Rust rewrite, this entity is relevant only insofar as externally visible behavior must be preserved; the specification does not require reproducing the original storage strategy.

**Traceability:** `quotearg.c`, `struct slotvec` [829-833], [839], [840], [845], [878]

#### Relationship between entities
- The wrapper functions apply quoting behavior defined by `quoting_options`.
- The original C module uses slot-based result storage (`slotvec`) to hold produced quoted strings.
- The Rust rewrite must preserve output behavior of the wrappers, while internal storage may differ if behavior remains equivalent.

## Success Criteria

### SC-1: Output equivalence for `quotearg_colon`
For a representative conformance test set of NUL-terminated inputs, the Rust implementation of the `quotearg_colon` equivalent shall produce the same quoted text as the C implementation.

**Traceability:** `quotearg_colon` [991-995]

### SC-2: Output equivalence for `quotearg_colon_mem`
For a representative conformance test set of explicit-length inputs, including inputs with interior NUL bytes, the Rust implementation of the `quotearg_colon_mem` equivalent shall produce the same quoted text as the C implementation.

**Traceability:** `quotearg_colon_mem` [997-1001]

### SC-3: Colon-sensitive cases are preserved
For test inputs containing one or more `:` characters, the Rust implementation shall match the C implementation’s quoted output exactly.

**Traceability:** `quotearg_colon` [991-995], `quotearg_colon_mem` [997-1001], `struct quoting_options`

### SC-4: Consistency between wrapper forms
For inputs without interior NUL bytes, the Rust equivalents of the string-input and explicit-length wrappers shall return identical output when given the same content.

**Traceability:** `quotearg_colon` [991-995], `quotearg_colon_mem` [997-1001]

### SC-5: No truncation in explicit-length mode
For explicit-length tests containing interior NUL bytes followed by additional data, the Rust implementation shall include the effect of bytes after the interior NUL in the quoted result, matching the C implementation.

**Traceability:** `quotearg_colon_mem` [997-1001]