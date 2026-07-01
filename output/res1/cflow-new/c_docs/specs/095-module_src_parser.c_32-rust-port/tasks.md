# Tasks: module_src_parser.c_32 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/parser.c` in `src/parser.rs`, and expose it from the crate root or parent module so the ported parser module can be compiled on branch `095-module_src_parser.c_32-rust-port`.
- [T002] [P] [Story] Review `src/parser.c` and map the 11 C data structures and 2 functions into Rust items to be implemented in `src/parser.rs`, recording direct name/shape correspondence needed for the port. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust representations for all 11 data structures from `src/parser.c` in `src/parser.rs`, preserving field relationships and ownership layout needed by the parser module. Depends on: T002.
- [T004] [P] [Story] Add associated constructors, `Default`, or small helper methods in `src/parser.rs` only where required to instantiate and connect the ported parser data structures used by the C module logic. Depends on: T003.

## Phase 3: Parser functions

- [T005] [Story] Port the first parser-related function from `src/parser.c` into `src/parser.rs`, adapting its control flow to operate on the Rust data structures implemented in Phase 2. Depends on: T004.
- [T006] [Story] Port the second parser-related function from `src/parser.c` into `src/parser.rs`, keeping behavior aligned with the original module and integrating with the first ported function as required. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/parser.rs` to remove C-specific artifacts introduced during translation, simplify borrow/ownership handling, and align naming and module organization with the surrounding Rust project without changing module behavior. Depends on: T006.
- [T008] [Story] Perform a final compile-oriented pass over `src/parser.rs` and related module exposure added in setup, resolving remaining integration issues for the `module_src_parser.c_32` port on branch `095-module_src_parser.c_32-rust-port`. Depends on: T007.