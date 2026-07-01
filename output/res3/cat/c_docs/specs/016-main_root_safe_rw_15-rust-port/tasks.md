# Tasks: main_root_safe_rw_15

## Phase 1: Setup

- [T001] [Story] Create the Rust module files for the `safe-read` port in `src/safe_read.rs` and register the module from `src/main.rs` so the branch has a concrete target for `include/safe-read.c` and `safe-read.c` migration.
- [T002] [Story] Define the public function signatures in `src/safe_read.rs` for the two functions migrated from `safe-read.c`, matching the C module split and expected call surface for the main cluster port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish the shared Rust-side aliases, imports, and internal helper constants needed by the `safe-read` function implementations in `src/safe_read.rs`, keeping all foundational items local to this module file. Depends on: T002

## Phase 3: Safe read/write function port

- [T004] [Story] Port the safe read functionality from `safe-read.c` into `src/safe_read.rs`, preserving the original retry and error-propagation behavior expected by the module interface. Depends on: T003
- [T005] [P] [Story] Port the safe write functionality from `safe-read.c` into `src/safe_read.rs`, preserving the original retry and error-propagation behavior expected by the module interface. Depends on: T003

## Final Phase: Polish

- [T006] [Story] Refine `src/safe_read.rs` to remove migration scaffolding, align naming and visibility with the Rust project conventions, and ensure the registered module usage in `src/main.rs` is clean and minimal. Depends on: T004, T005