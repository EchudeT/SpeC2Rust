# Tasks: module_gnu_if_09

## Phase 1: Setup

- [ ] [T001] [Story] Create the Rust module files `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs`, and register them from the existing GNU module tree so the `015-module_gnu_if_09-rust-port` branch can host the port of `gnu/printf-parse.c` and `gnu/vasnprintf.c`.
- [ ] [T002] [P] [Story] Establish shared module-level type and API placeholders in `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs` for the two migrated functions and their supporting data structure, keeping signatures and visibility aligned with the C module responsibilities. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Implement the single foundational data structure required by the `gnu/printf-parse.c` and `gnu/vasnprintf.c` port in `src/gnu/printf_parse.rs`, including its Rust field layout and basic constructors/helpers needed by downstream function implementations. Depends on: T002

## Phase 3: Format Parsing

- [ ] [T004] [Story] Implement the format-string parsing function migrated from `gnu/printf-parse.c` in `src/gnu/printf_parse.rs`, using the Phase 2 data structure as the parsed representation for directives and argument metadata. Depends on: T003

## Phase 4: Formatted Output Assembly

- [ ] [T005] [Story] Implement the formatted output construction function migrated from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, integrating with the parser/data representation from `src/gnu/printf_parse.rs` to reproduce the C module’s buffer-building behavior. Depends on: T003, T004

## Final Phase: Polish

- [ ] [T006] [Story] Refine the `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs` implementations to remove migration scaffolding, tighten module interfaces, and align Rust control flow and allocation usage with the completed port. Depends on: T005