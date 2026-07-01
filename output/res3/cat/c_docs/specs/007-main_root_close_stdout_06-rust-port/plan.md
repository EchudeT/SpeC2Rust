# Implementation Plan: main_root_close_stdout_06

## Summary

This module ports the `closeout.c` functionality into Rust for the `cat` project’s main execution path. The scope is limited to migrating the existing close-stdout behavior represented by:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

The Rust implementation should preserve the current operational shape: maintain module-level configuration for the output file name and EPIPE handling, and provide a close/finalization routine for standard output/stderr-related shutdown behavior at program end. The implementation should stay minimal and align with the existing main-cluster integration rather than introducing broader I/O abstractions.

Technically, the port should use Rust’s standard library facilities for process I/O and error inspection, with a small internal module that stores the migrated configuration state and exposes the three migrated functions with Rust-idiomatic signatures where practical. Because the C source relies on process-global behavior, the Rust module will likely also require narrowly scoped global state. This should be implemented with standard library synchronization primitives only if needed for safe initialization and mutation.

## Technical Context

- **Language/Version**: Rust 1.81+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - No measurable overhead in normal `cat` execution beyond a constant-time finalization step.
  - Avoid additional buffering layers or allocations in the close path except for storing an optional file name.
  - Preserve direct, low-complexity shutdown logic suitable for a command-line utility.

## Module Mapping

### C to Rust File Mapping

- `closeout.c` → `src/main_root_close_stdout_06.rs`
  or, if the current project layout already groups main-cluster logic under a module tree, the equivalent single Rust source file for this ported unit.

### Function Mapping

- `close_stdout_set_file_name`
  → `pub(crate) fn close_stdout_set_file_name(...)`
  - Store the current output file name for later diagnostics during stdout close/finalization.

- `close_stdout_set_ignore_EPIPE`
  → `pub(crate) fn close_stdout_set_ignore_epipe(...)`
  - Store whether broken-pipe errors should be ignored during close/finalization.

- `close_stdout`
  → `pub(crate) fn close_stdout() -> ...`
  - Perform stdout finalization/flush/close-equivalent behavior and map errors into Rust process-exit handling at the call site or preserve a direct termination pattern if that matches the existing main flow.

### Integration Mapping

- Existing callers in the main execution path should be updated to invoke the Rust module functions directly.
- The implementation should remain a leaf-like support module for program shutdown, not a general output management layer.

## Data Model

This C unit does not declare public data structures in the provided input, but it does imply module-scoped state.

### Implied C Static State → Rust State

- **C static file-name pointer**
  → `static` Rust storage for an optional file name
  Suggested Rust representation:
  - `Option<String>` if ownership is transferred/set from runtime values
  - `Option<&'static str>` only if call sites naturally provide static strings
  Prefer `Option<String>` for safer migration from C string-pointer semantics.

- **C static ignore-EPIPE flag**
  → `static` Rust storage for a boolean flag
  Suggested Rust representation:
  - `bool`

### State Management Approach

Because the original C behavior is process-global, the Rust port should keep process-global module state rather than introducing a new object model. Use the smallest standard-library mechanism that supports safe mutation:

- `std::sync::Mutex` guarded global state, initialized with `std::sync::OnceLock`, if mutation occurs after startup.
- If initialization order guarantees a single setup before use, keep the design simple but still safe under Rust’s rules.

Suggested internal state shape:

```rust
struct CloseStdoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}
```

This struct should remain private to the module.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and migrate configuration setters

- Add the Rust module file corresponding to `closeout.c`.
- Define a private `CloseStdoutState` struct holding:
  - optional file name
  - ignore-EPIPE flag
- Add module-global state using standard library initialization/synchronization primitives.
- Implement:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_epipe`
- Update call sites in the main-cluster code to use the Rust setters.

### Phase 1 Notes
- Convert incoming C-style file-name usage into owned Rust string storage at the module boundary.
- Keep setter behavior simple and side-effect-free beyond updating module state.
- Do not introduce additional configuration types or wrappers.

## Phase 2: Port `close_stdout` behavior

- Implement the Rust equivalent of `close_stdout` using standard output flush/finalization behavior available from `std::io`.
- Reproduce the intended broken-pipe handling:
  - detect `BrokenPipe`/EPIPE-equivalent write or flush failures
  - suppress reporting when the ignore flag is set
- Preserve file-name-aware diagnostics where the original behavior uses the configured name.
- Decide final signature based on surrounding main logic:
  - either return `io::Result<()>` / custom minimal result to caller
  - or retain direct process-exit/reporting behavior if required by the existing migrated main path

### Phase 2 Notes
- Rust does not expose a direct portable “close stdout” primitive identical to C `close`; use flush and shutdown-path error observation as the migration target.
- Keep stderr reporting behavior aligned with the current program flow and avoid building a new error framework.

## Phase 3: Integrate shutdown behavior into the main execution path

- Replace remaining references to the C closeout unit with the Rust module.
- Ensure the final output shutdown path calls `close_stdout` exactly where the original control flow expects it.
- Verify exit-status behavior for:
  - successful completion
  - ordinary write/flush error
  - broken pipe with ignore disabled
  - broken pipe with ignore enabled

### Phase 3 Notes
- Keep integration changes localized to the main-cluster code already involved with process termination.
- Do not broaden the module’s responsibility beyond stdout finalization.

## Phase 4: Add focused tests and remove the old C implementation from the Rust branch build path

- Add unit tests for module state transitions:
  - setting file name
  - setting ignore-EPIPE flag
- Add tests for close/finalization decision logic where it can be exercised deterministically.
- If direct stdout manipulation is difficult to unit test, isolate the minimal decision logic into private helper functions that operate on `io::ErrorKind`-level inputs while keeping public API unchanged.
- Confirm the C source is no longer used by this branch’s build path.

### Phase 4 Notes
- Prefer narrow tests over infrastructure-heavy end-to-end harnesses.
- Keep test helpers internal and strictly in support of the existing migrated functions.