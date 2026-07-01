# Implementation Plan: module_src_save_stack_14

## Summary

This module migration covers the stack-save helper logic currently located in `src/parser.c`, specifically the `save_stack` and `save_stack_is_empty` functions. The Rust implementation should preserve the existing parser-facing behavior while replacing C-style manual memory handling with ownership-based storage and explicit empty-state checks.

The implementation approach is to extract the stack-related state and operations into a focused Rust module within the parser area, using standard library collections to represent the saved parser stack entries. The port should remain narrow: migrate only the data and functions required by these two C functions, keep call patterns compatible with the surrounding parser logic, and avoid introducing broader parser redesign.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`Vec`, `Option`, slices, basic enums/structs)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s operational complexity for stack push/save and empty checks
  - Avoid unnecessary heap reallocations where the original C code reused or grew buffers predictably
  - Preserve linear memory layout for stack entries
  - Keep per-operation overhead minimal and suitable for parser hot paths

## Module Mapping

### C to Rust File Mapping

- `src/parser.c`
  - Migrate stack-save-related logic into `src/parser.rs`
  - If the Rust project already separates parser internals, place the implementation in the existing parser module file rather than creating new architectural layers

### Function Mapping

- `save_stack`
  - Port to a Rust method or private helper on the parser state structure
  - Recommended shape: `fn save_stack(&mut self, ...) -> Result<..., ...>` only if failure is possible in surrounding logic; otherwise keep infallible
- `save_stack_is_empty`
  - Port to a Rust method returning `bool`
  - Recommended shape: `fn save_stack_is_empty(&self) -> bool`

### Scope Restraint

- Only migrate the logic directly needed for stack state persistence and empty-state inspection
- Do not widen the responsibility into generic container utilities or parser framework abstractions
- Keep any integration points inside the parser module boundaries already implied by `parser.c`

## Data Model

The analysis only identifies anonymous C structures, so the Rust plan should derive concrete names from actual parser-local usage during implementation rather than inventing new domain concepts. The migration should follow these mapping rules.

### Data-Structure Mapping Rules

- **Anonymous C structs used only as grouped parser state**
  - Map to named Rust `struct`s with module-private visibility unless cross-function access requires broader visibility
- **Anonymous C records representing saved stack entries**
  - Map to a dedicated Rust `struct` for one stack element
- **C arrays / pointer-managed buffers for saved entries**
  - Map to `Vec<SaveStackEntry>`
- **Nullable pointer state**
  - Map to `Option<T>` or implicit emptiness via `Vec::is_empty()`, depending on original semantics
- **Integer flags**
  - Map to `bool` when the C usage is clearly binary; otherwise preserve integer type semantics
- **Indices / counts / capacities**
  - Map to `usize` unless exact-width integer behavior is required by surrounding parser code

### Expected Rust Structures

The exact field set should be taken from the C implementation, but the minimum expected model is:

```rust
struct ParserState {
    save_stack: Vec<SaveStackEntry>,
    // additional existing parser fields migrated from parser.c as needed by these functions
}

struct SaveStackEntry {
    // fields copied from the C saved-stack item layout
}
```

If the original C code stores save-stack metadata separately from the parser state, an alternative narrow mapping is acceptable:

```rust
struct SaveStack {
    entries: Vec<SaveStackEntry>,
}
```

with parser state owning `SaveStack`.

### Memory Management Decisions

- Replace any `malloc`/`realloc`/`free` behavior with `Vec` growth and drop semantics
- Preserve ordering and overwrite behavior exactly as in C
- If the C code reuses preallocated slots, use `Vec::push`, indexed assignment, or `truncate` to mirror that behavior without raw allocation management
- Avoid unsafe Rust unless the surrounding parser port already requires it for unrelated shared storage; these two functions should be implemented safely if possible

### Error Handling Decisions

- If the C implementation only failed on allocation, prefer infallible Rust logic and rely on standard allocation failure behavior
- If the original function reported explicit status codes for invalid parser state or capacity handling, map those to:
  - `bool` for simple success/failure checks already present in logic, or
  - a small internal `enum` error type if the call sites require branching on failure mode
- Do not introduce public error frameworks or external crates

## Implementation Phases

## Phase 1: Inspect and Name the Existing Stack-Save State

- Identify the exact C data involved in `save_stack` and `save_stack_is_empty`
- Determine:
  - where the saved stack is stored
  - the entry layout
  - whether there is separate count/capacity metadata
  - whether empty means null, zero count, or another sentinel
- Define Rust names for:
  - parser-owned save-stack container
  - saved stack entry type
  - any auxiliary metadata still required after conversion to `Vec`
- Add the corresponding structs/fields in `src/parser.rs` without extending parser responsibilities

**Exit criteria**:
- All C fields touched by the two target functions have a direct Rust home
- Empty-state semantics are documented in code comments for implementation use

## Phase 2: Port `save_stack_is_empty` and Container Semantics

- Implement the Rust empty-state check first, using the chosen representation
- Convert C null/count checks into a single Rust expression based on:
  - `Vec::is_empty()`, or
  - `Option` presence plus content emptiness if the original distinction matters
- Update internal call sites in the parser module to use the Rust method signature
- Add unit tests covering:
  - newly initialized state
  - one saved entry
  - state after clearing/truncation if that exists in current parser flow

**Exit criteria**:
- `save_stack_is_empty` behavior matches original parser expectations
- Empty and non-empty states are validated with `cargo test`

## Phase 3: Port `save_stack`

- Translate the C save logic into Rust using `Vec` operations
- Preserve:
  - push order
  - copied fields
  - any stack-depth or parser-position metadata
  - any replace/update behavior if the C function overwrote an existing slot
- Remove manual capacity bookkeeping where `Vec` makes it unnecessary, unless count fields are externally observed by adjacent parser code
- Keep the function private unless existing Rust parser organization requires wider visibility
- If the function returns status in C, keep only the minimum Rust return shape needed by its direct callers

**Exit criteria**:
- A saved stack entry in Rust contains all data required by downstream parser logic
- The function compiles with surrounding parser state and mirrors C behavior without raw memory management

## Phase 4: Integration Validation and Cleanup

- Verify the parser module compiles with the migrated save-stack state and functions
- Remove any now-redundant temporary count/capacity fields only if they are no longer used outside the migrated logic
- Add focused tests for:
  - repeated saves
  - save followed by empty check
  - edge cases from C logic such as initial empty buffer and first insertion
- Keep the cleanup local to `src/parser.rs`; avoid unrelated parser refactors

**Exit criteria**:
- Both functions are fully migrated and exercised by tests
- No manual memory-management patterns remain in the migrated save-stack path
- The Rust implementation stays confined to the existing parser module structure