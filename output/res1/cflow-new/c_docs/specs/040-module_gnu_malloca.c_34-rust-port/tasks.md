# Tasks: module_gnu_malloca.c_34

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/malloca.c` migration in `src/gnu/malloca.rs`, and expose it from the existing `src/gnu/mod.rs` or `src/lib.rs` as applicable to the current crate layout.
- [T002] [P] [Story] Add the module file-level placeholders and migration comments in `src/gnu/malloca.rs` for the two functions from `gnu/malloca.c`, keeping names and scope aligned with the source module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish the foundational Rust representation needed by `gnu/malloca.c` in `src/gnu/malloca.rs`, limited to the allocation/lifetime helper types or internal aliases directly required by the module’s two functions. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the core stack-or-heap allocation behavior from `gnu/malloca.c` in `src/gnu/malloca.rs` for the primary allocation function, using the foundational items already introduced for this module. Depends on: T003.
- [T005] [Story] Implement the paired cleanup/free behavior from `gnu/malloca.c` in `src/gnu/malloca.rs` for the module’s second function, matching the allocation strategy established by the primary function. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/malloca.rs` by removing migration placeholders, tightening internal visibility, and verifying the module API wiring remains minimal and consistent with `gnu/malloca.c`. Depends on: T005.