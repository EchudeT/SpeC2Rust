# Implementation Plan

## Summary

Port the `pwd.c` main-cluster logic into a single Rust binary entry module that preserves the existing behavior around current-directory resolution, pathname assembly, directory entry lookup, and process exit handling. The implementation should migrate the C functions `file_name_free`, `file_name_init`, `file_name_prepend`, `find_dir_entry`, `robust_getcwd`, and `main` into Rust with minimal structural expansion.

The Rust approach should rely primarily on `std::env`, `std::path`, `std::ffi`, `std::fs`, and `std::os::unix` facilities, while keeping the pathname-construction logic explicit rather than replacing it wholesale with unrelated abstractions. Memory ownership previously handled manually in C should become owned Rust values (`PathBuf`, `OsString`, `Vec<u8>`, and scoped iterators), and fallible operations should return `Result` and be propagated to `main` for final exit-status handling.

## Technical Context

- **Language/Version**: Rust 1.82 or current stable compatible with the repository toolchain
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by the analyzed module input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve linear path-building behavior comparable to the C implementation
  - Avoid unnecessary UTF-8 conversions; prefer platform-native path/filename types
  - Keep heap reallocations bounded by reusing owned path buffers where practical
  - Directory traversal and entry scanning should remain proportional to the number of path components and directory entries, without adding extra indexing layers

## Module Mapping

| C File / Function | Rust Target | Notes |
|---|---|---|
| `pwd.c` | `src/main.rs` | Keep the port in the binary entry file unless an existing project layout already requires a different binary file location |
| `file_name_init` | private helper in `src/main.rs` | Replace manual buffer initialization with owned `PathBuf` or `OsString` initialization |
| `file_name_prepend` | private helper in `src/main.rs` | Implement prepend-style path assembly using component accumulation, avoiding unsafe pointer arithmetic |
| `file_name_free` | removed as explicit function; ownership-based drop | If retaining a helper name helps migration clarity, implement as no-op wrapper only temporarily during port |
| `find_dir_entry` | private helper in `src/main.rs` | Recreate directory scan logic with `std::fs::read_dir` and Unix metadata comparison as needed |
| `robust_getcwd` | private helper in `src/main.rs` | Implement current-directory retrieval with `std::env::current_dir`; preserve fallback/error path structure required by surrounding logic |
| `main` | `fn main()` plus optional `run() -> Result<(), _>` | Keep CLI handling and exit behavior localized; use `run()` only to simplify error propagation |

## Data Model

The source analysis lists only anonymous C data structures, so the Rust plan should infer minimal replacements from function behavior rather than inventing extra models.

| C Data Shape | Rust Representation | Migration Notes |
|---|---|---|
| Manual filename buffer used by `file_name_init` / `file_name_prepend` / `file_name_free` | `std::path::PathBuf` as primary representation | Best fit for mutable path assembly with ownership-managed memory |
| Raw filename byte storage | `std::ffi::OsString` or `Vec<u8>` only if bytewise manipulation is required | Prefer `OsString`/`PathBuf`; use Unix byte access only when matching directory entry names without UTF-8 assumptions |
| Directory entry record (`struct dirent`-like usage) | `std::fs::DirEntry` | Obtain names through `file_name()` and metadata through `metadata()` / `symlink_metadata()` as needed |
| File identity fields used for parent/child matching (`dev`, `ino`, similar) | Small private Rust struct such as `FileId { dev: u64, ino: u64 }` | Only introduce if `find_dir_entry` needs stable metadata comparison using `std::os::unix::fs::MetadataExt` |
| Process/exit status temporaries | `Result<(), String>` or `Result<(), std::io::Error>` in helpers | Convert to message-bearing error only near `main` |
| Pointer/null-based optional state | `Option<T>` | Replace all null checks with explicit optional ownership |
| Integer status/error flags | `bool`, `i32`, or `Result` depending on use | Prefer `Result` for fallible filesystem operations |
| Heap allocation size/capacity fields | implicit in `PathBuf` / `Vec<u8>` | Do not manually track capacity unless a direct prepend emulation requires a temporary byte buffer |

## Implementation Phases

### Phase 1: Establish binary port skeleton and path-buffer migration

- Create or update `src/main.rs` as the Rust home for the `pwd.c` port.
- Port `main` into Rust structure first, keeping control flow close to the C version.
- Replace the C filename buffer lifecycle:
  - `file_name_init` -> initialize an empty or root-based `PathBuf`
  - `file_name_prepend` -> helper that prepends one path component into the accumulated path
  - `file_name_free` -> remove explicit deallocation in favor of ownership drop
- Decide early whether prepend behavior is best modeled by:
  - collecting components in reverse and constructing one `PathBuf` once, or
  - rebuilding a `PathBuf` on each prepend
- Keep the implementation minimal and local to this module; do not extract generic path utilities.

### Phase 2: Port directory-resolution helpers

- Port `robust_getcwd` using `std::env::current_dir`.
- Preserve error propagation rather than panicking.
- Port `find_dir_entry` using `std::fs::read_dir`.
- If the original logic matches entries by inode/device rather than by name alone, introduce a minimal private `FileId` helper based on `std::os::unix::fs::MetadataExt`.
- Ensure filename handling remains non-UTF-8-assuming by using `OsStr`/`OsString` and Unix extensions only when required.
- Validate root-directory handling explicitly, since the module name suggests root/file-name edge behavior is important.

### Phase 3: Integrate behavior and finalize CLI/exit semantics

- Connect `main`, `robust_getcwd`, and directory-entry search into the final execution path.
- Match the original module’s exit-status behavior and stderr reporting pattern as closely as practical in idiomatic Rust.
- Audit all former manual-memory and null/error-flag paths and convert them into `Result`/`Option` flow.
- Remove any transitional compatibility wrappers that are no longer needed after the direct port is stable.

### Phase 4: Add focused tests for migrated behavior

- Add unit tests for path-prepend behavior, especially:
  - empty initial path
  - single component
  - multiple components
  - root-preserving assembly
- Add tests around `robust_getcwd` success behavior where practical.
- Add integration-style tests for `main` behavior using temporary directories created through the standard library, focusing on:
  - normal current-directory output
  - root directory output
  - directories with non-UTF-8-compatible names only if the repository’s Unix-only scope already permits such tests
- Run and stabilize with `cargo test`, adjusting only module-local implementation details needed to preserve the original behavior.