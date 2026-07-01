# Implementation Plan: main_root_close_stdout_07

## Summary

This module migrates the `closeout.c` functionality into a focused Rust implementation that preserves the existing role of final standard-output shutdown handling in the program entry path. The Rust version should keep the same narrow responsibilities:

- store optional output file name context for diagnostics,
- store whether `EPIPE` should be ignored,
- perform final stdout close/flush handling with process-terminating behavior on failure.

The implementation should remain small and localized to the main execution cluster, using Rust standard library I/O primitives and OS error inspection where needed. Since the original C module exposes setter-style global configuration plus a final close routine, the Rust port should mirror that shape with minimal redesign, using module-local process-wide state only as required to preserve existing call patterns.

## Technical Context

- **Language/Version:** Rust 1.77 or newer
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - negligible runtime overhead relative to the C version,
  - no extra heap allocation on the normal close path beyond storing configured file name state,
  - constant-time configuration access during final stdout handling,
  - preserve direct OS error interpretation for broken-pipe and write/flush failures.

## Module Mapping

### C to Rust File Mapping

- `closeout.c` → `src/main_root_close_stdout_07.rs` or integrated into the branch’s main-cluster module file if that cluster is already consolidated in the Rust tree

If the Rust project already groups main-cluster ports into a single module tree, this module should be added there without introducing extra abstraction layers. Otherwise, keep it as a single Rust source file matching this migration unit.

### Function Mapping

- `close_stdout_set_file_name` → `pub(crate) fn close_stdout_set_file_name(...)`
- `close_stdout_set_ignore_EPIPE` → `pub(crate) fn close_stdout_set_ignore_epipe(...)`
- `close_stdout` → `pub(crate) fn close_stdout(...) -> !` or `pub(crate) fn close_stdout()` depending on surrounding exit-handling conventions

Notes:
- Naming may follow Rust snake_case while preserving the original function boundaries.
- If the current Rust main path already centralizes process exit in callers, `close_stdout` should return a `Result` internally and keep a thin outward wrapper matching existing call sites. Do not broaden behavior beyond what is needed to preserve the original closeout semantics.

## Data Model

This module has no explicit C structs in the provided input, so the migration centers on module state and OS error handling.

### State Mapping

- C static/global file-name pointer → Rust module-local `Option<String>` or `Option<PathBuf>`
- C static/global ignore-`EPIPE` flag → Rust module-local `bool`

Recommended representation:
- use `Option<String>` unless there is concrete evidence the wider Rust port already standardizes on path types here,
- use `std::sync::Mutex` plus `std::sync::OnceLock` for safe one-time global state initialization if mutable process-wide configuration is required by existing call order.

Example conceptual mapping:
- unnamed C module globals → Rust `struct CloseStdoutConfig { file_name: Option<String>, ignore_epipe: bool }`

This struct can remain private to the module and be stored in a `OnceLock<Mutex<...>>`. That keeps mutation explicit and memory-safe while preserving the original setter-based API. No extra public types are needed.

### Error Mapping

- C `errno == EPIPE` checks → Rust `io::Error::raw_os_error() == Some(libc::EPIPE)` is one possible route, but prefer avoiding external crates.
- Since third-party crates are not justified by the input, use platform constants available through `std`-compatible conditional code if already present in the codebase; otherwise isolate the raw OS error comparison behind a small helper and use a minimal platform-specific constant definition only where necessary.

The helper should distinguish:
- broken pipe with ignore enabled,
- all other flush/close failures,
- absence of OS error code, treated as generic failure.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and configuration state

- Add the Rust file for this migration unit in the existing main-cluster layout.
- Define private module configuration state for:
  - optional file name,
  - ignore-`EPIPE` flag.
- Implement:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_epipe`
- Keep signatures aligned with existing Rust call sites in the branch rather than inventing new ownership flows.
- Ensure setters copy incoming string data into owned Rust storage to avoid lifetime issues that differ from C pointer semantics.

### Deliverables
- compilable module file,
- private config holder,
- setter functions with unit tests for state updates.

## Phase 2: Implement stdout finalization behavior

- Implement the `close_stdout` routine using `std::io::stdout().lock()` and explicit `flush()`.
- Reproduce the original decision flow:
  - successful flush/close path returns normally or exits through the surrounding main path as currently structured,
  - broken-pipe failures are ignored only when configured,
  - all other failures produce the same failure path expected by the main cluster.
- Keep error reporting minimal and compatible with the rest of the Rust port:
  - include configured file name in diagnostics when present,
  - avoid introducing new reporting frameworks.

### Technical notes
- Rust does not expose an explicit close operation for process stdout analogous to C stream closure; use flush failure as the observable terminal I/O error point.
- If the wider port already writes through `BufWriter` or another owned output object, adapt this module to operate at that same finalization point instead of adding duplicate buffering or shutdown code.
- Encapsulate OS error inspection in a private helper to keep the main function body close to the original C control flow.

### Deliverables
- `close_stdout` implementation,
- private broken-pipe detection helper,
- tests covering success, ignored broken pipe classification logic where testable, and non-ignored error classification through helper-level tests.

## Phase 3: Integrate with the main cluster call path

- Replace or wire up the existing placeholder/fallback closeout logic in the Rust main path to call this migrated module.
- Preserve original ordering: configuration setters must run before final close handling if the C flow depended on that sequence.
- Verify that no duplicate final stdout flush/error reporting remains elsewhere in the main cluster.
- Keep integration scoped strictly to current module responsibilities.

### Deliverables
- main-cluster call-site update,
- removal of redundant local closeout logic if present,
- integration tests or targeted unit tests validating configured file name and ignore-`EPIPE` behavior from the main path boundary.

## Phase 4: Cleanup and parity review

- Review the Rust implementation against `closeout.c` for:
  - setter semantics,
  - default state values,
  - broken-pipe ignore behavior,
  - diagnostic context usage,
  - termination/error propagation shape.
- Tighten signatures and visibility to `pub(crate)` or private as appropriate.
- Confirm no unsafe code is needed; if any platform-specific raw error constant handling requires special care, keep it isolated and documented.

### Deliverables
- final parity pass,
- test suite passing under `cargo test`,
- module comments limited to migration-relevant behavior and platform assumptions.