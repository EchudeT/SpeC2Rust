# Implementation Plan

## Summary

This module ports the linked-list and symbol append logic currently implemented in `src/linked-list.c` and `src/symbol.c` into Rust on branch `068-module_src_linked_list_05-rust-port`.

The Rust implementation should preserve the existing module scope and behavior by translating the current C linked-list operations into a minimal Rust-owned data model using standard library collections and ownership rules. The core technical approach is:

- replace manual pointer-linked allocation patterns with Rust-managed structures,
- migrate the list creation and traversal/dereference flow without adding new abstractions beyond what the existing functions require,
- keep symbol append behavior aligned with current call structure,
- encode failure cases explicitly with `Option` and `Result` instead of null-pointer and implicit error signaling.

The implementation should remain narrowly focused on the existing files and functions:

- `deref_linked_list`
- `linked_list_create`
- `append_symbol`

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Recommended minimum compiler: `rustc 1.76+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:
- None required based on current evidence

### Testing

- `cargo test`

Testing focus:
- linked-list creation from expected inputs
- dereference/traversal behavior for empty and non-empty lists
- symbol append behavior preserving order and ownership expectations
- edge cases previously represented in C by null pointers or empty structures

### Performance Goals

- Preserve asymptotic behavior of the current C implementation for creation, append, and dereference/traversal paths
- Avoid unnecessary heap allocations beyond those needed to represent the existing list nodes or equivalent owned storage
- Keep symbol append operations linear or better, matching the original implementation shape rather than introducing heavier abstractions
- Prefer straightforward ownership-based code over indirection layers

## Module Mapping

### C to Rust File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `src/linked-list.c` | `src/linked_list.rs` | Port list creation and dereference logic |
| `src/symbol.c` | `src/symbol.rs` | Port symbol append logic that depends on list representation |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `deref_linked_list` | `deref_linked_list` | Return borrowed or optional access rather than raw pointer dereference |
| `linked_list_create` | `linked_list_create` | Construct owned Rust list/container from current inputs |
| `append_symbol` | `append_symbol` | Mutating append using `&mut` and explicit ownership/borrowing |

### Module Integration

- Declare `mod linked_list;` and `mod symbol;` in the crate root only if not already present
- Keep cross-module interfaces minimal and shaped around the migrated functions
- Do not introduce extra helper modules unless required to break a direct cyclic dependency during migration

## Data Model

The source analysis only identifies anonymous C data structures, so the Rust data model should be derived directly from the structs used by these three functions rather than inventing new domain types.

### Data-Structure Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Anonymous struct used as linked-list node | Named Rust `struct` | Assign a stable descriptive name based on actual usage in the C file |
| Raw next-pointer chain | `Option<Box<Node>>` or `Vec<T>` | Choose the narrowest representation that matches actual access patterns in migrated functions |
| Nullable pointer return/input | `Option<T>` / `Option<&T>` / `Option<&mut T>` | Replace null checks with explicit optionality |
| Mutated pointer-owned object | Owned struct with `&mut` access | Avoid aliasing and manual lifetime management |
| Opaque or variant-like C fields | Rust `enum` only if the C code already implies tagged alternatives | Do not introduce enums without evidence from actual fields |

### Preferred Rust Structures

The implementation should choose one of the following based on the original C layout observed during porting:

1. **Node-preserving port**
   - Use a named node struct:
     - payload field(s) mapped from the C anonymous struct
     - `next: Option<Box<Node>>`
   - Best when `deref_linked_list` operates on explicit node chaining

2. **Container simplification port**
   - Use `Vec<T>` inside a small wrapper struct
   - Best only if the migrated functions treat the list as append/traverse storage without exposing node identity

Preference order:
- preserve node structure first if the C logic depends on pointer traversal,
- otherwise use `Vec<T>` to reduce memory-management complexity.

### Ownership and Memory Management

- Replace `malloc`/`free` ownership with Rust RAII
- Eliminate manual lifetime tracking of list nodes
- Convert pointer validity checks into type-checked borrowing
- Avoid `unsafe` unless the existing surrounding project API forces raw-pointer compatibility; if so, isolate it to the smallest possible boundary and keep all internal logic safe

### Error Handling

- Use `Option` where C used null as an expected absence case
- Use `Result` where creation or append can fail due to invalid state or required allocation/input validation
- Keep error types local and minimal; prefer simple enums in the target module over project-wide error frameworks

## Implementation Phases

## Phase 1: Inspect and Name the Existing C Data Shapes

### Goals
- Extract the exact anonymous structs used by `linked_list_create`, `deref_linked_list`, and `append_symbol`
- Determine whether the list must remain node-based or can be represented as a simple owned sequence
- Identify nullability and mutation expectations at each function boundary

### Tasks
- Read `src/linked-list.c` and `src/symbol.c` together to identify shared struct usage
- Assign stable Rust names to each anonymous struct directly involved in these functions
- Document field-by-field C-to-Rust type mapping before writing code
- Confirm whether `append_symbol` appends nodes, payload values, or symbol records into an existing list structure

### Deliverables
- Rust type skeletons in `src/linked_list.rs` and `src/symbol.rs`
- Clear signature decisions for the three target functions

## Phase 2: Port Linked-List Construction and Dereference Logic

### Goals
- Implement Rust-owned list creation
- Implement dereference/traversal behavior without raw-pointer semantics in internal code

### Tasks
- Port `linked_list_create` into `src/linked_list.rs`
- Port `deref_linked_list` into `src/linked_list.rs`
- Replace null-return and null-input paths with `Option`/`Result`
- Keep naming aligned with the original functions unless crate style requires scoped visibility changes
- Add unit tests for:
  - empty input behavior
  - single-element list behavior
  - multi-element traversal/dereference behavior

### Deliverables
- Compiling Rust implementation of list creation and dereference
- Focused tests covering migrated list behavior

## Phase 3: Port Symbol Append Logic

### Goals
- Migrate `append_symbol` to use the Rust list/data model finalized in Phase 2
- Preserve mutation order and symbol ownership semantics

### Tasks
- Port `append_symbol` into `src/symbol.rs`
- Update call sites or module imports as needed to use the Rust list types
- Resolve ownership of symbol data during append without cloning unless required by the original call pattern
- Add unit tests for:
  - appending into empty state
  - appending into existing list/container
  - order preservation after multiple appends

### Deliverables
- Compiling Rust append implementation
- Tests verifying append semantics against expected list state

## Phase 4: Integration Cleanup and Behavioral Verification

### Goals
- Ensure the migrated module is consistent, minimal, and free of leftover C memory assumptions

### Tasks
- Remove transitional placeholders and tighten visibility (`pub` only where needed)
- Recheck all APIs for unnecessary allocation, cloning, or exposed internals
- Run `cargo test` and fix mismatches in optional/error behavior
- Verify no extra facilities or abstractions were introduced beyond the migrated module scope

### Deliverables
- Final Rust module pair integrated into the crate
- Passing `cargo test`
- Minimal, maintainable port aligned to the original C file boundaries