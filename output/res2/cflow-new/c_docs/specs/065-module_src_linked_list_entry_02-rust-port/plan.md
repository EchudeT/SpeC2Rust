# Implementation Plan

## Summary

This module ports the linked-list and call-target processing logic currently implemented in `src/symbol.c` into Rust for branch `065-module_src_linked_list_entry_02-rust-port`.

The Rust implementation should preserve the existing behavior of the C routines:

- `collect_functions`
- `move_parms`
- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`

Technical approach:

- Migrate only the logic associated with these functions from `src/symbol.c`.
- Represent C linked-list entries and related traversal state with explicit Rust structs and enums.
- Replace raw-pointer list traversal with owned collections and index-/handle-based references where mutation across entries is required.
- Keep the implementation concentrated in a Rust module corresponding to the existing source area, avoiding extra architectural expansion.
- Preserve execution order and filtering behavior from C, while converting implicit null checks and mutable global-style state into explicit `Option`, slices, and mutable references.
- Use standard-library memory management (`Vec`, `Option`, `Box` only if strictly needed) and return `Result` only where the original behavior can fail in a meaningful way during migration.

## Technical Context

- **Language/Version**: Rust 1.78 or current stable toolchain compatible with the repository baseline
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on available module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear traversal characteristics of the C implementation for list walks and target marking
  - Avoid unnecessary cloning of symbol/function records during list reordering and filtering
  - Keep allocation bounded to Rust container replacements for existing C list nodes
  - Maintain comparable asymptotic behavior for starter iteration, caller marking, and non-target elimination

## Module Mapping

| C Source | C Functions | Rust Target | Notes |
|---|---|---|---|
| `src/symbol.c` | `collect_functions` | `src/symbol.rs::collect_functions` | Port collection pass logic directly, preserving traversal and insertion ordering |
| `src/symbol.c` | `move_parms` | `src/symbol.rs::move_parms` | Port parameter-transfer/relink logic with explicit mutable borrowing |
| `src/symbol.c` | `first_starter` | `src/symbol.rs::first_starter` | Implement starter initialization over Rust collection state |
| `src/symbol.c` | `next_starter` | `src/symbol.rs::next_starter` | Implement sequential starter iteration replacing pointer advancement |
| `src/symbol.c` | `mark_callers` | `src/symbol.rs::mark_callers` | Port recursive/iterative caller-marking logic with explicit visited state if needed |
| `src/symbol.c` | `eliminate_non_targets` | `src/symbol.rs::eliminate_non_targets` | Port filtering/removal pass using safe mutation of collection entries |

If the Rust project already exposes a crate root structure, keep this code under the existing module tree and map `src/symbol.c` to a single Rust source file `src/symbol.rs` rather than splitting into new submodules.

## Data Model

Because the analysis exposes only anonymous C data structures, the Rust port should begin by identifying the concrete structs used by the six target functions in `src/symbol.c` and mapping only those fields actually touched by this module.

### Data-structure mapping strategy

| C Pattern | Rust Mapping | Migration Notes |
|---|---|---|
| Anonymous struct used as linked-list node | Named Rust `struct` | Introduce a stable, descriptive name based on usage in `symbol.c` |
| `struct *next` singly linked pointer | `Option<usize>` or ordered `Vec<T>` traversal | Prefer index-based linkage when nodes must be referenced and mutated from multiple passes |
| Nullable pointer to associated record | `Option<usize>` or `Option<RecordRef>` | Use explicit optional references instead of null checks |
| C string pointer (`char *`) | `String` or borrowed `&str` during parsing boundary | Prefer owned `String` once data enters Rust-owned structures |
| Flag fields / boolean markers | `bool` or small `enum` | Replace integer flags with typed values where behavior is unambiguous |
| Ad hoc category/type codes | `enum` | Only when clearly inferable from the C usage in these functions |
| List head / traversal cursor globals | Struct-held state | Consolidate iteration state into explicit module context passed by mutable reference |

### Expected Rust structs

The exact names depend on the actual C field usage, but the port should likely define only the minimum required set such as:

- `FunctionEntry` for collected function symbols
- `ParameterEntry` for moved parameter nodes, if distinct in C
- `StarterCursor` or equivalent iterator state for `first_starter` / `next_starter`
- `SymbolGraph` or module-local context struct holding the function list and any caller/target markers

### Ownership and memory decisions

- Prefer `Vec<FunctionEntry>` as the backing store for entries collected from C list-building logic.
- Use index references between records instead of self-referential pointers.
- If the original code depends heavily on stable node identity during re-linking, use `Vec` plus indices first; use `Box` only if inspection of `symbol.c` shows unavoidable node-by-node ownership patterns.
- Avoid `Rc<RefCell<_>>` unless the original logic cannot be expressed with phased mutable passes; this should not be the default.
- Convert destructive list filtering into `retain`, compaction, or rebuild passes only if node-reference updates remain correct.

### Error handling mapping

- C null-pointer and empty-list conditions become `Option`-based control flow.
- If these functions only transform in-memory state and have no external failure mode, keep signatures infallible.
- If invalid cross-references can arise during migration, use narrow internal validation with `debug_assert!` and promote to `Result` only when the caller can act on the failure.

## Implementation Phases

### Phase 1: Extract and model the C state used by the target functions

- Inspect `src/symbol.c` and isolate all structs, globals, and helper routines directly touched by:
  - `collect_functions`
  - `move_parms`
  - `first_starter`
  - `next_starter`
  - `mark_callers`
  - `eliminate_non_targets`
- Name the anonymous C structs according to their role in these functions.
- Define minimal Rust structs/enums containing only the fields used by this module logic.
- Decide the backing storage shape for migrated linked lists:
  - `Vec<T>` with indices by default
  - preserve ordering semantics from C exactly
- Record invariants needed for safe mutation, especially for:
  - moving parameter entries
  - iterating starters across filtered collections
  - marking caller relationships without dangling references

### Phase 2: Port collection and traversal behavior

- Implement `collect_functions` in `src/symbol.rs` using the chosen Rust data model.
- Implement `move_parms` with explicit mutable access and no raw-pointer aliasing.
- Implement `first_starter` and `next_starter` over Rust collection state, preserving the C iteration order and termination behavior.
- Keep helper logic local to `src/symbol.rs` unless an existing crate module already owns the relevant types.
- Add focused unit tests for:
  - empty input
  - single-entry lists
  - multiple entries with starter iteration order
  - parameter movement preserving associations and ordering

### Phase 3: Port marking and elimination passes

- Implement `mark_callers` using explicit state transitions on Rust records.
- Implement `eliminate_non_targets` as a safe filtering/relinking pass.
- Ensure caller-mark propagation and target elimination preserve the same reachability results as C.
- Validate interactions between the earlier collection pass and later elimination pass, especially where removals could invalidate traversal assumptions.
- Add tests covering:
  - no targets found
  - all entries retained
  - mixed retained/eliminated entries
  - caller chains requiring repeated marking or traversal

### Phase 4: Integrate and verify module-level parity

- Wire the Rust `symbol.rs` module into the existing crate structure in place of the migrated C functionality.
- Remove any temporary migration scaffolding not required by the final module layout.
- Confirm that function signatures and state handoff align with the rest of the Rust port on this branch.
- Run `cargo test` and add regression cases derived from observed C edge paths in `src/symbol.c`.
- Perform a final review for:
  - no unchecked null-style assumptions
  - no unnecessary heap indirection
  - no capability expansion beyond the six migrated functions
  - preserved ordering and mutation semantics across all list-processing passes