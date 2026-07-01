# Tasks: module_tilde Rust port

## Phase 1: Setup

- [T001] [Story] Create Rust module files for the `module_tilde` port by adding `src/tilde/mod.rs`, `src/tilde/shell.rs`, and `src/tilde/tilde.rs`, mirroring the source layout from `tilde/shell.c` and `tilde/tilde.c`.
- [T002] [P] [Story] Wire the new module into the crate root by exporting `src/tilde/mod.rs` from the existing Rust project entry module so later data structures and functions can be added in place.

## Phase 2: Foundational

- [T003] [Story] Define the 5 module data structures in Rust within the directly corresponding target files `src/tilde/shell.rs` and `src/tilde/tilde.rs`, preserving ownership and lifetime semantics needed by the C module interfaces. Depends on: T001, T002
- [T004] [Story] Add shared constructors, defaults, and internal helper type aliases required to initialize and pass the `module_tilde` data structures between `src/tilde/shell.rs` and `src/tilde/tilde.rs`. Depends on: T003

## Phase 3: Shell-related functions

- [T005] [Story] Port the shell-facing function group from `tilde/shell.c` into `src/tilde/shell.rs`, implementing the functions that operate on the module’s shell-specific data structures and state. Depends on: T004
- [T006] [P] [Story] Refine intra-module interfaces between `src/tilde/shell.rs` and `src/tilde/tilde.rs` so shell-related functions expose only the Rust-visible items required by the rest of `module_tilde`. Depends on: T005

## Phase 4: Tilde expansion functions

- [T007] [Story] Port the tilde expansion function group from `tilde/tilde.c` into `src/tilde/tilde.rs`, implementing the remaining module functions against the foundational data structures. Depends on: T004
- [T008] [P] [Story] Integrate calls between `src/tilde/tilde.rs` and `src/tilde/shell.rs` where the original C module shares shell lookup or expansion support across files. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Remove temporary placeholders and align naming, visibility, and module organization across `src/tilde/mod.rs`, `src/tilde/shell.rs`, and `src/tilde/tilde.rs` so the Rust port cleanly matches the original module boundaries. Depends on: T008
- [T010] [Story] Perform a final compile-focused cleanup of `module_tilde` in `src/tilde/shell.rs` and `src/tilde/tilde.rs`, simplifying obvious redundant conversions or allocations introduced during porting without changing behavior. Depends on: T009