# Implementation Plan

## Summary

Port `xgetcwd.c` into a Rust module that provides the `xgetcwd` behavior used by `pwd`. The Rust implementation should preserve the existing responsibility of obtaining the current working directory while replacing C-style heap management and null/error signaling with Rust ownership and `Result`-based error propagation.

The implementation should prefer the Rust standard library path APIs and current-directory facilities as the primary migration path. The port should stay narrowly scoped to the existing C function and its call expectations, avoiding new abstractions beyond what is needed to mirror the original module boundary and integrate with the rest of the `main_cluster`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::env`, `std::path`, `std::io`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the practical performance of the C implementation for normal `pwd` usage.
  - Avoid unnecessary path copies beyond conversion required by Rust ownership.
  - Keep allocation bounded to the path value returned by the operating system.
  - Preserve straightforward error paths without retry loops or extra buffering layers unless required during migration.

## Module Mapping

- **C source**: `xgetcwd.c`
- **Rust target**: `src/main_root_xgetcwd.rs` or the equivalent existing crate-local module file used for this branch's `main_cluster`
- **C function -> Rust function mapping**:
  - `xgetcwd` -> `pub(crate) fn xgetcwd(...) -> Result<..., std::io::Error>`

If the surrounding port already uses a shared crate-local error/result convention, adapt only the signature shape needed for call-site compatibility, while keeping the implementation centered on `std::env::current_dir()`.

## Data Model

This module has no named C structs to migrate.

### Function-level type mapping

- C dynamically allocated path buffer (`char *`) -> Rust owned path/string value:
  - Prefer `std::path::PathBuf` internally and at the module boundary if callers can accept path types.
  - Use `std::ffi::OsString`/`PathBuf` internally to preserve platform-correct path handling.
  - Convert to `String` only if the existing Rust port interface requires textual output at this layer.

- C error reporting through null return and external errno -> Rust `Result<T, std::io::Error>`

### Memory Management Mapping

- Manual allocation/free of returned path buffer in C -> Rust ownership of `PathBuf`/`String`, released automatically on drop.
- No direct buffer-size management should be retained unless required by an exact call signature during migration.

## Implementation Phases

### Phase 1: Establish module skeleton and function signature

- Create the Rust module file corresponding to `xgetcwd.c`.
- Define the Rust `xgetcwd` function with visibility consistent with current crate usage.
- Select the narrowest return type compatible with existing migrated callers:
  - `PathBuf` if path semantics are preserved across the port.
  - `String` only if this module is expected to produce display-ready text.
- Define error handling as `Result<_, std::io::Error>` instead of C null/errno behavior.

### Phase 2: Port core current-directory retrieval logic

- Implement `xgetcwd` using `std::env::current_dir()`.
- Keep the implementation minimal and centered on direct OS-backed current-directory retrieval.
- Preserve any required caller-visible behavior around owned return values without introducing helper subsystems.
- Ensure path handling remains platform-correct by avoiding premature UTF-8 assumptions unless required by the consuming interface.

### Phase 3: Integrate call expectations and error conversion

- Adjust the function boundary to align with the existing Rust port of `pwd` main flow.
- Where callers previously expected C-string-like data, perform the minimum necessary conversion at the boundary.
- Map failure cases cleanly into `std::io::Error` propagation.
- Confirm there are no lingering assumptions about manual freeing, mutable output buffers, or sentinel null returns at call sites.

### Phase 4: Add focused tests and finalize migration

- Add unit tests covering:
  - successful retrieval of the current working directory
  - returned value being non-empty
  - error-path behavior only if it can be exercised without artificial platform-specific hooks
- Verify the module compiles and integrates with `cargo test`.
- Remove or avoid any C-era buffer-management patterns that are no longer needed in Rust.