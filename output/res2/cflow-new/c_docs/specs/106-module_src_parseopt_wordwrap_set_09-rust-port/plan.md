# Implementation Plan

## Summary

This module ports the `src/parseopt/wordwrap.c` setter functionality for left and right margin configuration into Rust, limited to the behavior currently represented by:

- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`

The Rust implementation should preserve the existing module’s state-mutation behavior, with a direct translation of margin-related updates rather than redesigning the surrounding word-wrap subsystem. The preferred approach is to migrate the relevant state holder from the C implementation into a Rust struct and expose two setter methods or equivalent free functions with signatures aligned to the current crate layout.

The implementation should remain narrow:

- migrate only the code paths needed by these two setters,
- keep ownership and mutation explicit through Rust borrowing,
- replace any implicit C assumptions about valid pointers and integer state with checked Rust field access and integer types,
- avoid introducing additional formatting, wrapping, or parsing capabilities beyond what the original file already supports.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time setter operations.
  - No heap allocation in margin-setting paths.
  - Behavioral parity with the C implementation for valid and edge-case inputs used by the existing module.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` | Port only the data access and functions needed for left/right margin setters. |
| `wordwrap_set_left_margin` | `WordWrap::set_left_margin` or `set_left_margin(&mut WordWrap, ...)` | Final form should follow the crate’s existing Rust style if already established. |
| `wordwrap_set_right_margin` | `WordWrap::set_right_margin` or `set_right_margin(&mut WordWrap, ...)` | Keep semantics matched to C state updates without adding validation not present in the source behavior. |

If the surrounding Rust port already uses a module file such as `src/parseopt/mod.rs`, only add the minimal `mod wordwrap;` / export wiring required to make the migrated code compile.

## Data Model

The analysis input identifies only anonymous C data structures and does not provide field layouts. Because the target functions are setters, the Rust migration should infer and port only the concrete state required by these two functions.

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| Anonymous word-wrap state carrier used by setter functions | `struct WordWrap` | Define only fields directly read or written by the setter functions and any fields required for invariant preservation already present in adjacent migrated code. |
| C integer margin fields | `usize`, `u32`, or `isize`/`i32` depending on observed C semantics | Match signedness and range behavior from the original code. Use unsigned types only if the C code clearly treats margins as non-negative. |
| C pointer-based mutable object access | `&mut WordWrap` | Replaces nullable/raw pointer expectations with Rust mutable borrowing. |
| C sentinel/error convention, if any | `Result<(), Error>` only if the C code has explicit failure paths; otherwise `()` | Do not invent new error states for simple state assignment. |

### Data-structure migration rules

1. **Port the owning state struct minimally**
   Create or extend a Rust `WordWrap` struct only with fields touched by:
   - `wordwrap_set_left_margin`
   - `wordwrap_set_right_margin`

2. **Preserve integer semantics**
   If the C implementation uses plain `int`, start with `i32` in Rust unless surrounding migrated code already standardizes on `usize` and the C logic proves non-negative-only usage.

3. **Eliminate nullability at the API boundary**
   C functions that assume non-null object pointers should become Rust functions operating on `&mut self` or `&mut WordWrap`.

4. **Keep invariants local**
   If the C setters normalize or constrain margins relative to each other, encode the same behavior inside the setter methods only; do not create a broader validation layer.

## Implementation Phases

## Phase 1: Inspect and carve out the minimal word-wrap state

- Review `src/parseopt/wordwrap.c` and identify:
  - the concrete state structure used by both setter functions,
  - exact field names and integer types,
  - whether either setter adjusts related fields or relies on ordering constraints,
  - whether either function can fail or is pure assignment.
- Create `src/parseopt/wordwrap.rs` if it does not yet exist.
- Define the minimal Rust `WordWrap` struct needed for these functions.
- Add only the minimum module declarations/imports required for compilation.

**Deliverable**: compilable Rust module skeleton with the state struct and placeholders for both setters.

## Phase 2: Port the setter logic directly

- Implement Rust equivalents of:
  - `wordwrap_set_left_margin`
  - `wordwrap_set_right_margin`
- Translate C field mutation line-for-line where practical.
- Preserve any ordering or clamping semantics present in the C code.
- Use direct mutable borrowing instead of pointer mutation.
- Keep function visibility aligned to current crate needs; avoid public exposure unless the existing architecture requires it.

**Deliverable**: both setter functions implemented with C-equivalent state transitions.

## Phase 3: Reconcile types, ownership, and error behavior

- Verify that chosen Rust integer types match observed C usage.
- If the C code accepts values that may be negative or uses sentinel values, preserve that behavior explicitly.
- If the C functions do not fail, keep return type as `()`.
- If explicit failure behavior exists in the source, encode only that behavior with a small local error type or existing crate error type, without widening the API surface.
- Remove any unnecessary temporary abstractions introduced during translation.

**Deliverable**: finalized APIs and struct fields with stable type choices and no extra behavior.

## Phase 4: Add focused tests and complete integration

- Add unit tests covering:
  - setting left margin updates the intended field,
  - setting right margin updates the intended field,
  - edge conditions reflected in the C implementation,
  - interactions between left and right margins if the C code enforces any relationship.
- Run `cargo test` and fix integration issues with surrounding parseopt modules.
- Ensure no allocation, threading, or unrelated utility code was introduced.

**Deliverable**: passing targeted tests for migrated setter behavior and integrated Rust module wiring.

## Notes on Memory Management and Error Handling

- Memory management should be fully automatic via Rust ownership; no manual allocation logic should be introduced for these setter paths.
- Replace mutable raw-pointer access with `&mut` references.
- Do not add interior mutability or synchronization primitives.
- Error handling should remain minimal and source-driven:
  - use plain mutation with `()` return if the C functions are infallible,
  - use `Result` only when explicit C failure semantics must be preserved.