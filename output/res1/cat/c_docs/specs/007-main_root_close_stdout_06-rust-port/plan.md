# Implementation Plan

## Summary

This module ports the `closeout.c` functionality into Rust for the `cat` project branch `007-main_root_close_stdout_06-rust-port`. The scope is limited to migrating the existing close-stdout handling logic exposed by:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

The Rust implementation should preserve the current module role: storing minimal process-wide closeout configuration and performing stdout finalization with appropriate error handling at program shutdown. The technical approach should stay close to the C design, using the Rust standard library for stdout flushing and explicit process termination behavior where required by the existing call flow.

The migration should avoid introducing broader output abstractions or new subsystems. The implementation should focus on:

- replacing C global state with minimal Rust module-local state,
- mapping I/O finalization and error inspection to `std::io`,
- preserving current semantics for ignored broken-pipe behavior and optional file-name context,
- keeping the API surface aligned with the existing function set.

## Technical Context

- **Language/Version**: Rust 1.77 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the available input
- **Testing**: `cargo test`
- **Performance Goals**:
  - Negligible overhead relative to the C version
  - No additional buffering layers beyond standard library behavior
  - Constant-time configuration setters
  - Close/flush path should remain lightweight and only execute at shutdown/error-finalization points

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `closeout.c` | `src/closeout.rs` | Direct migration target for close-stdout configuration and finalization logic |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `close_stdout_set_file_name` | `pub fn close_stdout_set_file_name(...)` | Stores optional file-name context in module-local state |
| `close_stdout_set_ignore_EPIPE` | `pub fn close_stdout_set_ignore_epipe(...)` | Stores whether broken-pipe errors should be ignored |
| `close_stdout` | `pub fn close_stdout()` | Flushes/finalizes stdout handling and applies shutdown error policy |

### Integration Placement

| Concern | Rust Location | Notes |
|---|---|---|
| Closeout logic | `src/closeout.rs` | Keep implementation self-contained |
| Calls from main flow | existing `main` path | Replace C-style use sites with direct Rust function calls |
| Module declaration | `src/lib.rs` or `src/main.rs` | Follow current crate structure without adding extra layers |

## Data Model

The source module does not define custom C structs in the provided analysis. The relevant migration concern is module-level state.

### State Mapping

| C Representation | Rust Representation | Notes |
|---|---|---|
| static/global file name pointer | `static` module-local storage for optional file name | Represent as `Option<String>` or `Option<&'static str>` depending on caller ownership needs; prefer owned `String` if input is not guaranteed static |
| static/global ignore-EPIPE flag | `static` module-local boolean | Represent as `bool` in module-local state |

### Recommended Rust State Shape

A minimal internal state container is appropriate:

```rust
struct CloseStdoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}
```

This state should remain internal to `src/closeout.rs`. Because the original C design relies on process-wide mutable configuration, the Rust port will likely require a single module-level mutable holder. The implementation should keep this as narrow as possible and document that configuration is expected to occur during normal single-threaded startup/use, matching the original procedural model rather than expanding concurrency guarantees.

### Error Model Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| stdout close/flush failure | `std::io::Result<()>` from explicit flush operation | Rust does not expose raw `stdout` close in the same way; flushing is the practical finalization step |
| `EPIPE` detection | `io::ErrorKind::BrokenPipe` | Use standard library error classification |
| fatal closeout path | process termination path already used by caller or within `close_stdout` | Preserve existing behavior without introducing custom recovery APIs |

### Memory Management Notes

- Replace raw C string pointers with owned Rust strings where ownership is uncertain.
- Avoid leaking memory or using unsafe global raw pointers.
- Keep any global mutable state encapsulated within the module.
- If interior mutability is required for module-global configuration, use the smallest standard-library mechanism that fits the existing single-process procedural usage.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and migrate configuration state

### Goals
- Establish the direct Rust replacement for `closeout.c`
- Migrate the two configuration setter functions first
- Define the internal state representation

### Tasks
- Add `src/closeout.rs`
- Define internal module state for:
  - optional file name
  - ignore-`EPIPE` flag
- Implement:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_epipe`
- Choose parameter types based on existing call sites:
  - prefer `&str` input with owned internal copy if caller ownership is not static
- Expose only the functions required by the original module

### Notes
- Keep state process-wide to match the C module behavior.
- Do not add builder types, traits, or generalized output-management APIs.
- Keep naming close to the original, allowing only idiomatic Rust case normalization.

## Phase 2: Implement stdout finalization behavior

### Goals
- Port `close_stdout`
- Preserve broken-pipe handling policy and error path structure

### Tasks
- Implement stdout flush/finalization using `std::io::stdout().flush()`
- Inspect flush errors and distinguish:
  - broken pipe
  - all other I/O failures
- Apply the configured `ignore_epipe` behavior to broken-pipe failures
- Include optional file-name context in emitted diagnostics if the original call pattern expects it
- Keep termination behavior aligned with the current main-program expectations

### Notes
- Rust does not provide a direct equivalent to C stdio stream closing for `stdout`; flushing is the intended migration target.
- Error handling should remain explicit and minimal.
- Avoid introducing custom error enums unless necessary to preserve existing call integration.

## Phase 3: Wire the module into the main flow

### Goals
- Replace existing references to the C closeout logic
- Ensure shutdown ordering matches the original usage pattern

### Tasks
- Declare the module in the crate root as needed
- Update main-path call sites to use:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_epipe`
  - `close_stdout`
- Verify that `close_stdout` is invoked at the same lifecycle point as in the C program
- Remove or isolate any obsolete C-oriented assumptions in the translated main logic

### Notes
- Keep integration localized to existing use sites.
- Do not restructure unrelated startup or teardown code.

## Phase 4: Add focused tests and validate behavior

### Goals
- Verify the migrated behavior without broadening scope
- Cover configuration and closeout policy decisions

### Tasks
- Add unit tests for:
  - default state behavior
  - setting file name
  - setting ignore-`EPIPE` flag
- Add tests for closeout decision logic, especially:
  - successful flush path
  - broken-pipe classification path
  - non-broken-pipe error path
- Where direct stdout failure injection is difficult, isolate internal decision-making helpers enough to test error classification without introducing public APIs
- Run `cargo test`

### Notes
- Keep tests close to the module.
- Do not add benchmark or integration harness work beyond what is needed to validate the migration.