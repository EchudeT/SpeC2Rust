# Implementation Plan: module_gnu_close.c_24

## Summary

This module ports the behavior of `gnu/close.c` into Rust, covering the two exported functions `close_nothrow` and `rpl_close`. The implementation should stay narrowly aligned with the original file’s responsibility: invoking file-descriptor close operations while preserving the original module’s error-handling intent and replacement-layer behavior.

The Rust approach should use thin wrappers around OS file-descriptor closing primitives, keeping ownership rules explicit and avoiding any abstraction that would change call semantics. Since the source module is C and operates on raw file descriptors, the Rust port should likewise work with raw Unix file descriptors and return standard Rust error types derived from OS errors. Any logic that in C suppresses or normalizes close-time failures should be migrated directly into function-level Rust code rather than spread into additional helper subsystems.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `std::os::fd` for raw file descriptor types on Unix
  - `libc` crate only if direct `close(2)` interop is required to preserve exact low-level behavior not exposed cleanly through `std`
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time close-path behavior equivalent to the C implementation
  - Avoid heap allocation
  - Keep wrapper overhead negligible relative to the underlying syscall
  - Preserve direct OS error propagation or suppression behavior required by the original functions

## Module Mapping

### Source to Destination

- `gnu/close.c` -> `src/module_gnu_close.rs`

### Function Mapping

- `close_nothrow` -> `pub(crate) fn close_nothrow(fd: RawFd) -> Result<(), std::io::Error>` or `std::io::Result<()>`
- `rpl_close` -> `pub(crate) fn rpl_close(fd: RawFd) -> Result<(), std::io::Error>` or `std::io::Result<()>`

Final signature choice should be based on the surrounding ported codebase conventions, but both functions should remain raw-file-descriptor-based and should not take ownership via `File` or `OwnedFd` unless the original semantics clearly transfer ownership. For a direct C migration, `RawFd` is the safest match.

## Data Model

This module does not define dedicated C structs or persistent state.

### Data Mapping

- C `int fd` -> Rust `std::os::fd::RawFd`
- C errno-based failure signaling -> Rust `std::io::Error`
- C integer success/failure return convention -> Rust `Result<(), std::io::Error>`
  - If project consistency requires C-like return values, an internal conversion layer may be used, but the implementation should still rely on `std::io::Error::last_os_error()` for error capture.

No new Rust structs or enums are required for this port.

## Implementation Phases

### Phase 1: Establish module skeleton and syscall strategy

- Create `src/module_gnu_close.rs`.
- Add the two Rust function stubs matching the C functions:
  - `close_nothrow`
  - `rpl_close`
- Decide the syscall access path:
  - Prefer standard library types for descriptor representation
  - Use `libc::close` only if needed for exact direct close behavior
- Document the safety boundary for any `unsafe` syscall invocation:
  - Raw file descriptor is passed directly to OS
  - No ownership transfer is assumed by the wrapper itself
  - Error state must be captured immediately after syscall failure

### Phase 2: Port function logic faithfully

- Migrate `close_nothrow` first:
  - Implement the close attempt on a raw file descriptor
  - Preserve the intended “no throw” behavior by translating C’s failure handling into Rust `Result` behavior without panicking
  - Ensure no destructor-based automatic retry or alternate ownership behavior is introduced
- Migrate `rpl_close` second:
  - Port the replacement logic exactly as written in the C source
  - Preserve any special handling around retry, ignored errors, or descriptor-state assumptions present in the original function
  - Keep the logic local to this module rather than introducing shared utilities unless already required by existing project structure

### Phase 3: Error-handling and Unix semantics validation

- Verify that OS errors are captured exactly at the failing syscall boundary.
- Confirm behavior for:
  - valid open descriptor
  - invalid descriptor
  - repeated close on same descriptor, if meaningful to original logic
- Review differences between C errno conventions and Rust `Result` usage to ensure no behavior loss:
  - no panic paths
  - no accidental swallowing of errors beyond what the C implementation already does
- Keep `unsafe` usage minimal and confined to syscall invocation.

### Phase 4: Add focused tests and integrate

- Add unit tests in the module or corresponding `tests` coverage using temporary file descriptors.
- Test only behavior evidenced by the migrated functions:
  - successful close of a valid descriptor
  - failure on invalid descriptor
  - any special-case behavior required by `rpl_close`
- Use standard library facilities to create descriptors where possible; only use low-level calls where necessary to exercise raw-fd behavior.
- Integrate the module into the crate’s existing `mod` tree without adding unrelated supporting layers.