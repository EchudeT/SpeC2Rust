# Implementation Plan: module_src_wordsplit_wordsplit_c_07

## Summary

This module covers the quote-oriented helper routines currently implemented in `src/wordsplit/wordsplit.c`:

- `wordsplit_c_quoted_length`
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`
- `wordsplit_c_quote_copy`

The Rust implementation should port these routines as a small, self-contained module that preserves existing behavior and call ordering, with emphasis on byte-accurate string scanning, escaping, and quoted-copy logic. The technical approach is to translate the C logic into safe Rust over `&[u8]`, `&str`, and owned `String`/`Vec<u8>` as appropriate, avoiding feature expansion and keeping the code close to the original control flow.

The work should remain narrowly scoped to migrating the existing file functionality from the identified C region into one Rust source module on branch `118-module_src_wordsplit_wordsplit_c_07-rust-port`. Memory ownership should move from pointer-based buffer management to Rust-owned outputs and explicit return values, while preserving the same transformation semantics.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time scanning and copy behavior for quoted/unquoted text paths.
  - Avoid unnecessary intermediate allocations where output size can be estimated.
  - Keep character handling close to the original byte-processing model unless the C logic clearly depends on Unicode-aware semantics.
  - Match C behavior predictably for ASCII quoting and escaping cases.

## Module Mapping

### C to Rust File Mapping

- **C source**: `src/wordsplit/wordsplit.c`
- **Rust target**: `src/wordsplit.rs` or `src/wordsplit/mod.rs`

If the Rust project already has a `wordsplit` module, the ported functions should be added there directly rather than introducing extra submodules. If no such module exists yet, create a single `wordsplit` module and place only the migrated functions from this C slice into it.

### Function Mapping

- `wordsplit_c_quoted_length`
  - Rust: `fn quoted_length(...) -> ...`
  - Responsibility: compute output/input span associated with quoted content, preserving the original scan rules.

- `wordsplit_c_unquote_char`
  - Rust: `fn unquote_char(...) -> ...`
  - Responsibility: decode or normalize a quoted/escaped character according to the original C helper logic.

- `wordsplit_c_quote_char`
  - Rust: `fn quote_char(...) -> ...`
  - Responsibility: encode or escape a character into quoted form.

- `wordsplit_c_quote_copy`
  - Rust: `fn quote_copy(...) -> ...`
  - Responsibility: copy an input sequence into an output buffer/string while applying the same quoting rules as the C implementation.

### API Shape Guidance

Prefer private or `pub(crate)` functions unless the existing Rust crate structure requires wider visibility. Signatures should reflect actual ownership:

- Use `&str` when the logic is text-oriented and original behavior is ASCII-safe.
- Use `&[u8]` when preserving raw C byte semantics is important.
- Return `String`, `Vec<u8>`, `usize`, `Option<_>`, or `Result<_, _>` depending on whether the original C function could fail or signal malformed quoting.

## Data Model

The provided analysis lists only anonymous data structures and does not identify any named struct directly tied to these four functions. Therefore, the migration plan should avoid inventing replacement types unless required by the existing Rust crate API.

### Data-Structure Mapping

- **C anonymous structs/unions used elsewhere in `wordsplit.c`**
  - **Rust mapping**: no dedicated mapping in this module unless one is already required by surrounding ported code.

### Local Representation Decisions

For this module slice, prefer simple Rust primitives:

- C `char *` input buffer -> `&[u8]` or `&str`
- C output buffer pointer -> `String` or `Vec<u8>`
- C length/count values -> `usize`
- C single character values -> `u8` or `char`
- C status/error sentinel returns -> `Option<T>` or `Result<T, WordSplitError>`

If these functions are tightly coupled to an existing parser state in the broader port, pass minimal references to that existing Rust state instead of introducing new standalone wrapper structs.

### Memory Management Notes

- Replace caller-managed output buffers with owned Rust outputs or explicitly borrowed mutable buffers (`&mut String`, `&mut Vec<u8>`).
- Eliminate manual null-termination concerns; only preserve them if a surrounding compatibility layer explicitly requires byte buffers.
- Avoid unsafe code unless exact pointer arithmetic from the C implementation cannot be expressed safely; this module should normally be fully safe Rust.

### Error Handling Notes

Where the C code uses sentinel values or implicit failure conventions:

- Map malformed escape/quote cases to `Result` if failure must be surfaced.
- Use `Option` only for simple “not decodable / not applicable” outcomes.
- Do not add recovery behavior; preserve existing failure boundaries.

## Implementation Phases

## Phase 1: Extract and Map C Semantics

- Isolate the exact logic for:
  - `wordsplit_c_quoted_length`
  - `wordsplit_c_unquote_char`
  - `wordsplit_c_quote_char`
  - `wordsplit_c_quote_copy`
- Identify:
  - whether they operate on raw bytes or C characters with only ASCII assumptions,
  - whether they depend on surrounding constants/macros,
  - whether malformed quote sequences are possible and how C signals them.
- Define direct Rust signatures matching current call usage in the crate.
- Record any dependencies on shared helper functions from `wordsplit.c` and migrate only the minimum required support code.

**Exit criteria**:
- Finalized Rust function signatures.
- Clear mapping of each C return/output convention to Rust return types.

## Phase 2: Implement Core Helpers in Rust

- Port `wordsplit_c_unquote_char` and `wordsplit_c_quote_char` first, since they are likely the smallest behavior units and may be reused by the copy routine.
- Port `wordsplit_c_quoted_length` with the same scan boundaries and length counting behavior as C.
- Implement `wordsplit_c_quote_copy` using the Rust helper functions and owned/buffered output handling.
- Keep control flow close to the C implementation to reduce migration risk.
- Use standard-library string/byte facilities only.

**Exit criteria**:
- All four functions compiled in Rust.
- No placeholder logic remains.
- Ownership and buffer handling are fully safe.

## Phase 3: Integrate with Existing Module Structure

- Replace or wire up existing call sites in the Rust port to use the new functions.
- Ensure the migrated code lives in the designated `wordsplit` module without creating extra abstraction layers.
- Align visibility (`fn`, `pub(crate)`) with actual usage.
- Remove any temporary compatibility scaffolding introduced during translation if it is no longer needed.

**Exit criteria**:
- The Rust module builds cleanly with the new implementations in use.
- No duplicated quote-handling paths remain for this migrated slice.

## Phase 4: Verification and Behavioral Tests

- Add unit tests covering:
  - quoted length calculation for representative quoted and escaped inputs,
  - unquoting of valid escaped characters,
  - quoting of characters that do and do not require escaping,
  - quote-copy behavior across mixed plain/escaped content,
  - edge cases such as empty input, trailing escape markers, and boundary characters.
- Where practical, derive test cases from observed C behavior rather than reinterpreting semantics.
- Run `cargo test` and fix any mismatches between Rust and the original control flow.

**Exit criteria**:
- Tests cover the migrated functions directly.
- `cargo test` passes.
- Behavior matches the original C semantics for the covered cases.