# spec.md

## Title

Functional Specification for `main_root_c-strcasecmp.c_15`

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_c-strcasecmp.c_15`
- **Category**: `main_cluster`
- **Source File**: `c-strcasecmp.c`
- **Primary Function**: `c_strcasecmp(const char *s1, const char *s2) -> int`
- **Rust Target Branch**: `015-main_root_c_strcasecmp.c_15-rust-port`
- **Generation Date**: `2026-06-07`

## Overview

This module provides a single comparison operation for two C strings. Its functional role is to compare two null-terminated byte strings without distinguishing between uppercase and lowercase letter forms, and to return an integer indicating lexical ordering.

The Rust rewrite must preserve the observable behavior of this comparison operation as a module-level utility used by the surrounding program logic.

## Feature Specification

### Summary

The module performs case-insensitive comparison of two input C strings and reports whether the first string is ordered before, equal to, or after the second string.

### Functional Behavior

The Rust version must implement behavior equivalent to the source module:

- Accept two string inputs corresponding to C null-terminated strings.
- Compare the strings in lexical order.
- Treat letter case as insignificant during comparison.
- Continue comparison until:
  - a difference is found after case normalization, or
  - both strings reach termination.
- Return an integer result whose sign communicates ordering:
  - negative if the first string compares less than the second,
  - zero if they compare equal ignoring case,
  - positive if the first string compares greater than the second.

### Scope Boundary

This module is limited to string comparison behavior. The specification does not require any additional text processing, locale management, alternative encodings, or new public APIs beyond the behavior evidenced by `c_strcasecmp`.

## User Scenarios & Testing

### Scenario 1: Equal strings with identical case

A caller compares two identical null-terminated strings.

- Input example: `"root"` vs `"root"`
- Expected result: `0`

### Scenario 2: Equal strings with different case

A caller compares two strings that differ only in letter case.

- Input example: `"Root"` vs `"rOoT"`
- Expected result: `0`

### Scenario 3: Different strings with early mismatch

A caller compares two strings whose first differing character determines ordering.

- Input example: `"abc"` vs `"abd"`
- Expected result: negative value

### Scenario 4: Prefix relationship

A caller compares strings where one is a prefix of the other.

- Input example: `"ab"` vs `"abc"`
- Expected result: negative value
- Input example: `"abc"` vs `"ab"`
- Expected result: positive value

### Scenario 5: Mismatch after case-insensitive matching prefix

A caller compares strings that match for several characters ignoring case and then differ.

- Input example: `"AbCdX"` vs `"aBcDy"`
- Expected result: negative value

### Scenario 6: Empty string handling

A caller compares empty and non-empty null-terminated strings.

- Input example: `""` vs `""`
- Expected result: `0`
- Input example: `""` vs `"a"`
- Expected result: negative value
- Input example: `"a"` vs `""`
- Expected result: positive value

### Testing Expectations

The Rust rewrite must be tested against the above scenarios and must verify:

- equality when strings differ only by case,
- sign-only ordering correctness for non-equal comparisons,
- correct handling of null terminator boundaries,
- correct ordering when one string ends before the other.

## Requirements

### Functional Requirements

#### FR-1: Case-insensitive lexical comparison

The module shall compare two input C strings lexically while ignoring case distinctions between alphabetic characters.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-2: Null-terminated string traversal

The module shall evaluate the inputs as null-terminated strings and compare characters in sequence until a decisive difference is found or both strings terminate.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-3: Ordering result by integer sign

The module shall return an integer whose sign indicates the ordering outcome of the comparison: negative, zero, or positive.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-4: Equality under case folding for matched content

The module shall return zero when the two input strings contain the same character sequence modulo case differences and terminate at the same position.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-5: Prefix-sensitive ordering

The module shall treat a shorter string as less than a longer string when all compared characters match case-insensitively up to the end of the shorter string.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

### Key Entities

#### Entity: Input string 1

The first comparison operand, represented in the source module as a pointer to a null-terminated C string.

**Relationship**: Compared sequentially against Input string 2.

**Traceability**: `c_strcasecmp(const char *s1, const char *s2)`

#### Entity: Input string 2

The second comparison operand, represented in the source module as a pointer to a null-terminated C string.

**Relationship**: Compared sequentially against Input string 1.

**Traceability**: `c_strcasecmp(const char *s1, const char *s2)`

#### Entity: Comparison result

The integer return value expressing lexical ordering after case-insensitive comparison.

**Relationship**: Derived from comparing Input string 1 to Input string 2.

**Traceability**: `c_strcasecmp(const char *s1, const char *s2) -> int`

## Success Criteria

### SC-1: Equal-case and mixed-case equality

For test inputs that differ only in letter case, the Rust implementation returns `0`.

**Traceability**: `c_strcasecmp`

### SC-2: Correct sign for lexical mismatch

For test inputs with a case-insensitive mismatch, the Rust implementation returns a negative value when the first input is lexically less and a positive value when the first input is lexically greater.

**Traceability**: `c_strcasecmp`

### SC-3: Correct prefix ordering

For test inputs where one string is a case-insensitive prefix of the other, the Rust implementation returns the correct sign based on which string terminates first.

**Traceability**: `c_strcasecmp`

### SC-4: Correct empty-string handling

For empty-string test cases, the Rust implementation returns:
- `0` for two empty strings,
- negative when only the first string is empty,
- positive when only the second string is empty.

**Traceability**: `c_strcasecmp`

### SC-5: Behavioral parity at module scope

The Rust rewrite exposes and implements the module’s evidenced behavior without adding unrelated functionality beyond case-insensitive C-string comparison.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`