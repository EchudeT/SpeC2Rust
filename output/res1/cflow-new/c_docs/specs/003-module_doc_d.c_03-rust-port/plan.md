# Implementation Plan: module_doc_d.c_03

## Summary

This module migration covers `doc/d.c` and focuses on porting the directory-related helper logic behind `isdir`, `ignorent`, and `printdir` into a single Rust module with equivalent control flow and observable behavior.

The Rust implementation should stay close to the existing C file structure and function boundaries rather than introducing new abstractions. The technical approach is to map C filesystem and directory handling onto `std::fs`, `std::path`, and standard iterator-based directory traversal, while preserving the original filtering and printing responsibilities of each function. Error handling should replace implicit C failure paths and null/state checks with explicit `Result`, `Option`, and boolean return values where appropriate, but without changing the module’s role or widening scope.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**: Rust standard library only (`std::fs`, `std::path`, `std::io`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the original module’s lightweight filesystem interaction profile.
  - Avoid unnecessary path/string cloning during directory iteration.
  - Use streaming directory traversal via `std::fs::read_dir` rather than collecting entries unless required by existing behavior.
  - Keep syscall count and allocation behavior close to the C implementation’s practical usage.

## Module Mapping

### C to Rust File Mapping

- `doc/d.c` → `src/module_doc_d_c_03.rs`

### Function Mapping

- `isdir` → `pub(crate) fn isdir(...) -> bool` or `io::Result<bool>`
  - Use `std::fs::metadata` or `std::path::Path::is_dir`.
  - Prefer a signature that reflects existing caller expectations; if the C logic treats filesystem errors as “not a directory,” convert errors to `false` at the boundary.
- `ignorent` → `pub(crate) fn ignorent(...) -> bool`
  - Translate entry-ignore checks directly, preserving current name-based filtering logic.
  - Accept `&str`, `&OsStr`, or `&Path` according to how the original function consumes directory entry names.
- `printdir` → `pub(crate) fn printdir(...) -> io::Result<()>`
  - Use `std::fs::read_dir` for enumeration.
  - Preserve iteration order only if the C code depends on OS directory order; otherwise do not add sorting.
  - Write output through standard formatting macros or a passed writer if the original code prints to a configurable stream.

### Responsibility Boundaries

- Keep all migrated logic for this file in one Rust module.
- Do not split filtering, path handling, or printing into extra helper modules unless directly required by borrow-checking or testability.
- Keep function names recognizable relative to the source C implementation for migration traceability.

## Data Model

The analysis reports two anonymous C data structures. Since they are unnamed, the Rust mapping should be driven by actual field usage in `doc/d.c` during implementation.

### Data Structure Mapping

- `anonymous` → `struct Anonymous1`
- `anonymous` → `struct Anonymous2`

### Mapping Rules

- If an anonymous C struct is only used as a temporary grouping of local fields:
  - Prefer eliminating it in Rust and using local variables or tuples.
- If it is passed between functions or stored:
  - Introduce a private Rust `struct` with fields mapped exactly to the used C fields.
- Field mapping guidance:
  - `char *` / `const char *` → `String`, `&str`, or `PathBuf` / `&Path` depending on ownership and filesystem semantics.
  - integer flags → `bool` or fixed-width integers only if bit/ABI semantics matter internally.
  - raw optional pointers → `Option<T>` / `Option<&T>` / `Option<Box<T>>` as dictated by ownership.
- For directory entry names:
  - Prefer `OsString`/`&OsStr` if names are handled as filesystem-native values.
  - Convert to UTF-8 strings only at formatting boundaries if the original logic prints names.

### Memory Management and Error Handling

- Replace manual C lifetime management with Rust ownership and borrowing.
- Avoid storing borrowed directory entry data beyond the iteration step unless copied intentionally.
- Convert filesystem failure points into `io::Result` where propagation is natural.
- Only collapse errors into boolean results for functions whose original role is predicate-style and whose callers likely rely on simple truthiness.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Function Signatures

- Create `src/module_doc_d_c_03.rs`.
- Add Rust equivalents for:
  - `isdir`
  - `ignorent`
  - `printdir`
- Derive initial function signatures from actual call patterns in the surrounding ported codebase.
- Add placeholder private structs for the two anonymous C structures only if field usage requires them.
- Keep naming and call layout close to `doc/d.c` for straightforward review against the C source.

## Phase 2: Port Core Filesystem and Filtering Logic

- Implement `isdir` using `std::fs::metadata` / `Path::is_dir` with behavior aligned to the C predicate semantics.
- Implement `ignorent` by directly translating the existing ignore rules and special-case name handling.
- Implement `printdir` using `std::fs::read_dir`, applying `ignorent` during iteration and reproducing the original printing path composition and formatting behavior.
- Resolve C string/path handling into Rust `Path`/`OsStr` types, minimizing lossy conversion.

## Phase 3: Integrate Error Paths and Output Behavior

- Audit all C failure branches in `doc/d.c` and map them to:
  - `io::Result<()>` propagation for directory traversal/printing routines.
  - predicate-compatible fallback behavior for `isdir` if required.
- Ensure printing destinations and formatting remain compatible with surrounding migrated code.
- Remove any unnecessary temporary allocations introduced during the first pass.

## Phase 4: Validate with Focused Tests

- Add unit tests for:
  - directory detection behavior in `isdir`
  - ignore-rule edge cases in `ignorent`
  - basic directory enumeration and filtered output behavior in `printdir`
- Use temporary directories/files created within tests via the standard library.
- Verify behavior for:
  - existing directory
  - regular file
  - ignored names
  - empty directory
  - unreadable/nonexistent path handling as applicable to the chosen signatures