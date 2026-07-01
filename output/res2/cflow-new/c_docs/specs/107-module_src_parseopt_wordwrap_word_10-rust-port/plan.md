# Implementation Plan

## Summary

Port `src/parseopt/wordwrap.c` into a Rust module that preserves the existing word-boundary behavior implemented by:

- `wordwrap_word_start`
- `wordwrap_word_end`

The Rust implementation should stay narrowly scoped to the current module responsibilities: identifying the start and end of a word within wrapped text processing. The preferred approach is a direct translation of the existing control flow and boundary checks into safe Rust, using string/byte-slice indexing carefully so behavior remains aligned with the C implementation.

Because the analyzed input only exposes two functions and anonymous C data structures, the migration should prioritize function-level parity first and introduce Rust data types only where required by the existing code paths. The implementation should avoid adding new abstractions beyond what is necessary to represent the current logic and state.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s asymptotic behavior for word-boundary scanning.
  - Avoid unnecessary allocation during scanning; prefer borrowed `&str` or byte slices.
  - Keep per-call overhead low and limit conversions between `&str` and byte representations.
  - Preserve deterministic behavior for boundary detection on the same inputs.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` | Direct module port of the existing word-wrap helper logic. |
| `wordwrap_word_start` | `pub(crate)` or private Rust function in `src/parseopt/wordwrap.rs` | Visibility should match actual call sites in the Rust port; do not widen unless required. |
| `wordwrap_word_end` | `pub(crate)` or private Rust function in `src/parseopt/wordwrap.rs` | Preserve scanning semantics and edge-case handling. |

If the Rust project already exposes a `parseopt` module tree, the new file should be registered in the existing `mod` structure without creating additional layers.

## Data Model

The analysis lists only anonymous C data structures and does not provide named fields. Therefore, data-structure migration should be limited to the minimum required by the functions being ported.

| C Data Structure | Rust Representation | Migration Decision |
|---|---|---|
| anonymous struct/union instances used internally by `wordwrap.c` | Inline Rust locals, tuples, or small private structs as needed | Do not predeclare placeholder types without field evidence. |
| C string inputs (`char *`, `const char *`, or equivalent) | `&str` when valid UTF-8 is already guaranteed by surrounding Rust code; otherwise `&[u8]` | Prefer `&[u8]` if the C logic is byte-oriented and index-sensitive. |
| C index/pointer arithmetic | `usize` indices over slices | Replace raw pointer walking with bounds-checked indexing. |
| C boolean/integer condition flags | `bool`, `usize`, or `isize` as appropriate | Keep types close to actual comparison behavior. |

### Data Handling Notes

- If the original C logic treats text as raw bytes, the Rust port should also operate on byte slices rather than character iteration, to preserve exact boundary behavior.
- If the original logic returns positions via pointers, convert that behavior into index-based return values in Rust where practical.
- Any implicit sentinel-based behavior in C must become explicit bounds checks in Rust.

## Implementation Phases

### Phase 1: Extract and Map Existing Logic

- Inspect `src/parseopt/wordwrap.c` and isolate the exact inputs, outputs, and helper dependencies of:
  - `wordwrap_word_start`
  - `wordwrap_word_end`
- Determine whether the C implementation is:
  - byte-oriented,
  - ASCII-classification-oriented,
  - or character-oriented.
- Identify all anonymous temporary structures actually involved in these functions.
- Define the Rust file location and module declarations needed to host the port in `src/parseopt/wordwrap.rs`.

**Exit criteria**:
- Function signatures for the Rust equivalents are chosen.
- Any required local/private support types are identified from actual usage only.

### Phase 2: Port Core Functions Safely

- Implement Rust equivalents of `wordwrap_word_start` and `wordwrap_word_end` with control flow matching the C source.
- Replace pointer arithmetic with:
  - slice indexing,
  - index advancement/decrement,
  - explicit bounds checks.
- Preserve edge-case behavior for:
  - empty input,
  - start/end-of-buffer positions,
  - contiguous separator regions,
  - single-word and single-character cases.
- Keep helper logic in the same file unless the surrounding Rust project already mandates a different existing placement.

**Exit criteria**:
- The Rust functions compile and cover all identified branches from the C source.
- No unsafe Rust is introduced unless the C behavior cannot otherwise be matched; if unavoidable, isolate and document it narrowly.

### Phase 3: Integrate With Existing Parseopt Module

- Wire the Rust `wordwrap` module into the existing parseopt module tree.
- Update any call sites being migrated on this branch to use the Rust functions instead of C equivalents.
- Align function visibility with real usage:
  - private if internal,
  - `pub(crate)` only if needed within the crate.
- Remove any temporary compatibility code created during the port, if present.

**Exit criteria**:
- The Rust module is reachable from the same logical subsystem as the original C file.
- Call-site behavior remains unchanged from the C implementation.

### Phase 4: Add Focused Tests and Final Verification

- Add unit tests in or alongside `src/parseopt/wordwrap.rs` covering:
  - empty input,
  - leading and trailing delimiters,
  - words surrounded by separators,
  - multiple adjacent separators,
  - boundary positions at index `0` and at the last byte,
  - representative cases taken from the original C behavior.
- Run `cargo test` and fix any mismatches between expected C semantics and Rust behavior.
- Review for:
  - off-by-one errors,
  - invalid UTF-8 assumptions,
  - accidental allocations,
  - incorrect signed/unsigned conversions.

**Exit criteria**:
- `cargo test` passes.
- Behavior of `wordwrap_word_start` and `wordwrap_word_end` is validated against the original module semantics.