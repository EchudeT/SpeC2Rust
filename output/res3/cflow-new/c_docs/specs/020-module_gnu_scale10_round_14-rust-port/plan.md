# Implementation Plan: module_gnu_scale10_round_14

## Summary

Port the decimal scale-and-round logic from `gnu/vasnprintf.c` into a focused Rust module that preserves the existing function boundaries:

- `scale10_round_decimal_decoded`
- `scale10_round_decimal_long_double`
- `scale10_round_decimal_double`

The Rust implementation should mirror the current C behavior closely, especially around numeric rounding, decimal scaling by powers of ten, and formatting-adjacent edge cases. The preferred technical approach is a direct translation of the existing routines into safe Rust where possible, using small internal helper types only when needed to represent intermediate decoded decimal state that is currently implicit or anonymous in C.

The implementation should remain narrowly scoped to this module migration. It should not introduce broader formatting abstractions or reorganize unrelated `vasnprintf` logic. The main engineering concern is preserving numerical behavior while replacing C pointer-based and buffer-oriented manipulation with Rust slices, owned values, and explicit return types.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by the available input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the practical runtime characteristics of the C routines for typical formatting paths
  - Avoid unnecessary heap allocation in hot numeric paths where stack-based buffers or in-place mutation are sufficient
  - Preserve predictable rounding behavior without introducing heavy arbitrary-precision dependencies

## Module Mapping

### Source File Mapping

- **C source**: `gnu/vasnprintf.c`
- **Rust target**: `src/module_gnu_scale10_round_14.rs`

If the destination crate already groups migrated code by source lineage, the module should still remain a single Rust file corresponding to this extracted functionality rather than splitting into additional submodules.

### Function Mapping

- `scale10_round_decimal_decoded`
  - Port as an internal Rust function first, since it appears to represent the core rounding/scaling routine over a decoded decimal form
  - Visibility should be restricted to the module unless existing crate integration requires wider access

- `scale10_round_decimal_long_double`
  - Port as a Rust function operating on the closest practical Rust representation available in the current codebase
  - Since Rust has no native `long double`, implementation should be based on the behavior encoded in the C routine rather than on a distinct Rust primitive
  - Keep the API constrained to the migrated call sites

- `scale10_round_decimal_double`
  - Port as a Rust function using `f64`
  - Maintain the original call order and helper usage relative to the decoded routine

### Integration Boundary

This plan assumes the port is for internal use within the Rust project and not for C ABI compatibility. The Rust module should expose only the functions needed by the migrated equivalent of the surrounding formatting logic.

## Data Model

### C to Rust Data Structure Mapping

The analysis lists only an `anonymous` data structure. For planning purposes:

- **C anonymous struct used for decoded decimal state**
  - **Rust mapping**: a private named `struct` in `src/module_gnu_scale10_round_14.rs`
  - Suggested shape:
    - sign indicator
    - decimal digits buffer or slice-backed storage
    - decimal exponent / scale metadata
    - length or active digit range
  - Final field layout should be derived directly from the original local struct/member usage in `gnu/vasnprintf.c`

### Numeric Type Mapping

- `double` -> `f64`
- `long double` -> no direct Rust equivalent
  - Map according to how the C code actually consumes it:
    - If the original logic only needs extracted decimal characteristics already handled by local computation, port that logic directly
    - If the surrounding port already defines a compatibility type, reuse it instead of inventing a new abstraction
- Integer counters / exponents
  - Prefer `i32`, `u32`, `isize`, or `usize` based on indexing vs arithmetic role
  - Use checked conversions where C mixed signed and unsigned arithmetic

### Memory Management Notes

- Replace mutable raw buffers with:
  - `&mut [u8]` for byte-oriented digit storage, or
  - `Vec<u8>` only if dynamic resizing is required by the original algorithm
- Avoid unsafe code unless the C logic cannot be represented cleanly with slices and indexed mutation
- Keep ownership local to the module; return plain values or small structs rather than exposing internal buffers

### Error Handling Notes

These functions are likely computation-oriented rather than fallible in the C sense. Prefer:

- direct return values when the original routine assumes valid inputs
- `Result<_, _>` only if the Rust port must make formerly implicit preconditions explicit, such as impossible index/state combinations uncovered during translation

Do not add generalized error frameworks.

## Implementation Phases

## Phase 1: Extract and map the existing C logic

- Isolate the relevant code paths in `gnu/vasnprintf.c` for the three listed functions
- Identify the exact anonymous decoded-decimal data layout and all helper variables that participate in rounding and power-of-ten scaling
- Determine the real dependency order among:
  - `scale10_round_decimal_decoded`
  - `scale10_round_decimal_long_double`
  - `scale10_round_decimal_double`
- Create `src/module_gnu_scale10_round_14.rs`
- Define the minimal private Rust struct(s) and type aliases needed to represent the current C state without broad redesign

## Phase 2: Port the core decoded-decimal routine

- Implement the Rust version of `scale10_round_decimal_decoded` first
- Translate buffer mutation and carry/round propagation logic using indexed slice operations
- Preserve the C routine's branching structure where numerical edge behavior depends on operation order
- Replace implicit C assumptions with debug assertions or narrow checked conversions where appropriate
- Add unit tests for decoded-state transformations derived from the C behavior, especially:
  - carry propagation through trailing 9s
  - decimal exponent adjustment
  - boundary digit truncation and rounding decisions

## Phase 3: Port floating-point entry points

- Implement `scale10_round_decimal_double` using `f64`
- Implement `scale10_round_decimal_long_double` by reproducing the C routine's logic as closely as possible within the available Rust numeric representation and surrounding project constraints
- Reuse the decoded-decimal core instead of duplicating rounding logic
- Validate that both entry points produce the same digit/exponent outcomes expected from the original code for representative normal and edge inputs

## Phase 4: Integration cleanup and regression tests

- Wire the module into the existing Rust port at the same call sites served by the C code segment
- Remove translation-time placeholders and confirm function visibility is no broader than necessary
- Add regression tests covering:
  - values near rounding boundaries
  - zero and sign handling
  - very small and very large decimal scaling cases within the original routine's intended range
- Run `cargo test` and resolve any mismatches by aligning operation ordering and integer conversion behavior with the C implementation rather than by redesigning the API