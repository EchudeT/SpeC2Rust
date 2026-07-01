# Implementation Plan

## Summary

Port the C module `xalloc-die.c` into a focused Rust module that preserves the existing allocation-failure termination behavior used by `cat`. The Rust implementation should provide a direct replacement for `xalloc_die`, keeping the logic narrow: emit the expected fatal allocation message and terminate the process with failure semantics consistent with the original utility behavior.

The implementation should favor the Rust standard library. Since Rust allocation failures are commonly handled through panics or infallible collection growth paths, this port should isolate the explicit termination routine as a small function that can be called from translated allocation/error paths elsewhere in the project. The plan should avoid introducing broader allocation frameworks or generalized error infrastructure beyond what is needed to migrate this file.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - No meaningful runtime overhead beyond a direct error-reporting and process-exit path
  - Keep the function allocation-free in the failure path where practical
  - Preserve constant-time, minimal-control-flow behavior appropriate for fatal termination

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `xalloc-die.c` | `src/xalloc_die.rs` | Direct migration target for the allocation-failure termination routine |
| `xalloc-die.c` (`xalloc_die`) | `xalloc_die` function in `src/xalloc_die.rs` | Preserve single-purpose fatal-exit behavior |

If the crate currently keeps top-level logic in `src/main.rs`, expose the migrated function with a minimal `mod xalloc_die;` declaration there or in `src/lib.rs`, depending on the existing Rust port structure already established for this branch.

## Data Model

This module does not define persistent C data structures.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| None | None | Function-only module |
| C string literals for diagnostics | `&'static str` | Use static string messages in Rust |
| Process termination status | `std::process::ExitCode` or direct `std::process::exit` integer | Choose the form that best matches surrounding migrated code; direct exit is most likely for parity with C fatal behavior |

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/xalloc_die.rs`.
- Define the Rust equivalent of `xalloc_die` as a standalone function with crate-visible or public visibility only as required by current call sites.
- Keep the API minimal and aligned to the original role: no returned `Result`, since the C function is a fatal path.
- Wire the module into the existing crate layout without introducing extra abstraction layers.

### Phase 2: Port fatal reporting and termination semantics

- Implement the diagnostic emission using standard error output via the Rust standard library.
- Match the original behavior as closely as practical for:
  - allocation-failure wording
  - fatal/non-returning control flow
  - exit status semantics used by the utility
- Ensure the function does not rely on heap allocation during the failure path where avoidable.
- Represent the non-returning behavior explicitly, preferably with the Rust never type `-> !` if compatible with the surrounding code.

### Phase 3: Integrate translated call paths

- Update any already-ported allocation helpers or callers in this branch to invoke the Rust `xalloc_die` function instead of ad hoc termination logic.
- Keep integration scoped strictly to replacing the behavior supplied by `xalloc-die.c`; do not introduce new shared error systems.
- Confirm that module imports and visibility remain minimal and conventional.

### Phase 4: Add focused tests and verification

- Add unit tests for any non-exit formatting helpers only if such helpers are needed to make the function testable.
- If the function exits directly, prefer testing extracted message-generation behavior rather than process termination unless the project already uses subprocess-style tests.
- Run `cargo test` to confirm the module builds and any narrow tests pass.
- Verify that the final implementation preserves memory-safety and avoids unsafe code unless a specific existing interface requires it.