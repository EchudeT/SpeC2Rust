# spec.md

## Overview

This module provides a single comparison capability: case-insensitive comparison of two C strings.
It exists to determine lexical ordering while ignoring case differences between corresponding characters.

The Rust rewrite must preserve the observable behavior of the module’s exported function from `c-strcasecmp.c`:

- compare two NUL-terminated strings
- ignore case differences during comparison
- return an integer indicating less-than, equal-to, or greater-than ordering

This specification covers only behavior evidenced by the analyzed module.

## Feature Specification

### Feature: Case-insensitive C-string comparison

The module compares two input strings as C strings and evaluates them in lexical order without treating letter case as significant.

Behavior required from the Rust version:

- Accept two string inputs corresponding to the two C-string operands.
- Compare the strings from left to right.
- Treat uppercase and lowercase variants of the same character as equal for comparison purposes.
- Stop when a difference is found or when the string terminator is reached.
- Return:
  - zero when the strings are equal under case-insensitive comparison
  - a negative integer when the first string sorts before the second
  - a positive integer when the first string sorts after the second

### Functional boundary

The module’s responsibility is limited to string comparison behavior.
This specification does not require any capability beyond that single comparison function.

## User Scenarios & Testing

### Scenario 1: Equal strings with identical case

A caller compares two identical strings.

- Input example: `"cat"` and `"cat"`
- Expected result: returns `0`

### Scenario 2: Equal strings with differing case

A caller compares two strings that differ only by letter case.

- Input example: `"Cat"` and `"cAt"`
- Expected result: returns `0`

### Scenario 3: First differing character determines ordering

A caller compares two strings that differ at some position.

- Input example: `"abc"` and `"abd"`
- Expected result: negative result because the first string sorts before the second when compared case-insensitively

### Scenario 4: Prefix ordering

A caller compares a string with another string that has the first as a prefix.

- Input example: `"ab"` and `"abc"`
- Expected result: negative result because the shorter string ends first and therefore sorts before the longer one

### Scenario 5: Reverse ordering

A caller compares strings where the first sorts after the second.

- Input example: `"b"` and `"A"`
- Expected result: positive result because case-insensitive comparison places `"b"` after `"a"`

### Scenario 6: Empty string handling

A caller compares empty and non-empty strings.

- Input example: `""` and `""`
- Expected result: returns `0`

- Input example: `""` and `"x"`
- Expected result: negative result

### Testing expectations

The Rust version must be tested for:

- equality with same case
- equality with mixed case
- lexical ordering before and after
- prefix relationships
- empty-string comparisons
- stability of sign semantics:
  - negative for less-than
  - zero for equal
  - positive for greater-than

## Requirements

### Functional Requirements

#### FR-1: Provide case-insensitive comparison
The module shall provide a function equivalent in behavior to `c_strcasecmp(const char *s1, const char *s2)` from `c-strcasecmp.c`.

Traceability: `c_strcasecmp` in `c-strcasecmp.c`

#### FR-2: Compare two strings lexically from start to end
The function shall compare the two input strings in sequence from their first character onward until either a case-insensitive difference is found or both strings terminate.

Traceability: `c_strcasecmp` in `c-strcasecmp.c`

#### FR-3: Ignore case differences during comparison
The function shall treat character case differences as non-distinguishing for comparison purposes.

Traceability: `c_strcasecmp` in `c-strcasecmp.c`

#### FR-4: Return ordering by integer sign
The function shall return:
- `0` when the inputs are equal under case-insensitive comparison,
- a value less than `0` when the first input is ordered before the second,
- a value greater than `0` when the first input is ordered after the second.

Traceability: `c_strcasecmp` in `c-strcasecmp.c`

#### FR-5: Respect C-string termination
The function shall determine string length and comparison stopping points using C-string termination semantics.

Traceability: `c_strcasecmp` in `c-strcasecmp.c`

### Key Entities

#### Entity: Input string 1
The first operand is a C string supplied to the comparison function.

Relationship:
- compared against Input string 2 by the module’s sole function

Traceability: parameter `s1` of `c_strcasecmp`

#### Entity: Input string 2
The second operand is a C string supplied to the comparison function.

Relationship:
- compared against Input string 1 by the module’s sole function

Traceability: parameter `s2` of `c_strcasecmp`

#### Entity: Comparison result
The output is an integer representing the ordering relationship between the two inputs after case-insensitive comparison.

Relationship:
- produced from comparing Input string 1 and Input string 2

Traceability: return value of `c_strcasecmp`

## Success Criteria

### SC-1: Equal-under-case comparisons return zero
For test cases where the two inputs differ only by letter case, the Rust version returns `0`.

Traceability: FR-1, FR-3, FR-4

### SC-2: Ordered comparisons return correct sign
For representative test cases where the inputs differ in case-insensitive lexical order, the Rust version returns a negative value for less-than and a positive value for greater-than.

Traceability: FR-2, FR-3, FR-4

### SC-3: Prefix behavior matches C-string lexical comparison
For test cases where one input is a prefix of the other, the Rust version returns the correct sign based on which string terminates first.

Traceability: FR-2, FR-4, FR-5

### SC-4: Empty-string cases behave correctly
For test cases involving empty strings, including empty-vs-empty and empty-vs-non-empty, the Rust version returns results consistent with case-insensitive lexical comparison.

Traceability: FR-4, FR-5

### SC-5: Scope remains limited to comparison behavior
The Rust rewrite exposes and implements only the behavior evidenced for this module: case-insensitive comparison of two strings with integer ordering result semantics.

Traceability: `c_strcasecmp` in `c-strcasecmp.c`