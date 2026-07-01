# Implementation Plan

## Summary

Port `pwd.c` into a single Rust executable module that preserves the current command-line program behavior around computing and printing the working directory, with special focus on root/file-name handling represented by `main_root_file_name_03`. The Rust implementation should stay close to the existing C flow and migrate the current functions in place rather than redesigning the program.

The implementation approach is:

- keep the logic concentrated in `src/main.rs`
- translate the pathname-building helpers (`file_name_*`) into a small owned Rust path buffer type
- translate directory traversal helpers (`find_dir_entry`, `robust_getcwd`) into safe Rust functions using the standard library where behavior matches, and narrow Unix-specific OS access only where required
- replace manual allocation/free patterns with ownership and `Drop`-free standard containers such as `PathBuf`, `OsString`, `Vec<u8>`, and `String`
- convert integer/status-code based C error flow into `Result<_, std::io::Error>` internally, with `main` producing the final process exit status

This plan intentionally does not introduce extra abstraction layers or additional modules beyond what is needed to migrate the existing file and functions.

## Technical Context

- **Language/Version**: Rust 1.77+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended by default, since the input does not require behavior beyond standard filesystem/process facilities
- **Testing**:
  - `cargo test`
  - unit tests for path-building helper behavior
  - integration-style tests for executable output in representative directory layouts
- **Performance Goals**:
  - preserve command responsiveness equivalent to the C version for normal directory depths
  - avoid unnecessary string copying during path assembly
  - use iterative path construction and standard filesystem metadata lookups without adding extra allocations beyond what the C logic already implies

## Module Mapping

| C File / Function | Rust Location | Migration Notes |
|---|---|---|
| `pwd.c` | `src/main.rs` | Keep the implementation in one executable source file to mirror the original module scope. |
| `file_name_init` | `src/main.rs::FileNameBuf::new` | Replace manual initialization of a pathname buffer with a Rust-owned type constructor. |
| `file_name_prepend` | `src/main.rs::FileNameBuf::prepend_component` | Preserve prepend-based path assembly; use `OsString`/`PathBuf`-compatible storage to avoid UTF-8 assumptions. |
| `file_name_free` | removed as explicit function; ownership drop | Manual free is not needed; if call-site parity helps migration, a no-op wrapper can exist temporarily and then be removed. |
| `find_dir_entry` | `src/main.rs::find_dir_entry` | Implement as a helper that scans a parent directory and matches the current directory entry via metadata/device/inode comparison as needed. |
| `robust_getcwd` | `src/main.rs::robust_getcwd` | Primary helper returning the computed current directory path as a Rust path/string result. |
| `main` | `src/main.rs::main` | Parse args as currently required by the original program scope and map helper errors to stderr + exit code. |

## Data Model

The analysis only exposes anonymous C data structures, so the Rust plan should map them by role discovered during migration rather than inventing unrelated domain types.

| C Data Shape | Rust Type | Purpose / Notes |
|---|---|---|
| anonymous pathname buffer struct used by `file_name_*` | `struct FileNameBuf { bytes: Vec<u8> }` or `struct FileNameBuf { path: std::ffi::OsString }` | Prefer `OsString` if prepend operations remain manageable; use `Vec<u8>` on Unix if exact byte preservation is needed. Final choice should be driven by the original C buffer semantics in `pwd.c`. |
| anonymous directory entry view | `std::fs::DirEntry` | Used while scanning parent directories. |
| anonymous stat-like record | `std::fs::Metadata` plus `std::os::unix::fs::MetadataExt` | For Unix device/inode comparisons if `find_dir_entry` depends on identity matching. |
| anonymous owned C string buffers | `String`, `OsString`, or `Vec<u8>` | Choose `OsString`/`Vec<u8>` for filesystem names to avoid invalid UTF-8 issues. |
| anonymous return/status integers | `Result<T, std::io::Error>` and `ExitCode`/`process::exit` | Replace sentinel returns with typed error propagation. |
| anonymous temporary directory handles | `std::fs::ReadDir` / `std::fs::File` | Use standard owned handles with automatic cleanup. |

### Memory Management Decisions

- Eliminate explicit allocation/free pairs by storing owned path state in Rust containers.
- Avoid borrowing directory entry names beyond iterator scope; clone into owned path storage when prepending.
- Keep filesystem name handling non-UTF-8-safe by default; do not force conversion to `String` until final output, and only if the original behavior requires textual output conversion. On Unix, write bytes/OsStr directly when possible.

### Error Handling Decisions

- Internal helpers return `io::Result<_>`.
- Conditions that were previously handled by errno/status inspection should map to `io::Error::last_os_error()`-compatible standard errors or explicit `io::ErrorKind` values.
- `main` should remain thin: call `robust_getcwd`, print the resulting path, and convert failure into the same process-level success/failure behavior as the C program.

## Implementation Phases

## Phase 1: Establish the Rust executable skeleton and migrate path buffer helpers

### Goals
- Create the Rust binary crate structure for this branch.
- Port the `file_name_*` helper logic first, since it is a dependency for path reconstruction.

### Tasks
- Create `src/main.rs`.
- Define a minimal `FileNameBuf` type matching the original helper responsibilities.
- Implement:
  - `FileNameBuf::new` for `file_name_init`
  - `FileNameBuf::prepend_component` for `file_name_prepend`
- Remove explicit `file_name_free` behavior by relying on ownership; keep a temporary compatibility wrapper only if it simplifies incremental migration.
- Add focused unit tests for:
  - empty initialization
  - prepending one component
  - prepending multiple components in the expected order
  - root separator handling

### Completion Criteria
- Path-building behavior is covered by tests.
- No manual memory management remains for the translated helper logic.

## Phase 2: Port directory resolution helpers

### Goals
- Translate the filesystem traversal logic used to reconstruct the current working directory.
- Preserve C behavior closely, especially for parent traversal and root detection.

### Tasks
- Implement `find_dir_entry` in `src/main.rs`.
  - Use `std::fs::read_dir` iteration.
  - Use Unix metadata extensions if the original matching logic depends on inode/device identity.
- Implement `robust_getcwd`.
  - First inspect whether direct standard-library current-directory retrieval is sufficient for the migrated behavior.
  - If the original module reconstructs the path manually for robustness/root-name handling, preserve that algorithm rather than replacing it with a simpler call.
  - Build the resulting path through `FileNameBuf`.
- Keep helper signatures narrow and local to the executable module.

### Completion Criteria
- `robust_getcwd` returns the expected current directory path for ordinary directories and root.
- Helper code compiles without unsafe code unless direct Unix OS interop is proven necessary by the original C logic.

## Phase 3: Port `main` and align process behavior

### Goals
- Finish the executable flow and preserve user-visible behavior.
- Ensure errors are surfaced correctly.

### Tasks
- Implement `main` as the translation of the C entry point.
- Map helper results to stdout/stderr output and exit status.
- Keep argument handling limited to what exists in `pwd.c`; do not add new CLI features.
- Ensure printed path formatting matches the original program expectations, including newline behavior.

### Completion Criteria
- The binary runs end-to-end and prints the working directory.
- Failure paths return nonzero exit behavior consistent with the original program.

## Phase 4: Validation and cleanup

### Goals
- Verify parity and remove migration-only scaffolding.
- Lock down behavior with tests.

### Tasks
- Add integration tests covering:
  - execution from a normal nested directory
  - execution from filesystem root
  - non-UTF-8-safe path handling on Unix where practical
- Compare remaining helper names and comments against the C source to ensure all listed functions have been migrated or intentionally eliminated due to Rust ownership.
- Remove any temporary compatibility wrapper for `file_name_free` if still present.
- Run `cargo test` and `cargo fmt`.

### Completion Criteria
- All migrated functions are represented in Rust.
- Tests pass and the file layout remains restrained to standard Rust executable structure.