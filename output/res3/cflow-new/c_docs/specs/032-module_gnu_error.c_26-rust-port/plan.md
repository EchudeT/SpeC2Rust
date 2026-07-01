# Implementation Plan: module_gnu_error.c_26

## Summary

This module ports `gnu/error.c` into a focused Rust implementation that preserves the existing responsibilities of formatted error reporting, optional line-based location reporting, stdout flushing before stderr output, and process termination behavior when requested by the caller.

The Rust implementation should remain narrow in scope and map the current C file into a single Rust module with function-level equivalents for:

- open-stream detection used by stdout flushing logic
- stdout flush handling
- errno/message formatting
- generic error emission
- file/line-aware error emission

The technical approach is to implement these behaviors with the Rust standard library first:

- `std::io::{stdout, stderr, Write}` for output and flushing
- `std::process::exit` for termination paths
- `std::io::Error` / raw OS error codes for errno-style reporting
- formatting via `format!` / `writeln!`

Because the source module has process-global reporting behavior in C, the Rust port should keep equivalent module-level behavior and avoid introducing extra abstraction layers unless required by the existing implementation details in `gnu/error.c`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are required based on the provided module scope
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep reporting paths lightweight and allocation-minimal beyond unavoidable formatting
  - Preserve immediate write/flush behavior for stderr/stdout interactions
  - Avoid additional buffering layers beyond standard stream handling
  - Maintain behavior comparable to the C implementation for normal CLI-style error paths

## Module Mapping

### C to Rust File Mapping

- `gnu/error.c` → `src/module_gnu_error.rs`

If the project already uses a module tree matching source origins, this may instead be:

- `gnu/error.c` → `src/gnu/error.rs`

The final location should follow the existing crate layout, but the migration should remain a single Rust source file corresponding to the original C file.

### Function Mapping

- `is_open` → `fn is_open(...) -> bool`
  - Implement only to the degree needed by `flush_stdout`
  - Prefer standard-library-driven behavior rather than reproducing low-level descriptor probing unless the original behavior requires it

- `flush_stdout` → `fn flush_stdout()`
  - Flush `stdout`
  - Preserve the original error-handling intent when flush fails

- `print_errno_message` → `fn print_errno_message(...)`
  - Build the errno-derived trailing message portion
  - Accept either a raw OS error code or `std::io::Error`-compatible input, depending on the surrounding port shape

- `error` → `pub fn error(...) -> !` or `pub fn error(...)`
  - Final signature depends on whether nonzero status always exits in the C source logic
  - If behavior is conditional, keep a normal return type and call `process::exit` only on exit paths

- `error_at_line` → `pub fn error_at_line(...) -> !` or `pub fn error_at_line(...)`
  - Same exit-path treatment as `error`
  - Include filename and line number formatting in the emitted message

## Data Model

No explicit C structs were provided for this module, so the Rust port should avoid introducing new data structures unless the original file contains module-global state that must be preserved.

### Data-Structure Mapping

- **C global/static state** → **Rust module-level static or simple module-private state**
  - Only if present in `gnu/error.c`
  - Use the narrowest Rust representation needed for parity

- **C errno integer / error indicator** → **`i32` raw OS error code or `std::io::Error`**
  - Prefer `i32` when preserving direct C call patterns
  - Convert to display text using `std::io::Error::from_raw_os_error`

- **C string pointers (`char *`, `const char *`)** → **`&str`**
  - For internal Rust callers
  - If the surrounding crate still passes byte-oriented data, use `&CStr` or `&[u8]` only where already required by existing migration constraints

- **C line number integer** → **`u32` or `usize`**
  - Choose based on the original semantic range in the source
  - `u32` is usually the closest intent for source line reporting

## Implementation Phases

### Phase 1: Establish module skeleton and direct function signatures

- Create the Rust module file corresponding to `gnu/error.c`
- Add Rust equivalents for:
  - `is_open`
  - `flush_stdout`
  - `print_errno_message`
  - `error`
  - `error_at_line`
- Decide signatures based on actual call sites and existing port conventions in the crate
- Keep functions module-local unless they are used outside the translated file in the current project structure
- Map C string/number inputs to minimal Rust equivalents without adding wrapper types

### Phase 2: Port output, formatting, and exit behavior

- Implement stdout flush behavior before error emission where required
- Emit messages to stderr using standard formatting macros
- Reproduce message composition order from the C implementation:
  - program/component prefix if present in source
  - main formatted message
  - errno-derived suffix when applicable
  - newline termination
- Implement line-aware formatting in `error_at_line`
- Preserve conditional termination behavior using `std::process::exit`
- Ensure failure handling during stream writes does not introduce panics beyond what is unavoidable for process-exit paths

### Phase 3: Handle errno-style translation and C semantic edge cases

- Map raw OS error codes to displayable messages using `std::io::Error::from_raw_os_error`
- Reconcile any C-specific distinctions such as:
  - zero vs nonzero status
  - absence of errno
  - optional file/line fields
  - flush behavior when stdout is not meaningfully open
- Keep memory ownership simple by borrowing string inputs wherever possible
- Avoid unsafe code unless the existing crate interface forces interaction with C-style strings or descriptors

### Phase 4: Add focused tests and finalize module integration

- Add unit tests for:
  - basic stderr message formatting
  - errno suffix rendering
  - file/line message formatting
  - non-exit vs exit-triggering branches where testable
- Prefer extracting pure formatting helpers for testability if needed, without expanding the public API
- Integrate the module into the crate module tree
- Run `cargo test` and fix any signature mismatches with adjacent translated modules

## Notes on Memory Management and Error Handling

- Use borrowed Rust string slices wherever possible to avoid unnecessary allocation
- Limit owned `String` creation to final composed messages or helper formatting paths
- Avoid retaining references to temporary formatted data beyond statement scope
- Treat stderr/stdout operations as fallible I/O, but keep behavior aligned with the original reporting role rather than building recovery layers
- Use `std::process::exit` only in the same logical cases as the C implementation, so callers preserve existing control-flow expectations