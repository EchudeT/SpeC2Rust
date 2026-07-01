# Functional Specification: `module_gnu_vasnprintf.c_52`

- **Project**: `cflow-new`
- **Module**: `module_gnu_vasnprintf.c_52`
- **Category**: `module_cluster`
- **Source**: `gnu/vasnprintf.c`
- **Rust branch target**: `058-module_gnu_vasnprintf.c_52-rust-port`
- **Generation date**: `2026-06-17`

## 1. Overview

This module provides internal support functionality for formatted output generation in `vasnprintf`-style processing where the surrounding formatter needs:

- bounded length handling for narrow and wide strings,
- conversion of wide characters to multibyte output through locale-sensitive or fallback behavior,
- retrieval of the locale decimal separator character,
- arbitrary-precision style integer operations used to construct decimal representations,
- conversion of internal multi-precision numeric values into decimal digit strings.

The Rust rewrite must preserve the observable behavior of these support operations as used by the enclosing formatted-printing logic. The scope is limited to the functionality evidenced by the analyzed source functions and local data structure.

## 2. Feature Specification

### 2.1 String length support

The module supports length determination for C-style strings and wide strings:

- determine the length of a narrow string up to a maximum bound,
- determine the full length of a wide-character string terminated by a zero wide character,
- determine the length of a wide-character string up to a maximum bound.

The Rust version must implement equivalent bounded and terminator-based length behavior for the internal string representations used by the port.

### 2.2 Wide-character to multibyte conversion support

The module supports conversion of a single wide character into a multibyte character sequence for output preparation.

This support includes:

- a fallback conversion path for environments or cases where direct conversion support is unavailable or unsuitable,
- a wrapper that performs state-aware conversion using a multibyte conversion state object,
- a wrapper that performs state-independent conversion in the simpler `wctomb` style.

The Rust version must preserve the conversion outcomes relevant to formatting behavior:

- valid wide characters produce their corresponding output byte sequence,
- invalid or unconvertible values are reported as conversion failure,
- output byte count is returned when conversion succeeds.

### 2.3 Locale decimal separator support

The module provides access to the active locale’s decimal point character for use in numeric formatting.

The Rust version must return the same decimal separator character that the C module would use for formatting decisions within the same locale context, limited to the single-character behavior evidenced by the source.

### 2.4 Multi-precision arithmetic support for decimal conversion

The module supports internal arithmetic on a multi-precision numeric representation:

- multiplication of two internal numeric values,
- division of one internal numeric value by another, producing a quotient representation.

These operations exist to support decimal formatting logic rather than to provide a general-purpose arithmetic API. The Rust version must preserve arithmetic correctness for the values and relationships required by the decimal conversion flow.

### 2.5 Conversion of internal numeric representation to decimal text

The module converts an internal multi-precision numeric value into a decimal digit string, with support for appending a specified number of extra trailing zero digits.

The Rust version must produce decimal strings that:

- represent the numeric value accurately in base 10,
- contain only decimal digits plus the requested appended zeroes,
- are suitable for use by the enclosing formatter.

## 3. User Scenarios & Testing

### Scenario 1: Bounded narrow-string scanning during formatting

A formatting operation needs to inspect a narrow string argument but must not read beyond a supplied maximum length.

**Expected support**:
- the module returns the index of the first terminator if present before the bound,
- otherwise it returns the bound.

**Test coverage**:
- empty string with nonzero bound,
- string shorter than bound,
- string exactly at bound with no earlier terminator,
- large bound with early terminator.

### Scenario 2: Wide-string width or precision handling

A formatting operation receives a wide-character string and must determine its length, either fully or up to a precision limit.

**Expected support**:
- full wide-string length stops at the terminating zero wide character,
- bounded wide-string length stops at the earlier of terminator or maximum length.

**Test coverage**:
- empty wide string,
- bounded and unbounded length on the same input,
- no terminator before bound,
- terminator exactly at bound edge.

### Scenario 3: Emitting a wide character into formatted output

A formatted output path needs to encode one wide character into bytes.

**Expected support**:
- successful conversion returns the number of output bytes,
- output bytes match the active conversion behavior,
- invalid or unrepresentable wide characters are reported as failure.

**Test coverage**:
- ASCII-range wide character,
- multibyte character in a locale/encoding that supports it,
- invalid code value or unconvertible input,
- wrapper behavior consistency between state-aware and simple conversion paths where both are applicable.

### Scenario 4: Formatting a floating or decimal-like value using locale decimal point

The formatter needs the locale-specific decimal separator to build final output.

**Expected support**:
- the module returns the current locale decimal point as a single character used by formatting logic.

**Test coverage**:
- locale using `'.'`,
- locale using a different single-byte decimal separator if available in the test environment,
- repeated calls return consistent results under unchanged locale.

### Scenario 5: Internal decimal construction from large numeric parts

The formatter constructs a decimal representation from an internal large integer value and requires arithmetic support.

**Expected support**:
- multiplication produces the correct product value,
- division produces the correct quotient for the values used by the conversion algorithm.

**Test coverage**:
- multiplication by zero,
- multiplication of small values,
- multiplication carrying across internal limbs,
- division where dividend is smaller than divisor,
- division with exact quotient,
- division with nontrivial quotient across multiple limbs.

### Scenario 6: Rendering internal multi-precision value as decimal digits

The formatter needs a decimal string from an internal numeric value, optionally with extra trailing zeroes.

**Expected support**:
- zero converts to `"0"` plus any requested extra zeroes as dictated by the source behavior,
- positive values convert to exact base-10 digit strings,
- requested extra zeroes are appended to the decimal digits.

**Test coverage**:
- zero value,
- single-digit value,
- multi-digit value,
- very large value requiring repeated division or equivalent conversion steps,
- nonzero `extra_zeroes`,
- correctness against known decimal outputs.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Bounded narrow-string length
The module shall provide a function equivalent to `local_strnlen` that returns the number of bytes before the first `'\0'` in a narrow string, limited to a caller-supplied maximum bound.
**Traceability**: `gnu/vasnprintf.c:242-247`, `local_strnlen`

#### FR-2: Wide-string full length
The module shall provide a function equivalent to `local_wcslen` that returns the number of wide characters before the terminating zero wide character.
**Traceability**: `gnu/vasnprintf.c:262-270`, `local_wcslen`

#### FR-3: Wide-string bounded length
The module shall provide a function equivalent to `local_wcsnlen` that returns the number of wide characters before the first terminating zero wide character, limited to a caller-supplied maximum bound.
**Traceability**: `gnu/vasnprintf.c:281-289`, `local_wcsnlen`

#### FR-4: Fallback wide-character conversion
The module shall provide fallback conversion behavior equivalent to `wctomb_fallback` for converting one wide character into an output byte sequence when used by the module’s conversion path. Successful conversion shall report the output byte count; failure shall be distinguishable from success.
**Traceability**: `gnu/vasnprintf.c:296-332`, `wctomb_fallback`

#### FR-5: Stateful wide-character conversion wrapper
The module shall provide behavior equivalent to `local_wcrtomb` that converts one wide character into a multibyte sequence using a conversion state parameter.
**Traceability**: `gnu/vasnprintf.c:334-341`, `local_wcrtomb`

#### FR-6: Simple wide-character conversion wrapper
The module shall provide behavior equivalent to `local_wctomb` that converts one wide character into a multibyte sequence through the simpler wrapper form.
**Traceability**: `gnu/vasnprintf.c:343-350`, `local_wctomb`

#### FR-7: Locale decimal point retrieval
The module shall provide behavior equivalent to `decimal_point_char` that returns the decimal separator character used by the current locale-sensitive formatting logic.
**Traceability**: `gnu/vasnprintf.c:366-385`, `decimal_point_char`

#### FR-8: Multi-precision multiplication
The module shall provide multiplication of two internal multi-precision numeric values equivalent in result to `multiply`.
**Traceability**: `gnu/vasnprintf.c:435-498`, `multiply`

#### FR-9: Multi-precision division
The module shall provide division of one internal multi-precision numeric value by another, producing a quotient value equivalent in result to `divide` for the inputs used by this module.
**Traceability**: `gnu/vasnprintf.c:507-930`, `divide`

#### FR-10: Decimal string conversion
The module shall provide conversion of an internal multi-precision numeric value into a decimal digit string, with appended extra zero digits as requested, equivalent in result to `convert_to_decimal`.
**Traceability**: `gnu/vasnprintf.c:946-994`, `convert_to_decimal`

### 4.2 Key Entities

#### KE-1: Internal multi-precision number representation
The module uses an internal numeric representation identified in the analyzed source through `mpn_t`-based operations and an associated local struct in the arithmetic section. This representation models nonnegative integer values used for decimal conversion support.
**Traceability**: arithmetic and conversion functions `multiply`, `divide`, `convert_to_decimal`; local struct at `gnu/vasnprintf.c:426-430`

#### KE-2: Quotient/result relationship in arithmetic operations
Arithmetic operations consume one or two internal multi-precision values and produce a new internal result value representing either:
- the product of two operands, or
- the quotient of division.

These result values feed subsequent decimal conversion logic.
**Traceability**: `multiply`, `divide`, `convert_to_decimal`

#### KE-3: Character conversion inputs and outputs
Wide-character conversion functions consume a single wide character and optionally a conversion state, and produce either:
- a multibyte output sequence and its byte length, or
- a conversion failure result.

This output is used by enclosing formatted output logic.
**Traceability**: `wctomb_fallback`, `local_wcrtomb`, `local_wctomb`

## 5. Success Criteria

### 5.1 Behavioral correctness

1. For all test inputs used to validate bounded narrow-string scanning, the Rust module returns the same length result as the C module’s `local_strnlen`.
   **Traceability**: `local_strnlen`

2. For all test inputs used to validate wide-string length handling, the Rust module returns the same full and bounded lengths as the C module’s `local_wcslen` and `local_wcsnlen`.
   **Traceability**: `local_wcslen`, `local_wcsnlen`

3. For valid and invalid wide-character conversion test cases, the Rust module matches the C module’s success/failure behavior and byte-count results for `wctomb_fallback`, `local_wcrtomb`, and `local_wctomb` under the same runtime conditions.
   **Traceability**: `wctomb_fallback`, `local_wcrtomb`, `local_wctomb`

4. For locale-sensitive decimal separator tests, the Rust module returns the same decimal point character as the C module’s `decimal_point_char` under the same locale setting.
   **Traceability**: `decimal_point_char`

5. For validated arithmetic test cases, the Rust module’s multiplication and division results are numerically identical to those produced by the C module’s `multiply` and `divide`.
   **Traceability**: `multiply`, `divide`

6. For validated conversion test cases, the Rust module returns the same decimal digit strings as the C module’s `convert_to_decimal`, including handling of requested extra zeroes.
   **Traceability**: `convert_to_decimal`

### 5.2 Integration suitability

7. The Rust implementation supports all six usage scenarios in this specification without requiring functionality beyond the boundaries evidenced in `gnu/vasnprintf.c`.
   **Traceability**: all listed functions

8. The Rust implementation exposes or preserves sufficient internal behavior for the enclosing `vasnprintf` formatting logic to perform string measurement, character conversion, locale decimal separator lookup, internal arithmetic, and decimal rendering with no observable regression in covered scenarios.
   **Traceability**: all listed functions and local arithmetic struct