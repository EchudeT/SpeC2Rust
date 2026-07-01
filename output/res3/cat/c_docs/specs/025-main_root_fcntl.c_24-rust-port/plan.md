# Implementation Plan

## Summary

This module ports the `dupfd` logic from `fcntl.c` into Rust for the `cat` project’s `main_cluster`. The Rust implementation should keep the scope limited to the existing file/function behavior and preserve the current low-level file-descriptor-oriented design rather than introducing broader abstractions.

The technical approach is to translate the single C function into a focused Rust module that:
- operates on raw Unix file descriptors,
- uses `std` types where possible,
- performs any required descriptor duplication through direct OS interaction,
- maps C-style error outcomes to Rust’s `Result` with `std::io::Error`,
- keeps ownership rules explicit so duplicated descriptors are not leaked or double-closed.

Because this module is centered on descriptor handling, the main technical risk is matching C behavior around error propagation and descriptor lifetime. The Rust port should therefore prefer minimal wrapping and straightforward control flow.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the available module evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve the C implementation’s constant-time, syscall-bounded behavior
  - Avoid extra allocations
  - Avoid introducing buffering or layered abstractions beyond what is needed to represent file descriptors safely
  - Keep error-path overhead minimal and aligned with direct system call usage

## Module Mapping

- **C source file**: `fcntl.c`
- **Rust target module**: `src/main_root_fcntl.rs` or the closest existing module file matching the project’s current Rust layout
- **C function -> Rust function**:
  - `dupfd` -> `dupfd`

If the branch already has a file layout for `main_cluster` ports, this module should be added to that existing structure rather than creating a new parallel hierarchy.

## Data Model

The analysis lists only an anonymous data structure and no named persistent struct that must be carried over. The expected mapping is therefore minimal.

- **C anonymous data structure** -> **Rust local variables / tuple / inline control-state**
  - Do not introduce a dedicated Rust struct unless the translated logic requires a small internal representation for clarity.
  - Prefer plain local bindings and standard integer types for descriptor values.

### Type Mapping

- `int` file descriptor -> `std::os::fd::RawFd`
- C success/error return pattern -> `Result<RawFd, std::io::Error>` or, if constrained by surrounding module conventions, a thin compatibility signature with internal conversion to/from `io::Error`
- Temporary borrowed descriptor usage -> borrowed raw fd values without ownership transfer
- Owned duplicated descriptor, if retained past function boundary -> explicit ownership handling with standard fd ownership types only if the surrounding code already uses them

### Memory Management and Ownership

- File descriptors are the main owned resource.
- The port must define clearly whether `dupfd` returns:
  - a raw duplicated descriptor whose caller becomes responsible for closing, or
  - an owned standard-library fd wrapper if that matches surrounding code conventions.
- Avoid implicit ownership changes.
- Ensure failed duplication does not create partially-owned state.

## Implementation Phases

### Phase 1: Inspect and map existing C behavior

- Review `fcntl.c` and isolate the exact control flow and return semantics of `dupfd`.
- Determine:
  - required inputs,
  - minimum valid descriptor constraints,
  - error cases and errno-dependent behavior,
  - whether the function is a thin `fcntl`-style wrapper or includes validation logic.
- Check the current Rust branch structure and place the port into the existing `main_cluster` module layout without adding unrelated files.

### Phase 2: Implement the Rust function

- Create the Rust module file corresponding to `fcntl.c`.
- Port `dupfd` with a narrow, low-level implementation.
- Use Rust standard Unix fd types for descriptor representation.
- Preserve the original behavior for:
  - descriptor argument handling,
  - syscall invocation,
  - returned duplicated descriptor,
  - failure propagation.
- Keep unsafe usage, if required for syscall interop, tightly scoped and documented at the call site.
- Do not introduce extra helper layers unless needed to mirror the original function cleanly.

### Phase 3: Integrate error handling and ownership semantics

- Align the Rust function signature with the surrounding ported codebase conventions.
- Convert OS failures into `std::io::Error`.
- Verify that ownership of any returned duplicated descriptor is unambiguous.
- Confirm that no descriptor is closed prematurely and that no success path leaks internal temporaries.

### Phase 4: Add focused tests and finalize module migration

- Add unit tests for the translated `dupfd` behavior using `cargo test`.
- Cover:
  - successful duplication from a valid descriptor,
  - failure on invalid descriptor input,
  - behavior for minimum/new descriptor constraints if present in the C logic.
- Prefer deterministic tests using temporary files or pipes from the standard library.
- Finalize by checking that the module remains limited to the migrated `fcntl.c` scope and does not add unrelated facilities.