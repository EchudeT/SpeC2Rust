# Implementation Plan: module_gnu_error.c_26

## Summary

This module ports `gnu/error.c` into a focused Rust implementation that preserves the existing responsibilities: checking stream openness, flushing standard output before diagnostics, formatting errno-based messages, and emitting error reports with optional line context.

The Rust approach should stay close to the original file-level behavior and avoid introducing broader logging infrastructure. The implementation should center on a single Rust module that exposes Rust equivalents of:

- `is_open`
- `flush_stdout`
- `print_errno_message`
- `error`
- `error_at_line`

The main technical decisions are:

- represent output behavior with `std::io::{Write, stderr, stdout}`
- represent errno/message formatting with `std::io::Error` and OS error codes where needed
- preserve explicit formatting paths for plain error messages vs. file/line-qualified messages
- keep process termination behavior, if required by the original call pattern, localized to the top-level reporting function rather than spread across helpers
- manage global or static state only if the C file requires deduplication or counters during migration; otherwise avoid introducing new persistent state

This module should be migrated as a direct replacement unit, not as a generalized diagnostics subsystem.

## Technical Context

- **Language/Version**: Rust 1.81+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - diagnostic emission should remain effectively O(message length)
  - no unnecessary heap allocation beyond string formatting needed to construct messages
  - stdout flush should perform a single direct flush attempt per call path
  - preserve low overhead for repeated diagnostic calls by keeping helper logic simple and file-local

## Module Mapping

### Source File Mapping

- `gnu/error.c` -> `src/module_gnu_error.rs`

If the crate already uses a grouped module layout for clustered ports, the file may instead be placed at:

- `src/module_cluster/module_gnu_error.rs`

In either case, keep the implementation in one Rust source file corresponding directly to the C source file.

### Function Mapping

- `is_open` -> `fn is_open(...) -> bool`
  - Port as a private helper unless referenced outside this module.
  - The exact Rust parameter type should be chosen from the actual use sites during migration rather than inventing an abstraction.

- `flush_stdout` -> `fn flush_stdout()`
  - Implement with `std::io::stdout().flush()`.
  - Match C behavior for ignored vs. propagated flush failures based on current call flow.

- `print_errno_message` -> `fn print_errno_message(...)`
  - Port as a formatting/emission helper using `std::io::Error` or raw OS error codes.
  - Keep output generation centralized here rather than duplicating formatting in callers.

- `error` -> `pub fn error(...)`
  - Main externally visible reporting entry point for generic diagnostics.
  - Handle message formatting, stderr emission, and any exit-status behavior required by the original logic.

- `error_at_line` -> `pub fn error_at_line(...)`
  - Main externally visible reporting entry point for file/line-qualified diagnostics.
  - Reuse shared formatting/emission logic with `error` where possible without creating extra modules.

## Data Model

No C structs are listed for this module, so the Rust port should avoid introducing new public data structures unless they are required to preserve file-local behavior.

### Data Mapping

- C `FILE *` usage -> Rust standard streams and `Write`
  - Use `std::io::Stdout` / `std::io::Stderr` operations instead of modeling C stream pointers directly where the original file only targets standard streams.
  - If a helper conceptually checks whether a stream is open, constrain that logic to the actual migrated use case.

- C errno integer / `strerror` pattern -> Rust `i32` OS error code plus `std::io::Error`
  - Use `std::io::Error::from_raw_os_error(code)` for errno-derived text.
  - Keep raw error code handling explicit to avoid changing semantics.

- C static file-local state, if present in implementation details -> Rust `static` / `Atomic*` / local state only if strictly needed
  - Introduce such state only after confirming the source file depends on it.
  - Do not add synchronization or wrappers unless the C logic already requires shared mutable state semantics.

## Implementation Phases

## Phase 1: Create Direct Module Skeleton

- Add the Rust target file corresponding to `gnu/error.c`.
- Declare the Rust equivalents for:
  - `is_open`
  - `flush_stdout`
  - `print_errno_message`
  - `error`
  - `error_at_line`
- Keep helper visibility private by default; expose only the functions that must replace existing external C-facing call sites in the Rust codebase.
- Establish imports from `std::io` and `std::process` only as needed by the original behavior.
- Avoid adding traits, adapters, or cross-cutting utility modules.

### Phase 1 Exit Criteria

- The module compiles with placeholder or minimally complete function bodies.
- File placement and naming align with the project’s existing Rust layout.
- No extra support layers are introduced.

## Phase 2: Port Core Diagnostic Behavior

- Implement `flush_stdout` with standard library flushing semantics.
- Implement `print_errno_message` to produce errno-derived text and append it to the diagnostic output path.
- Implement `error` message formatting and stderr emission in the same migration unit.
- Implement `error_at_line` by extending the same formatting path with file/line components.
- Port any C file-local behavior that affects repeated output exactly where it is used, not as a new subsystem.
- Decide explicitly how to handle:
  - formatting errors
  - stderr write failures
  - optional process exit behavior
  based on the existing C function responsibilities and current Rust call sites.

### Phase 2 Exit Criteria

- The module reproduces the original reporting flow for plain and file/line-qualified diagnostics.
- Errno-based messages are generated through Rust OS error facilities.
- Memory ownership is fully safe, with no borrowed data escaping formatting scope.

## Phase 3: Align Signatures and Call Semantics

- Adjust Rust function signatures to match how the surrounding ported code passes:
  - program/module names
  - message text
  - errno values
  - status/exit indicators
  - filename and line number
- Replace any C varargs behavior with explicit Rust formatting inputs appropriate to actual call patterns in the project.
- Keep the signature design minimal and migration-driven; do not generalize for hypothetical future callers.
- Ensure `is_open` reflects only the stream-state check actually needed by the translated code path.

### Phase 3 Exit Criteria

- All migrated callers can invoke the Rust functions without C-style assumptions.
- No unnecessary abstraction has been introduced to simulate generic varargs machinery.
- Exit and non-exit paths match the intended module behavior.

## Phase 4: Add Focused Tests and Final Verification

- Add unit tests for:
  - errno text formatting from known OS error codes
  - message construction for plain diagnostics
  - message construction for file/line diagnostics
  - stdout flush path behavior where testable without extra infrastructure
- Prefer testing formatting helpers or isolated output-construction logic instead of building new capture frameworks.
- Run `cargo test` and resolve any platform-sensitive expectations conservatively.
- Verify that the final module does not allocate or retain state beyond what the original file semantics require.

### Phase 4 Exit Criteria

- `cargo test` passes.
- Output formatting and errno handling are covered by focused tests.
- The module remains a direct, contained port of `gnu/error.c`.

## Migration Notes

- Prefer `String`/`&str` over raw byte buffers unless the C implementation clearly depends on non-UTF-8 handling in this file.
- Keep OS error conversion localized so message text remains tied to the passed errno value.
- Do not infer broader logging, localization, or callback facilities from this module alone.
- If the original C implementation relies on process-global variables from adjacent code, mirror only the minimum required interface in this module and defer unrelated refactors.