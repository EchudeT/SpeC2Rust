# Implementation Plan

## Summary

Port `gnu/itold.c` into an idiomatic Rust module that preserves the existing conversion behavior of `_Qp_itoq` without adding new capabilities. The Rust implementation should focus on reproducing the original integer-to-extended-floating conversion logic with stable, explicit numeric handling and minimal abstraction.

The technical approach is to migrate the single C translation unit into one Rust source module, keep the function boundary aligned with the original exported behavior, and express the conversion steps using Rust primitive integer operations and a project-local floating representation already used elsewhere in the port, if present. If no existing equivalent type is available yet, the module should define only the smallest internal representation necessary to support the migrated function. Memory ownership should remain straightforward and stack-based, with no dynamic allocation.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s constant-time, allocation-free conversion profile.
  - Avoid unnecessary intermediate allocations or heap-backed buffers.
  - Keep bit-level manipulation explicit so generated code remains close to the original low-level intent.
  - Preserve deterministic behavior for all supported integer inputs.

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/itold.c` | `src/module_gnu_itold_c_32.rs` | Direct migration target for `_Qp_itoq` |
| `_Qp_itoq` | `pub(crate)` or visibility matching current crate export needs | Keep naming close to source unless crate conventions require a Rust-style wrapper while retaining the original symbol-level intent |

## Data Model

This module analysis does not list any standalone C structs or enums. The migration should therefore avoid inventing new public data structures.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| Primitive integer input used by `_Qp_itoq` | Corresponding Rust integer primitive (`i32`, `u32`, or exact source-matching type) | Confirm exact signedness and width from the original function signature |
| Return/output floating representation | Existing project type for the target extended/quad-like value, if already defined; otherwise a minimal crate-local struct used by this function only | Reuse existing representation rather than introducing a parallel type |
| Bit fields / word assembly logic | Plain Rust integer arithmetic and shifts | Prefer explicit masks/shifts over transmute-like approaches |

### Memory Management

- Use only stack-local values and returned-by-value results.
- Do not introduce heap allocation, reference counting, or interior mutability.
- Keep conversions explicit to avoid accidental widening, truncation, or sign-extension differences from C.

### Error Handling

- If the original C function is total over its input domain, keep the Rust function total as well.
- Do not introduce `Result` unless there is concrete evidence of failure behavior in the source.
- Handle edge values through direct numeric logic rather than fallback paths.

## Implementation Phases

### Phase 1: Source Inspection and Signature Lock-In

- Inspect `gnu/itold.c` to confirm:
  - exact `_Qp_itoq` signature,
  - integer width and signedness,
  - target floating representation and any dependent helper types/macros.
- Identify whether this function writes into an out-parameter, returns a value, or constructs a project-defined extended type.
- Map any C macros or constant definitions used by the function into local Rust `const` items in the same module.
- Create `src/module_gnu_itold_c_32.rs` with the function stub and exact type placeholders aligned to the existing Rust port structure.

### Phase 2: Core Conversion Port

- Port `_Qp_itoq` directly into Rust, preserving operation order where it affects bit layout or rounding behavior.
- Replace C-style casts with explicit Rust conversions at each step to preserve signedness semantics.
- Express normalization, sign handling, exponent construction, and mantissa/word assembly using primitive operations.
- Reuse an existing crate-local floating container if one already represents the same target format; otherwise add only the minimal internal type required by this function.
- Keep implementation local to this module rather than introducing utility layers unless the dependency already exists in the ported codebase.

### Phase 3: Validation Tests

- Add unit tests in the module or corresponding test file covering:
  - zero,
  - small positive and negative integers,
  - boundary values for the source integer width,
  - powers of two and adjacent values to validate normalization boundaries,
  - sign preservation and exact representability cases.
- Where feasible, derive expected values from the known bit construction rules or from existing project behavior rather than adding external tooling.
- Run `cargo test` and adjust any conversion details needed to align with C semantics.

### Phase 4: Integration Cleanup

- Ensure the module is wired into the crate with only the necessary `mod` and visibility declarations.
- Remove placeholders introduced during porting and keep comments limited to places where C bit-level behavior is not obvious in Rust.
- Verify there are no unused abstractions, extra wrappers, or module splits beyond the direct migration target.
- Confirm the final implementation remains allocation-free and limited in scope to the original C module responsibilities.