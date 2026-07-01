# Implementation Plan: module_src_dot.c_24

## Summary

This module ports `src/dot.c` into Rust with a narrow scope centered on the existing responsibilities of:

- starting DOT-format output,
- declaring graph nodes,
- handling output emission for DOT records.

The Rust implementation should preserve the current control flow and output behavior of `dot_begin`, `declare_node`, and `dot_output_handler` without adding new formatting features or expanding the module boundary.

Technical approach:

- translate the C module into a single Rust module aligned to the original file,
- replace C global/stateful patterns with an explicit Rust state struct owned by the module,
- use standard library string and I/O types for DOT text construction and emission,
- convert nullable pointers and ad hoc state checks into `Option`, enums, and `Result`,
- keep output generation mostly streaming-oriented to avoid unnecessary copying while preserving the original ordering of emitted DOT content.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fmt`, `std::collections` only if required by the original state tracking)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve output generation complexity comparable to the C implementation,
  - avoid per-fragment heap churn where direct writes or reusable `String` buffers are sufficient,
  - maintain deterministic output order matching the original module behavior,
  - keep node declaration lookup and repeated emission overhead low, using standard containers only if the C logic already implies tracked declaration state.

## Module Mapping

C source to Rust source mapping should remain minimal and direct.

| C File | Rust File | Notes |
|---|---|---|
| `src/dot.c` | `src/dot.rs` | Single-file port of DOT output logic |

Function mapping:

| C Function | Rust Item | Notes |
|---|---|---|
| `dot_begin` | `pub fn dot_begin(...) -> Result<..., ...>` or `impl DotModuleState { fn begin(...) -> Result<..., ...> }` | Initializes DOT output state and writes prologue |
| `declare_node` | `fn declare_node(...) -> Result<..., ...>` | Internal helper for node emission/tracking |
| `dot_output_handler` | `pub fn dot_output_handler(...) -> Result<..., ...>` or `impl DotModuleState { fn output_handler(...) -> Result<..., ...> }` | Main DOT event/output path |

Recommended Rust module shape:

- `src/dot.rs`
  - module state struct holding migrated C state
  - public entry points corresponding to externally used C functions
  - private helpers corresponding to local C helpers

If the original C file relies on static file-local mutable state, migrate that state into a dedicated struct instead of Rust globals.

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust plan should use named internal types derived from usage rather than attempt a literal anonymous translation.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct #1 | `struct DotState` | Primary module state replacing file-local mutable C state |
| anonymous struct #2 | `struct NodeDecl` | Node declaration data if the C code tracks node identity/attributes |
| anonymous struct #3 | `enum OutputEvent` or `struct OutputRecord` | Output-handler input mapped according to actual C call pattern |

Data mapping guidelines:

- **C strings (`char *`, `const char *`)** -> `&str`, `String`, or `Cow<'a, str>` only if borrowing is clearly supported by call sites.
- **Nullable pointers** -> `Option<T>` / `Option<&T>` / `Option<String>`.
- **Integer flags / mode constants** -> Rust `enum` where semantics are known; otherwise keep narrow integer types until behavior is confirmed.
- **Mutable output targets (`FILE *`, buffers, callbacks)** -> `&mut dyn std::io::Write` or a generic `W: Write` where practical.
- **Ownership-sensitive linked or dynamic allocations** -> `Vec<T>` or `BTreeMap`/`HashSet` only if declaration tracking exists in the C logic.
- **Anonymous state bundles** should be split only as needed to express ownership and borrowing safely; do not introduce extra abstraction layers.

Memory-management decisions:

- replace manual allocation/free with RAII-owned Rust fields,
- avoid self-referential layouts,
- prefer borrowed string slices for transient formatting inputs,
- clone only where ownership transfer is required by stored state.

Error-handling decisions:

- convert write failures into `io::Result`,
- convert invalid state transitions previously guarded by null checks or flags into explicit `Result` errors,
- keep a small module-local error type only if non-I/O failures are present; otherwise use `io::Result` directly.

## Implementation Phases

### Phase 1: File Skeleton and State Extraction

- Create `src/dot.rs` as the direct port target for `src/dot.c`.
- Identify all file-local C state, constants, and helper routines used by:
  - `dot_begin`
  - `declare_node`
  - `dot_output_handler`
- Define a minimal Rust state carrier (`DotState`) to hold migrated mutable state instead of static mutable globals.
- Establish exact function signatures for Rust entry points based on how the C functions are invoked elsewhere in the project.
- Replace raw C null/state conventions with `Option` and explicit initialization fields.

Exit criteria:

- Rust module compiles with placeholder logic,
- all required state from `src/dot.c` has a mapped Rust field or constant,
- public/private function boundaries are identified.

### Phase 2: Output Initialization and Node Declaration Port

- Port `dot_begin` first, including:
  - output prologue generation,
  - initialization of graph/module state,
  - any required header or mode setup.
- Port `declare_node` as an internal helper with the same declaration order and duplicate-handling behavior as the C version.
- Use `std::io::Write` for streaming output and `write!`/`writeln!` for formatting.
- Preserve escaping/quoting rules exactly as implemented in the original C logic; do not normalize or redesign formatting.

Exit criteria:

- DOT prologue output matches C behavior for representative inputs,
- node declaration behavior is implemented,
- repeated declaration handling follows original semantics.

### Phase 3: Main Output Handler Port

- Port `dot_output_handler` using the Rust state established earlier.
- Translate event-driven or record-driven branching directly from C control flow.
- Route node declaration through the migrated `declare_node` helper where the C code does so.
- Preserve output ordering and conditional emission rules exactly.
- Replace pointer-based dispatch and mutation with enums, pattern matching, and mutable references.

Exit criteria:

- full module logic is represented in Rust,
- output handler compiles cleanly with stateful interactions,
- no unresolved C-style memory or nullability patterns remain in the ported path.

### Phase 4: Validation and Cleanup

- Add focused unit tests around:
  - DOT header/prologue emission,
  - node declaration formatting,
  - output handler ordering and representative output cases,
  - edge cases previously represented by null/empty inputs.
- Compare Rust output against expected text fixtures derived from current C behavior.
- Remove temporary compatibility placeholders and tighten types where behavior is confirmed.
- Ensure all functions return idiomatic `Result` values and that error propagation is straightforward.

Exit criteria:

- `cargo test` passes,
- generated DOT text is stable for covered cases,
- implementation remains confined to the original module scope without added facilities.