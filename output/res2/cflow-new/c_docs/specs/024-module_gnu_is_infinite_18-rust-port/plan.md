# Implementation Plan: module_gnu_is_infinite_18

## Summary

Port the floating-point classification helpers from `gnu/vasnprintf.c` into a small Rust module that preserves the original decision logic and call boundaries without expanding scope. The Rust implementation should provide direct equivalents for:

- `is_infinite_or_zero`
- `is_infinite_or_zerol`

The technical approach is to map both functions onto Rust floating-point primitives and standard-library classification methods. Since the source analysis only identifies these helper functions and no stable named C structs, the port should remain function-focused and minimal. The implementation should stay close to the existing behavior by using `f64` and `f64`/`f32`-family standard classification operations as appropriate, with careful review of how the C `long double` use site should map in Rust.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time floating-point classification with no heap allocation
  - Zero-copy, value-based function interfaces
  - Behavior comparable to C helper logic for zero and infinity detection
  - No additional abstraction layers beyond the migrated functions

## Module Mapping

### Source to Destination

- `gnu/vasnprintf.c`
  - C function `is_infinite_or_zero`
    → Rust function `is_infinite_or_zero`
  - C function `is_infinite_or_zerol`
    → Rust function `is_infinite_or_zerol`

### Rust File Layout

Use a restrained project layout under standard Rust conventions:

- `src/module_gnu_is_infinite_18.rs`
  - Contains both migrated helper functions
- `src/lib.rs`
  - Exposes the module if the crate is library-oriented

If the current crate already has an established module tree, place the functions in the nearest existing equivalent location rather than introducing extra layers.

## Data Model

The analysis identifies only an anonymous data structure and no explicit module-owned struct requiring migration.

### C to Rust Type Mapping

- anonymous / unnamed local C data
  → retain as local Rust variables with primitive types
- `double`
  → `f64`
- `long double`
  → `f64` in Rust unless a verified calling context requires a different representation

### Notes

- Rust has no native `long double` equivalent. Since this plan must not introduce FFI or external numeric emulation without evidence, `is_infinite_or_zerol` should initially be implemented using `f64` while documenting that this is the direct practical mapping for a pure Rust port.
- No heap-managed data structures are required.
- No ownership-complex state is expected because the migrated logic is pure and input-only.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Mapping

- Create `src/module_gnu_is_infinite_18.rs`.
- Add Rust equivalents for the two identified C functions.
- Choose primitive parameter and return types based on the original usage pattern:
  - floating-point input by value
  - boolean-like result as `bool`
- Export functions through `src/lib.rs` only if needed by the existing crate structure.
- Keep naming close to the C source to preserve traceability during review.

### Phase 2: Logic Port and Numeric Semantics Review

- Translate the original zero-or-infinity checks using Rust standard-library methods:
  - zero detection via direct comparison to `0.0`
  - infinity detection via `is_infinite()`
- Verify behavior for:
  - positive zero
  - negative zero
  - positive infinity
  - negative infinity
  - finite nonzero values
  - NaN
- Ensure no accidental behavior changes from C macro or classifier assumptions.
- Keep implementation allocation-free and panic-free.

### Phase 3: Tests and Edge-Case Validation

- Add unit tests covering both functions with representative floating-point inputs.
- Validate that the functions return:
  - `true` for `0.0`, `-0.0`, `f64::INFINITY`, `f64::NEG_INFINITY`
  - `false` for ordinary finite nonzero values
  - `false` for `NaN` unless the original C logic clearly treated it otherwise
- Run `cargo test` and confirm deterministic behavior across supported targets.

### Phase 4: Final Integration and Cleanup

- Confirm the functions are placed in the correct crate module path and are reachable by the migrated call sites.
- Remove any unnecessary helper code introduced during porting.
- Add concise rustdoc or inline comments only where they clarify C-to-Rust type decisions, especially the `long double` to `f64` mapping.
- Perform a final pass for idiomatic but minimal Rust, avoiding extra wrappers or utility modules.