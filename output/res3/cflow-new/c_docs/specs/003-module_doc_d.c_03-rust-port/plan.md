# Implementation Plan

## Summary

Port `doc/d.c` into a Rust module that preserves the existing directory-oriented behavior of `isdir`, `ignorent`, and `printdir` without adding new capabilities. The Rust implementation should rely primarily on `std::fs`, `std::path`, and standard iterator-based directory traversal to replace C-level filesystem and string handling.

The technical approach is to migrate the existing functions into a single Rust source module with near one-to-one responsibility mapping:

- `isdir` becomes a small path inspection helper using `std::fs::metadata` or `std::path::Path`.
- `ignorent` becomes a name-filtering helper operating on Rust string/path types.
- `printdir` becomes the main directory-processing function, using `std::fs::read_dir` and invoking the helper functions in the same control flow order as the C code.

Memory management shifts from manual C allocation and pointer handling to owned Rust types such as `PathBuf`, `OsString`, and `String` where UTF-8 conversion is valid. Error handling should replace implicit C failure paths with explicit `Result` returns where needed, while preserving current observable behavior as closely as possible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::fs`, `std::path`, `std::ffi`, `std::io`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for directory scanning workloads.
  - Avoid unnecessary path/string cloning inside directory iteration.
  - Use streaming iteration from `read_dir` rather than collecting entries eagerly.
  - Keep helper functions lightweight and allocation-minimal.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `doc/d.c` | `src/module_doc_d_c_03.rs` | Direct port of this module’s functions into one Rust source file. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `isdir` | `fn is_dir(path: &Path) -> bool` | Replace C filesystem checks with `metadata().is_dir()` or `Path::is_dir()`. |
| `ignorent` | `fn ignore_entry(name: &OsStr) -> bool` | Preserve filtering logic while avoiding forced UTF-8 conversion unless required by original semantics. |
| `printdir` | `fn print_dir(path: &Path, ...) -> io::Result<()>` | Main migrated function; signature should be adapted only as required by surrounding Rust call sites. |

## Data Model

The analysis lists only anonymous C data structures. Since no named struct contract is provided, the Rust port should avoid inventing new public data models unless the original file contains local aggregate state that must be preserved.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | Local Rust struct if needed | Use a private `struct` only if the C file contains a local anonymous aggregate whose fields are needed across helper calls. |
| anonymous | Tuple/local variables if possible | Prefer direct local bindings over introducing additional types. |

### Type Mapping Guidance

| C Pattern | Rust Type |
|---|---|
| filesystem path as `char *` | `&Path` / `PathBuf` |
| directory entry name | `&OsStr` / `OsString` |
| integer boolean return | `bool` |
| status/error via return code | `io::Result<T>` where needed |

### Memory and Error Handling

- Replace raw pointer-based path handling with borrowed `&Path` inputs and owned `PathBuf` only when path composition is necessary.
- Preserve platform-correct filename handling with `OsStr`/`OsString` rather than assuming UTF-8.
- Convert C-style error checks into explicit `Result` propagation inside `print_dir`.
- If the original behavior ignores specific filesystem errors during iteration, mirror that behavior deliberately rather than broadening failure handling.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Helper Ports

- Create `src/module_doc_d_c_03.rs`.
- Port `isdir` first as `is_dir`, using standard filesystem metadata/path inspection.
- Port `ignorent` next as `ignore_entry`, keeping its filtering conditions as close to the original function as possible.
- Resolve C string assumptions into Rust path/name types with the least behavioral change.
- Keep all items private unless existing integration requires visibility.

### Deliverables

- Compiling Rust module with helper function stubs fully implemented.
- Initial unit tests for:
  - directory detection on existing temp directories/files
  - entry-ignore decisions for representative names from the C logic

## Phase 2: Port `printdir` Control Flow

- Migrate `printdir` into `print_dir` using `std::fs::read_dir`.
- Preserve iteration order only if the C code relied on filesystem order; otherwise do not introduce sorting.
- Recreate path joining using `PathBuf`/`Path::join`.
- Invoke `ignore_entry` and `is_dir` in the same decision points as the C implementation.
- Translate output behavior carefully, keeping formatting and side effects aligned with the original module’s role.

### Deliverables

- Functional Rust implementation of the main directory-processing routine.
- Compilation with all module logic connected.
- Tests covering:
  - traversal of a temporary directory tree
  - ignored entries not being processed
  - expected handling of file vs directory entries

## Phase 3: Error Semantics and Signature Alignment

- Review original C failure paths and map them to Rust return behavior.
- Adjust `print_dir` signature to fit the actual surrounding Rust crate interfaces while preserving module scope and behavior.
- Ensure helper functions do not panic on non-UTF-8 filenames or missing metadata.
- Keep the implementation standard-library-only unless surrounding crate constraints require otherwise.

### Deliverables

- Stable function signatures aligned to crate usage.
- Error propagation/ignoring behavior matched to the original module’s practical semantics.
- Unit tests for unreadable/nonexistent path cases as supported by the target platform.

## Phase 4: Integration Cleanup and Regression Tests

- Wire the module into the branch’s Rust crate structure using standard `mod` declarations.
- Remove any leftover C-style control-flow artifacts that are no longer needed after the safe Rust migration, without changing behavior.
- Add regression tests focused on the migrated functions only, avoiding expansion into unrelated functionality.
- Confirm `cargo test` passes on the module port.

### Deliverables

- Integrated Rust module replacing `doc/d.c` responsibilities.
- Final tests for helper logic and end-to-end directory handling.
- Ready-for-review migration limited to the existing file and functions.