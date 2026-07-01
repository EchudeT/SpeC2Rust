# Implementation Plan: module_src_gnu.c_25

## Summary

Port `src/gnu.c` into a Rust module that preserves the existing output behavior centered on `gnu_output_handler`. The implementation should stay narrowly scoped to the current module boundary and migrate the existing control flow, formatting logic, and state handling without adding new abstractions beyond what is required for safe ownership and explicit error propagation.

The Rust approach should:
- translate the C file into a single corresponding Rust source module,
- convert implicit C memory and pointer usage into borrowed references and owned standard-library types where needed,
- replace C-style output/error signaling with `Result`-based handling,
- keep the execution model and emitted output ordering aligned with the original module.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.76 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Preserve the original module’s runtime characteristics for sequential output generation
- Avoid unnecessary heap allocation during output handling where borrowing or stack-based formatting is sufficient
- Keep output generation single-pass and close to the C implementation’s control flow
- Do not introduce extra buffering layers unless required by Rust ownership or I/O APIs

## Module Mapping

### C to Rust File Mapping
- `src/gnu.c` -> `src/gnu.rs`

### Function Mapping
- `gnu_output_handler` -> `pub(crate)` or private Rust function in `src/gnu.rs`, depending on actual call visibility within the crate

### Module Boundary Notes
- Keep this port contained to one Rust module corresponding directly to the C source file
- Reuse existing crate-level types and interfaces if they already exist in the Rust branch; otherwise, introduce only the minimum local signatures needed to preserve the current module contract
- Do not split the module into submodules unless required by an already-established project layout

## Data Model

### Data-structure Mapping
The analysis identifies only an `anonymous` C data structure. Because the exact fields are not listed, the Rust mapping should be derived directly from actual usage inside `src/gnu.c`.

Recommended mapping rules:
- Anonymous C struct used only locally -> named private Rust `struct` in `src/gnu.rs`
- Anonymous C enum-like integer state -> Rust `enum` if variants are clearly closed and named in usage; otherwise retain as integer type alias with constrained access
- C string pointers (`char *`, `const char *`) -> `&str`, `String`, or `&CStr` only if null-terminated external data must be preserved
- Raw output/file handles -> standard library writer abstractions already used by the crate, or minimal direct equivalents
- Optional/null pointers -> `Option<T>` / `Option<&T>` / `Option<Box<T>>` as appropriate
- Mutable shared state previously updated through pointers -> `&mut` references to explicit Rust structs

### Memory Management Decisions
- Replace manual lifetime management with ownership-based Rust values
- Eliminate null-dependent control flow by converting nullable inputs and temporary state to `Option`
- Avoid copying string data unless the original C logic requires owned mutation or retention beyond the input scope
- Keep temporary formatting data local to function scope

### Error Handling Decisions
- Replace integer/status-code returns with `Result<(), E>` or `Result<T, E>` as appropriate to the surrounding crate API
- Convert write/output failures into propagated Rust I/O or crate-local errors
- Preserve early-return behavior from the C implementation through `?` and explicit branching

## Implementation Phases

### Phase 1: Source Analysis and Signature Locking
- Inspect `src/gnu.c` and identify the exact signature, inputs, side effects, and dependencies of `gnu_output_handler`
- Identify every anonymous data aggregate used by the function and give each a minimal named Rust representation
- Determine whether the Rust module function should be private, `pub(crate)`, or exposed through an existing crate interface
- Record all dependent constants, helper functions, and external state references required to compile the port

### Phase 2: Core Port to `src/gnu.rs`
- Create `src/gnu.rs` and migrate `gnu_output_handler` with control flow kept close to the C source
- Translate C pointer-based access into Rust references and `Option` handling
- Replace C string and buffer manipulation with standard-library string/formatting operations
- Replace manual error signaling with `Result` returns, preserving original failure points
- Introduce only the minimal Rust structs/enums needed for the anonymous C data usage

### Phase 3: Integration and Behavior Alignment
- Wire the Rust module into the existing crate module tree on branch `088-module_src_gnu.c_25-rust-port`
- Update call sites or imports as needed so the Rust implementation occupies the role of the original C module
- Verify that output ordering, formatting boundaries, and state mutation semantics remain aligned with the C implementation
- Remove any transitional assumptions that are not needed after integration

### Phase 4: Tests and Cleanup
- Add focused unit tests for `gnu_output_handler` behavior using representative input/state combinations derived from the C logic
- Cover normal output generation, empty/minimal input handling, and error propagation paths where output fails or inputs are absent
- Run `cargo test` and fix ownership, lifetime, or formatting mismatches exposed during compilation/testing
- Perform final cleanup to keep the module limited to the migrated functionality only