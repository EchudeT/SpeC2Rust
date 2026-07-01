# Implementation Plan

## Summary

Port `gnu/itold.c` into an idiomatic Rust module that preserves the existing conversion behavior of `_Qp_itoq` without adding new interfaces or expanding scope. The Rust implementation should focus on reproducing the original integer-to-extended-precision conversion logic as closely as possible, using Rust primitive integer operations and a narrowly scoped internal representation that matches the needs of the migrated function.

The implementation should remain file- and function-oriented: migrate the single source file into a corresponding Rust module, translate `_Qp_itoq` directly, and keep memory ownership explicit through Rust value semantics. Since the input analysis does not indicate external state, heap allocation, or auxiliary data structures, the port should prefer plain functions, fixed-size numeric fields, and deterministic conversion steps.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time, allocation-free execution for the conversion path.
  - Avoid heap allocation and unnecessary intermediate containers.
  - Keep generated code close to simple integer/bit-manipulation logic expected from the C implementation.
  - Maintain predictable behavior for edge cases such as zero, sign handling, and integer-width boundaries.

## Module Mapping

- **C source file**: `gnu/itold.c`
- **Rust module**: `src/module_gnu_itold_c_32.rs`

Function mapping:

- `gnu/itold.c::_Qp_itoq`
  - -> `src/module_gnu_itold_c_32.rs::_qp_itoq`

If the broader crate already exposes a namespace for translated GNU helpers, place this file within that existing module tree and keep the function localized there. Do not introduce additional helper modules unless they are required to complete the direct translation of this file.

## Data Model

No explicit C structs were identified in the analysis input. The data model should therefore be kept minimal and driven only by the requirements of `_Qp_itoq`.

Planned mapping:

- **C primitive integer input types**
  - -> Rust signed/unsigned fixed-width integers (`i32`, `u32`, or the exact source-equivalent type determined during translation)
- **C extended-precision result representation**
  - -> Rust struct only if required by the surrounding ported codebase to represent the target `_Qp`/long-double-like value
- **No dynamic memory structures**
  - -> Rust stack values and return-by-value semantics

If `_Qp_itoq` writes into a destination object rather than returning a value, map:

- `C out-parameter pointer`
  - -> `&mut TargetType` in Rust

If the original function returns the converted value directly, map:

- `C value return`
  - -> Rust direct return of the translated target type

If an internal representation is necessary for the destination extended-precision value, use a compact Rust struct matching only the original storage fields needed by this module, for example:

```rust
struct QpRepr {
    sign: bool,
    exponent: u16,
    significand: u128,
}
```

This struct should be introduced only if the existing Rust port does not already define the destination type. Reuse existing project types whenever available instead of creating parallel representations.

## Implementation Phases

### Phase 1: Analyze and establish direct module skeleton

- Create the Rust file `src/module_gnu_itold_c_32.rs`.
- Inspect `gnu/itold.c` to determine the exact signature of `_Qp_itoq`, including:
  - integer source width and signedness
  - whether the result is returned or written through a pointer
  - any dependency on shared numeric representation types or macros
- Map the exact C signature to Rust using:
  - primitive integer types for scalar inputs
  - `&mut` for writable output parameters
  - a project-local representation type if already defined elsewhere in the port
- Keep naming as close as practical to the original function for traceability, while adapting to Rust identifier rules.

### Phase 2: Translate conversion logic faithfully

- Port `_Qp_itoq` line-by-line into Rust-oriented integer and bit operations.
- Preserve the original handling for:
  - zero input
  - negative values and sign extraction
  - magnitude normalization
  - exponent/significand assembly
  - width-sensitive shifts and masks
- Replace pointer arithmetic or raw memory writes with direct field assignment on Rust values.
- Avoid `unsafe` unless the surrounding crate architecture makes it strictly unavoidable; for this function, safe Rust should be the default expectation.
- Keep helper logic private and local to this module if small translation aids are required.

### Phase 3: Validate edge cases and representation boundaries

- Add unit tests in the same module or the crate’s standard test location covering:
  - zero
  - one and minus one
  - maximum and minimum values of the translated integer input type
  - powers of two and adjacent values
  - sign-sensitive boundary cases, especially absolute-value handling for the most negative signed integer
- Where expected output can be expressed structurally, assert individual destination fields.
- If the destination type already supports comparison or formatting in the ported codebase, use that existing mechanism rather than introducing new utilities.

### Phase 4: Integrate and tighten module consistency

- Ensure the Rust module is wired into the crate using standard `mod` declarations only where necessary.
- Confirm that the function signature and visibility match how the rest of the translated project will call it.
- Remove any translation scaffolding that is not needed after tests pass.
- Verify `cargo test` succeeds for the module and that the final code remains limited to the migrated functionality from `gnu/itold.c`.