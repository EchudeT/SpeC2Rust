# Tasks: module_src_c.c_20 Rust port

**Project**: `cflow-new`
**Branch**: `083-module_src_c.c_20-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for `src/c.c` by adding the target Rust source file `src/c.rs` and wiring its module export in `src/lib.rs` or the nearest existing module root that exposes migrated source modules.
- [ ] T002 [Story] Establish the `module_src_c.c_20` migration boundary in `src/c.rs`, including placeholder sections for data structures and function groups derived from `src/c.c`. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Port the module-owned constants, type aliases, and the first subset of core data structures from `src/c.c` into Rust definitions in `src/c.rs`, covering the structures required broadly across the module’s functions. Depends on: T002
- [ ] T004 [P] [Story] Port the remaining module data structures from `src/c.c` into Rust definitions in `src/c.rs`, completing the full set of 13 data-structure migrations and preserving field relationships needed by later function ports. Depends on: T002
- [ ] T005 [Story] Reconcile all cross-structure references, initialization defaults, and ownership/borrowing strategy for the migrated data structures in `src/c.rs` so the foundational model is ready for function implementation. Depends on: T003, T004

## Phase 3: Core state and construction functions

- [ ] T006 [Story] Implement the data creation, initialization, and reset-related function group from `src/c.c` in `src/c.rs`, using the migrated structures as the canonical Rust state model. Depends on: T005
- [ ] T007 [Story] Implement the basic teardown, cleanup, or release-related function group from `src/c.c` in `src/c.rs`, matching the lifecycle expectations established by the construction functions. Depends on: T006

## Phase 4: Parsing and transformation functions

- [ ] T008 [P] [Story] Implement the input interpretation, parsing, or scanning-related function group from `src/c.c` in `src/c.rs`, keeping the logic localized to the `src/c.rs` module boundary. Depends on: T005
- [ ] T009 [Story] Implement the transformation, normalization, or update-related function group from `src/c.c` in `src/c.rs`, using outputs from the parsing/scanning functions where required. Depends on: T008
- [ ] T010 [Story] Integrate parsing and transformation flows inside `src/c.rs` so shared intermediate state and error paths from `src/c.c` are preserved consistently in Rust. Depends on: T008, T009

## Phase 5: Query and output functions

- [ ] T011 [P] [Story] Implement the lookup, query, or accessor-related function group from `src/c.c` in `src/c.rs`, exposing the read-oriented behaviors over the migrated data structures. Depends on: T005
- [ ] T012 [P] [Story] Implement the formatting, output, or emit-related function group from `src/c.c` in `src/c.rs`, preserving the module-local rendering behavior present in `src/c.c`. Depends on: T005
- [ ] T013 [Story] Complete the remaining helper and coordination functions from `src/c.c` in `src/c.rs` so all 15 module functions are migrated exactly once within their appropriate groups. Depends on: T007, T010, T011, T012

## Final Phase: Polish

- [ ] T014 [Story] Review `src/c.rs` for idiomatic Rust refinements that do not change module behavior, including simplifying ownership patterns, removing transitional placeholders, and tightening visibility to the minimum required scope. Depends on: T013
- [ ] T015 [Story] Perform final module-level compile and integration cleanup for the `src/c.c` migration path, ensuring the new Rust implementation is the active code path through `src/c.rs` and its module export. Depends on: T014