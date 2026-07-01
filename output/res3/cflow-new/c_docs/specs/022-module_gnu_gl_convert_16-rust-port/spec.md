# spec.md

## Title
Rust Functional Specification for `module_gnu_gl_convert_16`

## Document Control
- Project: `cflow-new`
- Module: `module_gnu_gl_convert_16`
- Category: `module_cluster`
- Source basis: `gnu/stat-w32.c`
- Rust branch: `022-module_gnu_gl_convert_16-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides Windows-specific time conversion behavior used to translate a `FILETIME` value into POSIX-oriented time representations.

The Rust rewrite must preserve the observable behavior of the source module for the identified conversion responsibilities:
- conversion from `FILETIME` to `struct timespec`
- conversion from `FILETIME` to POSIX `time_t`

The module scope is limited to these conversion behaviors evidenced by the source functions in `gnu/stat-w32.c`. No additional capabilities are part of this specification.

## Feature Specification

### Feature: Convert Windows `FILETIME` to POSIX-oriented time values
The module shall accept a Windows `FILETIME` input and produce equivalent POSIX-style time values in two forms:
- a `timespec` value with whole seconds and sub-second nanoseconds
- a `time_t` value representing whole POSIX seconds

This feature exists to support code that needs Windows file times expressed in the time model expected by POSIX-oriented logic.

### Supported behavior
1. The module shall interpret the input as a Windows `FILETIME`.
2. The module shall provide a conversion result in `timespec` form.
3. The module shall provide a conversion result in whole-second POSIX form.
4. The two conversion results shall be consistent with one another for the same input, with the `time_t` result matching the whole-second component of the converted time.

## User Scenarios & Testing

### Scenario 1: Convert a Windows file timestamp for POSIX-style metadata handling
A caller has a `FILETIME` obtained from Windows file metadata and needs a `timespec` value for POSIX-style timestamp handling.

Expected support:
- The caller provides a `FILETIME`.
- The module returns a `timespec` representing the same instant in POSIX-compatible form.

Testing:
- Provide representative `FILETIME` inputs.
- Verify that returned `timespec.tv_sec` and `timespec.tv_nsec` correspond to the expected POSIX instant.

### Scenario 2: Convert the same timestamp to whole POSIX seconds
A caller needs only the whole-second POSIX timestamp for comparison, storage, or compatibility with APIs using `time_t`.

Expected support:
- The caller provides a `FILETIME`.
- The module returns a whole-second POSIX time value.

Testing:
- Provide representative `FILETIME` inputs.
- Verify that the returned second count matches the expected POSIX second value.

### Scenario 3: Cross-check both conversion forms for consistency
A caller uses both conversion paths on the same `FILETIME` and expects aligned results.

Expected support:
- The `time_t` result matches the whole-second portion of the `timespec` result for the same input.

Testing:
- For the same input, compare the direct POSIX-second conversion result with the `timespec.tv_sec` result.
- Confirm consistency across multiple representative timestamps.

## Requirements

### Functional Requirements

#### FR-1: `FILETIME` to `timespec` conversion
The Rust module shall implement conversion from a Windows `FILETIME` input to a `timespec` result.

Traceability:
- Function: `_gl_convert_FILETIME_to_timespec`
- File: `gnu/stat-w32.c`

#### FR-2: `FILETIME` to POSIX whole-second conversion
The Rust module shall implement conversion from a Windows `FILETIME` input to a POSIX `time_t` result.

Traceability:
- Function: `_gl_convert_FILETIME_to_POSIX`
- File: `gnu/stat-w32.c`

#### FR-3: Equivalent-input consistency
For the same `FILETIME` input, the Rust module shall produce conversion outputs that represent the same underlying time, with the whole-second result aligned to the seconds component of the `timespec` conversion.

Traceability:
- Functions: `_gl_convert_FILETIME_to_timespec`, `_gl_convert_FILETIME_to_POSIX`
- File: `gnu/stat-w32.c`
- Type: `struct timespec`

#### FR-4: Sub-second preservation in `timespec` form
When converting to `timespec`, the Rust module shall return both whole-second and sub-second components rather than collapsing the result to seconds only.

Traceability:
- Function: `_gl_convert_FILETIME_to_timespec`
- File: `gnu/stat-w32.c`
- Type: `struct timespec`

### Key Entities

#### Entity: `FILETIME`
A Windows timestamp input value used as the source for both conversions.

Relationships:
- Serves as the input to `FILETIME`-to-`timespec` conversion.
- Serves as the input to `FILETIME`-to-`time_t` conversion.

Traceability:
- Functions: `_gl_convert_FILETIME_to_timespec`, `_gl_convert_FILETIME_to_POSIX`

#### Entity: `timespec`
A POSIX-style time structure containing second and sub-second fields.

Relationships:
- Produced by the `FILETIME`-to-`timespec` conversion.
- Its whole-second component must align with the POSIX whole-second conversion for the same input.

Traceability:
- Function: `_gl_convert_FILETIME_to_timespec`
- File: `gnu/stat-w32.c`

#### Entity: `time_t`
A POSIX whole-second time value.

Relationships:
- Produced directly from `FILETIME`.
- Represents the whole-second form of the same converted instant represented by `timespec`.

Traceability:
- Function: `_gl_convert_FILETIME_to_POSIX`
- File: `gnu/stat-w32.c`

## Success Criteria

### SC-1: Functional completeness
The Rust rewrite exposes behavior covering both evidenced conversions:
- `FILETIME` to `timespec`
- `FILETIME` to POSIX whole seconds

Traceability:
- Functions: `_gl_convert_FILETIME_to_timespec`, `_gl_convert_FILETIME_to_POSIX`

### SC-2: Result consistency
For every tested `FILETIME` input, the whole-second conversion result equals the seconds component of the `timespec` conversion result for that same input.

Measurement:
- Verified by tests over representative inputs.

Traceability:
- Functions: `_gl_convert_FILETIME_to_timespec`, `_gl_convert_FILETIME_to_POSIX`
- Type: `timespec`

### SC-3: Sub-second output presence
The `timespec` conversion returns a structure containing both second and nanosecond components, and tests validate that the sub-second component is populated according to the converted input where applicable.

Measurement:
- Verified by tests using inputs with non-zero sub-second content.

Traceability:
- Function: `_gl_convert_FILETIME_to_timespec`
- Type: `timespec`

### SC-4: Source-scope adherence
The Rust module implements only the evidenced conversion responsibilities from this module and does not require unrelated functionality to satisfy conformance.

Measurement:
- Specification review confirms all implemented required behaviors are traceable to the identified source file and functions.

Traceability:
- File: `gnu/stat-w32.c`
- Functions: `_gl_convert_FILETIME_to_timespec`, `_gl_convert_FILETIME_to_POSIX`