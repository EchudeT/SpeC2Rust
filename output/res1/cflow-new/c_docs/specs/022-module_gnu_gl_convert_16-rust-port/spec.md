# spec.md

## Title

Rust Functional Specification for `module_gnu_gl_convert_16`

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_gl_convert_16`
- Category: `module_cluster`
- Rust branch: `022-module_gnu_gl_convert_16-rust-port`
- Source basis: `gnu/stat-w32.c`
- Generation date: `2026-06-11`

## Overview

This module provides conversion from Windows `FILETIME` values into POSIX-oriented time representations used by the surrounding code.

The Rust rewrite must preserve the functional role of the source module segment: given a Windows file timestamp, it must compute either:

- a `timespec`-style value with seconds and nanoseconds, or
- a POSIX `time_t`-style seconds value.

The specification covers only the behavior evidenced by the analyzed functions and referenced types.

## Scope

### In Scope

- Converting a supplied `FILETIME` value into a `timespec`-equivalent result.
- Converting a supplied `FILETIME` value into a POSIX seconds result.
- Representing sub-second precision for the `timespec` conversion.
- Producing deterministic results from the provided input value.

### Out of Scope

- Retrieval of file metadata from the operating system.
- Population of `struct stat`.
- General date/time formatting, parsing, timezone handling, or calendar calculations.
- Any APIs or behaviors not evidenced by the analyzed module segment.

## Feature Specification

### Feature Summary

The module is a small time-conversion utility focused on translating Windows file timestamps into POSIX-compatible forms.

A Rust version must implement two conversion behaviors:

1. Convert a Windows `FILETIME` input into a `timespec`-like result containing whole seconds plus nanosecond remainder.
2. Convert the same kind of input into a POSIX seconds value.

### Functional Behavior

#### FILETIME to `timespec` Conversion

Given a valid `FILETIME` input, the module shall return a `timespec`-equivalent structure representing the same timestamp in POSIX time terms.

The returned value must include:

- a seconds field representing whole elapsed POSIX seconds, and
- a nanoseconds field representing the remaining fractional second component.

The conversion must preserve sub-second precision available from the `FILETIME` input to the granularity represented by nanoseconds.

#### FILETIME to POSIX Seconds Conversion

Given a valid `FILETIME` input, the module shall return a POSIX `time_t`-equivalent seconds value for that same timestamp.

This conversion is the whole-second form of the Windows timestamp expressed in POSIX time terms.

#### Consistency Between the Two Conversions

For the same input timestamp, the seconds component of the `timespec` conversion and the POSIX seconds conversion must agree.

This consistency is required because both functions convert the same source time into POSIX-based representations at different precision levels.

## User Scenarios & Testing

### Scenario 1: Convert a Windows file timestamp for POSIX-style metadata handling

A caller has a Windows `FILETIME` obtained from file metadata and needs a POSIX-style timestamp representation with sub-second precision.

Expected support:

- The module accepts the `FILETIME`.
- The module returns a `timespec`-equivalent value.
- The result contains both whole seconds and nanoseconds.

Test focus:

- Verify that a known `FILETIME` produces the expected seconds and nanoseconds values.

### Scenario 2: Convert a Windows file timestamp when only whole seconds are needed

A caller has a Windows `FILETIME` but only needs POSIX whole-second time for compatibility with code using `time_t` semantics.

Expected support:

- The module accepts the `FILETIME`.
- The module returns the corresponding whole-second POSIX value.

Test focus:

- Verify that a known `FILETIME` produces the expected POSIX seconds result.

### Scenario 3: Use both conversions on the same input

A caller invokes both conversion operations on one `FILETIME` and expects them to remain aligned.

Expected support:

- The `timespec` conversion returns seconds and nanoseconds.
- The POSIX conversion returns whole seconds.
- The whole seconds from both results are identical.

Test focus:

- For the same input, assert that:
  - `timespec.tv_sec == posix_seconds`
  - `timespec.tv_nsec` is within the valid nanosecond range.

### Scenario 4: Convert timestamps with sub-second content

A caller needs to preserve the fractional-second portion of a Windows timestamp.

Expected support:

- The module returns a non-zero nanosecond component when the input encodes a fractional second.
- The returned nanosecond component corresponds to the fractional part of the same timestamp represented by the seconds field.

Test focus:

- Use a `FILETIME` fixture representing a non-integer second boundary.
- Verify that the `timespec` nanoseconds value reflects the fractional component.
- Verify that the POSIX seconds result still matches the whole-second part.

## Requirements

### Functional Requirements

#### FR-1: FILETIME input conversion to `timespec`

The Rust module shall provide behavior equivalent to `_gl_convert_FILETIME_to_timespec`, converting a supplied Windows `FILETIME` into a `timespec`-style result.

Traceability:

- Function: `_gl_convert_FILETIME_to_timespec`
- Source: `gnu/stat-w32.c:114-137`

#### FR-2: `timespec` result shall contain whole seconds and nanoseconds

The `timespec`-style result shall expose:

- a whole-seconds component, and
- a nanoseconds component representing the remainder below one second.

Traceability:

- Function: `_gl_convert_FILETIME_to_timespec`
- Referenced type: `struct timespec`
- Source: `gnu/stat-w32.c:114-137`

#### FR-3: FILETIME input conversion to POSIX seconds

The Rust module shall provide behavior equivalent to `_gl_convert_FILETIME_to_POSIX`, converting a supplied Windows `FILETIME` into a POSIX `time_t`-style whole-seconds value.

Traceability:

- Function: `_gl_convert_FILETIME_to_POSIX`
- Source: `gnu/stat-w32.c:139-156`

#### FR-4: Shared timestamp meaning across both conversions

For the same `FILETIME` input, both conversion behaviors shall represent the same timestamp in POSIX terms.

Traceability:

- Functions:
  - `_gl_convert_FILETIME_to_timespec`
  - `_gl_convert_FILETIME_to_POSIX`
- Source: `gnu/stat-w32.c:114-156`

#### FR-5: Whole-second consistency across conversion outputs

For the same `FILETIME` input, the whole-seconds component produced by the `timespec` conversion shall equal the whole-seconds result produced by the POSIX conversion.

Traceability:

- Functions:
  - `_gl_convert_FILETIME_to_timespec`
  - `_gl_convert_FILETIME_to_POSIX`
- Referenced type: `struct timespec`
- Source: `gnu/stat-w32.c:114-156`

#### FR-6: Sub-second precision shall be represented in nanoseconds for `timespec` conversion

When the input `FILETIME` includes a fractional-second portion, the `timespec` conversion shall preserve that portion as a nanosecond remainder rather than discarding it.

Traceability:

- Function: `_gl_convert_FILETIME_to_timespec`
- Referenced type: `struct timespec`
- Source: `gnu/stat-w32.c:114-137`

### Key Entities

#### `FILETIME`

A Windows timestamp input value supplied to the conversion functions.

Relationship to module behavior:

- It is the source timestamp representation for both supported conversions.

Traceability:

- Functions:
  - `_gl_convert_FILETIME_to_timespec`
  - `_gl_convert_FILETIME_to_POSIX`
- Source: `gnu/stat-w32.c:114-156`

#### `timespec`

A POSIX-style timestamp structure with separate whole-second and nanosecond fields.

Relationship to module behavior:

- It is the output entity of the higher-precision conversion.
- Its seconds field must align with the POSIX-seconds conversion for the same input.

Traceability:

- Function: `_gl_convert_FILETIME_to_timespec`
- Referenced type: `struct timespec`
- Source: `gnu/stat-w32.c:114-137`

#### POSIX seconds value (`time_t`-style)

A whole-second POSIX timestamp result.

Relationship to module behavior:

- It is the output entity of the whole-second conversion.
- It corresponds to the seconds component of the `timespec` output for the same input.

Traceability:

- Function: `_gl_convert_FILETIME_to_POSIX`
- Source: `gnu/stat-w32.c:139-156`

## Success Criteria

### SC-1: Correct `timespec` conversion behavior

For a defined set of known `FILETIME` fixtures, the Rust module returns `timespec` results whose seconds and nanoseconds match expected POSIX-equivalent values.

Traceability:

- `_gl_convert_FILETIME_to_timespec`
- `struct timespec`
- `gnu/stat-w32.c:114-137`

### SC-2: Correct POSIX seconds conversion behavior

For a defined set of known `FILETIME` fixtures, the Rust module returns whole-second POSIX results matching expected values.

Traceability:

- `_gl_convert_FILETIME_to_POSIX`
- `gnu/stat-w32.c:139-156`

### SC-3: Cross-conversion consistency

For every shared test fixture used with both conversions, the seconds field of the `timespec` result equals the POSIX seconds result.

Traceability:

- `_gl_convert_FILETIME_to_timespec`
- `_gl_convert_FILETIME_to_POSIX`
- `gnu/stat-w32.c:114-156`

### SC-4: Valid nanosecond field in `timespec` results

For every `timespec` result produced by the Rust module, the nanosecond component is within the valid nanosecond range for a fractional second.

Traceability:

- `_gl_convert_FILETIME_to_timespec`
- `struct timespec`
- `gnu/stat-w32.c:114-137`

### SC-5: Fractional-second preservation

At least one test fixture containing a non-zero fractional-second `FILETIME` value produces a `timespec` result with a non-zero nanosecond component while maintaining whole-second consistency with the POSIX conversion.

Traceability:

- `_gl_convert_FILETIME_to_timespec`
- `_gl_convert_FILETIME_to_POSIX`
- `gnu/stat-w32.c:114-156`

## Acceptance Notes

- The Rust rewrite is acceptable only if it preserves the evidenced conversion behavior of the source module segment.
- No additional public functionality is required beyond the conversions specified here.
- Any Rust type substitutions are acceptable only if they preserve the documented functional meaning of `FILETIME`, `timespec`, and POSIX whole-second output.