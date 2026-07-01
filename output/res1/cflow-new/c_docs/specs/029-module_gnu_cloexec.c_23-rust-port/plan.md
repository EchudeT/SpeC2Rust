# Implementation Plan: module_gnu_cloexec.c_23

## Summary

This module ports `gnu/cloexec.c` into Rust with a narrow scope: preserve the existing close-on-exec file-descriptor behavior implemented by the C functions `set_cloexec_flag` and `dup_cloexec`, while adapting them to Rust’s ownership and error-reporting model.

The Rust implementation should remain a small, focused module that operates on raw Unix file descriptors. The technical approach is:

- map the C file to a single Rust module file,
- implement the same two functions with behavior aligned to the original Unix semantics,
- use direct Unix descriptor APIs through the standard library and, where necessary, minimal platform bindings for `fcntl`/`dup`-style operations,
- express failures as `std::io::Result`,
- avoid introducing broader abstractions beyond what is required to migrate the existing functions.

The implementation should carefully preserve descriptor ownership rules:
- `set_cloexec_flag` updates descriptor flags without taking ownership of the descriptor,
- `dup_cloexec` returns a newly created descriptor and transfers ownership of that new descriptor to the caller.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for Unix `fcntl`, `F_GETFD`, `F_SETFD`, `FD_CLOEXEC`, and duplication constants/functions if standard library coverage is insufficient for exact behavior
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve system-call-level performance comparable to the C implementation,
  - avoid heap allocation,
  - keep the implementation to a minimal number of syscalls per operation,
  - do not add wrapper layers that materially change descriptor handling cost.

## Module Mapping

### C to Rust File Mapping

- `gnu/cloexec.c` → `src/module_gnu_cloexec.rs`

If the project already groups migrated modules under an existing namespace, place the file within that existing layout, but do not introduce new architectural layers solely for this port.

### Function Mapping

- `set_cloexec_flag` → `pub(crate)` or `pub` Rust function `set_cloexec_flag`
- `dup_cloexec` → `pub(crate)` or `pub` Rust function `dup_cloexec`

Recommended Rust signatures:

```rust
pub fn set_cloexec_flag(fd: std::os::fd::RawFd, value: bool) -> std::io::Result<()>;
pub fn dup_cloexec(fd: std::os::fd::RawFd) -> std::io::Result<std::os::fd::OwnedFd>;
```

Notes:

- `RawFd` matches the borrowed/non-owning role of the input descriptor.
- `OwnedFd` is the preferred return type for a duplicated descriptor because it captures ownership safely and prevents leaks.
- If surrounding project conventions require returning `RawFd`, keep the external signature aligned with that convention, but implement internally with `OwnedFd` where practical and convert only at the boundary.

## Data Model

This module has no declared C structs or custom data structures to port.

### C to Rust Type Mapping

- `int fd` → `std::os::fd::RawFd`
- duplicated owned file descriptor result → `std::os::fd::OwnedFd`
- C integer success/failure convention → `std::io::Result<T>`

### Error Model

The C implementation likely reports failure through return codes and `errno`. In Rust:

- convert syscall failures into `std::io::Error::last_os_error()`,
- use `std::io::Result<()>` for flag-setting operations,
- use `std::io::Result<OwnedFd>` for descriptor duplication,
- do not suppress OS errors or invent new error enums unless the existing project already requires one.

## Implementation Phases

### Phase 1: Create the Rust module skeleton and map syscall boundaries

- Add the Rust module file corresponding to `gnu/cloexec.c`.
- Declare the two public-facing functions:
  - `set_cloexec_flag`
  - `dup_cloexec`
- Select the exact Unix API surface needed:
  - standard library fd types from `std::os::fd`,
  - `libc` calls/constants for `fcntl` and duplication behavior if needed.
- Keep all code Unix-specific and avoid speculative cross-platform abstractions.

#### Deliverables
- `src/module_gnu_cloexec.rs`
- function signatures established
- imports and minimal module-level documentation describing descriptor ownership expectations

### Phase 2: Implement `set_cloexec_flag`

- Implement flag retrieval using `fcntl(fd, F_GETFD)`.
- Modify only the `FD_CLOEXEC` bit according to the requested boolean.
- Write updated flags back with `fcntl(fd, F_SETFD, flags)`.
- Return `Ok(())` on success and `io::Error` on failure.
- Ensure the function does not close, duplicate, or otherwise assume ownership of the input descriptor.

#### Technical notes
- Guard against negative syscall returns and convert them directly to `last_os_error`.
- Do not introduce retry loops or recovery behavior unless clearly required by the original file’s semantics.
- Preserve exact bitwise behavior; only the close-on-exec flag should change.

#### Deliverables
- completed `set_cloexec_flag`
- focused unit tests for:
  - setting the flag,
  - clearing the flag,
  - invalid fd failure behavior

### Phase 3: Implement `dup_cloexec`

- Implement descriptor duplication with close-on-exec set on the new descriptor.
- Prefer the most direct OS-supported operation available to mirror the C code’s intent:
  - use `fcntl(..., F_DUPFD_CLOEXEC, ...)` if this matches the original semantics and target assumptions,
  - otherwise perform duplication and then set `FD_CLOEXEC` on the duplicate as a fallback only if the original C behavior requires such compatibility handling.
- Return the new descriptor as `OwnedFd`.

#### Technical notes
- Ownership begins only after a successful duplication syscall.
- Convert the returned raw descriptor into `OwnedFd` immediately after validating success.
- Avoid leaks on intermediate failure paths.
- Keep the implementation minimal and close to the original function logic rather than generalizing descriptor utilities.

#### Deliverables
- completed `dup_cloexec`
- unit tests for:
  - successful duplication of a valid fd,
  - returned duplicate being distinct from the source fd,
  - duplicate having close-on-exec set,
  - invalid fd failure behavior

### Phase 4: Validation and integration cleanup

- Run `cargo test`.
- Verify the module compiles cleanly on the intended Unix target.
- Align visibility (`pub` vs `pub(crate)`) and file inclusion with the existing project structure.
- Remove any unused helper code introduced during migration.
- Confirm that the final Rust module covers only the original file’s functions and does not introduce extra facilities.

#### Deliverables
- final integrated module
- passing tests
- concise comments on any platform-specific syscall choice retained from the port