# Implementation Plan: module_src_delete_level_12

## Summary

This module ports the level-deletion logic currently implemented in `src/symbol.c`, specifically the functions `delete_level_autos` and `delete_level_statics`, into Rust on branch `075-module_src_delete_level_12-rust-port`.

The Rust implementation should preserve the existing deletion behavior and traversal order of the C code while replacing manual memory management with ownership-based cleanup. The scope is limited to migrating the existing file-local logic and the data access patterns required by these two functions. The implementation should stay close to the current control flow so that behavior remains comparable during validation.

Technically, the port should:
- move the deletion routines into the Rust module corresponding to `src/symbol.c`
- translate C pointer-based structure traversal into `Option`, references, and owned container operations
- encode nullable links and conditional deletion paths explicitly in Rust types
- keep mutation localized to the symbol-storage structures already used by these routines
- replace implicit C failure modes with explicit, minimal Rust error handling only where needed by existing call boundaries

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the asymptotic complexity of the C implementation for level deletion
  - Avoid unnecessary cloning during symbol removal
  - Ensure deletion remains linear in the number of entries examined at the target level
  - Keep allocation churn limited to what is required by ownership-safe removal

## Module Mapping

| C Source | C Functions | Rust Target |
|---|---|---|
| `src/symbol.c` | `delete_level_autos` | `src/symbol.rs` as a direct function or `impl` method on the symbol table owner |
| `src/symbol.c` | `delete_level_statics` | `src/symbol.rs` as a direct function or `impl` method on the symbol table owner |

### Rust module structure

The Rust port should follow the existing project crate layout and place the migrated logic in the Rust equivalent of the symbol handling module:

- `src/symbol.rs`
  - migrated data types required by these deletion routines
  - `delete_level_autos`
  - `delete_level_statics`

If the surrounding port already centralizes symbol state in a struct, these functions should become methods on that struct rather than creating new helper modules.

## Data Model

The analysis identifies only anonymous C data structures, so the Rust plan should derive mappings from actual field usage inside `delete_level_autos` and `delete_level_statics`. The port should not invent broader abstractions beyond what these functions access.

### Mapping approach

| C Pattern | Rust Mapping |
|---|---|
| Anonymous struct used as symbol record | Named Rust `struct` with the accessed fields only, expanded as needed during port |
| Anonymous linked-list node | `struct` with `Option<Box<Node>>` for owned forward links, or index-based storage if already used by adjacent ported code |
| Nullable pointer field | `Option<T>` / `Option<Box<T>>` / `Option<NonNull<T>>` only if ownership is external |
| Integer level/category flags | `i32`, `u32`, or small `enum` depending on current C usage |
| Manual free of node chains | Automatic drop via ownership; explicit `take()`/drain-style removal during traversal |
| Shared global/static symbol storage | Fields on the existing Rust symbol-state owner struct |

### Planned Rust data definitions

Because the source analysis does not expose concrete struct names, the implementation should proceed by introducing named Rust types that mirror the C storage layout actually touched by the two functions:

- **Symbol entry struct**
  - represents one auto/static symbol entry
  - contains level information and any next-link or table-link needed for deletion
- **Level-scoped collection holder**
  - represents the storage being scanned or pruned
  - likely becomes part of the main symbol manager/table struct
- **Classification fields**
  - any C integer flags used to distinguish auto/static handling should remain primitive unless the C code clearly uses a closed set of values suitable for a Rust `enum`

### Memory management decisions

- Replace C `free`-driven unlinking with ownership-based removal.
- For singly linked structures, use cursor-style traversal with `Option<Box<Node>>` and `take()` to remove matching nodes safely.
- For vector- or table-backed storage, use `retain` or explicit drain/filter only if it preserves the original ordering and side effects.
- Avoid `Rc`, `Arc`, or interior mutability unless required by already-ported adjacent code.

### Error handling decisions

These deletion functions are expected to be primarily mutating procedures. Therefore:

- If the C functions are `void`-like and operate on valid in-memory state, prefer infallible Rust APIs.
- Use `Result` only when an existing Rust caller contract already requires it.
- Represent impossible/null-invalid states through type design where feasible rather than runtime checks.

## Implementation Phases

## Phase 1: Extract and map the C deletion paths

- Inspect `delete_level_autos` and `delete_level_statics` in `src/symbol.c`.
- Identify:
  - the exact storage roots each function mutates
  - the fields read for level comparison
  - unlink/free behavior
  - any side effects on counters, head pointers, or auxiliary links
- Define the minimum Rust structs/enums needed to represent those accessed fields.
- Place these definitions in `src/symbol.rs`, aligned with the surrounding symbol-state design already present in the Rust port.

**Exit criteria**:
- C control flow for both functions is fully mapped.
- Rust type skeletons exist for every field touched by the two functions.
- Ownership strategy for each linked/table structure is decided.

## Phase 2: Port `delete_level_autos` and `delete_level_statics`

- Implement `delete_level_autos` in Rust first, preserving:
  - traversal order
  - match conditions for target level
  - unlink behavior
  - updates to any head/root pointers or counters
- Implement `delete_level_statics` using the same direct-translation approach.
- Keep helper logic local to `src/symbol.rs`; do not introduce extra utility modules.
- Where C uses pointer rewiring, translate to:
  - `Option::take()` and reassignment for linked structures, or
  - bounded mutable iteration/removal for contiguous storage

**Exit criteria**:
- Both functions compile in Rust.
- No manual memory management remains in the ported logic.
- Behavior-affecting state mutations from the C implementation are represented.

## Phase 3: Integrate with existing symbol-state ownership

- Connect the ported functions to the existing Rust owner of symbol data.
- Remove any temporary duplication of state introduced during porting.
- Ensure call sites use mutable borrowing consistently and do not require unsafe code unless dictated by surrounding unported architecture.
- Confirm the Rust signatures match the intended internal calling style of the current branch.

**Exit criteria**:
- The functions are callable from the Rust symbol-management path.
- State ownership is coherent and compiles without workaround abstractions.
- Unsafe code is absent or narrowly justified.

## Phase 4: Validate behavior with focused tests

- Add `cargo test` coverage for:
  - deleting entries at a target level
  - leaving non-matching levels untouched
  - deleting from empty storage
  - deleting head/first elements and interior elements
  - repeated deletion calls on already-pruned levels
- Where practical, encode tests from observed C behavior rather than generalized new semantics.
- Verify ordering and remaining symbol visibility match the C logic after deletion.

**Exit criteria**:
- All focused deletion tests pass under `cargo test`.
- Edge cases around empty roots, single-entry chains, and consecutive matching entries are covered.
- The implementation remains within the original module scope only.