# Implementation Plan

## Summary

Port the `main_root_stat_04` C module into Rust by migrating the logic currently split across `pwd.c` and `root-dev-ino.c` into a small, Rust-standard module set that preserves the existing behavior and call structure.

The implementation should focus on two existing responsibilities:

- `logical_getcwd`: current-working-directory resolution logic used by the `pwd` main path.
- `get_root_dev_ino`: root directory device/inode lookup used to support path resolution and root detection.

The Rust approach should prefer the standard library for path handling, environment access, and error propagation, while using narrow Unix-specific metadata access where device/inode values are required. The migration should keep the logic close to the original file/function boundaries rather than introducing new abstractions. Memory ownership should move from manual buffer management to owned Rust types such as `PathBuf`, `OsString`, and `Metadata`-derived values, with `Result` used for error propagation instead of sentinel return codes.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `std::env` for current-directory and environment access
  - `std::path` for path representation
  - `std::fs` for metadata queries
  - `std::os::unix::fs::MetadataExt` for `dev`/`ino` access on Unix
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for cwd/root metadata operations.
  - Avoid unnecessary path/string copies beyond what is required for safe ownership.
  - Keep filesystem metadata calls limited to those already implied by the C logic.
  - No added background work, caching layer, or extra traversal passes.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `pwd.c` | `logical_getcwd` | `src/main_root_stat_04.rs::logical_getcwd` or `src/bin/pwd.rs` private helper | Keep as a direct helper for the main `pwd` execution path. |
| `root-dev-ino.c` | `get_root_dev_ino` | `src/main_root_stat_04.rs::get_root_dev_ino` | Implement as a Unix-specific helper returning root device/inode data. |

Recommended restrained layout:

| Rust File | Purpose |
|---|---|
| `src/main_root_stat_04.rs` | Direct port location for the two migrated helpers. |
| `src/bin/pwd.rs` | Existing or target binary entry point that invokes the migrated helpers. |

If the project already centralizes `pwd` logic elsewhere, place the two migrated functions in the nearest existing module rather than creating additional layers.

## Data Model

The source analysis lists only anonymous C data structures and does not provide named struct definitions. The plan should therefore map only the data shapes that are clearly implied by the migrated functions.

### Data-structure Mapping

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| anonymous filesystem identity pair for root | `struct DevIno { dev: u64, ino: u64 }` | Minimal explicit replacement for device/inode outputs from `get_root_dev_ino`. |
| anonymous path buffer/state | `PathBuf` / `OsString` | Replaces mutable C character buffers and manual allocation. |
| anonymous stat result usage | `std::fs::Metadata` plus extracted fields | Do not preserve a C-style stat struct unless needed by call sites. |
| anonymous integer status/error returns | `std::io::Result<T>` | Replaces sentinel return values and `errno`-style propagation. |
| anonymous optional output pointers | function return values / `Option<T>` where needed | Use explicit ownership instead of output-parameter mutation. |

### Proposed Rust Types

```rust
pub struct DevIno {
    pub dev: u64,
    pub ino: u64,
}
```

If the surrounding Rust code already has an equivalent filesystem identity type, reuse it instead of introducing a duplicate.

### Memory Management and Error Handling

- Replace C-owned heap or stack buffers with owned Rust path/string values.
- Avoid borrowing environment-derived or filesystem-derived data beyond the immediate scope unless ownership is transferred.
- Convert all fallible filesystem operations into `io::Result`.
- Keep error mapping simple and local; do not introduce a custom error hierarchy unless the existing Rust project already requires one.
- Preserve Unix-specific behavior behind `cfg(unix)` where device/inode access is needed.

## Implementation Phases

### Phase 1: Create the Rust module skeleton and migrate root device/inode lookup

- Add `src/main_root_stat_04.rs` or the closest existing module file used by the `pwd` binary.
- Introduce the minimal `DevIno` struct if no equivalent type already exists.
- Port `get_root_dev_ino` first:
  - query metadata for `/`
  - extract `dev` and `ino` via `MetadataExt`
  - return `io::Result<DevIno>`
- Keep the function signature narrow and aligned to current call needs.
- Add unit tests for:
  - successful lookup of `/`
  - nonzero or stable-looking `dev`/`ino` values without over-constraining platform behavior

### Phase 2: Port `logical_getcwd` with standard-library path ownership

- Translate `logical_getcwd` into a Rust helper using `PathBuf`/`OsString`.
- Preserve the original decision flow as closely as possible instead of redesigning it.
- Replace C string concatenation and resizing with Rust-owned path handling.
- Use `Result<PathBuf>` or `Result<OsString>` depending on the surrounding binary’s output expectations.
- Ensure all temporary buffers disappear naturally through ownership and scope.
- Add focused tests for:
  - successful cwd retrieval in a normal directory context
  - behavior when path data is consumed as non-UTF-8-capable OS strings
  - failure propagation if an underlying filesystem/environment access fails and can be injected or simulated within the existing test setup

### Phase 3: Integrate with the `pwd` main path

- Replace the old C-driven call sites with the new Rust helper calls in the binary/module entry path.
- Keep output formatting and exit-path behavior aligned with the existing project conventions.
- Ensure the call order between `logical_getcwd` and `get_root_dev_ino` matches the migrated logic rather than introducing a new orchestration layer.
- Remove or isolate any obsolete transitional code paths created during the port.

### Phase 4: Stabilize tests and perform migration cleanup

- Run `cargo test` and fix edge cases around Unix metadata access and path ownership.
- Verify that no manual buffer sizing or pointer-style output handling remains in the migrated code.
- Confirm that the final Rust module mirrors the original file/function responsibilities without adding unsupported capabilities.
- Document any unavoidable Unix-only assumptions inline in code comments near `MetadataExt` usage.