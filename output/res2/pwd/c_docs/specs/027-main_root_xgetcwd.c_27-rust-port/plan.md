# Implementation Plan

## Summary

Port `xgetcwd.c` into a Rust module that provides the existing `xgetcwd` behavior using Rust’s standard library path facilities as the primary implementation path. The Rust version should preserve the current module boundary and focus only on replacing the C function with an idiomatic, allocation-safe equivalent that returns the current working directory and propagates failures explicitly.

The implementation should rely on `std::env::current_dir()` and Rust-owned path/string buffers instead of manual heap allocation. Any conversion between platform path types and string output should be handled carefully to avoid introducing lossy behavior unless the surrounding Rust codebase already requires UTF-8 strings. Error handling should replace C null/error-result patterns with `Result`-based returns at the Rust API boundary, while keeping the migration aligned with the original call expectations in the `pwd` project.

## Technical Context

- **Language/Version**: Rust stable, edition 2021
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required based on the provided module scope
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the practical performance of the C implementation for normal current-directory retrieval
  - Avoid unnecessary intermediate allocations beyond what is needed to own the returned path
  - Preserve linear cost relative to resulting path length
  - Keep syscall usage limited to the standard library’s current-directory retrieval path

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `xgetcwd.c` | `xgetcwd` | `src/.../main_root_xgetcwd.rs` or existing module file for this cluster | Port as a single-function module or as a function inside the existing main-cluster module tree |
| `xgetcwd.c` | `xgetcwd` | `pub(crate) fn xgetcwd(...) -> Result<..., std::io::Error>` | Final signature should follow the actual needs of the Rust call sites while preserving behavior |

### Recommended Rust Placement

Use the existing Rust crate layout for the `main_cluster` area and place the migrated function in the closest corresponding module file rather than creating extra abstraction layers. If this branch already mirrors C files one-to-one, keep that pattern.

## Data Model

This module has no named C structs to migrate.

### Function-Level Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| heap-allocated current working directory buffer | `PathBuf` or `OsString` | Prefer native path ownership first |
| C string result (`char *`) | `PathBuf`, or `OsString`, or `String` only if required by callers | Avoid forcing UTF-8 unless call sites need it |
| null on failure / errno-based failure | `Result<T, std::io::Error>` | Standard Rust error propagation |
| manual allocation growth | standard library internal allocation | No direct buffer management in port |

### Memory Management

- Replace manual allocation and ownership transfer with Rust-owned values.
- Ensure there is no borrowed reference returned from temporary path buffers.
- If the surrounding code requires textual output, convert as late as possible from `PathBuf`/`OsString` to avoid invalid Unicode assumptions.

## Implementation Phases

### Phase 1: Establish Rust module and function signature

- Create the Rust module corresponding to `xgetcwd.c` within the existing `main_cluster` layout.
- Define the Rust `xgetcwd` function with a minimal signature derived from current usage.
- Prefer `Result<PathBuf, std::io::Error>` as the base implementation signature unless existing migrated callers clearly require `String`.
- Document any caller-driven signature adaptation at the module boundary only; do not add compatibility wrappers unless already needed by the codebase.

### Phase 2: Port core logic

- Implement `xgetcwd` using `std::env::current_dir()`.
- Return the owned path directly in native form where possible.
- If a string must be returned:
  - perform conversion in one place,
  - handle non-UTF-8 paths explicitly,
  - map conversion failures into a suitable error instead of silently altering content.
- Remove all C-style allocation concerns and ensure all error paths are represented through `Result`.

### Phase 3: Integrate with existing callers

- Update direct callers of `xgetcwd` in the Rust port branch to consume the new `Result`-based return value.
- Preserve existing module boundaries and call order from the C layout.
- Keep changes narrowly scoped to replacing the old function contract and adapting immediate uses.
- Avoid introducing shared utility modules or generalized path helpers unless they already exist.

### Phase 4: Testing and validation

- Add unit tests covering:
  - successful retrieval of the current working directory,
  - non-empty returned path,
  - stable ownership semantics of the returned value.
- If the project’s existing test style permits environment-sensitive tests, verify behavior against `std::env::current_dir()`.
- Run `cargo test` and confirm the migrated function integrates without widening module scope or introducing extra dependencies.