# Implementation Plan: module_gnu_cloexec.c_23

## Summary

Port `gnu/cloexec.c` into a focused Rust module that preserves the existing low-level file-descriptor behavior of:

- `set_cloexec_flag`
- `dup_cloexec`

The Rust implementation should remain narrow in scope: migrate the current descriptor-manipulation logic without adding broader I/O abstractions. The technical approach is to expose small Rust functions that operate on raw file descriptors, use standard library OS descriptor types where practical, and rely on direct Unix syscalls through `libc` only for operations not covered by `std`. Error handling should convert syscall failures into `std::io::Result`, and resource ownership must remain explicit so the port does not accidentally change descriptor lifetime semantics.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `fcntl`, `dup`, `dup2`/`F_DUPFD_CLOEXEC`, and flag constants on Unix
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C behavior with negligible overhead beyond direct syscall wrapping
  - Avoid extra allocations
  - Keep descriptor operations single-pass and syscall-minimal
  - Preserve close-on-exec behavior without introducing redundant descriptor duplication

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `gnu/cloexec.c` | `src/module_gnu_cloexec.rs` | Direct port of descriptor helper logic |
| `set_cloexec_flag` | `pub(crate) fn set_cloexec_flag(fd: RawFd, value: bool) -> io::Result<()>` | Maps C flag mutation to Rust syscall wrapper |
| `dup_cloexec` | `pub(crate) fn dup_cloexec(fd: RawFd) -> io::Result<RawFd>` | Returns duplicated raw descriptor with CLOEXEC set |

If the project already uses a module tree for cluster modules, place this file at the equivalent existing location and only wire the minimal `mod` declaration needed for compilation.

## Data Model

This module does not define persistent C structs and should remain structurally minimal in Rust.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| raw file descriptor `int` | `std::os::fd::RawFd` | Direct semantic match on Unix |
| integer return status / `-1` error | `std::io::Result<T>` | Converts errno-based failure into idiomatic Rust result handling |
| descriptor flags via `fcntl` | local integer flag values (`libc::c_int`) | Kept local to implementation, not promoted into public data types |

No new Rust structs or enums are required unless a small private helper is needed to avoid duplicated syscall handling.

## Implementation Phases

### Phase 1: Establish module skeleton and function signatures

- Create the Rust module file for the ported implementation.
- Define Rust signatures for:
  - `set_cloexec_flag`
  - `dup_cloexec`
- Import only the required Unix descriptor and I/O error types.
- Add the minimal module declaration in the crate tree.
- Keep visibility constrained to the current crate unless existing call sites require otherwise.

### Phase 2: Port syscall logic and error handling

- Implement `set_cloexec_flag` using `fcntl` to:
  - read current descriptor flags
  - set or clear `FD_CLOEXEC`
  - write updated flags back
- Implement `dup_cloexec` using the closest available C behavior:
  - prefer `F_DUPFD_CLOEXEC` when appropriate to preserve atomic CLOEXEC duplication
  - use a fallback sequence only if required by target compatibility already present in the C logic
- Convert all syscall failures to `io::Error::last_os_error()`.
- Preserve raw descriptor ownership semantics:
  - input `fd` is borrowed logically and must not be closed
  - returned duplicated descriptor transfers ownership to the caller
- Keep all `unsafe` blocks tightly scoped and documented by syscall contract.

### Phase 3: Migrate call behavior and validate semantics

- Update existing internal call sites to use the Rust module functions.
- Verify that no call site assumes automatic descriptor wrapping or closing.
- Confirm behavior for:
  - valid descriptor with CLOEXEC enabled
  - valid descriptor with CLOEXEC cleared
  - invalid descriptor error propagation
  - duplicated descriptor carries close-on-exec state as intended
- Ensure the port does not change return-value expectations beyond Rust `Result` conversion.

### Phase 4: Add focused tests and finalize integration

- Add unit tests under `cargo test` for Unix platforms covering:
  - setting CLOEXEC on a temporary descriptor
  - clearing CLOEXEC on a temporary descriptor
  - duplication returns a distinct valid descriptor
  - invalid fd produces an error
- Use standard library facilities to obtain temporary file descriptors and inspect flags via `fcntl`.
- Confirm all tests clean up descriptors deterministically to avoid leaks.
- Run formatting and existing test suite to ensure the new module integrates without expanding scope.