# Tasks: main_root_safe_rw_15

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/safe_read.rs` to host the port of `safe-read.c` and `include/safe-read.c`.
- [T002] [Story] Expose the new module from `src/lib.rs` so the `safe_read` implementation is reachable from the crate root. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust error/result types in `src/safe_read.rs` needed to support the two safe-read functions during the port, keeping the definitions scoped only to behavior evidenced by `safe-read.c`. Depends on: T001

## Phase 3: Safe read functions

- [T004] [P] [Story] Implement the first safe-read function from `safe-read.c` in `src/safe_read.rs`, porting its read-loop and return-value behavior onto the Phase 2 Rust types. Depends on: T003
- [T005] [P] [Story] Implement the second safe-read function from `safe-read.c` in `src/safe_read.rs`, porting its read-loop and error-handling behavior onto the Phase 2 Rust types. Depends on: T003

## Final Phase: Polish

- [T006] [Story] Refine `src/safe_read.rs` and `src/lib.rs` for consistency with the surrounding Rust crate, removing migration leftovers and ensuring the exported API for the safe-read module is minimal and coherent. Depends on: T002, T004, T005