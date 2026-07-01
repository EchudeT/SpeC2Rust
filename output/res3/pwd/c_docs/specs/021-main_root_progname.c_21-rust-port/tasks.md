# Task List: main_root_progname.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/progname.rs` to host the port of `progname.c` for branch `021-main_root_progname.c_21-rust-port`.
- [T002] [Story] Expose the new module from `src/main.rs` or `src/lib.rs` by adding the corresponding `mod progname;` declaration and wiring needed imports for use by the main cluster. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `progname.c` migration needs and define any minimal Rust-side internal constants, type aliases, or helper signatures directly in `src/progname.rs` required to support the single function port without introducing unevidenced new structures. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Port the function implemented in `progname.c` into `src/progname.rs`, preserving its main-cluster behavior and adapting pathname/program-name handling to idiomatic Rust while keeping the original module scope. Depends on: T002, T003
- [T005] [P] [Story] Integrate the ported `progname.c` function at its Rust call site in `src/main.rs` or `src/lib.rs`, replacing or connecting the prior placeholder flow so the main cluster uses the new module implementation. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/progname.rs` and the related `src/main.rs` or `src/lib.rs` integration for naming consistency, minimal duplication, and alignment with the original `progname.c` responsibilities after the port is complete. Depends on: T004, T005