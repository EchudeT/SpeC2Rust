# Implementation Plan

## Summary

Port `gnu/getdtablesize.c` into a Rust module that preserves the existing module boundary and behavior centered on `_setmaxstdio_nothrow`. The Rust implementation should remain narrow in scope: migrate the existing function into an idiomatic Rust function with explicit error handling and platform-aware conditional compilation where needed.

The technical approach is to:
- map the single C source file to a single Rust module,
- preserve the function’s observable contract as closely as possible,
- use standard-library facilities first,
- isolate any platform-specific behavior behind small internal helpers,
- avoid introducing new abstraction layers beyond what is needed for a direct migration.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided module scope
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain constant-time behavior for the migrated function
  - Avoid heap allocation
  - Keep call overhead minimal and comparable to the original C implementation
  - Preserve low-level behavior relevant to descriptor/stdio limit handling without adding extra indirection

## Module Mapping

### C to Rust File Mapping

- `gnu/getdtablesize.c` -> `src/module_gnu_getdtablesize.rs`

### Function Mapping

- `_setmaxstdio_nothrow` -> `pub(crate)` or private Rust function in `src/module_gnu_getdtablesize.rs`, depending on actual crate-internal call sites

### Rust Module Placement

Use standard Rust layout with a single module file registered from `src/lib.rs`:

- `src/lib.rs`
- `src/module_gnu_getdtablesize.rs`

If the surrounding project already uses a module tree, place this file within that existing structure without creating extra layers.

## Data Model

The analysis lists only an anonymous data structure and no named persistent C struct. The migration should therefore avoid inventing replacement domain types unless required by the function body.

### Data-Structure Mapping

- anonymous C data structure -> local Rust variables / tuples / small private helper types only if necessary

### Type Mapping Guidance

For the function migration, use direct scalar mappings:

- `int` -> `i32` when preserving exact C-facing semantics inside the module
- `unsigned int` -> `u32` if present in the original function logic
- C boolean-style return/status values -> `Result<_, _>` internally where practical, with final return shape matching the expected crate API
- sentinel error values from C -> explicit Rust error branches or checked integer handling

### Memory Management

- Prefer stack-only values
- No manual allocation
- No raw-pointer ownership unless the original function requires direct system interaction
- If any unsafe system interaction is necessary, keep it tightly scoped and document the invariant at the call site

### Error Handling

- Replace silent C-style failure paths with explicit internal handling
- Preserve nothrow behavior by avoiding panics in normal error paths
- Convert platform/system call failures into stable return values expected by the crate’s API rather than expanding functionality

## Implementation Phases

## Phase 1: Inspect and Define the Rust Surface

- Examine `gnu/getdtablesize.c` and confirm the exact signature and return contract of `_setmaxstdio_nothrow`
- Identify any compile-time platform assumptions in the C source
- Create `src/module_gnu_getdtablesize.rs`
- Register the module in `src/lib.rs`
- Define the Rust function signature to match existing crate usage as closely as possible
- Decide visibility (`pub(crate)` vs private) based only on actual internal callers

## Phase 2: Port Core Logic Directly

- Translate `_setmaxstdio_nothrow` into Rust with a line-by-line behavioral mapping
- Keep integer conversions explicit to avoid accidental semantic changes
- Encapsulate any platform-specific branches with `cfg` gates if required by the original C behavior
- Use small internal helpers only where needed to express system-limit logic clearly
- Keep unsafe code out unless strictly required; if required, minimize its scope

## Phase 3: Validate Error Semantics and Edge Cases

- Verify that invalid inputs, boundary values, and system-call failure cases preserve the intended “nothrow” behavior
- Add unit tests for:
  - nominal input handling
  - lower and upper boundary values relevant to stdio/file descriptor limits
  - failure-path behavior without panicking
- Ensure tests use `cargo test` and do not depend on expanded infrastructure

## Phase 4: Final Integration Review

- Confirm module naming and placement align with the rest of the Rust project on branch `035-module_gnu_getdtablesize.c_29-rust-port`
- Remove any unnecessary compatibility scaffolding introduced during porting
- Check for clippy-relevant issues such as unchecked casts or avoidable `unsafe`
- Verify the implementation remains limited to the original module responsibilities