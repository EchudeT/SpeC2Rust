# Tasks: main_root_xalloc-die.c_35

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module file for the `xalloc-die.c` port in `src/xalloc_die.rs`, reserving the module for the translated allocation-failure handler logic.
- [T002] [Story] Expose the new module from `src/lib.rs` or `src/main.rs` (whichever currently owns main-cluster module declarations) so the Rust port of `xalloc-die.c` is compiled and reachable.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Confirm that `xalloc-die.c` introduces no standalone data structures and keep `src/xalloc_die.rs` focused on function-only translation, with any needed imports and module-level constants aligned to the C source.
  - Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the allocation-failure termination function from `xalloc-die.c` in `src/xalloc_die.rs`, preserving the C module’s main-cluster behavior and mapping its process-exit/error-reporting logic into idiomatic Rust.
  - Depends on: T003
- [T005] [P] [Story] Wire any direct callers or module references in `src/lib.rs` or `src/main.rs` to use the Rust `xalloc_die` implementation in place of the C module boundary, keeping integration limited to this migrated file.
  - Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/xalloc_die.rs` and its export surface for minimal, clear imports and signatures, removing any migration scaffolding that is no longer needed after integration.
  - Depends on: T004, T005