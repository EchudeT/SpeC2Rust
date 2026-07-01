# spec.md

## Title
Functional Specification: `main_root_c-strcasecmp.c_18`

## Metadata
- Project: `cat`
- Module: `main_root_c-strcasecmp.c_18`
- Category: `main_cluster`
- Source file: `c-strcasecmp.c`
- Primary function: `c_strcasecmp(const char *s1, const char *s2) -> int`
- Rust branch: `019-main_root_c_strcasecmp.c_18-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides a case-insensitive string comparison function for C-style null-terminated strings. Its role is to compare two input strings lexicographically while ignoring differences in letter case, and to return an integer indicating their relative ordering or equality.

The Rust rewrite must preserve this observable behavior for the same class of inputs represented by the original C interface: two valid null-terminated byte strings.

## Feature Specification

### Summary
The module implements one functional feature: case-insensitive comparison of two strings.

### Behavior
Given two input strings:

- The module compares them sequentially from the first character onward.
- Character case differences are ignored during comparison.
- If corresponding characters are equal after case normalization, comparison continues.
- If a differing pair of characters is found after case normalization, the function returns an integer indicating whether the first string sorts before or after the second.
- If both strings reach the terminating null byte without any difference after case normalization, the function returns zero.

### Rust Port Scope
The Rust version must implement equivalent comparison behavior for C-style string inputs used by the surrounding ported code. The externally visible semantics must match the C module:

- equality when strings differ only by case,
- ordering when they differ by character value after case normalization,
- correct handling when one string is a prefix of the other.

No additional public functionality is required by this module specification.

## User Scenarios & Testing

### Scenario 1: Compare identical strings
A caller compares two strings with the same contents and same case.

Expected result:
- The comparison returns `0`.

Example test cases:
- `"cat"` vs `"cat"` => `0`
- `""` vs `""` => `0`

### Scenario 2: Compare strings differing only by case
A caller compares two strings whose letters differ only in uppercase/lowercase representation.

Expected result:
- The comparison returns `0`.

Example test cases:
- `"Cat"` vs `"cat"` => `0`
- `"ABC"` vs `"abc"` => `0`

### Scenario 3: Determine lexicographic ordering ignoring case
A caller compares two different strings and needs their order without regard to case.

Expected result:
- Result is negative when the first string is lexicographically smaller after case normalization.
- Result is positive when the first string is lexicographically greater after case normalization.

Example test cases:
- `"abc"` vs `"abd"` => negative
- `"abd"` vs `"abc"` => positive

### Scenario 4: Compare strings of different lengths with shared prefix
A caller compares two strings where one is a prefix of the other, ignoring case.

Expected result:
- The shorter string compares as less than the longer string when all earlier characters are equal after case normalization.

Example test cases:
- `"ab"` vs `"abc"` => negative
- `"ABC"` vs `"ab"` => positive

### Scenario 5: Compare strings containing non-letter bytes
A caller compares strings that include bytes not affected by case normalization.

Expected result:
- Non-letter bytes participate in ordering according to the same comparison process, with only case-insensitive treatment applied where relevant.

Example test cases:
- `"a-1"` vs `"A-1"` => `0`
- `"a_1"` vs `"a-1"` => non-zero ordering consistent with byte comparison after case normalization of letters

## Requirements

### Functional Requirements

#### FR-1: Case-insensitive equality
The module shall report two input strings as equal when they differ only by letter case.

Traceability:
- Source: `c-strcasecmp.c`
- Function: `c_strcasecmp`

#### FR-2: Lexicographic ordering
The module shall return an integer result indicating lexicographic ordering between two input strings after applying case-insensitive comparison.

Traceability:
- Source: `c-strcasecmp.c`
- Function: `c_strcasecmp`

#### FR-3: Sequential comparison until first effective difference or end
The module shall compare input strings from left to right and determine the result based on the first position at which the strings differ after case normalization, or return equality if no such position exists before both strings terminate.

Traceability:
- Source: `c-strcasecmp.c`
- Function: `c_strcasecmp`

#### FR-4: Prefix handling
The module shall treat a string that matches the beginning of another string, ignoring case, as less than the longer string if the shorter string terminates first.

Traceability:
- Source: `c-strcasecmp.c`
- Function: `c_strcasecmp`

#### FR-5: Null-terminated string semantics
The module shall operate on C-style strings terminated by a null byte and stop comparison at string termination.

Traceability:
- Source: `c-strcasecmp.c`
- Function: `c_strcasecmp`

### Key Entities

#### Entity: Input string `s1`
- Type role: first input C-style string
- Relationship: compared against `s2` by `c_strcasecmp`

#### Entity: Input string `s2`
- Type role: second input C-style string
- Relationship: compared against `s1` by `c_strcasecmp`

#### Entity: Comparison result
- Type role: integer ordering result
- Relationship: produced by comparing `s1` and `s2`; indicates equality, less-than, or greater-than

## Success Criteria

### Behavioral Correctness
- For pairs of valid null-terminated input strings that are identical except for letter case, the Rust implementation returns `0`.
- For valid null-terminated input strings that differ at some position after case normalization, the Rust implementation returns a negative or positive integer matching the ordering direction of the original C module.
- For valid null-terminated input strings where one is a prefix of the other ignoring case, the Rust implementation returns a non-zero value with the shorter string ordered before the longer one.
- For valid empty-string inputs, the Rust implementation matches the original module behavior.

### Compatibility
- The Rust rewrite exposes module behavior equivalent to `c_strcasecmp` for all supported inputs used by the ported project.
- The Rust rewrite does not require additional caller-visible features beyond the case-insensitive string comparison behavior defined in this specification.

### Testability
The Rust version is considered complete for this module when automated tests cover at least:
- equal strings,
- case-only differences,
- less-than and greater-than outcomes,
- prefix relationships,
- empty strings,
- strings containing non-letter bytes.