# Implementation Plan: module_gnu_rlimit_08

## Summary

This module ports the C implementation in `gnu/getdtablesize.c` to Rust on branch `014-module_gnu_rlimit_08-rust-port`. The scope is limited to migrating the existing `getdtablesize` functionality and preserving its behavior as a small system-oriented utility.

The Rust implementation should provide an equivalent function that retrieves the process file descriptor table size using the platform facility that corresponds to the original C logic. The preferred approach is to keep the implementation minimal and close to the source behavior: use Rust’s standard library where possible, and use a narrow libc-level system call binding only where the standard library does not expose the required resource limit query.

The port should remain a focused translation of the existing file and function, without introducing broader abstractions or unrelated supporting layers.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate, only for `getrlimit` and related constants/types if required by the source logic
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match C module behavior with negligible overhead
  - Single syscall-level lookup or equivalent constant-time operation
  - No unnecessary allocations
  - No persistent state or caching unless directly required by the original logic

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/getdtablesize.c` | `src/module_gnu_rlimit_08.rs` or `src/gnu/getdtablesize.rs` | Port the existing implementation directly, keeping function scope narrow and behavior aligned with the C source |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `getdtablesize` | `pub fn getdtablesize() -> usize` or `pub fn getdtablesize() -> libc::c_int` | Final return type should be selected to preserve source semantics as closely as possible; prefer a signed integer type if the C implementation can report fallback/error-style values |

## Data Model

The source analysis identifies only an anonymous data structure, which in this module is most likely a temporary system struct used for resource-limit retrieval.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous system struct usage | `libc::rlimit` | Use the platform-provided struct via `libc` rather than defining a custom Rust struct |

### Memory Management

- No heap allocation is expected.
- Any resource-limit structure should be stack-allocated as a local variable.
- Unsafe code, if needed for libc calls, must be tightly scoped around the syscall interaction only.

### Error Handling

- Preserve C behavior rather than redesigning the API.
- If the original logic falls back from a failed limit query to a constant or alternate mechanism, keep that fallback in Rust.
- Avoid introducing `Result` in the public API unless the migration target in the Rust project already requires it for consistency.
- Validate integer conversions from `rlim_t` to the chosen Rust return type to avoid silent truncation.

## Implementation Phases

## Phase 1: Inspect and Set Up the Direct Port

- Review `gnu/getdtablesize.c` to confirm:
  - which system call or macro is used
  - fallback behavior on failure
  - exact return type expectations
- Create the Rust target file in the existing crate structure.
- Add only the minimum required dependency (`libc`) if the standard library alone cannot express the original system interaction.
- Define the Rust function signature to mirror the C behavior as closely as practical within the project’s existing conventions.

## Phase 2: Implement Core Logic

- Translate the body of `getdtablesize` directly into Rust.
- Use `libc::getrlimit` and `libc::RLIMIT_NOFILE` if this is what the C file relies on.
- Map the temporary anonymous C structure usage to `libc::rlimit`.
- Keep unsafe blocks minimal and localized.
- Handle boundary cases explicitly:
  - failed syscall
  - infinite or very large limit values
  - conversion from platform limit type to Rust return type

## Phase 3: Integrate and Verify Behavior

- Expose the module through the crate’s existing module tree without adding unrelated structure.
- Add unit tests that verify:
  - the function returns a positive value on supported platforms
  - the return value is stable in shape/type and does not panic
- Where exact values are environment-dependent, assert only broad invariants rather than fixed numbers.
- Run `cargo test` and correct any platform-type mismatches or conversion issues.

## Phase 4: Final Compatibility Review

- Compare the Rust implementation against the C source for:
  - return semantics
  - fallback path preservation
  - integer-width compatibility
- Remove any helper code that is not required for the direct migration.
- Confirm the final module remains limited to the original file/function scope and does not introduce expanded capabilities.