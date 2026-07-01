# Implementation Plan: module_src_parseopt_optset.c_12

## Summary

This module ports the numeric option-parsing logic from `src/parseopt/optset.c` into Rust, limited to the existing responsibilities represented by:

- `get_signed_int`
- `get_unsigned_int`

The Rust implementation should preserve the current parsing behavior and range-checking semantics while replacing C-style pointer/error conventions with explicit Rust return types. The preferred approach is to migrate the logic into a focused Rust module that:

- accepts string input as `&str`
- parses signed and unsigned integer values using standard-library facilities
- performs explicit validation for invalid syntax, sign handling, overflow, and bounds behavior as required by the original functions
- returns typed `Result` values instead of implicit C error signaling

The implementation should stay narrow: port only the current file-local behavior needed for these functions, without introducing new parser layers or broader option-system redesign.

## Technical Context

- **Language/Version**: Rust 1.76 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear-time parsing relative to input length
  - Avoid unnecessary allocations during parsing
  - Keep conversions zero-copy at the string-slice level
  - Preserve behavior comparable to the C implementation for normal option-processing workloads

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/optset.c`
  → `src/parseopt/optset.rs`

If the Rust project already exposes parseopt functionality through a module tree, this file should be registered through the existing `mod` declarations only, without adding unrelated modules.

### Function Mapping

- `get_signed_int`
  → `pub(crate)` or private Rust function in `src/parseopt/optset.rs`
- `get_unsigned_int`
  → `pub(crate)` or private Rust function in `src/parseopt/optset.rs`

Visibility should be the minimum required by existing call sites in the Rust port branch.

## Data Model

The analysis input lists only anonymous C data structures and does not identify named structs directly tied to the two target functions. For this module plan, data migration should therefore remain minimal and function-driven.

### Data-Structure Mapping

Because the referenced C data structures are anonymous and the target functions are scalar parsing helpers, the expected Rust mapping is:

- C anonymous helper/state structures not directly required by `get_signed_int` / `get_unsigned_int`
  → no standalone Rust struct unless a current call site requires one

- C integer out-parameters
  → direct Rust return values, typically via `Result<i64, ParseOptError>` or `Result<u64, ParseOptError>`, or narrower integer types if the original function contracts prove that necessary in surrounding code

- C status/error return codes
  → Rust `Result<T, ParseOptError>`

### Error Model

Introduce only the minimal error representation needed by these two functions, for example:

```rust
enum ParseOptError {
    InvalidInteger,
    NegativeNotAllowed,
    Overflow,
}
```

This error enum should remain local to the parseopt module unless existing Rust port structure already defines a shared option-parsing error type. In that case, reuse the existing type instead of creating a new abstraction.

### Memory Management Notes

- No manual memory management should be carried over.
- Inputs should be borrowed as `&str`.
- Outputs should be plain integer values.
- No heap allocation should be introduced unless required by existing error formatting conventions.

## Implementation Phases

## Phase 1: Inspect and Define Rust Function Contracts

- Review the original `src/parseopt/optset.c` implementations of `get_signed_int` and `get_unsigned_int`.
- Identify:
  - accepted numeric syntax
  - base assumptions, if any
  - whitespace handling expectations
  - sign handling
  - overflow behavior
  - error signaling style
  - any min/max or target-width constraints imposed by call sites
- Review current Rust branch call sites to determine the narrowest correct Rust signatures.
- Define the Rust error type and final function signatures in `src/parseopt/optset.rs`.

### Deliverables

- Rust signatures for both functions
- Minimal module-local error type or confirmed reuse of an existing one
- Notes on exact behavioral parity requirements from the C source

## Phase 2: Port Parsing Logic into `src/parseopt/optset.rs`

- Implement `get_signed_int` using standard-library parsing and explicit validation steps that mirror the C logic.
- Implement `get_unsigned_int` similarly, ensuring rejection of negative values and preservation of overflow behavior.
- Replace C out-parameter patterns with direct return values.
- Replace integer/status-code branching with `Result`.
- Keep the implementation local and straightforward; do not generalize into a separate numeric parsing framework.

### Technical Notes

- Prefer `str::parse` or `from_str_radix` only if they match the original accepted syntax.
- If the C implementation accepts a syntax not exactly modeled by `parse`, implement explicit pre-validation before conversion.
- Use checked conversions when narrowing from parsed intermediate values to target integer widths.
- Keep helper functions private unless shared by existing neighboring Rust code.

### Deliverables

- `src/parseopt/optset.rs` implementation of:
  - `get_signed_int`
  - `get_unsigned_int`

## Phase 3: Integrate with Existing Module Call Sites

- Update existing Rust call sites that currently depend on the C behavior or temporary stubs.
- Align caller-side error handling with the new `Result`-based interface.
- Preserve existing option-processing control flow; only adapt the pieces necessary to consume the migrated functions.
- Confirm module declarations and imports follow the current project layout.

### Deliverables

- Compiling integration with current parseopt module structure
- No remaining dependencies on C-style integer parsing helpers for these functions

## Phase 4: Add Behavioral Tests and Validate Parity

- Add unit tests covering:
  - valid signed integers
  - valid unsigned integers
  - zero
  - negative input for unsigned parsing
  - malformed input
  - boundary and overflow cases
  - any whitespace or formatting edge cases confirmed from the C source
- Add focused regression tests for any branch conditions discovered during migration.
- Run `cargo test` and fix parity mismatches.

### Deliverables

- Unit tests for both parsing functions
- Passing `cargo test`
- Confirmed parity for the implemented function scope