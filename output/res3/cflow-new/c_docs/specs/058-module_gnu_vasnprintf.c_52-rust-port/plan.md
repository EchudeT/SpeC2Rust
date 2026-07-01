# Implementation Plan: module_gnu_vasnprintf.c_52

## Summary

This module ports the numeric and character-conversion helper logic currently implemented in `gnu/vasnprintf.c` into Rust, keeping scope limited to the listed functions and their direct supporting logic. The Rust implementation should preserve C behavior for bounded string-length helpers, wide-character length helpers, wide-character to multibyte conversion fallbacks, locale-aware decimal point extraction, and integer-based decimal conversion helpers.

The technical approach is to migrate the functionality into a single Rust module with small internal helper functions mirroring the C function boundaries where practical. The implementation should rely primarily on Rust standard library facilities for UTF-8/Unicode handling, checked arithmetic, and slice/string processing, while explicitly preserving edge-case behavior that in C depended on pointer traversal, manual bounds checks, and arithmetic overflow checks. Any locale-sensitive behavior not directly representable through the Rust standard library should be implemented conservatively with behavior matching the existing module’s needs rather than generalized locale infrastructure.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time behavior for string and wide-string length scans.
  - Avoid unnecessary heap allocation in helper functions except where conversion output requires owned bytes.
  - Preserve checked arithmetic semantics for multiplication and division helpers.
  - Keep decimal conversion logic efficient and deterministic, with no asymptotic regression from the C implementation.

## Module Mapping

- **C source file**
  - `gnu/vasnprintf.c`

- **Rust target module**
  - `src/module_gnu_vasnprintf_c_52.rs`

- **Function mapping**
  - `local_strnlen` -> `fn local_strnlen(...) -> usize`
  - `local_wcslen` -> `fn local_wcslen(...) -> usize`
  - `local_wcsnlen` -> `fn local_wcsnlen(...) -> usize`
  - `wctomb_fallback` -> `fn wctomb_fallback(...) -> Result<_, _>`
  - `local_wcrtomb` -> `fn local_wcrtomb(...) -> Result<_, _>`
  - `local_wctomb` -> `fn local_wctomb(...) -> Result<_, _>`
  - `decimal_point_char` -> `fn decimal_point_char() -> u8` or `char`, depending on call-site needs
  - `multiply` -> `fn multiply(...) -> Option<_>` or `Result<_, _>`
  - `divide` -> `fn divide(...) -> _`
  - `convert_to_decimal` -> `fn convert_to_decimal(...) -> _`

The Rust module should remain a direct migration target for this C file’s helper logic, not a broader formatting subsystem.

## Data Model

The analysis identifies only an anonymous data structure, so the plan should avoid inventing exported types unless the converted functions require a local representation.

### Data-structure mapping

- **C anonymous structure(s)**
  - Map to a private Rust `struct` only if a grouped return value or temporary state is required by the converted code.
  - If the anonymous structure in C is only used for local temporary aggregation, prefer direct local variables or tuples in Rust.

### Primitive and memory representation mapping

- `char *` with explicit bounds
  - Map to `&[u8]`, `&mut [u8]`, or `Vec<u8>` depending on ownership and mutability.
- NUL-terminated byte string traversal
  - Represent as byte slices with explicit scan logic; do not depend on implicit terminators unless the surrounding port already stores one.
- `wchar_t *`
  - Map according to actual semantic use:
    - If used as Unicode scalar values, prefer `&[char]`.
    - If preserving raw wide-code-unit behavior is necessary, use `&[u32]` internally with validation at conversion boundaries.
- Output buffers for multibyte conversion
  - Map to `&mut [u8]`, returning the number of bytes written or an error.
- Integer arithmetic helpers
  - Use Rust integer primitives with `checked_mul`, ordinary division, and explicit precondition handling where the C code relied on manual overflow control.

## Implementation Phases

### Phase 1: Establish module skeleton and bounded-length helpers

- Create `src/module_gnu_vasnprintf_c_52.rs`.
- Port:
  - `local_strnlen`
  - `local_wcslen`
  - `local_wcsnlen`
- Translate pointer-and-limit traversal into slice-based scanning.
- Preserve C-style stopping conditions:
  - stop at first NUL-equivalent element
  - respect explicit maximum length where applicable
- Add unit tests covering:
  - empty input
  - no terminator before bound
  - terminator at start, middle, and bound edge
  - wide-character inputs with embedded NUL values

### Phase 2: Port wide-character conversion helpers

- Port:
  - `wctomb_fallback`
  - `local_wcrtomb`
  - `local_wctomb`
- Choose a single internal representation for wide characters based on how the original logic uses `wchar_t`; prefer minimal translation logic rather than redesign.
- Implement conversion using standard library Unicode facilities where behavior matches the C helper expectations.
- For invalid scalar values or insufficient output space, return explicit Rust errors instead of relying on C sentinel values internally; only preserve sentinel-style behavior at the module boundary if required by surrounding migrated code.
- Ensure no unchecked writes to output buffers.
- Add unit tests covering:
  - ASCII characters
  - multibyte UTF-8 cases
  - invalid wide-character values
  - zero-length / insufficient destination buffer
  - fallback behavior consistency

### Phase 3: Port locale-decimal and arithmetic helpers

- Port:
  - `decimal_point_char`
  - `multiply`
  - `divide`
- Keep locale handling narrow:
  - if the C logic effectively needs a single-byte decimal separator and no standard-library locale source is available, default to the exact conservative behavior required by current formatting paths.
- Replace manual overflow detection in `multiply` with checked arithmetic.
- Preserve division semantics exactly, especially for truncation behavior and divisor assumptions.
- Add unit tests covering:
  - arithmetic overflow and non-overflow cases
  - zero and non-zero operands
  - division boundary cases
  - decimal point result expectations used by the ported formatting logic

### Phase 4: Port decimal conversion logic and finalize integration

- Port:
  - `convert_to_decimal`
- Keep the function structure close to the C implementation so arithmetic and buffer-manipulation behavior remain reviewable.
- Replace raw buffer growth/manipulation with `Vec<u8>` or mutable slices only where the original ownership model requires it.
- Ensure conversion logic composes with:
  - checked `multiply`
  - `divide`
  - decimal separator helper
- Add focused tests for:
  - small numeric inputs
  - large numeric inputs near arithmetic boundaries
  - formatting paths that depend on repeated multiply/divide steps
  - output stability against expected decimal byte sequences
- Perform final cleanup:
  - keep helper visibility private unless cross-module use is already required by the port
  - remove unused compatibility scaffolding
  - confirm `cargo test` passes for the module

## Error Handling and Memory Management Notes

- Replace C pointer arithmetic and implicit buffer trust with slice indexing and explicit capacity checks.
- Use `Option` or `Result` for internal failure paths such as overflow, invalid wide characters, and insufficient output storage.
- Avoid `unsafe` unless a direct representation constraint from the original module makes it unavoidable; if needed, isolate it to the smallest helper and document invariants locally.
- Keep allocations minimal and localized to conversion paths that inherently produce variable-length output.

## Validation Criteria

- All listed functions from `gnu/vasnprintf.c` are represented in the Rust module.
- Behavior for bounds checking, termination scanning, arithmetic overflow handling, and decimal conversion matches the original module’s operational intent.
- The implementation builds and passes `cargo test`.
- The resulting Rust code remains a direct migration of the existing helper logic without introducing unrelated abstractions or new subsystem boundaries.