# Implementation Plan: module_gnu_gl_convert_16

## Summary

This module ports the time-conversion logic currently implemented in `gnu/stat-w32.c` into Rust, limited to the two identified functions:

- `_gl_convert_FILETIME_to_timespec`
- `_gl_convert_FILETIME_to_POSIX`

The Rust implementation should preserve the existing conversion behavior from Windows `FILETIME` values into POSIX-oriented time representations, while keeping the scope narrow and aligned with the original file-level responsibility. The technical approach is to translate the arithmetic and boundary handling directly into safe Rust helpers, using explicit integer-width types and a small internal representation for the Windows timestamp fields.

The migration should remain focused on:
- deterministic conversion logic,
- equivalent handling of epoch offset and subsecond precision,
- explicit overflow-aware arithmetic where needed,
- minimal API surface matching the original module responsibility.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C implementation characteristics for constant-time timestamp conversion.
  - Avoid heap allocation.
  - Use simple integer arithmetic with no unnecessary abstraction overhead.
  - Preserve correctness for edge values representable by the original C logic.

## Module Mapping

### Source Mapping

- **C source**: `gnu/stat-w32.c`
- **Rust target**: `src/module_gnu_gl_convert_16.rs`

### Function Mapping

- **C**: `_gl_convert_FILETIME_to_timespec`
  **Rust**: `fn gl_convert_filetime_to_timespec(...) -> TimespecLike`

- **C**: `_gl_convert_FILETIME_to_POSIX`
  **Rust**: `fn gl_convert_filetime_to_posix(...) -> PosixTimeParts`

If these functions are only used internally by the containing module cluster, they should remain `pub(crate)` or private rather than being exposed publicly.

## Data Model

The C analysis identifies anonymous structures only, so the Rust plan should introduce only the minimum explicit types needed to replace field-based C access.

### Structure Mapping

- **C anonymous structure representing `FILETIME` fields**
  **Rust**:
  ```rust
  struct FileTime {
      dw_low_date_time: u32,
      dw_high_date_time: u32,
  }
  ```
  Notes:
  - Mirrors the Windows split 64-bit timestamp representation.
  - Conversion should reconstruct the 64-bit tick count using `u64`.

- **C anonymous POSIX time result container**
  **Rust**:
  ```rust
  struct TimespecLike {
      tv_sec: i64,
      tv_nsec: i32,
  }
  ```
  Notes:
  - Intended only if the surrounding Rust crate does not already define an equivalent timespec-style struct.
  - If an existing crate-local type already exists, reuse it instead of introducing a parallel one.

- **C anonymous intermediate/result fields for POSIX conversion**
  **Rust**:
  ```rust
  struct PosixTimeParts {
      seconds: i64,
      nanoseconds: i32,
  }
  ```
  Notes:
  - If `_gl_convert_FILETIME_to_POSIX` is only an internal arithmetic helper, this may be replaced by a tuple or folded into the final conversion function to avoid expanding the module surface.
  - Keep the representation signed on the POSIX side to support times before the POSIX epoch if the original arithmetic permits it.

### Numeric Mapping

- `DWORD`/32-bit unsigned fields -> `u32`
- 64-bit FILETIME combined value -> `u64`
- POSIX seconds -> `i64`
- Nanoseconds -> `i32`

### Memory Management and Error Handling

- No manual memory management is needed; values are plain stack data.
- Prefer pure functions returning value types.
- If the original C code assumes all inputs are valid and does not expose failure modes, keep the Rust API non-fallible.
- If overflow checks are required during reconstruction or epoch subtraction, use explicit checked or widened arithmetic internally and document any assumptions made to preserve original behavior.

## Implementation Phases

## Phase 1: Create Rust module skeleton and type mappings

- Add `src/module_gnu_gl_convert_16.rs`.
- Define the minimal Rust representations required for the migrated code:
  - `FileTime`
  - one timespec-style result type only if not already available in the crate
- Add constants required by the conversion logic:
  - Windows-to-Unix epoch offset
  - FILETIME tick resolution to seconds/nanoseconds conversion factors
- Establish function signatures for:
  - `gl_convert_filetime_to_posix`
  - `gl_convert_filetime_to_timespec`
- Keep visibility restricted to the narrowest level compatible with current crate usage.

## Phase 2: Port conversion arithmetic from `stat-w32.c`

- Migrate `_gl_convert_FILETIME_to_POSIX` first as the arithmetic core.
- Reconstruct the 64-bit FILETIME value from high/low 32-bit parts using explicit integer operations.
- Port the epoch adjustment and subsecond conversion logic directly, preserving:
  - tick-to-second scaling,
  - nanosecond derivation,
  - sign and range behavior.
- Implement `_gl_convert_FILETIME_to_timespec` on top of the POSIX conversion helper or inline the shared arithmetic if that better matches the original flow without adding extra abstraction.
- Keep the implementation safe Rust unless direct layout compatibility with existing crate code requires `#[repr(C)]` on data types.

## Phase 3: Add correctness tests for migrated behavior

- Add unit tests in the module or crate test section covering:
  - zero FILETIME input behavior,
  - exact POSIX epoch boundary conversion,
  - subsecond conversion accuracy,
  - representative normal timestamp conversion,
  - high/low field combination correctness,
  - edge values near arithmetic boundaries used by the original logic.
- Use deterministic expected values derived from the original C arithmetic rather than introducing external test dependencies.
- Verify that `cargo test` passes on the branch with no additional runtime facilities.

## Phase 4: Integrate and finalize module migration

- Replace any remaining references to the C-side helpers in the Rust port path with the new Rust functions.
- Confirm naming and file placement remain consistent with the existing Rust project conventions.
- Remove any temporary duplicate helpers introduced during translation if both conversion functions can share a single internal arithmetic path cleanly.
- Perform a final pass for:
  - integer conversion clarity,
  - minimal visibility,
  - absence of unnecessary allocations or support code,
  - alignment with the original module-only scope.