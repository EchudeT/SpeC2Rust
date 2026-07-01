# Tasks: Port `src/wordsplit/wordsplit.c` to Rust

## Phase 1: Setup

- [ ] [T001] [Story] Create the module file scaffold for the Rust port by adding `src/wordsplit/wordsplit.rs` and exposing it from the existing `src/wordsplit/mod.rs` or nearest inferable module entry so the ported implementation has a dedicated target location.
- [ ] [T002] [Story] Define the initial Rust-side module surface in `src/wordsplit/wordsplit.rs`, including placeholder public/private items for the wordsplit state and helper types needed to receive the migrated data structures from `src/wordsplit/wordsplit.c`.
- [ ] [T003] [P] [Story] Review the C module layout in `src/wordsplit/wordsplit.c` and map its 11 functions into implementation groups inside `src/wordsplit/wordsplit.rs` so later tasks follow a single non-overlapping migration plan. Depends on: T001, T002

## Phase 2: Foundational

- [ ] [T004] [Story] Implement the core wordsplit state structure in `src/wordsplit/wordsplit.rs`, translating the primary module-level C struct fields into Rust-owned data and preserving field groupings needed across multiple functions. Depends on: T002
- [ ] [T005] [Story] Implement supporting configuration and flag representations in `src/wordsplit/wordsplit.rs`, converting C constants, option fields, and mode-like values used by the module into Rust enums, bitflags-style representations, or constants as appropriate. Depends on: T004
- [ ] [T006] [Story] Implement the module’s internal buffer, segment, and token-related supporting structures in `src/wordsplit/wordsplit.rs` so string splitting state can be expressed without relying on C memory layout. Depends on: T004
- [ ] [T007] [P] [Story] Implement callback, environment, and auxiliary context holder structures referenced by the wordsplit module in `src/wordsplit/wordsplit.rs`, preserving only the data relationships evidenced by `src/wordsplit/wordsplit.c`. Depends on: T004
- [ ] [T008] [Story] Add shared internal constructors and state-initialization helpers in `src/wordsplit/wordsplit.rs` for the foundational structures so grouped function ports can reuse a single setup path. Depends on: T005, T006, T007

## Phase 3: Lifecycle and State Management Functions

- [ ] [T009] [Story] Port the function group responsible for wordsplit object initialization, reset, and teardown from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, using the foundational state types instead of C allocation patterns. Depends on: T008
- [ ] [T010] [Story] Port any option-application or configuration-binding functions from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, wiring Rust configuration types into the main wordsplit state. Depends on: T005, T009
- [ ] [T011] [P] [Story] Port any state-copying, clearing, or reuse-oriented helpers from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, keeping ownership transitions explicit and localized. Depends on: T009

## Phase 4: Core Splitting and Token Processing Functions

- [ ] [T012] [Story] Port the main word splitting execution function from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, preserving the C module’s central control flow while expressing iteration and buffer growth idiomatically in Rust. Depends on: T008, T010
- [ ] [T013] [Story] Port the internal tokenization and segment-building helper functions from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, reusing the Rust buffer and token support structures defined earlier. Depends on: T006, T012
- [ ] [T014] [P] [Story] Port auxiliary scanning helpers that classify characters, delimiters, quoting, or escape-related transitions used by the splitting path in `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`. Depends on: T006, T012
- [ ] [T015] [Story] Integrate the token-processing helper group with the main split execution path in `src/wordsplit/wordsplit.rs` so each migrated function is connected exactly once into the final module flow. Depends on: T012, T013, T014

## Phase 5: Expansion and Output Assembly Functions

- [ ] [T016] [Story] Port the function group that performs post-token expansion, substitution, or callback-driven transformation from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, using the Rust callback/context structures already introduced. Depends on: T007, T015
- [ ] [T017] [Story] Port the output assembly and final argv/token collection functions from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, converting C-style result population into Rust vectors or equivalent owned collections. Depends on: T015, T016
- [ ] [T018] [P] [Story] Port remaining small internal utility functions from `src/wordsplit/wordsplit.c` into `src/wordsplit/wordsplit.rs`, limited to helpers directly invoked by the wordsplit lifecycle, splitting, or output paths. Depends on: T009, T015, T017

## Final Phase: Polish

- [ ] [T019] [Story] Refine `src/wordsplit/wordsplit.rs` to remove temporary placeholders, collapse duplicated migration scaffolding, and ensure all 11 functions from `src/wordsplit/wordsplit.c` are represented once in the Rust module. Depends on: T011, T017, T018
- [ ] [T020] [Story] Perform a final module-level review of `src/wordsplit/wordsplit.rs` and adjacent `src/wordsplit/mod.rs` exposure to align naming, visibility, and internal helper organization with the completed Rust port of `src/wordsplit/wordsplit.c`. Depends on: T019