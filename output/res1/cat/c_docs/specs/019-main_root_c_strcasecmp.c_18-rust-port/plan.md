# Implementation Plan

## Summary
Port `c-strcasecmp.c` into a focused Rust module that preserves the existing comparison behavior of `c_strcasecmp` without adding new API surface beyond what is needed for the current project branch. The Rust implementation should mirror the C routine’s case-insensitive string comparison semantics for byte-oriented text, using standard-library facilities and explicit ASCII-oriented normalization where applicable, so behavior stays close to the original low-level implementation.

The technical approach is to migrate the single C function into a small Rust module under the main crate, keeping the logic straightforward and allocation-free. The implementation should operate on borrowed string data, compare incrementally, and return an ordering-compatible integer result matching the C-style contract expected by surrounding code.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve allocation-free comparison behavior.
  - Keep runtime linear in input length.
  - Avoid unnecessary Unicode case-folding overhead when C behavior is byte/ASCII-oriented.
  - Maintain predictable branch structure close to the original C implementation.

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `c-strcasecmp.c` | `c_strcasecmp` | `src/main_root_c_strcasecmp.rs` | `pub(crate) fn c_strcasecmp(...) -> i32` |

### Migration Notes
- Keep the port limited to the existing function from `c-strcasecmp.c`.
- Do not split the logic into extra helper modules unless required by borrow-checking or test clarity.
- Prefer a direct translation of the comparison loop and normalization steps over a higher-level redesign.

## Data Model

This module does not define persistent data structures in the analyzed C input.

### Function/Input Mapping

| C Type/Concept | Rust Mapping |
|---|---|
| `const char *` string input | `&str` if call sites guarantee valid UTF-8; otherwise `&[u8]` is preferred for byte-faithful behavior |
| integer comparison result (`<0`, `0`, `>0`) | `i32` |

### Data-Handling Decisions
- Prefer `&[u8]` internally if the original function compares raw bytes with ASCII case folding, because this more closely matches C pointer-based traversal and avoids changing semantics for non-UTF-8 data.
- If project call sites already operate on Rust `&str`, accept `&str` at the public boundary and immediately compare via `as_bytes()`.
- No heap allocation is required.
- No owned structs or enums are needed for this module.

## Implementation Phases

### Phase 1: Establish Rust module and function signature
- Create the Rust file for the migrated module at `src/main_root_c_strcasecmp.rs`.
- Define the `c_strcasecmp` function with a signature aligned to how the Rust port consumes string data in the surrounding crate.
- Decide the narrowest correct input type:
  - use `&[u8]` for closest C semantics, or
  - use `&str` with internal byte conversion if existing Rust call sites require text slices.
- Document the function contract briefly in code comments: case-insensitive comparison, C-style integer result, no allocation.

### Phase 2: Port comparison logic directly from C
- Translate the original loop structure into Rust with explicit byte iteration/indexing.
- Implement ASCII case normalization in a direct and local manner to preserve C behavior.
- Compare normalized bytes until a difference or terminator/end-of-slice is reached.
- Return an `i32` difference/result consistent with the C routine’s ordering expectations.
- Ensure memory safety by replacing pointer arithmetic with slice indexing or iterator traversal while keeping logic close to the source.

### Phase 3: Validate semantic parity with tests
- Add unit tests covering:
  - equal strings with identical case,
  - equal strings with mixed case,
  - left-less-than-right and left-greater-than-right outcomes,
  - prefix relationships,
  - empty-string handling,
  - ASCII boundary characters unaffected by case conversion.
- Add targeted tests for any known call-site expectations if visible in the crate.
- Run `cargo test` and adjust only for semantic parity with the original C behavior.

### Phase 4: Integrate and remove C dependency path
- Update module declarations and internal imports so callers use the Rust `c_strcasecmp`.
- Confirm there is no remaining reliance on the C source file in this branch’s build path.
- Keep integration narrow: only wire up the migrated function and avoid unrelated refactoring.
- Perform a final review for:
  - no allocations introduced,
  - no panics from indexing assumptions,
  - return values consistent with C-style comparison semantics.