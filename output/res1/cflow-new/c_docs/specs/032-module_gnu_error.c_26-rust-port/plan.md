# Implementation Plan: module_gnu_error.c_26

## Summary

This module ports `gnu/error.c` into a focused Rust module that preserves the existing error-reporting behavior and call structure without adding new capabilities. The implementation should cover the current function surface:

- `is_open`
- `flush_stdout`
- `print_errno_message`
- `error`
- `error_at_line`

The Rust approach should stay close to the original procedural design. The module should provide formatting and emission of diagnostics to standard error, optional inclusion of location information, and handling of OS error messages through Rust’s standard library error facilities. Any C-style global behavior used by the original file should be represented with the smallest Rust equivalent necessary to preserve module behavior.

The migration should prioritize:
- direct function-level porting,
- standard-library-based I/O and OS error handling,
- explicit ownership and borrowing instead of implicit C pointer lifetime assumptions,
- minimal state translation if the C file relies on module-global counters or flags.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.75+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:
- None required for the initial port

Standard library areas expected to be used:
- `std::io` for stream writes and flushing
- `std::fmt` for formatting support
- `std::sync` only if unavoidable for preserving existing module-global mutable state
- `std::process` if the original control flow exits on fatal error
- `std::ffi` only if any migrated signatures temporarily need C-compatible string handling during internal transition

### Testing

- `cargo test`

Test scope should remain limited to migrated module behavior:
- formatting of emitted messages
- handling of optional file/line location data
- flushing behavior return paths
- errno / OS error message rendering
- fatal vs non-fatal control-flow behavior, if present in the C logic

### Performance Goals

This module is I/O-bound and diagnostic-oriented, so performance goals should be conservative:

- no avoidable heap allocation beyond message formatting needs
- avoid duplicate formatting passes where a single write path is sufficient
- preserve straightforward write-to-stderr behavior
- maintain behavior comparable to the C implementation for normal diagnostic paths

## Module Mapping

### Source Mapping

- C source: `gnu/error.c`
- Rust target: `src/module_gnu_error.rs`

If the crate already uses a module tree for cluster ports, place the file in the existing standard location and expose only the migrated functions required by the current project structure. Do not split this single C file into multiple Rust modules unless required by the existing repository layout.

### Function Mapping

- `is_open`
  - Port as a private helper unless referenced outside the module
  - Represent file-descriptor/open-stream checks using the narrowest Rust equivalent possible
  - If exact stream-state probing is not directly expressible in safe Rust, preserve only the observable behavior needed by callers in this module

- `flush_stdout`
  - Port as a module-level function
  - Implement with `std::io::stdout().flush()`
  - Return/propagate status in a Rust form that matches surrounding module use

- `print_errno_message`
  - Port as a private helper unless externally needed
  - Use `std::io::Error::last_os_error()` or explicit OS error code conversion, depending on the C call pattern
  - Keep message emission logic close to the original ordering and punctuation

- `error`
  - Port as the primary public function corresponding to the C entry point
  - Preserve whether it reports a simple message, appends errno text, and optionally terminates

- `error_at_line`
  - Port as the primary public function for file/line-based diagnostics
  - Preserve location formatting and any line-level suppression logic if present in the C file

### Visibility Plan

- Public:
  - `error`
  - `error_at_line`
- Private:
  - `is_open`
  - `flush_stdout` if not used elsewhere
  - `print_errno_message`

Adjust visibility only if the current Rust crate structure already requires broader access for integration.

## Data Model

No explicit C structs are listed for this module. The expected data mapping is therefore limited to primitive state and possible module-global values.

### C Primitive / State Mapping

- `char *` / C strings used for message inputs
  - Prefer `&str` for internal Rust callers
  - Use `String` only when ownership is necessary
  - If temporary compatibility with byte-oriented inputs is needed, isolate it at the function boundary

- `int` error status / line number / errno
  - Map to `i32` or `u32` only where exact semantics are clear
  - Prefer `i32` for OS-style error codes
  - Prefer `u32` or `usize` for line numbers if the original code treats them as non-negative; otherwise preserve signedness only where behavior depends on it

- C varargs formatting behavior
  - Replace with Rust formatting-oriented function signatures appropriate to the calling code in the port
  - If exact call compatibility is needed within the Rust project, use explicit formatted string construction at call sites rather than introducing generalized dynamic varargs emulation

- Module-global counters or flags, if present in `error.c`
  - Map to `static`/`static mut` alternatives only as needed
  - Prefer `Atomic*` or interior mutability only when mutation is required by preserved behavior
  - Do not introduce extra synchronization beyond what is needed to compile safely and preserve current semantics

### Data Structure Mapping Table

| C construct | Rust construct | Notes |
|---|---|---|
| No named struct in this module | No dedicated struct required initially | Keep procedural design |
| C string inputs | `&str` / `String` | Choose borrowed strings by default |
| `int` errno/status | `i32` | Align with OS error code usage |
| file/line inputs | `&str`, integer line type | Keep formatting local to reporting functions |
| global module state, if any | minimal `static` state | Only if required by original behavior |

## Implementation Phases

## Phase 1: Establish module skeleton and port local helpers

Goals:
- create the Rust module file
- establish the smallest API surface
- port helper behavior before top-level reporting functions

Tasks:
- Add `src/module_gnu_error.rs`
- Implement `flush_stdout` using standard output flushing via `std::io::Write`
- Implement `print_errno_message` with standard-library OS error formatting
- Implement `is_open` only to the extent required by this module’s internal behavior
- Identify any C globals/macros from `error.c` that affect helper logic and map them to Rust constants or minimal state

Completion criteria:
- helper functions compile
- helper return types are consistent with intended `error`/`error_at_line` use
- no extra module decomposition is introduced

## Phase 2: Port core reporting functions

Goals:
- migrate `error`
- migrate `error_at_line`
- preserve message composition and control flow

Tasks:
- Implement stderr write path using `eprint!`/`writeln!` or explicit `stderr` locking, choosing the simpler approach that preserves ordering
- Recreate message assembly order:
  - program/module prefix if present in the original
  - optional file and line context
  - primary formatted message
  - optional errno-derived suffix
- Preserve any flush-before-error behavior from the C implementation
- Preserve exit-on-status behavior if `error` or `error_at_line` terminates when given a nonzero status
- Port any duplicate-line suppression behavior in `error_at_line` only if it is present in `error.c`

Completion criteria:
- both public functions compile
- stderr output structure matches expected C behavior
- termination behavior is explicit and testable

## Phase 3: Integrate state handling and finalize type boundaries

Goals:
- resolve any remaining C-to-Rust signature mismatches
- minimize unsafe usage
- make ownership and borrowing explicit

Tasks:
- Convert any temporary string or integer placeholders into final Rust parameter types
- Isolate any unavoidable mutable global state behind the smallest safe abstraction possible
- Remove unnecessary allocations and simplify formatting paths
- Ensure helper visibility is private unless external use is required by the crate layout

Completion criteria:
- module compiles cleanly with final signatures
- unsafe code is absent or narrowly justified
- state handling matches original module behavior without adding features

## Phase 4: Add focused tests and complete migration verification

Goals:
- verify observable behavior of the port
- keep test coverage centered on the migrated functions only

Tasks:
- Add unit tests for:
  - stdout flush success path
  - errno message formatting path
  - `error` output without location
  - `error_at_line` output with file and line context
  - fatal status behavior if applicable
  - suppression behavior if present in the original C file
- Use output-capture patterns compatible with the existing crate test setup; avoid introducing unrelated test infrastructure
- Compare edge cases against the original C logic where formatting or status handling is subtle

Completion criteria:
- `cargo test` passes
- migrated functions are covered by targeted tests
- output and control-flow behavior are stable enough for branch integration

## Risks and Technical Notes

- C varargs-based formatting does not translate directly to Rust; the port should shift formatting responsibility to Rust call sites or use explicit string parameters while preserving emitted content.
- Exact stream-open checks from C may not have a direct safe Rust equivalent; only behavior used by this module should be reproduced.
- If the original file uses mutable global counters or line-suppression state, Rust implementation should preserve semantics with minimal safe state machinery rather than redesigning the API.
- Fatal error paths that exit the process must be tested carefully to avoid disrupting normal test execution; isolate such tests if the current repository already has an accepted pattern.
- Error reporting order matters more than abstraction quality in this migration; keep output logic linear and close to the original file.