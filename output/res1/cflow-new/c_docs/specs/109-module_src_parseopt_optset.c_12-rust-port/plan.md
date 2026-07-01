# Implementation Plan: module_src_parseopt_optset.c_12

## Summary

This module ports the numeric option-parsing logic from `src/parseopt/optset.c` into Rust, limited to the existing responsibilities of:

- parsing signed integers (`get_signed_int`)
- parsing unsigned integers (`get_unsigned_int`)

The Rust implementation should preserve current parsing behavior, boundary handling, and call-level semantics as closely as possible, while replacing C pointer-based and errno-style patterns with explicit Rust return types.

The implementation approach is:

- migrate the logic into a Rust module dedicated to parse-option numeric conversion
- keep the scope narrowly aligned with the two existing C functions
- represent parse success/failure with `Result` and small module-local error types
- use standard-library integer parsing and checked range validation where it matches the C behavior; otherwise perform explicit validation to preserve edge-case compatibility
- avoid introducing new parsing abstractions beyond what is needed to host the migrated functions

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - maintain constant-space parsing behavior
  - keep runtime cost equivalent to C-style scalar parsing for typical option values
  - avoid heap allocation in the parsing path
  - preserve predictable failure handling for invalid and out-of-range inputs

## Module Mapping

### Source File Mapping

- `src/parseopt/optset.c`
  - migrate relevant logic into a Rust module such as:
    - `src/parseopt/optset.rs`
  - if the current Rust project already organizes parse-option code under a different file, place these functions in the existing corresponding module rather than creating extra layers

### Function Mapping

- `get_signed_int`
  - C function handling signed numeric conversion
  - Rust function: `get_signed_int(...) -> Result<..., ParseOptError>`

- `get_unsigned_int`
  - C function handling unsigned numeric conversion
  - Rust function: `get_unsigned_int(...) -> Result<..., ParseOptError>`

### Mapping Notes

- Preserve existing function boundaries instead of collapsing both functions into a generic helper unless such a helper is strictly internal and reduces direct duplication.
- Keep the Rust signatures aligned with actual call-site needs in the branch, especially:
  - input as `&str` or byte slice depending on surrounding parser representation
  - output integer width chosen from the C implementation’s effective destination type
- Any C output-parameter usage should be converted into direct Rust return values.

## Data Model

The analysis lists only anonymous C data structures and does not identify named structs directly used by this function subset. Since this module slice is function-focused, the plan should avoid inventing new public data structures unless required by existing call sites.

### Data-structure Mapping

- **anonymous C structures**
  - no standalone Rust struct should be created solely for this migration unless one of these anonymous types is proven to carry parse-state required by `get_signed_int` or `get_unsigned_int`
  - if one of the anonymous structures corresponds to an existing option/parser state container already represented elsewhere in Rust, reuse that existing Rust type
  - if local parse context is needed only inside this module, prefer:
    - plain function parameters, or
    - a small private struct only if it directly mirrors C state used by both functions

### Error Representation

Because C integer parsing often relies on sentinel returns and `errno`, the Rust port should use an explicit module-local error enum, for example:

```rust
enum ParseOptError {
    InvalidInteger,
    OutOfRange,
}
```

This should remain private unless external callers already require typed propagation.

### Scalar Type Mapping

Use direct Rust scalar mappings matching the effective C types used by the original functions:

- C signed integer target
  - `i32`, `i64`, or `isize` depending on actual destination width in the source
- C unsigned integer target
  - `u32`, `u64`, or `usize` depending on actual destination width in the source
- C string input
  - `&str` when source text is valid UTF-8 in the Rust parser path
  - otherwise `&[u8]` plus explicit ASCII digit handling if the surrounding parser is byte-oriented

### Memory Management Notes

- No manual allocation should be introduced.
- Eliminate raw-pointer output parameters by returning parsed values directly.
- Borrow input data rather than copying it.
- Keep temporary parsing state on the stack.

## Implementation Phases

## Phase 1: Inspect and Pin Down Existing C Semantics

- Read the `src/parseopt/optset.c` implementations of:
  - `get_signed_int`
  - `get_unsigned_int`
- Identify for each function:
  - accepted input format
  - whether leading `+` or `-` is accepted
  - whether whitespace trimming occurs or not
  - base assumptions (decimal only vs delegated C conversion behavior)
  - overflow and underflow handling
  - treatment of empty strings, trailing characters, and sign-only strings
  - actual destination integer width
  - return convention used by callers
- Inspect direct call sites in the Rust port branch to determine the narrowest compatible Rust signature.
- Document any behavior that depends on C library conversion details so it can be replicated intentionally rather than accidentally changed.

## Phase 2: Port the Parsing Functions into Rust

- Create or update the Rust module corresponding to `optset.c`.
- Implement `get_signed_int` with:
  - direct input validation
  - integer parsing via standard library or explicit digit scanning if needed for exact compatibility
  - checked bounds enforcement against the target integer type
  - `Result`-based failure reporting
- Implement `get_unsigned_int` with:
  - explicit rejection of negative forms
  - unsigned parsing and overflow checks
  - same invalid-trailing-character behavior as the C code
- Keep any shared internal helper private and minimal.
- Do not generalize into a broader option parsing framework.

## Phase 3: Integrate with Existing Call Paths

- Replace C-style success/failure propagation with Rust `Result` handling at the immediate call sites.
- Map previous sentinel/error-code logic into the project’s existing error reporting style, but only at the necessary integration boundary.
- Ensure there is no reliance on mutable global state or `errno` after migration.
- Preserve current module boundaries and do not move unrelated option-setting logic.

## Phase 4: Verify Behavior with Focused Tests

- Add unit tests covering both functions for:
  - valid signed values
  - valid unsigned values
  - zero
  - minimum and maximum in-range values
  - overflow and underflow
  - empty input
  - non-digit input
  - trailing junk characters
  - sign handling differences between signed and unsigned parsing
- Add call-site level tests only where needed to verify that integration preserved prior outcomes.
- Run `cargo test` and adjust edge-case handling until results match the original C behavior as closely as possible.