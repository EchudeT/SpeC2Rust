# Implementation Plan

## Summary

Port `xalloc-die.c` into a Rust module that preserves the existing allocation-failure termination behavior used by the `pwd` program. The Rust implementation should keep the logic narrow: provide a single module-level routine corresponding to `xalloc_die`, using Rust’s standard error-reporting and process-termination facilities rather than manual memory cleanup paths.

The implementation should favor direct migration of the current responsibility of the C file:
- emit the allocation failure diagnostic expected by the program,
- terminate execution in a non-recoverable path,
- avoid introducing broader allocation frameworks or reusable abstractions beyond what this single migrated function requires.

Because Rust already enforces memory safety for owned values, the port should not reproduce C-style cleanup mechanics unless they are directly required by the existing call pattern. Error handling should remain explicit and terminal for this routine, matching the C semantics of a fatal out-of-memory handler.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::process`, `std::io`, `std::alloc` only if directly needed by the port)
- **Testing**: `cargo test`
- **Performance Goals**:
  - No meaningful runtime overhead beyond a single diagnostic write and immediate process termination.
  - Preserve constant-time fatal path behavior.
  - Avoid heap allocation in the failure path where practical.

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `xalloc-die.c` | `src/main_root_xalloc_die.rs` or integrated into the crate’s existing main-cluster module layout | Keep the port limited to the one migrated function and place it where current main-cluster support code resides. |
| `xalloc_die` | `pub(crate) fn xalloc_die() -> !` | Rust return type should be `!` because the routine terminates unconditionally. |

## Data Model

This module does not define persistent data structures in the provided input.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| None | None | No struct or enum migration is required for this file. |

## Implementation Phases

### Phase 1: Establish module skeleton and signature mapping
- Create the Rust file for the migrated module within the existing crate structure.
- Add the direct Rust equivalent of `xalloc_die` with signature `fn xalloc_die() -> !`.
- Keep visibility restricted to the current crate unless an existing call site requires broader exposure.
- Document the function as the fatal allocation-failure path migrated from the C module.

### Phase 2: Port fatal reporting and termination behavior
- Translate the C routine’s behavior into Rust using standard-library facilities.
- Write the allocation-failure diagnostic to standard error with minimal formatting complexity.
- Terminate the process immediately using an appropriate non-returning path such as `std::process::exit(...)` or `panic!` only if that matches existing program termination conventions; prefer explicit process exit for closer parity with C fatal handlers.
- Ensure the implementation does not depend on heap allocation during the failure path when avoidable.

### Phase 3: Connect migrated call sites
- Update existing Rust-ported allocation-related call paths in the same branch to invoke `xalloc_die` where the C code would have called the fatal handler.
- Preserve current control flow expectations by relying on the `!` return type.
- Remove any temporary placeholders or duplicated fatal-allocation logic introduced during migration.

### Phase 4: Validate behavior with focused tests
- Add tests for any non-terminating helper logic only if such helpers are introduced during the port.
- If `xalloc_die` remains a direct terminating function, validate indirectly through isolated command/process-level tests where practical, using `cargo test`.
- Confirm compilation, module visibility, and call-site integration without adding extra infrastructure.
- Verify that no unnecessary allocations or ownership complications were introduced in the fatal path.