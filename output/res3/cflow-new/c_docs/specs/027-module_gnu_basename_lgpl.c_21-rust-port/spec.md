# Functional Specification: `module_gnu_basename-lgpl.c_21`

**Project:** `cflow-new`
**Module category:** `module_cluster`
**Source module:** `gnu/basename-lgpl.c`
**Rust target branch:** `027-module_gnu_basename_lgpl.c_21-rust-port`
**Generation date:** 2026-06-17

## 1. Overview

This module provides pathname basename support for string-based path processing.

Its functionality is limited to:
- locating the last pathname component within an input path string
- determining the length of the basename portion for a path string, including handling of trailing directory separators according to the source module behavior

The Rust rewrite must preserve the observable behavior of these pathname parsing operations for byte/string path inputs representing C-style paths.

## 2. Feature Specification

### 2.1 Basename Component Discovery

The module must support identifying the final pathname component in a path string.

Behavior covered by the source module:
- skip over directory separators when needed to locate the start of the final component
- return the position corresponding to the last component of the path
- support paths with no separator characters
- support paths with one or more trailing separators
- support paths composed entirely of separators

The Rust version must implement equivalent behavior for determining where the basename component begins.

### 2.2 Basename Length Determination

The module must support computing the length of the basename portion of a path string.

Behavior covered by the source module:
- compute the length of the final pathname component
- ignore trailing directory separators when determining that length, except where source behavior yields a separator-only basename case
- support ordinary names, nested paths, names with trailing separators, and separator-only inputs

The Rust version must implement equivalent behavior for determining the basename length from an input path.

## 3. User Scenarios & Testing

### Scenario 1: Plain filename input

A caller provides a path string with no directory separator, such as a single filename.

Expected support:
- the basename component begins at the start of the input
- the basename length equals the full string length

### Scenario 2: Nested path input

A caller provides a path containing directory prefixes and a final filename.

Expected support:
- the basename component begins immediately after the last directory separator
- the basename length equals the final component length

### Scenario 3: Path with trailing separators

A caller provides a path whose final filename is followed by one or more trailing separators.

Expected support:
- trailing separators do not shift the basename start past the actual final component
- basename length is computed from the actual final component, excluding trailing separators

### Scenario 4: Root-like or separator-only input

A caller provides an input consisting only of directory separators, or an input where basename resolution reaches a separator-only result.

Expected support:
- the module returns the same basename start and length behavior as the source C module for separator-only cases
- tests must verify these edge cases explicitly

### Scenario 5: Empty string input

A caller provides an empty path string.

Expected support:
- the Rust version must match the source module’s behavior for an empty input when locating the basename component and computing its length

### Testing expectations

The Rust port must include tests covering at least:
- empty input
- single-component name
- multi-component path
- path ending with one separator
- path ending with multiple separators
- separator-only input
- path where the final component is empty before trailing-separator normalization in source terms

Each test must validate observable results against the behavior defined by `last_component` and `base_len`.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Final component start resolution
The module shall determine the start of the last pathname component for a provided path input, matching the behavior of source function `last_component`.

**Traceability:** `gnu/basename-lgpl.c`, `last_component`

#### FR-2: Separator-aware basename parsing
The module shall treat directory separators as pathname delimiters when resolving the final component, including cases with consecutive or trailing separators, matching source behavior.

**Traceability:** `gnu/basename-lgpl.c`, `last_component`, `base_len`

#### FR-3: Basename length computation
The module shall compute the length of the basename portion for a provided path input, matching the behavior of source function `base_len`.

**Traceability:** `gnu/basename-lgpl.c`, `base_len`

#### FR-4: Trailing-separator handling
The module shall exclude trailing separators from basename length calculation where the source module does so, while preserving source behavior for separator-only and related edge cases.

**Traceability:** `gnu/basename-lgpl.c`, `base_len`, `last_component`

#### FR-5: Empty and degenerate path handling
The module shall accept empty and degenerate path strings and produce results consistent with the source module.

**Traceability:** `gnu/basename-lgpl.c`, `last_component`, `base_len`

### 4.2 Key Entities

#### Path input
A path input is a string value representing a C-style pathname to be analyzed by the module.

#### Basename component
The basename component is the final pathname component identified within the path input.

Relationship:
- `last_component` resolves where the basename component begins within the path input.
- `base_len` resolves how many characters belong to that basename component under source-defined rules.

#### Directory separator
A directory separator is the delimiter used by the source module to split pathname components.

Relationship:
- separator placement controls both basename start detection and basename length trimming behavior.

## 5. Success Criteria

### SC-1: Behavioral equivalence for component start
For all tested inputs in scope, the Rust implementation returns a basename start result equivalent to the source module’s `last_component` behavior.

### SC-2: Behavioral equivalence for basename length
For all tested inputs in scope, the Rust implementation returns a basename length equivalent to the source module’s `base_len` behavior.

### SC-3: Edge-case coverage
Automated tests verify behavior for:
- empty input
- no-separator input
- nested-path input
- trailing-separator input
- multiple-trailing-separator input
- separator-only input

### SC-4: No unsupported feature expansion
The Rust rewrite remains limited to pathname basename parsing behavior evidenced by the source module and does not require unrelated path normalization or filesystem interaction.