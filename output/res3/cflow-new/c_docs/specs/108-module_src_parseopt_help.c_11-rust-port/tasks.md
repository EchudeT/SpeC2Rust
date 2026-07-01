# Tasks: Rust port for `src/parseopt/help.c`

## Phase 1: Setup

- [T001] [Story] Create the Rust target module file for this port in `src/parseopt/help.rs`, mirroring the C source scope from `src/parseopt/help.c`.
- [T002] [Story] Wire the new `src/parseopt/help.rs` module into the existing Rust module tree from the nearest `src/parseopt/mod.rs` or equivalent parseopt module declaration point. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and map the 46 data structures referenced by `src/parseopt/help.c`, defining Rust-native type declarations, aliases, enums, constants, and placeholder structs directly required by this module in `src/parseopt/help.rs`. Depends on: T001.
- [T004] [P] [Story] Implement the module-local data structure conversions from the C model into Rust representations in `src/parseopt/help.rs`, preserving field layout intent and ownership assumptions only where evidenced by `src/parseopt/help.c`. Depends on: T003.
- [T005] [P] [Story] Add foundational helper declarations in `src/parseopt/help.rs` for any module-scoped state, lookup tables, or static descriptors required to support the help-processing logic in this port. Depends on: T003.
- [T006] [Story] Reconcile cross-structure relationships in `src/parseopt/help.rs` so all foundational types used by the module’s function compile together without introducing behavior beyond what is evidenced in `src/parseopt/help.c`. Depends on: T004, T005.

## Phase 3: Functions

- [T007] [Story] Port the single function implemented in `src/parseopt/help.c` into idiomatic Rust in `src/parseopt/help.rs`, using the established foundational types and preserving the original help/parseopt behavior. Depends on: T006.
- [T008] [Story] Resolve the function’s direct internal data access, control-flow translation, and return-value mapping in `src/parseopt/help.rs`, completing the migration of C-specific constructs into Rust equivalents within the same implementation task scope. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/help.rs` for compile cleanliness by removing temporary placeholders that are no longer needed after function porting and tightening visibility to the minimum required by the module. Depends on: T008.
- [T010] [Story] Perform a final module review of `src/parseopt/help.rs` and its parseopt module wiring to confirm the port remains scoped to `src/parseopt/help.c` and that all migrated structures and the function are consistently integrated. Depends on: T002, T009.