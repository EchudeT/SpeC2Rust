# Implementation Plan: module_src_output.c_28

## Summary

This module ports `src/output.c` into Rust with a narrow migration scope centered on the existing `set_active` and `output` functions. The Rust implementation should preserve the current control flow and output behavior while replacing C-style global/stateful patterns, raw pointers, and implicit ownership with explicit Rust data ownership and borrowing.

The implementation approach is to move the module logic into a single Rust source module that mirrors the original file boundary, keep state transitions explicit, and represent the C anonymous structures with named Rust structs or enums only as needed to support the two migrated functions. Output paths should use standard library I/O traits and buffers, with fallible operations surfaced through `Result` rather than hidden status conventions where practical. The migration should avoid adding capabilities beyond what is required to support the original file and functions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the original module’s asymptotic behavior.
  - Avoid unnecessary heap allocation during output generation.
  - Prefer borrowing and stack-based state where possible.
  - Keep I/O buffering explicit enough to avoid excessive small writes if the C module currently emits incrementally.

## Module Mapping

- **C source file**: `src/output.c`
- **Rust module target**: `src/output.rs`

### Function Mapping

- `set_active` → `pub(crate) fn set_active(...) -> ...`
  - Port as a focused state-update function.
  - Replace implicit/global mutation with explicit mutation of a module state struct or passed-in context.
  - If the C function cannot fail, keep a non-`Result` signature; otherwise convert failure paths to `Result`.

- `output` → `pub(crate) fn output(...) -> Result<..., ...>`
  - Port as the main emission routine for this module.
  - Use `std::io::Write` if the original function writes to streams/files.
  - Convert C return/status handling into idiomatic Rust `Result`/`Option` as appropriate.

### File/Responsibility Mapping

- `src/output.c` responsibilities remain consolidated in `src/output.rs`.
- Do not split helper logic into extra modules unless Rust borrow-checking forces small private helper functions inside the same file.

## Data Model

The analysis only identifies multiple anonymous C data structures. Since anonymous C structs/unions commonly appear as nested fields, local aggregates, or internal state carriers, the Rust port should introduce only the minimum set of named types required by `set_active` and `output`.

### Mapping Strategy

- **C anonymous struct/union used for persistent module state**
  - Map to a private named Rust `struct`.
  - Store owned data directly where lifetime is clear.
  - Replace nullable pointers with `Option<T>` or `Option<&T>` / `Option<&mut T>` as appropriate.

- **C anonymous struct used as a tagged variant or mode selector**
  - Map to a Rust `enum` if the C logic distinguishes cases by flags/type fields.
  - Otherwise keep as a plain `struct`.

- **C anonymous struct used only as a temporary aggregate**
  - Map to a local Rust `struct` only if it materially improves readability.
  - Otherwise inline fields into the owning Rust type or function scope.

### Expected Rust Type Conversions

- `char *` used for owned strings → `String`
- `const char *` borrowed input → `&str` where UTF-8 is valid; otherwise `&[u8]` or `&CStr` only if required by surrounding code
- raw buffer + length → `&[u8]` / `Vec<u8>`
- integer flags/booleans → `bool` or a small integer type if bitwise compatibility is required internally
- linked references/pointers → `Option<Box<T>>`, `Option<&T>`, or indices/slices depending on actual ownership
- output stream/file handle → generic `W: std::io::Write` or a concrete standard library writer if the surrounding code fixes the destination

### Memory Management and Error Handling Decisions

- Eliminate manual lifetime management from the C version by assigning clear ownership to Rust structs.
- Avoid `unsafe` unless the surrounding project interface forces raw interop; this module plan assumes a pure Rust port.
- Convert null checks into `Option` handling.
- Convert error codes from output operations into `std::io::Result`.
- Preserve any sentinel-driven behavior only when it affects logic correctness.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and state surface

- Create `src/output.rs` corresponding to `src/output.c`.
- Identify all module-level state, static variables, and anonymous aggregates referenced by `set_active` and `output`.
- Introduce the minimum private Rust structs/enums needed to represent:
  - active/inactive state manipulated by `set_active`
  - any output context consumed by `output`
- Define provisional Rust signatures for both migrated functions based on actual call patterns in the project.
- Replace C global mutation with explicit mutable access through a module state object if required by the call graph.

### Deliverables

- Compiling Rust module skeleton.
- Named Rust representations for the anonymous C structures that are directly used by these functions.
- Stubbed or partially implemented `set_active` and `output` signatures aligned with the rest of the port.

## Phase 2: Port `set_active` and dependent state transitions

- Translate `set_active` first because it likely controls module behavior consumed by `output`.
- Preserve the original branching and state updates exactly before attempting cleanup.
- Replace:
  - null pointer checks with `Option`
  - integer truth values with `bool` where compatible
  - direct field mutation on shared C state with explicit mutable borrowing
- Add unit tests for:
  - enabling/disabling or switching active state
  - repeated calls with the same input
  - boundary cases implied by null/sentinel inputs in C

### Deliverables

- Functionally complete Rust version of `set_active`.
- Tests covering state transition behavior.
- Finalized state struct fields needed by `output`.

## Phase 3: Port `output` and I/O behavior

- Translate the main `output` routine using the finalized Rust data model.
- Map output destinations to standard library writing APIs.
- Preserve ordering, conditional emission, and formatting behavior from the C implementation.
- Minimize allocations:
  - write borrowed string slices directly where possible
  - use local buffers only when the C code assembles composite output
- Convert file/stream failures into `Result` and propagate with `?`.
- Keep helper logic private and local to `src/output.rs`.

### Deliverables

- Functionally complete Rust version of `output`.
- Tests validating representative emitted output and error propagation behavior.
- Removal of temporary placeholders introduced in earlier phases.

## Phase 4: Validation and cleanup

- Compare Rust behavior against the C module for the reachable paths of `set_active` and `output`.
- Simplify only where it does not alter behavior:
  - narrow integer types
  - collapse redundant temporary structs
  - tighten borrowing/ownership
- Ensure there are no unnecessary clones or heap allocations.
- Run `cargo test` and resolve any borrow/lifetime issues without changing module scope.

### Deliverables

- Finalized `src/output.rs` with idiomatic but behavior-preserving Rust.
- Passing module tests under `cargo test`.
- Short migration notes in code comments only where the original C behavior is non-obvious.