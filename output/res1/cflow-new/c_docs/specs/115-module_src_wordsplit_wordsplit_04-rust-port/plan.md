# Implementation Plan: module_src_wordsplit_wordsplit_04

## Summary

This module ports the word-splitting logic in `src/wordsplit/wordsplit.c` to Rust, preserving the existing parsing flow and lifecycle semantics rather than redesigning behavior. The implementation should keep the current responsibilities together in a single Rust module that mirrors the C file and migrates the existing function set in dependency order.

The Rust approach should:
- translate the scanner-style parsing routines into slice/index-based Rust code,
- replace manual allocation and free paths with owned Rust containers,
- preserve externally visible behavior for word expansion, delimiter skipping, quoted-string scanning, processing lists, and result cleanup,
- model fallible operations with `Result` where C currently relies on status codes or null-sensitive control flow.

The plan should avoid adding new capabilities or splitting the logic into extra architectural layers beyond what is needed to represent the original module safely in Rust.

## Technical Context

- **Language/Version:** Rust 1.76+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Keep parsing complexity linear with respect to input length for the main scan path.
  - Avoid unnecessary intermediate string allocations during delimiter scanning and word detection.
  - Use `String`, `Vec<String>`, and borrowed `&str` slices where possible to match C performance expectations without unsafe memory handling.
  - Preserve predictable cleanup behavior by relying on ownership rather than emulating manual free operations.

## Module Mapping

### Source Mapping

- **C source:** `src/wordsplit/wordsplit.c`
- **Rust target:** `src/wordsplit/wordsplit.rs`

If the crate already exposes a `wordsplit` module tree, this file should be added under the existing structure and re-exported only as needed by current callers. The migration should not introduce additional helper modules unless required by existing crate layout.

### Function Mapping

Each C function should map to a Rust function in the same module, retaining naming correspondence where practical:

| C Function | Rust Mapping | Notes |
|---|---|---|
| `wordsplit_tildexpand` | `fn wordsplit_tildexpand(...) -> Result<..., ...>` | Preserve current expansion behavior; use `String`/`Path`-compatible logic only if directly needed by existing code paths. |
| `wordsplit_pathexpand` | `fn wordsplit_pathexpand(...) -> Result<..., ...>` | Keep current pathname expansion flow; do not broaden shell semantics. |
| `skip_delim_internal` | `fn skip_delim_internal(...) -> ...` | Internal scanner helper operating on indices/slices. |
| `skip_delim` | `fn skip_delim(...) -> ...` | Thin wrapper over internal delimiter scanning. |
| `skip_delim_real` | `fn skip_delim_real(...) -> ...` | Preserve current distinction from wrapper/helper behavior. |
| `scan_qstring` | `fn scan_qstring(...) -> Result<..., ...>` | Implement quoted-string scanning with explicit bounds checks. |
| `scan_word` | `fn scan_word(...) -> Result<..., ...>` | Core token scanner; keep state transitions close to C ordering. |
| `wordsplit_string_unquote_copy` | `fn wordsplit_string_unquote_copy(...) -> String` or `Result<String, ...>` | Remove escape/quote markers according to existing rules. |
| `exptab_matches` | `fn exptab_matches(...) -> bool` | Straightforward matching helper. |
| `wordsplit_process_list` | `fn wordsplit_process_list(...) -> Result<..., ...>` | Preserve list-processing sequence and mutation order. |
| `wordsplit_run` | `fn wordsplit_run(...) -> Result<..., ...>` | Main execution driver. |
| `wordsplit_len` | `fn wordsplit_len(...) -> usize` | Return computed result count/length per existing behavior. |
| `wordsplit` | `pub fn wordsplit(...) -> Result<..., ...>` | Public entrypoint matching current module contract. |
| `wordsplit_free_words` | `fn wordsplit_free_words(...)` or no-op compatibility helper | In Rust, usually reduced to clearing/dropping owned vectors. |
| `wordsplit_free_envbuf` | `fn wordsplit_free_envbuf(...)` or no-op compatibility helper | Retain only if needed for call-site parity during migration. |

## Data Model

The C analysis only exposes anonymous structures, so the Rust data model should be reconstructed from actual field usage in `wordsplit.c` and kept minimal. The migration should introduce named Rust types only for structures that correspond to persistent state in the C file.

### Planned Type Mapping

| C Representation | Rust Representation | Migration Notes |
|---|---|---|
| Anonymous parser/state struct | `struct WordSplitState` | Central mutable state that carries flags, input, current position, outputs, and temporary buffers. |
| Anonymous token/word entry | `struct WordEntry` | Use only if C stores per-word metadata beyond the final string. Otherwise collapse into `Vec<String>`. |
| Anonymous expansion table entry | `struct ExpansionEntry` or enum-backed lookup record | Needed only if `exptab_matches` and related code operate on table data. |
| Anonymous delimiter/scan context | inline fields in `WordSplitState` or a small `ScanContext` struct | Keep separate only if the C code clearly passes this context around repeatedly. |
| C strings (`char *`) | `String` / `&str` / `Vec<u8>` | Prefer `String`/`&str` for text paths; use `Vec<u8>` only if byte-preserving behavior is required by the C logic. |
| C dynamic arrays | `Vec<T>` | Replace manual realloc/free patterns directly. |
| C flags/bitmasks | `u32`/`u64` newtype or plain integer flags | Keep representation close to existing call patterns; avoid speculative enum-flags crate use. |
| C status/error codes | `Result<T, WordSplitError>` or `Result<T, i32>` during initial port | Start with a narrow custom error enum only for states actually needed by callers. |
| Null optional pointers | `Option<T>` / `Option<String>` / `Option<usize>` | Use explicit optionality instead of sentinel values. |

### Ownership and Memory Management

- Input text should be borrowed as `&str` where the original C code reads from caller-owned memory.
- Produced words should be owned `String` values in a `Vec<String>`.
- Temporary buffers used during scanning or expansion should be local `String`/`Vec<u8>` values owned by the state object or the current function.
- Functions corresponding to `wordsplit_free_words` and `wordsplit_free_envbuf` should be reduced to container clearing or dropped entirely if the public Rust API makes cleanup automatic. If legacy call shape must be preserved internally, they should be thin compatibility helpers.

### Error Handling

- Convert parse and expansion failures from integer return conventions into `Result`.
- Keep error categories narrow and based only on observed C branches, such as invalid quoting state, expansion failure, or malformed scan position.
- Avoid introducing generalized recovery behavior; errors should terminate processing in the same stage where the C code would fail.

## Implementation Phases

### Phase 1: Reconstruct State and Port Low-Level Scanning

- Inspect `src/wordsplit/wordsplit.c` and identify the actual anonymous structs and shared mutable fields used across the function set.
- Define the minimal Rust state structs required to represent:
  - input buffer,
  - scan cursor/index,
  - output words,
  - temporary expansion buffers,
  - flags and mode bits.
- Port the lowest-level scan helpers first:
  - `skip_delim_internal`
  - `skip_delim`
  - `skip_delim_real`
  - `scan_qstring`
  - `scan_word`
  - `wordsplit_string_unquote_copy`
  - `exptab_matches`
- Keep these functions in one Rust source file and mirror the original call graph rather than refactoring behavior.
- Add unit tests for scanner edge cases derived from current C behavior:
  - empty input,
  - delimiter-only input,
  - quoted substrings,
  - escaped characters if present in the C logic,
  - end-of-input bounds handling.

### Phase 2: Port Expansion and Processing Flow

- Port the expansion-related functions with the same sequencing used in C:
  - `wordsplit_tildexpand`
  - `wordsplit_pathexpand`
  - `wordsplit_process_list`
- Recreate any expansion-table or list state with standard-library collections only.
- Ensure expansion functions operate on the same intermediate word representation established in Phase 1.
- Preserve ordering and mutation semantics so that processing results match the original C flow.
- Add tests covering:
  - words that do not expand,
  - words that trigger tilde/path expansion,
  - processing of multiple scanned words through the list pipeline,
  - failure propagation from expansion into the processing stage.

### Phase 3: Port Public Execution Path and Cleanup Semantics

- Port the high-level orchestration functions:
  - `wordsplit_run`
  - `wordsplit_len`
  - `wordsplit`
- Replace manual lifecycle handling with ownership-driven return values while preserving any required public shape for current callers.
- Implement compatibility cleanup helpers for:
  - `wordsplit_free_words`
  - `wordsplit_free_envbuf`
  only if existing Rust-side call patterns still require explicit reset operations.
- Verify that result length/count reporting matches C behavior exactly.
- Add integration-style tests around the public entrypoint for representative complete inputs and output counts.

### Phase 4: Behavior Verification and Migration Cleanup

- Compare the Rust output against the C module behavior for the full function set and remove any temporary compatibility scaffolding that is no longer needed internally.
- Confirm there are no remaining manual-memory assumptions translated from C, such as stale index use after vector mutation or borrowed references outliving local buffers.
- Tighten signatures from transitional forms to final forms:
  - replace temporary integer status returns with `Result` where not already done,
  - remove no-op cleanup code if ownership fully subsumes it and no callers depend on those functions.
- Finalize `cargo test` coverage for:
  - scanning,
  - quoting,
  - expansion paths,
  - end-to-end word splitting,
  - cleanup/reset behavior if retained.