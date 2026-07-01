# Implementation Plan: module_doc_d.c_03

## Summary

Port `doc/d.c` into a Rust module that preserves the existing directory-handling behavior represented by `isdir`, `ignorent`, and `printdir`. The Rust implementation should stay narrow in scope: translate current file-system inspection and directory entry filtering logic into idiomatic Rust using the standard library, while keeping observable behavior aligned with the C source.

The technical approach is to map:
- directory detection to `std::fs::metadata` / `std::path::Path`,
- directory iteration to `std::fs::read_dir`,
- entry-name filtering to string/path processing on `OsStr`/`Path`,
- output behavior to explicit formatting and `Write` usage where needed.

Memory ownership should move from manual C lifetime management to Rust-owned values (`PathBuf`, `String` where valid, or `OsString` where exact platform names matter). Error handling should replace implicit C failure paths with `Result`-based internal helpers, while preserving caller-visible behavior expected from the original module.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only (`std::fs`, `std::io`, `std::path`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for directory checks and traversal.
  - Avoid unnecessary path/string allocations during directory iteration.
  - Perform metadata queries only where required by the original functions.
  - Keep implementation single-pass over directory entries for `printdir`-style behavior.

## Technical Context Details

### Platform and Filesystem Handling
The source module operates on directories and directory entries, so the Rust port should use platform-native path types first:
- `Path` / `PathBuf` for filesystem paths
- `OsStr` / `OsString` for entry names where UTF-8 cannot be assumed

Conversion to `String` should occur only when formatting/output requires it. This avoids changing behavior for non-UTF-8 names and keeps the port closer to C’s byte-oriented handling.

### Error Handling Strategy
The original C code likely signals failures through integer return values, null checks, or skipped entries. In Rust:
- internal helpers should return `io::Result<T>` where filesystem access can fail,
- boolean classification helpers should return `bool` where that matches the C role,
- top-level behavior should preserve the original semantics for ignored entries and directory printing rather than introducing new error-reporting features.

### Memory Management
No manual memory lifecycle should be reproduced. Replace:
- borrowed C string pointers with `&Path` / `&OsStr`,
- temporary constructed filenames with `PathBuf`,
- stack/global temporary buffers with scoped Rust values.

This removes leak and invalid-pointer risk while matching existing function boundaries as closely as possible.

## Module Mapping

### Source-to-Target Mapping
- **C source file**: `doc/d.c`
- **Rust target module**: `src/doc/d.rs`

If the crate already exposes a `doc` module, integrate this file as:
- `src/doc/mod.rs` -> `pub mod d;`

If the crate is currently flatter, place the migrated file at `src/d.rs` only if required by the existing project structure. Prefer preserving the source path shape through `src/doc/d.rs`.

### Function Mapping
- `isdir` -> `fn is_dir(path: &Path) -> bool` or a crate-private equivalent preserving call usage
- `ignorent` -> `fn ignore_entry(name: &OsStr) -> bool` or equivalent helper with minimal signature changes
- `printdir` -> `fn print_dir(...) -> io::Result<()>` or a visibility/signature variant aligned with existing callers

Function names may be adapted to Rust snake_case, but the mapping should remain one-to-one. Do not split one C function into multiple public APIs unless a small private helper is necessary to preserve clarity or error handling.

## Data Model

The analysis lists two anonymous data structures. Since no named fields are provided, the migration should proceed conservatively.

### Data-structure Mapping
- **C anonymous struct/record #1** -> Rust `struct` with a local descriptive name derived from usage in `doc/d.c`
- **C anonymous struct/record #2** -> Rust `struct` or `enum` with a usage-derived name

### Mapping Rules
- Preserve field ordering conceptually only where needed for behavior; Rust layout compatibility is not required.
- Replace C string fields:
  - `char *` / `char[]` -> `PathBuf`, `OsString`, or `String` depending on actual usage
- Replace integer flags:
  - `int` boolean-like fields -> `bool`
  - counters/sizes -> `usize` where naturally indexed, otherwise fixed-width integer only if behavior depends on width
- Replace nullable pointers:
  - optional owned/reference data -> `Option<T>`

### Anonymous Structure Resolution
Because the source analysis does not expose field lists, the implementation phase should first inspect `doc/d.c` and assign names based strictly on current use. Avoid inventing generalized abstractions; define only the minimum Rust structs/enums needed to support the migrated functions.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Inspect C Signatures
- Create the Rust destination file corresponding to `doc/d.c`.
- Add only the module declarations required to compile within the existing crate layout.
- Inspect the exact C signatures and call sites for `isdir`, `ignorent`, and `printdir`.
- Identify the two anonymous data structures and derive minimal Rust names from their actual roles.
- Decide final Rust function signatures based on existing caller expectations, preferring crate-private functions unless external use requires `pub`.

### Deliverables
- `src/doc/d.rs` created
- module wired into the crate
- placeholder Rust signatures for the three functions
- documented mapping for the two anonymous structures

## Phase 2: Port Classification and Filtering Logic
- Implement `isdir` using `Path` plus `fs::metadata` or `Path::is_dir`, choosing the variant that best matches the C error semantics.
- Implement `ignorent` as a pure helper over entry names, preserving the exact filtering rules from the C code.
- Translate any anonymous structure definitions directly needed by these helpers.
- Add focused unit tests for:
  - directory/non-directory detection
  - ignored vs non-ignored names
  - edge cases such as `.` / `..` or hidden names if present in the C logic

### Deliverables
- working Rust equivalents for `isdir` and `ignorent`
- tests covering helper behavior
- resolved Rust definitions for any helper-only data structures

## Phase 3: Port Directory Printing Flow
- Implement `printdir` using `fs::read_dir` and iteration over entries.
- Recreate the original ordering behavior only if the C code explicitly depends on it; otherwise preserve filesystem iteration behavior as-is.
- Build child paths with `PathBuf` joins rather than manual buffer concatenation.
- Route output through the closest Rust equivalent to the C implementation’s current printing behavior.
- Preserve skip/continue behavior for unreadable or ignored entries according to the original control flow.

### Deliverables
- working Rust `print_dir` equivalent
- integration-style tests using temporary directories created with the standard library
- output behavior aligned with the C function’s formatting and filtering

## Phase 4: Final Alignment and Cleanup
- Compare Rust behavior against the original `doc/d.c` function-by-function.
- Remove any temporary compatibility code not required by callers.
- Normalize naming to idiomatic Rust internally while keeping the one-to-one mapping obvious.
- Ensure all filesystem and output errors are either propagated or intentionally handled in a way that matches the original behavior.
- Run `cargo test` and fix any edge-case mismatches.

### Deliverables
- final migrated module
- passing tests
- concise code comments only where needed to explain preserved C behavior

## Notes and Constraints

- Prefer the Rust standard library exclusively; no third-party crates are required by the provided analysis.
- Do not introduce extra modules, wrappers, or abstractions beyond what is needed to replace `doc/d.c`.
- Keep migration focused on the listed functions and directly supporting structures only.
- Preserve behavior first; apply idiomatic Rust mainly in ownership, path handling, and error propagation.
- Avoid assuming UTF-8 for directory entry names unless the C logic clearly depends on text processing beyond byte comparison.