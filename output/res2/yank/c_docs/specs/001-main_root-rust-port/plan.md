# Implementation Plan: main_root

## Summary

Port `yank.c` into a single Rust main-module implementation that preserves the current executable behavior and keeps the migration scope limited to the existing entrypoint and helper functions.

The Rust implementation should:

- map the C `main` module into the standard Rust binary entrypoint in `src/main.rs`
- migrate the existing function set directly, keeping responsibilities close to the original layout
- use the Rust standard library for I/O, argument handling, string processing, and terminal-facing output where possible
- replace implicit C error signaling and manual resource handling with explicit `Result`-based flow and RAII cleanup
- keep terminal lifecycle operations (`tsetup`, `tend`, `tgetc`, `tputs`, `twrite`, `tmain`) grouped together rather than spreading them into extra abstractions
- preserve file and input processing order, pattern conversion/comparison behavior, and write paths while avoiding unnecessary architectural expansion

The technical approach is a conservative translation: first establish a working Rust binary that matches the current control flow, then migrate parsing/matching/output helpers, and finally tighten error propagation and tests around the observed behavior.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are required by the available module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain single-process CLI performance comparable to the C implementation for normal file and stdin processing
  - Avoid unnecessary allocation in hot paths such as input scanning, pattern preparation, and output writes
  - Use buffered I/O where the original code performs repeated small reads or writes
  - Preserve streaming behavior rather than loading full inputs into memory unless the original function behavior requires it

## Module Mapping

### C to Rust File Mapping

- `yank.c` → `src/main.rs`

### Function Mapping

All existing C functions should be migrated into `src/main.rs` first, with direct Rust equivalents and minimal renaming only where needed for Rust conventions.

- `input` → `fn input(...) -> Result<..., ...>`
  - Migrate file/stdin ingestion logic
  - Replace raw buffers and pointer arithmetic with slices, `String`, `Vec<u8>`, `BufRead`, or `Read` depending on actual byte/text behavior
- `strtopat` → `fn strtopat(...) -> Result<Pattern, ...>` or `fn strtopat(...) -> ...`
  - Convert C pattern preparation logic into an explicit Rust pattern type
- `fcmp` → `fn fcmp(...) -> bool` or `fn fcmp(...) -> Ordering`
  - Preserve original comparison semantics exactly
- `xwrite` → `fn xwrite(...) -> io::Result<()>`
  - Wrap write-all behavior using `Write::write_all`
- `yank` → `fn yank(...) -> Result<..., ...>`
  - Keep as the core extraction/selection routine
- `twrite` → `fn twrite(...) -> io::Result<()>`
  - Terminal-targeted write helper
- `tputs` → `fn tputs(...) -> io::Result<()>`
  - Terminal string output helper
- `tsetup` → `fn tsetup(...) -> Result<TerminalState, ...>`
  - Convert setup side effects into owned terminal state
- `tend` → `fn tend(state: TerminalState) -> Result<(), ...>` or automatic cleanup via `Drop`
  - Prefer RAII cleanup if feasible without changing behavior
- `tgetc` → `fn tgetc(...) -> io::Result<Option<u8>>` or `io::Result<char>`
  - Preserve blocking/input semantics
- `tmain` → `fn tmain(...) -> Result<i32, ...>`
  - Keep terminal-interactive control flow separate from generic `main`
- `usage` → `fn usage(program: &str)`
  - Emit usage text to stdout/stderr according to current behavior
- `main` → `fn main()`
  - Parse args, dispatch, convert errors to exit codes

### Rust Module Layout

Keep the module layout restrained:

- `src/main.rs`
  - argument parsing
  - pattern handling
  - input/file processing
  - terminal helpers
  - program entrypoint

If internal organization becomes necessary during translation, use private helper structs/functions within `src/main.rs` rather than adding extra files.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust plan should derive concrete named types from usage during migration rather than inventing new domain objects up front.

### Data-Structure Mapping Strategy

For each anonymous C struct encountered in `yank.c`:

- anonymous C struct used for CLI/config state → `struct Config`
- anonymous C struct used for prepared match/pattern state → `struct Pattern`
- anonymous C struct used for terminal mode/setup state → `struct TerminalState`
- anonymous C struct used for transient input scanning state → `struct InputState`
- anonymous C struct used for selection/output state → `struct YankState`

If multiple anonymous structs serve the same area, distinguish them with numeric or role-based names only as needed:

- `PatternAtom`
- `PatternRange`
- `TerminalCapabilities`
- `InputBuffer`
- `OutputTarget`

### C-to-Rust Representation Rules

- C integral flags/fields:
  - map to `bool` for true/false state
  - map to `u8`, `u32`, `usize`, or `i32` only when arithmetic/range semantics require it
- C character pointers:
  - `*const char` / `*mut char` used as text → `String`, `&str`, or `Cow<'_, str>` if borrowing is practical
  - byte-oriented data → `Vec<u8>` / `&[u8]`
- C arrays and manual buffers:
  - fixed working storage → `[u8; N]` only if size is truly compile-time and behavior depends on it
  - otherwise `Vec<u8>` or `String`
- C file handles:
  - `FILE *` / descriptors → `std::fs::File`, `std::io::Stdin`, `std::io::Stdout`, `std::io::Stderr`
- C enum-like integer modes:
  - map to Rust `enum` when the set of cases is closed and evident from usage
  - otherwise preserve as small integer type during initial port, then tighten if safe

### Memory Management and Ownership

- Eliminate manual allocation/free by assigning ownership to Rust structs and local variables.
- Convert borrowed slices and string views carefully where the original code stores pointers into mutable buffers.
- Avoid self-referential designs; if the C code keeps pointers into a working buffer, redesign as:
  - owned `String`/`Vec<u8>` plus index ranges, or
  - separately owned substrings where copying is minor and keeps the port simple
- Terminal setup/teardown state should be owned by a `TerminalState` value so cleanup is explicit and cannot be forgotten.

### Error Handling Model

- Functions that can fail in C via return codes, null pointers, or partial writes should return:
  - `io::Result<T>` for I/O-bound operations
  - `Result<T, String>` or a small local error enum for parsing/setup logic
- `main` should convert internal errors into exit status and user-facing messages without panics.
- Use panics only for unreachable internal invariants discovered during translation, not for routine CLI or I/O failures.

## Implementation Phases

## Phase 1: Skeleton Port and Entry Flow

### Goals

Establish a compiling Rust binary with the original top-level control flow and minimal placeholder behavior, preserving the existing module boundary.

### Tasks

- Create `src/main.rs` as the destination for the `yank.c` port.
- Migrate `main` and `usage` first.
- Introduce a local `Config` representation based on the observed argument usage.
- Translate high-level dispatch between normal processing and terminal-driven processing (`tmain`) without adding new modules.
- Define signatures for:
  - `input`
  - `strtopat`
  - `fcmp`
  - `xwrite`
  - `yank`
  - `twrite`
  - `tputs`
  - `tsetup`
  - `tend`
  - `tgetc`
  - `tmain`

### Deliverables

- `cargo test` and `cargo build` succeed
- CLI entrypoint exists in Rust
- usage/error-exit path is wired
- no unsafe code introduced unless strictly required by terminal APIs discovered during implementation

## Phase 2: Core Input, Pattern, and Output Migration

### Goals

Port the non-terminal core behavior from C into Rust with direct function-by-function replacement.

### Tasks

- Implement `strtopat` using a Rust-owned pattern representation derived from actual C field usage.
- Implement `fcmp` with semantics preserved exactly from the C comparison logic.
- Implement `input` for file and/or stdin processing using buffered standard-library I/O.
- Implement `xwrite` as a strict write-all helper.
- Implement `yank` as the central processing function, keeping the original operation order and data flow.
- Replace pointer-based buffer traversal with:
  - indices into `Vec<u8>` / `String`, or
  - iterator-based scanning where behavior remains exact
- Preserve byte/text distinctions from the C code instead of normalizing everything to UTF-8 prematurely.

### Deliverables

- Main non-terminal execution path works in Rust
- input processing and output writing are handled without manual memory management
- core helper functions are covered by focused unit tests where behavior is deterministic

## Phase 3: Terminal Function Migration

### Goals

Port the terminal-specific path conservatively, keeping setup, reads, writes, and teardown behavior grouped and explicit.

### Tasks

- Implement `tsetup`, `tend`, `tgetc`, `tputs`, `twrite`, and `tmain`.
- Model terminal lifecycle with a small owned `TerminalState`.
- Prefer standard library facilities first; if low-level terminal handling is unavoidable, isolate the minimum necessary OS interaction in `src/main.rs`.
- Ensure terminal teardown occurs on all normal error-return paths.
- Keep interactive control flow structure close to the C implementation rather than redesigning it.

### Deliverables

- Terminal mode/setup path migrated
- interactive input/output path operates through Rust-owned state
- cleanup behavior is explicit and testable where practical

## Phase 4: Behavioral Tightening and Test Completion

### Goals

Finish the port by validating edge cases, reducing translation risk, and aligning Rust error handling with the executable’s existing behavior.

### Tasks

- Compare C and Rust behavior for:
  - argument validation
  - pattern parsing edge cases
  - comparison/matching boundary conditions
  - stdin versus file input behavior
  - terminal teardown on early exit
- Add unit tests for pure helpers such as:
  - `strtopat`
  - `fcmp`
  - any deterministic parts of `yank`
- Add integration-style tests for CLI-visible behavior using `cargo test`.
- Remove dead translation scaffolding and placeholder types once all anonymous C structs are concretely mapped.
- Review for:
  - unnecessary allocations
  - accidental UTF-8 assumptions
  - incomplete error propagation
  - any lingering C-style sentinel logic that should be expressed as `Option` or `Result`

### Deliverables

- Rust module behavior aligned with the C module
- all migrated functions implemented
- tests run through `cargo test`
- `src/main.rs` remains the sole module file for this port unless a strictly necessary split is proven during implementation