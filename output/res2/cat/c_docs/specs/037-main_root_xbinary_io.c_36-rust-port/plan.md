# Implementation Plan

## Summary

Port `xbinary-io.c` into a Rust module that preserves the existing module boundary and behavior scope of the C source. The module appears to center on `xset_binary_mode_error`, so the Rust implementation should focus on representing the same side-effect and error-reporting path without adding unrelated abstractions.

The preferred approach is a small Rust module integrated into the existing `cat` crate structure, using the Rust standard library for platform-conditioned behavior and error propagation. Since this module surface is minimal, the port should keep logic close to the original control flow: evaluate whether binary mode setup can be performed, report failure in a Rust-idiomatic way, and expose a narrow function interface suitable for the surrounding main-cluster migration.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - No meaningful regression versus the C implementation for startup-path execution.
  - Zero unnecessary heap allocation in the ported function path.
  - Keep platform branching compile-time selected where possible.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xbinary-io.c` | `src/xbinary_io.rs` | Direct port of the file’s binary-mode/error helper logic. |
| `xset_binary_mode_error` | `pub(crate) fn xset_binary_mode_error(...)` or narrower internal function | Final signature should be chosen to match current call sites during migration, while keeping the name aligned to the source function. |

## Data Model

This module analysis does not list any C structs or custom data types.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| None identified | None required initially | Keep the module function-based unless migration of call sites proves a small enum/result type is necessary. |

If the original C function communicates failure through integer status codes or external error routines, map that to one of the following Rust forms based on actual usage during porting:

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Integer success/failure return | `Result<(), std::io::Error>` or `Result<(), ModuleError>` | Prefer `std::io::Error` if no richer domain error is needed. |
| Void function with process-global error reporting | `fn ... -> !` or `fn ... -> std::io::Error` feeding existing top-level reporting | Choose the narrowest option that matches current crate error flow. |
| Platform-specific mode flag handling | `#[cfg(windows)]` / `#[cfg(not(windows))]` branches | Avoid runtime dispatch when compile-time selection is sufficient. |

## Implementation Phases

### Phase 1: Establish module skeleton and interface
- Create `src/xbinary_io.rs`.
- Introduce the Rust counterpart for `xset_binary_mode_error`.
- Keep naming closely aligned with the C source to simplify review.
- Define the function signature only after checking how the surrounding `main_cluster` path expects to consume errors.
- Add the module to the crate using standard Rust module declarations, without introducing extra helper modules.

### Phase 2: Port core logic and platform handling
- Translate the body of `xset_binary_mode_error` directly from C control flow into Rust.
- Use standard-library facilities and conditional compilation for platform-specific behavior.
- Preserve observable behavior of the original function:
  - same success/failure decision points,
  - same narrow responsibility,
  - no new fallback behavior.
- Replace any C-style manual error propagation with Rust return values where possible, but do not widen the module API beyond what migration requires.
- Ensure there is no unsafe code unless the original behavior cannot be expressed otherwise; if unavoidable, isolate it to the smallest possible block and document the reason inline.

### Phase 3: Integrate with callers and normalize error handling
- Update existing or ported call sites in the main execution path to use the Rust function.
- Align error text and exit/reporting behavior with the existing crate conventions, but do not create new reporting infrastructure just for this module.
- Confirm that memory ownership is trivial and stack-based, with no retained buffers or global mutable state introduced by the port.
- Keep the module private or `pub(crate)` unless broader visibility is already required by the current project layout.

### Phase 4: Add focused tests and complete verification
- Add unit tests for platform-relevant behavior that can be checked without over-mocking.
- Prefer testing return-value/error-shape behavior rather than inventing integration scaffolding.
- Run `cargo test` to validate compile-time platform branching and caller integration.
- Verify that the final Rust module fully replaces the functionality of `xbinary-io.c` within the current migration branch scope.