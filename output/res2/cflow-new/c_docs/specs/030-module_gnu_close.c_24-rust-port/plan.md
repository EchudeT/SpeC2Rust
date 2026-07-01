# Implementation Plan: module_gnu_close.c_24

## Summary

Port `gnu/close.c` into a focused Rust module that preserves the existing close-path behavior of the C implementation without adding broader I/O abstractions. The Rust implementation should migrate the two exported functions, `close_nothrow` and `rpl_close`, into a small module centered on file-descriptor closing semantics and errno-aware error handling.

The implementation approach should stay close to the original control flow:

- represent raw file descriptors with Rust’s platform fd types,
- invoke the underlying close operation with minimal wrapping,
- preserve the distinction between a non-throwing close helper and the replacement close entry point,
- map OS errors into `std::io::Result` or a narrowly scoped internal result convention,
- avoid expanding the module into generic resource-management helpers.

Because this module is a thin systems boundary, the Rust port will likely require a small amount of `unsafe` code around the actual close syscall or libc binding. That unsafe scope should be kept local to the migrated functions and documented by invariant.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.75+

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependency only if direct POSIX close semantics are required beyond `std` exposure:

- `libc` — for `close`, errno-interacting constants/types, and Unix fd interop when the standard library is insufficient

Do not introduce additional crates unless the surrounding project already mandates them.

### Testing

- `cargo test`

Testing should be module-focused and limited to validating migrated close behavior, especially:

- successful close on a valid descriptor
- failure path on invalid/already-closed descriptor
- behavior distinction between `close_nothrow` and `rpl_close`

Unix-only tests may be conditionally compiled where raw fd handling is required.

### Performance Goals

- Match C-module behavior with negligible wrapper overhead
- No extra allocations on the normal path
- Single syscall on the close path where possible
- Error mapping should remain constant-time and stack-only

## Module Mapping

### Source File Mapping

- `gnu/close.c` → `src/gnu/close.rs`

If the project already exposes GNU-ported modules through a parent module file, wire it in minimally:

- `src/gnu/mod.rs` → add `pub mod close;`

### Function Mapping

- `close_nothrow` → `pub(crate)` or private Rust function in `src/gnu/close.rs`
- `rpl_close` → public module function in `src/gnu/close.rs` if it is referenced outside the module cluster; otherwise keep crate-visible

The exact visibility should follow existing call sites in the Rust port and should not be broadened speculatively.

### API Shape

Prefer direct function migration over new wrapper types:

- `close_nothrow(fd: RawFd) -> ...`
- `rpl_close(fd: RawFd) -> ...`

Return type should be selected to reflect existing project conventions:

- preferred: `std::io::Result<()>`
- if exact C-style status propagation is required by surrounding migrated code: `Result<c_int, io::Error>` or `c_int` plus explicit errno handling

Choose one convention and keep it consistent with adjacent migrated modules.

## Data Model

This module does not define standalone C structs in the provided input.

### C to Rust Type Mapping

- C file descriptor parameter (`int fd`) → `std::os::fd::RawFd`
- C integer status return (`int`) → `i32` or `std::os::raw::c_int` internally, with external Rust API preferably normalized to `std::io::Result<()>`
- `errno`-based failure reporting → `std::io::Error` derived from `last_os_error()`

### Memory and Ownership Notes

- No heap-backed data model is needed
- File descriptors are borrowed as raw integers for the duration of the call only
- The function must not assume ownership beyond attempting to close the supplied descriptor
- Unsafe interaction with the OS must not outlive the call boundary

## Implementation Phases

## Phase 1: Establish module skeleton and function signatures

### Goals

Create the Rust file layout and define the migrated close API surface with signatures matching expected call patterns.

### Tasks

- Add `src/gnu/close.rs`
- Register the module in `src/gnu/mod.rs` only if the parent module already exists in the port
- Define Rust signatures for:
  - `close_nothrow`
  - `rpl_close`
- Select the narrowest visibility that matches actual use
- Choose the result convention based on existing project patterns, preferring `std::io::Result<()>` unless the surrounding port requires integer status compatibility

### Notes

This phase should not introduce helper frameworks or generalized fd abstractions. Keep all design centered on the two migrated functions.

## Phase 2: Port syscall and error-handling logic

### Goals

Implement the actual close behavior with localized unsafe code and explicit OS error mapping.

### Tasks

- Port the body of `close_nothrow` first as the low-level helper
- Implement the underlying close call using:
  - `libc::close(fd)` on Unix, if needed for faithful behavior
- After syscall return:
  - treat success as immediate completion
  - capture OS error on failure via `std::io::Error::last_os_error()`
- Port `rpl_close` on top of the helper, preserving the original distinction in how close errors are handled or filtered
- Keep errno-sensitive logic adjacent to the syscall boundary so behavior remains easy to compare against C

### Error-Handling Rules

- Do not panic for OS-level close failures
- Do not allocate custom error objects
- Keep retry/suppression behavior limited strictly to what existed in `gnu/close.c`
- Document any assumptions around retryable interruption or ignored close errors only if the original logic requires them

## Phase 3: Integrate with callers and align return semantics

### Goals

Ensure the migrated module fits the existing Rust branch without widening responsibilities.

### Tasks

- Update call sites that previously expected the C replacement close function
- Normalize conversions between:
  - raw integer status and Rust result types, if required
  - crate-local error handling conventions and `std::io::Error`
- Confirm no caller relies on implicit errno mutation outside the function boundary; if such reliance exists in the branch, keep the interface closer to C for compatibility
- Remove any temporary placeholders introduced during scaffolding

### Notes

This phase is strictly about compatibility with existing migrated code, not about introducing safer ownership wrappers such as `OwnedFd`.

## Phase 4: Add focused tests and complete validation

### Goals

Verify parity of the migrated close behavior for success and failure paths.

### Tasks

- Add unit tests in `src/gnu/close.rs` or module-adjacent tests
- Cover:
  - closing a valid descriptor created during the test
  - attempting to close an invalid descriptor
  - behavior after double-close when representable in a deterministic way
  - any special handling difference between `close_nothrow` and `rpl_close`
- Use conditional compilation for Unix-specific raw-fd tests
- Run `cargo test` and fix signature or error-mapping mismatches

### Acceptance Criteria

- The module compiles cleanly on the target Unix environment
- Tests validate expected close success/failure behavior
- The Rust implementation remains a thin migration of `gnu/close.c` with no extra abstractions or unrelated facilities