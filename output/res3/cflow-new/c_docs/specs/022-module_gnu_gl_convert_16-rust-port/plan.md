# Implementation Plan: module_gnu_gl_convert_16

## Summary

This module ports the time-conversion logic currently located in `gnu/stat-w32.c` into Rust, limited to the two identified functions:

- `_gl_convert_FILETIME_to_timespec`
- `_gl_convert_FILETIME_to_POSIX`

The Rust implementation should preserve the existing conversion behavior and boundary handling while translating the code into idiomatic, safe Rust where possible. The technical approach is to isolate the Windows timestamp conversion rules into a small Rust module, represent the relevant C data shapes with minimal Rust equivalents, and keep the implementation narrowly scoped to the existing functionality rather than introducing broader date/time abstractions.

The migration should focus on:

- exact integer-based conversion from Windows `FILETIME` units to POSIX-oriented time values,
- preserving signedness and range behavior intentionally,
- making overflow-sensitive arithmetic explicit,
- keeping the module self-contained and aligned with the original file/function boundaries.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.74+

This is sufficient for standard-library-based integer and time-related implementation without requiring unstable features.

### Primary Dependencies

- Rust standard library only

Recommended approach:
- use `core`/`std` integer arithmetic and native structs defined locally for the migrated logic,
- avoid third-party crates because the input provides no evidence that external date/time or Windows binding crates are required for this isolated port.

### Testing

- `cargo test`

Test coverage should include:
- representative `FILETIME` inputs,
- epoch boundary conversions,
- sub-second conversion behavior,
- overflow/underflow-sensitive paths that correspond to the original C arithmetic expectations.

### Performance Goals

- Match the C implementation’s constant-time integer conversion behavior.
- Avoid heap allocation entirely.
- Keep conversions branch-light and based on primitive arithmetic only.
- Preserve predictable cost for repeated calls in filesystem/stat-related code paths.

## Module Mapping

### Source Mapping

- C source: `gnu/stat-w32.c`
- Rust target module: `src/module_gnu_gl_convert_16.rs`

If the crate already organizes code by cluster or source-area, the implementation should still remain a single Rust module for this migration unit and not be split further unless existing project layout requires a specific placement.

### Function Mapping

- `_gl_convert_FILETIME_to_timespec`
  - migrate to a Rust function with crate-level visibility matching actual call requirements
  - preserve input/output semantics with explicit Rust return/value mutation strategy

- `_gl_convert_FILETIME_to_POSIX`
  - migrate to a Rust helper function in the same module
  - keep it as an internal helper unless external use already exists in the Rust port structure

### Scope Constraint

Only migrate the logic necessary for the two listed functions from `gnu/stat-w32.c`. Do not absorb unrelated `stat-w32.c` behavior into this module plan.

## Data Model

The analysis only identifies anonymous C data structures, so the Rust plan should map only the data shapes actually needed by these functions.

### C to Rust Structure Mapping

- C `FILETIME`-compatible layout
  - Rust: local struct mirroring the two 32-bit fields used by Windows file time representation
  - example shape:
    - `dwLowDateTime: u32`
    - `dwHighDateTime: u32`

This should be a minimal structural representation used only if the wider project does not already define an equivalent type.

- C `struct timespec`
  - Rust: local struct if the project already uses a portable replacement
  - otherwise a module-local equivalent with:
    - seconds field as signed integer type compatible with project conventions
    - nanoseconds field as signed integer type or platform-compatible integer as required by surrounding code

The exact Rust integer sizes should be chosen to match the project’s existing porting conventions. If no convention exists, prefer:
- seconds: `i64`
- nanoseconds: `i32` or `i64` depending on broader compatibility needs

- C anonymous intermediate/integer aggregates
  - Rust: plain local variables and helper constants rather than creating additional structs

### Constant Mapping

The conversion logic will likely require fixed constants for:

- Windows tick size relative to nanoseconds
- offset between Windows epoch and Unix epoch

These should be defined as module-local `const` values with explicit integer types to avoid accidental promotion or truncation.

### Memory Management

- No dynamic allocation is required.
- Inputs should be passed by reference or value depending on layout and existing call patterns.
- Output should use direct return values where practical; if the original behavior mutates an out-parameter, mirror that only when required by surrounding migrated interfaces.

### Error Handling

The original functions appear to be pure conversion helpers rather than allocation or I/O routines. The Rust port should therefore:

- avoid introducing `Result` unless the original C logic has a real failure path,
- encode deterministic conversion behavior directly,
- make overflow assumptions explicit with checked or carefully bounded arithmetic during implementation,
- use debug assertions only for invariant documentation, not for new runtime control flow.

## Implementation Phases

## Phase 1: Establish module skeleton and type mappings

- Create `src/module_gnu_gl_convert_16.rs`.
- Add the minimal Rust representations required for:
  - `FILETIME` input shape
  - `timespec`-equivalent output shape, if not already available elsewhere in the project
- Define module-local constants for:
  - Windows-to-Unix epoch offset
  - 100-nanosecond tick conversions
- Decide and document function visibility based on actual call sites in the Rust branch.
- Keep naming close enough to the C source to preserve traceability during review.

### Deliverable

A compiling Rust module skeleton with type definitions and constants, but without finalized conversion logic.

## Phase 2: Port `_gl_convert_FILETIME_to_POSIX`

- Translate the integer reconstruction from high/low `FILETIME` fields into a single 64-bit value.
- Implement the epoch-offset conversion to POSIX time units.
- Preserve the arithmetic order carefully to avoid signed/unsigned mistakes.
- Keep the helper narrowly scoped and avoid introducing general-purpose date/time helpers.
- Add unit tests for:
  - zero or minimal values,
  - exact Unix epoch correspondence,
  - representative positive converted values,
  - boundary-oriented cases relevant to 64-bit arithmetic.

### Deliverable

A tested Rust implementation of `_gl_convert_FILETIME_to_POSIX` with clear constant usage and no allocation.

## Phase 3: Port `_gl_convert_FILETIME_to_timespec`

- Build the `timespec` conversion on top of the same `FILETIME` interpretation rules.
- Split whole seconds and sub-second remainder with explicit integer arithmetic.
- Ensure nanosecond normalization remains valid for the target `timespec` representation.
- Preserve any original truncation behavior rather than replacing it with rounding.
- Add unit tests covering:
  - second/nanosecond field composition,
  - exact second boundaries,
  - sub-second remainder handling,
  - consistency with `_gl_convert_FILETIME_to_POSIX` where both are comparable.

### Deliverable

A complete Rust implementation of `_gl_convert_FILETIME_to_timespec` with passing tests and behavior aligned to the C source.

## Phase 4: Integration cleanup and verification

- Align the module with the project’s existing naming, visibility, and import conventions.
- Remove any temporary compatibility scaffolding not required after the migration.
- Verify that all tests pass with `cargo test`.
- Perform a final review for:
  - integer overflow exposure,
  - signedness correctness,
  - no unnecessary allocations,
  - no scope expansion beyond the two migrated functions.

### Deliverable

Finalized module ready for inclusion on branch `022-module_gnu_gl_convert_16-rust-port`.