# Implementation Plan

## Summary

Port the `fcntl.c` logic for `rpl_fcntl_DUPFD` and `rpl_fcntl_DUPFD_CLOEXEC` into a single Rust module that preserves existing descriptor-duplication behavior and error semantics as closely as practical. The Rust implementation should remain narrowly scoped to the current module responsibilities: invoking `fcntl`-style duplication operations, handling `FD_CLOEXEC` behavior where required, and returning OS errors in an idiomatic Rust form without introducing broader abstractions.

The technical approach is to implement a Rust module that wraps the required Unix file-descriptor operations using the standard library where possible and minimal platform bindings where necessary. Since these functions operate on raw file descriptors and map directly to Unix system behavior, the implementation should use `std::os::fd::RawFd` and propagate errors through `std::io::Result`. Any unsafe calls should be tightly contained around the actual syscall boundary, with explicit ownership rules so duplicated descriptors are returned as raw descriptors and not accidentally closed early.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `fcntl`, `F_DUPFD`, `F_DUPFD_CLOEXEC`, `F_GETFD`, and `F_SETFD` constants and calls, if the standard library alone does not expose the needed operations
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve syscall-level behavior with no meaningful overhead beyond thin Rust wrappers
  - Avoid extra allocations
  - Keep the number of syscalls aligned with current C logic, especially for `CLOEXEC` handling paths

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `fcntl.c` | `src/main_root_rpl_fcntl_14.rs` | Direct migration target for the two replacement `fcntl` duplication helpers |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `rpl_fcntl_DUPFD` | `pub(crate) fn rpl_fcntl_dupfd(fd: RawFd, lowfd: RawFd) -> io::Result<RawFd>` | Returns a duplicated descriptor at or above `lowfd` |
| `rpl_fcntl_DUPFD_CLOEXEC` | `pub(crate) fn rpl_fcntl_dupfd_cloexec(fd: RawFd, lowfd: RawFd) -> io::Result<RawFd>` | Uses `F_DUPFD_CLOEXEC` when available; otherwise duplicates then sets `FD_CLOEXEC` |

### Project Integration

- Register the Rust module from the crate root using standard Rust module declarations only.
- Keep all logic in one source file unless the existing Rust project layout already requires a different placement.
- Do not create new utility layers beyond what is needed to replace the existing C functions.

## Data Model

This module has no named C structs and does not require a new persistent Rust data model.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | No dedicated Rust type | The module is function-oriented and operates on `RawFd` values and OS error codes |

### Scalar and System Type Mapping

| C Type/Concept | Rust Type | Notes |
|---|---|---|
| file descriptor `int` | `std::os::fd::RawFd` | Canonical Rust Unix FD representation |
| errno-based failure | `std::io::Error` / `std::io::Result<T>` | Preserve OS error information |
| `fcntl` command constants | `libc` constants | Direct syscall-compatible values |
| close-on-exec flag | integer flag bits | Managed through `fcntl` flag queries/updates |

### Memory Management and Ownership

- Input descriptors remain borrowed logically by the function and must not be closed by the implementation.
- Returned duplicated descriptors transfer ownership to the caller as raw descriptors.
- Unsafe blocks must be limited to direct `fcntl` calls and any related errno-sensitive operations.
- If a fallback `dup + set flag` path is used for `CLOEXEC`, ensure the duplicated descriptor is closed on intermediate failure before returning the error.

## Implementation Phases

## Phase 1: Create Module Skeleton and Low-Level Bindings

- Add `src/main_root_rpl_fcntl_14.rs`.
- Define the two Rust functions corresponding to:
  - `rpl_fcntl_DUPFD`
  - `rpl_fcntl_DUPFD_CLOEXEC`
- Import:
  - `std::io`
  - `std::os::fd::RawFd`
  - `libc` items needed for `fcntl` operations
- Add the module declaration in the crate root or existing parent module file.
- Keep function scope and visibility limited to current project needs (`pub(crate)` unless broader visibility is already required by the branch structure).

### Phase 1 Deliverable

A compiling module shell with exact function entry points and contained syscall helpers, but without finalized fallback and test coverage.

## Phase 2: Port Core Descriptor Duplication Logic

- Implement `rpl_fcntl_dupfd` as a thin wrapper over `fcntl(fd, F_DUPFD, lowfd)`.
- Convert return values:
  - nonnegative result -> `Ok(new_fd)`
  - negative result -> `Err(io::Error::last_os_error())`
- Implement `rpl_fcntl_dupfd_cloexec` with the same return-style contract.
- Prefer `F_DUPFD_CLOEXEC` directly where supported by the target platform bindings.
- If the C behavior includes compatibility fallback, mirror that behavior narrowly:
  - duplicate using `F_DUPFD`
  - fetch descriptor flags
  - set `FD_CLOEXEC`
  - close the new descriptor if flag setting fails
  - return the encountered OS error
- Preserve C-like ordering of operations so observable OS behavior remains aligned.

### Phase 2 Deliverable

Complete function behavior matching the current C module responsibilities, including explicit error propagation and cleanup on fallback-path failure.

## Phase 3: Validate Error Handling and Ownership Semantics

- Review all unsafe calls for:
  - correct argument types
  - no aliasing or lifetime misuse
  - immediate errno capture on failure
- Ensure no descriptor leaks occur on partial failure.
- Confirm callers receive owned duplicated file descriptors and that the input descriptor is never modified except through kernel duplication semantics.
- Keep implementation comments focused on syscall safety and fallback rationale only.

### Phase 3 Deliverable

Reviewed implementation with stable ownership/error semantics and no extra abstractions beyond the port.

## Phase 4: Add Targeted Tests

- Add unit tests or integration-style tests under standard Rust test layout.
- Cover:
  - successful duplication with `rpl_fcntl_dupfd`
  - successful duplication with `rpl_fcntl_dupfd_cloexec`
  - invalid input descriptor returns error
  - `lowfd` lower-bound behavior insofar as it can be reliably checked
  - `FD_CLOEXEC` set on the duplicated descriptor for the cloexec path
- Use standard library file-opening APIs to obtain valid descriptors for tests.
- Where assertions require reading descriptor flags, use the same minimal `libc::fcntl` access in test code.
- Keep tests Unix-specific if the module is Unix-specific.

### Phase 4 Deliverable

`cargo test` coverage for the migrated functions, focused strictly on behavior already present in the C module.