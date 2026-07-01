# Implementation Plan: module_doc_main_01

## Summary

This module cluster contains four standalone C entrypoint files under `doc/`, each defining its own `main` function:

- `doc/d.c`
- `doc/foo.c`
- `doc/wc.c`
- `doc/whoami.c`

The Rust implementation should preserve that layout as four small executable targets rather than merging behavior into a single program. The technical approach is a direct file-by-file migration into Rust binaries under standard Cargo conventions, keeping each executable isolated and translating local C control flow into straightforward Rust functions.

Because the available analysis only identifies anonymous data structures and `main` entrypoints, the port should remain conservative:
- map each C file to one Rust binary source file,
- keep logic local to the corresponding binary unless a shared helper is clearly repeated during migration,
- prefer standard library APIs for argument handling, I/O, string processing, and filesystem/user-context access where applicable,
- convert C-style integer status returns into Rust `Result`-based internal flow with explicit process exit codes at the binary boundary.

Memory management should rely entirely on Rust ownership and borrowing, replacing any implicit C buffer lifetime assumptions with scoped `String`, `Vec<u8>`, and iterator-based processing. Error handling should be explicit and minimal: propagate internal errors with `Result`, print diagnostics to stderr when required by the existing CLI behavior, and terminate with conventional nonzero exit codes.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended at planning stage, since the provided analysis does not justify external dependencies
- **Testing**:
  - `cargo test`
  - binary-focused unit tests where logic can be factored into small internal functions
  - smoke tests for argument parsing and status-code behavior where practical
- **Performance Goals**:
  - match or closely approximate the C utilities’ runtime characteristics for normal command-line usage
  - avoid unnecessary heap allocations beyond what is required by Rust string and I/O APIs
  - use streaming I/O for file/content processing paths instead of reading entire inputs into memory unless the original code requires whole-buffer semantics
  - preserve simple, predictable startup and execution costs appropriate for small command-line tools

## Module Mapping

### C to Rust File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `doc/d.c` | `src/bin/d.rs` | Direct migration of the standalone `main` program |
| `doc/foo.c` | `src/bin/foo.rs` | Direct migration of the standalone `main` program |
| `doc/wc.c` | `src/bin/wc.rs` | Direct migration of the standalone `main` program |
| `doc/whoami.c` | `src/bin/whoami.rs` | Direct migration of the standalone `main` program |

### Cargo Structure

The Rust project should use standard Cargo binary layout:

```text
cflow-new/
├── Cargo.toml
└── src/
    └── bin/
        ├── d.rs
        ├── foo.rs
        ├── wc.rs
        └── whoami.rs
```

If repeated internal logic is discovered during migration, it may be moved into a minimal shared module such as `src/lib.rs`, but only for code already duplicated across these four binaries. No additional abstraction layer should be introduced speculatively.

### Function Mapping

Since each source file exposes `main`, the expected Rust pattern is:

- C `main(...)` -> Rust `fn main()`
- internal procedural blocks -> local helper functions returning `Result<_, _>` as needed
- C exit-status returns -> `std::process::ExitCode` or explicit `std::process::exit`

Preferred boundary pattern:

```rust
fn main() -> std::process::ExitCode
```

with inner logic such as:

```rust
fn run() -> Result<(), String>
```

This keeps Rust error propagation local while preserving C-style process status behavior.

## Data Model

The analysis identifies only anonymous data structures, with no named struct inventory or fields. The plan should therefore avoid inventing persistent Rust models unless the source code requires them during translation.

### Data-Structure Mapping Strategy

| C Representation | Rust Representation | Usage Guidance |
|---|---|---|
| anonymous struct/local aggregate | local `struct` with a descriptive Rust name, only if required | Introduce only when a C aggregate has multiple related fields that benefit from named access |
| stack scalar variables | local Rust bindings | Prefer immutable bindings, using `mut` only where necessary |
| C strings (`char *`, string literals, argv elements) | `&str`, `String`, or `OsString`/`OsStr` depending on platform interaction | Use `OsString` for raw CLI/path values, `String` only when UTF-8 text semantics are required |
| byte/file buffers | `Vec<u8>` or buffered readers | Prefer streaming over whole-buffer loading |
| integer flags/counters | integer primitives (`i32`, `u64`, `usize`) | Choose widths based on actual C usage and indexing/counting semantics |

### Memory Management Decisions

- Replace manual buffer lifetime management with owned Rust values.
- Avoid shared mutable state across binaries.
- Use borrowing for temporary string/file processing where practical.
- Keep stack/local data scoped to each executable’s `run` path.

### Error Handling Decisions

- Translate C sentinel/error-code checks into `Result` returns.
- Use `std::io::Result` for filesystem and stream operations.
- Convert final errors into stderr output and a nonzero process exit status.
- Do not introduce custom error frameworks unless the source complexity clearly requires them.

## Implementation Phases

### Phase 1: Project Skeleton and Binary Entrypoints

- Create the Rust branch structure for `001-module_doc_main_01-rust-port`.
- Add Cargo project configuration if not already present.
- Create four binary files:
  - `src/bin/d.rs`
  - `src/bin/foo.rs`
  - `src/bin/wc.rs`
  - `src/bin/whoami.rs`
- For each binary, establish:
  - `main` entrypoint
  - internal `run` function
  - basic stderr/error-to-exit-code pattern
- Confirm the project builds successfully with all four binaries present.

**Exit criteria**:
- `cargo build` succeeds
- all four binaries compile with placeholder or migrated control flow

### Phase 2: File-by-File Logic Migration

- Port `doc/d.c` into `src/bin/d.rs` with direct control-flow preservation.
- Port `doc/foo.c` into `src/bin/foo.rs` with direct control-flow preservation.
- Port `doc/wc.c` into `src/bin/wc.rs`, paying particular attention to streaming input/counting logic if present.
- Port `doc/whoami.c` into `src/bin/whoami.rs`, using standard-library environment/user-related APIs where sufficient for the original behavior.
- During each port:
  - translate local anonymous C aggregates only when needed
  - replace C pointer/null checks with `Option`/`Result`
  - preserve process status semantics
  - avoid introducing cross-binary abstractions unless identical code is duplicated

**Exit criteria**:
- each C file has a corresponding working Rust implementation
- no remaining placeholder entrypoints
- basic command-line behavior is represented in Rust

### Phase 3: Consolidation of Shared Technical Patterns

- Review the four binaries for duplicated helper logic introduced during direct migration.
- Extract only clearly repeated, already-existing logic into minimal shared internal functions or `src/lib.rs`.
- Normalize:
  - argument collection patterns
  - common I/O error reporting
  - exit-code conversion behavior
- Keep module boundaries aligned with the original four-program layout.

**Exit criteria**:
- duplicated support code is reduced only where justified by actual repetition
- binary responsibilities remain unchanged from the C layout
- no unnecessary helper modules are added

### Phase 4: Testing and Behavioral Verification

- Add unit tests for any factored pure functions.
- Add targeted tests for:
  - argument edge cases
  - error-path exit behavior
  - file/input handling where applicable
- Run `cargo test` and resolve translation mismatches.
- Perform manual smoke checks of each binary to confirm expected CLI execution shape.

**Exit criteria**:
- `cargo test` passes
- migrated binaries have verified basic behavior
- error handling and memory usage rely solely on safe Rust standard patterns