# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `xalloc-die.c` migration in `src/xalloc_die.rs`, establishing the target location for the ported implementation.
- [T002] [Story] Wire the new module into the crate from `src/main.rs` or `src/lib.rs` by declaring `mod xalloc_die;` or the equivalent existing module inclusion pattern so the migrated code is compiled.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review the C module scope in `src/xalloc_die.rs` and define the minimal Rust items needed to support the migrated function, keeping signatures and visibility limited to what `xalloc-die.c` evidences.
  - Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the `xalloc_die` function in `src/xalloc_die.rs`, porting the behavior from `xalloc-die.c` and preserving its main-cluster termination/allocation-failure handling role.
  - Depends on: T003
- [T005] [P] [Story] Update call sites in `src/main.rs` or `src/lib.rs` as needed to use the Rust `xalloc_die` implementation from `src/xalloc_die.rs`, keeping changes limited to integration required by this module migration.
  - Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/xalloc_die.rs` and its crate integration for idiomatic Rust naming, visibility, and imports, and remove any migration-only scaffolding that is no longer needed after `xalloc_die` is connected.
  - Depends on: T004, T005