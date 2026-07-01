# Implementation Plan

## Summary

This module migration covers the `src/parseopt/wordwrap.c` functionality limited to:

- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`

The Rust implementation should preserve the existing behavior of margin configuration logic and remain narrowly scoped to the current module boundary. The technical approach is to port the existing state mutation logic into a Rust module that updates the Rust-owned word-wrap state directly, using explicit integer width choices and bounds checks that match the C semantics as closely as practical.

The implementation should avoid introducing new formatting features or broader text-wrapping redesign. Work should focus on translating the existing setter behavior, preserving call patterns, and making ownership and mutation explicit through Rust references instead of implicit pointer-based mutation.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time margin updates
  - No heap allocation in setter paths
  - Equivalent or lower overhead than the C setters
  - Preserve direct mutable state updates without introducing indirection

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/wordwrap.c`
  → `src/parseopt/wordwrap.rs`

### Function Mapping

- `wordwrap_set_left_margin`
  → `pub(crate) fn set_left_margin(...)` or `impl WordWrapState { fn set_left_margin(&mut self, ...) }`

- `wordwrap_set_right_margin`
  → `pub(crate) fn set_right_margin(...)` or `impl WordWrapState { fn set_right_margin(&mut self, ...) }`

### Recommended Rust Module Placement

Keep the Rust code in the direct structural equivalent of the C location:

- `src/parseopt/mod.rs`
- `src/parseopt/wordwrap.rs`

If the surrounding port already represents word-wrap state as a dedicated type, the setters should be implemented as methods on that existing type rather than introducing an additional abstraction layer.

## Data Model

The analysis lists only anonymous C data structures and does not provide field definitions. Because the requested functions are margin setters, the migration should map only the state actually touched by these functions.

### C to Rust Data Mapping Strategy

Anonymous C structures used by this module should be replaced with named Rust structures only where required by the existing ported API surface.

#### Minimal expected state mapping

If the C module mutates a word-wrap configuration/state object containing left and right margin fields, map it as:

```rust
pub(crate) struct WordWrapState {
    left_margin: usize,
    right_margin: usize,
}
```

If the original C fields are signed integers or use sentinel values, preserve that more exactly instead:

```rust
pub(crate) struct WordWrapState {
    left_margin: i32,
    right_margin: i32,
}
```

### Mapping Rules

- **C anonymous struct holding wrap configuration**
  → `WordWrapState` or the already-established Rust equivalent in this port
- **C integer margin fields**
  → `usize` if values are inherently non-negative and used as widths/index-like values
  → `i32`/`isize` if negative values or C-compatible signed behavior must be preserved
- **C mutable pointer to state (`struct *`)**
  → `&mut WordWrapState`
- **C nullability assumptions**
  → eliminate null handling where the Rust API guarantees a valid mutable reference
- **C implicit shared mutable access**
  → explicit exclusive mutable borrow in Rust

### Memory Management Notes

These setter functions should not own memory and should not allocate. Rust should model them as pure state mutation on an existing owner-managed structure. This removes the C risks of null dereference and invalid pointer mutation while keeping behavior aligned.

### Error Handling Notes

If the original C setters silently accepted values and only assigned fields, Rust should do the same and avoid inventing `Result` returns.

If the original implementation normalizes or constrains invalid relationships such as:

- `left_margin > right_margin`
- negative margin input
- zero-width right margin

then the Rust code should preserve that exact behavior locally inside the setter logic, without introducing new validation APIs.

## Implementation Phases

## Phase 1: Inspect and define the Rust state surface

- Identify the Rust type that corresponds to the C word-wrap state mutated by these setters.
- Confirm the exact C field types and any setter-side normalization behavior.
- Create or update `src/parseopt/wordwrap.rs` with the minimal state representation needed by these functions.
- Choose Rust integer types based on the original C field semantics rather than convenience.

### Deliverables

- Rust module file in the C-equivalent location
- Named Rust state type or integration with pre-existing ported state type
- Documented field/type decisions in code comments where C signedness or range behavior matters

## Phase 2: Port the setter functions directly

- Implement the Rust equivalents of `wordwrap_set_left_margin` and `wordwrap_set_right_margin`.
- Translate pointer-based mutation into `&mut` mutation.
- Preserve assignment order, clamping, and cross-field adjustments exactly as in the C implementation.
- Keep the function scope narrow: no extra helper layers unless necessary to express the original logic clearly.

### Deliverables

- Working Rust implementations of both setter functions
- Function signatures aligned with the surrounding Rust port conventions
- No added functionality beyond the C behavior

## Phase 3: Add targeted unit tests for migrated behavior

- Write unit tests covering direct field updates for left and right margins.
- Add tests for any boundary behavior observed in the C implementation:
  - zero values
  - signed/unsigned edge cases
  - left/right relationship adjustments, if present
- Verify that the state after each setter call matches expected C-compatible behavior.

### Deliverables

- `cargo test` coverage for both migrated functions
- Tests focused on state mutation semantics only

## Phase 4: Integrate and remove C dependency for this slice

- Wire call sites in the Rust branch to use the Rust setters.
- Confirm no remaining dependency on the C implementations for these two functions.
- Run module-level regression checks through `cargo test`.

### Deliverables

- Rust call paths active for both setter operations
- C implementation no longer needed for this module slice
- Final verification that behavior remains within the existing module contract