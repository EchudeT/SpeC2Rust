# Implementation Plan

## Summary

This module covers the root command-path logic centered in `cat.c::main` together with the local file-control helper from `fcntl.c::klibc_fcntl`. The Rust port should preserve the existing control flow and operating-system interaction shape rather than redesign behavior.

The implementation approach is a direct migration into a small Rust binary entry path, keeping:
- argument handling in the Rust `main` path,
- file-descriptor and flag-oriented operations expressed through safe standard-library APIs where they are sufficient,
- narrowly scoped Unix-specific calls only where the C behavior depends on `fcntl` semantics that are not exposed by `std`.

The plan should avoid introducing new abstraction layers beyond what is needed to mirror the current files and functions. Error handling should convert implicit C error signaling into explicit Rust `Result` propagation internally, with the binary exit behavior preserved at the top level.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate only if required to reproduce `klibc_fcntl` behavior that cannot be expressed via `std::os::unix`
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain command startup and file-processing overhead close to the C implementation
  - Avoid unnecessary heap allocation in argument and I/O paths
  - Preserve direct descriptor-based operations where needed instead of adding buffering layers unrelated to the original code
  - Keep syscall count comparable to the C path for file-control operations

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `cat.c` | `main` | `src/main.rs::main` with small internal helpers as needed | Keep binary entry semantics in the main crate file; only extract minimal local functions if required for readability during migration |
| `fcntl.c` | `klibc_fcntl` | `src/fcntl.rs::klibc_fcntl` or private module inside `src/main.rs` | Prefer a dedicated module file only because there is a direct source-file counterpart in C |

### Proposed Rust file layout

```text
src/
  main.rs
  fcntl.rs
```

This keeps the mapping close to the original two-file C module and avoids adding extra module layers.

## Data Model

The analysis identifies only anonymous C data structures. Since no named layout contract is indicated, the Rust port should minimize explicit struct recreation and instead translate each use site to the narrowest native Rust representation.

| C Data Structure | Rust Mapping | Rationale |
|---|---|---|
| anonymous | Local variables / tuples / small private struct only if a grouped state is required | Anonymous C structs often exist for local grouping; do not create public types without a demonstrated need |
| anonymous | Primitive integer types (`i32`, `u32`, `libc::c_int`) for flag and descriptor handling | `fcntl`-related state is typically integer-based and should remain close to OS ABI expectations |
| anonymous | Enum only if the C code expresses a closed local mode set during migration | Prefer enums for internal branch clarity, but only when directly replacing repeated constant-based control flow |

### C-to-Rust representation rules

- **File descriptors**: map to `std::os::fd::RawFd` for Unix-specific operations.
- **Open/status flags**: keep as integer bitflags using `libc::c_int` if direct `fcntl` interaction is necessary.
- **Optional pointers / absent values**: replace with `Option<T>`.
- **C error returns (`-1`, `errno`)**: convert to `std::io::Result<T>`.
- **Mutable shared buffers**: use owned Rust buffers (`Vec<u8>`) or stack arrays only where the original function requires them; do not introduce global mutable state.

### Memory management and error handling

- Eliminate manual lifetime and ownership concerns by keeping descriptor ownership explicit:
  - borrowed descriptors stay as `RawFd`,
  - owned files use `std::fs::File` where possible.
- Any path that mirrors `fcntl` should clearly distinguish:
  - operations that only inspect flags,
  - operations that mutate descriptor state.
- All internal failures should propagate as `io::Error`; only `main` should translate them into process-visible exit behavior.
- Avoid unsafe code unless direct `libc::fcntl` invocation is required. If unsafe is unavoidable, isolate it inside `fcntl.rs` with a narrow, documented boundary.

## Implementation Phases

## Phase 1: Create crate skeleton and port the entry path

### Goals
- Establish the Rust binary structure corresponding to `cat.c`.
- Port `main` with argument collection and top-level error handling.
- Keep behavior-oriented control flow close to the C source.

### Tasks
- Create `src/main.rs`.
- Translate `cat.c::main` into Rust in a mostly linear form.
- Use `std::env::args_os()` if the C path is filename-oriented and should preserve non-UTF-8 argument handling.
- Introduce only minimal private helper functions if needed to avoid an oversized `main`.
- Define a top-level internal result-returning function, with `main()` responsible for exit code mapping.

### Deliverables
- Compiling Rust binary entry point matching the C file/function mapping.
- No additional architectural layers beyond what is needed for direct migration.

## Phase 2: Port `klibc_fcntl` with Unix-specific descriptor handling

### Goals
- Recreate the behavior of `fcntl.c::klibc_fcntl`.
- Keep the implementation boundary small and OS-facing details isolated.

### Tasks
- Create `src/fcntl.rs`.
- Translate `klibc_fcntl` directly.
- Prefer `std` APIs if the exact operation is available.
- If not, use `libc::fcntl` with:
  - `RawFd` inputs,
  - exact command/flag integer types,
  - immediate conversion of return values into `io::Result`.
- Document any unsafe block with the specific syscall contract being relied on.

### Deliverables
- Rust equivalent of `klibc_fcntl`.
- Clean call integration from `main.rs`.

## Phase 3: Integrate data and error model adjustments

### Goals
- Remove remaining C-style sentinel handling from the migrated code.
- Finalize type mappings for anonymous structures and flag values.

### Tasks
- Replace ad hoc integer error checks with `Result` returns.
- Collapse anonymous C structure usage into local Rust variables or a private struct only where repeated grouped state exists.
- Verify descriptor ownership is not accidentally transferred or double-closed.
- Ensure all integer conversions around `fcntl` commands and flags are explicit.

### Deliverables
- Internally idiomatic but behavior-preserving Rust flow.
- Clear ownership and error propagation boundaries.

## Phase 4: Validation and regression-focused tests

### Goals
- Confirm the Rust port compiles, runs, and preserves expected command behavior for the migrated paths.

### Tasks
- Add unit tests for any isolated `klibc_fcntl` translation logic that can be tested without overbuilding infrastructure.
- Add focused integration-style tests where practical for the binary argument and file path flow.
- Run `cargo test`.
- Manually review edge cases around:
  - invalid file descriptors,
  - failed file-control operations,
  - exit-status propagation from `main`.

### Deliverables
- Test coverage for the migrated functions proportionate to their size and OS interaction.
- Stable build on branch `004-main_root_stat_03-rust-port`.