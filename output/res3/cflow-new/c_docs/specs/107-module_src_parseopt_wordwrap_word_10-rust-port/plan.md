# Implementation Plan

## Summary

Port the C module `src/parseopt/wordwrap.c` into a Rust module that preserves the existing behavior of the word-boundary helpers `wordwrap_word_start` and `wordwrap_word_end` without broadening scope.

The Rust implementation should translate the current logic directly, keeping the same operational boundaries:
- compute the start position of a word within a text buffer
- compute the end position of a word within a text buffer

The technical approach should favor:
- direct migration of the existing functions into a focused Rust source file
- slice/index-based processing over raw pointer arithmetic
- explicit bounds handling to replace implicit C memory assumptions
- minimal internal helper types only if required to represent currently anonymous C data layouts encountered during migration

No new parsing abstractions or formatting subsystems should be introduced. The goal is a narrow, behavior-preserving Rust port aligned with the existing file and function responsibilities.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear scan behavior equivalent to the C implementation
  - Avoid unnecessary allocation in word-boundary calculations
  - Operate on borrowed string/byte slices where possible
  - Keep branch structure close to the original logic to minimize migration risk

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/wordwrap.c`
  → `src/parseopt/wordwrap.rs`

### Function Mapping

- `wordwrap_word_start`
  → `pub(crate) fn wordwrap_word_start(...) -> ...`
- `wordwrap_word_end`
  → `pub(crate) fn wordwrap_word_end(...) -> ...`

### Module Placement

Follow standard Rust project layout and keep the port local to the existing parseopt area:

- `src/parseopt/mod.rs`
  - declare `mod wordwrap;` if not already present
- `src/parseopt/wordwrap.rs`
  - contains the Rust port of the two functions
  - contains any file-local helper routines strictly required to express the C logic safely

If the current Rust tree already has a corresponding parseopt module structure, integrate into that structure directly rather than adding parallel organization.

## Data Model

The analysis input lists only anonymous C data structures and does not identify any named struct used by the target functions. The plan should therefore avoid inventing replacement types unless inspection of `wordwrap.c` shows the two functions depend on local anonymous layouts.

### Expected Mapping Strategy

- **C anonymous/local aggregate used only within function scope**
  → Rust local tuple, local struct, or direct variables, whichever most closely preserves control flow
- **C pointer to character buffer**
  → `&[u8]` or `&str`, depending on whether the original logic is byte-oriented or character-oriented
- **C integer indexes / offsets**
  → `usize` for validated positions; `isize` only if the original algorithm requires backward sentinel movement before validation
- **C boolean/int condition flags**
  → `bool` or small integer types only when exact state encoding matters

### Constraints for Data Conversion

- Prefer `&[u8]` if the original C code classifies bytes directly and is not Unicode-aware.
- Prefer `&str` only if the original logic clearly operates on text characters rather than raw bytes.
- Do not expose anonymous C layout artifacts as public Rust types.
- Keep any replacement types private to `wordwrap.rs` unless another already-existing Rust module requires access.

## Implementation Phases

### Phase 1: Source Inspection and Signature Locking

- Inspect `src/parseopt/wordwrap.c` to determine:
  - exact signatures of `wordwrap_word_start` and `wordwrap_word_end`
  - whether they operate on mutable buffers, immutable buffers, indexes, or pointer ranges
  - whether the logic is byte-based, ASCII-classification-based, or character-based
  - whether any anonymous structs/unions are actually referenced by these functions
- Define the Rust function signatures to preserve call-site expectations as closely as possible within safe Rust.
- Decide the minimal module imports and whether any internal helper function is necessary for repeated boundary checks.

**Deliverable**:
- Rust file skeleton at `src/parseopt/wordwrap.rs`
- finalized function signatures matching current integration needs

### Phase 2: Core Logic Port

- Port `wordwrap_word_start` directly from C control flow into safe Rust.
- Port `wordwrap_word_end` directly from C control flow into safe Rust.
- Replace:
  - pointer arithmetic with index arithmetic
  - unchecked dereferences with slice access guarded by bounds checks
  - sentinel/null assumptions with explicit `Option`-style branching only where needed
- Preserve original classification rules for word characters, separators, and scan termination.
- Keep helper logic private and minimal; do not refactor into broader reusable text utilities.

**Deliverable**:
- behaviorally equivalent Rust implementation of both functions
- no unsafe code unless the C semantics cannot be expressed otherwise; if unavoidable, isolate and document the exact invariant

### Phase 3: Integration and Edge-Case Validation

- Wire the new module into the Rust crate’s existing parseopt module tree.
- Update any internal callers to use the Rust functions and Rust-native parameter types.
- Add unit tests covering:
  - empty input
  - single-word input
  - leading and trailing separators
  - mid-buffer start positions
  - boundary positions at index `0` and at buffer end
  - repeated separators or punctuation patterns reflected in the C behavior
- Confirm `cargo test` passes and that no ownership or lifetime issues leak outside the module.

**Deliverable**:
- integrated module build
- focused unit test coverage for migrated behavior

### Phase 4: Final Parity Review

- Compare Rust results against the C implementation for representative cases derived from the original control paths.
- Verify:
  - index return semantics match the C version
  - no allocation was introduced in hot scanning paths
  - error/edge handling is explicit and consistent with original assumptions
- Remove any temporary migration scaffolding that is not required by the final port.

**Deliverable**:
- final behavior-preserving Rust port ready on branch `107-module_src_parseopt_wordwrap_word_10-rust-port`