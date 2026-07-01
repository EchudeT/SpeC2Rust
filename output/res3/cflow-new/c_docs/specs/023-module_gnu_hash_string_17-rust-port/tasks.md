# Tasks: module_gnu_hash_string_17

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for `gnu/hash.c` in `src/gnu/hash.rs`, and expose it from `src/gnu/mod.rs` and the crate root module file if needed by the existing project layout.
- [ ] T002 [P] [Story] Add placeholder Rust items in `src/gnu/hash.rs` for the module-local data structures and the 2 function ports identified from `gnu/hash.c`, preserving source-level grouping for later implementation. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Port the foundational data structure definitions referenced by `gnu/hash.c` into Rust in `src/gnu/hash.rs`, covering the module-local structs, enums, aliases, constants, and field layouts required before function translation. Depends on: T002
- [ ] T004 [Story] Implement shared helper type definitions and internal representation details needed by the `gnu/hash.c` data structures in `src/gnu/hash.rs`, keeping naming and ownership decisions aligned with the C module’s usage patterns. Depends on: T003
- [ ] T005 [Story] Reconcile all 49 referenced data-structure definitions in `src/gnu/hash.rs` so the module compiles with complete type coverage before function implementation begins. Depends on: T003, T004

## Phase 3: Functions

- [ ] T006 [Story] Implement the GNU hash-string computation function port from `gnu/hash.c` in `src/gnu/hash.rs`, wiring it to the Rust data structures and preserving the original control flow and value semantics. Depends on: T005
- [ ] T007 [Story] Implement the remaining hash-module function from `gnu/hash.c` in `src/gnu/hash.rs`, grouping it with the string-hash logic only where they share direct internal representations. Depends on: T005
- [ ] T008 [Story] Integrate the 2 translated functions with the finalized module interfaces in `src/gnu/hash.rs`, resolving signature alignment, internal helper usage, and any compile-time mismatches introduced during the direct port. Depends on: T006, T007

## Final Phase: Polish

- [ ] T009 [Story] Refine `src/gnu/hash.rs` by removing translation placeholders, tightening visibility to module-appropriate scope, and simplifying obvious C-to-Rust port artifacts without changing behavior. Depends on: T008
- [ ] T010 [Story] Perform a final compile-oriented cleanup of `src/gnu/hash.rs` and related module declarations touched during the port so the migrated `gnu/hash.c` module is consistently organized within the Rust project branch. Depends on: T009