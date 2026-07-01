# Implementation Plan

## Summary

This module ports the `quotearg.c` entry points `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom` into Rust for the `cat` project branch `008-main_root_quotearg_n_07-rust-port`.

The Rust implementation should preserve the existing call-level behavior of indexed quoting operations while replacing C-managed buffers and pointer-based string handling with owned Rust storage. The technical approach is to migrate only the functionality needed by these three functions from the existing quoting subsystem in a minimal Rust module, using standard-library string and byte-slice types and keeping allocation behavior explicit and local to the module.

The implementation should focus on:
- reproducing the per-index quoting API behavior,
- handling both NUL-terminated and length-delimited inputs,
- supporting the custom quoting variant without widening scope beyond these entry points,
- replacing static/global buffer handling from C with safe Rust-owned storage scoped to the module.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing with respect to input length.
  - Avoid unnecessary intermediate allocations beyond what is required to produce quoted output.
  - Keep indexed result storage efficient and reusable where the C code reused per-slot buffers.
  - Maintain byte-oriented behavior for non-UTF-8 input where required by the migrated functions.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `quotearg_n` → `pub fn quotearg_n(...) -> ...`
- `quotearg_n_mem` → `pub fn quotearg_n_mem(...) -> ...`
- `quotearg_n_custom` → `pub fn quotearg_n_custom(...) -> ...`

### Internal Scope

Only the logic necessary to support these three exported functions should be migrated. Any helper logic from `quotearg.c` should be brought over as private functions inside `src/quotearg.rs` only if directly required by:
- indexed slot selection,
- memory-backed input handling,
- custom quoting delimiter behavior,
- output buffer production.

No extra module split is planned unless required by the existing Rust crate layout.

## Data Model

The analysis reports only anonymous C data structures, so the Rust plan should treat them as internal implementation details originating from `quotearg.c` and map them conservatively.

### Data-Structure Mapping

- anonymous quoting option/state structs from `quotearg.c`
  - → private Rust `struct` types in `src/quotearg.rs`
  - purpose: hold only fields needed by `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`

- anonymous static/per-slot buffer holder structs
  - → private Rust `struct` using `Vec<u8>` or `String` depending on byte requirements
  - preferred representation: `Vec<u8>` internally if original behavior is byte-oriented; convert to `String` only when valid and required by surrounding crate API

- anonymous mode/config discriminator structs or flag groups
  - → private Rust `enum` or compact `struct` of booleans/flags
  - use `enum` where the original C code selected among mutually exclusive quoting styles

- anonymous custom quoting delimiter carriers
  - → private Rust `struct { left: Vec<u8>, right: Vec<u8> }` or borrowed slice form if lifetimes remain local
  - use owned storage if the C path copied delimiters into long-lived indexed state

### Representation Notes

- **Input strings**:
  - C `char *` with implicit termination → Rust `&CStr`, `&str`, or `&[u8]` depending on call site needs.
  - C `(char *, size_t)` memory regions → Rust `&[u8]`.

- **Output buffers**:
  - C reusable slot buffers → Rust `Vec<Vec<u8>>`, `Vec<String>`, or equivalent private slot store.
  - Choose byte buffers first if escaping/quoting must preserve arbitrary bytes.

- **Indices**:
  - C integer slot index → Rust `usize` after checked conversion.

- **Errors and invariants**:
  - Cases that were impossible under C calling conventions but become explicit in Rust should use narrow internal validation.
  - Public behavior should remain compatible with the original interface expectations rather than introducing new error surface unnecessarily.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and slot-based storage

### Goals
Create the Rust file and migrate the minimal state model required for indexed quoting results.

### Tasks
- Add `src/quotearg.rs`.
- Identify the C state used specifically by `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`.
- Define private Rust structs/enums for:
  - indexed output slot storage,
  - custom delimiter configuration,
  - any required quoting options subset.
- Replace C static/global slot buffer handling with safe Rust-owned storage.
- Decide the internal output representation:
  - prefer `Vec<u8>` if arbitrary byte preservation is required,
  - otherwise use `String` only if all needed paths are text-safe.

### Deliverables
- Compilable Rust module with private data model.
- Internal helper for retrieving/growing the per-index storage slot.
- No behavior expansion beyond the original three functions.

## Phase 2: Port core quoting paths for memory-backed and indexed APIs

### Goals
Implement the shared quoting path and wire `quotearg_n_mem` and `quotearg_n`.

### Tasks
- Port the core quoting logic required by these functions from `quotearg.c` into private helpers.
- Implement `quotearg_n_mem` as the primary byte-slice entry point.
- Implement `quotearg_n` as a thin wrapper over the memory-based path.
- Ensure index handling resizes slot storage safely and deterministically.
- Preserve ownership semantics by writing results into the indexed Rust slot and returning the expected borrowed or owned view required by the crate interface.

### Memory Management Decisions
- Avoid raw pointers in the Rust implementation unless forced by an existing crate boundary.
- Use slice-based processing for input traversal.
- Reuse allocated slot buffers by clearing and appending where possible.

### Deliverables
- Working `quotearg_n_mem`.
- Working `quotearg_n` delegating to the shared implementation.
- Initial unit tests for:
  - empty input,
  - basic unquoted/quoted content,
  - multiple slot indices,
  - embedded non-text bytes if relevant.

## Phase 3: Port custom quoting variant

### Goals
Add `quotearg_n_custom` without widening support beyond the C function’s existing custom delimiter behavior.

### Tasks
- Identify the exact custom delimiter inputs used by the C function.
- Map delimiter validation and storage into the private Rust custom configuration type.
- Reuse the shared quoting engine with a custom mode branch rather than duplicating logic.
- Ensure delimiters are handled safely for byte-oriented input and output.

### Error Handling Decisions
- If the original C code relied on assertions/preconditions for invalid custom delimiters, mirror that behavior with `debug_assert!` and constrained internal APIs rather than introducing broad new result types.
- If a hard failure path existed in C for invalid inputs, keep the Rust behavior equally narrow and localized.

### Deliverables
- Working `quotearg_n_custom`.
- Unit tests covering:
  - normal custom left/right delimiters,
  - empty input with custom delimiters,
  - repeated use across multiple indices.

## Phase 4: Behavioral alignment and cleanup

### Goals
Validate parity with the migrated C logic and remove translation artifacts.

### Tasks
- Compare Rust output behavior against the C implementation for the targeted functions only.
- Review all helper functions and remove any unused code imported during translation.
- Confirm no accidental public API growth beyond the three functions and any required internal state accessors.
- Normalize naming to Rust conventions while keeping correspondence to the original functions clear.
- Run `cargo test` and fix edge cases around:
  - index growth,
  - buffer reuse,
  - byte-length handling versus NUL-termination assumptions.

### Deliverables
- Finalized `src/quotearg.rs`.
- Tests passing under `cargo test`.
- Minimal, maintainable Rust port restricted to the original module scope.