# Implementation Plan

## Summary

This module ports the C file `xalloc-die.c` into Rust for the `pwd` project branch `026-main_root_xalloc_die.c_26-rust-port`. The scope is limited to migrating the existing `xalloc_die` behavior into an idiomatic Rust implementation that preserves process-terminating semantics used after unrecoverable allocation failure.

The Rust approach should keep the implementation minimal and aligned with the original role of the C module: provide a single termination path for allocation-failure handling, without introducing broader allocation frameworks or recovery layers. The implementation should rely on the Rust standard library for message emission and process termination, with careful handling of divergence (`-> !`) to model the non-returning C function precisely.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Negligible overhead relative to the C implementation
  - Constant-time control flow for the termination path
  - No heap allocation required in the failure-handling path where avoidable
  - Preserve immediate process exit behavior for fatal allocation errors

## Module Mapping

- **C source file**: `xalloc-die.c`
- **C function**: `xalloc_die`
- **Rust target module**: `src/xalloc_die.rs`
- **Rust public API**:
  - `pub fn xalloc_die() -> !`

If the crate currently centralizes top-level module declarations in `src/lib.rs` or `src/main.rs`, add only the minimal module exposure needed for existing call sites to use `xalloc_die`.

## Data Model

This module has no declared C structs or persistent data structures to migrate.

### Data-structure Mapping

- **C structs**: none
- **Rust structs/enums**: none required

### Behavioral Mapping

- **C non-returning fatal function** -> **Rust diverging function**
  - `void xalloc_die(void)` -> `pub fn xalloc_die() -> !`

This mapping is important because Rust’s `!` return type communicates the same control-flow guarantee as the C routine’s fatal termination behavior.

## Implementation Phases

### Phase 1: Establish the Rust module skeleton

- Create `src/xalloc_die.rs`.
- Add the Rust definition for `xalloc_die` with a diverging signature: `-> !`.
- Wire the module into the existing crate structure using the smallest necessary change in `src/lib.rs` or `src/main.rs`.
- Keep naming close to the C source to simplify migration tracing.

**Completion check**:
- The crate builds with the new module file present.
- The symbol is reachable from intended internal call sites.

### Phase 2: Port fatal allocation-exit behavior

- Implement the body of `xalloc_die` using standard library facilities only.
- Preserve the original module’s role as a terminal error path for allocation failure.
- Prefer direct stderr reporting plus process termination, or an equivalent standard-library fatal path, depending on how the surrounding port handles fatal diagnostics.
- Ensure the function does not return under any path.
- Avoid introducing recoverable `Result`-based behavior in this module, since the original contract is fatal termination.

**Memory and error-handling notes**:
- Avoid extra heap-dependent formatting in the allocation-failure path where practical.
- Do not unwrap fallible operations in a way that obscures the intended fatal behavior; the function itself is already the terminal path.
- Keep ownership concerns trivial by using borrowed static strings or minimal formatting.

**Completion check**:
- `xalloc_die` compiles with `-> !`.
- The implementation clearly terminates the process in all cases.

### Phase 3: Migrate and align call sites

- Update existing Rust-ported call sites, if any in this branch, to use the new `xalloc_die` function rather than placeholder termination logic.
- Preserve original control flow from the C code: locations that called `xalloc_die` should still terminate rather than propagate errors.
- Keep the migration narrowly scoped to direct usage alignment; do not generalize into a new error subsystem.

**Completion check**:
- All current uses of allocation-failure termination in the migrated area refer to the shared module function.
- No duplicate local fatal-allocation helpers remain in the affected scope.

### Phase 4: Add focused tests and validation

- Add unit tests only for aspects that can be checked safely without terminating the test runner directly.
- Prefer compile-time or signature-level validation over runtime termination tests.
- If needed, use a subprocess-style integration test to verify exit behavior, but only if the crate already uses that pattern; otherwise keep tests minimal.
- Validate:
  - function visibility and module integration
  - diverging function signature compatibility in call sites
  - successful crate build and `cargo test`

**Completion check**:
- `cargo test` passes.
- The module integration is verified without adding unrelated test infrastructure.