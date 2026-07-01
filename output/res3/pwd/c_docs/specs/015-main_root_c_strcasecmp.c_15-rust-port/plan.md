# Implementation Plan

## Summary

Port the C module `c-strcasecmp.c` into a focused Rust module that preserves the existing behavior of `c_strcasecmp` without adding new API surface or auxiliary capabilities. The Rust implementation should mirror the original responsibility: compare two byte-oriented strings in a C-compatible, ASCII case-insensitive manner.

The technical approach is to implement the comparison using Rust standard library byte processing rather than locale-aware Unicode string comparison. This keeps behavior aligned with typical C `strcasecmp`-style semantics for plain C strings and avoids introducing Unicode or allocation-heavy transformations. The implementation should prefer borrowed byte/string views and return an integer ordering result compatible with the original function’s comparison contract.

## Technical Context

- **Language/Version**: Rust 1.75 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Linear-time comparison over input length
  - No heap allocation during comparison
  - Early exit on first differing byte
  - Behavior stable for ASCII-oriented case folding expected from the source module

## Module Mapping

| C Source File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `c-strcasecmp.c` | `c_strcasecmp` | `src/c_strcasecmp.rs` | `pub(crate) fn c_strcasecmp(...) -> i32` |

### Rust Project File Placement

| Rust File | Purpose |
|---|---|
| `src/c_strcasecmp.rs` | Direct port of the comparison logic from `c-strcasecmp.c` |
| `src/lib.rs` or `src/main.rs` | Module declaration and internal exposure, depending on current crate layout |

## Data Model

This module does not define custom data structures in the analyzed C input.

### C-to-Rust Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| NUL-terminated string input | `&[u8]` or `&str` at call boundary | Prefer the narrowest type already used by the surrounding crate; use byte-wise logic internally |
| integer comparison result | `i32` | Preserve negative / zero / positive contract |
| raw pointer traversal in C | slice iteration / indexed byte access | Eliminates manual memory management while preserving sequential comparison behavior |

### Memory Management Notes

- No owned storage is required for the ported logic.
- Avoid temporary lowercase copies; compare bytes on the fly.
- Prefer safe slice access and iteration.
- If integration requires handling C-style terminators explicitly, stop comparison at `0` byte rather than scanning past bounds.

### Error Handling Notes

- The original function is a comparison routine, not an error-reporting API.
- Keep the Rust function non-fallible if inputs are already valid Rust references.
- If the surrounding crate requires direct emulation of C string inputs, constrain validation at the boundary and keep `c_strcasecmp` itself focused on comparison logic.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Alignment

- Create `src/c_strcasecmp.rs`.
- Add the module declaration in the crate root (`lib.rs` or `main.rs`) following the existing project structure.
- Choose the Rust function signature based on the closest existing crate conventions:
  - prefer borrowed inputs,
  - preserve an `i32` return value,
  - avoid introducing wrappers or new public types.
- Document the function briefly to indicate ASCII-oriented, C-style case-insensitive comparison behavior.

### Phase 2: Port Core Comparison Logic

- Translate the byte-by-byte comparison from `c-strcasecmp.c` into safe Rust.
- Implement ASCII case folding inline for each compared byte.
- Preserve C comparison semantics:
  - return `0` when equal,
  - return negative or positive value based on first differing folded byte,
  - terminate appropriately when either input ends.
- Avoid locale-dependent behavior and avoid Unicode case conversion APIs.
- Ensure no allocation and no unnecessary copying.

### Phase 3: Add Targeted Tests

- Add unit tests covering:
  - exact equality,
  - ASCII case-insensitive equality,
  - first differing character ordering,
  - prefix vs longer string ordering,
  - empty input handling,
  - non-alphabetic byte preservation in comparisons.
- Keep tests close to the module in idiomatic Rust style.
- Validate that the function returns stable sign behavior consistent with the C implementation’s intent.

### Phase 4: Integration Verification and Cleanup

- Confirm the module is wired into the current crate without adding extra abstraction layers.
- Run `cargo test` and fix any signature or visibility mismatches with surrounding code.
- Perform a final review for:
  - safe memory use,
  - no allocation in the hot path,
  - no accidental Unicode/locale behavior,
  - close correspondence between original C function flow and Rust implementation.