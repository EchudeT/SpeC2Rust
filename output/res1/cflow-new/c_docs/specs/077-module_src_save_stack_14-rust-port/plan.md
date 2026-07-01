# Implementation Plan

## Summary

Port the C stack-saving logic from `src/parser.c` into a focused Rust module that preserves current behavior for `save_stack` and `save_stack_is_empty` without extending scope. The Rust implementation should model the original stack state explicitly, replace raw memory handling with ownership-based containers from the standard library, and keep interfaces narrow so the migrated code remains aligned with the existing parser flow.

The technical approach is:

- extract the stack-related state and operations from the parser area into a Rust module dedicated to this migrated functionality;
- represent the saved stack as a Rust-owned collection, most likely `Vec<T>` or `Option<Vec<T>>` depending on whether the C code distinguishes “uninitialized” from “empty”;
- translate C null/empty checks into explicit Rust state checks;
- keep error behavior simple and local, using `Result` only where allocation or invariant validation must be surfaced;
- migrate only the data and helper logic required by `save_stack` and `save_stack_is_empty`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the current asymptotic behavior of stack save and emptiness checks;
  - avoid unnecessary copying beyond what the C implementation already requires for stack preservation;
  - use contiguous storage (`Vec`) where the original logic is array- or node-list-like but consumed as a linear saved stack;
  - maintain predictable memory lifetime and eliminate manual free/realloc patterns.

## Module Mapping

### C to Rust File Mapping

- `src/parser.c`
  - migrate stack-save functionality into `src/parser.rs` if the parser remains a single Rust module;
  - if the parser port already uses submodules, place only this migrated logic in `src/parser/save_stack.rs` and re-export it from `src/parser/mod.rs`.

Preferred mapping should follow the existing Rust parser layout on branch `077-module_src_save_stack_14-rust-port`. Do not introduce new top-level crates or unrelated support modules.

### Function Mapping

- `save_stack`
  - map to a Rust method or free function named `save_stack`
  - preferred as a method on the parser state if the C function mutates parser-owned save-stack state
- `save_stack_is_empty`
  - map to a Rust method or free function named `save_stack_is_empty`
  - implement as a read-only state check over the migrated saved-stack representation

## Data Model

Because the source analysis identifies only anonymous C data structures, the Rust mapping should be driven by actual field usage within `save_stack` and `save_stack_is_empty`, not by inventing broader types.

### Data-Structure Mapping Strategy

- **C anonymous parser-owned state**
  - **Rust**: named `struct` embedded in the parser state or a small dedicated `SaveStackState`
  - Purpose: hold the saved stack container and any counters/markers accessed by the migrated functions

- **C anonymous stack element record**
  - **Rust**: named `struct SaveStackEntry`
  - Purpose: represent one saved unit from the parser stack
  - Fields: only those read or written by `save_stack`

- **C anonymous linked or array-based storage**
  - **Rust**: `Vec<SaveStackEntry>` by default
  - If the C logic stores optional absence distinctly from an empty allocation, use `Option<Vec<SaveStackEntry>>`

- **C pointer/null state**
  - **Rust**: `Option<T>` or an empty `Vec<T>`, selected based on whether null has semantic meaning separate from empty

- **C integer counters/lengths**
  - **Rust**: `usize` for indexes and lengths
  - If the C code uses sentinel negative values, map to `Option<usize>` or a small enum instead of signed indexing where possible

- **C ownership of copied stack memory**
  - **Rust**: owned values inside `Vec`, cloned only when the C implementation truly copies stack content
  - Avoid borrowed references for saved stack entries if the saved state must outlive transient parser operations

### Memory Management Decisions

- Replace manual allocation, resizing, and freeing with `Vec` lifecycle management.
- Replace null-pointer checks with `Option` or `Vec::is_empty`.
- Ensure that any data copied into the saved stack is owned by the save-stack structure, matching the C code’s lifetime semantics.
- Avoid unsafe code unless the surrounding parser port already requires it for direct layout compatibility; these two functions should prefer safe Rust.

### Error Handling Decisions

- If `save_stack` in C cannot fail except by allocation failure, keep the Rust API simple:
  - return `()` if the surrounding parser code treats this as infallible;
  - return `Result<(), SaveStackError>` only if the existing Rust parser layer already propagates recoverable internal errors.
- `save_stack_is_empty` should return `bool`.

## Implementation Phases

## Phase 1: Extract and map current stack-save state

- Inspect `src/parser.c` and identify:
  - the concrete fields touched by `save_stack`;
  - the exact condition used by `save_stack_is_empty`;
  - whether the saved stack is represented as nodes, arrays, or pointer/count pairs.
- Define minimal Rust names for the anonymous C structures actually involved:
  - parser-owned save-stack state;
  - saved stack entry type.
- Place these definitions in the parser Rust file that corresponds to the current parser port layout.
- Preserve field semantics exactly; do not normalize or redesign unrelated parser state.

## Phase 2: Implement `save_stack_is_empty` and state invariants

- Implement `save_stack_is_empty` first as a direct translation of the C emptiness logic.
- Encode the relevant invariant in Rust:
  - empty container means no saved entries;
  - optional uninitialized state is distinct only if required by the C behavior.
- Add unit tests covering:
  - newly initialized state;
  - state after explicit clear/reset if such behavior exists in the parser state;
  - non-empty state after manual fixture setup.

## Phase 3: Implement `save_stack`

- Translate `save_stack` using the chosen owned container representation.
- Preserve:
  - element ordering;
  - count updates;
  - replacement vs append semantics;
  - any reset-before-save behavior present in the C function.
- Replace C memory operations with:
  - `Vec::clear`, `Vec::push`, `Vec::extend`, or direct construction as appropriate;
  - cloning/copying only for fields the C function duplicates.
- If the C function depends on parser-stack traversal, port only that traversal logic required to materialize the saved snapshot.

## Phase 4: Validate integration and behavior parity

- Wire the migrated functions into the existing Rust parser state and call sites.
- Add focused tests for:
  - empty check before and after save;
  - repeated save operations;
  - edge case of saving an empty parser stack if allowed by the original C logic;
  - preservation of element order and count.
- Confirm there is no remaining dependence on manual lifetime management for this module slice.