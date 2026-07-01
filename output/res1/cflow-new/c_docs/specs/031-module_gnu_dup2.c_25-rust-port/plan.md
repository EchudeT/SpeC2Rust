# Implementation Plan: module_gnu_dup2.c_25

## Summary

Port `gnu/dup2.c` into a Rust module that preserves the existing file-descriptor duplication behavior and platform-specific branching already present in the C source. The Rust implementation should stay narrowly scoped to the current module surface: translating `dup2_nothrow`, `ms_windows_dup2`, `klibc_dup2dirfd`, and `rpl_dup2` into idiomatic Rust functions while keeping the same low-level operating-system interaction model.

The implementation approach should prefer the Rust standard library for descriptor/file abstractions where practical, but use direct OS calls through `libc` for `dup2`-style semantics, errno-compatible failure reporting, and platform-specific details that are not exposed by `std`. The migration should focus on preserving observable behavior, especially around invalid descriptors, same-descriptor cases, directory-descriptor handling, and Windows-specific duplication paths.

## Technical Context

- **Language/Version**: Rust 1.76+ stable
- **Primary Dependencies**:
  - `libc` for POSIX/Unix and Windows C runtime descriptor operations
  - Rust standard library (`std::os::fd`, `std::io`, conditional compilation facilities)
- **Testing**:
  - `cargo test`
  - Unit tests with platform-conditional coverage for Unix/Windows branches as applicable
- **Performance Goals**:
  - Maintain syscall-level performance comparable to the C implementation
  - Avoid unnecessary allocations and descriptor wrapper churn
  - Keep the implementation as thin as possible over the native OS duplication APIs

## Module Mapping

### Source File Mapping
- `gnu/dup2.c` → `src/module_gnu_dup2.rs`

### Function Mapping
- `dup2_nothrow` → `pub(crate) fn dup2_nothrow(oldfd: RawFd, newfd: RawFd) -> std::io::Result<()>`
- `ms_windows_dup2` → `#[cfg(windows)] pub(crate) fn ms_windows_dup2(oldfd: RawFd, newfd: RawFd) -> std::io::Result<()>`
- `klibc_dup2dirfd` → `#[cfg(unix)] pub(crate) fn klibc_dup2dirfd(oldfd: RawFd, newfd: RawFd) -> std::io::Result<()>`
- `rpl_dup2` → `pub(crate) fn rpl_dup2(oldfd: RawFd, newfd: RawFd) -> std::io::Result<()>`

### Integration Notes
- Keep all migrated functions in a single Rust module unless the existing crate layout already mandates another file placement.
- Use `cfg(unix)` and `cfg(windows)` to mirror the original compile-time branching rather than introducing new abstraction layers.
- If the surrounding project uses a crate-level replacement naming convention, keep `rpl_dup2` named as-is to minimize migration risk.

## Data Model

This C module does not define named structs; the only listed data structure is anonymous, so the Rust port should avoid inventing persistent data models.

### Data-Structure Mapping
- anonymous C data / local temporary state → Rust local variables and small helper expressions
- C file descriptor integers (`int`) → `std::os::fd::RawFd` on Unix and the project’s descriptor-compatible raw integer type on Windows as needed through `libc`
- C error signaling via return codes and `errno` → `std::io::Result<()>`, constructed from `std::io::Error::last_os_error()`

### Memory and Resource Handling
- Do not introduce ownership-bearing wrappers such as `File` unless required by an existing surrounding API.
- Pass raw descriptors directly to OS functions to avoid accidental close-on-drop behavior.
- Preserve descriptor lifetimes as external to this module; the module should not assume ownership of input descriptors.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and migrate shared duplication path

### Goals
- Establish the Rust file for the port.
- Migrate the common low-level descriptor duplication behavior with minimal transformation.
- Define the public/internal function signatures used by the rest of the crate.

### Tasks
- Add `src/module_gnu_dup2.rs`.
- Import:
  - `std::io`
  - `libc`
  - platform-specific raw descriptor traits/types from `std::os::fd` or conditional equivalents
- Implement `dup2_nothrow` as the thinnest safe wrapper practical over the native duplication call:
  - call `libc::dup2` or equivalent under `unsafe`
  - convert negative/error returns to `io::Error::last_os_error()`
  - return `Ok(())` on success
- Implement the common signature and return type conventions for all four functions up front so later phases only fill in platform-specific logic.

### Acceptance Criteria
- The module compiles with placeholder-free signatures.
- The shared low-level duplication helper returns `io::Result<()>`.
- No heap allocation or ownership transfer is introduced.

## Phase 2: Port platform-specific branches exactly at module scope

### Goals
- Translate the Windows-specific and Unix-specific special cases without broad refactoring.
- Preserve compile-time platform selection behavior from the C source.

### Tasks
- Implement `ms_windows_dup2` behind `#[cfg(windows)]`:
  - map the C runtime duplication path to the nearest `libc`/CRT call sequence
  - preserve failure handling via OS error conversion
  - keep descriptor validation and same-descriptor handling aligned with the C logic
- Implement `klibc_dup2dirfd` behind `#[cfg(unix)]`:
  - port the directory-descriptor-specific path directly
  - use raw descriptor operations only
  - preserve return/failure behavior expected by `rpl_dup2`
- Where the C source has compile-time conditionals inside a function, mirror them with Rust `cfg` blocks rather than splitting into extra modules.

### Acceptance Criteria
- Unix builds exclude Windows-only code cleanly.
- Windows builds exclude Unix-only code cleanly.
- All platform-specific functions are implemented without adding new helper subsystems.

## Phase 3: Implement `rpl_dup2` orchestration and edge-case parity

### Goals
- Complete the top-level replacement function.
- Preserve C behavior for dispatch and edge cases.

### Tasks
- Port `rpl_dup2` as the entry point that chooses between:
  - direct nothrow duplication
  - Windows-specific handling
  - klibc/directory-descriptor path
  - any same-fd short-circuit present in the C logic
- Keep condition ordering aligned with the original source to reduce behavior drift.
- Ensure invalid descriptor handling and `errno`-equivalent reporting remain observable through `io::Result`.
- Review all `unsafe` blocks:
  - narrow each block to the specific syscall invocation
  - document the raw-fd preconditions inline

### Acceptance Criteria
- `rpl_dup2` is the only top-level behavioral dispatcher.
- Error cases map consistently to `io::Error`.
- The implementation remains limited to the original four functions and local helpers only if strictly needed.

## Phase 4: Add targeted tests for descriptor semantics

### Goals
- Verify behavioral parity for success and failure cases.
- Keep testing constrained to this module’s current responsibilities.

### Tasks
- Add unit tests in the module or `tests/` only if required by existing project patterns.
- Cover:
  - successful duplication from one valid descriptor to another
  - duplication when `oldfd == newfd`
  - invalid source descriptor failure
  - invalid destination descriptor failure as supported by platform behavior
- Add conditional tests for:
  - Unix directory-descriptor path if it is observable and testable
  - Windows-specific duplication branch under `#[cfg(windows)]`
- Use temporary files and raw descriptors from standard library file handles only for test setup; avoid introducing new runtime utilities.

### Acceptance Criteria
- `cargo test` passes on supported target platforms.
- Tests validate descriptor-level behavior without changing module scope.
- No benchmark, recovery, or auxiliary infrastructure is added.