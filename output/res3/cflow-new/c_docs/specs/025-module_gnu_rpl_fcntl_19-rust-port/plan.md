# Implementation Plan: module_gnu_rpl_fcntl_19

## Summary

This module ports the logic from `gnu/fcntl.c` related to descriptor duplication helpers into Rust, limited to the two existing entry points:

- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

The Rust implementation should preserve the existing low-level behavior of file descriptor duplication, especially error propagation and close-on-exec handling, while keeping the scope strictly aligned with the current C module. The preferred approach is a thin Rust module that uses standard library types where possible and minimal OS interaction for `fcntl`-style behavior where required by the functions being migrated.

The implementation should avoid introducing broader abstractions around file descriptors. Instead, it should migrate the current C functions into direct Rust equivalents with clear ownership boundaries, explicit unsafe blocks only where OS calls are necessary, and result-based error handling that maps OS failures predictably.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `fcntl` constants and system-call interop, if direct OS calls are required by the migrated logic
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain syscall-level performance comparable to the C implementation
  - Avoid extra allocations
  - Keep wrapper overhead negligible relative to the underlying `fcntl`/descriptor operations

## Module Mapping

### C to Rust File Mapping

- `gnu/fcntl.c` -> `src/module_gnu_rpl_fcntl_19.rs`

### Function Mapping

- `rpl_fcntl_DUPFD` -> `pub(crate) fn rpl_fcntl_dupfd(...) -> Result<RawFd, std::io::Error>`
- `rpl_fcntl_DUPFD_CLOEXEC` -> `pub(crate) fn rpl_fcntl_dupfd_cloexec(...) -> Result<RawFd, std::io::Error>`

### Rust Module Placement

Use a single Rust source file for this migration unit. If the crate has an existing module tree, expose this file through the nearest existing `mod` declaration without creating extra layers beyond what is needed to compile and test the migrated functions.

## Data Model

This module does not define named C structs. The only relevant data model elements are raw operating-system descriptors and integer command/flag values.

### C to Rust Type Mapping

- anonymous / raw file descriptor values -> `std::os::fd::RawFd`
- C integer arguments used for descriptor bounds or flags -> `libc::c_int` internally, or `i32`/`RawFd` at the Rust API boundary as appropriate
- C error reporting via `errno` and integer return codes -> `Result<RawFd, std::io::Error>`

### Ownership and Memory Model

- File descriptors are non-owning inputs to the duplication functions.
- Returned duplicated descriptors are newly created OS resources; ownership transfers to the caller.
- No heap allocation is required for the migrated logic.
- Unsafe code should be limited to direct syscall boundaries and constant/flag interaction.

## Implementation Phases

### Phase 1: Module Skeleton and API Port

- Create `src/module_gnu_rpl_fcntl_19.rs`.
- Define Rust signatures for the two migrated functions only.
- Select the exact argument types based on the original C function parameters, preserving integer width and descriptor semantics.
- Add module exports/imports needed for `RawFd`, `std::io::Error`, and any OS constants.
- Establish a narrow internal helper only if both functions share the same syscall path; do not introduce broader descriptor utility APIs.

### Phase 2: Syscall Logic and Error Mapping

- Port the logic of `rpl_fcntl_DUPFD`.
- Port the logic of `rpl_fcntl_DUPFD_CLOEXEC`.
- Implement direct mapping of OS failure states into `std::io::Error::last_os_error()`.
- Preserve any fallback behavior present in the C code for platforms where `F_DUPFD_CLOEXEC` may not behave as required, but keep it local to this module.
- Ensure close-on-exec flag handling remains explicit and does not rely on implicit ownership wrappers.
- Audit unsafe blocks so they contain only the syscall invocation and immediate result extraction.

### Phase 3: Tests for Behavioral Equivalence

- Add unit tests covering successful descriptor duplication.
- Add tests for invalid input descriptors and propagation of OS errors.
- Add tests distinguishing ordinary duplication from close-on-exec duplication, including verification of descriptor flags when supported on the target platform.
- Keep tests local to the migrated module and use standard test patterns with temporary files or pipes from the standard library.

### Phase 4: Integration and Cleanup

- Wire the new Rust module into the crate’s existing module tree.
- Remove any placeholder stubs used during migration.
- Confirm the implementation builds cleanly with `cargo test`.
- Perform a final pass to ensure the module scope remains limited to the original C file and functions, with no added utility surface beyond what migration requires.