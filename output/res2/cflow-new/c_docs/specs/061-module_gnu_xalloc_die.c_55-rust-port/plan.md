# Implementation Plan

## Summary

This module is a narrow port of `gnu/xalloc-die.c`, centered on the `xalloc_die` routine that terminates execution after allocation failure reporting has already been prepared by the caller or surrounding allocation helpers.

The Rust implementation should keep the same operational scope: provide a single module-level function that unconditionally reports a fatal allocation condition and exits the process with a failure status. The port should favor Rust standard library facilities for process termination and stderr output, while preserving the original non-recovering control flow semantics as closely as practical in safe Rust.

The implementation should remain minimal and aligned to the existing file/function boundary, avoiding any expansion into generalized allocation frameworks or additional error infrastructure.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time execution path for the fatal routine
  - No heap allocation required in the fatal path beyond unavoidable standard-library internals
  - Preserve immediate process termination behavior without introducing retry or recovery logic

## Module Mapping

- **C source file**
  - `gnu/xalloc-die.c`
- **Rust target module**
  - `src/gnu/xalloc_die.rs`

### Function mapping

- `xalloc_die` -> `pub fn xalloc_die() -> !`

### Suggested crate module wiring

- Declare `pub mod gnu;` in `src/lib.rs` or `src/main.rs`, matching the current crate shape
- Declare `pub mod xalloc_die;` in `src/gnu/mod.rs`

This mapping should stay limited to the existing source file and function, without introducing adjacent utility modules unless already required by the crate layout.

## Data Model

This module has no owned C data structures to port.

### Data-structure mapping

- **C structs/enums**: none
- **Rust structs/enums**: none required

### Semantic model notes

- The C function’s “does not return” behavior should map to Rust’s never type:
  - `fn xalloc_die() -> !`
- Any C global-state interaction that may have existed indirectly through stderr/process exit should be handled via Rust standard library calls rather than explicit data modeling.

## Implementation Phases

### Phase 1: Establish module file and public function boundary

- Create `src/gnu/xalloc_die.rs`
- Add the Rust signature for the migrated routine:
  - `pub fn xalloc_die() -> !`
- Wire the module into the existing crate module tree with the smallest possible change set
- Keep the function body initially minimal but terminating, so the crate compiles while preserving the non-returning contract

### Phase 2: Port fatal reporting and termination behavior

- Implement stderr reporting in the function body using standard library I/O
- Implement unconditional failure termination using `std::process::exit`
- Keep behavior narrow:
  - no returned `Result`
  - no recovery branch
  - no configurable exit handling
- Ensure the implementation does not rely on heap-managed state owned by this module
- If the original C behavior depends on a fixed diagnostic string, encode that as a module-local constant string slice

### Phase 3: Align error-handling semantics with C behavior

- Verify that the Rust function is used only as a terminal path and that its `-> !` signature is compatible with call sites
- Check that stderr emission is best-effort and that termination remains unconditional even if writing fails
- Avoid introducing panic-based termination unless required by existing project conventions; prefer explicit process exit to mirror C semantics more closely
- Confirm that no extra abstraction layers are added around allocation failure handling

### Phase 4: Add focused tests and finalize migration

- Add unit tests for any pure, non-terminating internal details only if such details exist after the port
- For the terminating routine itself, prefer a subprocess-style integration test only if the crate already uses that pattern; otherwise keep tests minimal and compile-focused
- Run `cargo test` to verify module integration and signature correctness
- Remove or retire the original C implementation from active use once the Rust path is wired in on branch `061-module_gnu_xalloc_die.c_55-rust-port`

## Migration Notes

- Memory management is trivial in this module because it owns no heap structures; the main concern is preserving terminal control flow.
- Error handling should remain intentionally non-recoverable.
- The Rust implementation should not broaden the original API surface beyond the single migrated function.
- Any formatting or message text should remain local to the module and not motivate creation of shared diagnostics helpers unless such helpers already exist in the crate.