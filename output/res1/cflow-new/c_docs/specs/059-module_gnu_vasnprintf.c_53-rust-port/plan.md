# Implementation Plan: module_gnu_vasnprintf.c_53

## Summary

This module migration covers the floating-point helper logic currently implemented in `gnu/vasnprintf.c`, specifically the functions:

- `decode_long_double`
- `decode_double`
- `floorlog10l`
- `floorlog10`
- `is_borderline`

The Rust implementation should preserve the existing computation flow and edge-case behavior used by the surrounding formatting code, while translating low-level C floating-point handling into safe Rust where possible. The approach is to migrate these functions into a focused Rust module with signatures and internal helper behavior aligned to the original call patterns, avoiding any expansion beyond the existing responsibilities.

The implementation should prefer Rust primitive floating-point types (`f64` and platform-supported extended handling only where directly representable), explicit helper enums/structs for decoded state, and standard-library numeric operations. Any C patterns relying on pointer outputs, mutable shared state, or bit-level reinterpretation should be rewritten into explicit return values and narrow internal helpers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep floating-point helper execution close to the original C behavior for formatting-related hot paths.
  - Avoid unnecessary heap allocation.
  - Use stack-based return values and primitive numeric operations.
  - Preserve predictable control flow for special-case handling (`NaN`, infinities, boundary rounding conditions).

## Module Mapping

### C to Rust File Mapping

- `gnu/vasnprintf.c`
  - migrate relevant floating-point helper logic into:
    - `src/module_gnu_vasnprintf_c_53.rs`

If the target crate already centralizes migrated modules under a different existing file layout, this module should still remain a single Rust source file corresponding only to this C extraction, without introducing extra submodules unless already required by the crate structure.

### Function Mapping

- `decode_long_double`
  - migrate to `decode_long_double(...)` in Rust
  - if Rust target behavior cannot represent C `long double` distinctly on the platform, document and implement using the closest supported internal representation consistent with the rest of the port

- `decode_double`
  - migrate to `decode_double(...)`

- `floorlog10l`
  - migrate to `floorlog10l(...)`

- `floorlog10`
  - migrate to `floorlog10(...)`

- `is_borderline`
  - migrate to `is_borderline(...)`

### API Direction

Because these functions are internal helpers from a C formatting module, they should remain crate-private or module-private unless an existing Rust call graph requires wider visibility. Do not promote them to public API unless already necessary for compilation of the migrated formatting path.

## Data Model

The C analysis reports only an anonymous data structure. The Rust mapping should therefore infer the minimum internal representation required by the decoded floating-point logic.

### Data-Structure Mapping

- **C anonymous struct used for decoded floating-point state**
  - **Rust**: a private named struct, for example:
    - `DecodedDouble`
    - or a single shared internal struct if fields align for both decode paths

Recommended shape:
- sign information
- exponent information
- mantissa/significand information
- classification flags for zero / finite / infinity / NaN as needed by the original logic

Example mapping approach:

| C Construct | Rust Mapping |
| --- | --- |
| anonymous struct for decoded fields | private `struct DecodedFloat { ... }` |
| integer sign/exponent fields | `i32`, `i16`, `u32`, `u64`, or `i64` as required by bit width |
| mantissa/significand words | `u64` or split integer fields matching original bit use |
| output parameters | return struct or tuple |
| special-value flags | `enum FloatClass` or boolean fields |

### Representation Guidance

- Prefer a single small internal struct over multiple abstractions.
- Use `f64` directly for `decode_double`.
- For `decode_long_double`, use a separate path only if the original logic materially differs and the target Rust environment has a concrete equivalent to preserve; otherwise keep a clearly documented approximation layer tied to surrounding migrated behavior.
- If bit extraction is required, use standard-library bit conversions such as `to_bits()` for `f64`.
- Avoid `unsafe` unless there is no safe standard-library equivalent.

## Implementation Phases

### Phase 1: Establish module skeleton and decoded-state mapping

- Create `src/module_gnu_vasnprintf_c_53.rs`.
- Identify the exact local C logic boundaries for the five target functions within `gnu/vasnprintf.c`.
- Define the minimum private Rust data structure needed to replace the anonymous decoded-state representation.
- Translate any C macros or constant expressions directly used by these functions into Rust `const` items local to this module.
- Decide final Rust signatures based on current C call sites:
  - convert pointer-out parameters into returned structs/tuples
  - keep numeric widths explicit

**Exit criteria**:
- Rust module file exists.
- All constants and internal decoded-state types needed by the five functions are defined.
- Function signatures are fixed to support direct migration of existing call patterns.

### Phase 2: Port decoding helpers and numeric base-10 exponent helpers

- Implement `decode_double` using safe Rust bit extraction and explicit special-case classification.
- Implement `decode_long_double` with the narrowest viable representation matching existing usage in the project:
  - either a dedicated decode path if required by source behavior
  - or a documented compatibility implementation if no true `long double` distinction exists in Rust target assumptions
- Implement `floorlog10` and `floorlog10l` by preserving the original arithmetic strategy and correction logic rather than replacing it with broader formatting utilities.
- Keep all intermediate computations explicit to avoid accidental behavior drift from implicit C conversions.

**Exit criteria**:
- Four functions compile.
- Unit tests cover normal, zero, subnormal, infinity, and NaN-adjacent decode behavior where applicable.
- Base-10 logarithm floor helpers match expected outputs for representative boundary values.

### Phase 3: Port borderline detection and integrate with surrounding formatting path

- Implement `is_borderline` using the same threshold and comparison behavior as the C logic.
- Review interactions among decode helpers and logarithm helpers to ensure the migrated numeric assumptions remain consistent.
- Connect the Rust module into the existing crate module tree and replace any pending internal references to the C-derived implementation.
- Keep visibility restricted to the minimum needed by the formatting code that consumes these helpers.

**Exit criteria**:
- All five functions compile and are referenced from the intended formatting path.
- Borderline rounding-related tests pass for values near decimal conversion boundaries.
- No unnecessary public exports are introduced.

### Phase 4: Validation and cleanup

- Add focused `cargo test` coverage for:
  - exact powers of ten
  - values immediately above and below powers of ten
  - positive and negative finite values
  - `0.0`, `-0.0`
  - infinities and `NaN`
  - borderline cases exercised by `is_borderline`
- Compare Rust outputs against expected behavior derived from the current C implementation for the migrated helper scope.
- Remove dead translation artifacts such as unused temporary constants or C-style helper remnants once behavior is validated.

**Exit criteria**:
- `cargo test` passes.
- The module contains only the migrated helper logic required for this C extraction.
- Memory handling is fully expressed through Rust ownership/returns with no unmanaged allocation.

## Memory Management and Error Handling Notes

- These functions should not require heap allocation; use plain values and small stack-resident structs.
- Replace C pointer-based output mutation with returned values to make state transfer explicit.
- For functions that cannot fail under the original C contract, keep direct return types rather than introducing unnecessary `Result`.
- If any impossible input state appears during translation due to representation mismatch, handle it with tightly scoped assertions or internal normalization, not expanded recovery flows.
- Preserve special floating-point classifications explicitly rather than relying on implicit downstream assumptions.

## Deliverables

- `src/module_gnu_vasnprintf_c_53.rs`
- tests integrated into the crate's normal Rust test layout
- migrated implementations of:
  - `decode_long_double`
  - `decode_double`
  - `floorlog10l`
  - `floorlog10`
  - `is_borderline`