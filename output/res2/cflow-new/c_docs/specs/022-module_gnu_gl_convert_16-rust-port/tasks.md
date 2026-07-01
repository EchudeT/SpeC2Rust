# Tasks: module_gnu_gl_convert_16

## Phase 1: Setup

- [ ] T001 [Story] Initialize the Rust module scaffold for `module_gnu_gl_convert_16` in `src/gnu/stat_w32.rs`, mirroring the source scope from `gnu/stat-w32.c`.
- [ ] T002 [Story] Expose the new module from the Rust crate root by wiring `src/gnu/mod.rs` to include `stat_w32`.

## Phase 2: Foundational

- [ ] T003 [Story] Identify and define the 3 data structures required by `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, preserving the C module’s data layout and field intent needed by the conversion logic.
- [ ] T004 [P] [Story] Add foundational associated constructors or helper mappings for the new data structures in `src/gnu/stat_w32.rs` where directly required to support the module’s 2 function implementations. Depends on: T003

## Phase 3: Stat conversion functions

- [ ] T005 [Story] Implement the first Windows stat conversion function from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, using the Phase 2 data structures as its direct Rust backing. Depends on: T003, T004
- [ ] T006 [Story] Implement the second Windows stat conversion function from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, grouped with the same conversion pathway and reusing the shared foundational structures. Depends on: T003, T004, T005

## Final Phase: Polish

- [ ] T007 [Story] Refine `src/gnu/stat_w32.rs` to remove duplication between the two conversion functions, tighten visibility to module-appropriate scope, and align naming/documentation with the migrated `gnu/stat-w32.c` behavior. Depends on: T005, T006