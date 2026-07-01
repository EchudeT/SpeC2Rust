# Implementation Plan

## Summary

Port the option-file parsing portion of `src/main.c` into a focused Rust module that preserves the existing control flow and observable behavior of `optfile_lookup` and `parseopt_from_rc`. The Rust implementation should keep the migration narrow: translate the current file lookup and rc-driven option parsing logic into safe Rust using standard library filesystem and string handling APIs, while retaining the existing parsing order, lookup precedence, and error propagation semantics as closely as practical.

The technical approach is to extract the C logic from `src/main.c` into a Rust module with two primary functions mirroring the original responsibilities:

- `optfile_lookup`: resolve and open or identify the configuration/option file path using `std::path` and `std::fs`.
- `parseopt_from_rc`: read and parse rc file content into the program’s existing option state using line-based processing and explicit parse helpers.

Implementation should avoid introducing new abstractions beyond what is needed to represent current state safely. Memory ownership will move from implicit C buffer management to Rust-owned `String`, `PathBuf`, and explicit option/state structs. Error handling should use `Result` with a small module-local error type or `std::io::Error`-based propagation where sufficient.

## Technical Context

### Language/Version
- Rust 1.75+
  Chosen to provide stable modern standard-library support for file I/O, path handling, and idiomatic error propagation without requiring newer language features.

### Primary Dependencies
- Rust standard library only:
  - `std::fs`
  - `std::io`
  - `std::path`
  - `std::env`
  - `std::ffi` only if exact C-like argument handling remains necessary during migration

No third-party crates are recommended from the available evidence. The source scope does not justify external parsing, error, or CLI libraries.

### Testing
- `cargo test`

Testing should focus on:
- option file path resolution behavior
- rc file reading and line parsing
- handling of missing files, unreadable files, and malformed entries
- preservation of precedence/order semantics inferred from the C flow

### Performance Goals
- Preserve equivalent runtime characteristics for small configuration files.
- Avoid unnecessary heap churn beyond standard Rust string/path ownership.
- Parse rc files in a single pass over file contents or buffered lines.
- Keep filesystem checks minimal and aligned with the original lookup order.

## Module Mapping

### C to Rust File Mapping
- `src/main.c`
  - Extract relevant logic into a dedicated Rust module file:
    - `src/parseopt_file.rs`

If the Rust port already keeps a central binary entry file, only the migrated functions and their directly required helpers should move into `src/parseopt_file.rs`; integration glue should remain in the existing Rust entrypoint without broad restructuring.

### Function Mapping
- `optfile_lookup`
  - C role: locate/select the rc/options file
  - Rust target: `pub(crate) fn optfile_lookup(...) -> Result<Option<PathBuf>, ParseOptFileError>` or closest signature compatible with surrounding ported code

- `parseopt_from_rc`
  - C role: load and parse options from rc file
  - Rust target: `pub(crate) fn parseopt_from_rc(...) -> Result<(), ParseOptFileError>`

### Internal Helpers
Only introduce helper functions required to split C logic safely, for example:
- line trimming/comment filtering
- key/value extraction
- option application into the existing state structure

Helpers should remain module-private and directly traceable to logic already present in `src/main.c`.

## Data Model

The analysis lists only anonymous C data structures, so the Rust data model should be derived conservatively from actual usage in `optfile_lookup` and `parseopt_from_rc`, not from speculative restructuring.

### Data-Structure Mapping Principles
- C anonymous structs used only as local aggregates
  - Rust mapping: local named `struct` only if the fields must persist across helper boundaries
  - Otherwise: tuples, local variables, or small private structs

- C string pointers / mutable char buffers
  - Rust mapping:
    - `String` for owned mutable text
    - `&str` for borrowed parsed segments
    - `PathBuf` for filesystem paths

- C nullable pointers
  - Rust mapping: `Option<T>`

- C integer status/error returns
  - Rust mapping: `Result<T, E>` where call sites already branch on success/failure
  - If exact tri-state behavior exists, use `Result<Option<T>, E>`

- C flag fields / booleans
  - Rust mapping: `bool`

- C enums expressed as integer constants
  - Rust mapping: private `enum` only when distinct parser states are clearly present in the original logic

### Expected Rust Structures
These should be introduced only if required by the migrated code:

```rust
struct RcEntry<'a> {
    key: &'a str,
    value: &'a str,
}
```

Use this only if parsing repeatedly benefits from a clear intermediate representation; otherwise parse key/value inline.

```rust
enum ParseOptFileError {
    Io(std::io::Error),
    InvalidEntry,
    InvalidPath,
}
```

This error type should remain minimal and only cover cases actually observed in the C logic.

```rust
struct OptionFileLookup {
    path: std::path::PathBuf,
}
```

Add only if `optfile_lookup` returns more than a raw path or if metadata from lookup is already represented in C. If a plain `PathBuf` is enough, avoid this struct.

### Memory Management Notes
- Replace C stack/heap buffers with owned Rust `String` and `PathBuf`.
- Avoid exposing references tied to temporary line buffers outside parsing loops.
- Apply options immediately while iterating lines unless the original logic requires deferred application.
- Use buffered reading if file content is processed line by line.

## Implementation Phases

## Phase 1: Isolate and map the C logic
- Inspect `src/main.c` and extract the exact code paths for `optfile_lookup` and `parseopt_from_rc`.
- Identify all direct dependencies:
  - global option state touched by rc parsing
  - helper functions invoked from these two functions
  - constants controlling search paths, file names, separators, and comment syntax
- Define the minimal Rust function signatures needed to integrate with the current port branch.
- Decide whether each anonymous C aggregate can remain implicit or needs a small private Rust struct.

### Deliverables
- `src/parseopt_file.rs` created
- Rust signatures for the two migrated functions established
- dependency list documented in code comments or TODO markers for remaining call-site wiring

## Phase 2: Port option-file lookup
- Implement `optfile_lookup` in Rust using:
  - `std::env` for environment/home directory access only if the C code does so
  - `std::path::PathBuf` for path assembly
  - `std::fs` for existence/readability checks as required by current behavior
- Preserve the original lookup order exactly.
- Represent “file not found” distinctly from hard I/O failure if the C function does so.
- Keep return values compatible with the rest of the program’s option parsing flow.

### Deliverables
- Rust `optfile_lookup` implementation
- unit tests for:
  - existing file found in expected location
  - no candidate file found
  - unreadable/invalid path behavior where supported by original semantics

## Phase 3: Port rc file parsing
- Implement `parseopt_from_rc` using `std::fs::File` and `std::io` line reading or full-file read, depending on the C parsing style.
- Preserve:
  - trimming rules
  - comment handling
  - key/value splitting
  - duplicate handling/precedence
  - error behavior for malformed lines
- Translate C string/token logic into explicit Rust slices and owned strings only where necessary.
- Apply parsed values directly into the existing option state interface rather than introducing a new configuration subsystem.

### Deliverables
- Rust `parseopt_from_rc` implementation
- module-private parsing helpers only where needed
- unit tests covering:
  - valid rc entries
  - comments/blank lines
  - malformed entries
  - repeated options and precedence behavior

## Phase 4: Integrate and stabilize
- Wire the new module into the Rust equivalent of `main.c` without broad entrypoint refactoring.
- Remove or retire duplicated transitional logic once the new functions are used.
- Normalize error propagation so lookup and parse failures match existing program behavior at call sites.
- Run `cargo test` and fix any behavior mismatches revealed by tests.

### Deliverables
- integration of `src/parseopt_file.rs` into the active binary/library path
- tests passing under `cargo test`
- migration notes in code comments for any intentionally preserved C-style behavior