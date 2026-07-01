# Implementation Plan: main_root_quotearg_style_13

## Summary

This module ports the `quotearg.c` portion covering `quotearg_style` and `quotearg_style_mem` into Rust for the `cat` project branch `014-main_root_quotearg_style_13-rust-port`.

The Rust implementation should preserve the existing quoting-style behavior and call patterns of these two functions without broadening the scope beyond the analyzed module. The technical approach is to migrate the relevant quoting-style selection and memory-based quoting path into a focused Rust module, using owned `String`/`Vec<u8>` outputs and borrowed byte-slice inputs where appropriate. Any C global/static configuration used by these entry points should be represented in Rust with narrow enums and plain structs, keeping the implementation local to the migrated file surface rather than introducing new abstraction layers.

Primary goals:

- Preserve function-level behavior of `quotearg_style` and `quotearg_style_mem`.
- Translate C pointer/length handling into safe Rust slice-based APIs internally.
- Keep allocation behavior explicit and minimal.
- Restrict the implementation to the existing file/function scope from `quotearg.c`.

## Technical Context

### Language/Version

- Rust 1.75 or newer

### Primary Dependencies

- Rust standard library only
- No third-party crates are recommended from the available module evidence

### Testing

- `cargo test`

### Performance Goals

- Maintain linear-time processing with respect to input length.
- Avoid unnecessary intermediate allocations beyond the final quoted result.
- Preserve efficient handling of byte-oriented input for the `_mem` variant.
- Keep branching and state handling close to the C control flow to reduce migration risk.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `quotearg_style` → `pub(crate) fn quotearg_style(...) -> String`
- `quotearg_style_mem` → `pub(crate) fn quotearg_style_mem(...) -> String` or byte-backed internal helper with a narrow public wrapper, depending on surrounding project expectations

### Integration Scope

- Keep both migrated functions in the same Rust module to match the original C file locality.
- If these functions depend on existing quoting helpers already ported elsewhere in the branch, call those helpers directly rather than duplicating logic.
- If helper logic from `quotearg.c` is still needed and not yet ported, add only the minimum private helpers inside `src/quotearg.rs` required to support these two functions.

## Data Model

The analysis lists only anonymous C data structures, which is consistent with internal tables or unnamed structs/unions in `quotearg.c`. Because no named public data structures are provided, the Rust mapping should stay minimal and limited to the data actually touched by `quotearg_style` and `quotearg_style_mem`.

### Data-Structure Mapping

- C anonymous integral/style constants → Rust `enum` for quote style
- C anonymous option/config aggregates used by the target functions → Rust `struct` with private fields
- C anonymous static lookup tables → Rust `const` arrays or `static` items
- C `char *` input with explicit length → Rust `&[u8]`
- C NUL-terminated string input → Rust `&str` when UTF-8 is guaranteed by callers; otherwise `&[u8]` plus conversion only at output boundary
- C heap-allocated result buffers → Rust `String` for textual output, or `Vec<u8>` internally if byte-preserving construction is needed before final conversion

### Rust Type Guidance

- Define a narrow `QuoteStyle` enum corresponding to the C style selector values used by these functions.
- Represent per-call quoting options, if needed, as a small private struct rather than spreading multiple scalar parameters through helpers.
- Prefer `usize` for lengths and indexing.
- Use `Option` only where the C code permits absent configuration; do not introduce broader nullable patterns without evidence.

### Memory Management

- Replace manual buffer sizing/reallocation with standard Rust-owned buffers.
- Use `String::with_capacity` or `Vec::with_capacity` when the C code implies a predictable output growth pattern.
- Eliminate raw ownership transfer patterns from C; keep all allocations scoped to function return values.

### Error Handling

- If the original functions are effectively infallible in normal use, keep Rust signatures infallible as well.
- If invalid style values are possible from translated callers, handle them through a constrained internal default or explicit `panic!` only if that mirrors existing project conventions; avoid introducing `Result` unless there is clear evidence of recoverable failure in the original interface.

## Implementation Phases

## Phase 1: Establish Rust module surface

- Create `src/quotearg.rs` if not already present in the branch.
- Add Rust equivalents for the style selector and any directly referenced option/config data needed by `quotearg_style` and `quotearg_style_mem`.
- Define the function signatures for the two target functions based on how the rest of the Rust port calls them.
- Wire the module into the crate using standard Rust module declarations only as needed for existing callers.

### Deliverables

- Compiling module skeleton
- Rust enum/struct definitions for directly used quoting metadata
- Stubbed or partially implemented `quotearg_style` and `quotearg_style_mem`

## Phase 2: Port core quoting logic for the two functions

- Translate the control flow from `quotearg_style` into Rust, preserving style dispatch behavior.
- Translate `quotearg_style_mem` using slice-based input and owned output construction.
- Port only the helper logic required by these two functions from `quotearg.c`, keeping such helpers private to `src/quotearg.rs`.
- Convert C static tables or character-class logic used by these functions into Rust `const`/`static` data.
- Ensure byte-length-sensitive behavior from the `_mem` variant is preserved without relying on NUL termination.

### Deliverables

- Working Rust implementations of both functions
- Private helper functions/constants migrated from `quotearg.c` only where required
- No remaining raw-pointer-style logic in the migrated path

## Phase 3: Align API behavior and edge cases

- Verify style-selection behavior matches the C implementation for all relevant enum values used by the project.
- Check empty input, embedded NUL bytes for `_mem`, and boundary-length handling.
- Reconcile any differences between byte-oriented internal processing and string-oriented Rust return types.
- Remove temporary migration scaffolding and ensure signatures match actual crate usage.

### Deliverables

- Behaviorally aligned Rust implementation for normal and edge inputs
- Cleaned module API with only necessary private helpers retained

## Phase 4: Add focused tests and finalize migration

- Add unit tests in the same module or crate test layout covering:
  - style-based output selection from `quotearg_style`
  - explicit-length input handling in `quotearg_style_mem`
  - empty and representative special-character inputs
- Compare expected results against known C behavior or existing project expectations where available.
- Run `cargo test` and fix mismatches introduced during translation.
- Confirm the migrated Rust file fully covers the targeted C functions for this module scope.

### Deliverables

- Passing `cargo test`
- Test coverage for the two migrated functions
- Finalized `src/quotearg.rs` implementation for this module