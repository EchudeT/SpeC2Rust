# Implementation Plan: module_gnu_close.c_24

## Summary

This module ports the behavior of `gnu/close.c` into Rust, limited to the existing responsibilities of `close_nothrow` and `rpl_close`. The Rust implementation should preserve low-level file-descriptor close semantics, especially around errno-style failure handling and any retry or validation logic present in the C code, without introducing broader I/O abstractions.

The technical approach is to implement a small Rust module that works directly with Unix file descriptors and returns standard Rust error results derived from OS error codes. The implementation should stay close to the C control flow: translate the existing close wrapper logic into narrow Rust functions, use direct OS bindings where needed, and keep ownership and lifetime rules explicit so that file descriptors are not accidentally closed twice.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for direct `close(2)` access and errno-compatible OS interaction when standard library APIs are insufficient
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C-level behavior with negligible overhead beyond a direct system call wrapper
  - Avoid heap allocation
  - Preserve constant-time wrapper behavior around `close`
  - Do not introduce extra descriptor tracking or synchronization layers

## Module Mapping

### Source File Mapping

- `gnu/close.c` -> `src/module_gnu_close.rs`

### Function Mapping

- `close_nothrow` -> `pub(crate) fn close_nothrow(fd: std::os::fd::RawFd) -> std::io::Result<()>`
- `rpl_close` -> `pub(crate) fn rpl_close(fd: std::os::fd::RawFd) -> std::io::Result<()>`

### Rust Module Placement

The Rust code should live as a single module file corresponding to the original C file. If the crate already has a module tree for migrated GNU replacements, this file should be added there without creating additional abstraction layers.

## Data Model

This module has no declared C structs to migrate.

### C to Rust Type Mapping

- `int fd` -> `std::os::fd::RawFd`
- `int` return status / `-1` error convention -> `std::io::Result<()>`
- `errno`-based failure signaling -> `std::io::Error` via `last_os_error()`

### Memory and Resource Model

- File descriptors remain borrowed integer handles unless the surrounding crate already uses owned FD wrappers.
- This module should not convert the API to `OwnedFd` if that would change call-site ownership expectations.
- No heap-managed state is required.
- Error propagation must preserve OS error codes accurately enough for callers that depend on close failure distinctions.

## Implementation Phases

## Phase 1: Port the low-level close wrapper

- Create `src/module_gnu_close.rs`.
- Implement `close_nothrow` as the direct translation target for the simplest close path.
- Use `libc::close` rather than `std::fs::File`-based abstractions to preserve raw descriptor behavior.
- Convert the C success/error convention into `std::io::Result<()>`.
- Ensure the function does not panic and does not allocate.

### Deliverables

- New Rust module file with `close_nothrow`
- Minimal unit tests for:
  - successful close of a valid descriptor
  - failure on an invalid descriptor returning an OS error

## Phase 2: Port replacement close behavior

- Implement `rpl_close` by mirroring the control flow in `gnu/close.c`.
- Preserve any special handling around interrupted calls, invalid descriptors, or errno-sensitive behavior found in the C implementation.
- Keep the function narrow and avoid merging it into `close_nothrow` if the original file keeps them distinct.
- Verify that error mapping does not lose the distinction between close success and close failure states.

### Deliverables

- `rpl_close` in the same module
- Focused tests covering:
  - normal descriptor close
  - invalid descriptor path
  - any distinct behavior between `rpl_close` and `close_nothrow` present in the original C logic

## Phase 3: Integrate and align with crate conventions

- Export the module according to the existing crate layout.
- Update any internal call sites that currently depend on the C port boundary to use the Rust functions.
- Keep signatures and visibility restricted to the narrowest level compatible with existing usage.
- Review for double-close hazards and confirm that migrated callers do not assume ownership transfer unless already established.

### Deliverables

- Module wired into `src/lib.rs` or the existing module tree
- Compilation clean on the target Unix platform
- Passing `cargo test`

## Phase 4: Behavior verification and cleanup

- Compare Rust behavior against the original C file for return-value and error-path equivalence.
- Remove any temporary migration scaffolding used during implementation.
- Confirm there are no extra helper modules, wrappers, or convenience APIs beyond the original scope.

### Deliverables

- Finalized Rust port of `gnu/close.c`
- Clean test suite with no scope expansion
- Short code comments only where needed to explain syscall/error-handling choices