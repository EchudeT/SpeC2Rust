# spec.md

## Title

Functional Specification for `module_gnu_basename-lgpl.c_21` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_basename-lgpl.c_21`
- Category: `module_cluster`
- Source file: `gnu/basename-lgpl.c`
- Rust branch: `027-module_gnu_basename_lgpl.c_21-rust-port`
- Generation date: 2026-06-11

## Overview

This module provides pathname basename support limited to identifying the final pathname component and determining the effective length of that component when trailing directory separators are present.

The Rust rewrite must preserve the observable behavior of the source module’s two functions:

- locating the last pathname component within an input path string
- computing the length of that basename component while excluding trailing separators from the component length calculation

The specification is limited to behavior evidenced by `last_component` and `base_len` in `gnu/basename-lgpl.c`.

## Feature Specification

### Summary

The module operates on path-like strings and supports basename-oriented analysis without performing filesystem access. It treats the input as a character string and derives:

1. the start position of the last path component
2. the length of that component after ignoring trailing separators

### Supported Behavior

#### Last component identification

Given a pathname string, the module must identify the substring that begins at the final component of the path. This includes handling:

- paths containing one or more directory separators before the final component
- paths with trailing separators
- paths consisting entirely of separators
- paths with no separator at all

The returned result is conceptually a view into the original input starting at the detected basename position.

#### Basename length calculation

Given a pathname string, the module must determine the length of the basename portion associated with the last component, excluding trailing separators from the measured length.

This behavior must be consistent with the last-component interpretation used by the module.

### Out of Scope

The Rust version must not introduce behavior not evidenced by the source module, including:

- filesystem resolution
- path normalization beyond the basename-oriented separator handling already described
- platform-specific path semantics beyond the separator handling evidenced in the source file
- allocation-heavy path transformation APIs as replacement functionality

## User Scenarios & Testing

### Scenario 1: Path with a normal filename

A caller supplies a path containing directories and a final filename component.

Example shape:
- input: `dir/sub/file`

Expected support:
- last-component lookup identifies `file`
- basename length is the character count of `file`

### Scenario 2: Input already is a basename

A caller supplies a string with no directory separator.

Example shape:
- input: `file`

Expected support:
- last-component lookup identifies the whole string
- basename length equals the full string length

### Scenario 3: Path with trailing separators after the final component

A caller supplies a path ending in one or more separators.

Example shape:
- input: `dir/file///`

Expected support:
- the last component remains the final basename component
- basename length excludes the trailing separators

### Scenario 4: Directory-only or separator-only input

A caller supplies a string that contains only separators, or separators with no ordinary trailing component.

Example shape:
- input: `///`

Expected support:
- last-component lookup returns the module-defined final component position consistent with the C source behavior
- basename length reflects the effective basename length defined by the source behavior for this case

### Scenario 5: Mixed internal repeated separators

A caller supplies a path containing repeated separators between components.

Example shape:
- input: `dir///sub//file`

Expected support:
- last-component lookup identifies the final non-separator component
- basename length corresponds to that component

### Testing Expectations

The Rust port must be tested against behavior derived from the source module for:

- no-separator input
- single-separator and multi-separator paths
- trailing-separator input
- separator-only input
- repeated internal separator input
- empty-string input if accepted by the calling context of the original functions

Tests must verify both:
- component start selection behavior
- component length behavior

Where direct pointer semantics do not apply in Rust, tests must verify equivalent slice or index outcomes.

## Requirements

### Functional Requirements

#### FR-1: Final component start detection

The module shall provide behavior equivalent to identifying the start of the last pathname component from an input path string.

Traceability:
- `gnu/basename-lgpl.c`
- `last_component`

#### FR-2: Whole-string basename handling

When the input contains no directory separator before the basename, the module shall treat the input as a single component.

Traceability:
- `gnu/basename-lgpl.c`
- `last_component`
- `base_len`

#### FR-3: Trailing separator tolerance

When the input ends with one or more directory separators, the module shall still determine the basename according to the source module’s rules rather than treating trailing separators as part of the basename length.

Traceability:
- `gnu/basename-lgpl.c`
- `last_component`
- `base_len`

#### FR-4: Basename length reporting

The module shall provide behavior equivalent to computing the effective length of the basename component associated with the input path.

Traceability:
- `gnu/basename-lgpl.c`
- `base_len`

#### FR-5: Exclusion of trailing separators from length

The reported basename length shall exclude trailing directory separators from the measured component length.

Traceability:
- `gnu/basename-lgpl.c`
- `base_len`

#### FR-6: Separator-only path handling

The module shall preserve source-compatible behavior for inputs composed entirely of directory separators.

Traceability:
- `gnu/basename-lgpl.c`
- `last_component`
- `base_len`

#### FR-7: Non-owning interpretation of input

The module shall derive basename position and length from the supplied input string content, without requiring filesystem interaction or semantic reinterpretation of the path beyond separator-based component detection.

Traceability:
- `gnu/basename-lgpl.c`
- `last_component`
- `base_len`

### Key Entities

This module does not define custom structs or aggregate data types in the analyzed source. Its key entities are function-level string inputs and derived basename views:

- **Input path string**: the source pathname text examined by the module.
- **Last component position**: the derived start location of the basename within the input path.
- **Basename length**: the derived size of the effective basename component after excluding trailing separators as defined by the source behavior.

Relationship:
- both derived entities are computed from the same input path string
- basename length must be consistent with the module’s interpretation of the last component

## Success Criteria

### Behavioral Correctness

- The Rust port reproduces the source module’s basename start-selection behavior for representative inputs covering plain names, nested paths, repeated separators, trailing separators, and separator-only strings.
- The Rust port reproduces the source module’s basename length results for the same representative inputs.
- For each tested input, the Rust port’s component position and length are mutually consistent.

### Interface Equivalence

- The Rust implementation exposes module behavior sufficient to obtain the equivalent of:
  - the final component start within a provided path string
  - the effective basename length for that path string
- Equivalent results are expressed in Rust-native form without changing the specified behavior.

### Verification Criteria

- A test suite maps each functional requirement to one or more unit tests derived from the source file behavior.
- No test demonstrates divergence from the C module for the covered path cases.
- The port introduces no additional required behaviors beyond those specified from `last_component` and `base_len`.