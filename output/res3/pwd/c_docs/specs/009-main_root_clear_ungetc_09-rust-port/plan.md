# Implementation Plan: main_root_clear_ungetc_09

## Summary

This module ports the C logic in `fflush.c` that clears any pending `ungetc` state while preserving the underlying stream position semantics expected by the original implementation. The Rust implementation should keep the scope narrow: migrate the two existing functions into a Rust module that operates on the project’s existing stream abstraction, without adding broader buffering or I/O capabilities.

The technical approach is to translate the current file/stream state manipulation into safe Rust where possible, using explicit mutable access to the stream state and returning `Result` for fallible operations. Any behavior that depends on low-level cursor repositioning should be modeled through the existing Rust-side file handle or reader abstraction already used by the `pwd` port. The implementation should preserve observable behavior of:
- clearing pushed-back input state, and
- preserving current logical position when required.

The port should prefer standard-library I/O traits and types, while keeping the code organized as a direct migration of the current C module rather than redesigning stream management.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the provided module analysis
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve the constant-time nature of clearing in-memory pushed-back state
  - Avoid unnecessary allocations
  - Limit repositioning calls to only the cases required by the original semantics
  - Match existing I/O behavior closely enough that no measurable regression is introduced in normal command execution

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `fflush.c` | `clear_ungetc_buffer_preserving_position` | `src/main_root_clear_ungetc_09.rs` or the existing main-cluster module file for this branch | `fn clear_ungetc_buffer_preserving_position(...) -> Result<..., ...>` |
| `fflush.c` | `clear_ungetc_buffer` | `src/main_root_clear_ungetc_09.rs` or the existing main-cluster module file for this branch | `fn clear_ungetc_buffer(...) -> Result<..., ...>` |

### Notes
- Keep both migrated functions together in one Rust module corresponding to this C source file.
- If the branch already has a central `main_cluster` module layout, place the functions into that existing file hierarchy rather than introducing a new abstraction layer.
- Do not split helper logic into extra modules unless required by borrow-checking or existing project structure.

## Data Model

No standalone C structs were listed for this module, so the primary mapping concern is the stream state manipulated by these functions.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` or internal stream handle | Existing Rust stream/file abstraction used by the port | Reuse the current project type rather than inventing a new wrapper |
| ungetc pushback buffer/state | Fields on the Rust stream state structure, or equivalent internal buffer representation | Represent as explicit optional byte/state or buffer slice, depending on the existing port architecture |
| file position / seek offset | `u64`/`i64`-backed cursor position via `std::io::Seek` | Use standard-library seeking APIs when the original logic requires position preservation |
| C integer status returns | `Result<(), std::io::Error>` or project-local result type | Prefer structured error propagation over sentinel integer returns |

### Memory Management and Error Handling

- Replace implicit C ownership and mutable global-style stream mutation with explicit `&mut` access.
- Keep all ungetc-state clearing operations allocation-free if the original logic is allocation-free.
- Convert any fallible position-save or seek-restore steps into `Result` returns.
- Avoid `unsafe` unless the broader port already exposes unavoidable low-level stream internals that cannot be expressed safely.
- If the original C code distinguishes between infallible state reset and fallible repositioning, preserve that distinction in function structure.

## Implementation Phases

### Phase 1: Inspect and map existing stream state
- Identify where the Rust port currently stores:
  - pushed-back input state,
  - current logical read position,
  - underlying seek capability.
- Determine the exact Rust target file in the branch’s `main_cluster` layout for the migrated functions.
- Define the Rust function signatures to match current project conventions for mutable stream access and error returns.
- Confirm whether position preservation must be implemented through:
  - logical state adjustment only, or
  - actual `Seek` operations on the underlying handle.

### Phase 2: Port `clear_ungetc_buffer`
- Translate the simpler clear operation first.
- Implement direct reset of the ungetc-related state on the existing stream abstraction.
- Preserve any required invariants between buffer contents, EOF/error flags, and read cursor state from the current C behavior.
- Add focused unit tests covering:
  - empty pushback state,
  - single pushed-back byte/state,
  - repeated clear calls.

### Phase 3: Port `clear_ungetc_buffer_preserving_position`
- Implement the preserving-position variant using the already-ported basic clear logic where appropriate.
- Save and restore logical/underlying position exactly as required by the original behavior.
- Ensure borrow lifetimes allow temporary position inspection and subsequent state mutation without introducing unnecessary wrappers.
- Add tests for:
  - no pending ungetc state,
  - pending ungetc state with preserved position,
  - fallible seek/path where the underlying stream cannot preserve position as expected by the project model.

### Phase 4: Integrate and verify module behavior
- Replace any remaining call sites in the Rust branch that still depend on placeholder or incomplete logic for this module.
- Run `cargo test` and fix mismatches in status/error propagation.
- Perform a final pass to keep the implementation minimal and aligned with the original C file boundaries, without adding new public API beyond what the migration requires.