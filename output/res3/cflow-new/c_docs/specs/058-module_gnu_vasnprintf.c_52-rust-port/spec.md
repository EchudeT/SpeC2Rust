# spec.md

## Title

Rust Functional Specification for `module_gnu_vasnprintf.c_52`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_52`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `058-module_gnu_vasnprintf.c_52-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides internal support functions used by formatted string construction logic. Its evidenced responsibilities are:

- bounded and unbounded length calculation for narrow and wide character strings
- locale-aware retrieval of the decimal point character
- conversion of wide characters to multibyte output through local wrappers and fallback behavior
- arithmetic on an internal multi-precision numeric representation, specifically multiplication and division
- conversion of the internal multi-precision representation into a decimal digit string, including requested trailing zero extension

The Rust rewrite must preserve these functional behaviors so that higher-level formatting logic can rely on the same results and edge-case handling boundaries.

## Scope

### In Scope

The Rust version must implement the functionality evidenced by the analyzed module functions:

- narrow string length up to a maximum bound
- wide string full length and bounded length
- wide character to multibyte conversion through:
  - a fallback encoder
  - a restartable/local wrapper
  - a non-restartable/local wrapper
- retrieval of the active decimal separator as a single byte character
- multiplication of two internal multi-precision numeric values into a destination value
- division of one internal multi-precision numeric value by another, producing a quotient value
- conversion of an internal multi-precision numeric value to a decimal string with optional extra zeroes

### Out of Scope

The following are not evidenced by this module input and must not be added as requirements:

- new public APIs beyond what is needed by the surrounding formatter implementation
- generalized arbitrary-precision math beyond the operations listed above
- thread-safety guarantees
- serialization or persistence
- FFI interfaces
- recovery workflows, logging, or benchmark targets

## Feature Specification

### Feature 1: Local string length helpers

The module supplies internal helpers for measuring string lengths needed during formatting operations.

The Rust version must implement:

- length of a narrow character string, stopping at either the first terminator or a caller-provided maximum bound
- length of a wide character string up to the first terminator
- length of a wide character string, stopping at either the first terminator or a caller-provided maximum bound

These helpers must behave as pure length calculations and must not read beyond the supplied bound in bounded forms.

Traceability:
- `local_strnlen`
- `local_wcslen`
- `local_wcsnlen`

### Feature 2: Wide character to multibyte conversion support

The module supplies internal support for converting a single wide character into a multibyte form for formatted output generation.

The Rust version must implement:

- a fallback conversion path for a single wide character into bytes
- a local restartable conversion wrapper that accepts conversion state
- a local non-restartable conversion wrapper

Behaviorally, the Rust version must preserve the distinction between:
- conversion through a restartable interface
- conversion through a simpler wrapper
- fallback handling when direct conversion support is unavailable or unsuitable

The module’s responsibility is limited to producing the converted byte sequence size and output bytes for one wide character, as evidenced by the source functions.

Traceability:
- `wctomb_fallback`
- `local_wcrtomb`
- `local_wctomb`

### Feature 3: Decimal point character retrieval

The module retrieves the decimal point character used for formatting-related numeric output.

The Rust version must implement retrieval of a single character representing the decimal separator relevant to the active locale context as used by this module.

If the surrounding runtime environment exposes locale-dependent decimal formatting, the Rust rewrite must return the same effective single-byte decimal separator expected by higher-level formatting code.

Traceability:
- `decimal_point_char`

### Feature 4: Internal multi-precision arithmetic support

The module performs arithmetic on an internal multi-precision numeric representation used by formatting logic.

The Rust version must implement:

- multiplication of two multi-precision values into a destination multi-precision value
- division of one multi-precision value by another, producing a quotient multi-precision value

These operations must support the downstream decimal conversion workflow and therefore must preserve arithmetic correctness for the internal representation accepted by this module.

Traceability:
- `multiply`
- `divide`
- anonymous struct at `gnu/vasnprintf.c:426-430`

### Feature 5: Decimal conversion from internal numeric representation

The module converts an internal multi-precision numeric value into a decimal digit string.

The Rust version must implement conversion that:

- accepts an internal multi-precision numeric value
- returns a decimal string representation of that value
- appends the requested number of extra trailing zeroes to the decimal output

The result must be suitable for use by higher-level formatting logic expecting decimal digits as text.

Traceability:
- `convert_to_decimal`

## User Scenarios & Testing

### Scenario 1: Bounded narrow string inspection during formatting

A formatting path needs to determine how many bytes from a narrow string argument can be consumed without scanning past a precision or safety limit.

Expected support:
- the module returns the smaller of:
  - the number of bytes before the terminator
  - the provided maximum bound

Validation:
- input shorter than the bound returns true string length
- input longer than the bound returns the bound
- empty string returns zero

Traceability:
- `local_strnlen`

### Scenario 2: Wide string inspection for field width or precision handling

A formatting path needs the length of a wide string, either complete or capped to a maximum count.

Expected support:
- complete wide string length is returned up to the first terminator
- bounded wide string length stops at the terminator or maximum count, whichever occurs first

Validation:
- empty wide string returns zero
- bounded length on a longer string returns the bound
- complete length matches the count of non-terminator wide characters

Traceability:
- `local_wcslen`
- `local_wcsnlen`

### Scenario 3: Emitting a single wide character into multibyte output

A formatting path is producing output from a wide character conversion and needs the byte form of one wide character.

Expected support:
- a conversion routine produces the multibyte bytes for the character
- the reported size matches the emitted bytes
- local wrappers behave consistently with the fallback path for valid input within their supported environment assumptions

Validation:
- a basic single-byte character converts to one output byte where appropriate
- a non-ASCII wide character either converts according to the active encoding support or follows the fallback behavior implemented by the module
- the restartable and non-restartable wrappers provide equivalent output for the same character when both are applicable

Traceability:
- `wctomb_fallback`
- `local_wcrtomb`
- `local_wctomb`

### Scenario 4: Obtaining the decimal separator for numeric formatting

A formatting path needs the character used between integer and fractional parts.

Expected support:
- the module returns one byte representing the decimal separator used by the active locale-sensitive formatting path

Validation:
- in environments using `.` as decimal separator, the returned character is `.`
- in environments where a different single-byte decimal separator applies, the returned character matches that separator

Traceability:
- `decimal_point_char`

### Scenario 5: Converting internal large numeric state into decimal digits

A formatting path has constructed an internal multi-precision numeric value and needs decimal digits for output.

Expected support:
- multiplication and division operations correctly transform internal values as needed by formatting
- decimal conversion returns the expected decimal text for the internal numeric value
- requested extra zeroes are appended to the generated decimal text

Validation:
- multiplying known internal values yields the expected product before conversion
- dividing known internal values yields the expected quotient before conversion
- converting zero yields `"0"` plus any requested extra zeroes
- converting a non-zero value yields digits without non-decimal characters
- requesting `n` extra zeroes appends exactly `n` trailing `0` characters

Traceability:
- `multiply`
- `divide`
- `convert_to_decimal`

## Requirements

### Functional Requirements

#### FR-1: Bounded narrow string length

The module shall provide a narrow string length operation that returns the number of characters before the first terminator, but no more than a supplied maximum bound.

Traceability:
- `local_strnlen`

#### FR-2: Full wide string length

The module shall provide a wide string length operation that returns the number of wide characters before the first terminator.

Traceability:
- `local_wcslen`

#### FR-3: Bounded wide string length

The module shall provide a wide string length operation that returns the number of wide characters before the first terminator, but no more than a supplied maximum bound.

Traceability:
- `local_wcsnlen`

#### FR-4: Fallback wide-character conversion

The module shall provide a fallback conversion from a single wide character to a multibyte byte sequence and report the produced byte count.

Traceability:
- `wctomb_fallback`

#### FR-5: Restartable local wide-character conversion

The module shall provide a local conversion from a single wide character to a multibyte byte sequence that accepts conversion state.

Traceability:
- `local_wcrtomb`

#### FR-6: Non-restartable local wide-character conversion

The module shall provide a local conversion from a single wide character to a multibyte byte sequence through a non-restartable wrapper.

Traceability:
- `local_wctomb`

#### FR-7: Decimal separator retrieval

The module shall provide a function that returns the decimal point character used by the module’s numeric formatting support.

Traceability:
- `decimal_point_char`

#### FR-8: Multi-precision multiplication

The module shall provide multiplication of two values in the module’s internal multi-precision representation, producing a destination value in the same representation.

Traceability:
- `multiply`
- anonymous struct at `gnu/vasnprintf.c:426-430`

#### FR-9: Multi-precision division

The module shall provide division of one value in the module’s internal multi-precision representation by another, producing a quotient value in the same representation.

Traceability:
- `divide`
- anonymous struct at `gnu/vasnprintf.c:426-430`

#### FR-10: Decimal conversion of internal numeric values

The module shall provide conversion of an internal multi-precision value into a decimal digit string.

Traceability:
- `convert_to_decimal`
- anonymous struct at `gnu/vasnprintf.c:426-430`

#### FR-11: Extra zero extension in decimal output

The decimal conversion operation shall append exactly the requested number of trailing zero characters to the generated decimal representation.

Traceability:
- `convert_to_decimal`

### Key Entities

#### Entity 1: Internal multi-precision number

An internal struct-defined numeric representation is used by the arithmetic and decimal conversion routines.

Relationships:
- serves as input to multiplication
- serves as input to division
- serves as input to decimal conversion
- receives arithmetic results as destination or quotient values

Traceability:
- anonymous struct at `gnu/vasnprintf.c:426-430`
- `multiply`
- `divide`
- `convert_to_decimal`

#### Entity 2: Narrow string input

A null-terminated byte string used for bounded length calculation.

Relationships:
- consumed by the bounded narrow string helper

Traceability:
- `local_strnlen`

#### Entity 3: Wide string input

A null-terminated wide character string used for full and bounded length calculation.

Relationships:
- consumed by the full wide string helper
- consumed by the bounded wide string helper

Traceability:
- `local_wcslen`
- `local_wcsnlen`

#### Entity 4: Wide character input

A single wide character value used for conversion into multibyte output bytes.

Relationships:
- converted by the fallback converter
- converted by the restartable wrapper
- converted by the non-restartable wrapper

Traceability:
- `wctomb_fallback`
- `local_wcrtomb`
- `local_wctomb`

#### Entity 5: Decimal digit string output

A decimal textual representation produced from an internal multi-precision numeric value, optionally with additional trailing zeroes.

Relationships:
- produced by decimal conversion
- depends on arithmetic correctness when used after multiplication or division

Traceability:
- `convert_to_decimal`

## Success Criteria

### SC-1: Correct bounded narrow length behavior

For representative test inputs, the Rust module returns:
- `0` for an empty narrow string
- the true string length when it is less than the maximum bound
- the maximum bound when no terminator occurs before that bound

Traceability:
- `local_strnlen`

### SC-2: Correct wide length behavior

For representative test inputs, the Rust module returns:
- `0` for an empty wide string
- the full count of wide characters before termination for unbounded measurement
- the lesser of actual length and supplied maximum for bounded measurement

Traceability:
- `local_wcslen`
- `local_wcsnlen`

### SC-3: Valid single-character multibyte conversion behavior

For representative valid wide characters under the supported runtime environment, the Rust module:
- emits a byte sequence for one wide character
- reports a byte count equal to the emitted sequence length
- produces consistent results between local wrapper forms when both apply

Traceability:
- `wctomb_fallback`
- `local_wcrtomb`
- `local_wctomb`

### SC-4: Decimal separator compatibility

When executed in a locale context with a known single-byte decimal separator, the Rust module returns the expected separator character.

Traceability:
- `decimal_point_char`

### SC-5: Arithmetic correctness for internal numeric values

For selected known values representable by the module’s internal numeric entity, the Rust module’s arithmetic operations produce results that match expected multiplication and quotient outcomes.

Traceability:
- `multiply`
- `divide`
- anonymous struct at `gnu/vasnprintf.c:426-430`

### SC-6: Correct decimal conversion output

For selected known internal numeric values, the Rust module’s decimal conversion:
- returns only decimal digit characters
- represents zero as `"0"` before extra zero extension
- appends exactly the requested number of trailing zeroes
- matches expected decimal text for tested non-zero values

Traceability:
- `convert_to_decimal`
- anonymous struct at `gnu/vasnprintf.c:426-430`