# Implementation Plan: module_gnu_gl_convert_16

## Summary

Port the time-conversion logic from `gnu/stat-w32.c` into a focused Rust module that preserves the existing behavior and scope of the C implementation. The Rust work should migrate only the functionality represented by:

- `_gl_convert_FILETIME_to_timespec`
- `_gl_convert_FILETIME_to_POSIX`

The implementation should center on translating Windows `FILETIME`-style values into Rust representations equivalent to POSIX time components, using explicit integer arithmetic and narrow helper functions rather than introducing broader date/time abstractions. The preferred approach is to represent the relevant Windows timestamp layout with small Rust structs, perform conversion using fixed-width integer types, and expose internal conversion functions aligned closely with the original C responsibilities.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.76 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the provided module scope

### Testing
- `cargo test`

### Performance Goals
- Match the C module’s constant-time conversion behavior
- Avoid heap allocation
- Use fixed-width integer arithmetic with minimal branching
- Preserve conversion accuracy for supported input ranges
- Keep overhead limited to plain value transformation

## Module Mapping

### Source Mapping
- C source: `gnu/stat-w32.c`
- Rust target: `src/module_gnu_gl_convert_16.rs`

### Function Mapping
- `_gl_convert_FILETIME_to_timespec`
  - Port to a Rust function in `src/module_gnu_gl_convert_16.rs`
  - Keep it narrowly scoped to converting Windows file time input into a Rust timespec-like representation
- `_gl_convert_FILETIME_to_POSIX`
  - Keep it as the lower-level conversion routine producing POSIX-aligned seconds and sub-second components or equivalent internal values

### Visibility Guidance
- Prefer `pub(crate)` visibility unless the wider crate already requires public export
- Keep helper constants and arithmetic utilities private to the module

## Data Model

The input identifies only anonymous C data structures, so the Rust plan should map only the structures required by these conversion functions and avoid introducing unrelated wrappers.

### C to Rust Structure Mapping

- Anonymous Windows `FILETIME`-style structure
  - Rust: `struct FileTime { low_date_time: u32, high_date_time: u32 }`
  - Purpose: represent the 64-bit Windows timestamp split into two 32-bit fields

- Anonymous POSIX time output structure equivalent to `timespec`
  - Rust: `struct Timespec { tv_sec: i64, tv_nsec: i32 }`
  - Purpose: hold converted second and nanosecond components
  - If the crate already has an established internal timespec type, reuse it instead of creating a new one

- Anonymous intermediate combined timestamp representation
  - Rust: no dedicated struct unless needed
  - Prefer direct use of `u64` for combined 100-nanosecond tick arithmetic

### Type and Memory Notes
- Combine the two `u32` `FILETIME` halves into a `u64` with explicit shifts and masking
- Use signed output for POSIX seconds where negative values may occur after epoch adjustment
- Keep conversion as pure value transformation without borrowing complexity or dynamic allocation
- Avoid `unsafe` unless required by surrounding project constraints; none is expected for this module alone

### Error Handling Notes
- If the C logic assumes all inputs are structurally valid, mirror that with infallible internal conversion functions
- If range checks are needed during signed conversion, use explicit checked or saturating arithmetic only when demanded by the original behavior
- Do not introduce general-purpose error enums unless a concrete failure mode is required by the existing call pattern

## Implementation Phases

### Phase 1: Create module skeleton and data mappings
- Add `src/module_gnu_gl_convert_16.rs`
- Define the minimal Rust representations needed for:
  - Windows `FILETIME` input
  - POSIX-style `timespec` output
- Add private constants for:
  - Windows-to-Unix epoch offset
  - 100-nanosecond to second conversion factors
  - Nanosecond scaling values
- Wire the module into the crate using standard Rust module declarations only where required by the existing project layout

### Phase 2: Port conversion routines
- Implement the low-level conversion corresponding to `_gl_convert_FILETIME_to_POSIX`
  - Combine high/low 32-bit fields into a single `u64`
  - Subtract the epoch offset using explicit fixed-width arithmetic
  - Derive whole seconds and sub-second remainder from 100-nanosecond units
- Implement the higher-level conversion corresponding to `_gl_convert_FILETIME_to_timespec`
  - Build the Rust `Timespec` value from the POSIX conversion result
  - Preserve normalization of sub-second fields into nanoseconds
- Keep logic closely aligned with the original function split rather than redesigning the API surface

### Phase 3: Validate arithmetic behavior with unit tests
- Add unit tests in the same module or the crate’s standard test location
- Cover:
  - Epoch-aligned conversions
  - Non-zero sub-second conversions
  - Representative timestamps after the Unix epoch
  - Boundary-oriented cases relevant to integer splitting and recombination
- Confirm normalized output:
  - seconds in expected signed range
  - nanoseconds within `0..1_000_000_000`

### Phase 4: Integrate and clean up
- Replace any direct dependence on the C implementation for this module’s responsibilities within the Rust branch
- Review for unnecessary allocations, extra abstractions, and unused items
- Ensure naming, visibility, and documentation remain limited to the migrated scope
- Run `cargo test` and resolve any type or arithmetic mismatches found during integration