# Implementation Plan: module_doc_main_01

## Summary

This module cluster contains four standalone C entry-point programs from `doc/`:

- `doc/d.c`
- `doc/foo.c`
- `doc/wc.c`
- `doc/whoami.c`

Each source file defines its own `main`, so the Rust port should preserve that shape as four separate executable targets rather than merging behavior into a shared CLI. The Rust implementation should focus on a direct migration of per-file control flow, argument handling, standard input/output behavior, and exit status propagation.

The technical approach is to map each C file to one Rust binary target under standard Cargo layout, using the Rust standard library for:

- command-line argument access
- stdin/stdout interaction
- filesystem and environment queries if required by the original logic
- explicit `Result`-based error propagation and process exit codes

Because the analysis shows only anonymous data structures and `main` functions, the Rust port should remain minimal: migrate code file-by-file, introduce small private helper functions only when needed to keep each binary readable, and avoid creating extra abstraction layers not justified by the source.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the operational scale of the original C utilities without avoidable heap allocation or unnecessary buffering layers.
  - Preserve streaming behavior for stdin/stdout-oriented logic where applicable.
  - Keep startup overhead minimal by implementing each program as a small standalone binary.
  - Maintain equivalent asymptotic behavior to the C versions rather than introducing broader architectural changes.

## Module Mapping

### C to Rust File Mapping

The source set should be migrated as separate binaries in a single Cargo package:

- `doc/d.c` → `src/bin/d.rs`
- `doc/foo.c` → `src/bin/foo.rs`
- `doc/wc.c` → `src/bin/wc.rs`
- `doc/whoami.c` → `src/bin/whoami.rs`

### Entry-Point Mapping

Each C `main` maps to a Rust binary entry point:

- `main` in `doc/d.c` → `fn main()` in `src/bin/d.rs`
- `main` in `doc/foo.c` → `fn main()` in `src/bin/foo.rs`
- `main` in `doc/wc.c` → `fn main()` in `src/bin/wc.rs`
- `main` in `doc/whoami.c` → `fn main()` in `src/bin/whoami.rs`

### Internal Organization

Use Cargo’s standard multi-binary structure only:

- `Cargo.toml`
- `src/bin/d.rs`
- `src/bin/foo.rs`
- `src/bin/wc.rs`
- `src/bin/whoami.rs`

If a tiny amount of logic is shared after inspection of the C code, prefer a small internal module such as `src/lib.rs` only for code already duplicated by the migration. Do not introduce a library crate unless duplication is real and narrow.

## Data Model

The analysis identifies only anonymous data structures. Since no named C structs are listed, the Rust port should avoid inventing broad data models and instead map structures according to actual usage found during implementation.

### Data-Structure Mapping Rules

- **C anonymous struct used only locally** → Rust local `struct` inside the binary source if retaining grouped state improves directness.
- **C anonymous aggregate used as flags or mode values** → Rust local `enum` only if the C logic clearly represents a closed set of cases.
- **C string pointers (`char *`, `char **argv`)** → Rust `String`, `&str`, or `Vec<String>` depending on ownership needs.
- **C integer counters / sizes** → Rust integer types chosen by purpose:
  - counts and indices: `usize`
  - explicit signed status or arithmetic mirroring C behavior: `isize`/`i32` only when needed
- **C booleans encoded as int** → Rust `bool`
- **C file handles / stdio streams** → `std::fs::File`, `std::io::Stdin`, `std::io::Stdout`, and buffered wrappers only where beneficial to preserve efficient streaming

### Memory Management Decisions

- Replace manual C memory handling with owned Rust values and scoped lifetimes.
- Avoid collecting whole input into memory if the C utility processes streams incrementally.
- Prefer stack-local state and iterator-based processing where it keeps behavior clear and equivalent.
- Use byte-oriented processing for text utilities if exact byte counting or newline handling matters.

### Error Handling Decisions

- Convert C-style return-code paths into `Result`-returning helper functions called from `main`.
- Keep `main` responsible for:
  - printing concise error messages to stderr
  - mapping failures to non-zero exit status
- Use standard library error types where possible; avoid custom error frameworks unless the original code forces materially different categories.

## Implementation Phases

## Phase 1: Project Skeleton and Binary Target Setup

### Goals

Establish the Rust crate and create a one-to-one executable mapping for the four C files.

### Tasks

- Create or update the Cargo package for branch `001-module_doc_main_01-rust-port`.
- Add binary files:
  - `src/bin/d.rs`
  - `src/bin/foo.rs`
  - `src/bin/wc.rs`
  - `src/bin/whoami.rs`
- For each file, create a minimal `main` and a private `run() -> Result<(), Box<dyn std::error::Error>>` or similarly narrow standard-library-based return path.
- Confirm the package builds with all four binaries present.
- Keep all dependencies limited to the Rust standard library.

### Deliverables

- Compiling Cargo project with four binary targets.
- Stable file mapping from original C sources to Rust executables.

## Phase 2: File-by-File Logic Migration

### Goals

Port each C program’s behavior directly, preserving its own control flow and I/O semantics.

### Tasks

- Migrate `doc/d.c` into `src/bin/d.rs`:
  - copy the original execution order into Rust equivalents
  - convert argument parsing from `argv` access to `std::env::args()`
  - translate stdout/stderr writes to `print!`, `println!`, `eprintln!`, or `Write`
- Migrate `doc/foo.c` into `src/bin/foo.rs` with the same directness.
- Migrate `doc/wc.c` into `src/bin/wc.rs`:
  - preserve counting behavior exactly
  - use streaming input processing if the C code reads incrementally
- Migrate `doc/whoami.c` into `src/bin/whoami.rs`:
  - translate environment or user-identification access using standard library facilities where sufficient
- Where the C code uses anonymous temporary structs, either inline the fields as local variables or define small local Rust structs in the same file.
- Preserve observable behavior first; avoid cross-binary refactoring during initial migration.

### Deliverables

- Rust implementations for all four binaries.
- Direct replacement of the original `main` logic with idiomatic but behavior-preserving Rust control flow.

## Phase 3: Error Paths, Exit Codes, and I/O Semantics Verification

### Goals

Align Rust behavior with C expectations around failures and input/output handling.

### Tasks

- Review each binary for places where C returns status codes; map these explicitly in Rust.
- Ensure stderr is used for diagnostics and stdout remains reserved for program output.
- Verify handling of:
  - missing arguments
  - unreadable input sources if applicable
  - invalid state previously handled by C conditionals
- Check whether any utility depends on byte counts rather than Unicode scalar counts; use byte-based processing accordingly.
- Replace panic-prone constructs with explicit error propagation in normal failure paths.

### Deliverables

- Consistent exit behavior across all binaries.
- Rust implementations that match C-style utility behavior under normal and failing inputs.

## Phase 4: Tests and Final Conformance Pass

### Goals

Lock in migrated behavior with restrained tests and remove accidental divergences from the original C module.

### Tasks

- Add unit tests for isolated helper logic only where it exists naturally.
- Add integration tests under `tests/` that execute the binaries with representative arguments and input.
- Cover, at minimum:
  - successful invocation paths
  - basic output shape
  - failure exit for invalid invocation where applicable
  - stdin-driven behavior for stream-processing utilities
- Run `cargo test` and fix behavior mismatches.
- Perform a final pass to remove unnecessary abstractions introduced during porting.

### Deliverables

- Passing `cargo test` suite for the migrated module cluster.
- Final Rust layout that remains close to the source module boundaries and executable structure.

## Notes and Constraints

- Preserve the four-program structure; do not collapse them into a single dispatcher.
- Do not add CLI frameworks, async runtimes, or external parsing crates without source-driven need.
- Keep shared code minimal and only introduce it after duplicate migrated logic is confirmed.
- Favor exact behavior preservation over stylistic consolidation.
- Treat anonymous C data as implementation detail, not as a prompt to design a broader Rust domain model.