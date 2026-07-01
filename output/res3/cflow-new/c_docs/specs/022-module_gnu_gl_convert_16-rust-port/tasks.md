# Tasks: module_gnu_gl_convert_16

## Phase 1: Setup

- [ ] [T001] [Story] Create the Rust module scaffold for `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, and expose it from the existing crate module tree in `src/gnu/mod.rs` or `src/lib.rs` as appropriate for `module_gnu_gl_convert_16`.
- [ ] [T002] [Story] Add placeholder item definitions in `src/gnu/stat_w32.rs` for the 3 module data structures and 2 functions identified from `gnu/stat-w32.c`, preserving the source-oriented naming and grouping needed for the port. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Implement the first foundational data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, including its Rust field layout and visibility needed by this module. Depends on: T002
- [ ] [T004] [P] [Story] Implement the second foundational data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, including its Rust field layout and visibility needed by this module. Depends on: T002
- [ ] [T005] [P] [Story] Implement the third foundational data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, including its Rust field layout and visibility needed by this module. Depends on: T002
- [ ] [T006] [Story] Reconcile the 3 data structures in `src/gnu/stat_w32.rs` by adding the enum/struct relationships, constructors, helper conversions, and internal type aliases required before function porting. Depends on: T003, T004, T005

## Phase 3: Functions

- [ ] [T007] [Story] Port the first function from `gnu/stat-w32.c` to `src/gnu/stat_w32.rs`, implementing the module’s core Windows-stat conversion behavior against the foundational Rust data structures. Depends on: T006
- [ ] [T008] [Story] Port the second function from `gnu/stat-w32.c` to `src/gnu/stat_w32.rs`, implementing the remaining conversion or support logic required by this module. Depends on: T006
- [ ] [T009] [Story] Integrate the two ported functions in `src/gnu/stat_w32.rs` by resolving shared internal helpers, call ordering, and module-local visibility so the converted functionality matches the original `gnu/stat-w32.c` responsibility without duplicating logic. Depends on: T007, T008

## Final Phase: Polish

- [ ] [T010] [Story] Refine `src/gnu/stat_w32.rs` by removing placeholder code, tightening signatures and visibility, and aligning comments and naming with the completed Rust port of `gnu/stat-w32.c`. Depends on: T009