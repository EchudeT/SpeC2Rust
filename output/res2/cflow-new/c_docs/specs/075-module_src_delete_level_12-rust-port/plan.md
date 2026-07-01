# Implementation Plan: module_src_delete_level_12

## Summary

This module migrates the `src/symbol.c` logic associated with:

- `delete_level_autos`
- `delete_level_statics`

The Rust implementation should preserve the existing deletion behavior for symbol data associated with a given level, with particular attention to ownership transfer, removal order, and any implicit list traversal semantics present in the C code.

The technical approach is to port the existing symbol-table deletion routines directly into a Rust module using standard-library collections and explicit ownership. The implementation should avoid introducing new abstractions beyond what is required to represent the current C data flow. The focus is on:

- translating level-based symbol removal into safe Rust iteration and mutation,
- replacing manual memory release with ownership-driven drop behavior,
- preserving any distinctions between automatic-storage and static-storage symbol handling,
- keeping the migrated logic close to the current file/function boundaries in `src/symbol.c`.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain effectively linear deletion behavior relative to the number of symbols examined.
  - Avoid unnecessary cloning during symbol removal.
  - Preserve in-place mutation patterns where feasible using standard collection APIs.
  - Keep allocation overhead no worse than the original practical behavior of the C implementation.

## Module Mapping

### C to Rust File Mapping

- `src/symbol.c` -> `src/symbol.rs`

### Function Mapping

- `delete_level_autos` -> `delete_level_autos`
- `delete_level_statics` -> `delete_level_statics`

### Rust Module Placement

Use a single Rust source file matching the C source responsibility:

- `src/symbol.rs`

If the project already exposes a symbol module from `lib.rs` or `main.rs`, wire this file into the existing module tree without creating extra helper modules unless already required by the current Rust project layout.

## Data Model

The analysis only identifies multiple anonymous C data structures and does not provide field layouts. The migration plan should therefore preserve structure boundaries by deriving Rust data types from the actual definitions in `src/symbol.c` and its directly used headers, limiting changes to the minimum needed for safe ownership.

### C Struct to Rust Type Mapping Strategy

For each anonymous C structure used by these functions:

- `anonymous` -> named Rust `struct` with a role-based name derived from usage in `symbol.c`
- If a structure functions as a tagged variant in C -> Rust `enum`
- If a structure is only linked-list glue or a node wrapper -> Rust `struct` holding owned payload and next/index linkage as required by the existing algorithm

### Expected Representation Decisions

Because these functions are deletion-oriented, the important mappings are:

- **Raw owning pointers** -> owned Rust values (`Box<T>`, `Vec<T>`, `LinkedList<T>`, or map entry ownership), chosen only according to the existing storage shape
- **Non-owning pointers** -> references or stable identifiers/indexes, depending on mutation constraints
- **Nullable pointers** -> `Option<T>`
- **Manual free/delete paths** -> implicit `Drop` through container removal
- **C integer level markers** -> integer field in the migrated Rust struct, likely `usize` or `i32`, matching actual C semantics
- **Storage-class distinctions** used by the two functions -> either separate collections or a discriminant field, based strictly on the current C organization

### Data-Structure Migration Rules

1. Name each anonymous C structure after its actual role in symbol management once the source is inspected.
2. Keep field ordering and semantics close to C during the initial port.
3. Prefer `Option<Box<Node>>` only if the original logic truly depends on pointer-chained lists.
4. Prefer `Vec<T>` retention with `retain`, `drain_filter`-style equivalent patterns, or indexed removal only if the C logic is logically collection-based rather than pointer-topology-based.
5. Do not redesign the symbol table; only encode enough structure to support the existing delete-level operations safely.

## Implementation Phases

## Phase 1: Inspect and Define Rust Data Structures

- Review `src/symbol.c` and any directly referenced headers to identify:
  - the symbol record layout,
  - storage for automatic and static symbols,
  - level-tracking fields,
  - any side effects performed during deletion.
- Introduce Rust structs/enums in `src/symbol.rs` corresponding to the C anonymous structures actually touched by:
  - `delete_level_autos`
  - `delete_level_statics`
- Replace nullable links and ownership-bearing pointers with:
  - `Option`
  - owned container elements
  - references/indexes only where mutation permits
- Keep names and field groupings aligned with the original C responsibilities.

## Phase 2: Port Deletion Logic

- Implement `delete_level_autos` in Rust first, preserving:
  - traversal order,
  - deletion criteria by level,
  - any updates to head pointers, current scope state, or adjacent links,
  - any associated cleanup on symbol removal.
- Implement `delete_level_statics` using the same approach, preserving any differences in:
  - targeted storage,
  - symbol qualification,
  - cleanup behavior.
- During the port:
  - avoid cloning symbol payloads unless required by borrow rules,
  - use container removal semantics that preserve correctness over structural elegance,
  - ensure removal does not leave invalid internal references.

## Phase 3: Integrate Error and Ownership Semantics

- Replace C assumptions about valid pointers with explicit Rust state handling:
  - `Option` for absent nodes/entries,
  - exhaustive matching for storage variants if applicable.
- Where C code silently no-ops on missing data, preserve that behavior unless the surrounding Rust code already uses `Result` for module-local invariants.
- Ensure all removed symbols are dropped exactly once through container ownership.
- Confirm there are no remaining aliasing patterns that would require unsafe code; if unsafe becomes unavoidable for layout parity, isolate it narrowly and document the reason in code comments.

## Phase 4: Add Focused Tests and Validate Behavior

- Add unit tests in the existing Rust test style covering:
  - deletion of symbols at the target level,
  - retention of symbols from other levels,
  - empty collection behavior,
  - repeated deletion calls,
  - mixed automatic/static populations if both functions interact with shared state.
- Validate edge cases from the original C behavior, especially:
  - deleting the first element,
  - deleting consecutive matching elements,
  - deleting all elements at a level,
  - leaving unrelated symbol state unchanged.
- Run `cargo test` and adjust the implementation only to restore parity with the C behavior, not to introduce broader refactoring.