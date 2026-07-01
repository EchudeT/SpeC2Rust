# Implementation Plan: module_src_dot.c_24

## Summary

This module ports `src/dot.c` into Rust with a narrow migration scope centered on the existing entry points:

- `dot_begin`
- `declare_node`
- `dot_output_handler`

The Rust implementation should preserve the current module responsibilities and call flow, with a direct translation of the DOT-output state and formatting logic into a single Rust module. The preferred approach is to replace C-style global or implicit mutable state with an explicit Rust-owned module state structure, while keeping function boundaries aligned with the original C functions as closely as possible.

The implementation should prioritize:

- direct file/function migration over redesign,
- safe ownership of output buffers and node-related data,
- minimal allocation beyond what the C code already implies,
- explicit error propagation for output operations and invalid state transitions.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
  - `std::io` for output handling
  - `std::fmt` / `String` for DOT text assembly
  - No third-party crates recommended based on current evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain output generation performance comparable to the C implementation for typical graph emission workloads
  - Avoid unnecessary cloning of node identifiers or output fragments
  - Prefer streaming/writer-based output or append-to-buffer patterns over repeated temporary allocations
  - Keep per-node declaration work effectively constant aside from required string formatting

## Module Mapping

### C to Rust File Mapping

- `src/dot.c` → `src/dot.rs`

### Function Mapping

- `dot_begin` → `pub(crate) fn dot_begin(...) -> Result<..., DotError>`
- `declare_node` → `pub(crate) fn declare_node(...) -> Result<..., DotError>`
- `dot_output_handler` → `pub(crate) fn dot_output_handler(...) -> Result<..., DotError>`

### Rust Module Shape

The Rust port should remain a single focused module:

- `src/dot.rs`
  - module state structure for DOT emission
  - translated helper logic kept private unless required by existing crate interfaces
  - migrated public/internal functions corresponding to the original C entry points

If the surrounding Rust crate already has an output abstraction or shared graph state, this module should integrate with that existing structure rather than introducing a new subsystem.

## Data Model

Because the analysis only exposes three anonymous C data structures, the Rust data model should be derived from actual field usage during migration rather than invented up front. The mapping should stay conservative and field-for-field where possible.

### Struct Mapping Strategy

- `anonymous` → `struct DotState`
  - Use for mutable module-level emission state formerly held in C static/global data or passed through opaque structures
  - Store writer/buffer handle, graph-started flag, and any formatting/configuration values required by `dot_begin` and `dot_output_handler`

- `anonymous` → `struct NodeDecl`
  - Use for node declaration inputs or temporary assembled node metadata if the C code groups node-related fields
  - Prefer owned `String` only where the Rust call path requires ownership; otherwise use borrowed `&str`

- `anonymous` → `enum OutputEvent` or `struct OutputContext`
  - Choose `enum` if the C logic branches on event/type codes
  - Choose `struct` if the C code uses a context record with multiple flags and pointers
  - Final shape should follow the actual control/data usage in `dot_output_handler`

### C-to-Rust Type Conventions

- `char *` / string buffers → `String`, `&str`, or `Cow<'a, str>` only if borrowing is clearly needed
- output file handles / stream pointers → generic writer parameter `W: std::io::Write` or a concrete owned writer already used by the crate
- integer flags → `bool` where semantics are binary; otherwise preserve exact integer width if values are meaningful
- pointer-linked optional fields → `Option<T>`
- mutable shared state → `&mut DotState`
- sentinel/error return codes → `Result<T, DotError>`

### Memory Management Decisions

- Replace manual buffer management with `String` and writer APIs
- Eliminate raw ownership transfer patterns from C in favor of lexical ownership
- Avoid storing borrowed references in long-lived state unless lifetimes are simple and directly justified by the existing call graph
- Keep temporary formatting data local to each function to reduce lifetime complexity

### Error Handling Decisions

- Introduce a small module-local `DotError` type
- Convert write failures and invalid usage/state into `Result`
- Preserve caller-visible behavior where possible by mapping prior C success/failure paths directly to Rust return values
- Do not add recovery behavior beyond what is necessary to represent existing failure cases

## Implementation Phases

### Phase 1: Skeleton Port and State Extraction

- Create `src/dot.rs`
- Identify all file-scope state, constants, and helper routines used by:
  - `dot_begin`
  - `declare_node`
  - `dot_output_handler`
- Define initial Rust equivalents for the anonymous C structures based strictly on observed field access
- Define the module-local error type and function signatures
- Establish the writer/buffer strategy used throughout the module

**Exit criteria:**
- Rust module compiles with placeholder logic
- State/data structure definitions are sufficient to support direct function translation

### Phase 2: Core Function Translation

- Port `dot_begin` first to establish initialization semantics and opening DOT output
- Port `declare_node` next, preserving node formatting, naming, and declaration order
- Port `dot_output_handler` last, using the now-defined state and node declaration path
- Translate C control flow directly before applying minor Rust cleanups
- Keep helper functions private and limited to code required by these three functions

**Exit criteria:**
- All three target functions are implemented in Rust
- Output behavior is structurally equivalent to the C module for normal call sequences
- No unsafe code is introduced unless a surrounding crate interface strictly requires it

### Phase 3: Behavior Preservation and Edge-Case Alignment

- Verify formatting details such as delimiters, quoting, indentation, and graph open/close sequencing
- Align handling of duplicate declarations, empty names, null-equivalent inputs, and state preconditions with the C behavior
- Reduce unnecessary allocations discovered during the first-pass translation
- Remove any temporary compatibility scaffolding that is not needed by the final module

**Exit criteria:**
- Rust output matches expected DOT text behavior from the C implementation
- Error paths are explicit and compile-time checked
- Memory ownership is fully safe and localized

### Phase 4: Tests and Final Integration Cleanup

- Add unit tests around:
  - graph initialization via `dot_begin`
  - node declaration emission via `declare_node`
  - event/output sequencing via `dot_output_handler`
- Add focused regression tests for empty or unusual identifiers if supported by the original logic
- Confirm module integration with the crate build and existing call sites
- Ensure naming and visibility are minimal and consistent with the rest of the Rust port branch

**Exit criteria:**
- `cargo test` passes
- Module is integrated on branch `087-module_src_dot.c_24-rust-port`
- Port stays limited to the original module responsibilities without adding new facilities