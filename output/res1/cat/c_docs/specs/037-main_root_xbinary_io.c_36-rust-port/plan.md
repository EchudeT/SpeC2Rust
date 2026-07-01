# Implementation Plan: `main_root_xbinary-io.c_36`

## Summary

This module is a small platform-adaptation unit centered on `xset_binary_mode_error`, which exists to report failure when switching a stream or file descriptor into binary mode. The Rust port should keep the implementation minimal and aligned with the existing project structure: migrate the behavior into a single Rust module that exposes one function with equivalent responsibility and integrates with the crate’s existing error-reporting path.

The technical approach is to replace C-style manual error propagation and platform-specific binary-mode handling with a Rust function that:
- accepts the same logical inputs as needed by the current call sites,
- uses standard library error types and formatting,
- preserves observable behavior at the call boundary, especially message construction and failure signaling,
- avoids introducing new abstraction layers beyond what is required to host the migrated function.

Because Rust’s standard I/O model generally does not require explicit binary/text mode switching on Unix and handles Windows differently through higher-level APIs, the port should focus on reproducing the reporting behavior rather than recreating unnecessary low-level mode toggling machinery inside this module.

## Technical Context

### Language/Version
- Rust stable
- Recommended minimum version: **Rust 1.74+**

### Primary Dependencies
- Rust standard library only:
  - `std::io`
  - `std::fmt`
  - `std::path` only if required by existing call signatures
- No third-party crates are recommended, since the input provides no evidence of external dependency needs.

### Testing
- `cargo test`

### Performance Goals
- Negligible overhead relative to the C version.
- No heap allocation beyond what is inherently needed for error message formatting.
- No additional buffering, I/O layers, or stateful wrappers introduced by this migration.
- Preserve straightforward call flow suitable for a startup or command-path utility function.

## Module Mapping

### C to Rust File Mapping
- `xbinary-io.c` -> `src/xbinary_io.rs`

### Function Mapping
- `xset_binary_mode_error` -> `pub(crate) fn xset_binary_mode_error(...)`

### Integration Mapping
- Expose the Rust function within the crate using standard module declarations from the existing root module layout.
- Keep the implementation in a single Rust source file unless the current crate layout already requires a different placement.
- Do not split reporting helpers into extra modules unless this is already established by the surrounding port structure.

## Data Model

This module analysis lists no module-specific C structs or persistent data objects.

### Data-Structure Mapping
- No C struct/union/enum requires migration for this module.
- C scalar inputs used by `xset_binary_mode_error` should be mapped directly to Rust primitives or borrowed string types based on the actual call signature:
  - C string pointers used for program name, file name, or mode label -> `&str` where valid UTF-8 is already guaranteed by the surrounding port, otherwise `&OsStr` or `&Path` only if required by existing interfaces.
  - C integer status or file descriptor values -> `i32` or the narrowest equivalent matching caller expectations.
  - C `errno`-style error context -> `std::io::Error` or `std::io::ErrorKind`, preferring `std::io::Error` when preserving OS error detail is needed.

### Memory Management
- Replace any C ownership/manual lifetime concerns with borrowed parameters where possible.
- Avoid storing references or constructing long-lived state; this function should remain stateless.
- Use stack-based formatting and immediate emission/return paths.

### Error Handling
- Replace implicit `errno` access and C-side reporting conventions with explicit Rust error values passed in or captured at the point of failure.
- If the surrounding crate convention is “report and exit,” this function should align with that convention rather than inventing a new `Result` layer.
- If the surrounding crate convention is “return error to caller,” return `io::Result<()>` or a crate-local error type only if already used nearby. Do not create a new error framework solely for this module.

## Implementation Phases

### Phase 1: Inspect and Define Rust Interface
- Review `xbinary-io.c` and all call sites of `xset_binary_mode_error`.
- Identify the exact argument contract:
  - whether it receives a file descriptor, stream name, operation label, and/or status,
  - whether it prints directly, returns status, or terminates.
- Define the Rust function signature to match actual usage with minimal adaptation.
- Add the target Rust module file and wire it into the crate module tree.

### Phase 2: Port Core Reporting Logic
- Reimplement `xset_binary_mode_error` in Rust with behavior equivalent to the C function.
- Map C formatting and error-reporting behavior to `eprintln!`, crate-local diagnostics, or existing reporting helpers already present in the port.
- Preserve platform-sensitive behavior only where observable by callers; do not recreate unnecessary text/binary mode machinery if the function’s role is only to report failure.
- Ensure OS error details are preserved when available via `std::io::Error`.

### Phase 3: Integrate With Existing Main-Cluster Code
- Update callers to use the Rust function signature.
- Remove any temporary compatibility shims created during migration.
- Verify that module visibility is limited to what the current crate structure requires (`pub(crate)` preferred over `pub` unless external exposure is already necessary).
- Confirm no extra helper modules or utility layers were introduced beyond this file migration.

### Phase 4: Validate Behavior With Tests
- Add focused unit tests for message construction or return behavior if the implementation is directly testable.
- Add integration coverage through the existing command-path tests if this function is only exercised indirectly.
- Validate platform-conditional compilation if Windows-specific paths are present.
- Run `cargo test` and fix any signature or error-type mismatches uncovered during integration.