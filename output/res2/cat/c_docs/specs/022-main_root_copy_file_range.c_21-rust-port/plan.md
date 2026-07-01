# Implementation Plan

## Summary
This module ports the single C source file `copy-file-range.c` and its exported function `copy_file_range` into Rust with behavior kept as close as practical to the current Unix-oriented implementation.

The Rust implementation should:
- preserve the existing file-to-file copy path centered on the kernel `copy_file_range` operation,
- keep the scope limited to migrating the current function and its immediate helper logic from the C file,
- use safe Rust where possible, with narrowly contained `unsafe` only if direct libc/syscall access is required,
- translate C-style return/error handling into `Result`-based Rust APIs internally, with a thin compatibility-facing layer if the surrounding port requires integer/status-style results.

The technical approach is to place the Rust port in a focused module under the main executable crate, map file descriptors and offsets to Rust/Unix types, and preserve current control flow around partial copies, retries, and syscall error propagation without introducing broader abstractions.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate, only for Unix `copy_file_range` access and related low-level file-descriptor types if the standard library is insufficient
- **Testing**: `cargo test`
- **Performance Goals**:
  - remain syscall-based and avoid unnecessary userspace buffering,
  - preserve large-file handling and partial-copy behavior,
  - keep overhead near the C implementation by operating directly on file descriptors and offsets,
  - avoid extra allocations on the copy path.

## Module Mapping

### C to Rust File Mapping
- `copy-file-range.c` -> `src/copy_file_range.rs`

### Function Mapping
- `copy_file_range` -> `copy_file_range`

### Integration Mapping
- Expose the Rust function from the crate module tree only where the existing main-cluster port needs it.
- Do not split this migration into additional helper modules unless a small private helper is required to mirror existing control flow from the C file.

## Data Model

This module analysis identifies only an anonymous data structure, so the migration should avoid inventing persistent models unless the C file requires temporary structured state.

### Data-structure Mapping
- anonymous temporary C state -> local Rust variables or a private Rust struct scoped to `src/copy_file_range.rs` only if needed for readability and direct migration of state

### Type Mapping Guidance
- C file descriptor integers -> `std::os::fd::RawFd`
- C size/count types -> `usize` or `u64`, depending on the original semantic role
- C offsets (`off_t`-style) -> `libc::off_t` or a clearly bounded Rust integer type matching platform behavior
- C integer status returns -> `std::io::Result<T>` internally, with explicit errno-to-`io::Error` conversion
- nullable pointer offset parameters -> `Option<&mut libc::off_t>` or tightly scoped raw-pointer handling inside `unsafe`

### Memory and Error Handling
- No manual heap management is expected; keep all state stack-local.
- Confine `unsafe` to the exact syscall boundary.
- Preserve errno-sensitive behavior by converting OS errors immediately after failed syscall calls.
- Preserve partial progress semantics: if the underlying operation copies fewer bytes than requested, the Rust loop should continue or return consistently with the C logic rather than masking short copies.

## Implementation Phases

### Phase 1: Create the Rust module skeleton
- Add `src/copy_file_range.rs`.
- Define the Rust signature for `copy_file_range` based on the surrounding port’s expected call pattern.
- Add the module declaration and minimal visibility needed for the main cluster.
- Establish imports for Unix file-descriptor and low-level error types.
- Decide whether `libc::copy_file_range` is required on the target branch and keep that dependency limited to this module.

### Phase 2: Port syscall and control flow logic
- Migrate the body of `copy_file_range` from `copy-file-range.c` directly into Rust.
- Preserve the existing loop structure, byte-count updates, offset handling, and termination conditions.
- Implement direct syscall invocation with tightly scoped `unsafe` if needed.
- Convert C error handling into `io::Result`, ensuring immediate capture of OS errors and preserving behavior for interrupted or unsupported operations exactly as present in the C flow.
- Keep helper logic private and in the same file if extraction is necessary to mirror the original function cleanly.

### Phase 3: Integrate with surrounding main-cluster code
- Replace existing references or stubs so callers use the Rust `copy_file_range` implementation.
- Align return-value conventions with the rest of the Rust port without broadening functionality.
- Verify that descriptor ownership remains external to this module and that the function does not close or otherwise manage caller-owned resources.
- Confirm platform gating matches the original module assumptions if the C implementation is Unix-specific.

### Phase 4: Add focused tests and validate edge behavior
- Add unit and/or integration tests under standard Rust test layout targeting only this module’s migrated behavior.
- Cover successful file-to-file copying, short-copy progression, zero-length requests, and representative syscall failure cases that can be exercised in tests.
- Run `cargo test` and adjust the port only for behavioral parity and Rust safety correctness.
- Keep tests local to the migrated function; do not introduce benchmark or infrastructure work beyond what is needed to validate the port.