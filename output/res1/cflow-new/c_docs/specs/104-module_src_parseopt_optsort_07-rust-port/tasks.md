# Tasks: module_src_parseopt_optsort_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parseopt/help.c` port on branch `104-module_src_parseopt_optsort_07-rust-port`, adding the target source file `src/parseopt/help.rs` and wiring it into the existing Rust module tree.
- [T002] [Story] Establish the initial public/private item layout in `src/parseopt/help.rs` for the 46 module data structures and 2 functions so later migration work can be added without reshaping the file. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the module-local constants, enums, type aliases, and simple record definitions from `src/parseopt/help.c` into Rust definitions in `src/parseopt/help.rs`, preserving names and field intent needed by the option-sorting/help logic. Depends on: T002.
- [T004] [P] [Story] Port the remaining non-recursive composite structs and container-style data structures from `src/parseopt/help.c` into `src/parseopt/help.rs`, deriving or implementing only the traits directly required by the migrated module logic. Depends on: T003.
- [T005] [P] [Story] Port any linked, nested, or callback-bearing data structures referenced by the help/option-sorting routines into `src/parseopt/help.rs`, resolving pointer-based C layouts into Rust ownership or reference patterns that match the original module behavior. Depends on: T003.
- [T006] [Story] Reconcile the full set of 46 data structures in `src/parseopt/help.rs`, ensuring cross-references between definitions compile cleanly and match the needs of the function migration work. Depends on: T004, T005.

## Phase 3: Functions

- [T007] [Story] Implement the first migrated function from `src/parseopt/help.c` in `src/parseopt/help.rs`, using the new Rust data structures directly and preserving the original option/help processing behavior. Depends on: T006.
- [T008] [Story] Implement the second migrated function from `src/parseopt/help.c` in `src/parseopt/help.rs`, completing the module’s help/option-sorting behavior and integrating it with the first migrated function as required. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/help.rs` to remove migration-only placeholders, tighten visibility, and simplify any direct C-to-Rust translations that are no longer needed after both functions and all data structures compile together. Depends on: T008.
- [T010] [Story] Perform a final module compile pass for `src/parseopt/help.rs`, resolving warnings and small integration issues introduced during the port while keeping scope limited to the migrated `src/parseopt/help.c` functionality. Depends on: T009.