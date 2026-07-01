# Implementation Plan

## Summary

Port the C module `include/safe-read.c` into a small Rust module that preserves the existing low-level read/write retry behavior of `safe_rw`. The Rust implementation should stay narrowly scoped to the current module responsibility: perform a single safe I/O helper migration without introducing broader I/O abstractions.

The technical approach is to translate the existing retry/error-handling logic into a Rust function built on the standard library, using `std::io` types and explicit handling of interrupted system calls and partial transfers where applicable. Ownership and buffer safety will be enforced by Rust slice types instead of raw pointer arithmetic. The port should keep behavior close to the C implementation, especially around return values, loop structure, and propagation of OS I/O failures.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C helper’s operational behavior without adding allocation overhead.
  - Keep the implementation zero-copy over caller-provided buffers.
  - Preserve efficient retry behavior for interrupted I/O.
  - Avoid unnecessary abstraction layers beyond what is needed to represent the original function safely.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `include/safe-read.c` | `safe_rw` | `src/module_include.rs` -> `safe_rw(...)` | Direct function migration; keep module scope limited to this helper. |

## Data Model

This module does not define persistent C structs in the provided input.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| Raw buffer pointer + length parameters | Borrowed slices such as `&mut [u8]` / `&[u8]` as appropriate | Replaces manual memory management with bounds-checked slice access. |
| Integer return code / byte count | `std::io::Result<usize>` or a narrowly equivalent return type | Prefer standard error propagation while preserving byte-count semantics. |
| File descriptor style I/O handle | Standard library reader/writer trait bound or the narrowest concrete handle needed by surrounding code | Final signature should be chosen to stay closest to current call sites. |

## Implementation Phases

### Phase 1: Inspect and define the Rust signature
- Examine the existing `safe_rw` implementation in `include/safe-read.c`.
- Identify the exact operational contract:
  - whether it is read-only, write-only, or parameterized for both;
  - return-value semantics for success, EOF, partial transfer, and failure;
  - retry conditions such as `EINTR`.
- Define the Rust function signature that most closely matches current usage while remaining idiomatic and safe.
- Create the destination module file `src/module_include.rs` and expose only the migrated function required for this module.

### Phase 2: Port the core loop and error handling
- Translate the C transfer loop into Rust using standard library I/O operations.
- Replace raw pointer movement and manual byte-count tracking with slice indexing/sub-slicing.
- Preserve partial-progress behavior from the C version rather than redesigning semantics.
- Map OS and I/O failures into `std::io::Error` handling.
- Pay specific attention to:
  - interrupted operations (`ErrorKind::Interrupted`);
  - zero-length transfers and termination conditions;
  - avoiding panics in normal error paths.

### Phase 3: Validate behavior with focused tests
- Add unit tests covering the migrated helper’s observable behavior.
- Test retry handling and partial transfer scenarios using standard test doubles or in-memory I/O where sufficient.
- Verify success-path byte counts and failure propagation.
- Confirm no allocation is required for the normal path and that buffer boundaries are respected.

### Phase 4: Integrate and finalize module parity
- Align naming, visibility, and placement with the Rust project’s existing module layout on branch `001-module_include-rust-port`.
- Ensure the Rust function is used in place of the C helper where this branch expects the module port.
- Run `cargo test` and perform a final parity review against the original C function to confirm no extra capabilities were introduced.