# Tasks: module_src_parseopt_optsort_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parseopt/help.c` port in `src/parseopt/help.rs`, and expose it from the existing Rust module tree on branch `104-module_src_parseopt_optsort_07-rust-port`.
- [T002] [P] [Story] Establish the module-local file organization needed for this port in `src/parseopt/help.rs`, including placeholders for the option-sorting data structures and the two function implementations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and define the Rust representations for the option/help-related data carried by `src/parseopt/help.c` inside `src/parseopt/help.rs`, covering the module’s required structs, enums, aliases, and constants before any function porting begins. Depends on: T002.
- [T004] [Story] Implement the foundational ownership and borrowing layout for the defined option/help data structures in `src/parseopt/help.rs`, so the later option-sorting logic can operate without C-style raw layout assumptions. Depends on: T003.
- [T005] [P] [Story] Add internal constructors, default initializers, and helper accessors required by the ported data structures in `src/parseopt/help.rs`, limited to what is directly needed by the two functions in this module. Depends on: T004.

## Phase 3: Functions

- [T006] [Story] Port the first `src/parseopt/help.c` function into `src/parseopt/help.rs`, implementing the option/help preparation logic that feeds option sorting using the Rust data structures from Phase 2. Depends on: T005.
- [T007] [Story] Port the second `src/parseopt/help.c` function into `src/parseopt/help.rs`, implementing the option-sorting/output-facing logic that completes the behavior of this module. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/parseopt/help.rs` to remove C-centric patterns left from the initial port, simplify control flow around the option-sorting path, and align naming and visibility with the surrounding Rust project conventions. Depends on: T007.
- [T009] [Story] Perform a final compile-integration pass for the `src/parseopt/help.rs` module on branch `104-module_src_parseopt_optsort_07-rust-port`, resolving any import, visibility, or type mismatches introduced during the port. Depends on: T008.