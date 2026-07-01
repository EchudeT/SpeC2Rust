# Implementation Plan

## Summary

Port `gnu/xalloc-die.c` into a focused Rust module that preserves the original module’s narrow role: terminating execution after allocation failure handling is invoked. The Rust implementation should keep this behavior minimal and explicit, using standard-library process termination and error-reporting facilities rather than introducing broader allocation abstractions.

The technical approach is to migrate the single C entry point `xalloc_die` into a Rust function with equivalent no-return behavior. The implementation should remain small, isolated, and callable from the rest of the ported codebase wherever fatal allocation failure must be reported. Any message emission should use standard error output and then terminate the process with a failure exit status or equivalent immediate termination behavior, depending on surrounding project conventions already established in the Rust port.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Negligible runtime overhead compared with the C implementation
  - Constant-time execution for the failure path
  - No additional heap allocation in the fatal path where reasonably avoidable
  - Preserve direct termination semantics without recovery layers

## Module Mapping

- **C source file**
  - `gnu/xalloc-die.c`

- **Rust target**
  - `src/gnu/xalloc_die.rs`

- **Function mapping**
  - `xalloc_die` -> `pub fn xalloc_die() -> !`

- **Module exposure**
  - If the project already exposes GNU compatibility helpers under a `gnu` namespace, add:
    - `src/gnu/mod.rs` -> `pub mod xalloc_die;`
  - If an existing module file already declares sibling GNU ports, only append the minimal declaration needed for this file.

## Data Model

This module contains no owned data structures and no persistent state.

- **C structs/enums**
  - None

- **Rust mapping**
  - No struct or enum required
  - Function-only module

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/gnu/xalloc_die.rs`.
- Declare the Rust equivalent of the C entry point:
  - `pub fn xalloc_die() -> !`
- Register the module in `src/gnu/mod.rs` only if that declaration is not already present.
- Keep the file scoped strictly to this migrated function; do not introduce shared allocation utilities or new error types.

### Phase 2: Port termination behavior

- Implement the function as a fatal-path routine with diverging return type `!`.
- Use standard-library facilities for:
  - writing the failure message to standard error, if the original surrounding port expects visible diagnostics
  - terminating the process immediately afterward
- Favor implementation that avoids unnecessary allocation on the error path.
- Preserve the semantic expectation that callers do not continue after invocation.
- If surrounding migrated GNU helpers already define exact failure wording or exit code conventions, align with those rather than introducing new policy in this module.

### Phase 3: Integrate call sites and module references

- Replace uses of the C symbol with the Rust function in already ported code that depends on allocation-failure termination.
- Ensure call sites accept the diverging return type cleanly and do not expect a return value.
- Remove or stop referencing the original C file from the Rust build path, while keeping migration localized to this module branch.

### Phase 4: Add focused tests

- Add tests only for behavior that can be validated safely within `cargo test`.
- Prefer validating compile-time and interface properties over direct in-process termination checks.
- If termination behavior must be checked, use a subprocess-based test pattern so the test harness itself is not aborted.
- Verify:
  - the function is callable from the expected module path
  - invocation results in process failure semantics
  - any emitted diagnostic is written to standard error if the project requires it