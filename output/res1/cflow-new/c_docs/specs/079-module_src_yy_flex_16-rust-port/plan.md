# Implementation Plan: module_src_yy_flex_16

## Summary

This module is a narrow port of the `src/c.c` utility slice that provides the helper routines `yy_flex_strncpy` and `yy_flex_strlen`. The Rust implementation should preserve current behavior and call patterns while replacing manual C string traversal and buffer copying with explicit, bounded Rust logic.

The implementation approach is to migrate the functionality into a small Rust module that:
- keeps the scope limited to these two functions,
- models C-style string handling using byte slices and/or `CStr`-compatible inputs where required by surrounding code,
- preserves null-termination and bounded-copy semantics expected by the original helpers,
- avoids introducing broader abstractions beyond what is needed to replace the existing file content.

The port should favor safe Rust where possible. Any unavoidable low-level handling should be isolated to small functions with documented preconditions. Memory ownership should remain explicit: length calculation is read-only, and bounded copy writes only into caller-provided buffers.

## Technical Context

### Language / Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended from the available evidence

### Testing
- `cargo test`

### Performance Goals
- Match the original C helper behavior with no unnecessary allocations
- Keep string length calculation linear in input size
- Keep bounded copy linear in the copied byte count
- Use direct slice/byte operations so overhead remains comparable to the original C utility functions

## Module Mapping

### C to Rust File Mapping
- `src/c.c` -> `src/module_src_yy_flex_16.rs`

### Function Mapping
- `yy_flex_strlen` -> `pub(crate) fn yy_flex_strlen(...) -> usize`
- `yy_flex_strncpy` -> `pub(crate) fn yy_flex_strncpy(...) -> ...`

### Rust Module Placement
- Declare the migrated code as a single focused Rust module under standard `src/`
- Re-export only if required by existing crate-internal callers
- Do not split into additional helper modules unless needed to satisfy borrow or visibility constraints during migration

## Data Model

The analysis lists only anonymous C data structures and the function set for this module is limited to string helpers. There is no evidence that these anonymous structures are owned or defined by this specific migration slice. The plan should therefore avoid inventing Rust data types that are not directly required.

### Data-Structure Mapping
- anonymous C data structures not directly referenced by `yy_flex_strncpy` / `yy_flex_strlen` -> no dedicated Rust struct at this module stage
- C string pointer inputs -> Rust byte-oriented views:
  - `*const c_char` / null-terminated source semantics -> `&CStr` or documented raw-pointer access at the boundary
  - destination character buffer -> `&mut [u8]` or equivalent internal representation matching caller constraints

### Representation Decisions
- Prefer slice-based internal logic after validating or converting inputs
- Use `usize` for lengths and indexes
- Treat null termination explicitly rather than relying on implicit C behavior
- If raw pointers are required by surrounding port compatibility, keep them at the function boundary and convert immediately into bounded operations

## Implementation Phases

## Phase 1: Establish module skeleton and function signatures

### Goal
Create the Rust file and define the minimal API surface needed to replace the C helpers.

### Tasks
- Add `src/module_src_yy_flex_16.rs`
- Introduce Rust equivalents for:
  - `yy_flex_strlen`
  - `yy_flex_strncpy`
- Choose signatures based on actual crate integration needs:
  - prefer slice-based signatures internally,
  - retain raw-pointer-compatible wrappers only if existing migrated callers require them
- Add module declaration in the crate root or parent module where `src/c.c` functionality is being relocated

### Notes
- Keep visibility restricted to `pub(crate)` unless broader exposure is already required
- Do not add unrelated utility functions

## Phase 2: Port core logic with bounded memory handling

### Goal
Translate the C string operations into Rust while preserving semantics and avoiding undefined behavior.

### Tasks
- Implement `yy_flex_strlen` as a null-terminated byte scan or equivalent compatible logic
- Implement `yy_flex_strncpy` with explicit bounded copy behavior:
  - copy at most the requested count,
  - preserve expected null-byte handling from the original C routine,
  - avoid writing past the destination buffer
- Document preconditions for any raw-pointer entry points
- Isolate `unsafe` to the smallest possible region if raw pointers are unavoidable

### Memory and Error-Handling Decisions
- Do not allocate for these helper operations
- Treat destination capacity as an explicit contract
- Prefer returning plain values if the C code assumes infallible helpers
- If signature adaptation requires validation, use simple result types only where the surrounding migrated code can consume them without widening scope

## Phase 3: Integrate with callers and remove C dependency for this slice

### Goal
Wire the Rust helpers into the in-progress port and ensure they replace the original C implementations cleanly.

### Tasks
- Update existing Rust call sites on branch `079-module_src_yy_flex_16-rust-port` to use the new module
- Verify that call patterns preserve:
  - source string interpretation,
  - destination buffer sizing assumptions,
  - returned length/copy semantics
- Remove or stop referencing the corresponding helper logic from the C translation path for this module slice only

### Notes
- Keep migration localized to `src/c.c` functionality identified in this analysis
- Do not broaden integration into unrelated lexer/runtime code beyond necessary call replacement

## Phase 4: Add focused tests and finalize behavior parity

### Goal
Lock in behavior parity for the two helpers with small, direct unit tests.

### Tasks
- Add unit tests for `yy_flex_strlen`:
  - empty string
  - normal ASCII content
  - early null termination behavior if applicable at the boundary
- Add unit tests for `yy_flex_strncpy`:
  - copy shorter than bound
  - copy equal to bound
  - truncation at bound
  - destination null-byte behavior consistent with the C implementation
- Run `cargo test` and fix any signature mismatches uncovered during integration

### Acceptance Criteria
- Both helper functions are implemented in Rust
- The migrated module compiles cleanly
- Tests cover boundary conditions relevant to C-style string handling
- No extra facilities or abstractions were introduced beyond this file-level migration