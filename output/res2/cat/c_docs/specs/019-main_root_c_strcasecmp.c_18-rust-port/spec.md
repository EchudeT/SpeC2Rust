# spec.md

## Title
Rust Port Functional Specification: `main_root_c-strcasecmp.c_18`

## Metadata
- Project: `cat`
- Module: `main_root_c-strcasecmp.c_18`
- Category: `main_cluster`
- Source file: `c-strcasecmp.c`
- Primary function: `c_strcasecmp(const char *s1, const char *s2) -> int`
- Rust branch: `019-main_root_c_strcasecmp.c_18-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides a single string comparison capability: comparing two C strings without regard to letter case and returning an integer that indicates their relative ordering.

The Rust rewrite must preserve the functional behavior of this comparison routine as a module-level facility used by other parts of the project that need case-insensitive comparison of NUL-terminated strings.

## Feature Specification

### Purpose
Provide a case-insensitive comparison between two input strings represented as C-style strings.

### Functional Behavior
The Rust version must implement behavior equivalent to `c_strcasecmp`:

- Accept two input strings for comparison.
- Compare them character by character in a case-insensitive manner.
- Treat strings that differ only by letter case as equal.
- Determine ordering when the strings differ after case normalization.
- Correctly handle strings of different lengths by considering string termination as part of ordering.
- Return an integer result with standard comparison semantics:
  - zero when the inputs are equal ignoring case,
  - a negative value when the first input sorts before the second,
  - a positive value when the first input sorts after the second.

### Scope Boundary
This module’s scope is limited to case-insensitive comparison of two strings. The Rust port must not introduce additional public behaviors beyond this comparison functionality.

## User Scenarios & Testing

### Scenario 1: Equal strings with identical case
A caller compares two identical strings.

- Input: `"cat"` and `"cat"`
- Expected result: `0`

### Scenario 2: Equal strings with different letter case
A caller compares strings that differ only by upper/lower case.

- Input: `"Cat"` and `"cAt"`
- Expected result: `0`

### Scenario 3: First string sorts before second
A caller compares two strings that differ at a position where, after case normalization, the first string is lexically smaller.

- Input: `"abc"` and `"abd"`
- Expected result: negative integer

### Scenario 4: First string sorts after second
A caller compares two strings that differ at a position where, after case normalization, the first string is lexically greater.

- Input: `"abe"` and `"abd"`
- Expected result: positive integer

### Scenario 5: Prefix relationship
A caller compares strings where one is a full prefix of the other.

- Input: `"ab"` and `"abc"`
- Expected result: negative integer
- Input: `"abc"` and `"ab"`
- Expected result: positive integer

### Scenario 6: Difference only after several equal characters
A caller compares strings that share a common beginning and differ later.

- Input: `"AlphaX"` and `"alphay"`
- Expected result: negative integer

### Testing Expectations
The Rust version must be testable with cases covering:

- equality under exact match,
- equality under case-only differences,
- ordering before and after,
- shorter-versus-longer string ordering,
- empty string behavior when compared with empty and non-empty strings.

## Requirements

### Functional Requirements

#### FR-1: Case-insensitive string comparison
The module shall compare two input C strings without regard to alphabetic case.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-2: Equality result
The module shall return `0` when both input strings are equal under case-insensitive comparison.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-3: Ordering result
The module shall return a value less than `0` when the first input is ordered before the second under case-insensitive comparison, and greater than `0` when ordered after.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-4: Sequential comparison behavior
The module shall determine the result from the first position at which the case-normalized characters differ, or from string termination if no earlier difference exists.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-5: NUL-terminated string handling
The module shall operate on NUL-terminated string inputs and treat the terminating NUL as the end of comparison.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

### Key Entities

#### Entity: Input string
A C-style input string supplied to the comparison routine.

Properties:
- represented as a pointer to a NUL-terminated character sequence,
- consumed read-only by the module,
- compared against another input string.

#### Entity: Comparison result
The integer return value produced by the comparison routine.

Properties:
- expresses equality or relative ordering,
- derived from case-insensitive comparison of the two input strings.

#### Relationship
Two input strings are provided to the module, and the module produces one comparison result describing their case-insensitive ordering relationship.

## Success Criteria

### Behavioral Correctness
- The Rust port returns `0` for all tested input pairs that differ only by alphabetic case.
- The Rust port returns a negative value for tested cases where the first input is lexically smaller under case-insensitive comparison.
- The Rust port returns a positive value for tested cases where the first input is lexically greater under case-insensitive comparison.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

### String Termination Handling
- The Rust port correctly distinguishes equal strings from prefix-related strings by honoring NUL termination in test cases such as empty/non-empty and prefix/non-prefix pairs.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`

### Scope Conformance
- The Rust module exposes and implements only the evidenced comparison behavior of this source module, without adding unrelated functional responsibilities.
- Traceability: `c-strcasecmp.c`, `c_strcasecmp`