# Implementation Plan: module_gnu_dup2.c_25

## Summary

Port `gnu/dup2.c` into a Rust module that preserves the existing file-descriptor duplication behavior and platform-specific branching without introducing broader abstractions. The Rust implementation should focus on translating the existing function set:

- `dup2_nothrow`
- `ms_windows_dup2`
- `klibc_dup2dirfd`
- `rpl_dup2`

The technical approach is to keep the port close to the C control flow and syscall/error semantics. Use Rust’s standard library where possible, and rely on direct OS bindings only for descriptor-level operations that are not exposed by `std`. Error handling should map C return/error conventions into Rust `io::Result` internally, with any public compatibility layer preserving the original integer return style if required by the surrounding port. Memory ownership is minimal in this module; the main concern is safe handling of raw file descriptors/handles and preserving correct close-on-error behavior.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - `std`
  - `libc` for Unix `dup2`-related raw descriptor operations and errno-compatible OS calls
  - No additional third-party crates unless required elsewhere in the project
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C implementation behavior with no meaningful added overhead beyond Rust error translation
  - Avoid heap allocation in the core duplication paths
  - Keep syscall count aligned with the original implementation logic
  - Preserve platform-conditional fast paths and fallback behavior already present in the C module

## Module Mapping

### Source File Mapping
- `gnu/dup2.c` -> `src/module_gnu_dup2.rs`

### Function Mapping
- `dup2_nothrow` -> `dup2_nothrow`
- `ms_windows_dup2` -> `ms_windows_dup2`
- `klibc_dup2dirfd` -> `klibc_dup2dirfd`
- `rpl_dup2` -> `rpl_dup2`

### Rust Module Shape
The Rust file should contain only the direct port of this C file’s logic:
- platform-gated helper functions for Unix/Windows-specific duplication behavior
- one replacement entrypoint corresponding to `rpl_dup2`
- no new utility modules unless required by existing project layout

## Data Model

This module does not define named C structs; the listed data structure is anonymous. The port should therefore keep data modeling minimal and use primitive OS-facing types.

### Type Mapping
- C anonymous/local state -> Rust local variables and small helper scopes
- C file descriptor integers (`int`) -> `std::os::fd::RawFd` on Unix
- Windows descriptor/handle-compatible integers -> platform-appropriate raw integer type used by the surrounding port, kept private to this module
- C return codes (`int`, `-1` on error) -> internal `std::io::Result<c_int>` or direct `c_int` compatibility returns, depending on the surrounding interface needs
- `errno`-based failure signaling -> `std::io::Error::last_os_error()` internally, with compatibility conversion back to C-style failure where needed

### Ownership and Resource Rules
- Do not wrap borrowed descriptors in owning Rust file types such as `File`, since ownership transfer is not part of `dup2` semantics.
- Use raw descriptor values throughout to avoid unintended close-on-drop behavior.
- Any temporary duplicated descriptor created during fallback logic must be explicitly closed on all paths.
- Preserve original “do not clobber target descriptor incorrectly on failure” behavior during translation.

## Implementation Phases

## Phase 1: Establish File Port Skeleton and Platform Boundaries
- Create `src/module_gnu_dup2.rs`.
- Add the four function placeholders with the same migration names as the C functions.
- Introduce `cfg(unix)` / `cfg(windows)` sections matching the original conditional behavior.
- Define the minimal imports:
  - `std::io`
  - raw descriptor types from `std::os::fd` on Unix
  - `libc` items required for low-level duplication and close operations
- Decide the exact Rust signature style based on the surrounding project:
  - prefer raw-descriptor integer parameters for fidelity
  - keep return values compatible with the original module contract

## Phase 2: Port Core Descriptor Duplication Logic
- Translate `dup2_nothrow` first as the lowest-level helper.
- Port `rpl_dup2` next, keeping branching and error propagation close to the C implementation.
- For Unix paths:
  - call the underlying OS duplication primitive through `libc`
  - preserve handling for identical source/target descriptors
  - preserve errno-driven failure cases
- Avoid converting descriptors into owned Rust objects.
- Keep unsafe blocks tightly scoped to syscall invocations and document each one with the OS contract being relied on.

## Phase 3: Port Platform-Specific Fallbacks
- Translate `ms_windows_dup2` under `cfg(windows)` only.
- Translate `klibc_dup2dirfd` under the relevant non-Windows configuration, preserving its specialized fallback behavior.
- Keep these functions private unless the existing project interface requires exposure.
- Ensure the replacement entrypoint dispatches to these helpers in the same order and under the same conditions as the C source.

## Phase 4: Validation and Behavior-Focused Tests
- Add unit tests for descriptor duplication behavior that can run under `cargo test`.
- Cover only behaviors evidenced by the C module’s responsibilities:
  - successful duplication to a different descriptor
  - same-source-and-target handling
  - invalid descriptor error propagation
  - target replacement semantics when the destination is already open
- Gate platform-specific tests with `cfg` attributes.
- Validate that no temporary descriptors leak on failure paths by explicitly closing test descriptors and checking expected outcomes.