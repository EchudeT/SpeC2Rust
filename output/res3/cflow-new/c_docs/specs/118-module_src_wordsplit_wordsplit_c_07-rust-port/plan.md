# Implementation Plan: module_src_wordsplit_wordsplit_c_07

## Summary

This module covers the quote-oriented helper logic currently implemented in `src/wordsplit/wordsplit.c`, specifically the functions:

- `wordsplit_c_quoted_length`
- `wordsplit_c_unquote_char`
- `wordsplit_c_quote_char`
- `wordsplit_c_quote_copy`

The Rust implementation should migrate these routines as a small, self-contained module that preserves existing parsing and escaping behavior without introducing broader API redesign. The technical approach is to translate the C byte-oriented logic into Rust string/byte processing with explicit bounds checks, using `&[u8]`, `&str`, and `String` where appropriate. Allocation should be limited to the cases corresponding to C output-buffer construction, while pure length/counting logic should remain non-allocating.

The implementation should stay close to the original control flow so behavior remains comparable to the C source. Error handling should replace implicit C failure modes such as null pointers or unchecked buffer writes with explicit `Result` or well-bounded internal helpers, but only where required by the migrated call sites.

## Technical Context

### Language / Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library only

No third-party crate is required based on the available module scope. The functions are narrow string/character transformation helpers and can be implemented with `std` facilities alone.

### Testing
- `cargo test`

Testing should focus on:
- direct function-level unit tests for quoting and unquoting behavior
- round-trip cases where quoting followed by unquoting reproduces the expected bytes/chars
- boundary cases such as empty input, single quote characters, backslash-escaped content, and mixed quoted/unquoted segments
- parity checks against behavior inferred from the original C logic for supported inputs

### Performance Goals
- Preserve the original helper-level efficiency characteristics
- Keep `quoted_length` computation allocation-free
- Use linear-time scanning over the input
- Avoid unnecessary intermediate buffers during quote copying
- Ensure output growth is predictable and reserve capacity when constructing `String` results

## Module Mapping

### C to Rust File Mapping
- `src/wordsplit/wordsplit.c`
  - migrate the targeted helper functions into a Rust module under the project’s normal source tree, preferably:
  - `src/wordsplit.rs`
  - or, if this project already uses directory modules for `wordsplit`, `src/wordsplit/mod.rs`

The implementation should be placed in the existing Rust wordsplit area rather than introducing a new parallel subsystem.

### Function Mapping
- `wordsplit_c_quoted_length`
  - Rust function with equivalent counting semantics
  - likely signature centered on `&str` or `&[u8]` depending on the C logic’s byte assumptions

- `wordsplit_c_unquote_char`
  - Rust helper returning the decoded character/byte and consumed input width as needed
  - should use explicit return values instead of pointer mutation

- `wordsplit_c_quote_char`
  - Rust helper that appends escaped/quoted form into a destination `String` or byte buffer

- `wordsplit_c_quote_copy`
  - Rust helper for copying a source segment into a destination while applying quote/escape rules
  - should own destination growth safely instead of emulating raw pointer writes

## Data Model

The provided analysis lists only anonymous data structures and does not identify named structs directly tied to these four functions. Therefore the migration plan should keep data-model changes minimal.

### C Structure Mapping
- anonymous C structs/unions used elsewhere in `wordsplit.c`
  - no new Rust struct is required for this helper-only migration unless an existing wordsplit state type already exists in the Rust branch
  - if these functions currently depend on parser state or flags from enclosing anonymous C layouts, map only the actually used fields into an existing Rust `struct` or a small dedicated Rust configuration type local to the wordsplit module

### Rust Data Representation Decisions
- C `char *` input buffers
  - map to `&[u8]` when exact byte semantics are required
  - map to `&str` only if the original logic clearly operates on textual ASCII-compatible content and call sites already validate UTF-8

- C output buffer plus pointer arithmetic
  - map to `String` for textual escaped output
  - map to `Vec<u8>` only if byte preservation beyond UTF-8 assumptions is necessary

- C integer lengths/counters
  - map to `usize`

- C status/error signaling through sentinel values
  - map to `Option<T>` or `Result<T, E>` only where the original function can fail meaningfully
  - avoid inventing broad error enums if simple local error signaling is sufficient

- C single-character escape handling
  - represent as `u8` for byte-exact logic or `char` for text-oriented logic, chosen according to the source behavior and existing Rust wordsplit APIs

## Implementation Phases

### Phase 1: Source Analysis and Rust Module Skeleton
- Inspect the exact bodies and call sites of:
  - `wordsplit_c_quoted_length`
  - `wordsplit_c_unquote_char`
  - `wordsplit_c_quote_char`
  - `wordsplit_c_quote_copy`
- Determine whether their semantics are byte-based or UTF-8 text-based
- Identify any shared constants, escape tables, or flag checks used by these functions in `wordsplit.c`
- Place the Rust destination code into the existing wordsplit module file structure
- Define minimal internal helper signatures that match current Rust-side call patterns without expanding public surface area unnecessarily

### Phase 2: Port Core Helper Logic
- Implement `wordsplit_c_unquote_char` first, because it is the lowest-level decoding helper
- Implement `wordsplit_c_quote_char` next, using safe destination appending instead of writable raw buffers
- Implement `wordsplit_c_quoted_length` as a read-only scan using the same escape rules as the quote/unquote helpers
- Implement `wordsplit_c_quote_copy` last, reusing the lower-level quoting helper to minimize duplicate logic
- Preserve original edge-case behavior closely, including consumed-length rules and treatment of escape prefixes

### Phase 3: Integrate With Existing Wordsplit Flow
- Replace or wire the corresponding C-derived call paths in the Rust branch to use the new helpers
- Remove any temporary duplicated logic introduced during porting
- Ensure ownership and borrowing are clear at call sites, especially where C previously relied on in-place pointer advancement
- Verify that no unchecked indexing or invalid UTF-8 assumptions remain in the migrated paths

### Phase 4: Verification and Cleanup
- Add unit tests for each migrated function using representative inputs from the original logic
- Add focused integration tests for any parser path that depends on quote copying/unquoting behavior
- Confirm output length calculations match produced quoted output where applicable
- Perform final cleanup of signatures, visibility, and comments so the module remains limited to the migrated functionality only