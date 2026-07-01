# Implementation Plan: module_doc_whoami.c_06

## Summary

This module ports `doc/whoami.c` into Rust with a narrow migration scope centered on the existing `who_am_i` function. The Rust implementation should preserve the current behavior and control flow of the C source while replacing manual memory handling and C-style string processing with safe standard-library equivalents where possible.

The technical approach is to create a single Rust module corresponding to `doc/whoami.c`, implement `who_am_i` with explicit result-based error handling, and map any anonymous C data structure usage into a private Rust struct only if the source logic requires retained state. The port should avoid introducing new abstractions or functionality beyond what is necessary to reproduce the current module behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent runtime characteristics for the migrated function.
  - Avoid unnecessary heap allocation beyond what is required by Rust string and path handling.
  - Keep I/O and lookup logic direct, with no added caching or background processing.
  - Prefer zero-copy borrowing for internal processing when source data lifetime permits.

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `doc/whoami.c` | `src/module_doc_whoami.rs` | Single-module port containing the Rust implementation of `who_am_i`. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `who_am_i` | `pub(crate) fn who_am_i(...) -> Result<..., ...>` or `pub(crate) fn who_am_i(...)` | Final signature should be chosen from the C behavior: use `Result` if the C function can fail or returns status; use a direct return only if failure is impossible or already encoded in the return value. Preserve existing call semantics as closely as practical. |

## Data Model

The analysis reports an `anonymous` data structure. Since the source module contains only one listed function and no named exported types, the Rust mapping should remain minimal.

### Data Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | Private `struct` or local tuple/temporary bindings | Introduce a named private Rust struct only if the anonymous C structure stores multiple related fields across steps in `who_am_i`. If it is only used for short-lived grouping, prefer local variables. |

### Memory Management Mapping

- Replace stack or heap buffers from C with:
  - `String` for owned text
  - `&str` for borrowed text
  - `PathBuf` or `OsString` if the original logic is path- or OS-string-oriented
- Eliminate manual allocation and deallocation.
- Avoid exposing raw pointers unless unavoidable for compatibility with surrounding migrated code.
- Keep ownership local to the function unless the original module clearly shares state.

### Error Handling Mapping

- Convert integer status codes, null checks, and sentinel returns into:
  - `Result<T, E>` for recoverable failure
  - `Option<T>` only if absence is the sole non-success case
- If the C code distinguishes several failure branches, define a small private error enum for this module rather than collapsing failures into strings.
- Preserve observable failure behavior at the module boundary as closely as possible.

## Implementation Phases

## Phase 1: Source Audit and Interface Definition

- Inspect `doc/whoami.c` and identify:
  - exact signature of `who_am_i`
  - all inputs, outputs, and side effects
  - whether the anonymous structure is persistent or purely local
  - all external library or project-local functions referenced by the module
- Define the Rust module file `src/module_doc_whoami.rs`.
- Translate the C function signature into a Rust signature that preserves behavior while using Rust ownership and borrowing rules.
- Decide the return shape:
  - direct value if the C function returns a computed identity string/value
  - `Result` if failures are represented in the C implementation
  - output-parameter replacement with a return value if C writes through pointers

## Phase 2: Core Function Port

- Implement `who_am_i` in Rust following the C control flow closely.
- Migrate C string and buffer operations to standard-library string/path operations.
- Replace null-pointer and length checks with typed validation.
- If the anonymous structure is required, define a private Rust struct within the module and map field usage directly.
- Keep helper logic inline unless extraction is necessary to mirror distinct C-local operations cleanly.

## Phase 3: Error and Boundary Alignment

- Convert all C failure paths into explicit Rust error returns.
- Ensure no unchecked indexing, invalid UTF-8 assumptions, or lifetime leaks are introduced during the port.
- If the original C code interacts with environment, filesystem, or process identity APIs, wrap these accesses with standard-library calls and preserve the original precedence and fallback order.
- Verify that observable outputs, formatting, and failure cases remain aligned with the original implementation.

## Phase 4: Tests and Final Integration

- Add unit tests covering:
  - normal successful `who_am_i` execution
  - failure or empty-result paths present in the C code
  - edge conditions driven by buffer size, missing inputs, or missing external state if applicable
- Use focused tests derived from the original function behavior, without adding new feature scenarios.
- Run `cargo test` and fix any behavioral mismatches against the source module.
- Confirm the final Rust file fully replaces the logic from `doc/whoami.c` for this module scope only.