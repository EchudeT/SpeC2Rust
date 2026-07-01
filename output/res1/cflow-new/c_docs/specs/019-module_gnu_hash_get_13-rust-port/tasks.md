# Tasks: module_gnu_hash_get_13

## Phase 1: Setup

- [T001] [Story] Create the Rust module skeleton for the `gnu/hash.c` port on branch `019-module_gnu_hash_get_13-rust-port`, adding `src/gnu/hash.rs` and exposing it from the existing Rust crate module tree.
  - File paths: `src/gnu/hash.rs`
  - Dependencies: none

- [T002] [P] [Story] Define the Rust-side file-local organization for the GNU hash port in `src/gnu/hash.rs`, including sections for translated data structures, helper types, and function implementations so later migration work lands in one inferred target file.
  - Dependencies: T001

## Phase 2: Foundational

- [T003] [Story] Port the foundational data structures referenced by `gnu/hash.c` into Rust in `src/gnu/hash.rs`, translating the C structs, aliases, enums, and constants needed by the module before any function bodies are implemented.
  - File paths: `src/gnu/hash.rs`
  - Dependencies: T002

- [T004] [P] [Story] Add Rust representations for GNU-hash-specific layout and accessor helpers required to read and manipulate the translated `gnu/hash.c` data structures in `src/gnu/hash.rs`.
  - Dependencies: T003

- [T005] [Story] Resolve ownership, borrowing, and mutability boundaries for the translated `gnu/hash.c` data structures in `src/gnu/hash.rs` so the later function ports can use stable Rust signatures without revisiting the same structure definitions.
  - Dependencies: T003, T004

## Phase 3: GNU hash access functions

- [T006] [Story] Port the GNU hash retrieval entry function from `gnu/hash.c` into `src/gnu/hash.rs`, wiring it to the translated foundational data structures and preserving the original module-level behavior.
  - File paths: `src/gnu/hash.rs`
  - Dependencies: T005

- [T007] [P] [Story] Port the internal GNU hash lookup/support function from `gnu/hash.c` into `src/gnu/hash.rs`, reusing the shared translated structures and helpers without redefining module state.

- [T008] [Story] Port the remaining GNU hash access/helper function from `gnu/hash.c` into `src/gnu/hash.rs`, completing the 3-function migration for this module and aligning its signature and control flow with the Rust data model.

- [T009] [Story] Integrate the three ported functions within `src/gnu/hash.rs`, consolidating shared helper usage and removing any placeholder translation scaffolding introduced during the individual function ports.
  - Dependencies: T006, T007, T008

## Final Phase: Polish

- [T010] [Story] Refine `src/gnu/hash.rs` for idiomatic Rust within the boundaries of the `gnu/hash.c` migration, simplifying local control flow, tightening visibility, and eliminating redundant translated constructs that are no longer needed after integration.
  - File paths: `src/gnu/hash.rs`
  - Dependencies: T009

- [T011] [Story] Perform a final module review of the `gnu/hash.c` to `src/gnu/hash.rs` migration to confirm all referenced data structures and all 3 functions are present exactly once and that task-scoped file changes remain limited to the inferred Rust target file.
  - Dependencies: T010