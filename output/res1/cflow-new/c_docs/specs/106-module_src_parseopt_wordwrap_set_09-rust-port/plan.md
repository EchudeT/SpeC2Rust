# Implementation Plan: module_src_parseopt_wordwrap_set_09

## Summary

This work ports the margin-setting portion of `src/parseopt/wordwrap.c` into Rust, limited to the existing responsibilities of:

- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`

The Rust implementation should preserve the current module behavior and state-updating semantics without adding new formatting features or widening the public API. The expected technical shape is a small Rust module that owns the word-wrap configuration state and exposes direct setter functions/methods for left and right margins.

The implementation approach is:

- migrate the relevant state fields from the C word-wrap context into a Rust struct;
- translate the two setter functions into narrow Rust methods or module-level functions operating on that struct;
- represent C integer and flag fields with Rust integer/boolean types chosen to preserve range and behavior;
- replace implicit C memory/state assumptions with explicit ownership through `&mut` access;
- keep validation behavior aligned with the C implementation rather than inventing stricter policy.

This plan is intentionally constrained to the existing file and functions and does not introduce unrelated parsing, wrapping, concurrency, or serialization facilities.

## Technical Context

### Language/Version

- Rust 1.78+
  A current stable toolchain is sufficient; no nightly features are needed.

### Primary Dependencies

- Rust standard library only

Recommended crates: none.
There is no evidence in the input that third-party dependencies are necessary for these setter functions or the associated state migration.

### Testing

- `cargo test`

Testing should cover:

- direct migration parity for left margin updates;
- direct migration parity for right margin updates;
- repeated updates to ensure state replacement semantics match C behavior;
- edge conditions that are present in the original C logic, especially integer boundary and ordering cases if enforced by the source behavior.

### Performance Goals

Because this module only updates configuration state, performance goals are modest:

- constant-time margin updates;
- no heap allocation in setter paths unless the surrounding migrated state already requires allocation for unrelated fields;
- no additional copying beyond normal Rust mutable access.

The Rust port should remain operationally equivalent to the C setters and should not introduce measurable overhead beyond safe state access.

## Module Mapping

### Source Mapping

| C Source File | Rust Target |
|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` |

### Function Mapping

| C Function | Rust Mapping |
|---|---|
| `wordwrap_set_left_margin` | `WordWrap::set_left_margin` or `set_left_margin(&mut WordWrap, ...)` |
| `wordwrap_set_right_margin` | `WordWrap::set_right_margin` or `set_right_margin(&mut WordWrap, ...)` |

Preferred mapping is inherent methods on the migrated word-wrap state struct if the surrounding port already models the C context as a Rust type. If the branch structure is still function-oriented, module-level functions taking `&mut WordWrap` are acceptable. The choice should follow the existing Rust port style in this branch and should not create an extra abstraction layer solely for elegance.

### Visibility

- Keep visibility restricted to the minimum needed by current callers.
- If the C functions are internal to the parseopt subsystem, use `pub(crate)` rather than `pub`.
- Do not expose additional helper APIs unless they directly replace C-local helper logic needed by these two functions.

## Data Model

The analysis only identifies anonymous C data structures, so the plan should map only the state actually touched by the two setter functions and avoid speculative full-structure redesign.

### Primary State Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| Word-wrap state/context struct | `struct WordWrap` | Consolidates the migrated fields used by the setters. |
| Left margin field | integer field on `WordWrap` | Use a signed or unsigned Rust integer matching the effective C usage. Prefer `usize` only if the C code clearly treats margins as non-negative widths; otherwise use `i32`/`isize` for parity. |
| Right margin field | integer field on `WordWrap` | Same type decision as left margin for consistent comparisons and assignments. |
| Related mode/flag fields, if referenced by setters | `bool` or integer fields on `WordWrap` | Only migrate fields directly read or written by the two functions. |
| Optional sentinel values from C | `Option<T>` or retained integer sentinel | Prefer retaining integer sentinel semantics if the C code depends on exact numeric comparisons. Do not normalize unless behavior is unchanged. |

### Anonymous Struct Handling

Since the C analysis lists only anonymous structures, use this migration rule:

1. inspect `src/parseopt/wordwrap.c` for the concrete enclosing state type used by the setter functions;
2. identify the exact fields read/written by `wordwrap_set_left_margin` and `wordwrap_set_right_margin`;
3. model only those fields in the Rust struct if the broader word-wrap context has not yet been ported;
4. if a larger Rust `WordWrap` struct already exists in the branch, extend it minimally rather than creating a second parallel state type.

### Type Conversion Guidance

- C integral widths:
  - prefer `i32` when C uses `int` semantics and negative values may be meaningful;
  - prefer `usize` only when the source logic guarantees non-negative width-style values and no sentinel negatives are used.
- C flags:
  - map to `bool` when only truth-value behavior is needed;
  - keep numeric types if callers or comparisons rely on exact integer values.
- C pointers to mutable state:
  - map to `&mut WordWrap`.
- Nullability:
  - if the C setters assume non-null input, Rust should encode this by requiring a valid `&mut WordWrap` and omit nullable handling;
  - do not add `Option<&mut WordWrap>` unless null is a real input case in the original code path.

### Memory Management and Error Handling

- State ownership becomes explicit under Rust borrowing; no manual memory management is needed in the setter paths.
- If the C functions only assign values and do not report failure, keep Rust signatures infallible.
- If the C logic clamps, rejects, or normalizes invalid margin values, preserve that behavior directly:
  - use plain assignment if C does plain assignment;
  - return `Result` only if the original behavior has a real error outcome that callers observe.
- Avoid panics for invalid input unless the original C code treats such input as impossible internal misuse and the Rust call sites enforce that invariant.

## Implementation Phases

## Phase 1: Inspect and carve out the migrated state

### Goals

- Determine the exact state used by the two target functions.
- Establish the minimal Rust representation needed for direct migration.

### Tasks

- Review `src/parseopt/wordwrap.c` and locate:
  - the containing word-wrap state type;
  - field names touched by `wordwrap_set_left_margin`;
  - field names touched by `wordwrap_set_right_margin`;
  - any local validation or ordering logic between margins.
- Create or update `src/parseopt/wordwrap.rs`.
- Define the Rust state struct used by the setters:
  - either extend an existing `WordWrap` type in the branch;
  - or introduce the minimal struct required by current migrated callers.
- Choose Rust integer types to preserve C semantics exactly.

### Deliverables

- Rust module file with the migrated state definition.
- Field-level mapping notes in code comments only where needed to clarify non-obvious C parity decisions.

## Phase 2: Port the two setter functions

### Goals

- Translate the two C setters into Rust with behavior-preserving state updates.

### Tasks

- Implement Rust equivalents of:
  - `wordwrap_set_left_margin`
  - `wordwrap_set_right_margin`
- Preserve all source-level semantics:
  - assignment behavior;
  - boundary handling;
  - interaction between left and right margins if present;
  - any normalization performed by the C code.
- Keep signatures narrow and caller-oriented:
  - prefer `&mut self` methods if a struct-centric Rust port already exists;
  - otherwise use module functions receiving `&mut WordWrap`.
- Avoid introducing helper layers unless the C code already factors the logic or duplication is otherwise unavoidable.

### Deliverables

- Compiling Rust implementations of both setters.
- Removal or isolation of the corresponding C logic in the active Rust port path, according to branch conventions.

## Phase 3: Align callers and module boundaries

### Goals

- Ensure the migrated Rust functions are used in place of the C counterparts within the branch scope.

### Tasks

- Update existing Rust-side call sites, if already ported, to invoke the new setters.
- Keep module visibility minimal (`pub(crate)` if sufficient).
- Verify no parallel duplicate state is maintained between old and new representations.
- Confirm any existing defaults or constructors initialize the migrated margin fields consistently with C expectations.

### Deliverables

- Integrated Rust module usage for the two setter operations.
- Clean module boundary with no unnecessary exported API.

## Phase 4: Add parity tests and finalize

### Goals

- Lock in behavior for current scope and verify migration correctness.

### Tasks

- Add unit tests in the Rust module covering:
  - setting left margin from default state;
  - setting right margin from default state;
  - overwriting previously set left margin;
  - overwriting previously set right margin;
  - left/right interaction cases if the C code enforces constraints or normalization;
  - any edge numeric values observed in the C implementation.
- Run `cargo test`.
- Resolve any mismatches between Rust behavior and the original C logic without broadening the design.

### Deliverables

- Passing Rust unit tests.
- Finalized module migration for the two setter functions only.

## Notes and Constraints

- Do not redesign the word-wrap subsystem beyond what is needed to host these two functions.
- Do not introduce extra utility modules, builder APIs, validation frameworks, or formatting abstractions.
- Keep the migration focused on the existing file/function set and on exact behavior preservation.
- Prefer straightforward field mutation and explicit state ownership over generalized architecture changes.