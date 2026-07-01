# Implementation Plan: main_root_close_stdout_06

## Summary

Port the `closeout.c` functionality into a single Rust module that preserves the existing responsibility boundaries: storing optional closeout context and performing final standard-output shutdown/error reporting logic. The Rust implementation should stay narrowly aligned with the C module’s role in the `cat` main execution path, replacing C global/static state and close-time error handling with idiomatic Rust equivalents based on `std::io`.

The implementation should migrate the three existing functions as direct Rust counterparts:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

The technical approach is to use module-local mutable state for the configured file name and EPIPE-handling flag, expose small setter functions matching the original call pattern, and implement close/flush behavior using Rust standard I/O facilities. Error handling should map OS write/flush failures into process-facing diagnostics without introducing broader abstractions or additional subsystems.

## Technical Context

### Language/Version
- Rust stable
- Recommended minimum: Rust 1.74+

### Primary Dependencies
- Rust standard library only:
  - `std::io` for stdout flushing and error inspection
  - `std::sync` primitives only if needed for safe module-level mutable state
  - `std::process` only if existing port structure expects direct termination behavior here

No third-party crates are recommended because the provided C scope does not justify external dependencies.

### Testing
- `cargo test`

Testing should focus on:
- setter behavior for stored configuration
- closeout behavior when no file name is configured
- closeout behavior when EPIPE is configured to be ignored
- error classification/mapping logic separated enough to be unit tested without requiring fragile real pipe failures where possible

### Performance Goals
- Negligible runtime overhead versus the C implementation
- Constant-time configuration updates
- No heap allocation beyond storing the optional file name
- Closeout path should remain a single finalization step with minimal branching

## Module Mapping

### C to Rust File Mapping
- `closeout.c` → `src/main_root_close_stdout_06.rs` or `src/closeout.rs`

Preferred choice:
- `closeout.c` → `src/closeout.rs`

This keeps the port close to the original source naming and avoids introducing extra module layers beyond the migrated file.

### Function Mapping
- `close_stdout_set_file_name` → `pub(crate) fn close_stdout_set_file_name(...)`
- `close_stdout_set_ignore_EPIPE` → `pub(crate) fn close_stdout_set_ignore_epipe(...)`
- `close_stdout` → `pub(crate) fn close_stdout()`

Notes:
- Rust naming should follow snake_case; `ignore_EPIPE` becomes `ignore_epipe`.
- If the surrounding port requires preserving exact call naming patterns in internal code review, a thin wrapper can retain a closer name, but the implementation should still remain Rust-idiomatic.
- The `close_stdout` function should keep the same narrow purpose: finalize stdout and handle failure according to the configured state.

## Data Model

The input module defines no public structs, so the migration is primarily from implicit C file-scope state to Rust module state.

### C Static/File-Scope State → Rust Module State
- C file-scope `char const *` / nullable file name pointer
  - Rust: `Option<String>` or `Option<&'static str>` depending on call-site ownership
  - Preferred: `Option<String>` unless the wider port already guarantees `'static` string lifetimes

- C file-scope boolean/int flag for ignoring `EPIPE`
  - Rust: `bool`

### State Container
Because Rust does not allow unsynchronized mutable global state directly, encapsulate the migrated state in one restrained module-local holder:

```rust
struct CloseStdoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}
```

Then store it as:
- `static` guarded by `std::sync::Mutex<CloseStdoutState>`, or
- `std::sync::OnceLock<Mutex<CloseStdoutState>>`

Preferred:
- `OnceLock<Mutex<CloseStdoutState>>`

This avoids unsafe global mutation while keeping the design close to the original single-instance C state.

### Error Representation
No new custom error hierarchy is necessary. Use:
- `std::io::Error`
- `std::io::ErrorKind`

Where platform-specific broken-pipe detection needs finer control than `ErrorKind::BrokenPipe`, inspect `raw_os_error()` only within the closeout implementation.

## Implementation Phases

## Phase 1: Create Rust Module Skeleton and State Mapping

### Goals
- Establish the Rust file corresponding to `closeout.c`
- Port module-level state and the two setter functions
- Keep interfaces narrow and aligned with the original module role

### Tasks
- Add `src/closeout.rs`
- Define internal `CloseStdoutState`
- Add module-local lazy-initialized storage using `OnceLock<Mutex<CloseStdoutState>>`
- Implement:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_epipe`
- Choose setter parameter type based on existing call sites:
  - `&str` with owned copy into `String` is the safest default
- Keep mutation failure handling simple; if mutex locking fails due to poisoning, recover with `into_inner()` rather than creating new error infrastructure

### Completion Criteria
- Module compiles
- State updates can be verified with unit tests
- No unsafe code is introduced unless forced by surrounding integration constraints

## Phase 2: Port `close_stdout` Finalization Logic

### Goals
- Recreate the close/flush semantics from `closeout.c`
- Preserve special handling for broken-pipe/EPIPE when configured
- Keep output/error behavior aligned with main-program expectations

### Tasks
- Implement `close_stdout` using `std::io::stdout().lock()` and flush semantics
- Detect and classify flush/write-close related errors
- Treat broken pipe as non-fatal when `ignore_epipe` is set
- Use stored `file_name` only for message context if the surrounding port expects diagnostics here
- If the original C function terminates the process directly on failure, mirror that only if required by current Rust main-cluster architecture; otherwise return through the existing program error path if already established elsewhere in the port

### Rust Mapping Notes
- There is no direct explicit `close(1)` equivalent for the process-global `stdout` handle in safe Rust std APIs; flushing the standard output stream is the practical migration target
- If the broader application writes via buffered wrappers around stdout, ensure `close_stdout` is called after those writers are dropped or flushed by existing main logic rather than adding new ownership layers here

### Completion Criteria
- `close_stdout` compiles and handles success/error paths
- Broken pipe handling is explicitly covered
- No extra abstractions beyond this module are introduced

## Phase 3: Integrate with Main Cluster and Align Call Sites

### Goals
- Replace use of the C module with the Rust module in the current branch scope
- Ensure function ordering and invocation points match the original lifecycle

### Tasks
- Update the main-cluster ported code to call:
  - `close_stdout_set_file_name` at the same logical point as the C code
  - `close_stdout_set_ignore_epipe` where configured
  - `close_stdout` during final shutdown
- Remove or avoid duplicate closeout logic elsewhere in the Rust main path
- Verify that ownership/lifetime of passed file names is compatible with the chosen owned-string storage

### Completion Criteria
- Main-cluster code builds against the Rust module
- Finalization path is single-sourced through this migrated module
- No duplicate stdout shutdown handling remains

## Phase 4: Add Focused Tests and Final Validation

### Goals
- Validate the migrated state handling and error-path decisions
- Keep tests local and deterministic where possible

### Tasks
- Add unit tests for:
  - default state initialization
  - setting/replacing file name
  - toggling ignore-epipe flag
- Isolate error classification logic into a small internal helper if needed so broken-pipe recognition can be tested without depending on real OS pipe failures
- Add a minimal integration-oriented test only if the project already has a pattern for process-level I/O tests; otherwise keep to unit coverage

### Completion Criteria
- `cargo test` passes
- Core setter/state behavior is covered
- Broken-pipe decision path is exercised in tests without introducing extra harness infrastructure

## Notes and Constraints

- Prefer the Rust standard library exclusively.
- Do not expand the port into a generalized output-management subsystem.
- Do not add thread-safety APIs beyond the minimum required for safe module-global state.
- Do not introduce FFI, custom allocators, or platform abstraction layers.
- Keep memory ownership explicit: copy incoming file-name text into owned Rust storage unless the larger codebase already provides a stable static lifetime.
- Keep error handling local to this module and aligned with the existing main-cluster control flow.