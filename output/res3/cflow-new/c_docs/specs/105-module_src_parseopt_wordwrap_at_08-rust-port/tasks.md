# Tasks: module_src_parseopt_wordwrap_at_08

## Phase 1: Setup

- [ ] [T001] [Story] Create the Rust module scaffold for the `wordwrap` port by adding `src/parseopt/wordwrap.rs` and exposing it from the existing `src/parseopt/mod.rs` module tree for branch `105-module_src_parseopt_wordwrap_at_08-rust-port`.
- [ ] [T002] [Story] Review `src/parseopt/wordwrap.c` and map the 2 functions and 18 data-structure usages to Rust ownership and module-local type definitions to constrain the port scope in `src/parseopt/wordwrap.rs`. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Define the foundational Rust data structures required by `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, including direct Rust representations for the module-local state, wrap configuration, token/word tracking, line state, iterator/cursor state, and any helper enums or structs evidenced by the C module. Depends on: T002
- [ ] [T004] [P] [Story] Implement constructor/default/helper methods for the `wordwrap` data structures in `src/parseopt/wordwrap.rs` where needed to support the later function port without expanding behavior beyond the C module. Depends on: T003
- [ ] [T005] [P] [Story] Add internal field-level documentation comments in `src/parseopt/wordwrap.rs` to record how each Rust data structure corresponds to the original `src/parseopt/wordwrap.c` state and data flow. Depends on: T003

## Phase 3: Functions

- [ ] [T006] [Story] Port the lower-level word wrapping support function from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, wiring it to the foundational data structures and preserving the original wrap-state transitions. Depends on: T004
- [ ] [T007] [Story] Port the higher-level word wrapping entry function from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, using the support function and keeping the original control flow, argument handling, and output assembly semantics. Depends on: T006
- [ ] [T008] [Story] Integrate the completed `wordwrap` Rust API into the surrounding parseopt module interfaces by updating the relevant exports or internal call sites in `src/parseopt/mod.rs` only where required by the original `src/parseopt/wordwrap.c` usage boundaries. Depends on: T007

## Final Phase: Polish

- [ ] [T009] [P] [Story] Refine `src/parseopt/wordwrap.rs` to remove redundant temporary state introduced during translation and align naming and visibility with Rust module conventions while preserving the original C behavior. Depends on: T008
- [ ] [T010] [Story] Perform a final pass on `src/parseopt/wordwrap.rs` and `src/parseopt/mod.rs` to verify the port remains limited to the original `src/parseopt/wordwrap.c` responsibilities and that no unevidenced behaviors or extra module surface were introduced. Depends on: T009