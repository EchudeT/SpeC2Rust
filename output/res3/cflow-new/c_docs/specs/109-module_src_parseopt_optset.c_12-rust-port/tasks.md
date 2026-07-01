# Tasks: module_src_parseopt_optset.c_12

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the optset port by adding the target module file at `src/parseopt/optset.rs` and wiring it into the existing `src/parseopt/mod.rs` module tree for branch `109-module_src_parseopt_optset.c_12-rust-port`.
- [T002] [P] [Story] Review `src/parseopt/optset.c` and map the 14 C data structures and 2 functions to Rust items to be implemented in `src/parseopt/optset.rs`, documenting naming and ownership decisions inline as implementation comments where needed. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust representations for all 14 data structures from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, preserving the original module-local relationships, field intent, and visibility needed by the ported functions. Depends on: T002.
- [T004] [P] [Story] Add associated enums, type aliases, constants, and helper impl blocks in `src/parseopt/optset.rs` that are directly required to make the ported optset data structures usable by the upcoming function implementations. Depends on: T003.

## Phase 3: Function Implementation

- [T005] [Story] Implement the first optset-related function from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, porting its logic against the new Rust data structures without expanding behavior beyond the C module scope. Depends on: T004.
- [T006] [Story] Implement the second optset-related function from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, reusing the same foundational types and keeping the function aligned with the original module behavior. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/parseopt/optset.rs` by resolving compile-time ownership/borrowing issues, removing redundant porting scaffolding comments, and tightening intra-module visibility to match actual use after both functions and all data structures are in place. Depends on: T006.