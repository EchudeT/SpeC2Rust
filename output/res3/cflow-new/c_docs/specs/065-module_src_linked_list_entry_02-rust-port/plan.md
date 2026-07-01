# Implementation Plan: module_src_linked_list_entry_02

## Summary

This module ports the linked-list and call-target selection logic currently implemented in `src/symbol.c` into Rust on branch `065-module_src_linked_list_entry_02-rust-port`.

The Rust implementation should keep the existing behavior and traversal order of the C code while replacing implicit pointer-driven state with explicit Rust-owned and borrowed structures. The main technical approach is:

- migrate the relevant functions from `src/symbol.c` into a single Rust module with closely corresponding function boundaries;
- represent C linked-list nodes and mutable graph/list relationships with Rust structs using standard library containers;
- preserve in-place list filtering and traversal semantics where possible, but express them through safe iteration and index/key-based references instead of raw pointer mutation;
- isolate any unavoidable aliasing-sensitive operations behind narrow internal helper logic so the public module stays fully safe Rust.

The implementation should not broaden scope beyond the listed functions and their directly required supporting data structures.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the available input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain asymptotic behavior comparable to the C implementation for list traversal, caller marking, and target elimination
  - Avoid unnecessary cloning of symbol/function records during list rewrites
  - Keep traversal and filtering operations linear in the size of the relevant lists where the C code is linear
  - Prefer stable ownership and borrowing patterns that do not require repeated heap churn beyond what the original linked-list behavior implies

## Module Mapping

### Source File Mapping

- **C source**: `src/symbol.c`
- **Rust target**: `src/symbol.rs`

If the project already exposes module declarations from `src/lib.rs` or `src/main.rs`, this port should be wired there without creating extra architectural layers beyond what is needed to host the migrated code.

### Function Mapping

The following functions should be migrated with near one-to-one correspondence in responsibility and order:

- `collect_functions` -> `collect_functions`
- `move_parms` -> `move_parms`
- `first_starter` -> `first_starter`
- `next_starter` -> `next_starter`
- `mark_callers` -> `mark_callers`
- `eliminate_non_targets` -> `eliminate_non_targets`

### Migration Notes

- Keep the Rust function set limited to these functions plus only the minimal internal helpers needed to replace pointer-link manipulation safely.
- Preserve existing call sequencing dependencies between these functions rather than redesigning the module around new abstractions.
- Any C file-scope mutable state used by these functions should be converted into explicit module-local structs or explicit function parameters, depending on how tightly shared the state is across the listed functions.

## Data Model

The input analysis identifies only anonymous C structures, so the Rust plan should derive concrete names from actual usage in `src/symbol.c` during migration rather than inventing broad new domain models.

### Data-Structure Mapping Strategy

Because the C side appears to rely on anonymous structs and linked entries, map them according to their operational role:

- **Anonymous linked-list node carrying symbol/function metadata**
  - **C form**: anonymous struct with next-pointer fields and record payload
  - **Rust form**: named `struct` with explicit fields; replace raw next pointers with one of:
    - `Option<usize>` for index-linked storage, if the original code mutates list topology frequently; or
    - `Vec<T>` plus iteration/filtering, if stable order matters more than physical node identity
- **Anonymous starter/target/caller list entries**
  - **C form**: anonymous struct nodes connected by pointers
  - **Rust form**: dedicated named `struct` per role only if field sets differ materially; otherwise a shared record type plus role-specific iteration logic
- **Anonymous flag-bearing records**
  - **C form**: integer or bit-style fields embedded in list records
  - **Rust form**: `bool`, integer fields, or small enums, depending on actual usage in the C code
- **Anonymous optional relationships**
  - **C form**: nullable pointers
  - **Rust form**: `Option<usize>`, `Option<NonZeroUsize>`, or `Option<RecordId>` style references within module-local collections

### Recommended Rust Representation Rules

1. **Name structs by role, not by C anonymity**
   - Example categories likely needed during migration:
     - function/symbol entry
     - parameter entry
     - starter cursor/state
     - caller-marking state
   - Final names should be taken from field semantics in `src/symbol.c`.

2. **Avoid self-referential borrowed linked lists**
   - Do not mirror C raw-pointer chains with Rust references.
   - Prefer:
     - owned vectors of records, with indices for links; or
     - owned `Vec<Record>` plus temporary filtered index lists when eliminating non-targets.

3. **Represent destructive list edits safely**
   - C patterns that unlink nodes in place during traversal should become:
     - `Vec::retain` when node identity is not externally required; or
     - explicit cursor loops over index-linked storage when traversal order and in-place relinking are semantically important.

4. **Encode invalid/null states explicitly**
   - Nullable C pointers become `Option<_>`.
   - Sentinel integers should become `Option` or enums where the mapping is direct and local.

5. **Memory management**
   - Rust ownership should make lifetime of collected function and parameter records explicit.
   - No manual deallocation logic should be ported; dropping containers should replace `free` paths.
   - If the C code transfers node ownership between lists, Rust should model that as moving records or moving indices between collections, not duplicating records unless the C behavior requires copies.

6. **Error handling**
   - If the original C code assumes valid internal state, use internal assertions sparingly for invariants that should never fail after parsing/collection.
   - If function signatures already imply fallible operations in the Rust codebase, return `Result` only where actual failure modes are introduced by the port.
   - Do not introduce broad custom error layers unless required by existing project conventions.

### Provisional Mapping Table

Since the exact anonymous structs are not named in the analysis output, the migration should establish a table like the following during implementation and keep it local to this module:

| C anonymous structure role | Rust representation | Notes |
|---|---|---|
| Function/symbol linked entry | `struct FunctionEntry` | Holds symbol metadata, flags, and link/index fields |
| Parameter linked entry | `struct ParameterEntry` | Used by `move_parms` |
| Starter traversal state | `struct StarterState` or cursor fields | Supports `first_starter` / `next_starter` |
| Caller relationship entry | `struct CallerLink` or fields on `FunctionEntry` | Supports `mark_callers` |
| Target filtering state | flags on `FunctionEntry` or dedicated list | Supports `eliminate_non_targets` |

The exact field list should be derived strictly from the C file and limited to fields touched by the listed functions.

## Implementation Phases

## Phase 1: Extract and Model Existing State

- Inspect `src/symbol.c` and identify the exact structs, globals, and helper fields used by:
  - `collect_functions`
  - `move_parms`
  - `first_starter`
  - `next_starter`
  - `mark_callers`
  - `eliminate_non_targets`
- Create `src/symbol.rs` with Rust struct definitions for only the records required by those functions.
- Replace anonymous C structures with named Rust structs based on operational role.
- Decide per list whether to use:
  - `Vec<T>` with filtering/iteration; or
  - `Vec<T>` plus index links for C-like relinking behavior.
- Convert shared mutable file-scope state from C into explicit Rust module state or explicit parameters, whichever matches existing project structure with the least expansion.

### Phase 1 Exit Criteria

- All data needed by the six functions exists as Rust types.
- Nullability and ownership transfer cases are mapped to `Option` and move semantics.
- No raw-pointer-based design is carried over as the default representation.

## Phase 2: Port Collection and Traversal Functions

- Port `collect_functions` first, because it likely builds the working set consumed by later passes.
- Port `move_parms` next, preserving ownership transfer semantics for parameter records.
- Port `first_starter` and `next_starter` together, using a Rust cursor/index-based traversal model that preserves C iteration order.
- Add focused unit tests for:
  - function collection order
  - parameter movement behavior
  - starter iteration behavior across empty, singleton, and multi-entry cases

### Phase 2 Exit Criteria

- Collection and starter traversal behavior matches the C logic on representative fixtures.
- Traversal does not depend on aliasing-unsafe mutation.
- Tests cover list edge cases introduced by null/sentinel conversion.

## Phase 3: Port Caller Marking and Target Elimination

- Port `mark_callers`, mapping recursive or iterative caller propagation into safe Rust traversal over owned collections.
- Port `eliminate_non_targets`, preserving the exact filtering semantics and any ordering guarantees.
- Where the C code removes nodes during traversal, implement either:
  - `retain`-style filtering, if sufficient; or
  - explicit cursor-based relinking over index-linked storage, if removal semantics depend on node linkage state during iteration.
- Add tests covering:
  - marking reachable callers
  - preserving target entries
  - removing non-target entries without corrupting subsequent traversal
  - repeated passes, if the C logic relies on them

### Phase 3 Exit Criteria

- All six functions are implemented in Rust.
- Filtering and marking semantics match existing C behavior.
- No memory-leak or use-after-free concerns remain because ownership is fully expressed through Rust containers.

## Phase 4: Integrate and Validate Module Replacement

- Wire `src/symbol.rs` into the current Rust crate structure in place of the migrated C-backed logic.
- Reconcile function signatures with surrounding code, keeping interface changes minimal and local.
- Remove or quarantine superseded C-specific assumptions in the migrated path, such as manual node lifetime handling.
- Run `cargo test` and fix any mismatches in ordering, mutation timing, or optional-state handling discovered during integration.

### Phase 4 Exit Criteria

- The Rust module is the active implementation for this module scope.
- Tests pass under `cargo test`.
- The port remains constrained to the listed file and functions, without introducing unrelated infrastructure.