# Implementation Plan

## Summary

Port `src/parseopt/wordwrap.c` into an idiomatic Rust module that preserves the existing wrapping behavior exposed by:

- `wordwrap_at_bol`
- `wordwrap_at_eol`

The Rust implementation should stay narrowly scoped to this file’s current responsibilities: locating wrap points relative to the beginning or end of a line and returning equivalent results to the C code. The preferred approach is a direct migration of the existing logic into a single Rust source file under the parse-option area of the crate, using borrowed string/slice inputs where possible and avoiding new abstraction layers unless they are required to represent C state safely.

The implementation should prioritize:

- behavior parity with the C functions
- safe handling of string indexing and bounds
- minimal allocation
- straightforward translation of control flow from C to Rust

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the linear scan characteristics of the original C routines.
  - Avoid unnecessary heap allocation during wrap-point calculation.
  - Keep per-call overhead low by operating on borrowed data (`&str`, `&[u8]`, or iterator-based scans as appropriate).
  - Match existing behavior without introducing extra passes beyond what the C implementation requires.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` | Direct port of the file’s wrapping logic. |
| `wordwrap_at_bol` | `pub(crate)` or private Rust function in `src/parseopt/wordwrap.rs` | Visibility should match actual crate usage; keep restricted unless required elsewhere. |
| `wordwrap_at_eol` | `pub(crate)` or private Rust function in `src/parseopt/wordwrap.rs` | Same visibility guidance as above. |

If the current Rust crate already has a `parseopt` module tree, this file should be added there and referenced through the existing `mod` declarations only as needed for compilation. No extra helper modules should be introduced unless they replace file-local C helpers.

## Data Model

The analysis lists only anonymous C data structures and does not identify named structs that define stable API surface for this file. The plan should therefore assume one of two migration outcomes:

1. **No persistent struct mapping required**
   If `wordwrap.c` uses only local variables, pointers, and primitive parameters, represent them directly with Rust primitives and borrowed references.

2. **Inline replacement of anonymous local structs/unions if present**
   If the file contains file-local anonymous aggregates used only to organize temporary state, replace them with one of:
   - local Rust tuples
   - small private Rust structs with explicit field names
   - enums only if the C code models discrete state branches

### Expected C-to-Rust Type Mapping

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| `char *` / `const char *` | `&str` or `&[u8]` | Use `&str` if logic is text-oriented and valid UTF-8 is guaranteed by surrounding Rust code; use `&[u8]` if behavior depends on byte-wise scanning identical to C. |
| Pointer + length / index arithmetic | slice + `usize` indices | All indexing must be bounds-checked via safe Rust patterns. |
| `int` return used as position/status | `usize`, `Option<usize>`, or `Result<usize, _>` | Choose the narrowest form that preserves current semantics; use signed integers only if negative sentinel values are part of the C contract. |
| Local anonymous struct | private `struct` | Only if needed to keep logic readable during migration. |
| Boolean-like integer flags | `bool` | Convert only when there is no dependency on numeric sentinel values. |

### Memory Management and Error Handling

- Replace all raw pointer traversal with slice or string traversal.
- Eliminate manual lifetime and ownership concerns by borrowing caller-owned input.
- Preserve sentinel-based behavior where needed, but prefer `Option` internally if it does not change external behavior.
- If the original functions never report recoverable errors and only compute positions, do not introduce `Result` unnecessarily.
- Be careful with Rust string indexing: if exact byte-position behavior matters, implement scanning over bytes rather than character indices.

## Implementation Phases

### Phase 1: Source Analysis and Rust File Scaffolding

- Inspect `src/parseopt/wordwrap.c` and identify:
  - exact function signatures
  - return-value conventions
  - whether operations are byte-based or character/whitespace-classification based
  - any file-local helpers, macros, or anonymous structs actually used by the two target functions
- Create `src/parseopt/wordwrap.rs`.
- Add the minimal `mod` wiring required by the existing Rust crate layout.
- Define Rust function signatures that preserve the observable contract of:
  - `wordwrap_at_bol`
  - `wordwrap_at_eol`

**Exit criteria**:
- Rust module compiles with placeholder or skeletal implementations.
- Signature decisions are documented in code comments where C semantics require non-obvious mapping.

### Phase 2: Direct Logic Port

- Port `wordwrap_at_bol` first as a near-structural translation from C into safe Rust.
- Port `wordwrap_at_eol` second, reusing only strictly necessary shared local helpers.
- Translate:
  - pointer increments/decrements into index movement
  - whitespace or delimiter checks into equivalent Rust byte tests
  - sentinel returns into equivalent Rust return forms
- Keep helper functions file-local and minimal; do not generalize beyond current usage.

**Key technical checks**:
- No unchecked indexing.
- No accidental UTF-8 character-boundary assumptions if C logic is byte-oriented.
- Matching edge-case behavior for empty input, boundary widths, and no-wrap cases.

**Exit criteria**:
- Both target functions are implemented in Rust.
- The C control flow has a clear counterpart in Rust without adding unrelated features.

### Phase 3: Behavioral Verification

- Add unit tests in the Rust module or the existing crate test layout covering:
  - empty input
  - single-word input
  - wrap width at start/end boundaries
  - strings with internal spaces or separators relevant to the C logic
  - no valid wrap point cases
  - exact parity for beginning-of-line vs end-of-line scan behavior
- Where the C behavior is ambiguous from reading alone, derive expected cases directly from the original implementation and encode those as regression tests.
- Run `cargo test` and fix any parity issues.

**Exit criteria**:
- Tests cover nominal and boundary behavior of both functions.
- Rust implementation behavior matches the C implementation for representative cases.

### Phase 4: Final Cleanup and Integration Review

- Remove translation scaffolding, dead comments, and unused temporary helpers.
- Confirm function visibility is no broader than needed.
- Verify the module uses only standard library facilities and does not allocate unnecessarily.
- Perform a final review for:
  - ownership/borrowing simplicity
  - absence of panic-prone indexing in normal operation
  - faithful handling of all sentinel/edge conditions from C

**Exit criteria**:
- Module is cleanly integrated into the Rust branch.
- Implementation is minimal, safe, and behaviorally aligned with the original C file.