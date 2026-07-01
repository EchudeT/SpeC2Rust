# Tasks: main_root_alignalloc.c_16

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `alignalloc.c` port on branch `017-main_root_alignalloc.c_16-rust-port`, adding the target source file at `src/alignalloc.rs` and declaring the module from `src/lib.rs` or `src/main.rs` according to the existing crate entry layout.
- [T002] [P] [Story] Add the initial public/internal item placeholders in `src/alignalloc.rs` for the four functions identified in `alignalloc.c`, preserving a one-to-one migration target for each C function.
- [T003] [Story] Review `alignalloc.c` and map each C function signature, ownership expectation, and allocation-related dependency into Rust implementation notes directly in `src/alignalloc.rs` comments to constrain the porting work.
  **Depends on:** T001, T002

## Phase 2: Foundational

- [T004] [Story] Establish the foundational Rust type aliases, constants, and helper definitions required by `alignalloc.c` inside `src/alignalloc.rs`, limited to items directly evidenced by the source module and needed by the function ports.
  **Depends on:** T003

## Phase 3: Functions

- [T005] [Story] Implement the low-level alignment/allocation helper function group from `alignalloc.c` in `src/alignalloc.rs`, covering only the functions that compute alignment boundaries or prepare allocation parameters before actual allocation/free logic.
  **Depends on:** T004
- [T006] [Story] Implement the allocation/deallocation function group from `alignalloc.c` in `src/alignalloc.rs`, porting the remaining functions that perform the aligned allocation lifecycle and wiring them to the foundational helpers.
  **Depends on:** T004, T005

## Final Phase: Polish

- [T007] [P] [Story] Refine `src/alignalloc.rs` by removing placeholder comments, tightening visibility to match module usage, and ensuring the final Rust code mirrors the original `alignalloc.c` behavior without introducing extra module scope.
- [T008] [Story] Perform a final integration pass on `src/alignalloc.rs` and the crate entry file updated in Phase 1 so the migrated `alignalloc.c` module builds cleanly within the Rust project branch.
  **Depends on:** T006, T007