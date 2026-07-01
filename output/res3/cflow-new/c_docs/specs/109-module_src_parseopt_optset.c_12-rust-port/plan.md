# plan.md

## Summary

This plan ports the numeric option-parsing logic from `src/parseopt/optset.c` into Rust on branch `109-module_src_parseopt_optset.c_12-rust-port`, limited to the existing responsibilities represented by:

- `get_signed_int`
- `get_unsigned_int`

The Rust implementation should preserve current parsing behavior and edge-case handling while replacing C-style integer conversion, pointer-based outputs, and implicit error signaling with explicit Rust return types.

Technical approach:

- Migrate only the logic currently housed in `src/parseopt/optset.c` that is required for signed and unsigned integer extraction.
- Implement the parsing using the Rust standard library, primarily `str` parsing and checked range validation.
- Represent parse outcomes with `Result<_, _>` and keep error categories aligned with the original control flow rather than broadening functionality.
- Keep module boundaries narrow: do not introduce new parser subsystems or unrelated abstractions.
- Preserve behavior around invalid input, overflow/underflow, and accepted numeric forms as determined during porting from the C functions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s practical performance for short option-value strings.
  - Avoid unnecessary heap allocation during numeric parsing.
  - Keep parsing paths single-pass where feasible using standard-library primitives.
  - Maintain deterministic error handling with negligible overhead relative to the original C code.

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/optset.c`
  - Migrate relevant logic into a Rust module following standard project layout, preferably:
    - `src/parseopt/optset.rs`
  - If the target crate already uses a parent `mod.rs`/`parseopt.rs` organization, expose only the migrated functions through the existing module tree without creating extra layers.

### Function Mapping

- `get_signed_int`
  - Port to a Rust function in `src/parseopt/optset.rs`
  - Replace output-parameter mutation with a direct return value, typically `Result<i64, ParseOptError>` or the exact integer type required by surrounding call sites
  - Preserve original acceptance/rejection rules and range checks

- `get_unsigned_int`
  - Replace output-parameter mutation with a direct return value, typically `Result<u64, ParseOptError>` or the exact integer type required by surrounding call sites
  - Preserve original rejection of negative values and overflow handling

### Integration Mapping

- Existing callers in the parse-option flow should be updated to consume returned `Result` values instead of relying on C-style status codes and pointer outputs.
- Any module-local helper logic currently embedded in `optset.c` should remain local to `optset.rs` unless already shared elsewhere in the Rust port.

## Data Model

The analysis lists only anonymous C data structures and does not identify named structs directly tied to `get_signed_int` or `get_unsigned_int`. Since these functions are typically scalar-parsing utilities, the Rust port should avoid inventing replacement structs unless required by real call signatures.

### Data-Structure Mapping

- `anonymous` -> **No standalone Rust type unless required by surrounding migrated code**
  - If an anonymous C struct is only incidental to the containing file and not used by these two functions, do not recreate it for this module port.
  - If one or more anonymous structs define option state or parse context that these functions access indirectly, map them narrowly as named Rust structs with only the fields actually used by the migrated functions.

### Scalar and API Mapping

- C signed integer outputs
  - `int`, `long`, or similar -> Rust fixed-width or pointer-width integer chosen to match actual caller expectations
  - Prefer explicit widths (`i32`, `i64`, `u32`, `u64`) once the original range expectations are confirmed

- C unsigned integer outputs
  - `unsigned`, `unsigned long`, or similar -> matching Rust unsigned integer type

- C string inputs
  - `const char *` -> `&str` where UTF-8 is already guaranteed by the Rust-side input path
  - If exact byte preservation is needed because the original parser operates on raw C strings, use `&[u8]` internally only if required by existing Rust interfaces; otherwise keep `&str`

- C status/error returns
  - integer success/failure code -> `Result<T, ParseOptError>`

### Error Model

Define a small module-local error enum only if the surrounding Rust code does not already provide one. Keep it minimal, for example:

- invalid numeric syntax
- signed overflow/underflow
- unsigned overflow
- unexpected negative sign for unsigned parsing

Do not add richer diagnostics unless existing Rust interfaces already require them.

### Memory Management Notes

- No manual memory management should remain in the ported functions.
- Parsing should operate on borrowed input and return plain scalar values.
- Avoid heap allocation for normal parse paths.
- Eliminate pointer writes and null-check patterns by using Rust references and return values.

## Implementation Phases

## Phase 1: Isolate Existing C Semantics and Rust Module Placement

- Inspect `src/parseopt/optset.c` and identify the exact logic boundaries of:
  - `get_signed_int`
  - `get_unsigned_int`
- Determine:
  - accepted prefixes/sign handling
  - whitespace behavior
  - base assumptions
  - overflow/underflow behavior
  - caller-visible error signaling
- Place the Rust target in:
  - `src/parseopt/optset.rs`
- Add only the minimum module declarations required for compilation in the existing crate layout.
- Confirm the concrete integer types expected by Rust-side callers before fixing function signatures.

## Phase 2: Port Signed and Unsigned Parsing Functions

- Implement `get_signed_int` in Rust using:
  - borrowed string input
  - standard-library parsing
  - explicit bounds validation matching the C behavior
- Implement `get_unsigned_int` in Rust using:
  - rejection of negative values consistent with the C implementation
  - overflow checking matching the original numeric width
- Replace C output-parameter patterns with direct returned values.
- Introduce a minimal module-local error type if no existing error type is already used by neighboring Rust ports.
- Keep helper functions private and limited to logic already present in the C source; do not generalize beyond this file’s needs.

## Phase 3: Update Call Sites and Preserve Behavior

- Modify existing Rust-side callers of the ported option-setting logic to use the new return signatures.
- Map previous C success/failure branches onto `Result` handling without changing external module behavior.
- Ensure no extra parsing modes or broader integer support are introduced during integration.
- Remove any now-obsolete transitional code tied to pointer-style output handling.

## Phase 4: Verification with Targeted Tests

- Add focused unit tests in the same module or its existing test location covering:
  - valid signed integers
  - valid unsigned integers
  - zero values
  - maximum in-range values
  - minimum signed in-range values
  - overflow and underflow cases
  - invalid syntax
  - negative input rejected by unsigned parsing
- If the original C behavior includes specific edge cases such as leading sign handling or trailing characters, encode those cases directly in tests.
- Run `cargo test` and resolve discrepancies against the original C semantics rather than “improving” behavior.