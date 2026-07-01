# Tasks: module_gnu_hash.c_31 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/hash.rs` and declare it from the existing parent module so the port of `gnu/hash.c` has a dedicated target location on branch `037-module_gnu_hash.c_31-rust-port`.
- [T002] [P] [Story] Establish the skeleton of `src/gnu/hash.rs` with public/private section layout for constants, data structures, and function groups corresponding to `gnu/hash.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and translate the C data definitions used by `gnu/hash.c` into Rust-native type aliases, enums, structs, and helper constants in `src/gnu/hash.rs`, preserving module-local visibility and ownership semantics needed by the ported functions. Depends on: T002.
- [T004] [Story] Implement the core GNU-hash-related record/container structures required by the module logic in `src/gnu/hash.rs`, including field mapping from the C layout where function behavior depends on it. Depends on: T003.
- [T005] [P] [Story] Add foundational conversion/access helper methods on the translated structures in `src/gnu/hash.rs` only where directly needed to support the function port from `gnu/hash.c`. Depends on: T004.

## Phase 3: Hash computation functions

- [T006] [Story] Port the GNU hash value computation function(s) from `gnu/hash.c` into `src/gnu/hash.rs`, keeping behavior aligned with the original algorithm and using the foundational types from Phase 2. Depends on: T004.
- [T007] [P] [Story] Port any small supporting helper function(s) directly tied to hash computation in `src/gnu/hash.rs`, without expanding beyond the functions present in `gnu/hash.c`. Depends on: T006.

## Phase 4: Table construction and preparation functions

- [T008] [Story] Port the function group in `gnu/hash.c` responsible for initializing or preparing GNU hash table state into `src/gnu/hash.rs`, using the translated data structures and preserving original control flow. Depends on: T005, T006.
- [T009] [Story] Port the function group in `gnu/hash.c` responsible for bucket, chain, bloom, or related table population/build steps into `src/gnu/hash.rs`. Depends on: T008.
- [T010] [P] [Story] Port any local helper function(s) used only by GNU hash table construction in `src/gnu/hash.rs`, keeping them colocated with the build logic and avoiding duplication of work across phases. Depends on: T008.

## Phase 5: Lookup and query functions

- [T011] [Story] Port the function group in `gnu/hash.c` responsible for GNU hash lookup/query traversal into `src/gnu/hash.rs`, including bucket and chain navigation behavior required by the module. Depends on: T009.
- [T012] [P] [Story] Port any local helper function(s) dedicated to lookup/query checks in `src/gnu/hash.rs`, keeping the implementation scoped to functions evidenced by `gnu/hash.c`. Depends on: T011.

## Final Phase: Polish

- [T013] [Story] Refine `src/gnu/hash.rs` to remove C-specific portability artifacts that are no longer needed in Rust, simplify signatures where the completed port allows, and ensure internal naming/visibility are consistent across all 9 ported functions. Depends on: T007, T010, T012.
- [T014] [Story] Perform a final pass on `src/gnu/hash.rs` for idiomatic Rust cleanup that does not change behavior, including module organization, inline documentation comments where useful for maintainability, and elimination of redundant helper code introduced during the port. Depends on: T013.