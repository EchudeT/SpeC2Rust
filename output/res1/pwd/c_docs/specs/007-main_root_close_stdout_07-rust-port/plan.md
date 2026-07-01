# Implementation Plan

## Summary

Port the C module `closeout.c` into a small Rust module that preserves the existing responsibility: configuring close-of-stdout behavior and performing the final stdout close/error check path. The Rust implementation should stay narrow and map the existing functions directly, using standard library I/O facilities and process termination/error propagation conventions already present in the Rust port of the project.

The implementation should keep the same operational shape as the C code:

- retain module-level configuration for:
  - an optional file name used in diagnostics,
  - whether `EPIPE` should be ignored,
- provide a close/check function that flushes and evaluates stdout completion state,
- convert low-level I/O failures into deterministic Rust-side error handling without introducing new abstractions beyond what is needed to replace the C globals and functions.

Because Rust does not expose a direct equivalent of C `close(stdout)` on `std::io::Stdout`, the technical approach should use `Write::flush()` on `stdout()` as the observable completion boundary for buffered output and then evaluate `BrokenPipe` handling in place of `EPIPE`. The plan should preserve behavior as closely as practical within safe Rust and the project’s existing CLI error-reporting style.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - no meaningful regression versus the C module for normal CLI execution,
  - constant-time configuration setters,
  - no unnecessary allocations on the normal `close_stdout` path aside from any diagnostic formatting required on error.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `closeout.c` | `src/closeout.rs` or `src/main_root_close_stdout_07.rs` | Prefer a single Rust source file corresponding to the migrated module; final placement should follow the existing crate layout. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `close_stdout_set_file_name` | `pub fn close_stdout_set_file_name(name: Option<String>)` or `pub fn close_stdout_set_file_name(name: &str)` | Stores module-level diagnostic context. Final signature should match surrounding project ownership patterns while avoiding unsafe global string lifetimes. |
| `close_stdout_set_ignore_EPIPE` | `pub fn close_stdout_set_ignore_epipe(ignore: bool)` | Maps `EPIPE` handling to `io::ErrorKind::BrokenPipe`. |
| `close_stdout` | `pub fn close_stdout() -> Result<(), std::io::Error>` | Performs stdout flush/finalization check; caller decides whether to print diagnostics or exit, unless the surrounding Rust port already centralizes fatal reporting in this module. |

## Data Model

The C module does not define exported structs, but it typically relies on file-scope state. That state should be mapped into minimal Rust module-private storage.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| static file name pointer | `static` module state holding `Option<String>` | Use owned `String` to avoid lifetime hazards from borrowed data. |
| static ignore-`EPIPE` flag | `static` module state holding `bool` | Simple module configuration flag. |

### Rust State Representation

Use the smallest safe standard-library mechanism that supports module-level mutation during process setup:

- `std::sync::Mutex<Option<String>>` for the optional file name
- `std::sync::atomic::AtomicBool` for the ignore flag

This keeps the port safe without introducing extra crates. Although the original C code may assume single-threaded CLI setup, these primitives allow straightforward mutation from the existing call sites and avoid unsafe global mutable state.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and migrate configuration state

- Add the target Rust file for the module in the existing crate layout.
- Introduce module-private static state for:
  - optional file name,
  - ignore-broken-pipe flag.
- Implement direct Rust equivalents of:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_EPIPE`
- Keep names close to the C originals, adjusting only for Rust naming conventions if the project uses snake_case consistently.
- Ensure setter behavior is simple replacement of prior state, with no extra caching or auxiliary APIs.

### Deliverables
- Compiling module with the two setter functions.
- Unit tests covering:
  - setting and replacing file name state,
  - toggling ignore flag state.

## Phase 2: Port `close_stdout` behavior to Rust I/O semantics

- Implement `close_stdout` using `std::io::stdout().flush()` as the Rust-visible completion check for stdout output.
- Inspect returned `std::io::Error`:
  - if error kind is `BrokenPipe` and ignore mode is enabled, treat as success,
  - otherwise propagate the error.
- If the existing Rust project expects this module to emit diagnostics directly, format diagnostics using the configured file name when present; otherwise return the error and let the caller handle reporting.
- Avoid unsafe code and avoid trying to extract raw file descriptors unless the surrounding port already depends on OS-specific low-level stdio handling.

### Deliverables
- Working `close_stdout` implementation.
- Unit tests for:
  - successful flush path,
  - broken-pipe-ignore decision logic at helper level,
  - non-broken-pipe error propagation logic.

## Phase 3: Integrate with the main command path

- Replace C-side or placeholder closeout usage in the Rust branch with calls into this module.
- Confirm the module is invoked at the same lifecycle point as the original C implementation: after normal output generation is complete.
- Align return types/signatures with the crate’s existing main-path error handling so that process exit behavior remains consistent with the current Rust port structure.
- Remove any redundant placeholder logic for stdout finalization if present elsewhere in the branch.

### Deliverables
- Main-path integration completed.
- `cargo test` passing for the crate.
- Any existing command-level tests updated only as needed for final stdout error handling.

## Phase 4: Validate behavior and finish parity review

- Review the migrated module against the original `closeout.c` function-by-function to confirm:
  - configuration state exists and is used,
  - broken-pipe suppression matches intent,
  - diagnostics context is preserved where applicable.
- Run the full test suite and fix any behavior mismatches caused by Rust buffering or ownership adjustments.
- Keep the final code limited to the original module scope; do not expand into unrelated output/error infrastructure.

### Deliverables
- Final parity review completed.
- `plan` branch ready for implementation follow-through with no additional module expansion.