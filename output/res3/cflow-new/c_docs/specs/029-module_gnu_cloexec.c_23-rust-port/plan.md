# Implementation Plan: module_gnu_cloexec.c_23

## Summary

Port `gnu/cloexec.c` to a Rust module that preserves the current low-level behavior around close-on-exec file descriptor handling, limited to the existing exported functions:

- `set_cloexec_flag`
- `dup_cloexec`

The Rust implementation should stay narrowly scoped to the current module responsibilities: manipulating file descriptor flags and duplicating descriptors with close-on-exec semantics. The preferred approach is to use Rust’s standard library where it directly exposes required primitives, and use targeted POSIX bindings only for operations that are inherently descriptor- and `fcntl`-based. The port should keep the API surface minimal, translate C integer/error conventions into idiomatic Rust `Result`-based handling internally, and preserve OS-level behavior for Unix platforms.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum practical toolchain target: Rust 1.74+

### Primary Dependencies
- Rust standard library
- `libc` crate for Unix file descriptor operations not covered directly by `std`

Dependency choice should remain minimal. No additional crates are needed unless the surrounding project already standardizes another POSIX wrapper, which is not indicated here.

### Testing
- `cargo test`

Tests should focus on:
- setting and clearing `FD_CLOEXEC`
- duplicating a valid file descriptor
- confirming duplicated descriptors receive close-on-exec as intended
- validating error behavior on invalid file descriptors

### Performance Goals
- Match C implementation characteristics closely
- Keep operations as direct syscalls or syscall-equivalent libc calls
- Avoid heap allocation in core descriptor-manipulation paths
- Introduce no additional descriptor copies beyond those required by the original behavior

## Module Mapping

### C to Rust File Mapping
- `gnu/cloexec.c` → `src/module_gnu_cloexec.rs`

If the project already uses per-module files under another existing path convention, follow that convention without introducing extra layers.

### Function Mapping
- `set_cloexec_flag` → `pub(crate) fn set_cloexec_flag(fd: RawFd, value: bool) -> std::io::Result<()>`
- `dup_cloexec` → `pub(crate) fn dup_cloexec(fd: RawFd) -> std::io::Result<RawFd>`

Notes:
- Use `std::os::unix::io::RawFd`
- Return `std::io::Result` instead of C-style sentinel returns in Rust-facing code
- If a compatibility layer is required by the surrounding project, keep it confined to thin call-site adaptation rather than expanding this module

## Data Model

This module has no named C data structures to port.

### C to Rust Type Mapping
- C `int fd` → Rust `RawFd`
- C boolean/int flag parameters → Rust `bool` where semantics are binary
- C error signaling via return codes / `errno` → Rust `std::io::Error` and `std::io::Result<T>`

### Resource Ownership
- Raw file descriptors remain non-owning in `set_cloexec_flag`
- `dup_cloexec` returns a newly created raw descriptor; ownership transfers to the caller
- Avoid wrapping returned descriptors in `File` or other owning abstractions unless the existing Rust project API already requires that exact form

## Implementation Phases

### Phase 1: Create Module Skeleton and Map Low-Level APIs
- Add the Rust module file corresponding to `gnu/cloexec.c`
- Import:
  - `std::io`
  - `std::os::unix::io::RawFd`
  - `libc`
- Define the two target functions only:
  - `set_cloexec_flag`
  - `dup_cloexec`
- Document internal assumptions:
  - Unix-only behavior
  - caller-managed descriptor lifetime
  - direct mapping to OS descriptor semantics

### Phase 2: Port `set_cloexec_flag`
- Implement flag retrieval with `fcntl(fd, F_GETFD)`
- Modify `FD_CLOEXEC` bit according to the input boolean
- Apply updated flags with `fcntl(fd, F_SETFD, new_flags)`
- Convert `-1` returns into `io::Error::last_os_error()`
- Keep behavior strict:
  - no retries
  - no alternate fallback paths unless required by the original C logic
- Ensure no memory ownership issues arise; this function should remain allocation-free

### Phase 3: Port `dup_cloexec`
- Implement descriptor duplication with close-on-exec preserved at creation when supported:
  - prefer `fcntl(fd, F_DUPFD_CLOEXEC, 0)` where available through `libc`
- If the original C module contains a compatibility fallback, mirror only that fallback:
  - duplicate descriptor
  - then call `set_cloexec_flag` on the new descriptor
  - if flag setting fails, close the duplicated descriptor before returning the error
- Preserve ownership correctness on all failure paths:
  - no leaked duplicated descriptors
  - no accidental close of the input descriptor

### Phase 4: Validate Behavior with Focused Tests
- Add unit tests or integration tests using temporary files/pipes to obtain valid descriptors
- Cover:
  - setting `FD_CLOEXEC` to true on a valid descriptor
  - clearing `FD_CLOEXEC` on a valid descriptor
  - duplicating a descriptor successfully
  - verifying duplicated descriptor is distinct from source
  - verifying invalid descriptor inputs produce errors
- Keep tests Unix-specific and minimal
- Run via `cargo test`