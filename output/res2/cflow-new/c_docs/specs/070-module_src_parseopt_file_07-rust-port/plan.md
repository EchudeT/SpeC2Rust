# Implementation Plan: module_src_parseopt_file_07

## Summary

This module ports the option-file lookup and rc-driven option parsing logic from `src/main.c` into Rust, preserving the existing execution flow and behavior boundaries of `optfile_lookup` and `parseopt_from_rc` without adding new configuration features.

The Rust implementation should keep the logic close to the original C code:
- translate file lookup behavior into `std::path` and `std::fs` operations,
- translate rc-file parsing into explicit line/string processing with `String` and slices,
- preserve the existing call ordering and failure behavior,
- replace implicit C memory ownership with scoped Rust ownership,
- convert C-style status/error propagation into `Result` where possible, while keeping caller-visible behavior aligned with the current program.

The migration should remain narrowly scoped to the code currently embodied in `src/main.c` for these functions, with supporting types added only as needed to represent existing data and parsing state.

## Technical Context

### Language / Version
- Rust 1.78+ edition 2021

### Primary Dependencies
- Rust standard library
- No third-party crates required by current evidence

### Testing
- `cargo test`

### Performance Goals
- Match the practical performance characteristics of the C implementation for startup-time option file handling.
- Avoid unnecessary heap allocations beyond those required for safe string/path handling.
- Perform file lookup and rc parsing in a single pass where the C code does so.
- Keep parsing overhead negligible relative to file I/O.

## Module Mapping

### Source File Mapping
- C: `src/main.c`
- Rust: `src/main.rs` or `src/lib.rs` plus `src/main.rs`, depending on current crate layout

### Function Mapping
- `optfile_lookup` -> `optfile_lookup`
- `parseopt_from_rc` -> `parseopt_from_rc`

### Recommended Rust Placement
To keep the port restrained and close to the original source:
- if the project is currently binary-oriented, place the migrated functions in `src/main.rs` first;
- if extraction is needed for testability, move only these functions and their direct helper types into a single focused module such as `src/parseopt_file.rs`, and call them from `src/main.rs`.

No broader module breakup is planned.

## Data Model

The input analysis exposes only anonymous C data structures, so the Rust data model should be derived strictly from the fields actually touched by `optfile_lookup` and `parseopt_from_rc`.

### Mapping Strategy
Each anonymous C struct used by these functions should be mapped by role, not by synthetic one-to-one naming from the parser output. Introduce Rust structs/enums only when a concrete grouping is needed by migrated code.

### Expected C-to-Rust Conversions
- C string pointers (`char *`, `const char *`) -> `String`, `&str`, `PathBuf`, or `&Path` depending on usage
- C file handles (`FILE *`) -> `std::fs::File` with `std::io::{BufRead, BufReader}`
- C integer status codes -> `Result<T, E>` internally; convert back to legacy-compatible return shape at the boundary if required
- C booleans / flag integers -> `bool`
- C arrays of chars used as temporary buffers -> `String` or fixed local byte buffer only if exact low-level behavior is required
- Nullable pointers -> `Option<T>` / `Option<&T>` / `Option<PathBuf>`

### Rust Data Structures
Because the concrete anonymous structs are not identified in the analysis, use the following restrained plan:

- Existing global/program option state touched by `parseopt_from_rc`
  - map to the crate’s existing Rust option/state struct if already present
  - otherwise define one local struct that contains only the fields directly read or mutated by this module

- Option-file lookup result
  - if the C code returns a path through output parameters, replace with:
    - `Option<PathBuf>` when absence is non-error
    - `Result<PathBuf, ParseOptFileError>` when absence or I/O should be distinguished

- Rc parsing context
  - keep as local variables where possible
  - introduce a small struct only if multiple values are passed together repeatedly during parsing

### Error Type
Define a narrow module-local error enum only if needed by the translated logic, for example:
- file open/read failure
- invalid rc line / parse failure
- path resolution failure

If the surrounding port already has a shared error type, reuse it instead of adding a new abstraction.

## Implementation Phases

### Phase 1: Extract and map current C behavior
- Inspect `src/main.c` and isolate the exact logic and dependencies of `optfile_lookup` and `parseopt_from_rc`.
- Identify:
  - inputs and outputs,
  - global state reads/writes,
  - file path resolution rules,
  - rc-line tokenization/parsing rules,
  - current failure and ignore conditions.
- Document the minimal set of anonymous C structs and fields actually used by these functions.
- Decide whether the Rust port can remain in `src/main.rs` or needs one dedicated module for locality and tests.

### Phase 2: Port `optfile_lookup`
- Implement the path/file lookup logic using `std::path::{Path, PathBuf}` and `std::fs` metadata/open checks as needed.
- Preserve lookup order and fallback behavior exactly.
- Replace C buffer-based path construction with owned `PathBuf` composition.
- Replace pointer/null-based returns with `Option` or `Result`, while keeping the externally observable behavior unchanged.
- Validate edge cases:
  - missing file,
  - unreadable file,
  - empty path inputs,
  - any environment/home/current-directory lookups that the C function currently uses.

### Phase 3: Port `parseopt_from_rc`
- Implement rc-file reading using `File` + `BufReader`.
- Translate line processing carefully:
  - trim or preserve whitespace only where the C logic does,
  - skip comments/blank lines only if the original function does,
  - preserve option parsing order,
  - apply parsed values to the same option state fields as in C.
- Convert manual token/buffer handling into safe string slicing and iteration.
- Keep error handling aligned with current behavior:
  - ignore malformed lines if C ignores them,
  - stop on parse errors only if C stops,
  - preserve diagnostics behavior if this function emits messages.

### Phase 4: Integrate and verify
- Wire the Rust functions into the existing startup/option parsing flow in the same order as the C implementation.
- Remove or disable the original C-path logic for these functions within the Rust branch.
- Add focused unit tests for:
  - successful option-file lookup,
  - no-file-found behavior,
  - rc parsing of valid lines,
  - handling of comments/blank lines,
  - malformed or partial rc entries as currently expected.
- Run `cargo test` and confirm the migrated functions preserve behavior without introducing broader structural changes.