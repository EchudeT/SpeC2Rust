# Implementation Plan: module_gnu_dup2.c_25

## Summary

Port `gnu/dup2.c` to a focused Rust module that preserves the existing descriptor-duplication behavior and platform-conditional logic without adding new surface area. The Rust implementation should center on safe wrappers where possible, with small, well-contained OS interaction points for file-descriptor duplication and validation.

The migration should keep the original function boundaries conceptually intact:

- `dup2_nothrow`
- `ms_windows_dup2`
- `klibc_dup2dirfd`
- `rpl_dup2`

Technical approach:

- Use Rust standard library types for error propagation and platform gating.
- Use direct OS descriptor/handle types from `std::os::*` where available.
- Use minimal libc-level calls only where Rust std does not expose equivalent `dup2` behavior.
- Represent C integer return/error conventions as `Result<_, std::io::Error>` internally, converting to C-compatible integer conventions only if required by surrounding crate APIs.
- Preserve platform-specific branches with `cfg(unix)` / `cfg(windows)` and avoid introducing broader abstraction layers.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.75+

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates only if required by missing std coverage:

- `libc` for Unix `dup2`, `fcntl`, `close`, and raw file descriptor constants/functions.
- `windows-sys` only if Windows descriptor/handle duplication cannot be expressed adequately through std alone.

Dependency guidance:

- Prefer `libc` for Unix system-call parity because the source module is syscall-oriented.
- Add `windows-sys` only if the Windows-specific function body is implemented on this branch and cannot remain conditionally stubbed behind existing project behavior.

### Testing

- `cargo test`

Testing scope:

- Unit tests for success/error propagation on supported platforms.
- Platform-gated tests for:
  - duplicating one valid descriptor into another target descriptor
  - handling identical source/target descriptors
  - reporting invalid descriptor errors
- Keep tests local to the module or adjacent integration tests; do not add broader harnesses.

### Performance Goals

- Match C implementation characteristics for descriptor duplication paths.
- Keep overhead effectively negligible relative to the underlying OS syscall.
- Avoid heap allocation in normal execution.
- Avoid extra descriptor opens/closes beyond what the original logic requires.

## Module Mapping

### Source-to-Rust File Mapping

- `gnu/dup2.c` -> `src/module_gnu_dup2.rs`

If the crate already uses a mirroring layout for GNU ports, use the existing file placement convention, but keep the implementation as a single Rust module corresponding directly to this C file.

### Function Mapping

- `dup2_nothrow` -> `fn dup2_nothrow(...) -> std::io::Result<...>`
  - Internal helper for syscall-level duplication without panic-oriented behavior.
  - Encapsulates raw OS call and errno/GetLastError conversion.

- `ms_windows_dup2` -> `#[cfg(windows)] fn ms_windows_dup2(...) -> std::io::Result<...>`
  - Windows-only helper preserving the original conditional behavior.
  - Isolated to prevent Unix code paths from carrying Windows-specific types.

- `klibc_dup2dirfd` -> `#[cfg(unix)] fn klibc_dup2dirfd(...) -> std::io::Result<...>`
  - Unix-only helper for the special directory-fd-related path present in the C source.
  - Keep semantics narrow and aligned with the original branch conditions.

- `rpl_dup2` -> `pub(crate) fn rpl_dup2(...) -> std::io::Result<...>` or project-equivalent visibility
  - Main replacement entry point.
  - Coordinates validation, same-fd handling, and platform/helper dispatch.

### API Shape Notes

- C `int` file descriptors map to `std::os::fd::RawFd` on Unix.
- On Windows, use `std::os::windows::io::RawHandle` or CRT-style raw descriptor type only if the original logic clearly operates on CRT file descriptors rather than Win32 handles.
- Keep the external signature aligned with surrounding crate conventions; if the rest of the Rust port uses `i32` descriptors directly, maintain that instead of introducing wrapper types.

## Data Model

This module has no named C structs; the analysis lists only `anonymous`.

### Data Structure Mapping

- Anonymous C-local state -> Rust local variables and small helper functions
- C integer descriptors/return codes -> Rust primitive integers and OS raw descriptor types
- `errno` / Windows last-error state -> `std::io::Error`

### Mapping Table

| C Concept | Rust Mapping | Notes |
|---|---|---|
| file descriptor `int` | `RawFd` / `i32` | Prefer `RawFd` on Unix-facing internals |
| Windows fd/handle state | `RawHandle` or platform integer type | Choose the narrowest type consistent with actual implementation path |
| return `-1` + `errno` | `Result<T, io::Error>` | Convert at module boundaries if caller expects C-style integers |
| anonymous temporary state | local bindings | No dedicated struct needed |

### Memory Management

- No owned heap-backed data structures are expected.
- Raw descriptors are non-owning in helper parameters unless explicitly duplicated.
- Be careful not to create Rust-owned `File` objects from borrowed raw descriptors unless ownership transfer is unquestionably correct.
- Any temporary wrapper around raw descriptors must avoid double-close; prefer direct syscall use over RAII wrappers when ownership is ambiguous.

### Error Handling

- Convert OS failures immediately into `std::io::Error::last_os_error()`.
- Preserve invalid-descriptor and same-descriptor behavior in the same order as the C implementation.
- Do not suppress close/dup failures unless the original helper specifically models a no-throw path; even then, convert to non-panicking `Result` rather than ignoring status internally.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Platform-Specific Signatures

Goals:

- Create the Rust module corresponding to `gnu/dup2.c`.
- Define the Rust signatures for:
  - `dup2_nothrow`
  - `ms_windows_dup2`
  - `klibc_dup2dirfd`
  - `rpl_dup2`
- Add `cfg(unix)` / `cfg(windows)` gates matching the original conditional structure.
- Decide the exact descriptor types used by this crate on Unix and Windows.

Deliverables:

- `src/module_gnu_dup2.rs` added.
- Function stubs with final intended signatures.
- Imports limited to `std` plus `libc` and/or `windows-sys` only where required.

Notes:

- Keep this phase focused on signature correctness and compile-time platform separation.
- Do not introduce extra abstraction modules.

## Phase 2: Port Core Duplication Logic

Goals:

- Implement `dup2_nothrow` as the lowest-level duplication helper.
- Implement `rpl_dup2` using the original migration order and branch behavior from the C source.
- Preserve edge cases such as:
  - source and target descriptor equality
  - invalid descriptor handling
  - target replacement semantics

Implementation focus:

- Use direct OS calls for exact `dup2` semantics where std lacks an equivalent.
- Keep helper boundaries close to the original C functions rather than merging behavior.
- Use `Result` internally and convert only if required by the surrounding module interface.

Validation:

- Module compiles on the primary target platform for this branch.
- Basic unit tests cover successful duplication and invalid input errors.

## Phase 3: Port Conditional Helper Paths

Goals:

- Implement `klibc_dup2dirfd` for Unix if the original C path is active in the source variant.
- Implement `ms_windows_dup2` for Windows if the source includes a meaningful Windows-specific branch.
- Ensure `rpl_dup2` dispatches to these helpers exactly where the C code does.

Implementation focus:

- Keep helper behavior narrow; no generalized descriptor utility layer.
- Mirror original fallback ordering and error propagation.
- Confirm no ownership mistakes around raw descriptors/handles.

Validation:

- Platform-gated tests compile and pass where supported.
- Non-target platform code remains excluded by `cfg` without dead code expansion.

## Phase 4: Final Semantics Review and Test Completion

Goals:

- Compare Rust control flow and return/error behavior against the original C module.
- Add remaining targeted tests for:
  - same-descriptor no-op semantics
  - replacement of an already-open target descriptor
  - platform-specific failure cases that are observable through return values
- Remove any unused helper code introduced during porting.

Deliverables:

- Finalized `plan`-aligned Rust implementation with concise module-local tests.
- Clean error handling based on `std::io::Error`.
- No extra public API beyond the mapped replacement function(s).

## Notes and Constraints

- Prefer direct migration of existing file/function responsibilities over redesign.
- Do not introduce wrapper types unless necessary to express platform descriptor types safely.
- Do not convert descriptor ownership into `File`/`OwnedFd` abstractions unless the original logic clearly transfers ownership.
- Keep the module limited to descriptor duplication semantics already present in `gnu/dup2.c`.