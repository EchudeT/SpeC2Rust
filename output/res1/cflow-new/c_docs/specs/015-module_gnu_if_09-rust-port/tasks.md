# Tasks: module_gnu_if_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for this port on branch `015-module_gnu_if_09-rust-port`, adding target source files `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs` to mirror `gnu/printf-parse.c` and `gnu/vasnprintf.c`.
- [T002] [P] [Story] Wire the new module files into the crate module tree so `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs` are compiled and accessible from the existing `src/gnu/mod.rs` or equivalent module entry point. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the single foundational data structure required by the parsed printf processing logic in `src/gnu/printf_parse.rs`, translating the module-local C structure used by `gnu/printf-parse.c` into an idiomatic Rust representation with fields aligned to current module needs. Depends on: T002.
- [T004] [Story] Add minimal shared type aliases, enums, or helper constants in `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs` only where directly required to support the data structure and upcoming function ports, keeping ownership and mutability rules explicit. Depends on: T003.

## Phase 3: Format Parsing Functions

- [T005] [Story] Port the printf format parsing function from `gnu/printf-parse.c` into `src/gnu/printf_parse.rs`, implementing parsing flow around the foundational data structure and preserving the original module behavior for interpreting format directives. Depends on: T004.
- [T006] [Story] Align the public/internal function signature and result types in `src/gnu/printf_parse.rs` so the parser can be consumed directly by the formatting logic in `src/gnu/vasnprintf.rs` without introducing extra compatibility layers. Depends on: T005.

## Phase 4: Formatted Output Assembly

- [T007] [Story] Port the variadic string formatting function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, reusing the parser interface from `src/gnu/printf_parse.rs` and translating buffer growth and output assembly behavior into Rust. Depends on: T006.
- [T008] [P] [Story] Refine direct call sites and imports between `src/gnu/vasnprintf.rs` and `src/gnu/printf_parse.rs` so the formatting implementation uses the parsed format structure consistently and without duplicated parsing logic. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Review `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs` for Rust-specific cleanup, removing redundant C-style state handling, tightening visibility, and simplifying ownership/borrowing where this does not change module behavior. Depends on: T008.
- [T010] [Story] Perform a final compile-pass cleanup for the ported files `src/gnu/printf_parse.rs` and `src/gnu/vasnprintf.rs`, resolving warnings and ensuring the module builds cleanly on branch `015-module_gnu_if_09-rust-port`. Depends on: T009.