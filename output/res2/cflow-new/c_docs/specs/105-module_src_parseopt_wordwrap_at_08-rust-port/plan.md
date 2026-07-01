# Implementation Plan: module_src_parseopt_wordwrap_at_08

## Summary

This module ports the C word-wrapping logic in `src/parseopt/wordwrap.c` to Rust, preserving the existing behavior of the two exported routines:

- `wordwrap_at_bol`
- `wordwrap_at_eol`

The Rust implementation should remain narrowly scoped to the current module responsibilities: computing and applying wrapping behavior around beginning-of-line and end-of-line boundaries. The port should migrate existing control flow and text-scanning behavior directly rather than redesigning the API or introducing broader text-layout abstractions.

The technical approach is:

- translate the C routines into a Rust module with equivalent function boundaries,
- replace pointer-based string traversal with safe slice- and index-based processing over `&str` / `String` or byte slices where exact character positions matter,
- model C anonymous helper structures only if they are required to preserve local state cleanly during translation,
- keep allocation behavior explicit and minimal,
- express error conditions through Rust return types only where the C code has observable failure paths; otherwise preserve total-function behavior.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time scanning behavior relative to input length,
  - avoid unnecessary intermediate allocations during wrapping decisions,
  - keep output construction bounded to the final wrapped text plus small temporary state,
  - maintain behavior close to the C implementation for ASCII-oriented parsing and line-boundary handling.

## Technical Context Details

### Rust Edition and Tooling
- Use the current project Rust edition already established by the workspace; if unspecified, target Rust 2021.
- No external crates are recommended because the input provides no evidence of regex, Unicode segmentation, or parser-generator requirements.

### Error Handling Strategy
- If the C functions do not expose recoverable errors and instead operate deterministically on provided buffers, prefer plain return values.
- If invalid index/state conditions appear during translation, represent them as internal checked branches rather than panics where practical.
- Avoid introducing custom error hierarchies unless the original module already has explicit failure signaling that must be preserved.

### Memory Management Strategy
- Replace raw pointer arithmetic with:
  - `&str` when logic is text-oriented and valid UTF-8 is guaranteed by the Rust-side caller,
  - `&[u8]` plus explicit index tracking when the C logic depends on byte-level scanning.
- Use `String` for produced wrapped output only if the original functions materially build or rewrite strings.
- Keep helper state stack-allocated in small structs/enums where needed.

## Module Mapping

### Source File Mapping
- **C source**: `src/parseopt/wordwrap.c`
- **Rust target**: `src/parseopt/wordwrap.rs`

### Function Mapping
- `wordwrap_at_bol` -> `pub(crate)` or module-visible Rust function `wordwrap_at_bol`
- `wordwrap_at_eol` -> `pub(crate)` or module-visible Rust function `wordwrap_at_eol`

Visibility should match actual call sites in the Rust port of the surrounding `parseopt` area. Do not widen API exposure beyond what the C module required.

### Integration Mapping
- If `wordwrap.c` was included through a local parseopt subsystem, place the Rust module under the same subsystem tree:
  - `src/parseopt/mod.rs` should declare `mod wordwrap;`
- Keep naming close to the original C names to simplify verification and review during migration.

## Data Model

The analysis lists only anonymous C data structures and does not identify stable named structs. This suggests either:
- local/temporary structs used within the C file, or
- analyzer-detected anonymous aggregates/macros that do not require direct public modeling.

Because no named cross-module data structures are provided, the Rust plan should be conservative:

### Data-Structure Mapping Policy
- **Anonymous C structs/unions used only for local temporary grouping**
  - Map to local Rust `struct` definitions inside `wordwrap.rs` only if they improve fidelity of the port.
  - Otherwise flatten their fields into local variables.

- **Anonymous flag/state carriers**
  - Map to `enum` when the C code represents mutually exclusive modes.
  - Map to `bool` or integer counters when the C code uses simple binary or numeric state.

- **Character/buffer traversal state**
  - Map raw pointer pairs to:
    - `usize` indices over `&[u8]`, or
    - iterator state if the C logic is naturally sequential and does not depend on random backtracking.

### Expected Rust Representations
Since the anonymous structures are unspecified, use only the minimum required representations:

| C construct | Rust mapping |
|---|---|
| anonymous local struct for scan state | private `struct ScanState { ... }` if needed |
| anonymous local struct for wrap boundaries | private `struct WrapBoundary { ... }` if needed |
| anonymous mode/flag grouping | private `enum` or flattened locals |
| char pointers into buffer | `usize` indices over `&[u8]` or slices |
| mutable output buffer | `String` or mutable byte buffer, depending on exact mutation pattern |

### Ownership Notes
- Inputs should be borrowed where possible.
- Outputs should own rewritten text only when wrapping changes require construction of a new buffer.
- If the original behavior mutates caller-provided storage, model that with `&mut String` or `&mut Vec<u8>` only if required by surrounding Rust APIs.

## Implementation Phases

## Phase 1: Translate module skeleton and scanning model
- Create `src/parseopt/wordwrap.rs`.
- Establish Rust signatures for `wordwrap_at_bol` and `wordwrap_at_eol` based on current Rust-side call expectations.
- Read through `src/parseopt/wordwrap.c` and identify:
  - whether operations are byte-oriented or text-oriented,
  - whether the functions mutate in-place or return derived positions/text,
  - which anonymous local structures are actually necessary.
- Convert raw pointer traversal to explicit indexed scanning over borrowed input.
- Keep function names and internal branch structure closely aligned with the C source for ease of comparison.

## Phase 2: Port wrap-boundary logic exactly
- Implement the full logic of `wordwrap_at_bol`.
- Implement the full logic of `wordwrap_at_eol`.
- Preserve original boundary conditions, especially:
  - empty input handling,
  - line-start and line-end detection,
  - whitespace and delimiter treatment,
  - off-by-one behavior around wrap positions.
- Introduce only minimal private helper structs/enums if repeated state needs to be carried across branches.

## Phase 3: Integrate with parseopt module and stabilize memory/error behavior
- Wire `wordwrap.rs` into the existing Rust parseopt module tree.
- Verify that ownership and borrowing fit the calling code without unnecessary cloning.
- Resolve any C assumptions about mutable buffers, sentinel terminators, or pointer comparisons into explicit Rust invariants.
- Ensure no unchecked indexing remains unless it is locally proven and documented in code comments.

## Phase 4: Add focused tests and equivalence checks
- Add unit tests covering:
  - beginning-of-line wrapping cases,
  - end-of-line wrapping cases,
  - empty and single-character inputs,
  - exact-width boundary cases,
  - inputs with consecutive spaces or separators if present in the C logic,
  - no-wrap and forced-wrap scenarios reflected by the original code.
- Add regression-style tests derived from observed C behavior where branch edges are subtle.
- Run `cargo test` and adjust implementation only to restore behavioral equivalence with the C source.

## Migration Notes

### Behavior Preservation Priorities
1. Keep exported function behavior identical.
2. Preserve byte/character boundary semantics used by the original implementation.
3. Avoid API redesign during the port.

### Items Explicitly Out of Scope
- introducing generalized word-wrapping frameworks,
- adding Unicode-aware segmentation beyond current C behavior,
- expanding module responsibilities beyond the two existing functions,
- adding concurrency or asynchronous interfaces.

## Acceptance Criteria

- `src/parseopt/wordwrap.c` has a direct Rust counterpart at `src/parseopt/wordwrap.rs`.
- Both `wordwrap_at_bol` and `wordwrap_at_eol` are implemented and integrated.
- Anonymous C data carriers are either eliminated or represented as minimal private Rust types.
- The port compiles cleanly and passes `cargo test`.
- The Rust module preserves the original module’s wrapping behavior without expanding scope.