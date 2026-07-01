# Implementation Plan

## Summary

This module covers the quote-oriented helper logic currently implemented in `src/wordsplit/wordsplit.c`, specifically the functions:

- `wordsplit_c_quoted_length`
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`
- `wordsplit_c_quote_copy`

The Rust implementation should migrate these routines as a narrowly scoped string/byte processing unit without extending behavior beyond the existing C semantics. The preferred approach is to port the logic into a single Rust module that operates on `&[u8]`, `&str`, and owned output buffers as appropriate, while preserving the original escape and quoting rules.

Because the source functions are low-level helpers, the Rust port should emphasize:

- exact control over indexing and output length
- explicit handling of escaped and quoted characters
- clear ownership of produced buffers
- minimal allocation beyond what the original copy/quote operations require

The implementation should remain closely aligned with the original file and function boundaries so that validation against the C behavior is straightforward.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Keep helper functions linear in input size.
  - Avoid unnecessary intermediate allocations.
  - Use byte-oriented processing where C logic is byte-based.
  - Preallocate destination buffers when output length can be derived from the input.

## Module Mapping

### C to Rust File Mapping

- `src/wordsplit/wordsplit.c`
  - migrate relevant quoting helpers into:
    - `src/wordsplit.rs` if this project already keeps module logic consolidated, or
    - `src/wordsplit/mod.rs` if the Rust crate already uses directory modules

Given the narrow scope of this migration, the preferred choice is to place the port in the existing Rust wordsplit module file rather than creating extra submodules.

### Function Mapping

- `wordsplit_c_quoted_length`
  - Rust: `fn quoted_length(input: &[u8]) -> usize` or `fn quoted_length(input: &str) -> usize`
- `wordsplit_c_unquote_char`
  - Rust: `fn unquote_char(ch: u8) -> u8` or `fn unquote_char(ch: char) -> char`
- `wordsplit_c_quote_char`
  - Rust: `fn quote_char(ch: u8, out: &mut Vec<u8>)` or equivalent helper returning encoded bytes
- `wordsplit_c_quote_copy`
  - Rust: `fn quote_copy(input: &[u8]) -> Vec<u8>` or `fn quote_copy(input: &str) -> String`

Preferred internal representation is byte-based if the C logic treats characters as raw bytes and escape handling is ASCII-oriented. Public-facing wrappers may use `&str` only if the surrounding Rust port already does so safely without changing semantics.

## Data Model

The analysis lists only anonymous C data structures and does not identify named structs directly tied to this function set. For this module scope, no new Rust data structures should be introduced unless required by surrounding migrated code.

### Data Structure Mapping

- Anonymous C helper/local state
  - Rust: local variables, tuple state, slice indices, and `Vec<u8>` buffers
- C character and string pointers used by these functions
  - Rust:
    - input views as `&[u8]` or `&str`
    - output accumulation as `Vec<u8>` or `String`
- C length/count values
  - Rust: `usize`
- C mutable destination buffer writes
  - Rust: `&mut Vec<u8>` or returned owned buffer

### Memory Management Decisions

- Replace manual buffer management with owned Rust containers.
- Preserve output sizing behavior by computing required length first where the C code does so implicitly or explicitly.
- Avoid unsafe code unless a specific pointer-based translation is unavoidable; for these helper functions, safe Rust should be sufficient.
- Keep transformations local and deterministic so lifetimes remain simple.

### Error Handling Decisions

These functions appear to be pure transformation helpers rather than error-rich parsing entry points. The Rust port should therefore:

- preserve infallible behavior if the C functions are infallible
- use plain return values instead of `Result` where no actual error state exists in the original logic
- only introduce `Option`/`Result` if the C logic has a distinct sentinel or invalid-input branch that must be represented explicitly

## Implementation Phases

## Phase 1: Extract and Define Rust Equivalents

- Inspect the exact logic of the four target functions in `src/wordsplit/wordsplit.c`.
- Determine whether the C implementation is byte-oriented, ASCII-specific, or assumes NUL-terminated strings.
- Create the Rust wordsplit module entry for these helpers in the existing crate layout.
- Define Rust function signatures that match the original semantics closely:
  - length computation
  - single-character quote/unquote helpers
  - quoted copy helper
- Document any C assumptions that must be preserved, especially:
  - escape character set
  - quoting rules
  - whether zero bytes are relevant or excluded by caller assumptions

## Phase 2: Port Core Logic with Minimal Structural Change

- Port `wordsplit_c_unquote_char` first as the smallest semantic unit.
- Port `wordsplit_c_quote_char` next, keeping emitted byte sequences identical to the C behavior.
- Port `wordsplit_c_quoted_length` using the same traversal rules as the C implementation.
- Port `wordsplit_c_quote_copy` last, reusing the prior helper functions where the C code already shares logic.
- Keep control flow close to the original implementation order to simplify comparison and review.
- Use safe indexing patterns and explicit iteration over bytes to avoid semantic drift from Unicode-aware character iteration.

## Phase 3: Validate Behavior and Edge Cases

- Add unit tests for each helper function based on behavior observed in the C code.
- Cover:
  - empty input
  - already unescaped characters
  - escapable characters
  - quote characters
  - mixed escaped/unescaped sequences
  - output length consistency between `quoted_length` and `quote_copy`
- Add targeted regression tests for boundary conditions such as:
  - single-byte inputs
  - trailing escape-related characters if relevant
  - repeated quoting characters
- Confirm that output bytes and lengths match the C implementation for representative cases.

## Phase 4: Integrate and Finalize Module Migration

- Replace or wire up any existing call sites in the Rust port to use the new helper functions.
- Ensure there is no duplicate logic for quote copying or quoted-length calculation elsewhere in the Rust branch.
- Keep the migrated implementation confined to the wordsplit module without creating additional abstraction layers.
- Run `cargo test` and resolve any semantic mismatches discovered during integration.
- Perform a final pass for:
  - ownership clarity
  - absence of unnecessary allocations
  - close correspondence between original C helpers and Rust counterparts