# Implementation Plan: module_doc_main_01

## Summary

This module cluster contains four standalone C entry-point files under `doc/`: `d.c`, `foo.c`, `wc.c`, and `whoami.c`, each exposing its own `main` function. The Rust implementation should preserve this shape by migrating them as separate binary targets rather than combining behavior into a shared abstraction without evidence.

The technical approach is a direct port of each C file into a corresponding Rust binary under standard Cargo conventions, using the Rust standard library for argument handling, I/O, process exit behavior, and filesystem or environment queries as needed by each migrated file. Any anonymous C-local data structures should be converted into file-local Rust structs or enums only if required by the original implementation; otherwise, transient values should remain as plain local variables.

The plan prioritizes:
- one-to-one file migration,
- preservation of command-line behavior,
- explicit error propagation with `Result` where practical,
- no additional feature expansion beyond the existing C files.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.76 or newer

### Primary Dependencies
- Rust standard library only:
  - `std::env` for argument and environment access
  - `std::fs` and `std::io` for file and stream handling
  - `std::path` for path-safe operations
  - `std::process::ExitCode` for exit status handling

No third-party crates are recommended because the input provides no evidence of requirements beyond basic command-line utility behavior.

### Testing
- `cargo test`

Testing approach:
- unit tests for any extracted pure helper functions,
- integration-style tests where feasible for binary behavior,
- validation of exit behavior and output formatting using standard test facilities.

### Performance Goals
- Match or closely approximate the original C utilities' runtime characteristics for normal command-line use.
- Avoid unnecessary allocations when processing arguments, input text, or filesystem paths.
- Prefer streaming and borrowed string/path handling over whole-input buffering unless the original file behavior requires buffering.
- Keep startup overhead minimal by using only the standard library and direct binary targets.

## Module Mapping

### C to Rust File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `doc/d.c` | `src/bin/d.rs` | Direct migration of standalone `main` program |
| `doc/foo.c` | `src/bin/foo.rs` | Direct migration of standalone `main` program |
| `doc/wc.c` | `src/bin/wc.rs` | Direct migration of standalone `main` program |
| `doc/whoami.c` | `src/bin/whoami.rs` | Direct migration of standalone `main` program |

### Function Mapping

Because each C source contains only `main`, each should map to a Rust binary entry point with minimal helper extraction:

| C Function | Rust Mapping |
|---|---|
| `main` in `doc/d.c` | `fn main()` or `fn main() -> ExitCode` in `src/bin/d.rs` |
| `main` in `doc/foo.c` | `fn main()` or `fn main() -> ExitCode` in `src/bin/foo.rs` |
| `main` in `doc/wc.c` | `fn main()` or `fn main() -> ExitCode` in `src/bin/wc.rs` |
| `main` in `doc/whoami.c` | `fn main()` or `fn main() -> ExitCode` in `src/bin/whoami.rs` |

### Project Structure

Recommended Rust layout:

```text
cflow-new/
├─ Cargo.toml
├─ src/
│  └─ bin/
│     ├─ d.rs
│     ├─ foo.rs
│     ├─ wc.rs
│     └─ whoami.rs
└─ tests/
   └─ module_doc_main_01.rs
```

A shared library module should not be introduced unless duplicated logic becomes evident during migration and can be extracted without changing behavior.

## Data Model

The analysis reports only anonymous data structures and does not identify stable named structs. Therefore, data modeling should remain minimal and file-local.

### Data-Structure Mapping

| C Data Structure | Rust Mapping | Guidance |
|---|---|---|
| anonymous structure in `doc/d.c` | local `struct` or tuple/local variables | Introduce only if the C code groups state meaningfully |
| anonymous structure in `doc/foo.c` | local `struct` or tuple/local variables | Prefer plain locals unless repeated state manipulation exists |
| anonymous structure in `doc/wc.c` | local `struct` for counters if present | A small private struct is appropriate for grouped counts |
| anonymous structure in `doc/whoami.c` | local `struct` or plain locals | Keep minimal and scoped to the binary file |

### C-to-Rust Type Guidance

Typical conversion rules to apply during migration:

| C Pattern | Rust Pattern |
|---|---|
| `char *argv[]` / argument parsing | `std::env::args_os()` or `std::env::args()` |
| C strings | `String`, `OsString`, or `PathBuf` depending on usage |
| integer counters | `usize`, `u64`, or signed integer only if negative values are semantically required |
| file handles / `FILE *` | `std::fs::File`, `std::io::Stdin`, `std::io::Stdout` |
| status return from `main` | `std::process::ExitCode` |

### Memory Management

- Replace manual C ownership and lifetime assumptions with Rust’s scoped ownership.
- Eliminate raw pointer handling unless the original logic absolutely depends on byte-level buffer traversal; even then, prefer safe slice iteration.
- Use stack-local structs for compact grouped state.
- Avoid cloning strings or buffers unless needed for output or multi-step processing.

### Error Handling

- Convert C-style status checks into `Result`-based helper functions.
- Keep `main` responsible for:
  - argument validation,
  - printing user-facing error messages to stderr,
  - returning appropriate exit codes.
- Use standard I/O error propagation with `?` inside helper functions.
- Preserve original utility semantics by not introducing layered error taxonomies unless required by the migrated code.

## Implementation Phases

## Phase 1: Scaffold the Rust Binary Targets

### Goal
Create the Cargo layout and establish one binary per C source file.

### Tasks
- Create or update `Cargo.toml` for a standard multi-binary project.
- Add:
  - `src/bin/d.rs`
  - `src/bin/foo.rs`
  - `src/bin/wc.rs`
  - `src/bin/whoami.rs`
- For each binary:
  - add a minimal `main`,
  - establish argument intake and exit-code pattern,
  - define file-local helper function stubs only as needed.

### Deliverables
- Compilable Rust project with four binary entry points.
- No shared support module unless direct duplication is already obvious.

## Phase 2: Port Program Logic File by File

### Goal
Migrate each C file directly into its corresponding Rust binary with behavior preservation.

### Tasks
- Port `doc/whoami.c` to `src/bin/whoami.rs`
  - typically the smallest command utility and a low-risk migration starting point.
- Port `doc/foo.c` to `src/bin/foo.rs`
  - preserve its exact command-line and output behavior.
- Port `doc/d.c` to `src/bin/d.rs`
  - translate any file/path/environment interactions using `std`.
- Port `doc/wc.c` to `src/bin/wc.rs`
  - use streaming reads and grouped counters where applicable.

### Migration Rules
- Keep helpers private to the file unless proven reusable.
- Preserve processing order, output shape, and exit behavior.
- Replace anonymous C structs with minimal Rust-local structs only when they improve direct state translation.
- Keep parsing and I/O code safe and explicit.

### Deliverables
- Functional Rust equivalents for all four C binaries.
- No intentional behavior expansion.

## Phase 3: Normalize Error Paths and Resource Handling

### Goal
Align the migrated binaries with idiomatic but restrained Rust error handling.

### Tasks
- Refactor repetitive status checks into small `Result`-returning helpers inside each file.
- Ensure stderr output is used for operational failures.
- Map success/failure to explicit exit codes.
- Confirm all file and stdin/stdout usage relies on automatic resource cleanup via ownership and scope.
- Replace any provisional panicking code paths with controlled exits.

### Deliverables
- Stable binary behavior under normal and failure conditions.
- Clear ownership-based resource management with no manual cleanup logic.

## Phase 4: Add Tests and Verify Behavioral Parity

### Goal
Provide basic regression coverage for the migrated binaries.

### Tasks
- Add integration tests in `tests/module_doc_main_01.rs`.
- Cover, as applicable per binary:
  - invocation with expected arguments,
  - invocation with missing or invalid arguments,
  - output formatting,
  - non-zero exit cases,
  - representative input handling for `wc`-style counting behavior if present.
- Add unit tests only for pure helper logic extracted during migration.

### Deliverables
- `cargo test` passing.
- Core behavior checks for each migrated binary.
- Final review that project structure remains narrowly scoped to the original C module cluster.