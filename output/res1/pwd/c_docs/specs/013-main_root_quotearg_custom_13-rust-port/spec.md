# spec.md

## Title

Functional Specification: `main_root_quotearg_custom_13`

## Overview

This module provides custom-quote argument formatting for the `pwd` project through two entry points in `quotearg.c`:

- `quotearg_custom`
- `quotearg_custom_mem`

Its functional role is to return a quoted representation of input data using caller-provided left and right quote strings. The Rust rewrite must preserve the observable behavior of these custom quoting entry points and their reliance on the module’s quoting configuration model.

The module is part of the main execution cluster and operates as a formatting utility for producing quoted strings suitable for display or messaging. The available evidence supports custom quoting of either:

- a NUL-terminated string input, or
- a byte sequence with explicit length.

## Feature Specification

### Summary

The Rust version must implement custom quoting for arbitrary argument content using caller-supplied opening and closing quote delimiters.

### Supported functionality

1. **Custom quoting of NUL-terminated text**
   - Given `left_quote`, `right_quote`, and `arg`, the module returns a quoted string representation of `arg`.
   - The output must use the caller-provided left and right quote delimiters.

2. **Custom quoting of explicit-length input**
   - Given `left_quote`, `right_quote`, `arg`, and `argsize`, the module returns a quoted string representation of exactly `argsize` bytes from `arg`.
   - This behavior must not depend on the presence of a terminating NUL within the input range.

3. **Use of quoting options for custom style selection**
   - The custom-quote operations are defined in terms of the module’s quoting options structure.
   - The Rust rewrite must preserve the behavior that these entry points apply a custom quoting style driven by the supplied quote delimiters.

4. **Returned quoted result**
   - Both entry points return a character string result representing the formatted argument.
   - The returned result must contain the quoted form, not the original unmodified input.

## User Scenarios & Testing

### Scenario 1: Quote a normal path-like string with custom delimiters

A caller has a regular C-style string and wants it displayed with non-default delimiters, such as `<<` and `>>`.

- Input:
  - left quote: `<<`
  - right quote: `>>`
  - argument: `home/user`
- Expected behavior:
  - The result is a newly returned quoted representation beginning with `<<` and ending with `>>`.
  - The content corresponds to the provided argument as processed by the module’s custom quoting behavior.

### Scenario 2: Quote an argument using asymmetric quote markers

A caller wants a string displayed with different opening and closing markers, such as `(` and `)`.

- Input:
  - left quote: `(`
  - right quote: `)`
  - argument: `tmp`
- Expected behavior:
  - The result starts with `(` and ends with `)`.
  - The returned string is the custom-quoted form of the argument.

### Scenario 3: Quote data containing embedded NUL bytes

A caller has byte data that may include embedded NULs and therefore cannot be processed correctly as a plain C string.

- Input:
  - left quote: `"`
  - right quote: `"`
  - argument bytes: `a\0b`
  - size: `3`
- Expected behavior:
  - The explicit-length API processes exactly 3 bytes.
  - The result reflects quoting of the full byte sequence, not truncation at the first NUL.

### Scenario 4: Quote an empty argument

A caller provides an empty string or a zero-length byte sequence and still expects the configured delimiters to be applied.

- Input:
  - left quote: `'`
  - right quote: `'`
  - argument: empty
- Expected behavior:
  - The result is the quoted representation of an empty argument using the supplied delimiters.

### Scenario 5: Distinguish string and memory-based entry points

A caller selects the API based on the input form:

- `quotearg_custom` for NUL-terminated text
- `quotearg_custom_mem` for counted byte data

Expected behavior:
- Both produce custom-quoted output using the same quote delimiters.
- The memory-based function must honor the explicit size.
- The string-based function must operate on the NUL-terminated argument.

### Testing guidance

The Rust rewrite should be tested with:
- ordinary ASCII strings,
- empty strings,
- multi-character left and right quote delimiters,
- asymmetric delimiters,
- explicit-length inputs with embedded NUL bytes,
- explicit-length inputs where `argsize` is shorter than the backing buffer’s remaining content.

Each test should verify:
- output begins with the supplied left quote,
- output ends with the supplied right quote,
- the correct API-specific input extent is used,
- the returned value is a quoted representation rather than a pass-through copy.

## Requirements

### Functional Requirements

#### FR-1: Custom quoting for string input
The module shall provide a function corresponding to `quotearg_custom` that accepts a left quote string, a right quote string, and a NUL-terminated argument string, and returns the quoted representation of that argument using the supplied delimiters.
**Traceability:** `quotearg.c:1030-1035` (`quotearg_custom`)

#### FR-2: Custom quoting for explicit-length input
The module shall provide a function corresponding to `quotearg_custom_mem` that accepts a left quote string, a right quote string, a pointer to argument data, and an explicit byte length, and returns the quoted representation of exactly that input extent using the supplied delimiters.
**Traceability:** `quotearg.c:1037-1043` (`quotearg_custom_mem`)

#### FR-3: Caller-provided quote delimiters
The module shall use caller-supplied `left_quote` and `right_quote` values as the delimiters for the produced quoted result in both supported entry points.
**Traceability:** `quotearg.c:1030-1043` (`quotearg_custom`, `quotearg_custom_mem`)

#### FR-4: Shared quoting configuration model
The module shall preserve the role of `struct quoting_options` as the configuration entity that defines quoting behavior for these custom quoting operations.
**Traceability:** `quotearg.c` references to `struct quoting_options`, including `quotearg.c:57-74`, `782`, `784`, `795`, `808`, `810`, `874`, `952`, `960`, `979`, `1006`, `1025`, `1047`

#### FR-5: Return quoted string output
The module shall return a character-string result for each custom quoting operation.
**Traceability:** `quotearg.c:1030-1043` return types of `quotearg_custom` and `quotearg_custom_mem`

### Key Entities

#### `quoting_options`
The core configuration entity for quoting behavior in this module.

Functional role:
- represents the quoting configuration used by quoting operations,
- provides the configuration context that custom quoting entry points depend on,
- links supplied custom delimiters to the formatting behavior.

Relationship to module functions:
- `quotearg_custom` and `quotearg_custom_mem` operate within this quoting-options model rather than as unrelated standalone string wrappers.

**Traceability:** `quotearg.c` multiple `struct quoting_options` references, including `57-74` and later operational references through `1047`

#### `slotvec`
A storage-related entity present in the same module and associated with management of quoted argument results.

Functional role evidenced by presence:
- participates in handling returned quoted-string storage within the module’s quoting subsystem.

Relationship to module functions:
- supports the broader quoting result lifecycle used by quoting operations in `quotearg.c`.

Because the provided evidence names the type but does not fully expose its behavior, the Rust rewrite must preserve only the necessary observable behavior of returned results, not any specific internal storage layout.

**Traceability:** `quotearg.c:829-833`, `839`, `840`, `845`, `878`

## Success Criteria

1. **API coverage**
   - The Rust rewrite exposes behaviorally equivalent functionality for both custom quoting entry points: one for NUL-terminated string input and one for explicit-length input.
   **Traceability:** `quotearg_custom`, `quotearg_custom_mem`

2. **Delimiter correctness**
   - For tested inputs, each returned result begins with the supplied left quote and ends with the supplied right quote.
   **Traceability:** `quotearg.c:1030-1043`

3. **Explicit-length correctness**
   - For inputs containing embedded NUL bytes, the Rust equivalent of `quotearg_custom_mem` processes the full specified byte length rather than stopping at the first NUL.
   **Traceability:** `quotearg.c:1037-1043`

4. **Empty-input correctness**
   - For empty string and zero-length memory inputs, the Rust implementation returns a valid quoted result using the supplied delimiters.
   **Traceability:** custom quoting behavior exposed by `quotearg_custom` and `quotearg_custom_mem`

5. **Configuration-model preservation**
   - The Rust design preserves a distinct quoting-configuration concept corresponding to `struct quoting_options`, and the custom quoting entry points operate through that model.
   **Traceability:** `struct quoting_options` references throughout `quotearg.c`

6. **Observable behavior parity**
   - For the same delimiters and same effective input extent, the Rust rewrite produces quoted output consistent with the C module’s custom quoting behavior in all covered scenarios.
   **Traceability:** `quotearg.c:1030-1043`, associated `struct quoting_options` usage