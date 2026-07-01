# Implementation Plan

## Summary
This module ports the `fcntl.c` replacement logic for file-descriptor duplication into Rust, limited to the existing behaviors represented by:

- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

The Rust implementation should preserve the current module scope: provide internal helpers that perform descriptor duplication with the same observable error behavior and close-on-exec handling as the C code. The implementation should stay close to the original control flow and system-call usage rather than introducing broader abstractions.

Technical approach:

- Map the C logic into a Rust module dedicated to descriptor-duplication helpers.
- Use Rust’s standard library for file-descriptor types where possible, and use low-level OS bindings only for the required `fcntl`/`dup`-family operations not exposed directly by `std`.
- Represent failures with `std::io::Error`, while keeping integer file-descriptor return values to match the C-style call boundary expected by surrounding code.
- Use small, explicit unsafe blocks around syscall boundaries only, with validation and errno-to-error propagation kept local.

## Technical Context

- **Language/Version:** Rust 1.77+
- **Primary Dependencies:**
  - Rust standard library
  - `libc` crate for `fcntl` constants and syscall access on Unix, if the surrounding project already uses direct POSIX bindings
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain syscall-level performance equivalent to the C implementation
  - Avoid extra allocation and unnecessary descriptor wrapper construction
  - Preserve constant-time control flow around duplication and flag-setting operations
  - Keep overhead limited to minimal Rust error conversion at syscall boundaries

## Module Mapping

### C to Rust File Mapping
- `fcntl.c` -> `src/main_root_rpl_fcntl_14.rs`

### Function Mapping
- `rpl_fcntl_DUPFD` -> `pub(crate) fn rpl_fcntl_dupfd(fd: RawFd, target: RawFd) -> io::Result<RawFd>`
- `rpl_fcntl_DUPFD_CLOEXEC` -> `pub(crate) fn rpl_fcntl_dupfd_cloexec(fd: RawFd, target: RawFd) -> io::Result<RawFd>`

### Scope Notes
- Keep both functions in a single Rust module corresponding directly to the original C file.
- Do not introduce extra public APIs beyond what is needed to replace current call sites.
- If platform-specific branching is required for `F_DUPFD_CLOEXEC` availability or behavior, keep it local to this module with `cfg(unix)` and narrowly scoped helper logic.

## Data Model

### Data-structure Mapping
- `anonymous` -> no dedicated Rust data structure required

### Type Mapping
- C file descriptor integers -> `std::os::fd::RawFd`
- C integer return/error convention -> `std::io::Result<RawFd>`
- `errno`-based failure reporting -> `std::io::Error::last_os_error()`

### Memory and Resource Handling
- No heap-managed state is needed for this module.
- Returned file descriptors remain owned by the caller, matching the C behavior.
- Avoid constructing owning descriptor wrappers such as `OwnedFd` unless needed internally for temporary safety; if used, convert back carefully without changing ownership semantics expected by callers.
- Unsafe code should be limited to direct syscall invocation and constant usage.

## Implementation Phases

### Phase 1: Create module skeleton and syscall boundary
- Add `src/main_root_rpl_fcntl_14.rs`.
- Define the two Rust function signatures using `RawFd` and `io::Result<RawFd>`.
- Add the minimal imports:
  - `std::io`
  - `std::os::fd::RawFd`
  - `libc` items required for `fcntl`-related operations
- Implement a small internal helper for translating negative syscall returns into `io::Result<RawFd>`.
- Keep the file focused only on the functions migrated from `fcntl.c`.

### Phase 2: Port `rpl_fcntl_DUPFD`
- Translate the existing duplication logic as directly as possible.
- Preserve input validation assumptions from the C implementation rather than redesigning the API.
- Ensure the function returns the duplicated descriptor on success and propagates OS errors unchanged on failure.
- Keep ownership rules identical to C: the new descriptor is returned as a raw integer and not auto-closed by Rust.

### Phase 3: Port `rpl_fcntl_DUPFD_CLOEXEC`
- Implement the close-on-exec duplication path using the closest available OS primitive.
- If the original C logic includes fallback behavior when `F_DUPFD_CLOEXEC` is unavailable or unsuitable, reproduce that behavior in Rust within this function or a private helper.
- Ensure any fallback that sets `FD_CLOEXEC` after duplication handles intermediate failure paths without leaking file descriptors.
- Keep all cleanup explicit and local.

### Phase 4: Integrate and test
- Update the calling module(s) to use the Rust replacements from `main_root_rpl_fcntl_14`.
- Add focused unit tests for:
  - successful descriptor duplication
  - duplication with minimum target descriptor
  - close-on-exec duplication behavior
  - expected OS error propagation for invalid descriptors
- Run `cargo test` and verify behavior matches the prior C implementation at the module boundary.