# tasks.md

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for `main_root_clear_ungetc_09` on branch `009-main_root_clear_ungetc_09-rust-port`, creating or updating the target source file `src/fflush.rs` to host the ported logic from `fflush.c`.
- [T002] [P] [Story] Wire the new module into the Rust crate entry points by declaring and exposing `src/fflush.rs` from the existing crate root file needed by the project structure (for example `src/lib.rs` or `src/main.rs`), keeping the integration limited to the migrated `fflush.c` scope.
- [T003] [Story] Review `fflush.c` and map its two functions to Rust function stubs in `src/fflush.rs`, preserving current module-local responsibilities and noting any direct dependencies between the two functions before implementation.

## Phase 2: Foundational

- [T004] [Story] Establish the foundational Rust module structure in `src/fflush.rs`, including shared imports, internal helper signatures, and any minimal state handling required by both ported functions from `fflush.c`, without introducing unevidenced new data structures.
- [T005] [Story] Define the common control-flow and error-handling conventions used by the migrated `fflush.c` functions in `src/fflush.rs` so both implementations share the same return and internal-state patterns. Depends on: T003, T004.

## Phase 3: Functions

- [T006] [Story] Implement the root clear behavior from `fflush.c` in `src/fflush.rs`, porting the function logic responsible for the module’s clear operation and aligning it with the foundational conventions. Depends on: T005.
- [T007] [Story] Implement the `ungetc`-related behavior from `fflush.c` in `src/fflush.rs`, porting the second function and preserving its interaction with the clear logic where required by the original C flow. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/fflush.rs` for parity and readability by removing migration scaffolding, tightening function visibility, and ensuring the two ported functions remain confined to the original `fflush.c` responsibilities. Depends on: T006, T007.
- [T009] [P] [Story] Perform a final module integration pass over `src/fflush.rs` and the crate root wiring file to confirm the migrated `main_root_clear_ungetc_09` module builds cleanly and does not expose functionality beyond the original `fflush.c` scope. Depends on: T008.