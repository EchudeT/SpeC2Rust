# spec.md

## Title

Functional Specification: `module_gnu_basename-lgpl.c_21`

## Overview

This module provides pathname basename support for string-based path processing. Its responsibility is limited to identifying the final pathname component within an input path string and determining the effective length of that basename component while accounting for trailing directory separators.

The Rust rewrite must preserve the observable behavior of the C module in `gnu/basename-lgpl.c`, specifically the functionality represented by:

- `last_component`
- `base_len`

This module is a low-level utility module. It does not own path storage, perform filesystem access, normalize paths, or allocate new path results as part of its core behavior.

## Scope

### In Scope

- Finding the start of the last component in a pathname-like string.
- Computing the effective length of the basename portion of a pathname-like string.
- Handling paths with and without directory separators.
- Handling trailing separators as part of basename-length determination.

### Out of Scope

- Filesystem queries.
- Path canonicalization or normalization beyond the behavior implied by the existing functions.
- Allocation-based path rewriting or path construction.
- Platform abstraction beyond the separator behavior evidenced by the source module.
- Any API not traceable to the listed module functions.

## Feature Specification

### Feature: Last Path Component Identification

The module identifies the last pathname component within an input path string.

Behavioral intent of the Rust version:

- Accept a pathname-like string input.
- Return a view, position, or equivalent Rust representation pointing to the start of the final component of the path.
- If the input contains one or more directory separators, the result starts immediately after the final separator considered by the original module behavior.
- If the input contains no separator, the result refers to the original input start.
- The result must be derived without requiring filesystem interaction.

This feature corresponds to `last_component`.

### Feature: Basename Length Determination

The module determines the effective length of the basename component for a pathname-like string.

Behavioral intent of the Rust version:

- Accept a pathname-like string input.
- Determine the basename component associated with the path.
- Exclude trailing directory separators from the computed basename length where required by the original module behavior.
- Produce a length value that can be used together with the last-component position to identify the basename span within the original input.

This feature corresponds to `base_len`.

### Rust Port Functional Expectation

The Rust rewrite must preserve functional equivalence for valid inputs representable by the chosen Rust API surface. Because the C module operates on string pointers, the Rust version may expose equivalent behavior through borrowed string or byte-slice based interfaces, so long as the module behavior remains traceable to the original functions and preserves the same pathname interpretation rules.

## User Scenarios & Testing

### Scenario 1: Path Without Separators

A caller provides a single-component name such as a filename without any directory separator.

Expected support:

- Last-component identification returns the full input as the basename start.
- Basename-length determination returns the length of the full input.

### Scenario 2: Multi-Component Path

A caller provides a path containing one or more directory separators.

Expected support:

- Last-component identification locates the component following the final separator.
- Basename-length determination returns the length of that final component.

### Scenario 3: Path With Trailing Separators

A caller provides a path whose final component is followed by one or more trailing separators.

Expected support:

- Basename-length determination reflects the effective basename length after ignoring trailing separators according to the source module behavior.
- The result must allow callers to distinguish the meaningful basename portion from separator suffixes.

### Scenario 4: Directory-Only or Separator-Heavy Input

A caller provides an input consisting only of separators or dominated by separators.

Expected support:

- The module returns results consistent with the source logic for locating the last component and computing its effective length.
- The Rust version must match the source module’s observable outcomes for these edge cases.

### Scenario 5: Caller Uses Both Results Together

A caller needs the basename span within the original path without allocating a new string.

Expected support:

- The caller can use the result of last-component identification together with basename-length determination to identify the basename region of the original input.
- The Rust API should support this usage pattern through non-owning results or equivalent index/span semantics.

### Testing Expectations

The Rust rewrite must include tests covering at least:

- Empty or minimal path-like inputs as accepted by the chosen API.
- Names with no separators.
- Names with one separator.
- Names with multiple separators.
- Names ending in one trailing separator.
- Names ending in multiple trailing separators.
- Inputs consisting entirely of separators.
- Consistency between last-component identification and basename-length determination when used together.

## Requirements

### Functional Requirements

#### FR-1: Last Component Start Resolution

The module shall determine the start location of the last pathname component within an input path string.

Traceability: `last_component` in `gnu/basename-lgpl.c`.

#### FR-2: No-Separator Handling

When the input path contains no directory separator recognized by the module, the module shall treat the full input as the last component.

Traceability: `last_component` in `gnu/basename-lgpl.c`.

#### FR-3: Final-Separator Handling for Component Resolution

When the input path contains directory separators, the module shall resolve the last component relative to the final separator according to the source module behavior.

Traceability: `last_component` in `gnu/basename-lgpl.c`.

#### FR-4: Basename Length Computation

The module shall compute the effective length of the basename portion of an input path string.

Traceability: `base_len` in `gnu/basename-lgpl.c`.

#### FR-5: Trailing-Separator Exclusion in Length Result

The basename length computation shall exclude trailing directory separators from the effective basename length where the source module does so.

Traceability: `base_len` in `gnu/basename-lgpl.c`.

#### FR-6: Non-Filesystem Operation

The module shall provide its results purely from the input path string content and shall not require filesystem access.

Traceability: implied by both functions operating on path strings only in `gnu/basename-lgpl.c`.

#### FR-7: Composable Basename Span Semantics

The module’s outputs shall support caller derivation of the basename span within the original input without requiring the module to allocate a replacement string.

Traceability: combined behavior of `last_component` and `base_len` in `gnu/basename-lgpl.c`.

### Key Entities

#### Entity: Input Path String

The central input entity is a pathname-like string supplied by the caller. Both module functions operate on this entity.

Relationships:

- `last_component` derives a position or equivalent reference within this input.
- `base_len` derives a length associated with the basename portion of this input.

Traceability: function signatures in `gnu/basename-lgpl.c`.

#### Entity: Last Component Reference

This entity represents the location of the final pathname component within the input path string.

Relationships:

- It is derived from the input path string.
- It can be paired with basename length to identify the basename span.

Traceability: `last_component` in `gnu/basename-lgpl.c`.

#### Entity: Basename Length Value

This entity represents the effective length of the basename portion of the input path string.

Relationships:

- It is derived from the input path string.
- It complements the last component reference when callers need the basename span.

Traceability: `base_len` in `gnu/basename-lgpl.c`.

## Success Criteria

### Functional Correctness

- For representative path inputs without separators, the Rust version returns results equivalent to treating the entire input as the basename.
- For representative path inputs with separators, the Rust version returns results equivalent to selecting the final path component.
- For representative path inputs with trailing separators, the Rust version computes basename length equivalent to the C module behavior.
- For separator-only edge cases, the Rust version matches the C module’s observable results.

Traceability: `last_component`, `base_len` in `gnu/basename-lgpl.c`.

### API-Level Utility

- The Rust version enables callers to identify the basename region of the original input without requiring allocation of a new string value.
- The Rust results for component start and basename length are consistent when used together on the same input.

Traceability: combined function behavior of `last_component` and `base_len`.

### Test Coverage

- Automated tests demonstrate all scenarios listed in this specification.
- Tests explicitly verify edge cases involving trailing separators and separator-only inputs.
- Tests compare expected basename span outcomes derived from the Rust API against the behavior specified for the source module.

Traceability: `last_component`, `base_len` in `gnu/basename-lgpl.c`.