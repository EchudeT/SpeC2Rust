# Implementation Plan

## Summary

Port `progname.c` into a focused Rust module that preserves the existing responsibility of `set_program_name`: deriving and storing the executable's program name for later use by the main command path. The Rust implementation should stay minimal and align with standard Rust project structure, using safe standard-library path handling where possible and containing any unavoidable global state behind a small module-local interface.

The implementation should migrate the existing behavior rather than redesign it. In practice, this means translating the C routine that extracts the basename of the invoked program into a Rust function that accepts the invocation path, normalizes it to the final path component, and stores it in a process-global location suitable for the rest of the ported main cluster.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time global access after initialization
  - Single allocation at initialization time at most
  - No meaningful regression versus the C basename extraction path
  - Negligible startup overhead within the main program flow

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `progname.c` | `set_program_name` | `src/progname.rs::set_program_name` | Direct migration of program-name extraction and storage logic |
| `progname.c` | module-level program name state | `src/progname.rs` module-local static storage | Replace C global with controlled Rust global initialization |

## Data Model

This module does not define dedicated C structs.

| C Element | Rust Mapping | Notes |
|---|---|---|
| global program name pointer/string storage | `static` module state holding owned string data | Prefer `std::sync::OnceLock<String>` if write-once behavior matches the original initialization pattern |
| `char *` / C string program path input | `&str` or `&std::ffi::OsStr` at module boundary | Use `&str` if upstream Rust main path already provides UTF-8; otherwise keep extraction logic on `Path`/`OsStr` and convert only when required by callers |

## Implementation Phases

### Phase 1: Establish Rust module and state mapping

- Create `src/progname.rs` for the migrated contents of `progname.c`.
- Add a narrow module API centered on `set_program_name`.
- Replace the C global storage model with a Rust module-local static initialized once.
- Choose the smallest viable ownership model:
  - preferred: `OnceLock<String>` for single-assignment semantics
  - avoid introducing synchronization wrappers beyond what is required for safe global initialization
- Define how callers in the main cluster will access the stored program name, but keep this limited to what is necessary for the existing port.

### Phase 2: Port basename extraction logic

- Translate the logic of `set_program_name` into Rust using `std::path::Path`.
- Preserve the C behavior of deriving the executable name from the invocation path rather than expanding functionality.
- Handle edge cases relevant to the original routine:
  - input containing directory separators
  - input already equal to a bare program name
  - empty or malformed input only to the extent required to avoid panics
- Keep memory ownership explicit:
  - store an owned `String` in module state
  - avoid borrowed data tied to `argv` lifetimes
- Keep error handling simple and local:
  - if signature design permits, return a small `Result` for invalid initialization input
  - otherwise use a conservative fallback consistent with the main program’s expectations

### Phase 3: Integrate with the main cluster

- Update the Rust main-path code on branch `021-main_root_progname.c_21-rust-port` to call `progname::set_program_name` at the same point the C code initializes program naming.
- Ensure call order matches the original initialization sequence so later main-cluster logic can rely on the stored value.
- Remove any temporary duplication of program-name extraction logic from the Rust main entry path once this module is wired in.
- Keep integration limited to existing files and functions that depend on this initialization behavior.

### Phase 4: Add focused tests and finalize behavior

- Add unit tests in `src/progname.rs` or adjacent module tests covering:
  - simple executable name input
  - path input with parent directories
  - repeated initialization behavior, matching the selected single-assignment policy
- Add integration-level verification only if already required by the current Rust main path; do not create extra harnesses.
- Run `cargo test` and confirm:
  - no panics on valid invocation-path inputs
  - stable stored program name across later accesses
  - behavior remains constrained to the original module’s scope