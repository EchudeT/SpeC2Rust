# Implementation Plan: module_gnu_is_infinite_18

## Summary

This module ports the floating-point classification helpers from `gnu/vasnprintf.c` into Rust, limited to the two existing functions:

- `is_infinite_or_zero`
- `is_infinite_or_zerol`

The Rust implementation should preserve the original intent and call boundaries while replacing C floating-point inspection logic with standard-library methods on Rust floating-point primitives. Since the source only exposes function-level behavior and no persistent module state, the port should remain a small utility module with direct function translations and no added abstraction layers.

The technical approach is:

- map the C floating-point checks to Rust `f64` and `f64`/platform-equivalent handling using `is_infinite()` and zero comparison;
- keep behavior explicit for signed zero and infinities;
- avoid unsafe code unless required by surrounding migration constraints in the target file;
- place the implementation in a Rust module mirroring the source responsibility from `gnu/vasnprintf.c`, without introducing unrelated helpers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - constant-time floating-point classification;
  - no heap allocation;
  - no observable overhead beyond direct standard-library intrinsic checks;
  - behavior suitable for use in formatting-related hot paths.

## Module Mapping

| C Source File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `gnu/vasnprintf.c` | `is_infinite_or_zero` | `src/gnu/vasnprintf.rs` | `fn is_infinite_or_zero(x: f64) -> bool` |
| `gnu/vasnprintf.c` | `is_infinite_or_zerol` | `src/gnu/vasnprintf.rs` | `fn is_infinite_or_zerol(x: f64) -> bool` |

### Mapping Notes

- Both C functions should remain colocated in one Rust module because they originate from the same source file and form a narrowly scoped utility set.
- If the broader port already defines a `gnu` module tree, this file should be added there directly rather than creating a new crate or standalone utility module.
- If `long double` behavior from C is not materially distinguishable in the current migration target, `is_infinite_or_zerol` should use the closest Rust scalar type available in the port, which is expected to be `f64` unless an existing project-wide alias is already established.

## Data Model

No named C structs are identified for this module.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | No dedicated Rust data structure required | Function-only port; preserve behavior through scalar argument mapping |

### Scalar Type Mapping

| C Type Context | Rust Type | Notes |
|---|---|---|
| `double` input | `f64` | Direct standard mapping |
| `long double` input | `f64` by default | Rust has no stable native `long double`; use project-established alias if one already exists |

### Memory Management

- No owned heap memory is required.
- All logic should operate on by-value floating-point inputs.
- No lifetimes or borrowing complexity are expected.

### Error Handling

- These functions should remain total boolean predicates.
- No `Result` return type is needed.
- NaN should naturally evaluate to `false` unless the original surrounding semantics require otherwise.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and signatures

- Create or update `src/gnu/vasnprintf.rs`.
- Add Rust function signatures corresponding to:
  - `is_infinite_or_zero`
  - `is_infinite_or_zerol`
- Wire the file into the existing `mod` tree only as needed for the current migration branch.
- Confirm the chosen Rust scalar type for the `long double`-based function from existing project conventions before implementation.

### Deliverables

- Compiling module file with placeholder or direct implementations.
- Stable function names and visibility aligned with current migration needs.

## Phase 2: Port floating-point classification logic

- Implement `is_infinite_or_zero(x)` using Rust standard-library floating-point classification.
- Implement `is_infinite_or_zerol(x)` with the same decision logic, using the selected Rust type.
- Preserve recognition of:
  - positive infinity;
  - negative infinity;
  - positive zero;
  - negative zero.
- Ensure finite nonzero values return `false`.
- Ensure NaN returns `false`.

### Technical Decisions

- Prefer `x.is_infinite() || x == 0.0`.
- Rely on IEEE-compatible Rust float behavior for signed zero equality.
- Keep implementation free of unnecessary wrappers or trait generalization.

## Phase 3: Add focused tests for translated behavior

- Add unit tests in the same module or the module’s test section.
- Cover:
  - `0.0`;
  - `-0.0`;
  - `f64::INFINITY`;
  - `f64::NEG_INFINITY`;
  - representative finite nonzero values;
  - `f64::NAN`.
- Duplicate the same behavior coverage for `is_infinite_or_zerol` using the chosen mapped type.

### Deliverables

- `cargo test` passing for all predicate cases.
- Explicit assertions documenting the ported classification behavior.

## Phase 4: Final integration review

- Verify function placement remains consistent with the migrated contents of `gnu/vasnprintf.c`.
- Remove any temporary placeholders or dead code introduced during translation.
- Confirm there are no added module layers, utility traits, or extra APIs beyond the original function set.
- Run full project `cargo test` to ensure no regressions from the added module linkage.

### Exit Criteria

- Functions compile and are reachable from the intended Rust module path.
- Tests pass locally with standard toolchain.
- The port remains narrowly scoped to the original C functions only.