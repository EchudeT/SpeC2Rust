# spec.md

## Title
Rust Functional Specification for `module_gnu_vasnprintf.c_52`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_52`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `058-module_gnu_vasnprintf.c_52-rust-port`
- Generation date: `2026-06-11`

## Overview
This module provides internal support behavior used by formatted string production in `gnu/vasnprintf.c`. The analyzed portion covers:

- bounded and unbounded character/wide-character length inspection,
- fallback conversion of a single wide character into a multibyte form,
- retrieval of the locale decimal-point character,
- arithmetic over an internal multiple-precision numeric representation,
- conversion of that internal numeric representation into a decimal character string.

The Rust rewrite must preserve the observed functional behavior of these helpers so that higher-level formatting logic can rely on equivalent results.

## Feature Specification

### Summary
The Rust version must implement the following functional areas evidenced in the analyzed source:

1. Length calculation for narrow strings with a maximum bound.
2. Length calculation for wide-character strings, both bounded and unbounded.
3. Single wide-character to multibyte conversion, including a fallback path.
4. Retrieval of the active decimal-point character for numeric formatting support.
5. Multiplication and division over the module’s internal multi-precision number representation.
6. Conversion of an internal multi-precision value into a decimal digit string, with optional appended zero digits.

### In Scope
- Functional equivalence for the helper behaviors represented by:
  - `local_strnlen`
  - `local_wcslen`
  - `local_wcsnlen`
  - `wctomb_fallback`
  - `local_wcrtomb`
  - `local_wctomb`
  - `decimal_point_char`
  - `multiply`
  - `divide`
  - `convert_to_decimal`
- Preservation of behavior that higher-level formatting code depends on:
  - correct stopping at bounds or terminators,
  - correct handling of wide-character conversion attempts,
  - correct decimal separator lookup,
  - correct arithmetic and decimal rendering for internal numeric values.

### Out of Scope
The following are not evidenced by the analyzed module segment and must not be added as requirements:
- new public APIs,
- thread-safety guarantees,
- serialization or persistence,
- error recovery beyond source-evidenced behavior,
- external formatting features not directly supported by the listed functions,
- benchmarking or performance targets.

## User Scenarios & Testing

### Scenario 1: Bounded scan of a possibly unterminated byte string
A formatting path needs to know how many bytes can be consumed from a character buffer without reading past a caller-provided limit.

The Rust module must support:
- returning the number of bytes before the first NUL byte if one occurs within the limit,
- otherwise returning the limit.

#### Test expectations
- Given a buffer containing a NUL before `maxlen`, the returned length equals the offset of the first NUL.
- Given a buffer with no NUL in the first `maxlen` bytes, the returned length equals `maxlen`.
- Given `maxlen == 0`, the returned length is `0`.

### Scenario 2: Scan of a wide-character string
A formatting path needs the length of a wide-character string for width or precision handling.

The Rust module must support:
- unbounded counting until the terminating wide NUL,
- bounded counting up to either the wide NUL or a maximum length.

#### Test expectations
- For a terminated wide string, the unbounded result equals the count before the terminator.
- For a bounded scan with a terminator before the bound, the result equals the count before the terminator.
- For a bounded scan with no terminator before the bound, the result equals the bound.

### Scenario 3: Convert one wide character for formatted output
A formatting path needs to emit a single wide character as multibyte output and must work even when normal conversion support is unavailable or insufficient.

The Rust module must support:
- attempting conversion of one wide character into a multibyte sequence,
- behavior equivalent to the source’s local conversion wrappers and fallback path,
- reporting the produced byte count according to source behavior.

#### Test expectations
- For representable input, conversion returns a nonzero produced length and writes corresponding bytes.
- For NUL wide character input, conversion behavior matches source semantics for a single converted character.
- For values requiring fallback behavior, the produced bytes and returned count match the source-defined fallback path.

### Scenario 4: Obtain decimal separator used in numeric formatting
A formatting path needs the locale-relevant decimal point character when assembling numeric text.

The Rust module must support:
- retrieving the decimal-point character used by the source logic,
- producing a single byte character result suitable for formatted output assembly.

#### Test expectations
- The returned value is a single character.
- In environments where locale data provides a decimal point, the result matches the source behavior.
- If locale lookup does not provide a usable decimal point, the result matches the source fallback behavior.

### Scenario 5: Multiply internal multi-precision numeric values
A numeric formatting path needs to scale an internal number by another internal number.

The Rust module must support:
- taking two internal multi-precision values,
- producing their mathematical product in the same internal representation.

#### Test expectations
- Multiplying by zero yields zero in the output representation.
- Multiplying by one preserves the other operand’s value.
- For representative nontrivial operands, the resulting internal value matches exact arithmetic.

### Scenario 6: Divide internal multi-precision numeric values
A numeric formatting path needs quotient computation over internal multi-precision values.

The Rust module must support:
- dividing one internal multi-precision value by another according to the source behavior,
- producing the quotient in the internal representation used by the module.

#### Test expectations
- Division by a larger positive value yields a quotient representing zero when source logic does so.
- Division of equal values yields a quotient representing one when source logic does so.
- For representative exact and non-exact divisions, the quotient matches source behavior.

### Scenario 7: Render internal multi-precision value as decimal digits
A formatting path needs to convert an internal numeric value to decimal text and optionally append extra trailing zero digits.

The Rust module must support:
- converting the value to base-10 digit characters,
- appending exactly the requested number of extra zero digits,
- returning the decimal string in the form expected by higher-level formatting code.

#### Test expectations
- Zero converts to the source-equivalent decimal text for zero.
- Positive values convert to their exact decimal digit sequence.
- Providing `extra_zeroes = n` appends exactly `n` `'0'` characters after the converted digits.
- No non-digit characters are introduced by this conversion step itself.

## Requirements

### Functional Requirements

#### FR-1: Bounded byte-string length
The module shall provide bounded length calculation for a byte string, returning the count of bytes preceding the first terminator byte within a supplied maximum, or the maximum when no terminator occurs within that range.

**Traceability:** `local_strnlen` in `gnu/vasnprintf.c`

#### FR-2: Wide-string length
The module shall provide unbounded length calculation for a wide-character string, returning the count of wide characters preceding the terminating wide NUL.

**Traceability:** `local_wcslen` in `gnu/vasnprintf.c`

#### FR-3: Bounded wide-string length
The module shall provide bounded length calculation for a wide-character string, returning the count of wide characters preceding the terminating wide NUL within a supplied maximum, or the maximum when no terminator occurs within that range.

**Traceability:** `local_wcsnlen` in `gnu/vasnprintf.c`

#### FR-4: Single wide-character multibyte conversion
The module shall support conversion of a single wide character to a multibyte representation through the local conversion helpers, including source-equivalent fallback behavior where the primary conversion path is not used or not available.

**Traceability:** `wctomb_fallback`, `local_wcrtomb`, `local_wctomb` in `gnu/vasnprintf.c`

#### FR-5: Decimal point character lookup
The module shall provide the decimal-point character used by this formatting subsystem, matching source behavior for locale-based lookup and any source-defined fallback result.

**Traceability:** `decimal_point_char` in `gnu/vasnprintf.c`

#### FR-6: Multi-precision multiplication
The module shall support multiplication of two values in the module’s internal multi-precision numeric representation and produce the exact product in that representation.

**Traceability:** `multiply` in `gnu/vasnprintf.c`

#### FR-7: Multi-precision division quotient
The module shall support division of one internal multi-precision numeric value by another and produce the quotient in the module’s internal numeric representation, matching source behavior.

**Traceability:** `divide` in `gnu/vasnprintf.c`

#### FR-8: Decimal conversion of internal multi-precision value
The module shall convert an internal multi-precision numeric value into a decimal digit string and append an exact caller-specified number of trailing zero digits.

**Traceability:** `convert_to_decimal` in `gnu/vasnprintf.c`

### Key Entities

#### Internal multi-precision number
The module uses an internal multi-precision numeric entity (`mpn_t` as referenced by arithmetic and conversion functions) to represent values operated on by multiplication, division, and decimal conversion.

Relationship:
- consumed by `multiply`,
- consumed by `divide`,
- consumed by `convert_to_decimal`,
- produced or updated as arithmetic results for subsequent conversion.

**Traceability:** `multiply`, `divide`, `convert_to_decimal` in `gnu/vasnprintf.c`

#### Internal anonymous struct associated with numeric operations
The analyzed file contains an anonymous struct in the arithmetic region. The Rust rewrite must preserve any data relationship required by the evidenced arithmetic and decimal-conversion behavior, without inventing new externally visible semantics.

Relationship:
- associated with the internal numeric-processing portion of the module.

**Traceability:** anonymous struct at `gnu/vasnprintf.c:426-430`

#### Narrow string input
A NUL-terminated byte string, optionally subject to a maximum readable length, is used for bounded length inspection.

Relationship:
- consumed by bounded byte-string length logic.

**Traceability:** `local_strnlen` in `gnu/vasnprintf.c`

#### Wide string input
A NUL-terminated wide-character string, optionally subject to a maximum readable length, is used for wide-string length inspection and per-character conversion.

Relationship:
- consumed by `local_wcslen`,
- consumed by `local_wcsnlen`,
- individual wide characters consumed by `wctomb_fallback`, `local_wcrtomb`, and `local_wctomb`.

**Traceability:** `local_wcslen`, `local_wcsnlen`, `wctomb_fallback`, `local_wcrtomb`, `local_wctomb` in `gnu/vasnprintf.c`

#### Decimal digit string result
The decimal conversion step produces a character string consisting of decimal digits, with optional appended zero digits.

Relationship:
- produced by `convert_to_decimal`,
- used by higher-level formatting assembly outside this analyzed segment.

**Traceability:** `convert_to_decimal` in `gnu/vasnprintf.c`

## Success Criteria

1. The Rust implementation returns the same bounded byte-string lengths as the source behavior for:
   - empty strings,
   - strings terminated before the bound,
   - strings not terminated within the bound,
   - zero-length bounds.
   **Traceability:** `local_strnlen`

2. The Rust implementation returns the same wide-string lengths as the source behavior for both bounded and unbounded scans across representative terminated and bound-limited inputs.
   **Traceability:** `local_wcslen`, `local_wcsnlen`

3. The Rust implementation converts single wide characters to multibyte output with results matching the source behavior for:
   - ordinary representable characters,
   - the wide NUL character,
   - inputs requiring the fallback path.
   **Traceability:** `wctomb_fallback`, `local_wcrtomb`, `local_wctomb`

4. The Rust implementation returns the same decimal-point character as the source behavior in environments covered by the project’s tests, including source-defined fallback behavior where applicable.
   **Traceability:** `decimal_point_char`

5. The Rust implementation produces exact arithmetic results equivalent to the source for representative multi-precision multiplication cases, including identity and zero cases.
   **Traceability:** `multiply`

6. The Rust implementation produces quotient results equivalent to the source for representative multi-precision division cases, including dividend smaller than divisor and equal-operand cases.
   **Traceability:** `divide`

7. The Rust implementation converts representative internal multi-precision values into exact decimal digit strings matching source behavior, and appends exactly the requested number of extra zero digits.
   **Traceability:** `convert_to_decimal`

8. All required behaviors are implemented without introducing requirements or externally visible capabilities not evidenced by the analyzed source segment.
   **Traceability:** entire analyzed module segment in `gnu/vasnprintf.c`

## Acceptance Notes
- Equivalence is defined against the behavior of the analyzed C module, not against newly designed APIs.
- Test fixtures for arithmetic and decimal conversion should include cross-checking against source-produced outputs for the same internal values.
- Locale-sensitive verification for decimal point behavior should be performed only to the extent supported by the project’s test environment and source behavior.