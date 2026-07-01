# spec.md

## Title

Rust Functional Specification: `module_gnu_gl_convert_16`

## Overview

This module provides conversion from Windows `FILETIME` values to POSIX-style time representations. The analyzed C module exposes behavior for:

- converting a `FILETIME` value into a `struct timespec`
- converting a `FILETIME` value into a POSIX `time_t`

The Rust rewrite must preserve these conversion behaviors for the same conceptual inputs and outputs. The module scope is limited to time representation conversion and does not include broader file status retrieval or unrelated stat functionality.

## Feature Specification

### Purpose

The module translates a Windows file timestamp into POSIX-compatible time forms used by higher-level file metadata logic.

### In-Scope Functionality

The Rust version must implement the following functional behavior:

1. Accept a Windows `FILETIME` input value.
2. Convert that value to a POSIX `timespec`-equivalent result containing:
   - whole seconds
   - sub-second nanoseconds
3. Convert that value to a POSIX `time_t`-equivalent result containing whole seconds only.
4. Preserve the relationship between both conversions so that the seconds component derived for the `timespec` conversion is consistent with the whole-second POSIX conversion for the same input.

### Out of Scope

The Rust module specification does not require:

- file metadata retrieval
- construction or mutation of `struct stat`
- formatting timestamps
- timezone handling
- clock acquisition
- persistence or serialization
- any API beyond the two evidenced conversions

## User Scenarios & Testing

### Scenario 1: Convert a Windows file timestamp to seconds and nanoseconds

A caller has a `FILETIME` representing a file-related timestamp and needs a POSIX-style timestamp with sub-second precision.

Expected behavior:

- the module accepts the `FILETIME`
- the module returns a `timespec`-equivalent value
- the result contains valid second and nanosecond fields representing the same instant

Tests should verify:

- a known `FILETIME` converts to the expected seconds value
- the nanosecond field is produced and is within the valid nanosecond range
- the result matches the expected POSIX representation for known sample values

### Scenario 2: Convert a Windows file timestamp to POSIX whole seconds

A caller needs only the POSIX `time_t` form of a `FILETIME`.

Expected behavior:

- the module accepts the `FILETIME`
- the module returns the whole-second POSIX value for that input

Tests should verify:

- known inputs produce expected `time_t` outputs
- the returned whole seconds match the seconds component of the `timespec` conversion for the same input

### Scenario 3: Use both conversions consistently in file timestamp handling

A caller uses the whole-second conversion in one path and the sub-second conversion in another path for the same source timestamp.

Expected behavior:

- both conversions represent the same source time
- no contradictory second values are produced for the same input

Tests should verify:

- for each tested input, `convert_to_posix(ft)` equals the seconds field of `convert_to_timespec(ft)`

## Requirements

### Functional Requirements

#### FR-1: FILETIME to POSIX timespec conversion

The module shall provide behavior equivalent to converting a Windows `FILETIME` input into a POSIX `timespec`-equivalent value.

Traceability:

- `_gl_convert_FILETIME_to_timespec` in `gnu/stat-w32.c:114-137`
- referenced type: `struct timespec`

#### FR-2: Whole-second and sub-second output for timespec conversion

The `timespec`-equivalent conversion result shall contain both a whole-second component and a nanosecond component representing the source `FILETIME`.

Traceability:

- `_gl_convert_FILETIME_to_timespec` in `gnu/stat-w32.c:114-137`
- referenced type: `struct timespec`

#### FR-3: FILETIME to POSIX time_t conversion

The module shall provide behavior equivalent to converting a Windows `FILETIME` input into a POSIX `time_t`-equivalent whole-second result.

Traceability:

- `_gl_convert_FILETIME_to_POSIX` in `gnu/stat-w32.c:139-156`

#### FR-4: Conversion consistency across outputs

For the same `FILETIME` input, the whole-second result from the POSIX conversion shall be consistent with the whole-second component of the `timespec`-equivalent conversion.

Traceability:

- `_gl_convert_FILETIME_to_timespec` in `gnu/stat-w32.c:114-137`
- `_gl_convert_FILETIME_to_POSIX` in `gnu/stat-w32.c:139-156`

### Key Entities

#### FILETIME

A Windows timestamp input entity used as the source value for both conversions.

Relationship:

- serves as the sole evidenced input to the module’s conversion behavior

Traceability:

- `_gl_convert_FILETIME_to_timespec (const FILETIME *ft)`
- `_gl_convert_FILETIME_to_POSIX (const FILETIME *ft)`

#### POSIX timespec-equivalent value

A time value with second and nanosecond components produced from a `FILETIME`.

Relationship:

- derived from `FILETIME`
- provides the higher-precision output of the module
- shares its whole-second interpretation with the POSIX whole-second conversion

Traceability:

- return type of `_gl_convert_FILETIME_to_timespec`
- referenced type: `struct timespec`

#### POSIX time_t-equivalent value

A whole-second POSIX time value produced from a `FILETIME`.

Relationship:

- derived from `FILETIME`
- represents the lower-precision output of the module
- must align with the second component of the timespec-equivalent value for the same input

Traceability:

- return type of `_gl_convert_FILETIME_to_POSIX`

## Success Criteria

1. The Rust module exposes functionality covering both evidenced conversions from `FILETIME` input:
   - to a `timespec`-equivalent result
   - to a `time_t`-equivalent result

   Traceability:
   - `_gl_convert_FILETIME_to_timespec`
   - `_gl_convert_FILETIME_to_POSIX`

2. For a defined set of known `FILETIME` test inputs, the Rust `timespec` conversion returns expected second and nanosecond values matching the C module’s observable conversion behavior.

   Traceability:
   - `struct timespec`

3. For the same set of known `FILETIME` test inputs, the Rust whole-second conversion returns expected POSIX whole-second values matching the C module’s observable conversion behavior.

   Traceability:

4. For every tested input shared between both conversion paths, the Rust whole-second result exactly equals the seconds component of the Rust `timespec`-equivalent result.

   Traceability:

5. For every tested `timespec` conversion result, the nanosecond component is valid for POSIX-style representation.

   Traceability: