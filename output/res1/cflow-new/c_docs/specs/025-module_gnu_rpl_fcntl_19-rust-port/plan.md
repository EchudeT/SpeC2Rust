# Implementation Plan

## Summary

Port `gnu/fcntl.c` into a focused Rust module that preserves the existing file-descriptor duplication behavior implemented by:

- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

The Rust implementation should stay narrow in scope: migrate only the logic needed for these two routines, keeping POSIX-style file descriptor handling and error propagation aligned with the C behavior. The preferred approach is to implement a small Rust module that wraps the relevant `fcntl` operations through stable Rust-compatible system interfaces, using safe Rust where possible and isolating any required `unsafe` syscall interaction to minimal internal helpers.

The plan should preserve:
- integer file descriptor inputs/outputs,
- close-on-exec handling for the `CLOEXEC` variant,
- OS error reporting through standard Rust error types,
- no expansion into broader descriptor-management abstractions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `fcntl` constants and syscall bindings where std does not expose the needed operations directly
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep overhead effectively identical to the C implementation aside from minimal Rust error conversion.
  - Avoid heap allocation.
  - Preserve direct syscall-oriented behavior for descriptor duplication paths.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `gnu/fcntl.c` | `src/module_gnu_rpl_fcntl_19.rs` | Single Rust module containing the migrated descriptor-duplication logic |
| `rpl_fcntl_DUPFD` | `pub(crate)` or internal Rust function with equivalent name adapted to Rust naming conventions | Implement direct duplication with minimum target fd |
| `rpl_fcntl_DUPFD_CLOEXEC` | `pub(crate)` or internal Rust function with equivalent name adapted to Rust naming conventions | Implement duplication with close-on-exec semantics |

If the surrounding crate already uses a module tree, expose this file through the existing `mod` declarations only as needed for current call sites. Do not introduce additional layering beyond one source file unless required by the crate’s existing structure.

## Data Model

This module has no named C structs to migrate.

| C Element | Rust Mapping | Notes |
|---|---|---|
| anonymous / implicit integer fd handling | `std::os::fd::RawFd` (`type RawFd = i32` on Unix) | Use raw file descriptors directly |
| C integer return values | `std::io::Result<RawFd>` internally, or project-required integer-style return wrapper at boundary | Prefer `Result` internally for precise OS error handling |
| `errno`-based failures | `std::io::Error` | Convert syscall failure through `last_os_error()` |

No heap-managed state or owned descriptor wrapper is required unless already mandated by the existing crate interface. Avoid adding new structs or enums when raw descriptors are sufficient.

## Implementation Phases

### Phase 1: Create module skeleton and map syscall surface

- Add `src/module_gnu_rpl_fcntl_19.rs`.
- Introduce the Rust equivalents for:
  - duplicate fd with lower-bound target (`F_DUPFD`)
  - duplicate fd with lower-bound target plus close-on-exec (`F_DUPFD_CLOEXEC`) where supported
- Define the narrow internal interface using:
  - `RawFd` for arguments and return values,
  - `std::io::Result<RawFd>` for internal error handling.
- Keep any `unsafe` usage confined to tiny helpers calling `libc::fcntl`.

### Phase 2: Port function logic and error handling

- Implement the Rust equivalent of `rpl_fcntl_DUPFD`.
  - Call `fcntl` with `F_DUPFD`.
  - Return duplicated fd on success.
  - Translate `-1` return into `std::io::Error::last_os_error()`.
- Implement the Rust equivalent of `rpl_fcntl_DUPFD_CLOEXEC`.
  - Prefer `F_DUPFD_CLOEXEC` when available on the target.
  - If the original C logic includes compatibility behavior for environments lacking native support, mirror only that existing behavior in Rust without adding new fallback strategies beyond what the C file already requires.
- Ensure no descriptor leaks occur in fallback/set-flag paths:
  - if a duplicate is created and a subsequent `FD_CLOEXEC` set operation fails, close the duplicated fd before returning the error.
- Keep function boundaries close to the original C routines rather than redesigning the API.

### Phase 3: Integrate with crate module layout

- Wire the new module into the crate using standard Rust module declarations.
- Update existing call sites that referenced the C-side functionality to use the Rust module.
- Preserve current module boundaries and naming expectations used by the branch; avoid introducing a broader portability layer.

### Phase 4: Add focused tests for descriptor duplication semantics

- Add unit tests or integration-style tests using `cargo test` that validate:
  - successful duplication returns a valid distinct fd,
  - the duplicated fd is at least the requested lower bound,
  - invalid input fd returns an OS error,
  - `CLOEXEC` duplication path applies close-on-exec semantics when supported,
  - cleanup closes all test-created descriptors.
- Keep tests Unix-targeted if the original module is Unix/POSIX-specific.
- Avoid benchmark or concurrency-oriented tests; keep coverage centered on migrated behavior only.