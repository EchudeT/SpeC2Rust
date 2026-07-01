# Implementation Plan: module_src_parseopt_wordwrap_word_10

## Summary

This plan ports the word-boundary logic from `src/parseopt/wordwrap.c` into Rust on branch `107-module_src_parseopt_wordwrap_word_10-rust-port`. The scope is limited to the existing responsibilities represented by:

- `wordwrap_word_start`
- `wordwrap_word_end`

The Rust implementation should preserve the current behavior of locating word boundaries used by the word-wrapping path, while replacing pointer-based C logic with safe slice- and index-based Rust code. The preferred approach is to migrate the existing logic into a Rust module with narrowly scoped helper functions, using borrowed string or byte-slice inputs and returning indices or equivalent boundary positions rather than raw pointers.

The migration should avoid broad refactoring. Only the code needed to represent the current file and these functions should be introduced, and any anonymous C data layouts referenced by the original code should be translated only as far as this module requires.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep boundary-scanning operations linear in the inspected span.
  - Avoid unnecessary allocation during word-boundary detection.
  - Preserve behavior close to the C implementation for ASCII and existing byte-oriented parsing rules.
  - Use borrowing (`&str` or `&[u8]`) so the Rust port does not introduce extra copies.

## Module Mapping

### C to Rust File Mapping

- **C source**: `src/parseopt/wordwrap.c`
- **Rust target**: `src/parseopt/wordwrap.rs`

If the crate already exposes parseopt functionality through a parent module, register the migrated file there with the minimal required `mod` and re-export changes only as needed for current callers.

### Function Mapping

- `wordwrap_word_start`
  → `pub(crate)` Rust function in `src/parseopt/wordwrap.rs` with index-based traversal over input text/buffer.

- `wordwrap_word_end`
  → `pub(crate)` Rust function in `src/parseopt/wordwrap.rs` with index-based traversal over input text/buffer.

### Interface Direction

Because the C implementation likely works through raw character pointers, the Rust port should normalize this into one of the following internal forms, selected according to actual caller usage:

- `&[u8]` plus a current position index, if the original logic is byte-oriented.
- `&str` plus a byte index, only if the original logic operates on valid text boundaries and all callers already provide UTF-8 text.

Preference should be given to `&[u8]` if the C code treats bytes directly, since that most faithfully preserves the existing semantics.

## Data Model

The analysis reports only anonymous C data structures and does not identify named structs directly tied to this function pair. The migration should therefore avoid inventing broad Rust models and instead map only concrete state actually used by `wordwrap_word_start` and `wordwrap_word_end`.

### Data-Structure Mapping

- **Anonymous C structs/unions used only locally**
  → Replace with:
  - local Rust variables,
  - tuples,
  - small private structs in `src/parseopt/wordwrap.rs` only if needed to preserve readability of migrated state.

- **C `char *` / `const char *` ranges**
  → `&[u8]` with explicit indices, or `&str` with byte indices if UTF-8 validity is guaranteed by callers.

- **C pointer pair representing a span (`start`, `end`)**
  → `Range<usize>`, `(usize, usize)`, or separate `start` / `end` indices.

- **C integer flags / character classification temporaries**
  → `bool`, `u8`, `usize`, or small private enums if the original code has distinct branch states that benefit from explicit representation.

### Memory Management

The Rust implementation should use borrowed input data and stack-local state only. No heap allocation should be introduced unless the original function contract already requires owned output, which is not indicated here.

### Error Handling

If the original C functions assume valid non-null pointers and bounded ranges, the Rust version should encode those assumptions in the type signature:

- borrowed slices instead of nullable pointers,
- explicit indices checked before access,
- return values that make out-of-range conditions unrepresentable where possible.

No new recovery layer should be added. If callers can legally provide empty spans, handle that directly with deterministic boundary results.

## Implementation Phases

## Phase 1: Inspect and Freeze the Existing C Semantics

- Read `src/parseopt/wordwrap.c` and isolate the exact logic and call relationships for:
  - `wordwrap_word_start`
  - `wordwrap_word_end`
- Determine whether the original implementation:
  - scans bytes or logical characters,
  - depends on ASCII classification or locale-sensitive ctype behavior,
  - uses inclusive or exclusive boundary conventions,
  - expects pointer arguments into a larger buffer.
- Identify any anonymous structures or local state patterns actually touched by these functions.
- Record the exact caller-visible contract needed for the Rust signatures before coding.

## Phase 2: Create the Rust Module Skeleton and Port Core Logic

- Add `src/parseopt/wordwrap.rs`.
- Register the module in the existing Rust crate tree with the smallest necessary change.
- Port `wordwrap_word_start` and `wordwrap_word_end` directly, preserving branch order and boundary rules from C.
- Replace pointer arithmetic with:
  - slice indexing guarded by bounds checks,
  - explicit start/end indices,
  - helper predicates for character or byte classification only where they correspond directly to existing C checks.
- Keep helper functions private unless required by existing callers.

## Phase 3: Resolve Type and State Mapping Cleanly

- Translate any required anonymous C data usage into minimal Rust equivalents.
- Remove nullability assumptions by expressing required preconditions through function parameters.
- Ensure no unsafe Rust is used unless the original call pattern makes it unavoidable; if avoidable, prefer entirely safe code.
- Validate that empty input, single-byte input, and edge-position scans behave the same as the original implementation.

## Phase 4: Verification and Test Porting

- Add unit tests in the Rust module or the project’s existing test layout covering:
  - start-of-buffer behavior,
  - end-of-buffer behavior,
  - words surrounded by whitespace,
  - punctuation or delimiter transitions exactly as treated in C,
  - empty and one-character inputs,
  - repeated scans near boundaries.
- Where practical, derive test cases directly from the C logic and any existing examples around word wrapping.
- Run `cargo test` and adjust only for semantic parity issues, not stylistic rewrites.