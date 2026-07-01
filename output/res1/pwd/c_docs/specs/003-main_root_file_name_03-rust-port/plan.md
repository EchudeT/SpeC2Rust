# Implementation Plan

## Summary

Port the `pwd.c` main-cluster logic into a single Rust binary entry module that preserves the existing control flow for constructing and printing the current working directory, including the fallback path-building logic implied by `robust_getcwd`, directory entry lookup, and filename buffer management.

The Rust implementation should stay close to the current C module structure:
- migrate `main` into the Rust binary entry point,
- convert filename buffer helpers into a small internal Rust type with explicit ownership,
- implement directory traversal and entry matching with standard-library filesystem APIs where possible,
- isolate OS-specific calls behind narrow helper functions only where the Rust standard library does not expose equivalent behavior.

The plan should favor a direct migration of existing functions and file-local behavior over redesign. Memory ownership should move from manual allocation/free patterns to RAII with `PathBuf`, `OsString`, `Vec<u8>`, and scoped values. Error handling should replace integer/status-code propagation with `Result`, while preserving the original command failure behavior in `main`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s practical runtime for normal `pwd` execution.
  - Avoid unnecessary path string copying during prepend/build operations.
  - Keep filesystem traversal bounded to the same directory-walk behavior as the source module.
  - Maintain low allocation count by reusing owned path buffers where practical.

## Module Mapping

### C to Rust File Mapping

- `pwd.c` -> `src/main.rs`

### Function Mapping

- `file_name_free` -> removed as an explicit function; ownership handled by Rust drop semantics, with any needed reset logic kept as a method on a local buffer type
- `file_name_init` -> `FileNameBuf::new()` or equivalent constructor in `src/main.rs`
- `file_name_prepend` -> `FileNameBuf::prepend(...)`
- `find_dir_entry` -> `find_dir_entry(...)` helper in `src/main.rs`
- `robust_getcwd` -> `robust_getcwd(...)` helper in `src/main.rs`
- `main` -> `fn main()`

### Rust Module Shape

Use a restrained single-file binary layout unless migration pressure clearly requires a private helper module:

- `src/main.rs`
  - local filename buffer type
  - local directory-entry search helper
  - local cwd recovery helper
  - argument/environment handling needed by `main`
  - `fn main()`

This keeps the migration aligned to the original single C translation unit.

## Data Model

The source analysis reports only anonymous C data structures. For planning purposes, map them only to the structures required by the listed functions rather than inventing additional abstractions.

### C Anonymous Buffer/State Structures -> Rust Local Structs

1. **Filename accumulation buffer**
   - **C role**: dynamically allocated buffer managed by `file_name_init`, `file_name_prepend`, `file_name_free`
   - **Rust mapping**:
     `struct FileNameBuf { inner: PathBuf }`
     or, if byte-precise prepending is required by the source behavior,
     `struct FileNameBuf { inner: OsString }`
   - **Notes**:
     - Prefer `PathBuf` if prepend behavior can be expressed safely with path components.
     - If exact Unix byte preservation is necessary during reconstruction, use Unix-specific `OsStringExt`/`OsStrExt` internally under `cfg(unix)`.

2. **Directory entry view/state**
   - **C role**: iteration/search state used by `find_dir_entry`
     - use `std::fs::ReadDir`
     - compare entries using `std::fs::DirEntry`, metadata, and inode/device identity where needed
     - no persistent struct required unless needed to mirror function-local state.

3. **Filesystem identity records**
   - **C role**: likely `stat`-derived device/inode comparison inputs
     `struct FileIdentity { dev: u64, ino: u64 }`
     - Unix-only extraction via `std::os::unix::fs::MetadataExt`
     - keep local/private to helper logic.

4. **Main execution status/config temporaries**
   - **C role**: anonymous local aggregates in `main`
     - plain local variables
     - small private enum only if return-path clarity materially improves migration
     - do not create broader configuration layers unless the original `main` requires them.

## Implementation Phases

### Phase 1: Establish direct binary skeleton and buffer migration

- Create Rust binary target at `src/main.rs`.
- Port `main` structure first as a compilable scaffold with placeholder helper calls.
- Replace manual filename allocation/free flow with a local `FileNameBuf` type:
  - constructor replacing `file_name_init`
  - prepend method replacing `file_name_prepend`
  - implicit drop replacing `file_name_free`
- Choose the narrowest path representation that preserves source behavior:
  - default to `PathBuf`
  - switch to Unix `OsString` byte-oriented handling only if required by prepend semantics
- Convert C-style error returns into `Result<_, io::Error>` or `Result<_, String>` internally, with `main` mapping failures to process exit behavior.

### Phase 2: Port directory identity and search helpers

- Implement `find_dir_entry` using `std::fs::read_dir`.
- Reconstruct the C lookup behavior by comparing candidate entries against target metadata identity:
  - use `symlink_metadata`/`metadata` consistently according to source intent
  - compare device and inode on Unix via `MetadataExt`
- Keep helper signatures close to original responsibilities instead of introducing generalized traversal APIs.
- Ensure directory iteration does not retain unnecessary allocations beyond the selected matching entry name.

### Phase 3: Port robust current-directory reconstruction

- Implement `robust_getcwd` as the Rust equivalent of the C fallback logic.
- Prefer `std::env::current_dir()` for the fast path if it matches the source behavior.
- Add fallback reconstruction only for the logic already present in the C module:
  - walk upward through parent directories,
  - find current directory’s entry in parent,
  - prepend discovered component names into `FileNameBuf`,
  - stop at filesystem root based on identity comparison.
- Preserve behavior for edge cases implied by the source:
  - inaccessible path text but traversable directory hierarchy,
  - root detection,
  - path separator handling during prepend.

### Phase 4: Complete CLI behavior and tests

- Finalize `main` argument handling and output/error paths to match the source module’s observable behavior.
- Add focused unit tests for:
  - filename prepend behavior
  - root handling in reconstructed paths
  - directory entry matching by identity
- Add integration-style tests using temporary directories for successful cwd reconstruction scenarios that `cargo test` can run reliably.
- Verify memory and error behavior:
  - no manual frees remain,
  - no lossy path conversions on Unix,
  - all filesystem failures propagate deterministically to `main`.

## Migration Notes

- Keep all migrated logic in `src/main.rs` unless a compiler-driven need for a private helper submodule emerges.
- Avoid replacing the original algorithm with a higher-level crate or a broader abstraction layer.
- Treat anonymous C structures as function-local state unless they clearly map to the filename buffer or filesystem identity concepts.
- On Unix, prefer standard-library extension traits over external crates for inode/device access.
- Be conservative with string conversion: filesystem names should remain in `OsStr`/`OsString` form as long as possible.

## Definition of Done

- `pwd.c` behavior represented in Rust by `src/main.rs`
- listed C functions fully migrated or intentionally eliminated through ownership-based Rust replacement
- `cargo test` passes
- normal cwd printing and fallback reconstruction compile and run on the target branch
- no added modules or facilities beyond those needed to migrate the existing file and functions