# Task List: `main_root_localcharset.c_20`

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/localcharset.rs` for the port of `localcharset.c`, and register it from the crate root in the existing project entry file so the `020-main_root_localcharset.c_20-rust-port` branch can compile with the new module in place.
- [T002] [P] [Story] Establish the module skeleton in `src/localcharset.rs` with placeholders for the 8 data structures and the module function ported from `localcharset.c`, keeping names and responsibilities aligned with the source module.

## Phase 2: Foundational

- [T003] [Story] Implement the 8 data structures required by `localcharset.c` in `src/localcharset.rs`, translating the C module’s state carriers, tables, and record layouts into Rust-native types appropriate for this module. Depends on: T001, T002.
- [T004] [P] [Story] Refine ownership, visibility, and internal construction patterns for the `src/localcharset.rs` data structures so the later function port can use them without unsafe C-style layout assumptions. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the module’s single function from `localcharset.c` into `src/localcharset.rs`, wiring it to the previously implemented 8 data structures and preserving the original module behavior in Rust. Depends on: T003, T004.
- [T006] [Story] Complete function integration cleanup in `src/localcharset.rs` by resolving return types, constant/table access, and any module-local helper organization needed for the ported `localcharset.c` function to build cleanly. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Perform module-level polish in `src/localcharset.rs` by removing placeholder code, simplifying obvious C-to-Rust translation artifacts, and ensuring the `localcharset.c` port remains concise and idiomatic without changing behavior. Depends on: T006.
- [T008] [Story] Verify crate-level compilation after integrating `src/localcharset.rs`, fixing any remaining module registration or type consistency issues in the directly affected Rust entry file and `src/localcharset.rs`. Depends on: T007.