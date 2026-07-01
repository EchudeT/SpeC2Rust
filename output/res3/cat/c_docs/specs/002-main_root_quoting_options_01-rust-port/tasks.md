# Tasks: main_root_quoting_options_01

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for quoting option support by adding `src/quotearg.rs` and wiring it into the crate from `src/main.rs` or `src/lib.rs`, matching the `quotearg.c` migration target.
- [ ] T002 [Story] Define the module boundary and migration placeholders in `src/quotearg.rs` for the 15 functions and related quoting-option state so later tasks can land without reshaping the file structure.

## Phase 2: Foundational

- [ ] T003 [Story] Implement the foundational quoting-related data structures and enums inferred from `quotearg.c` in `src/quotearg.rs`, including Rust representations for quoting styles, quoting options, slot state, and any constant tables required by later function work.
- [ ] T004 [P] [Story] Add constructors, defaults, and internal helper representations for the quoting-option structures in `src/quotearg.rs` so function groups can share stable initialization behavior. Depends on: T003
- [ ] T005 [P] [Story] Add internal ownership/borrowing layout for buffers and argument storage used by quoting operations in `src/quotearg.rs`, keeping the state model close to the original C module responsibilities. Depends on: T003

## Phase 3: Option State and Accessors

- [ ] T006 [Story] Implement the global/default quoting option access functions in `src/quotearg.rs`, covering retrieval and reset-style operations that manage the root quoting configuration. Depends on: T003, T004
- [ ] T007 [P] [Story] Implement setter functions for quoting style and related option fields in `src/quotearg.rs`, grouping the direct option mutation APIs together. Depends on: T006
- [ ] T008 [P] [Story] Implement accessor or cloning-style functions that derive per-call quoting option state from the default/root configuration in `src/quotearg.rs`. Depends on: T006

## Phase 4: Character Quoting and Customization

- [ ] T009 [Story] Implement the functions in `src/quotearg.rs` that enable, disable, or query character-specific quoting flags within a quoting-options instance. Depends on: T003, T004, T007
- [ ] T010 [P] [Story] Implement custom quoting delimiter/control functions in `src/quotearg.rs` for option variants that require caller-provided quoting behavior. Depends on: T007, T009

## Phase 5: Quoting Execution Functions

- [ ] T011 [Story] Implement the core quoting routine group in `src/quotearg.rs` that applies a `QuotingOptions` instance to input text and produces quoted output. Depends on: T005, T008, T009, T010
- [ ] T012 [P] [Story] Implement convenience wrapper functions in `src/quotearg.rs` that quote using the default/root options or caller-supplied options without duplicating the core quoting logic. Depends on: T011
- [ ] T013 [P] [Story] Implement slot-based or reusable-buffer quoting entry points in `src/quotearg.rs`, preserving the original module’s cached/output reuse behavior where present. Depends on: T005, T011

## Phase 6: Allocation-Oriented Variants

- [ ] T014 [Story] Implement allocation-returning quoting function variants in `src/quotearg.rs` that expose owned Rust strings/byte buffers corresponding to the C module’s allocation-focused APIs. Depends on: T011
- [ ] T015 [P] [Story] Implement specialized wrapper variants in `src/quotearg.rs` for common call patterns, such as style-specific or option-specific quoting helpers, by routing through the already-implemented core routines. Depends on: T012, T014

## Final Phase: Polish

- [ ] T016 [Story] Refine `src/quotearg.rs` to remove duplicated logic across wrappers, tighten visibility on internal data structures/helpers, and ensure the migrated API surface remains cohesive and centered on `quotearg.c` responsibilities. Depends on: T006, T007, T008, T009, T010, T011, T012, T013, T014, T015
- [ ] T017 [Story] Perform final integration cleanup for the module wiring in `src/main.rs` or `src/lib.rs` so the Rust port branch cleanly exposes the completed `quotearg` migration unit. Depends on: T016