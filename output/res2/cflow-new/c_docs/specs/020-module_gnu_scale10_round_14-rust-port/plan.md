# Implementation Plan

## Summary

This module ports the decimal scaling-and-rounding helpers currently embedded in `gnu/vasnprintf.c` into Rust, limited to the existing function surface:

- `scale10_round_decimal_decoded`
- `scale10_round_decimal_long_double`
- `scale10_round_decimal_double`

The Rust implementation should preserve the current computational behavior and rounding flow without adding new formatting features or broader numeric abstractions. The technical approach is to translate the existing decimal-digit manipulation logic into a small Rust module that uses standard-library numeric types and slice-based digit buffers. The implementation should keep the original control flow recognizable so that correctness can be checked function-by-function against the C source.

Because one C helper operates on a decoded decimal representation and two helpers are type-specific entry points for floating-point values, the Rust port should keep the same layering:

1. a core internal routine operating on a decoded decimal form,
2. a `f64` entry point,
3. a long-double replacement entry point mapped to the closest Rust-supported representation.

Special care is required for:
- preserving rounding edge cases,
- handling carry propagation across decimal digits,
- avoiding buffer overrun patterns that are possible in C,
- representing any anonymous decoded state with explicit Rust fields,
- documenting the `long double` mapping limitation.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Match the C implementation’s asymptotic behavior for digit scanning and carry propagation.
  - Avoid heap allocation beyond what is already required by digit-buffer ownership in the calling path.
  - Keep the core rounding path branch structure simple and close to the original implementation.
  - Prefer in-place mutation of digit buffers/slices where the C code mutates existing storage.

## Module Mapping

### Source File Mapping

- **C source:** `gnu/vasnprintf.c`
- **Rust target:** `src/module_gnu_scale10_round_14.rs`

This Rust file should contain only the migrated logic for the three listed functions and any minimal private helper types/functions required to support them.

### Function Mapping

- `scale10_round_decimal_decoded`
  - **Rust:** `fn scale10_round_decimal_decoded(...) -> ...`
  - Role: internal core routine operating on decoded decimal digits/exponent/sign state.

- `scale10_round_decimal_long_double`
  - **Rust:** `fn scale10_round_decimal_long_double(...) -> ...`
  - Role: type-specific adapter for C `long double`.
  - Note: since Rust has no portable `long double`, map this implementation to the same internal path as `f64` unless the existing surrounding port already defines a project-local extended-precision substitute. Do not introduce a new abstraction solely for this module.

- `scale10_round_decimal_double`
  - **Rust:** `fn scale10_round_decimal_double(...) -> ...`
  - Role: type-specific adapter for `f64`.

### Visibility

- Keep these functions private to the module unless the surrounding Rust port requires cross-module access.
- Expose only what is needed to replace the call sites migrated from `gnu/vasnprintf.c`.

## Data Model

The C analysis reports an anonymous data structure. For the Rust port, replace any anonymous decoded-decimal carrier with a named internal struct whose fields directly mirror the values the C code reads or writes.

### Data Structure Mapping

- **C:** anonymous decoded decimal structure
- **Rust:** internal struct, for example:
  ```rust
  struct DecodedDecimal<'a> {
      negative: bool,
      exponent10: i32,
      digits: &'a mut [u8],
      digits_len: usize,
  }
  ```

### Mapping Notes

- **Digit storage**
  - If the C code uses a mutable character buffer, represent it as `&mut [u8]` when operating in place.
  - If ownership is required by the surrounding caller path, use `Vec<u8>` at the boundary and pass `as_mut_slice()` internally.
  - Keep digits as ASCII bytes or numeric digit values according to the original algorithm; do not redesign the representation unless required for safe indexing.

- **Exponent**
  - Map C integer exponent fields to `i32` unless the source clearly requires a wider type.
  - Preserve signed arithmetic checks explicitly with `checked_*` only where overflow is plausible from translated logic; do not add broad defensive frameworks.

- **Length and indexes**
  - Replace C `size_t` with `usize`.
  - Convert pointer arithmetic into explicit slice indexing and bounds-checked loops.

- **Floating-point inputs**
  - C `double` → Rust `f64`
  - C `long double` → Rust `f64` for this port unless an existing project type already covers extended precision

### Memory Management

- Eliminate raw pointer mutation in favor of slice-based updates.
- Keep buffer ownership with the caller where possible.
- Avoid temporary allocations in the core rounding routine unless needed to express carry expansion safely.

### Error Handling

- If the C functions are total helpers with no explicit error channel, keep them as total Rust functions returning plain values.
- If any C path signals failure through return codes or output invariants, map that to `Option` or `Result` only at the exact migrated boundary.
- Do not add new error categories beyond those already implied by the source behavior.

## Implementation Phases

## Phase 1: Extract and Define the Rust Module Surface

- Create `src/module_gnu_scale10_round_14.rs`.
- Identify the exact local variables and anonymous decoded state used by the three C functions in `gnu/vasnprintf.c`.
- Define the minimal Rust internal struct(s) needed to represent that decoded state.
- Declare Rust equivalents for:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_double`
  - `scale10_round_decimal_long_double`
- Establish the chosen `long double` mapping in code comments and keep it local to this module.
- Add skeletal unit tests for compilation and basic call flow only.

## Phase 2: Port the Core Decoded-Decimal Rounding Logic

- Translate `scale10_round_decimal_decoded` first, preserving:
  - digit traversal order,
  - rounding-threshold decisions,
  - tie-handling logic,
  - carry propagation,
  - exponent adjustment when carry creates a new leading digit.
- Replace pointer arithmetic with slice indexing.
- Keep the algorithm in-place where the C code mutates an existing buffer.
- Validate edge conditions:
  - zero/empty digit ranges if present in C,
  - rounding that does not change the prefix,
  - rounding that turns trailing 9s into carry chains,
  - exponent changes after full carry propagation.
- Add focused `cargo test` unit tests for the decoded helper using manually constructed digit buffers.

## Phase 3: Port the `double` and `long double` Entry Points

- Translate `scale10_round_decimal_double` using `f64`.
- Translate `scale10_round_decimal_long_double` using the same Rust numeric path unless an existing project-local type already replaces C `long double`.
- Preserve the original order of:
  - floating-point decomposition/normalization,
  - decoded representation setup,
  - invocation of the core decoded helper,
  - write-back of adjusted decimal state.
- Keep function signatures and outputs aligned with the migrated call sites from `gnu/vasnprintf.c`.
- Add test cases covering representative floating-point inputs that exercise:
  - no rounding change,
  - exact half-way rounding,
  - carry into a new decimal position,
  - negative values if sign participates in formatting state.

## Phase 4: Integrate and Verify Against the Original Module Behavior

- Wire the new Rust module into the Rust port location corresponding to `gnu/vasnprintf.c`.
- Replace only the existing call paths that depend on these three functions.
- Remove any temporary scaffolding created during translation.
- Expand tests to compare stable expected decimal-state outcomes for known cases derived from the C behavior.
- Run `cargo test` and resolve any mismatches by tightening the translation rather than redesigning the algorithm.