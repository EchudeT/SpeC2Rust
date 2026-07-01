# Implementation Plan

## Summary

Port the position-tracking and line-wrapping logic from `src/parseopt/wordwrap.c` into a Rust module that preserves the existing behavior and call structure as closely as practical. The Rust implementation should focus on migrating the current functions:

- `position_init`
- `position_incr`
- `position_add`
- `position_eq`
- `wordwrap_last_ws`
- `flush_line`

The technical approach is to translate the current C stateful text-position handling into small Rust value types and plain functions or inherent methods, using `String`, `&str`, and index-based scans where needed. Ownership should replace manual buffer lifetime management, while keeping mutation localized to the wrapping state that corresponds to the original C code. The migration should avoid introducing broader formatting abstractions or new wrapping features beyond what is already present in `wordwrap.c`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time behavior for line scanning and whitespace search.
  - Avoid unnecessary string reallocations where the C code already works in-place or incrementally.
  - Keep position updates as cheap value operations.
  - Match current module-level performance characteristics without adding extra indirection or heap-heavy abstractions.

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/wordwrap.c`
  - `src/parseopt/wordwrap.rs` or `src/parseopt/mod.rs` with a `wordwrap` submodule, depending on the existing Rust crate layout

### Function Mapping

- `position_init`
  - Rust: `Position::new(...)` or `Position::init(...)`
- `position_incr`
  - Rust: `Position::incr(&mut self, ...)`
- `position_add`
  - Rust: `Position::add(&mut self, ...)` or `fn position_add(pos: &mut Position, ...)`
- `position_eq`
  - Rust: `impl PartialEq for Position` or a retained helper `fn position_eq(...) -> bool`
- `wordwrap_last_ws`
  - Rust: internal helper scanning `&str` or byte slice for the last wrap-eligible whitespace position
- `flush_line`
  - Rust: internal mutable function operating on the wrapping state and output sink

### Rust Module Scope

Keep the migration confined to the parseopt area corresponding to the original file. Do not split the behavior into additional support modules unless required by the existing Rust project structure. Internal helpers should remain private to the Rust module unless another already-ported module requires direct access.

## Data Model

Because the C analysis only exposes anonymous structures, the Rust data model should be reconstructed directly from actual field usage in `wordwrap.c` during implementation. The mapping should remain minimal and file-local where possible.

### Data-structure Mapping Strategy

- **C anonymous struct used for text position**
  - Rust: `struct Position`
  - Expected contents: line/column-style counters or equivalent position state inferred from `position_*` functions
  - Notes:
    - Use integer types appropriate to indexing/counting, typically `usize`
    - Implement by-value semantics if the C struct is copied freely

- **C anonymous struct used for wrapping/output state**
  - Rust: `struct WordWrapState` or similarly named file-local struct
  - Expected contents inferred from `flush_line` and `wordwrap_last_ws`:
    - current line buffer
    - width/accounting fields
    - current position and/or indent-related position
    - output destination state if held in the C struct
    - Replace raw character buffers with `String` or `Vec<u8>` only if byte-level mutation is required by the original logic
    - Prefer `String` when the C code is fundamentally text-oriented and emits textual lines

- **C pointer/reference relationships**
  - Rust:
    - borrowed parameters as `&str`, `&mut str` rarely, `&mut Position`, `&mut WordWrapState`
    - owned mutable accumulation as `String`
    - Eliminate nullable pointer patterns using `Option<T>` or explicit preconditions only where the C code actually permits absence

### C-to-Rust Type Guidance

- C integral counters/indexes
  - Rust: `usize` for lengths/indexes, or `u32`/`i32` only if semantics require exact range/sign behavior
- C char buffers
  - Rust: `String` for UTF-8 text, or byte slices if the original code is byte-oriented and depends on ASCII whitespace checks by index
- C boolean-style comparisons
  - Rust: `bool`
- C mutation through out-parameters
  - Rust: return values or `&mut` updates, favoring the simpler form that best matches each original function

### Memory Management and Error Handling

- Remove manual memory management entirely in favor of Rust ownership.
- Keep APIs infallible where the original functions are pure state transforms.
- For output operations in `flush_line`, use `std::fmt::Write` or an existing crate-local output abstraction only if already present in the Rust port; otherwise keep writing into a `String` or module-local sink interface.
- Avoid introducing custom error enums unless the C code already exposes failure paths requiring propagation.

## Implementation Phases

## Phase 1: Reconstruct and Port Position State

- Inspect `src/parseopt/wordwrap.c` to recover the exact fields used by `position_init`, `position_incr`, `position_add`, and `position_eq`.
- Define the Rust `Position` struct with only those fields.
- Port:
  - `position_init`
  - `position_incr`
  - `position_add`
  - `position_eq`
- Decide whether equality should be expressed through `PartialEq` while retaining a small wrapper helper if that makes call-site migration simpler.
- Add unit tests covering:
  - initialization values
  - increment behavior
  - additive updates
  - equality/inequality cases

## Phase 2: Port Whitespace Search Logic

- Reconstruct the exact search rules used by `wordwrap_last_ws`:
  - what counts as whitespace
  - whether scanning is byte-based or character-based
  - whether boundary conditions depend on current width/position state
- Implement the helper as a private Rust function operating on the same conceptual inputs as the C version.
- Prefer byte-slice scanning if the original C logic is ASCII/byte oriented; this best preserves behavior and indexing.
- Add focused tests for:
  - no-whitespace input
  - trailing whitespace
  - internal last-whitespace detection
  - edge cases at empty or single-character inputs

## Phase 3: Port Line Flush Behavior

- Reconstruct the wrapping state structure required by `flush_line`.
- Define the minimal Rust state struct containing only the fields referenced by the C implementation.
- Port `flush_line` with close attention to:
  - mutation ordering
  - buffer clearing/reset behavior
  - emitted newline handling
  - position updates coupled to flushing
- Replace raw buffer operations with `String` mutation while preserving output order and trimming/splitting semantics from the C code.
- Add unit tests using representative wrapped-line scenarios to verify:
  - flush on non-empty buffer
  - repeated flush behavior
  - interactions with tracked position state
  - handling of whitespace boundaries already identified by the helper

## Phase 4: Integration and Behavioral Verification

- Wire the Rust `wordwrap` module into the existing `cflow-new` Rust branch structure in the same area as the C source it replaces.
- Update call sites only as needed to match Rust ownership and borrowing requirements; avoid widening the public API.
- Run `cargo test` and add regression tests derived from observed C behavior in this module.
- Perform a final review for:
  - absence of unnecessary allocations
  - no unchecked indexing unless justified by prior bounds checks
  - no added capabilities or altered formatting semantics beyond the original implementation