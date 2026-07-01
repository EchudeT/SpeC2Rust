# Tasks: main_root Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust entry module structure for the `main_root` port by adding `src/main.rs` and mapping responsibilities currently held in `sds.c`.
- [T002] [Story] Establish module-level migration scaffolding in `src/main.rs` for the 45 functions and 5 data structures from `sds.c`, including grouped implementation sections and placeholders for translated logic. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the core data structure definitions migrated from `sds.c` in `src/main.rs`, covering all 5 module-owned structures required by the main program flow. Depends on: T002.
- [T004] [P] [Story] Add associated constructors, default initialization, and basic helper methods for the migrated `sds.c` data structures in `src/main.rs` where directly required by subsequent function ports. Depends on: T003.

## Phase 3: Program bootstrap and argument flow

- [T005] [Story] Port the main entry and startup control flow functions from `sds.c` into `src/main.rs`, including program initialization ordering and top-level execution dispatch. Depends on: T003.
- [T006] [P] [Story] Port command-line argument parsing and option interpretation functions from `sds.c` into `src/main.rs`, keeping behavior aligned with the original main-module input handling. Depends on: T003.
- [T007] [Story] Integrate startup control flow with parsed argument state in `src/main.rs` so the Rust entry path mirrors the original `sds.c` execution path. Depends on: T005, T006.

## Phase 4: Configuration and state preparation

- [T008] [P] [Story] Port configuration loading, initial state setup, and runtime context preparation functions from `sds.c` into `src/main.rs`. Depends on: T003.
- [T009] [P] [Story] Port environment-derived initialization and default state population functions from `sds.c` into `src/main.rs` where they support the main program setup path. Depends on: T003.
- [T010] [Story] Connect configuration, environment initialization, and startup sequencing in `src/main.rs` so all prerequisite runtime state is ready before primary execution. Depends on: T007, T008, T009.

## Phase 5: Core main-module operations

- [T011] [P] [Story] Port the central operational functions from `sds.c` that execute the module’s primary work after initialization into `src/main.rs`. Depends on: T010.
- [T012] [P] [Story] Port supporting helper functions used by the primary operational path in `sds.c` into `src/main.rs`, avoiding duplication of logic already migrated in earlier phases. Depends on: T010.
- [T013] [Story] Integrate the core operational path and its helpers in `src/main.rs` so the full main-module workflow executes end-to-end. Depends on: T011, T012.

## Phase 6: Output, reporting, and shutdown flow

- [T014] [P] [Story] Port output, status display, and reporting-related functions from `sds.c` into `src/main.rs` where they are part of the main module behavior. Depends on: T013.
- [T015] [P] [Story] Port termination, cleanup, and shutdown sequencing functions from `sds.c` into `src/main.rs`, limited to teardown behavior evidenced by the source module. Depends on: T013.
- [T016] [Story] Wire reporting and shutdown behavior into the main execution flow in `src/main.rs` so completion and exit handling match the original module structure. Depends on: T014, T015.

## Final Phase: Polish

- [T017] [Story] Refine the migrated code in `src/main.rs` by removing translation scaffolding, consolidating duplicated helper patterns introduced during porting, and ensuring idiomatic Rust control flow without changing module behavior. Depends on: T016.
- [T018] [Story] Perform a final pass on `src/main.rs` to tighten type usage, ownership/borrowing patterns, and function organization for the completed `main_root` Rust port. Depends on: T017.