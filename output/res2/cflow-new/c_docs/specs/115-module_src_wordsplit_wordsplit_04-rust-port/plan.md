# Implementation Plan

## Summary

This module ports the word-splitting and expansion logic currently implemented in `src/wordsplit/wordsplit.c` into Rust on branch `115-module_src_wordsplit_wordsplit_04-rust-port`.

The Rust implementation should preserve the existing control flow and processing order of the C code rather than redesigning the feature set. The technical approach is to migrate the current parsing, delimiter skipping, quoted-string scanning, list processing, and expansion helpers into a single Rust module with closely corresponding internal functions. State that was previously carried through mutable C structs, raw pointers, and owned buffers will be represented with explicit Rust structs, slices, `String`, and `Vec<String>`.

Special care is required in three areas:

- translating pointer/index-based scanning into byte-indexed or character-aware Rust traversal without changing behavior,
- preserving ownership boundaries for produced word lists and temporary expansion buffers,
- replacing C cleanup routines with Rust drop-based ownership while keeping explicit reset/free-style methods only where the public API requires equivalent lifecycle behavior.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Keep parsing and scanning operations linear in input size.
  - Avoid unnecessary intermediate allocations during delimiter scanning and word extraction.
  - Preserve behavior comparable to the C implementation for expansion and list processing.
  - Use `String`/`Vec` capacity management where straightforward, but do not introduce extra optimization layers beyond the migrated logic.

## Module Mapping

### Source Mapping

- **C source:** `src/wordsplit/wordsplit.c`
- **Rust target:** `src/wordsplit/wordsplit.rs`

If the crate already exposes a `wordsplit` module, the implementation should be added there and wired through the existing `mod.rs`/module declarations only as needed for this file migration. Do not split this port into additional submodules unless the current Rust tree already requires that shape.

### Function Mapping

Each C function should map to a Rust function with the same responsibility and near-equivalent call ordering:

- `wordsplit_tildexpand` -> `fn wordsplit_tildexpand(...) -> Result<..., WordSplitError>`
- `wordsplit_pathexpand` -> `fn wordsplit_pathexpand(...) -> Result<..., WordSplitError>`
- `skip_delim_internal` -> `fn skip_delim_internal(...) -> ...`
- `skip_delim` -> `fn skip_delim(...) -> ...`
- `skip_delim_real` -> `fn skip_delim_real(...) -> ...`
- `scan_qstring` -> `fn scan_qstring(...) -> Result<..., WordSplitError>`
- `scan_word` -> `fn scan_word(...) -> Result<..., WordSplitError>`
- `wordsplit_string_unquote_copy` -> `fn wordsplit_string_unquote_copy(...) -> String`
- `exptab_matches` -> `fn exptab_matches(...) -> bool`
- `wordsplit_process_list` -> `fn wordsplit_process_list(...) -> Result<..., WordSplitError>`
- `wordsplit_run` -> `fn wordsplit_run(...) -> Result<..., WordSplitError>`
- `wordsplit_len` -> `fn wordsplit_len(...) -> usize`
- `wordsplit` -> `pub fn wordsplit(...) -> Result<WordSplitResult, WordSplitError>`
- `wordsplit_free_words` -> `fn wordsplit_free_words(...)`
- `wordsplit_free_envbuf` -> `fn wordsplit_free_envbuf(...)`

Notes for migration:

- The free-style functions should usually become internal state-reset helpers that clear `Vec`/`String` buffers, not manual deallocation code.
- Public exposure should remain minimal and follow current crate needs; helper functions should stay private unless external call sites already depend on them.

## Data Model

The C analysis lists only anonymous structures, so the Rust data model should be derived from actual field usage in `wordsplit.c` and mapped conservatively.

### Core Mapping Rules

- **C parsing state structs** -> named Rust `struct`s with explicit fields
- **C flag sets / mode integers** -> `u32` bitflags-style constants or `enum` where field usage proves exclusivity
- **C string pointers (`char *`)** -> `String` for owned mutable text, `&str` for borrowed input, `Vec<u8>` only if byte-level mutation is required by the original logic
- **C arrays of strings (`char **`)** -> `Vec<String>`
- **C temporary linked or indexed lists** -> `Vec<T>` unless the original algorithm truly depends on node-level insertion/removal
- **C nullable pointers** -> `Option<T>` / `Option<&str>` / `Option<String>`
- **C lengths and indexes** -> `usize`
- **C status/error returns** -> `Result<T, WordSplitError>`

### Planned Rust Structures

Because the source contains anonymous C structures, define Rust names from usage rather than inventing extra abstractions:

- `WordSplitState`
  - main mutable state migrated from the primary wordsplit control structure
  - expected to hold:
    - original input
    - current scan position
    - delimiter configuration
    - output words
    - temporary expansion buffers
    - flags/options
    - any environment/path expansion state needed by this file

- `ScanState`
  - local scanning cursor/state if the C code uses a distinct anonymous helper struct for token scanning
  - only introduce this if the C file clearly maintains grouped scanning fields; otherwise keep fields on `WordSplitState`

- `ExpansionEntry` or `ExpansionRule`
  - for data behind `exptab_matches` if the C code uses an expansion table
  - represent as a small struct or enum matching actual table content

- `WordSplitResult`
  - if the Rust API needs to return owned output separate from internal mutable state
  - likely contains `Vec<String>` and any retained metadata already observable at call sites

- `WordSplitError`
  - enum covering parse/scan/expansion failures that the C code reports through integer return values or status fields
  - include variants only for actual error categories present in this file

### Memory Management Mapping

- Remove manual heap ownership patterns from C and replace them with Rust ownership.
- `wordsplit_free_words` should become a `clear_words`-style internal helper that empties the output vector when reusing state.
- `wordsplit_free_envbuf` should become a helper that clears temporary environment expansion buffers.
- Any C code that returns pointers into mutable buffers must be translated carefully so Rust returns owned strings or borrowed slices with valid lifetimes; prefer owned `String` results when in doubt to avoid unsafe lifetime coupling.

### Error Handling Mapping

- Integer return codes from scanning and expansion helpers should become `Result`.
- Boolean-style C helpers should map to `bool` when no diagnostic is needed.
- If the original implementation accumulates an error code in the state object, preserve that state field only if later logic depends on it; otherwise return early with `Result`.

## Implementation Phases

## Phase 1: Establish module skeleton and state translation

- Create or update `src/wordsplit/wordsplit.rs`.
- Identify the primary C state structure(s) used by the listed functions and define the minimum equivalent Rust structs.
- Port constant definitions, flags, and small helper data tables used directly by this file.
- Implement the top-level ownership model for:
  - input text
  - output words
  - temporary expansion/environment buffers
  - parser position/index state
- Add placeholder signatures for all listed functions so the migration preserves call topology from the start.
- Implement `wordsplit_free_words`, `wordsplit_free_envbuf`, and `wordsplit_len` first as straightforward safe state operations.

## Phase 2: Port scanning and tokenization logic

- Port delimiter-skipping helpers in order:
  - `skip_delim_internal`
  - `skip_delim`
  - `skip_delim_real`
- Port quoted-string and word scanning:
  - `scan_qstring`
  - `scan_word`
  - `wordsplit_string_unquote_copy`
- Translate pointer arithmetic to index-based scanning over the input buffer.
- Preserve exact boundary handling for:
  - empty segments
  - quoted spans
  - delimiter adjacency
  - end-of-input conditions
- Keep helper functions private and closely aligned with the C implementation to reduce behavioral drift.
- Add unit tests focused on scan boundaries and quote handling as each function lands.

## Phase 3: Port expansion and list processing

- Port expansion-table matching via `exptab_matches`.
- Port word/path/home expansion helpers:
  - `wordsplit_tildexpand`
  - `wordsplit_pathexpand`
- Port `wordsplit_process_list` using `Vec<String>` for staged output accumulation.
- Preserve the C processing order between scanning, unquoting, tilde expansion, and path expansion.
- Use standard-library filesystem/path handling only where it directly fits the existing behavior; otherwise keep string-based behavior to avoid semantic changes.
- Add tests for expansion-triggering and non-triggering cases based on current C behavior.

## Phase 4: Port orchestration and finalize public entry points

- Port execution flow:
  - `wordsplit_run`
  - `wordsplit`
- Ensure cleanup/reset behavior matches the original lifecycle expectations when the same state is reused across runs.
- Replace C-style status propagation with `Result` while preserving externally visible success/failure semantics.
- Validate that output word counts and contents match the C implementation for representative inputs.
- Run `cargo test` and complete targeted regression tests covering:
  - basic splitting
  - delimiter skipping
  - quoted words
  - unquoting
  - list processing
  - expansion paths
  - repeated invocation/reset behavior