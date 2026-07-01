# Implementation Plan

## Summary

Port the C module `src/parseopt/wordwrap.c` into an equivalent Rust module that preserves the existing wrapping behavior implemented by `wordwrap_at_bol` and `wordwrap_at_eol`. The Rust work should remain narrowly scoped to migrating these functions and any directly required local state or helper logic from the source file, without introducing new formatting features or broader parser changes.

The technical approach is to translate the current line-wrapping logic into safe Rust using standard-library string and slice handling. Where the C implementation relies on pointer arithmetic, mutable buffers, and implicit ownership rules, the Rust version should use explicit borrowing, indexed traversal over bytes or characters as required by the original behavior, and return values that make failure or boundary conditions explicit. Any state currently represented by anonymous C structs should be converted only as needed into private Rust structs with fields matching the original operational role.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the current asymptotic behavior of the C implementation for line scanning and wrap-point detection.
  - Avoid unnecessary string reallocations in the main wrapping path.
  - Keep per-call overhead close to the original by using slices and in-place position calculations where possible.
  - Ensure no uncontrolled copying of whole input buffers unless required by the original function contract.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` | Direct module port of the wrapping logic. |
| `wordwrap_at_bol` | `parseopt::wordwrap::wordwrap_at_bol` | Preserve signature intent; adapt to Rust borrowing and result conventions. |
| `wordwrap_at_eol` | `parseopt::wordwrap::wordwrap_at_eol` | Preserve signature intent; adapt to Rust borrowing and result conventions. |

If the crate already exposes a `parseopt` module tree, register `wordwrap.rs` from the existing `src/parseopt/mod.rs`. If not yet present, only add the minimum module declaration needed for this file migration.

## Data Model

The analysis lists only anonymous C data structures, so the migration plan should avoid inventing broad public models. Map structures only when the C file requires retained local state beyond plain parameters.

| C Data Structure | Rust Mapping | Migration Rule |
|---|---|---|
| anonymous local struct used only within `wordwrap.c` | private `struct` in `src/parseopt/wordwrap.rs` | Introduce only if needed to preserve grouped state used across both functions. |
| anonymous record representing wrap boundaries, offsets, or positions | private `struct` with integer fields (`usize`/`isize` as appropriate) | Use `usize` for indices into Rust slices; use signed integers only if the C logic depends on sentinel negatives. |
| anonymous flag-bearing struct | private `struct` with `bool` and integer fields | Replace C integer flags with `bool` where semantics are binary. |
| anonymous temporary aggregate | local variables or tuple | Prefer not to materialize as a struct if state is not shared. |

### Type Mapping Guidelines

| C Type Pattern | Rust Type |
|---|---|
| `char *` / `const char *` input buffer | `&str` if the algorithm is text-oriented and UTF-8-safe under expected inputs; otherwise `&[u8]` if byte-exact behavior is required |
| mutable character buffer | `&mut String` or `&mut [u8]` depending on actual mutation pattern |
| length / offset / position | `usize` |
| status code / sentinel return | `Option<usize>`, `Result<_, _>`, or exact integer return if needed for compatibility inside the crate |
| C booleans encoded as `int` | `bool` |

### Memory Management and Error Handling

- Replace implicit C ownership with borrowed inputs and explicit mutable outputs.
- Avoid unsafe Rust unless exact in-place buffer behavior cannot be represented safely; safe Rust is the default expectation.
- Convert null-pointer checks into type-level guarantees wherever possible.
- Use `Option` for “no wrap position found” cases and `Result` only if the original function distinguishes operational errors from ordinary no-match outcomes.
- Keep error handling internal and minimal; do not introduce new recovery layers.

## Implementation Phases

### Phase 1: Source Audit and Signature Translation

- Inspect `src/parseopt/wordwrap.c` and identify:
  - exact parameter and return contracts of `wordwrap_at_bol`
  - exact parameter and return contracts of `wordwrap_at_eol`
  - any file-local helpers, macros, or anonymous structs used by these functions
- Decide whether the Rust implementation must operate on `&str` or `&[u8]` based on whether the C logic is byte-indexed and whether wrap decisions depend on raw byte values.
- Define Rust function signatures that preserve the original call semantics as closely as possible within the crate.
- Create `src/parseopt/wordwrap.rs` and add only the minimum module declaration needed.

### Phase 2: Core Logic Port

- Port `wordwrap_at_bol` first, translating:
  - pointer movement into explicit index arithmetic
  - delimiter and boundary checks into slice access
  - mutable output updates into returned positions or direct mutation of passed state
- Port `wordwrap_at_eol` using the same indexing model so both functions share consistent boundary semantics.
- Introduce private helper functions only where they are directly extracted from repeated C logic inside this file.
- Introduce private structs only if anonymous C aggregates are required to keep the control flow readable and faithful to the original implementation.

### Phase 3: Behavioral Validation

- Add focused unit tests in the same module or crate test layout covering:
  - wrap detection at beginning-of-line conditions
  - wrap detection at end-of-line conditions
  - empty input
  - single-word input
  - boundary widths and exact-fit cases
  - whitespace and delimiter edge cases reflected in the original C logic
- Build expected outputs from observed C behavior rather than reinterpreting the feature.
- Confirm that index handling does not panic on boundary inputs and that “no wrap” outcomes match the C behavior.

### Phase 4: Cleanup and Integration Check

- Remove any temporary translation scaffolding left from the C port.
- Ensure all anonymous-structure replacements are private and minimal.
- Verify the module compiles cleanly under `cargo test`.
- Confirm that the final Rust file remains limited to the migrated functionality from `wordwrap.c` and does not pull in unrelated parsing or formatting work.