# Implementation Plan: main_root

## Summary

Port the existing `yank.c` main module into a Rust binary crate entry path without expanding scope beyond the current file and function set. The Rust implementation should preserve the current command-line driven control flow, terminal setup/teardown behavior, buffered input/output behavior, and pattern/file processing semantics represented by the existing functions:

- input
- strtopat
- fcmp
- xwrite
- yank
- twrite
- tputs
- tsetup
- tend
- tgetc
- tmain
- usage
- main

The technical approach is a direct migration of `yank.c` logic into `src/main.rs`, with narrowly scoped internal helper functions and Rust data types replacing the anonymous C structures. Ownership and borrowing should replace manual memory management, while `Result`-based error propagation replaces C-style status/error handling where appropriate. Terminal and byte-stream operations should remain close to the current execution model and avoid introducing extra abstractions not required by the existing module.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
- **Testing**:
  - `cargo test`
  - Unit tests for parsing/comparison helpers that can be isolated from terminal state
  - Lightweight integration-style tests for argument handling and non-interactive paths where feasible
- **Performance Goals**:
  - Preserve the current single-process CLI performance characteristics
  - Avoid unnecessary allocation during input scanning, pattern preparation, and writes
  - Use buffered standard-library I/O where it matches the C behavior
  - Keep file comparison and terminal byte output near the current cost profile
  - Do not introduce additional concurrency or background processing

## Module Mapping

### Source File Mapping

- `yank.c` -> `src/main.rs`

This module should remain a single Rust main module unless a minimal internal split becomes strictly necessary during implementation. Since the source scope is one C file and one main-category module, the default target is one Rust source file with private helpers.

### Function Mapping

- `input` -> `fn input(...) -> Result<..., ...>`
- `strtopat` -> `fn strtopat(...) -> Result<..., ...>`
- `fcmp` -> `fn fcmp(...) -> Result<..., ...>` or `fn fcmp(...) -> ...` if comparison is infallible after setup
- `xwrite` -> `fn xwrite(...) -> io::Result<()>`
- `yank` -> `fn yank(...) -> Result<..., ...>`
- `twrite` -> `fn twrite(...) -> io::Result<()>`
- `tputs` -> `fn tputs(...) -> io::Result<()>`
- `tsetup` -> `fn tsetup(...) -> Result<..., ...>`
- `tend` -> `fn tend(...)`
- `tgetc` -> `fn tgetc(...) -> io::Result<u8>` or `Option<u8>` depending on current EOF/error distinction
- `tmain` -> `fn tmain(...) -> Result<i32, ...>` or `Result<(), ...>` with exit-code translation in `main`
- `usage` -> `fn usage(...) -> !` or `fn usage(...) -> i32`, depending on the existing control flow
- `main` -> `fn main()`

### Rust Module Layout

Use standard Rust binary crate layout:

```text
yank/
├─ Cargo.toml
└─ src/
   └─ main.rs
```

## Data Model

The analysis reports multiple anonymous C data structures. Since names are unavailable, the migration should assign Rust names based on actual usage in `yank.c`, not invent new conceptual layers. The goal is one Rust type per meaningful C aggregate currently used by the module.

### Mapping Strategy

- **anonymous C struct used for CLI/runtime state** -> `struct AppState`
- **anonymous C struct used for terminal state** -> `struct TerminalState`
- **anonymous C struct used for pattern representation** -> `struct Pattern`
- **anonymous C struct used for input/file entry tracking** -> `struct InputEntry`
- **anonymous C struct used for transient buffer state** -> `struct BufferState`
- **anonymous C struct used for comparison state** -> `struct CompareState`
- Any remaining anonymous aggregates -> private `struct` definitions named strictly from their actual role in the C file

If some anonymous C aggregates are only local packing helpers and do not carry across function boundaries, convert them to:
- ordinary Rust local variables,
- tuples,
- small private structs, or
- enums when the C code encodes tagged state manually.

### C-to-Rust Type Conventions

- `char *` string data:
  - `String` for owned textual data
  - `&str` for borrowed textual data
  - `Vec<u8>` when byte-preserving terminal/file behavior matters
- file handles / descriptors:
  - `std::fs::File`
  - `std::io::Stdin`, `Stdout`, `Stderr`
  - `AsRawFd`/`RawFd` only if direct descriptor behavior is required by the existing logic
- integer flags / mode fields:
  - `bool` when truly binary
  - small enums when representing distinct states already implied by the C code
  - fixed-width integers only where byte-level behavior matters
- mutable buffers:
  - `Vec<u8>` for growable binary data
  - `String` for validated text buffers
- arrays and pointer ranges:
  - slices (`&[u8]`, `&mut [u8]`, `&str`) where bounds are explicit
- error codes:
  - `std::io::Error` for I/O failures
  - private error enum for mixed parse/runtime failures if needed
  - avoid panics for normal operational errors

### Memory Management Notes

- Replace raw ownership transfer with Rust-owned values.
- Remove manual allocation/free logic by storing owned buffers in structs.
- Convert pointer arithmetic and sentinel traversal into indexed slices or iterators.
- Where the C code stores borrowed views into a mutable backing buffer, model this carefully in Rust:
  - either store indices into a shared buffer, or
  - materialize owned substrings/byte vectors if lifetimes would otherwise complicate the direct port.
- Prefer the simpler safe representation when both preserve behavior and do not alter scope.

### Error Handling Notes

- Terminal setup and teardown must be explicit in control flow.
- If `tsetup`/`tend` represent paired resource handling, use a small local guard pattern only if required to guarantee teardown on all exits; otherwise keep explicit calls mirroring the C sequence.
- Preserve user-facing failure behavior from the C tool:
  - usage errors produce the expected exit status
  - I/O and parse failures are reported to stderr
  - write paths propagate partial/failed writes correctly

## Implementation Phases

## Phase 1: Establish Binary Skeleton and Type Inventory

### Goals
Create the Rust crate entry point and migrate the top-level control skeleton before detailed logic conversion.

### Tasks
- Create `Cargo.toml` for a binary crate targeting Rust 1.78+ stable.
- Add `src/main.rs`.
- Read `yank.c` and inventory each anonymous C struct by usage site.
- Define minimal private Rust structs/enums corresponding to the actual C aggregates.
- Port `usage` and `main` first, with placeholder internal calls if necessary.
- Decide the common result/error type used across the module.
- Map C global/static state, if any, into Rust module-level constants or explicit state structs.

### Deliverables
- Compiling Rust binary skeleton
- Initial type definitions with stable names based on actual roles
- Top-level argument and exit-path scaffolding

### Acceptance Criteria
- `cargo test` compiles successfully
- `cargo run -- ...` reaches the Rust entry point and usage path
- No unresolved design gaps around ownership of the main runtime state

## Phase 2: Port Core Parsing and Non-terminal Processing

### Goals
Migrate the logic that does not depend on terminal mode transitions first, so parsing and comparison behavior can be stabilized independently.

### Tasks
- Port `strtopat` with a direct Rust representation of the pattern data.
- Port `fcmp` using standard-library file and byte comparison primitives while preserving existing semantics.
- Port `input` and any associated input record/file list processing.
- Port `xwrite` as the common write helper, ensuring retry/complete-write behavior matches the C intent.
- Add unit tests for:
  - pattern parsing edge cases
  - file comparison outcomes
  - write helper behavior using in-memory writers where practical

### Deliverables
- Working helper layer for parsing, comparison, and base I/O
- Test coverage for deterministic helper functions

### Acceptance Criteria
- `cargo test` passes helper-focused tests
- No unsafe code is introduced unless direct OS interaction proves unavoidable
- Helper functions return structured errors instead of implicit failure states

## Phase 3: Port Terminal I/O Path and Main Operational Logic

### Goals
Migrate the interactive/terminal-facing portion of the module with behavior preserved and teardown paths made explicit.

### Tasks
- Port `tsetup`, `tend`, `tgetc`, `twrite`, and `tputs`.
- Keep terminal handling as close as possible to the C file’s existing behavior.
- If the C implementation relies on low-level terminal APIs, confine that logic to small isolated functions inside `src/main.rs`.
- Port `yank` using the previously migrated parsing, comparison, and write helpers.
- Port `tmain` as the main operational dispatcher.
- Wire `main` to call `tmain` and translate results into process exit codes/messages.

### Deliverables
- Full end-to-end Rust execution path for the main module
- Explicit terminal lifecycle handling
- Preserved operational flow from argument parsing through final output

### Acceptance Criteria
- The program executes the primary CLI flow without placeholder logic
- Terminal setup/teardown occurs on both success and failure paths
- Output and interactive input paths are functionally aligned with the C implementation

## Phase 4: Behavioral Validation and Cleanup

### Goals
Verify parity with the C module and remove migration-only scaffolding.

### Tasks
- Compare Rust behavior against the C implementation for:
  - argument validation
  - non-interactive file/path handling
  - interactive terminal paths
  - error reporting and exit codes
- Tighten type names and field types to match final observed usage.
- Remove dead code, temporary adapters, and redundant conversions.
- Add focused regression tests for bugs found during parity validation.
- Ensure all write/read/error branches are covered by at least one test or manual validation note.

### Deliverables
- Cleaned Rust port of `main_root`
- Finalized tests and stable internal interfaces

### Acceptance Criteria
- `cargo test` passes
- The Rust module fully replaces the behavior implemented by `yank.c` within the current scope
- No extra modules, frameworks, or unsupported features were introduced