# spec.md

## Title

Functional Specification: `main_root_c-strcasecmp.c_15`

## Document Control

- **Project**: `pwd`
- **Module**: `main_root_c-strcasecmp.c_15`
- **Category**: `main_cluster`
- **Source file**: `c-strcasecmp.c`
- **Primary function**: `c_strcasecmp(const char *s1, const char *s2) -> int`
- **Rust branch target**: `015-main_root_c_strcasecmp.c_15-rust-port`
- **Generation date**: 2026-06-07

## 1. Feature Specification

### 1.1 Purpose

This module provides a single comparison operation for C strings: a case-insensitive string comparison routine named `c_strcasecmp`.

The Rust rewrite must preserve the observable behavior of this routine as a comparison function over two null-terminated strings, ignoring differences in letter case while determining lexical ordering.

### 1.2 Functional Scope

The Rust version must implement the behavior evidenced by `c_strcasecmp` in `c-strcasecmp.c`:

- Accept two input strings for comparison.
- Compare them without treating case differences in letters as significant.
- Return an integer result expressing ordering:
  - zero when the strings are equal under case-insensitive comparison,
  - a negative value when the first string sorts before the second,
  - a positive value when the first string sorts after the second.
- Continue comparison until a difference is found or both strings end.

### 1.3 Out of Scope

The Rust version must not introduce functionality not evidenced by this module, including:

- additional public comparison APIs,
- locale-sensitive comparison modes,
- Unicode-specific case folding guarantees,
- non-null-terminated string protocol changes,
- stateful configuration or option handling.

## 2. User Scenarios & Testing

### 2.1 Usage Scenarios

#### Scenario A: Equal strings differing only by case
A caller compares two strings whose letters differ only in uppercase/lowercase form.

- Example: `"abc"` and `"ABC"`
- Expected behavior: the function reports equality by returning `0`.

#### Scenario B: Different strings with first mismatch after case folding
A caller compares two strings that become unequal at some position even after ignoring case.

- Example: `"abc"` and `"abd"`
- Expected behavior: the function returns a negative value because the first string orders before the second.

#### Scenario C: Prefix relationship
A caller compares two strings where one is a case-insensitive prefix of the other.

- Example: `"Ab"` and `"aBc"`
- Expected behavior: the shorter string compares as less than the longer string.

#### Scenario D: Immediate mismatch
A caller compares two strings that differ at the first character after case-insensitive normalization.

- Example: `"z"` and `"A"`
- Expected behavior: the function returns a positive value because the first string orders after the second.

#### Scenario E: Empty string handling
A caller compares empty and non-empty strings.

- Example: `""` and `""` → equality
- Example: `""` and `"a"` → empty string compares as less

### 2.2 Testing Expectations

The Rust rewrite must be testable with cases covering:

- equality under differing ASCII letter case,
- inequality with negative result,
- inequality with positive result,
- one string ending before the other,
- both strings empty,
- comparisons involving non-letter bytes, which must remain comparison-significant.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Case-insensitive comparison
The module shall compare two input strings while ignoring case differences in alphabetic characters.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-2: Lexical ordering result
The module shall return an integer comparison result whose sign indicates ordering between the two inputs: negative, zero, or positive.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-3: Equality detection
The module shall return zero when the two inputs are equal under the module’s case-insensitive comparison behavior.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-4: End-of-string aware comparison
The module shall continue comparing characters in order until either a case-insensitive difference is found or a string terminator determines the ordering.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

#### FR-5: Non-letter preservation
The module shall preserve comparison significance for characters that are not altered by the module’s case-insensitive treatment.

**Traceability**: `c-strcasecmp.c`, `c_strcasecmp`

### 3.2 Key Entities

#### Entity: Input string
A null-terminated character sequence supplied as one of the two operands to the comparison function.

#### Entity: Comparison result
An integer value returned by the function to represent relative ordering between the two input strings.

#### Relationship
`c_strcasecmp` consumes two input strings and produces one comparison result based on case-insensitive lexical ordering.

## 4. Success Criteria

### 4.1 Behavioral Correctness

- The Rust implementation returns `0` for inputs that differ only by letter case.
- The Rust implementation returns a value less than `0` when the first input is ordered before the second under the module’s case-insensitive comparison.
- The Rust implementation returns a value greater than `0` when the first input is ordered after the second under the module’s case-insensitive comparison.
- The Rust implementation correctly handles comparisons where one input ends before the other.
- The Rust implementation correctly handles empty-string comparisons.

### 4.2 Traceable Test Coverage

Automated tests for the Rust rewrite shall demonstrate all of the following against the module behavior defined by `c_strcasecmp`:

- equal strings with different case,
- unequal strings with negative result,
- unequal strings with positive result,
- prefix/length-based ordering,
- empty vs empty,
- empty vs non-empty.

### 4.3 Scope Conformance

- The Rust rewrite exposes and implements only the behavior evidenced by this module’s comparison function.
- The rewrite does not require locale configuration or introduce alternate comparison semantics not present in the source module.