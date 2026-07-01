# Tasks: module_src_parseopt_parseopt_01

## Phase 1: Setup

- [T001] [Story] Create the Rust module skeleton for the parseopt cluster by adding `src/parseopt/mod.rs`, `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`, and wire the module exports needed to port `src/parseopt/help.c`, `src/parseopt/optset.c`, and `src/parseopt/parseopt.c`.
- [T002] [Story] Define the migration surface for this module in `src/parseopt/mod.rs` by declaring the shared public/internal types and function namespaces that will be implemented across `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`; depends on [T001].

## Phase 2: Foundational

- [T003] [Story] Port the foundational option-description data structures from the C parseopt cluster into Rust structs/enums in `src/parseopt/optset.rs`, covering the option records, value/argument descriptors, flags, and parser-state fields required by the module files; depends on [T002].
- [T004] [P] [Story] Port the help/usage formatting support data structures from `src/parseopt/help.c` into Rust in `src/parseopt/help.rs`, including any intermediate layout, grouping, or display-oriented records directly used by help generation; depends on [T002].
- [T005] [P] [Story] Port the parsing workflow support data structures from `src/parseopt/parseopt.c` into Rust in `src/parseopt/parseopt.rs`, including parser context and transient state not already defined in `src/parseopt/optset.rs`; depends on [T002].
- [T006] [Story] Reconcile shared ownership and cross-file type usage by moving or re-exporting common parseopt data structures through `src/parseopt/mod.rs` so `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs` compile against a single consistent type model; depends on [T003], [T004], [T005].

## Phase 3: Option Set Construction and Mutation

- [T007] [Story] Implement the option-set creation, initialization, and teardown functions from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, using the foundational Rust data structures to replace the C lifecycle logic; depends on [T006].
- [T008] [Story] Implement the option registration and mutation functions from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, covering insertion, update, and field assignment behavior for option definitions; depends on [T007].
- [T009] [Story] Implement the option lookup and iteration support functions from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, preserving the C module’s search/traversal behavior required by parsing and help generation; depends on [T008].

## Phase 4: Help and Usage Rendering

- [T010] [Story] Implement the core help/usage text assembly functions from `src/parseopt/help.c` in `src/parseopt/help.rs`, including formatting of option names, arguments, and descriptive text from the Rust option-set types; depends on [T006], [T009].
- [T011] [Story] Implement the grouping, layout, and output-order functions from `src/parseopt/help.c` in `src/parseopt/help.rs` so rendered help matches the source module’s structure and uses the shared option metadata consistently; depends on [T010].

## Phase 5: Argument Parsing Flow

- [T012] [Story] Implement the parser entry-point and initialization functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, establishing argument stream setup and parser context preparation against the Rust option-set APIs; depends on [T006], [T009].
- [T013] [Story] Implement the option token classification and dispatch functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, covering recognition and routing for supported option forms using the Rust parser state; depends on [T012].
- [T014] [Story] Implement the option argument consumption and value-assignment functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, preserving the C module’s handling of attached/separate values and parser state updates; depends on [T013].
- [T015] [Story] Implement the parse completion and remaining-argument handling functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, finalizing parser results and end-of-input behavior; depends on [T014].

## Final Phase: Polish

- [T016] [P] [Story] Refine the internal APIs and data flow across `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs` to remove C-centric patterns that are no longer needed after the port while preserving module behavior; depends on [T011], [T015].
- [T017] [Story] Perform a final compile-pass cleanup across `src/parseopt/mod.rs`, `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`, resolving unused items, tightening visibility, and ensuring the migrated module builds cleanly on branch `098-module_src_parseopt_parseopt_01-rust-port`; depends on [T016].