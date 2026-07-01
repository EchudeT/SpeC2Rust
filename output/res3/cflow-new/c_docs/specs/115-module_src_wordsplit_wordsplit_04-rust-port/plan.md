# Implementation Plan

## Summary

Port `src/wordsplit/wordsplit.c` into a Rust module that preserves the current parsing and expansion flow for word splitting, quoting, delimiter skipping, list processing, and cleanup-related behavior. The Rust implementation should stay close to the existing execution order and function boundaries so migration can proceed function-by-function rather than redesigning the subsystem.

The technical approach is a direct C-to-Rust translation centered on:
- replacing pointer-and-buffer manipulation with slice/index-based parsing,
- representing mutable parser state with owned Rust structs,
- converting cleanup functions into ownership-driven drop behavior where possible,
- keeping explicit helper functions for expansion, scanning, and delimiter traversal aligned with the current C entry points.

The plan should avoid introducing new capabilities or reorganizing this module into broader parser frameworks. The target is a conservative Rust port on branch `115-module_src_wordsplit_wordsplit_04-rust-port`.

## Technical Context

- **Language/Version:** Rust 1.78+ stable
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain linear-time scanning over input for the common path.
  - Avoid unnecessary string cloning during scan and split stages.
  - Preserve predictable memory growth for word/result buffers.
  - Keep expansion and unquote operations bounded to the minimum required allocations.

## Module Mapping

### C to Rust File Mapping

- `src/wordsplit/wordsplit.c`
  - `src/wordsplit/wordsplit.rs`

### Rust Module Placement

Use the existing Rust crate layout and place the port in a module matching the source area:

- `src/wordsplit/mod.rs`
- `src/wordsplit/wordsplit.rs`

If the crate already has a `wordsplit` module root, only add `wordsplit.rs` and wire it through the existing `mod` declarations. Do not introduce extra helper modules unless required by compilation constraints in this file’s direct translation.

### Function Mapping

Preserve the current function-level decomposition as closely as practical:

| C Function | Rust Target |
|---|---|
| `wordsplit_tildexpand` | `fn wordsplit_tildexpand(...) -> Result<..., WordSplitError>` |
| `wordsplit_pathexpand` | `fn wordsplit_pathexpand(...) -> Result<..., WordSplitError>` |
| `skip_delim_internal` | `fn skip_delim_internal(...) -> ...` |
| `skip_delim` | `fn skip_delim(...) -> ...` |
| `skip_delim_real` | `fn skip_delim_real(...) -> ...` |
| `scan_qstring` | `fn scan_qstring(...) -> Result<..., WordSplitError>` |
| `scan_word` | `fn scan_word(...) -> Result<..., WordSplitError>` |
| `wordsplit_string_unquote_copy` | `fn wordsplit_string_unquote_copy(...) -> String` or `Result<String, WordSplitError>` depending on failure cases |
| `exptab_matches` | `fn exptab_matches(...) -> bool` |
| `wordsplit_process_list` | `fn wordsplit_process_list(...) -> Result<..., WordSplitError>` |
| `wordsplit_run` | `fn wordsplit_run(...) -> Result<..., WordSplitError>` |
| `wordsplit_len` | `fn wordsplit_len(...) -> usize` |
| `wordsplit` | `pub fn wordsplit(...) -> Result<WordSplitResult, WordSplitError>` |
| `wordsplit_free_words` | removed as explicit API if ownership suffices; retained as internal `clear_words` only if call-site compatibility requires it |
| `wordsplit_free_envbuf` | removed as explicit API if ownership suffices; retained as internal reset helper only if needed |

## Data Model

The source analysis only exposes anonymous C data structures, so the Rust port should derive named internal types from actual usage in `wordsplit.c` rather than inventing new abstractions. The mapping should be usage-driven and minimal.

### Core Mapping Rules

- **Anonymous C structs used as persistent parser state**
  - Map to named Rust `struct`s with owned fields.
  - Prefer `String`, `Vec<String>`, `Vec<u8>`, `Option<T>`, and index counters.

- **Anonymous C structs used as tagged variants or mode/state selectors**
  - Map to Rust `enum`s.

- **Linked temporary nodes or expandable lists**
  - Map to `Vec<T>` unless strict insertion semantics require another structure present in the C logic.
  - Do not preserve heap-node shape if the C code only uses it as an append/process/free list.

- **Character pointers into source buffers**
  - Map to `&str` plus byte index, or `&[u8]` plus index where bytewise parsing is clearer.
  - Use byte indexing only where the original logic is byte-oriented.

- **Output word arrays**
  - Map to `Vec<String>`.

- **Temporary mutable output buffers**
  - Map to `String` for textual data or `Vec<u8>` if escape handling is byte-based before UTF-8 validation.
  - Prefer `String` if the C logic is fundamentally text-oriented and all inputs are assumed string data.

### Planned Rust Types

Because the C structs are anonymous in the analysis, define only the minimum named types required by this file:

| C Shape | Rust Type |
|---|---|
| parser/session state struct | `struct WordSplitState` |
| split result storage | `struct WordSplitResult` |
| expansion table entry / matcher record | `struct ExpansionEntry` or `enum ExpansionKind` as indicated by actual usage |
| temporary scanned token/word item | `struct ScannedWord` |
| parser mode / quote mode flags | `enum QuoteMode` and/or bitflags represented as plain integers/constants if needed for compatibility |

### Memory Management Mapping

- C manual allocation/free pairs should become Rust ownership:
  - word lists become `Vec<String>`
  - environment/expansion buffers become owned `String`/`Vec<String>`
  - temporary scan buffers become local variables dropped automatically
- `wordsplit_free_words` and `wordsplit_free_envbuf` should become no-op from a resource-management perspective unless the surrounding API shape requires explicit reset semantics. In that case, implement them as methods that `clear()` owned buffers rather than manually freeing memory.

### Error Handling Mapping

- Replace integer return codes and sentinel error states with `Result<T, WordSplitError>`.
- Use a module-local error enum:
  - parse errors,
  - unterminated quote conditions,
  - expansion-related failures,
  - invalid internal state where the C code would propagate failure.
- Keep error granularity limited to what the current C function boundaries already distinguish.

## Implementation Phases

## Phase 1: Establish Rust State and Public Entry Mapping

### Goals
Create the Rust file and define the minimum state, result, and error types needed to support a direct port of the current top-level control flow.

### Tasks
- Add `src/wordsplit/wordsplit.rs`.
- Wire the module through `src/wordsplit/mod.rs` only as needed.
- Define:
  - `WordSplitState`
  - `WordSplitResult`
  - `WordSplitError`
- Port the top-level entry and utility functions first:
  - `wordsplit`
  - `wordsplit_run`
  - `wordsplit_len`
- Translate output ownership from C buffers/arrays to `Vec<String>` and owned state fields.
- Identify where explicit free functions correspond to state reset rather than real deallocation.

### Notes
This phase should not finalize all parsing details; it should establish compileable scaffolding and preserve the top-level call sequence.

## Phase 2: Port Core Scanning and Delimiter Logic

### Goals
Move the input traversal logic into Rust using slice/index-based parsing while preserving current scan boundaries and quote behavior.

### Tasks
- Port delimiter helpers:
  - `skip_delim_internal`
  - `skip_delim`
  - `skip_delim_real`
- Port token scanning helpers:
  - `scan_qstring`
  - `scan_word`
- Port `wordsplit_string_unquote_copy`.
- Choose a consistent input representation:
  - `&str` with byte indices for text scanning, or
  - `&[u8]` if the original code relies on raw byte behavior.
- Keep helper signatures narrow and stateful only where required by the C control flow.

### Notes
Be careful to preserve:
- quote termination rules,
- delimiter consumption rules,
- escape/unquote copying behavior,
- index advancement semantics previously encoded by pointer updates.

## Phase 3: Port Expansion and List Processing

### Goals
Translate the module’s expansion-related logic and any temporary list processing without redesigning the expansion model.

### Tasks
- Port:
  - `wordsplit_tildexpand`
  - `wordsplit_pathexpand`
  - `exptab_matches`
  - `wordsplit_process_list`
- Replace temporary C list structures with `Vec`-backed storage where traversal order remains the same.
- Keep expansion application order identical to the C implementation.
- Limit filesystem or path-related behavior to what is already implied by these functions; do not broaden feature scope.

### Notes
This phase should maintain current matching and expansion sequencing exactly enough that existing behavior remains recognizable to callers.

## Phase 4: Cleanup Semantics, Tests, and Behavioral Tightening

### Goals
Finish the migration by resolving cleanup APIs, validating ownership semantics, and locking behavior with tests.

### Tasks
- Port or collapse:
  - `wordsplit_free_words`
  - `wordsplit_free_envbuf`
- If external call patterns require these names, implement them as state-clearing methods rather than manual free routines.
- Add unit tests covering:
  - delimiter skipping,
  - quoted string scanning,
  - word scanning,
  - unquote copy behavior,
  - top-level split results,
  - cleanup/reset behavior if exposed.
- Run `cargo test` and fix mismatches caused by ownership or index translation.

### Notes
Testing should be based on observed behavior from the C implementation and current module expectations, not on new parser features or generalized infrastructure.

## Completion Criteria

The module migration is complete when:
- all listed C functions have Rust equivalents or are intentionally absorbed into ownership-based state methods,
- `src/wordsplit/wordsplit.c` functionality is represented in `src/wordsplit/wordsplit.rs`,
- manual free paths are replaced with safe reset/ownership semantics,
- the Rust module builds and passes `cargo test`,
- no additional subsystem restructuring or unevidenced helper layers have been introduced.