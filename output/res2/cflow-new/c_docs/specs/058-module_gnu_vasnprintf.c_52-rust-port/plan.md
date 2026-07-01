# Implementation Plan

## Summary

Port `gnu/vasnprintf.c` support logic into a Rust module that preserves the existing computation flow for bounded string/wide-string length helpers, wide-character to multibyte conversion fallbacks, decimal separator lookup, and internal integer arithmetic used by decimal conversion. The Rust implementation should stay tightly scoped to the listed functions and migrate them into a single module-oriented file layout without adding new formatting features.

The technical approach is to translate C pointer- and buffer-centric logic into Rust slice- and `Vec<u8>`-based logic where possible, while keeping helper functions private unless needed by existing call sites. Length-related helpers should map to explicit bounded scans over byte or wide-character slices. Wide-character conversion helpers should model C behavior conservatively, including fallible conversion paths and explicit result reporting. Arithmetic helpers should use fixed-width integer types with checked operations where overflow semantics must be made explicit. Decimal conversion should preserve the original step ordering and output expectations, using standard-library string and numeric utilities only where they do not alter observable behavior.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the original module’s linear-time behavior for bounded length scans.
  - Avoid unnecessary allocations in helper functions except where output buffering is intrinsic.
  - Preserve predictable numeric conversion cost for decimal conversion helpers.
  - Keep conversion and arithmetic paths suitable for use in formatting code without avoidable copies.

## Module Mapping

- **C source file**: `gnu/vasnprintf.c`
- **Rust target module**: `src/module_gnu_vasnprintf_c_52.rs`

Function migration mapping:

- `local_strnlen` -> `fn local_strnlen(...)`
- `local_wcslen` -> `fn local_wcslen(...)`
- `local_wcsnlen` -> `fn local_wcsnlen(...)`
- `wctomb_fallback` -> `fn wctomb_fallback(...) -> Result<..., ...>`
- `local_wcrtomb` -> `fn local_wcrtomb(...) -> Result<usize, ...>`
- `local_wctomb` -> `fn local_wctomb(...) -> Result<usize, ...>`
- `decimal_point_char` -> `fn decimal_point_char() -> u8` or `char`, depending on existing call expectations
- `multiply` -> `fn multiply(...) -> Result<..., ...>` or checked in-place helper
- `divide` -> `fn divide(...) -> ...`
- `convert_to_decimal` -> `fn convert_to_decimal(...) -> Result<..., ...>`

Recommended file placement is a single Rust source file for this migration unit. Do not split helpers into additional modules unless required by existing crate structure.

## Data Model

No named C structs are listed for this module. The implementation should therefore map only implicit C data patterns into Rust-native representations:

- **C anonymous/internal buffer state** -> local Rust variables, slices, and `Vec<u8>`
- **C `char *` / byte buffer** -> `&[u8]`, `&mut [u8]`, or `Vec<u8>` depending on ownership and mutability
- **C wide-character sequences (`wchar_t *`)** -> platform-conscious internal representation, preferably explicit integer code units such as `u32` for migrated logic, or slices over the project’s chosen wide-char equivalent if already established
- **C size/count types (`size_t`)** -> `usize`
- **C status/error sentinel returns** -> `Result<T, E>` where failure is meaningful; plain `Option<T>` only for simple absence cases

Error model decisions:

- Conversion helpers should return explicit errors for invalid code points, insufficient output capacity, or conversion failure instead of encoding C sentinel values directly.
- Arithmetic helpers should use checked math if overflow must be surfaced; if the original logic depends on bounded manual carry handling, represent that directly rather than relying on wrapping arithmetic.
- Functions that are purely internal may still use narrow custom error enums scoped to this module to avoid broad crate-wide abstractions.

## Implementation Phases

### Phase 1: Create module scaffold and migrate bounded-length helpers

- Add `src/module_gnu_vasnprintf_c_52.rs`.
- Translate:
  - `local_strnlen`
  - `local_wcslen`
  - `local_wcsnlen`
- Express scans using explicit loops over slices to preserve C termination and bound behavior.
- Decide and document the Rust representation for wide-character inputs before implementing the wide-string helpers.
- Add focused unit tests for:
  - zero-length inputs
  - early terminators
  - exact-bound termination
  - no-terminator-within-bound behavior

### Phase 2: Migrate character conversion helpers

- Translate:
  - `wctomb_fallback`
  - `local_wcrtomb`
  - `local_wctomb`
- Preserve the original fallback ordering and byte-count behavior.
- Replace raw output-pointer writes with mutable byte slices or temporary fixed buffers.
- Model conversion failure with a local error enum.
- Keep locale-sensitive behavior limited to what is already present in the C logic; do not introduce broader Unicode or locale facilities beyond the standard library behavior needed to match the original implementation.
- Add unit tests covering:
  - ASCII/basic single-byte cases
  - multibyte output cases as applicable
  - invalid wide-character/code-point inputs
  - insufficient destination capacity if the C code distinguishes this case

### Phase 3: Migrate decimal separator and arithmetic helpers

- Translate:
  - `decimal_point_char`
  - `multiply`
  - `divide`
- Keep these helpers private unless existing Rust call paths require wider visibility.
- For `decimal_point_char`, use standard-library-compatible lookup logic only if it matches the existing behavior; otherwise keep the function narrowly implemented to the required single-character result.
- For `multiply` and `divide`, preserve the original integer-width assumptions and carry/remainder behavior.
- Add unit tests for:
  - common arithmetic cases
  - boundary values
  - overflow-sensitive paths if applicable
  - stable decimal-point result behavior

### Phase 4: Migrate decimal conversion and finalize integration

- Translate `convert_to_decimal` after helper functions are in place.
- Preserve the original sequencing of arithmetic and buffer writes rather than replacing it with higher-level formatting routines that may change results.
- Use `Vec<u8>` or mutable slices for output accumulation according to the original ownership pattern.
- Integrate with any existing crate-internal call sites using the minimal required visibility.
- Add end-to-end tests for representative decimal conversion scenarios, including:
  - small values
  - large values
  - edge cases that exercise `multiply` and `divide`
  - output formatting consistency with expected byte content