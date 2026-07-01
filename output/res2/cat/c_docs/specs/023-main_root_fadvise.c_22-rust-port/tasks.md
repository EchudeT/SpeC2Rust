# Tasks: cat — main_root_fadvise.c_22

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/fadvise.rs` and declare it from the crate root so the port of `fadvise.c` has a dedicated target file on branch `023-main_root_fadvise.c_22-rust-port`.
- [T002] [P] [Story] Add the function skeletons corresponding to the two `fadvise.c` functions in `src/fadvise.rs`, preserving C-level responsibility boundaries for later implementation. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define any module-local constants, type aliases, and minimal helper signatures directly required by the `fadvise.c` port inside `src/fadvise.rs`, keeping foundational items in place before implementing function bodies. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the core file-advise behavior function from `fadvise.c` in `src/fadvise.rs`, mapping the original system-call-facing logic into Rust while preserving module scope and expected main-cluster behavior. Depends on: T003.
- [T005] [Story] Implement the remaining supporting `fadvise.c` function in `src/fadvise.rs`, completing the module’s two-function port and wiring it to the core advise logic as required by the original file structure. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/fadvise.rs` for idiomatic Rust within the existing ported behavior by removing placeholder code, tightening signatures, and simplifying control flow without changing functionality. Depends on: T005.