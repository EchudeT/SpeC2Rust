# Implementation Plan: module_src_main.c_26

## Summary

This module is the program entry layer migrated from `src/main.c`. Its scope is limited to command-line startup, option classification, initialization sequencing, configuration parsing, symbol-selection helpers, numeric and level parsing, path expansion, and process termination behavior currently embedded in the C entrypoint.

The Rust implementation should keep this logic concentrated in the standard `src/main.rs` file and migrate the listed C functions into private helper functions plus a small set of internal structs/enums where needed. The technical approach is a direct behavioral port:

- preserve the current startup flow around `init`, `parse_rc`, and `main`
- replace C string and pointer handling with `String`, `&str`, `PathBuf`, and owned collections
- replace implicit global-state mutation with narrowly scoped Rust state holders where the C code currently relies on file-level statics
- convert fatal allocation/termination paths such as `xalloc_die` into explicit Rust error propagation where practical, with controlled process exit remaining in `main`
- keep parsing helpers (`number`, `parse_level_string`, `find_option_type`, `tildexpand`) as small, local functions rather than spreading them into extra modules

The migration should aim for behavior parity with the existing C module and avoid introducing new capabilities or architectural layers beyond what is necessary for safe Rust ownership and error handling.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only by default
  - `std::env` for argument and environment handling
  - `std::path::{Path, PathBuf}` for path processing
  - `std::fs` and `std::io` for rc/config file reading
  - `std::process` for exit handling
  - `std::num` for numeric parsing
- **Testing**: `cargo test`
- **Performance Goals**:
  - maintain startup-path efficiency comparable to the C implementation
  - avoid unnecessary string cloning during option and config parsing
  - keep parsing helpers linear in input size
  - preserve single-pass handling for command-line and rc-file processing where the C logic already does so

## Module Mapping

### C to Rust File Mapping

- `src/main.c` → `src/main.rs`

### Function Mapping

All listed functions remain in the same Rust file unless a minimal internal type definition is required.

- `CHAR_TO_SM` → `const fn` or small private helper function in `src/main.rs`
- `find_option_type` → private function in `src/main.rs`
- `symbol_override` → private function in `src/main.rs`
- `number` → private parsing function in `src/main.rs`
- `parse_level_string` → private parsing function in `src/main.rs`
- `tildexpand` → private path expansion function in `src/main.rs`
- `parse_rc` → private function in `src/main.rs`
- `globals_only` → private predicate/helper in `src/main.rs`
- `include_symbol` → private predicate/helper in `src/main.rs`
- `xalloc_die` → private termination/error helper in `src/main.rs`
- `init` → private initialization function in `src/main.rs`
- `main` → Rust crate entrypoint in `src/main.rs`

### Rust Module Shape

Keep a restrained single-file structure:

- top-level constant/enum definitions needed by option and level parsing
- internal state structs replacing C global aggregates as needed
- private helper functions in migration order
- `fn main()` as the final orchestration entrypoint

## Data Model

The C analysis only reports multiple anonymous data structures. Since names are unavailable, the Rust plan should map them by role as they are discovered in `src/main.c`, without inventing extra abstractions.

### Expected Mapping Strategy

- **Anonymous option-description structs** → named Rust `struct` or `enum` in `src/main.rs`
  - use when `find_option_type` depends on static option tables
  - prefer `&'static str` fields for fixed option names
  - use enums for option categories instead of integer tags

- **Anonymous parser/config structs** → named Rust `struct`
  - use owned `String`/`PathBuf` instead of raw `char *`
  - use `Vec<T>` instead of manual dynamic arrays
  - use `Option<T>` for nullable fields

- **Anonymous flag/state aggregates** → named Rust `struct`
  - group mutable startup/config state that is currently spread across C globals
  - use `bool` for binary flags rather than integer markers
  - use small enums for multi-state values where C uses macros or integer constants

- **Character/symbol classification tables** → `const`, `static`, array, or enum-backed helper
  - if `CHAR_TO_SM` indexes a table, represent it as a bounded Rust array or match expression
  - enforce range checks before indexing

### C-to-Rust Type Conventions

- `char *` → `String`, `&str`, or `PathBuf` depending on ownership and semantic role
- nullable `char *` → `Option<String>` or `Option<PathBuf>`
- integer option tags/macros → Rust `enum`
- bitflag-like integers → `u32` only if bit operations are required by the C logic; otherwise use enums/bools
- manual linked or resizable collections → `Vec<T>`
- file handles → `std::fs::File` / buffered readers
- error-return integers → `Result<T, E>` internally, with final exit code mapping in `main`

### Memory Management Decisions

- eliminate manual allocation/free behavior by using Rust ownership
- convert any C buffer growth logic into `String` or `Vec<u8>`
- remove explicit out-of-memory pathways except for preserving externally visible fatal-exit semantics in `xalloc_die`-equivalent behavior if the current flow requires a specific message/exit path
- avoid borrowing from temporary argument/config buffers across mutation points

### Error Handling Decisions

- parsing and initialization helpers should return `Result`
- use a small internal error enum for:
  - invalid numeric input
  - invalid level string
  - malformed rc/config lines
  - file I/O failures
  - invalid option forms
- keep process exit and user-facing diagnostics centralized in `main`
- if exact C behavior requires immediate abort from certain helper paths, wrap that behavior narrowly rather than spreading `process::exit` calls across helpers

## Implementation Phases

## Phase 1: Establish entrypoint skeleton and state mapping

- Create `src/main.rs` as the Rust port target for `src/main.c`.
- Identify all file-scope globals and anonymous structs used by the listed functions.
- Introduce the minimum named Rust structs/enums required to represent:
  - startup/config state
  - option categories
  - symbol inclusion/override state
  - level parsing outputs
- Port constant tables and macro-like helpers, including `CHAR_TO_SM`, into safe Rust constants or helper functions.
- Define a compact internal error enum and a `type Result<T> = ...` alias for this file.

**Exit criteria**:
- Rust file builds with placeholder function bodies.
- All C globals referenced by the target functions have a planned Rust owner or state container.
- No unsafe code is introduced unless a specific C dependency makes it unavoidable.

## Phase 2: Port pure parsing and selection helpers

- Port `find_option_type`.
- Port `number`.
- Port `parse_level_string`.
- Port `globals_only`.
- Port `include_symbol`.
- Port `symbol_override`.
- Port `tildexpand`.

Implementation guidance:
- translate string scanning logic directly using `&str`, byte iteration, or `chars()` only where UTF-8 semantics are acceptable; prefer byte-based handling if the C code is ASCII-oriented
- preserve exact acceptance/rejection rules for numeric and level parsing
- preserve path expansion behavior only to the extent present in the C code; do not generalize beyond current tilde-handling cases
- replace C sentinel returns with `Option`/`Result`

**Exit criteria**:
- helper functions compile and are covered by focused unit tests derived from observed C cases
- invalid inputs produce deterministic Rust errors matching current behavior as closely as practical
- no helper leaks ownership or depends on mutable global state unless required by the original logic

## Phase 3: Port configuration parsing and initialization flow

- Port `parse_rc`.
- Port `init`.
- Recreate rc/config file reading order and state updates from the C implementation.
- Map C line/token parsing into standard library string processing while preserving existing syntax and precedence rules.
- Keep initialization side effects explicit through mutable state passed into helpers instead of hidden globals where possible.

Implementation guidance:
- use `std::fs::File` and buffered reads
- preserve current handling for missing rc files versus malformed rc files
- keep option/state mutation order consistent with the C module

**Exit criteria**:
- initialization and rc parsing execute end-to-end in Rust
- unit tests cover representative rc line forms and error cases
- state transitions match the original ordering and precedence rules

## Phase 4: Port main orchestration and finalize behavior parity

- Port `xalloc_die` semantics into the Rust error/exit path as narrowly as necessary.
- Port `main` and wire it to:
  - argument collection
  - initialization
  - rc/config parsing
  - option dispatch/classification
  - final exit-code production
- Ensure all diagnostics and exit statuses remain aligned with the C module’s observable behavior.
- Remove temporary placeholders and confirm all migrated helpers are used from the Rust entrypoint.

Testing focus:
- argument combinations that exercise `find_option_type`
- startup without rc file
- startup with malformed config
- symbol inclusion/override cases
- invalid number and level parsing
- path expansion edge cases already handled in C

**Exit criteria**:
- `cargo test` passes
- `cargo build` succeeds on branch `089-module_src_main.c_26-rust-port`
- the Rust `main` fully replaces the C module logic within the defined scope, without adding new modules or extra facilities