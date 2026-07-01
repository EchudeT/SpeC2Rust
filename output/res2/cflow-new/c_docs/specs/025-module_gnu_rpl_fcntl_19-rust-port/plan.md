# Implementation Plan: module_gnu_rpl_fcntl_19

## Summary

This module ports the behavior of `gnu/fcntl.c` into Rust, limited to the two replacement helpers:

- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

The Rust implementation should preserve the existing module scope and focus on translating the file-descriptor duplication logic and associated error propagation. The technical approach is to implement thin Rust functions around platform file-descriptor operations, using the Rust standard library where possible and narrowly scoped OS-specific APIs where necessary. Memory ownership remains simple because the module operates on raw file descriptors rather than heap-managed C structures. Error handling should map C-style failure reporting into `std::io::Result` while preserving OS error codes.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `std::os::fd` for Unix file descriptor handling
  - No third-party crates recommended based on current input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Keep overhead effectively equivalent to the C implementation
  - Avoid extra allocation
  - Perform direct descriptor operations with minimal abstraction
  - Preserve syscall-level behavior and error paths without added retry layers

## Module Mapping

### C to Rust File Mapping

- `gnu/fcntl.c`
  → `src/module_gnu_rpl_fcntl_19.rs`

If the target project already uses a clustered module layout, this file should be added to that existing `src` tree without introducing extra submodules beyond what is required to expose the migrated functions.

### Function Mapping

- `rpl_fcntl_DUPFD`
  → `pub(crate) fn rpl_fcntl_dupfd(...) -> std::io::Result<RawFd>`

- `rpl_fcntl_DUPFD_CLOEXEC`
  → `pub(crate) fn rpl_fcntl_dupfd_cloexec(...) -> std::io::Result<RawFd>`

Notes for mapping:
- Keep signatures close to the C intent: input file descriptor plus minimum target descriptor / flags as needed by the original logic.
- Use Rust snake_case naming internally, while preserving traceability to the C source through comments or docstrings.
- If surrounding project conventions require exact exported names, isolate that naming decision locally without changing module scope.

## Data Model

### Data Structure Mapping

The source analysis reports only an anonymous data structure and the visible functionality is descriptor-oriented. No dedicated persistent C struct appears to require migration.

- Anonymous / implicit C state
  → No standalone Rust struct required

### Primitive and OS Type Mapping

- `int` file descriptor
  → `std::os::fd::RawFd`
- C return code with `-1` on error
  → `std::io::Result<RawFd>`
- `errno`-based failure state
  → `std::io::Error::last_os_error()` or equivalent preserved OS error mapping

### Memory Management

- No heap allocation is expected for core functionality.
- Descriptor ownership must be explicit:
  - Returned duplicated descriptors become owned by the caller.
  - Input descriptors are borrowed as raw integers and must not be closed by this module.
- Any temporary descriptor created during `CLOEXEC` fallback logic must be closed on intermediate failure paths to avoid leaks.

## Implementation Phases

## Phase 1: Create Rust Module Skeleton and Map Interfaces

- Add `src/module_gnu_rpl_fcntl_19.rs`.
- Define the two Rust functions corresponding to:
  - `rpl_fcntl_DUPFD`
  - `rpl_fcntl_DUPFD_CLOEXEC`
- Import only required standard-library OS descriptor types and error types.
- Establish result-based error handling with OS error preservation.
- Add concise source-trace comments referencing the original C file and function names.

## Phase 2: Port Descriptor Duplication Logic

- Translate the `DUPFD` behavior into direct Rust implementation using platform descriptor duplication calls.
- Translate the `DUPFD_CLOEXEC` behavior with close-on-exec handling:
  - Prefer direct close-on-exec duplication behavior when available in the target API surface.
  - If the original C logic includes fallback behavior, port only that same fallback path, including explicit close-on-exec flag setting.
- Preserve C-equivalent validation and failure behavior for invalid descriptors and syscall errors.
- Ensure no descriptor leaks occur on partial failure.

## Phase 3: Integrate Error Semantics and Platform Guarding

- Verify all syscall failure cases return `std::io::Error` with the original OS error code.
- Keep any Unix-specific implementation behind the appropriate Rust OS imports rather than adding broad portability layers.
- Align function visibility and module inclusion with the existing project structure on branch `025-module_gnu_rpl_fcntl_19-rust-port`.
- Avoid introducing helper frameworks; use local private helpers only if needed to directly mirror repeated C logic.

## Phase 4: Add Focused Tests and Finalize Migration

- Add unit tests covering:
  - successful duplication of a valid descriptor
  - duplication with minimum descriptor constraints, if applicable to the original logic
  - `CLOEXEC` duplication success path
  - invalid input descriptor error propagation
- Where supported by the standard library and platform APIs, assert close-on-exec flag behavior for duplicated descriptors.
- Run `cargo test` and correct any mismatches between expected C behavior and Rust results.
- Confirm the final module contains only the migrated functionality from `gnu/fcntl.c` and no expanded capabilities.