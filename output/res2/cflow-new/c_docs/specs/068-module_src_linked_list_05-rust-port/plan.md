# Implementation Plan: module_src_linked_list_05

## Summary

This module ports the linked-list and symbol-append logic from `src/linked-list.c` and `src/symbol.c` into Rust, preserving the existing behavior and migration scope without adding new capabilities. The Rust implementation should replace C-style manual allocation and pointer traversal with ownership-based data structures from the standard library, while keeping the original operation boundaries centered on:

- creating a linked-list instance,
- dereferencing or traversing linked-list content,
- appending symbol items into the list structure.

The preferred technical approach is to model the original C linked-list nodes and list container with explicit Rust structs, using heap allocation only where needed through `Box` and optional links through `Option`. If the original usage pattern only requires append and traversal, a minimal singly linked representation is sufficient. Error handling should be made explicit with `Option`/`Result` instead of null-pointer checks, but function behavior should stay aligned with the C module’s existing semantics.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Preserve the operational profile of the original C module for list creation, append, and traversal
- Avoid unnecessary cloning during symbol insertion
- Keep per-node allocation costs explicit and bounded to what is required by the original linked-list structure
- Maintain predictable linear traversal behavior for dereference/walk operations
- Prefer straightforward safe Rust first; only use interior mutability or more complex ownership patterns if required by the exact append behavior during migration

## Module Mapping

### Source File Mapping
- `src/linked-list.c` -> `src/linked_list.rs`
- `src/symbol.c` -> `src/symbol.rs`

### Function Mapping
- `linked_list_create` -> `linked_list::linked_list_create` or `LinkedList::new`
- `deref_linked_list` -> `linked_list::deref_linked_list` or a focused traversal/access helper on `LinkedList`
- `append_symbol` -> `symbol::append_symbol` or a symbol-specific append helper using the linked-list types

### Rust Module Layout
Keep the port constrained to the existing C file split:

- `src/linked_list.rs`
  - list container and node definitions
  - list creation logic
  - dereference/traversal helper logic

- `src/symbol.rs`
  - symbol-facing append logic
  - any symbol type adaptation required by the append path

- `src/lib.rs`
  - `mod linked_list;`
  - `mod symbol;`

If the project is already binary-oriented, expose the same modules through the existing crate root without adding new architectural layers.

## Data Model

The C analysis only identifies anonymous structures, so the Rust data model should be derived directly from field usage in `src/linked-list.c` and `src/symbol.c` during implementation. The mapping should remain minimal and local to the migrated files.

### Expected Structural Mapping

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| anonymous linked-list container struct | `struct LinkedList<T>` | Holds head pointer and any required tail/length metadata if present in C |
| anonymous linked-list node struct | `struct ListNode<T>` | Stores payload and next link |
| C pointer to next node | `Option<Box<ListNode<T>>>` | Replaces nullable node pointers safely |
| null pointer for empty list | `None` | Standard empty-state representation |
| symbol payload referenced by node | concrete Rust symbol type or borrowed/owned equivalent | Chosen based on actual field ownership in `symbol.c` |
| C out-parameter / returned pointer | direct return value, `Option`, or `Result` | Depends on whether C used null for failure or absence |

### Proposed Core Shapes

These shapes are planning defaults and should be adjusted only to match the original field usage:

```rust
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}
```

If append performance in the original C code depends on retaining a tail pointer, extend the list container only with that exact metadata. Do not add indexing caches or unrelated helpers.

### Memory Management Decisions
- Replace `malloc`/`free` ownership with RAII through Rust drop semantics
- Remove manual null checks in favor of `Option`
- Avoid shared ownership (`Rc`, `Arc`) unless the original code requires the same node to be owned from multiple places
- Use borrowing for traversal where possible to avoid moving symbol values out of the list
- If symbol append needs mutation through nested ownership, use `&mut` traversal first; use interior mutability only if direct borrowing cannot express the original control flow cleanly

### Error Handling Decisions
- Convert null-return or invalid-input cases into `Option` or `Result`
- Use `Result` only when the original function has an actual failure mode beyond “not found” / “empty”
- Keep error types local and minimal; avoid introducing broad custom error frameworks

## Implementation Phases

## Phase 1: Inspect and Define Rust Data Structures
- Read `src/linked-list.c` and `src/symbol.c` together to reconstruct the anonymous struct layouts actually used by:
  - `linked_list_create`
  - `deref_linked_list`
  - `append_symbol`
- Define the minimal Rust structs and enums needed to represent:
  - list container
  - node linkage
  - symbol payload involved in append operations
- Decide ownership for symbol data based on whether the C code stores raw references, copied values, or transferred allocations
- Create `src/linked_list.rs` and `src/symbol.rs` with type definitions and function signatures matching the migration target

## Phase 2: Port Core Linked-List Operations
- Implement `linked_list_create` as the Rust constructor path
- Implement `deref_linked_list` using safe traversal and explicit empty-list handling
- Preserve the original traversal semantics, including whether dereference returns:
  - the current node,
  - the payload,
  - or a nullable/optional view of list state
- Add focused unit tests for:
  - empty list creation
  - single-node traversal/dereference
  - multi-node traversal behavior as required by the original function behavior

## Phase 3: Port Symbol Append Logic
- Implement `append_symbol` in `src/symbol.rs` against the migrated linked-list structures
- Preserve insertion order and mutation semantics from the C code
- Ensure symbol append does not introduce extra ownership copies unless required by the source behavior
- Add unit tests covering:
  - append into empty list
  - append into non-empty list
  - append ordering
  - interaction between appended symbols and dereference/traversal behavior

## Phase 4: Final Integration and Cleanup
- Wire module exports through the crate root using the existing project structure
- Remove any temporary placeholders introduced during struct reconstruction
- Reconcile naming and visibility so functions remain usable where the original C module was consumed
- Run `cargo test` and fix borrow/ownership edge cases without changing scope
- Perform a final review to confirm:
  - no extra modules or capabilities were introduced,
  - memory ownership is explicit and safe,
  - behavior stays aligned with the original C implementation