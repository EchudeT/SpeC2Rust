# Implementation Plan

## Summary

Port the `main_root_stat_04` C module cluster into Rust by migrating the logic currently split across `pwd.c` and `root-dev-ino.c` into a small, direct Rust implementation centered on two functions:

- `logical_getcwd`
- `get_root_dev_ino`

The Rust approach should preserve the existing low-level filesystem behavior rather than redesign it. The implementation should rely primarily on the Rust standard library for path handling and I/O, with narrowly scoped Unix-specific system metadata access where device/inode values are required. Error handling should be converted from C-style status/result conventions into `Result`-based Rust APIs, while keeping control flow close to the original function boundaries and call order.

The plan should avoid introducing new abstraction layers beyond what is needed to map the existing files and functions into Rust source files. The migration should focus on equivalent filesystem queries, root device/inode detection, and current-directory resolution behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `std::os::unix::fs::MetadataExt` for Unix device/inode access
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior and runtime characteristics comparable to the C implementation for filesystem metadata reads and current-directory resolution.
  - Avoid unnecessary path cloning or repeated metadata lookups beyond what is required by the original logic.
  - Keep allocation bounded to path construction and string/path ownership needed by safe Rust APIs.

## Module Mapping

### C to Rust File Mapping

- `pwd.c`
  - Rust target: `src/main.rs` if this module remains the program entry implementation
  - Or `src/pwd.rs` if the project already separates executable entry from migrated logic
  - Contains migrated implementation of:
    - `logical_getcwd`

- `root-dev-ino.c`
  - Rust target: `src/root_dev_ino.rs`
    - `get_root_dev_ino`

### Rust Module Layout

A restrained module layout is recommended:

```text
src/
  main.rs
  root_dev_ino.rs
```

If existing project structure already uses library-style separation, use:

```text
src/
  lib.rs
  pwd.rs
  root_dev_ino.rs
```

In either case, do not introduce extra helper modules unless required by existing project structure.

### Function Mapping

- `logical_getcwd`
  - C role: current working directory resolution with logical path considerations
  - Rust role: direct function migration with `Result<PathBuf, std::io::Error>` or equivalent internal return type that matches surrounding project conventions

- `get_root_dev_ino`
  - C role: retrieve root filesystem device/inode identity
  - Rust role: use `std::fs::metadata("/")` plus Unix metadata extensions to obtain `(dev, ino)`

## Data Model

The input analysis only identifies multiple anonymous C data structures and does not provide named struct definitions. The implementation plan should therefore avoid speculative Rust type design and only introduce explicit Rust structs if the migrated code requires them during translation.

### Data-Structure Mapping Strategy

- **C anonymous structs/unions used only as temporary groupings**
  - Rust mapping: prefer local variables, tuples, or small private structs only where necessary to preserve readability and ownership boundaries

- **Device/Inode pair data**
  - Rust mapping: private struct or tuple
  - Recommended form:
    - `(u64, u64)` for minimal migration footprint, or
    - `struct RootDevIno { dev: u64, ino: u64 }` if the values are passed together multiple times

- **CString / mutable character buffer patterns**
  - Rust mapping:
    - `PathBuf` for filesystem paths
    - `OsString` where non-UTF-8 path preservation is needed
    - `String` only if UTF-8 text is explicitly required by surrounding code

- **Manual error/status fields**
  - Rust mapping: `Result<T, std::io::Error>`

### Proposed Rust Types

Because the C data structures are unnamed and unspecified, keep the Rust data model minimal:

```rust
struct RootDevIno {
    dev: u64,
    ino: u64,
}
```

If this struct is only used once, replace it with a tuple:

```rust
type DevIno = (u64, u64);
```

### Memory Management Notes

- Replace stack/heap-managed C path buffers with owned `PathBuf`/`OsString`.
- Eliminate manual lifetime and buffer-capacity management.
- Keep borrowed path references as `&Path` where possible.
- Avoid converting paths to `String` unless required, to preserve Unix path fidelity.

### Error Handling Notes

- Replace integer return codes and output parameters with `Result`.
- Propagate I/O failures with `?`.
- Where the original code distinguishes specific filesystem cases, map those checks explicitly using `std::io::ErrorKind` only if needed by existing control flow.

## Implementation Phases

## Phase 1: Establish module skeleton and root metadata migration

### Goals
- Create the Rust file mapping for the existing C sources.
- Port `get_root_dev_ino` first because it is a small, isolated filesystem metadata function.

### Tasks
- Create `src/root_dev_ino.rs`.
- Implement `get_root_dev_ino` using:
  - `std::fs::metadata("/")`
  - `std::os::unix::fs::MetadataExt::{dev, ino}`
- Choose the narrowest return shape compatible with the rest of the module:
  - `Result<RootDevIno, io::Error>` or `Result<(u64, u64), io::Error>`
- Add unit tests that verify:
  - function returns successfully on Unix
  - returned device/inode values are nonzero where that assumption is valid
  - values match direct metadata reads for `/`

### Deliverables
- Compiling Rust implementation of root device/inode retrieval
- Basic tests for metadata extraction

## Phase 2: Port `logical_getcwd` with direct control-flow preservation

### Goals
- Migrate the current-directory resolution logic from `pwd.c` without redesigning behavior.
- Preserve logical-path handling semantics as closely as possible within safe Rust.

### Tasks
- Create or update the Rust file that owns migrated `pwd.c` logic.
- Translate `logical_getcwd` into Rust with:
  - `PathBuf` / `OsString` for path state
  - explicit metadata checks where the C code compares filesystem identities
  - `Result<PathBuf, io::Error>`-style error propagation
- Use standard-library current-directory APIs where they match the original behavior:
  - `std::env::current_dir()`
- Where the original implementation validates path correctness against device/inode state, use:
  - `std::fs::metadata`
  - `MetadataExt`
  - comparison against `get_root_dev_ino` results if required by original logic
- Keep helper logic private and file-local rather than introducing general utility modules

### Deliverables
- Rust implementation of `logical_getcwd`
- Compilation with the root metadata module integrated

## Phase 3: Integrate file boundaries and normalize error/ownership behavior

### Goals
- Ensure the translated functions fit the project’s executable/library structure.
- Remove remaining C-style patterns from the migrated code.

### Tasks
- Wire `root_dev_ino` module imports into the file containing `logical_getcwd`.
- Convert any remaining output-parameter style logic into returned values.
- Replace mutable raw-buffer workflows with owned/buffered Rust path objects.
- Verify all filesystem accesses borrow paths where possible and only allocate on state transitions.
- Keep visibility minimal:
  - `pub(crate)` only where cross-file access is required
  - private otherwise

### Deliverables
- Clean module integration
- Idiomatic but restrained ownership/error model
- No manual memory-management remnants

## Phase 4: Complete test coverage for migrated behavior

### Goals
- Validate the Rust port against the expected filesystem behavior of the original functions.
- Focus tests on migrated functionality only.

### Tasks
- Add unit tests for `logical_getcwd` covering:
  - successful current-directory retrieval
  - root-directory behavior
  - path metadata consistency checks if present in the migrated logic
- Add tests that compare:
  - returned path from `logical_getcwd`
  - direct `std::env::current_dir()` behavior where equivalence is expected
- Add integration-level checks only if the project already uses executable tests; otherwise keep tests close to the migrated modules
- Confirm all tests pass with `cargo test`

### Deliverables
- Stable test suite for both migrated functions
- Verified module behavior on the Rust branch

## Notes and Constraints

- Restrict implementation scope to the existing files and listed functions.
- Do not add portability layers beyond Unix-specific support implied by device/inode access.
- Do not introduce FFI bindings to preserve the C implementation; this is a source migration.
- Do not add extra architectural layers, traits, or generic filesystem wrappers absent from the C module structure.
- Preserve function responsibilities and migration order according to the original file split.