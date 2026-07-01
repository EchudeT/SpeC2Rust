# Implementation Plan

## Summary
Port `fcntl.c` into a Rust module that preserves the existing replacement logic for file-descriptor duplication paths represented by `rpl_fcntl_DUPFD` and `rpl_fcntl_DUPFD_CLOEXEC`. The Rust implementation should stay narrowly scoped to these existing functions, using direct OS-level file descriptor operations and Rust error propagation around `std::io::Error`.

The implementation approach is to:
- map the C entry points to Rust functions in a single corresponding module,
- represent raw file descriptors with standard platform types,
- preserve return-value and error behavior expected from the original logic,
- keep ownership rules explicit so duplicated descriptors are returned as raw descriptors without introducing broader abstractions not present in the source module.

## Technical Context

### Language/Version
- Rust 1.75 or newer

### Primary Dependencies
- Rust standard library
- `libc` crate for `fcntl` constants, syscall bindings, and platform file descriptor types where `std` is insufficient

### Testing
- `cargo test`

### Performance Goals
- Maintain syscall-level performance characteristics equivalent to the C implementation
- Avoid additional heap allocation
- Keep wrappers thin so the port adds negligible overhead beyond error conversion and argument translation

## Module Mapping

### C to Rust File Mapping
- `fcntl.c` -> `src/main_root_rpl_fcntl_14.rs`

### Function Mapping
- `rpl_fcntl_DUPFD` -> `pub(crate) fn rpl_fcntl_dupfd(fd: RawFd, target: libc::c_int) -> std::io::Result<RawFd>`
- `rpl_fcntl_DUPFD_CLOEXEC` -> `pub(crate) fn rpl_fcntl_dupfd_cloexec(fd: RawFd, target: libc::c_int) -> std::io::Result<RawFd>`

### Integration Mapping
- Expose the Rust module from the crate root or the existing main-cluster module tree only as needed for current call sites
- Do not introduce extra helper modules unless a small private helper is required to remove duplication between the two migrated functions

## Data Model

### Data Structure Mapping
- anonymous C data usage -> no dedicated Rust struct required
- C file descriptor integers -> `std::os::fd::RawFd`
- C integer arguments and return intermediates -> `libc::c_int`

### Memory Management
- No heap-managed state is needed
- Returned duplicated file descriptors remain raw integer descriptors; ownership transfer must be documented at the function boundary
- Any temporary values remain stack-allocated

### Error Handling
- C errno-based failures -> `std::io::Result<RawFd>`
- OS errors should be captured with `std::io::Error::last_os_error()`
- If callers require C-like sentinel handling, contain that translation at the Rust/CALLSITE boundary rather than inside unrelated module layers

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port
- Create `src/main_root_rpl_fcntl_14.rs`
- Add the two Rust function signatures corresponding to the C functions
- Wire the module into the existing crate layout on branch `015-main_root_rpl_fcntl_14-rust-port`
- Establish any minimal imports for `RawFd`, `std::io`, and `libc`

### Phase 2: Syscall Logic Migration
- Port the logic of `rpl_fcntl_DUPFD`
- Port the logic of `rpl_fcntl_DUPFD_CLOEXEC`
- Preserve the original control flow around `fcntl` command selection, descriptor duplication semantics, and close-on-exec handling
- Keep unsafe usage tightly scoped to the direct syscall invocations
- Verify that invalid inputs and syscall failures map cleanly to `std::io::Error`

### Phase 3: Behavior Validation
- Add focused unit or integration tests covering:
  - successful duplication to a descriptor at or above the requested minimum,
  - failure on invalid source descriptors,
  - `CLOEXEC` behavior where supported,
  - fallback/error behavior matching the original implementation expectations
- Run `cargo test`
- Perform a final review to ensure no extra abstractions or unsupported capability expansions were introduced