# Tasks: module_doc_main_01 Rust port

## Phase 1: Setup

- [ ] [T001] [Story] Initialize the Rust module workspace for `module_doc_main_01` on branch `001-module_doc_main_01-rust-port`, creating target source files corresponding to the C inputs: `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs`.
- [ ] [T002] [P] [Story] Add the module declarations and wiring needed to expose the document command modules from Rust, updating `src/doc/mod.rs` to include `d`, `foo`, `wc`, and `whoami`. Depends on: `T001`.

## Phase 2: Foundational

- [ ] [T003] [Story] Implement the first inferred foundational data structure needed by `doc/d.c` in `src/doc/d.rs`, preserving the original module-local responsibility before porting function logic. Depends on: `T002`.
- [ ] [T004] [P] [Story] Implement the second inferred foundational data structure needed by `doc/foo.c` in `src/doc/foo.rs`, keeping its shape aligned with the original C module usage. Depends on: `T002`.
- [ ] [T005] [P] [Story] Implement the third inferred foundational data structure needed by `doc/wc.c` or `doc/whoami.c` in the directly corresponding Rust file under `src/doc/`, based on where the C definition belongs. Depends on: `T002`.

## Phase 3: Functions

- [ ] [T006] [Story] Port the function from `doc/d.c` into `src/doc/d.rs`, rewriting it against the Rust data structure introduced for this module and keeping behavior scoped to the original file responsibility. Depends on: `T003`.
- [ ] [T007] [P] [Story] Port the function from `doc/foo.c` into `src/doc/foo.rs`, using the foundational structure defined for that module and maintaining the original functional scope. Depends on: `T004`.
- [ ] [T008] [P] [Story] Port the function from `doc/wc.c` into `src/doc/wc.rs`, connecting it to any required foundational structure implemented for this file. Depends on: `T005`.
- [ ] [T009] [P] [Story] Port the function from `doc/whoami.c` into `src/doc/whoami.rs`, preserving the single-module behavior of the original implementation. Depends on: `T002`.

## Final Phase: Polish

- [ ] [T010] [Story] Refine the Rust ports in `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` by removing C-centric implementation leftovers, aligning shared idioms across the four files, and ensuring the module compiles cleanly as an integrated cluster. Depends on: `T006`, `T007`, `T008`, `T009`.