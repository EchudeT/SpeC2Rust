# spec.md

## Overview

- **Project**: `pwd`
- **Module**: `main_root_c-strcasecmp.c_15`
- **Category**: `main_cluster`
- **Source file**: `c-strcasecmp.c`
- **Primary function**: `c_strcasecmp(const char *s1, const char *s2) -> int`
- **Rust target branch**: `015-main_root_c_strcasecmp.c_15-rust-port`
- **Generation date**: `2026-06-09`

## Feature Specification

This module provides a single comparison operation for two C strings, performing a case-insensitive lexical comparison.

The Rust rewrite must implement the same functional behavior as the C module:

- Accept two input strings corresponding to null-terminated C strings.
- Compare them character by character without treating differences in letter case as significant.
- Return an integer indicating relative ordering:
  - negative if the first string compares less than the second,
  - zero if they compare equal ignoring case,
  - positive if the first string compares greater than the second.
- Stop comparison at the first decisive difference or when both strings terminate.
- Treat equal strings with different ASCII letter case as equal.

This module’s scope is limited to comparison behavior. No additional public capabilities are evidenced by the source input and must not be added.

## User Scenarios & Testing

### Scenario 1: Equal strings with identical case
A caller compares two identical C strings.

- Input: `"abc"` and `"abc"`
- Expected result: `0`

### Scenario 2: Equal strings with different letter case
A caller compares two strings that differ only by case.

- Input: `"Alpha"` and `"aLPHA"`
- Expected result: `0`

### Scenario 3: First differing character determines ordering
A caller compares two strings where the first non-matching character determines lexical order after case folding.

- Input: `"abc"` and `"abd"`
- Expected result: negative
- Input: `"abe"` and `"abd"`
- Expected result: positive

### Scenario 4: Prefix versus longer string
A caller compares strings where one is a case-insensitive prefix of the other.

- Input: `"ab"` and `"ABC"`
- Expected result: negative
- Input: `"ABCD"` and `"ab"`
- Expected result: positive

### Scenario 5: Empty string handling
A caller compares empty and non-empty strings.

- Input: `""` and `""`
- Expected result: `0`
- Input: `""` and `"a"`
- Expected result: negative
- Input: `"A"` and `""`
- Expected result: positive

### Scenario 6: Non-letter bytes are compared as-is except for case-insensitive letter handling
A caller compares strings containing digits or punctuation.

- Input: `"a-1"` and `"A-1"`
- Expected result: `0`
- Input: `"a-1"` and `"a-2"`
- Expected result: negative

### Testing expectations
The Rust version must include tests covering:

- equality,
- equality under case differences,
- lexical ordering,
- prefix-length differences,
- empty-string comparisons,
- mixed content with letters and non-letters.

## Requirements

### Functional Requirements

#### FR-1: Case-insensitive string comparison
The module shall compare two input strings using case-insensitive character comparison behavior traceable to `c_strcasecmp` in `c-strcasecmp.c`.

#### FR-2: Lexical ordering result
The module shall return a signed integer result whose sign indicates whether the first input sorts before, equal to, or after the second input, traceable to `c_strcasecmp`.

#### FR-3: First-difference semantics
The module shall determine ordering based on the first position at which the compared strings differ after case-insensitive normalization, traceable to `c_strcasecmp`.

#### FR-4: Null-terminated string end handling
The module shall treat the string terminator as the end of comparison and use termination to distinguish equal strings from prefix relationships, traceable to `c_strcasecmp`.

#### FR-5: Case-only equality
The module shall report equality for strings whose only differences are letter case, traceable to `c_strcasecmp`.

### Key Entities

#### Entity: Input C strings
- Two read-only input strings are provided to the comparison function.
- Each string is interpreted as a null-terminated sequence of characters.
- The comparison operation relates these two inputs and produces one integer ordering result.

#### Entity: Comparison result
- A signed integer value represents the ordering outcome.
- Its meaning is based on sign only:
  - less than zero,
  - equal to zero,
  - greater than zero.

## Success Criteria

### Behavioral correctness
- For all tested pairs of inputs that differ only in letter case, the Rust version returns `0`.
- For all tested pairs with a decisive first differing character, the Rust version returns a result with the same sign as the C module.
- For tested prefix pairs, the Rust version distinguishes shorter-prefix and longer-string cases with the same sign behavior as the C module.
- For tested empty-string cases, the Rust version returns the same sign behavior as the C module.

### Interface preservation
- The Rust rewrite exposes functionality corresponding to the module’s single evidenced behavior: comparison of two C-string inputs producing an integer ordering result.

### Scope adherence
- The Rust rewrite does not add unrelated comparison modes or extra module responsibilities beyond the behavior evidenced by `c_strcasecmp` in `c-strcasecmp.c`.