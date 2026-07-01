# Implementation Plan

## Summary

Port `src/dot.c` into a Rust module that preserves the existing DOT-output behavior and call flow for:
- `dot_begin`
- `declare_node`
- `dot_output_handler`

The Rust implementation should stay narrowly aligned with the current C module responsibilities: initializing DOT output state, emitting node declarations, and handling output events/records in the same execution order as the original code. The preferred approach is a direct translation of control flow and data ownership into safe Rust, using explicit structs and enums to replace anonymous C data shapes where needed.

The implementation should favor:
- standard-library I/O traits and buffered writing where applicable,
- owned Rust strings instead of raw character pointers,
- explicit error propagation with `Result`,
- minimal internal state required to preserve the original output formatting and sequencing.

No additional capabilities should be introduced beyond the behavior already implied by `src/dot.c`.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum practical toolchain target: Rust 1.75+

### Primary Dependencies
- Rust standard library only:
  - `std::io` for writing DOT output
  - `std::fmt` for formatting helpers if needed
  - `std::collections` only if the original implementation requires in-memory tracking for declaration deduplication or ordering already present in C logic

No third-party crates are recommended based on the available module analysis.

### Testing
- `cargo test`

Test coverage should focus on:
- DOT prologue emission from `dot_begin`
- correct node declaration formatting from `declare_node`
- output sequencing and dispatch behavior in `dot_output_handler`
- edge cases around empty names/labels, repeated declarations if relevant to current C behavior, and I/O error propagation

### Performance Goals
- Maintain behavior and throughput comparable to the C implementation for sequential DOT generation
- Avoid unnecessary string cloning in hot paths
- Prefer streaming writes over building large intermediate buffers unless the C logic already accumulates output
- Keep memory usage bounded to the module’s existing state requirements

## Module Mapping

### C to Rust File Mapping
- `src/dot.c` -> `src/dot.rs`

### Function Mapping
- `dot_begin` -> `pub(crate) fn dot_begin(...) -> Result<..., std::io::Error>` or `Result<(), std::io::Error>` depending on surrounding call contract
- `declare_node` -> `fn declare_node(...) -> Result<..., std::io::Error>` or `Result<(), std::io::Error>`
- `dot_output_handler` -> `pub(crate) fn dot_output_handler(...) -> Result<..., std::io::Error>` or a narrow internal result type if the wider project uses a module-local error alias

### Rust Module Shape
Keep the module concentrated in a single file unless the existing Rust project already has an established split for output backends. The expected shape is:
- state struct(s) replacing file-scope C state and anonymous structs
- internal helper(s) for escaping/formatting if directly required by the C code
- direct Rust equivalents of the three listed functions

## Data Model

The analysis identifies three anonymous C data structures. Since names are unavailable, the Rust port should introduce local, purpose-based names tied directly to observed usage during migration.

### C Struct to Rust Mapping
- `anonymous` -> `DotState`
  - Purpose: persistent module state used across begin/declare/output operations
  - Rust form: `struct DotState { ... }`
  - Likely fields:
    - output target handle reference or generic writer binding strategy
    - graph/session flags needed by `dot_begin`
    - any declaration tracking state already present in the C logic

- `anonymous` -> `DotNode`
  - Purpose: node declaration payload or temporary node description used by `declare_node`
  - Rust form: `struct DotNode { ... }`
    - node identifier
    - display label or derived text
    - optional attributes reflected by DOT output

- `anonymous` -> `DotOutputEvent`
  - Purpose: input record/event consumed by `dot_output_handler`
  - Rust form: `struct DotOutputEvent { ... }` or `enum DotOutputEvent`
  - Choose `enum` only if the C code branches over event kinds/tags that are represented as discriminants; otherwise use a struct with explicit fields

### C Type Conversion Guidance
- `char *` / `const char *` -> `String`, `&str`, or `Cow<'_, str>` only if borrowing is clearly sufficient
- integer flags -> `bool` or small Rust enums when the C semantics are discrete and known
- raw pointers to shared mutable state -> `&mut DotState`
- nullable pointers -> `Option<T>` / `Option<&T>` / `Option<&mut T>`
- C output stream abstraction -> generic `W: std::io::Write` or a concrete writer field in `DotState`

### Memory Management
- Replace manual allocation/free patterns with owned fields and lexical lifetimes
- Avoid self-referential layouts; copy short-lived parsed strings when necessary
- Keep temporary formatting buffers local to each function
- If the C code stores borrowed input pointers in module state, convert them to owned `String` values in Rust unless the caller’s lifetime can be expressed simply and safely

### Error Handling
- Convert write failures into `std::io::Error`
- Convert invalid state transitions that were previously implicit in C into explicit error returns or `debug_assert!` only when the invalid path is provably internal
- Do not use panics for normal output or formatting failures

## Implementation Phases

## Phase 1: Establish module skeleton and state translation

### Goals
- Create the Rust file and define the minimal data structures required to represent the C module state
- Translate file-scope constants, flags, and anonymous structs into explicit Rust types
- Decide the exact function signatures based on how the surrounding Rust branch expects to call this module

### Tasks
- Add `src/dot.rs`
- Introduce Rust replacements for the three anonymous C data structures with temporary, usage-based names
- Identify whether output is writer-driven (`W: Write`) or routed through existing project abstractions, and bind the module to that existing pattern only
- Port any static formatting fragments and DOT header/footer literals used by `dot_begin`
- Preserve the original ordering assumptions and mutable state transitions

### Deliverables
- Compiling Rust module skeleton
- Struct and enum definitions in place
- Function signatures for `dot_begin`, `declare_node`, and `dot_output_handler` established, even if bodies are initially stubbed behind `todo!()` during short-lived migration steps

## Phase 2: Port core output logic function-by-function

### Goals
- Translate the behavior of each C function directly into Rust
- Ensure output formatting remains byte-for-byte compatible where practical

### Tasks
- Implement `dot_begin`
  - initialize graph output state
  - emit opening DOT structure
  - map any C initialization side effects into `DotState`
- Implement `declare_node`
  - translate node formatting rules
  - preserve quoting/escaping behavior already present in C
  - maintain any existing duplicate-declaration handling only if it is already part of the source logic
- Implement `dot_output_handler`
  - port dispatch logic in original branch order
  - preserve interaction between handler state and node declaration/output emission
  - propagate I/O failures immediately

### Deliverables
- Working Rust equivalents for all three functions
- No unsafe code unless the surrounding project API makes it unavoidable; if unavoidable, isolate it at the call boundary only

## Phase 3: Integrate with project interfaces and tighten correctness

### Goals
- Replace remaining C-shaped patterns with idiomatic but minimal Rust
- Confirm the module fits the existing branch structure without adding new architecture

### Tasks
- Wire `src/dot.rs` into the current crate module tree
- Adjust signatures to align with adjacent migrated modules, while keeping behavior unchanged
- Remove temporary placeholders and verify ownership/borrowing is explicit
- Review all nullable/optional inputs and convert them into `Option` handling paths
- Confirm formatting does not introduce extra whitespace, reordered attributes, or changed line endings unless required by the existing Rust project conventions

### Deliverables
- Fully integrated Rust module replacing the C implementation responsibility for this file
- Clean compile with `cargo test`

## Phase 4: Add focused tests for migrated behavior

### Goals
- Lock in output compatibility for the migrated functions
- Verify error propagation and state transitions

### Tasks
- Add unit tests for `dot_begin` output header/prologue
- Add unit tests for `declare_node` formatting using representative node inputs
- Add unit tests for `dot_output_handler` covering:
  - normal event handling path
  - any branching event types or flags visible in the original C code
  - repeated or empty inputs where current behavior matters
- Add tests using an in-memory writer and a failing writer stub to verify `std::io::Error` propagation

### Deliverables
- Focused `cargo test` coverage for the migrated module
- Confirmed behavior stability for DOT output generation

## Notes and Constraints

- Keep the port limited to `src/dot.c` responsibilities only.
- Do not introduce new public APIs unless required by the existing Rust crate integration.
- Do not add concurrency primitives, serialization, FFI layers, or generalized graph abstractions.
- Prefer exact migration of formatting logic over stylistic Rust refactoring.
- If anonymous C structs turn out to be tightly coupled to external types, mirror only the fields actually used by `src/dot.c` and reference existing project types for the rest.