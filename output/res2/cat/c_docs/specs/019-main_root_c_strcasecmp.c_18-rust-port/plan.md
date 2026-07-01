# Implementation Plan

## Summary
Port `c-strcasecmp.c` into a focused Rust module that preserves the existing case-insensitive string comparison behavior without adding new capabilities. The Rust implementation should translate the current byte-oriented comparison logic into safe Rust using standard library string/byte access, while keeping semantics aligned with the original C function.

The implementation should stay narrow in scope:
- migrate the single function `c_strcasecmp`
- keep behavior compatible with the C routine’s comparison contract
- avoid introducing new parsing, locale layers, or broader string utility abstractions

The preferred technical approach is to implement the comparison over byte slices, applying ASCII case normalization during iteration so the port remains close to the source logic and avoids unnecessary allocation. Public exposure should match project needs only, with tests centered on equivalence for representative input pairs and edge cases.

## Technical Context
- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time comparison behavior, `O(n)` in the shorter input plus termination handling
  - Avoid heap allocation during comparison
  - Keep per-byte processing simple and branch-light, comparable to the C implementation
  - Maintain predictable behavior for ASCII-oriented case folding used by the original routine

## Module Mapping
- **C source**: `c-strcasecmp.c`
- **Rust target**: `src/c_strcasecmp.rs` or equivalent existing main-cluster module file if the project already centralizes root-level helpers
- **Function mapping**:
  - `c_strcasecmp` -> `pub(crate) fn c_strcasecmp(...) -> i32` (or the narrowest visibility compatible with current crate usage)

If the surrounding Rust port already has an established module tree for translated root C files, this function should be added there rather than creating extra layers. The goal is direct migration of the existing file/function, not reorganization.

## Data Model
This module has no declared C structs or persistent data types to migrate.

Relevant type mapping for function inputs and outputs:
- C string pointer inputs -> Rust borrowed string/byte representations, chosen to preserve comparison semantics
- C integer comparison result -> `i32`

Recommended Rust representation:
- accept `&[u8]` internally for byte-accurate iteration
- if external callers operate on `&str`, provide only the minimal adaptation needed within the existing crate structure

No new Rust structs or enums are required.

## Implementation Phases

### Phase 1: Establish Rust module and function signature
- Create the Rust destination file corresponding to `c-strcasecmp.c`.
- Add the translated `c_strcasecmp` function with a signature consistent with existing crate call sites.
- Decide the narrowest practical input form:
  - prefer byte-slice processing for C-like behavior
  - only expose `&str` if the surrounding port already standardizes on UTF-8 string inputs
- Document the comparison contract in code comments briefly, focusing on ASCII-style case-insensitive ordering and return-value expectations.

### Phase 2: Port comparison logic directly
- Translate the original loop structure into safe Rust iteration over bytes.
- Apply case normalization per byte during comparison using standard-library ASCII helpers.
- Preserve the C-style ordering result:
  - zero when equal under case-insensitive comparison
  - negative/positive result according to first differing normalized byte or termination difference
- Avoid allocation, temporary owned strings, or broader Unicode case folding.
- Handle input length differences explicitly so end-of-string behavior remains aligned with the source logic.

### Phase 3: Validate behavior with targeted tests
- Add unit tests covering:
  - equal strings with identical case
  - equal strings with differing ASCII case
  - first-character difference
  - later-character difference
  - prefix vs longer string
  - empty vs empty
  - empty vs non-empty
  - mixed punctuation/digits alongside letters
- Keep expected results focused on sign and equality semantics unless exact numeric deltas are known to matter in current usage.
- Run `cargo test` and adjust implementation only for semantic parity with the C behavior.

### Phase 4: Integrate with the existing crate layout
- Wire the module into the current Rust project structure with minimal exposure.
- Replace or connect any existing placeholder/stub for this translated C file.
- Confirm there are no unnecessary wrappers, duplicate helpers, or expanded utility APIs beyond the migrated function.
- Perform a final review for:
  - safe memory handling through borrowing only
  - no panics in normal comparison paths
  - no ownership/lifetime complexity beyond direct input borrowing