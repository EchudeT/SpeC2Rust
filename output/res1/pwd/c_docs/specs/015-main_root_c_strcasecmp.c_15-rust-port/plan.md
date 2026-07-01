# Implementation Plan

## Summary

Port the C module `c-strcasecmp.c` into a focused Rust implementation that preserves the original module boundary and behavior of `c_strcasecmp` without adding new capabilities. The Rust version should provide a single case-insensitive string comparison routine aligned with the C logic, including C-style byte-wise handling and return-value ordering semantics.

The implementation approach should stay close to the source algorithm: compare input strings byte by byte, normalize ASCII letter case during comparison, stop at the first difference or terminating boundary, and return an integer ordering result compatible with the C function’s expectations. The migration should prefer safe Rust where possible, with careful handling of string inputs so that null-termination assumptions and byte-level comparison behavior are made explicit in the Rust API used internally by the port.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time comparison behavior proportional to the shorter input length plus termination handling.
  - Avoid unnecessary allocation or Unicode case-folding overhead.
  - Keep the implementation byte-oriented and ASCII-focused to match the C routine closely.

## Module Mapping

| C Source File | C Function | Rust Target File | Rust Item |
|---|---|---|---|
| `c-strcasecmp.c` | `c_strcasecmp` | `src/c_strcasecmp.rs` | `pub(crate) fn c_strcasecmp(...) -> i32` |

If this module is part of a binary crate root today, expose it from the existing crate entry point using a minimal `mod c_strcasecmp;` declaration and only the visibility needed by current callers.

## Data Model

This module has no module-specific structs defined in the provided input.

### C-to-Rust Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char *` / `const char *` input strings | `&[u8]` or `&CStr` internally, depending on caller context | Prefer the narrowest representation that preserves byte-wise comparison and termination expectations. |
| `int` return value | `i32` | Maintain negative / zero / positive ordering semantics. |

### Data Handling Notes

- The implementation should treat inputs as byte sequences rather than `String` / `&str` if the original C function operates on C strings.
- Case normalization should be limited to ASCII semantics unless the analyzed source explicitly shows broader locale-aware behavior.
- Memory ownership should remain with the caller; the function should borrow inputs and perform no heap allocation.

## Implementation Phases

### Phase 1: Establish module skeleton and API mapping

- Create `src/c_strcasecmp.rs`.
- Add the Rust function corresponding to `c_strcasecmp`.
- Choose the function signature based on how the surrounding port passes C-like strings:
  - prefer borrowed byte-oriented input;
  - use `&CStr` only if existing migrated callers already operate on C string wrappers.
- Wire the module into the current crate structure with the smallest necessary visibility surface.
- Document the intended comparison contract in code comments: ASCII-only case-insensitive comparison, byte-wise ordering, C-compatible integer result convention.

### Phase 2: Port comparison logic faithfully

- Translate the C comparison loop directly into Rust.
- Implement ASCII case folding using standard-library byte helpers or explicit normalization logic.
- Ensure termination behavior matches C expectations:
  - comparison stops on first differing normalized byte;
  - equal strings return `0`;
  - prefix/termination cases produce correctly signed ordering results.
- Avoid allocations, temporary owned strings, or Unicode-aware transformations.
- Keep arithmetic and byte conversion explicit so signedness differences between C `char` and Rust byte types do not change behavior unexpectedly.

### Phase 3: Validate behavior with focused tests

- Add unit tests in the module or crate test layout covering:
  - equal strings with identical case;
  - equal strings with differing ASCII case;
  - ordering when strings differ by letters;
  - ordering when one string is a prefix of the other;
  - empty-string handling;
  - non-alphabetic byte preservation during comparison.
- Include tests that confirm return sign behavior rather than overfitting to an exact nonzero magnitude unless the original C implementation requires exact subtraction semantics.
- Run `cargo test` and adjust the implementation only where required to preserve the original module behavior.

### Phase 4: Final integration and cleanup

- Replace or connect existing call sites to the Rust implementation within the current migration branch.
- Remove any unnecessary transitional code introduced during porting.
- Confirm the module remains narrowly scoped to the original file/function responsibility.
- Perform a final pass for:
  - safe borrowing and lifetime correctness;
  - no hidden allocations;
  - no expansion beyond the original `c_strcasecmp` functionality.