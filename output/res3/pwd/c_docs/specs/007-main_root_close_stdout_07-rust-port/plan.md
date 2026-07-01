# Implementation Plan: main_root_close_stdout_07

## Summary
Port `closeout.c` into a Rust module that preserves the existing close-stdout behavior used by the program entry path, limited to the three exported functions:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

The Rust implementation should stay narrow in scope: migrate the current file-level behavior into a small Rust module under the main program area, using the Rust standard library for stream flushing and process termination signaling through returned errors or explicit exit handling at the call site. Since the C source exposes setter-style global configuration plus a final close routine, the Rust port should mirror that shape with minimal internal state, avoiding new abstractions beyond what is needed to represent the configured output target name and the `EPIPE`-ignore flag.

The technical approach is:

- model the C module as one Rust source file with module-private global state,
- use standard output flushing as the Rust equivalent of checking write/close failures on `stdout`,
- preserve user-visible error behavior by reporting failures consistently through the main call path,
- map broken-pipe handling to Rust’s `BrokenPipe` I/O error kind and suppress/report it according to the migrated flag.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74+`

### Primary Dependencies
- Rust standard library only:
  - `std::io`
  - `std::sync` for minimal global-state coordination
  - `std::process` only if the existing main path requires direct exit behavior

No third-party crates are recommended because the input does not indicate any external dependency need.

### Testing
- `cargo test`

Test coverage should focus on:
- configuration state updates from the two setter functions,
- `close_stdout` success path,
- broken-pipe suppression behavior,
- non-broken-pipe error propagation/reporting behavior where testable.

### Performance Goals
- No material regression from the C implementation.
- Constant-time configuration updates.
- Single flush/check on close path, with no unnecessary allocation on steady-state execution.
- Overhead should remain negligible relative to terminal or pipe I/O costs.

## Module Mapping

### C to Rust File Mapping
- `closeout.c` -> `src/main_root_close_stdout_07.rs`

If the project already uses a clustered main-module layout, this file should be declared from the existing root/main module without introducing extra layers.

### Function Mapping
- `close_stdout_set_file_name` -> `pub(crate) fn close_stdout_set_file_name(...)`
- `close_stdout_set_ignore_EPIPE` -> `pub(crate) fn close_stdout_set_ignore_epipe(...)`
- `close_stdout` -> `pub(crate) fn close_stdout() -> Result<(), std::io::Error>`

Notes:
- Rust naming should follow snake_case; keep any external call sites updated accordingly.
- If the surrounding main path currently expects termination inside this routine, keep the module logic focused on detection and error classification, and perform final exit/message behavior at the existing caller location unless direct in-function termination is already structurally required by the port.

## Data Model

No explicit C structs were identified in the input. The C module appears to rely on file-local mutable state. The Rust mapping should therefore use minimal module-private state rather than inventing public data types.

### State Mapping
- C static file name pointer -> Rust module-private optional owned string
  - Suggested type: `Option<String>`
- C static ignore-`EPIPE` flag -> Rust module-private boolean
  - Suggested type: `bool`

### Storage Strategy
Because the original C module likely uses mutable static state, Rust should represent this with a minimal synchronized global:

- `static` state initialized once
- inner fields protected with `Mutex`

Suggested internal shape:

```rust
struct CloseStdoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}
```

This struct should remain private to the module and exist only to group the migrated state. It is not a new public abstraction, only a direct replacement for C file-scope variables.

### Memory Management Notes
- Convert incoming file name data to owned `String` at configuration time to avoid lifetime issues.
- Avoid retaining borrowed references in global state.
- Keep lock scope short around reads/writes of configuration.

### Error Handling Notes
- Map Rust `io::ErrorKind::BrokenPipe` to the C `EPIPE` behavior.
- Return I/O errors to the caller rather than hiding them.
- Use the configured file name only for caller-side diagnostics if needed; do not create extra error wrapper types unless already required elsewhere in the port.

## Implementation Phases

### Phase 1: Create the Rust Module Skeleton
- Add `src/main_root_close_stdout_07.rs`.
- Declare the three migrated functions with Rust naming.
- Add a private `CloseStdoutState` and a single module-private global holder using standard-library synchronization.
- Wire the module into the existing main-module tree on branch `007-main_root_close_stdout_07-rust-port`.

Deliverable:
- Compiling module skeleton with placeholder logic and no expanded functionality.

### Phase 2: Port Configuration State and Close Logic
- Implement `close_stdout_set_file_name` to replace the stored file name.
- Implement `close_stdout_set_ignore_epipe` to update the broken-pipe suppression flag.
- Implement `close_stdout` using `std::io::stdout().flush()` as the Rust close/check equivalent for standard output finalization.
- On error:
  - suppress `BrokenPipe` when the flag is enabled,
  - otherwise return the error unchanged.
- Keep diagnostics responsibilities aligned with the existing main call path rather than adding new reporting facilities.

Deliverable:
- Functional parity for the module’s state updates and stdout-finalization decision logic.

### Phase 3: Integrate with Existing Main Call Path
- Update the current main/root logic to call the Rust setters and `close_stdout` in the same order and positions as the C-based flow.
- Preserve existing exit/error behavior by handling the returned `Result` at the same semantic boundary as before.
- Ensure naming and module imports remain local to the migrated main cluster and do not introduce unrelated refactors.

Deliverable:
- End-to-end integration of the Rust module in place of the C implementation.

### Phase 4: Add Focused Tests
- Unit test state mutation for file name replacement and `ignore_epipe` toggling.
- Add tests for `close_stdout` behavior where practical:
  - success path,
  - broken-pipe classification logic through factored internal helpers if direct stdout failure injection is awkward,
  - non-broken-pipe propagation.
- Keep tests limited to migrated behavior; do not add new infrastructure beyond what is required to validate the port.

Deliverable:
- `cargo test` coverage for the migrated module behavior and integration-sensitive error handling.