# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_04`

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_04`
- Category: `module_cluster`
- Source basis: `src/wordsplit/wordsplit.c`
- Target branch: `115-module_src_wordsplit_wordsplit_04-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is the late-stage word splitting and expansion portion of the `wordsplit` subsystem. It is responsible for turning an input command string into processed words by scanning delimiters and quoted regions, processing pending expansion nodes, applying tilde expansion and pathname expansion, and exposing entry points that run the split process and release allocated result buffers.

The Rust rewrite must preserve the observable behavior evidenced by the module functions in `src/wordsplit/wordsplit.c`, specifically:

- scanning command text into words with delimiter handling,
- handling quoted substrings during scanning,
- copying text while removing quote syntax where required,
- processing an internal expansion list,
- performing tilde expansion,
- performing pathname expansion,
- running the overall split pipeline on a full command or a bounded-length command,
- and freeing word and environment-related result storage owned by the split state.

## Scope

### In Scope

The Rust version must implement the functionality evidenced by these module responsibilities:

- command scanning and word boundary detection,
- delimiter skipping with modes that either treat delimiter bytes as consumed separators or preserve them for caller-visible logic,
- quoted-string scanning,
- per-word scanning with support for a mode that can continue through remaining input,
- unquoted copying of string content into destination buffers,
- processing of deferred expansion work items associated with the split state,
- tilde expansion over split results,
- pathname expansion over split results,
- top-level execution of the word split pipeline,
- convenience entry points for full-string and explicit-length invocation,
- release of words storage and environment buffer storage held by the split state.

### Out of Scope

The Rust version must not introduce capabilities not evidenced here, including:

- new public APIs beyond the behaviors represented by the existing entry points,
- serialization or persistence features,
- concurrency guarantees,
- recovery flows beyond the source module’s success/error behavior,
- FFI-specific behavior,
- or shell features not supported by the analyzed module functions.

## Source Traceability

Primary behavioral evidence comes from the following functions in `src/wordsplit/wordsplit.c`:

- `wordsplit_tildexpand`
- `wordsplit_pathexpand`
- `skip_delim_internal`
- `skip_delim`
- `skip_delim_real`
- `scan_qstring`
- `scan_word`
- `wordsplit_string_unquote_copy`
- `exptab_matches`
- `wordsplit_process_list`
- `wordsplit_run`
- `wordsplit_len`
- `wordsplit`
- `wordsplit_free_words`
- `wordsplit_free_envbuf`

Primary state evidence comes from:

- `struct wordsplit`
- `struct wordsplit_node`
- referenced expansion table type `exptab`

## Feature Specification

### Feature 1: Top-level word splitting execution

The module must accept an input command string and a split-state object, run the module’s word-splitting pipeline, and populate the split state with the resulting processed words and related buffers.

This feature includes two invocation forms:

- processing a NUL-terminated command string,
- processing a command string with an explicit length limit.

The Rust version must preserve the distinction between these entry modes because both are evidenced by `wordsplit` and `wordsplit_len`.

### Feature 2: Delimiter-aware scanning

The module must scan input text while recognizing delimiters according to split-state configuration. It must support:

- skipping delimiters during normal word separation,
- and a variant that returns or preserves delimiter presence for callers that need non-normalized delimiter handling.

The Rust version must preserve the behavioral distinction evidenced by `skip_delim_internal`, `skip_delim`, and `skip_delim_real`.

### Feature 3: Quoted text recognition during scan

The module must detect and traverse quoted substrings while scanning input into words. Quoted regions must be recognized as structured input spans so that word boundaries are computed correctly and later unquoting can remove quote syntax while preserving content.

The Rust version must preserve this behavior as evidenced by `scan_qstring` and `scan_word`.

### Feature 4: Word scanning over a command buffer

The module must scan from a starting position to identify a word or a run of text that belongs to the current processing unit. Scanning must respect delimiters and quoted regions and must support a mode that consumes all remaining relevant input when requested.

The Rust version must preserve the caller-observable scanning outcomes evidenced by `scan_word`.

### Feature 5: Quote removal during string copy

The module must support copying string content from source to destination while removing quoting syntax according to whether the copy occurs in a quoted context. The resulting copied text must be suitable for later expansion or final word output depending on pipeline stage.

The Rust version must preserve the behavior of `wordsplit_string_unquote_copy`.

### Feature 6: Expansion-list processing

The module must process an internal list of pending expansion-related items stored in the split state. Each list item is processed only when its matching conditions apply to the current split state.

The Rust version must preserve:

- matching of list items against split-state conditions, and
- ordered processing of the list from a given start position,

as evidenced by `exptab_matches` and `wordsplit_process_list`.

### Feature 7: Tilde expansion

The module must perform tilde expansion on split results where applicable. This includes handling expansion cases associated with user-home style input.

The Rust version must preserve the observable effect that applicable tilde-prefixed word content is expanded during the pipeline, as evidenced by `wordsplit_tildexpand` and the source reference to `passwd`.

### Feature 8: Pathname expansion

The module must perform pathname expansion on split results where applicable. Words requiring path pattern expansion must be replaced or rewritten according to the module’s source behavior.

The Rust version must preserve the expansion stage represented by `wordsplit_pathexpand`.

### Feature 9: Owned-result cleanup

The module must provide cleanup operations that release:

- the stored split words, and
- stored environment-related buffer data

owned by the split-state object.

The Rust version must preserve the behavior and separation of these cleanup responsibilities as evidenced by `wordsplit_free_words` and `wordsplit_free_envbuf`.

## User Scenarios & Testing

### Scenario 1: Split a complete command string into processed words

A caller provides a command string and a split-state object, invokes the full-string entry point, and receives processed words in the split state.

The Rust version must support testing that:

- input is accepted through the full-string entry,
- the split pipeline runs to completion,
- and resulting words reflect delimiter handling, quote handling, and enabled expansion stages.

**Traceability:** `wordsplit`, `wordsplit_run`

### Scenario 2: Split only a bounded prefix of a command buffer

A caller has a command buffer that may contain additional trailing bytes outside the intended parse region and invokes the length-bounded entry point.

The Rust version must support testing that:

- only the provided length is processed,
- scanning does not consume bytes beyond that boundary,
- and produced words correspond only to the bounded region.

**Traceability:** `wordsplit_len`, `wordsplit_run`

### Scenario 3: Parse words containing quoted substrings

A caller provides input containing single-quoted or double-quoted regions embedded in words or separated by delimiters.

The Rust version must support testing that:

- quoted regions are scanned as part of the correct word,
- delimiters inside recognized quoted regions do not prematurely terminate the word,
- and later unquote-copy behavior removes quote syntax while preserving quoted content.

**Traceability:** `scan_qstring`, `scan_word`, `wordsplit_string_unquote_copy`

### Scenario 4: Handle leading or repeated delimiters

A caller provides input with leading separators or multiple adjacent separators between words.

The Rust version must support testing that:

- delimiter skipping advances over separator runs correctly,
- normal word scanning begins at the correct next word position,
- and delimiter-preserving behavior remains distinguishable where required by caller logic.

**Traceability:** `skip_delim_internal`, `skip_delim`, `skip_delim_real`

### Scenario 5: Expand tilde-prefixed words

A caller provides words beginning with tilde syntax in a context where tilde expansion applies.

The Rust version must support testing that:

- applicable tilde words are expanded during processing,
- non-applicable words remain unchanged,
- and expansion integrates correctly with split results.

**Traceability:** `wordsplit_tildexpand`

### Scenario 6: Expand pathname patterns

A caller provides words containing pathname pattern syntax in a context where pathname expansion applies.

The Rust version must support testing that:

- applicable words undergo pathname expansion,
- non-matching or non-applicable words preserve source behavior,
- and resulting words are reflected in the split state after the expansion stage.

**Traceability:** `wordsplit_pathexpand`

### Scenario 7: Process deferred expansion items in order

A caller reaches a pipeline stage where the split state contains pending expansion work items.

The Rust version must support testing that:

- work items are checked against split-state matching conditions,
- matching items are processed from the requested start position,
- and non-matching items are skipped without being applied.

**Traceability:** `exptab_matches`, `wordsplit_process_list`

### Scenario 8: Release words and environment buffers after use

After a successful or failed split attempt, a caller invokes cleanup on the split state.

The Rust version must support testing that:

- word storage is releasable independently,
- environment buffer storage is releasable independently,
- and cleanup leaves the state without dangling owned result buffers.

**Traceability:** `wordsplit_free_words`, `wordsplit_free_envbuf`

## Requirements

### Functional Requirements

#### FR-1: Full-string split entry
The module shall provide functionality equivalent to `wordsplit` that processes an entire command string using a caller-supplied split state and flags.

**Traceability:** `wordsplit`

#### FR-2: Explicit-length split entry
The module shall provide functionality equivalent to `wordsplit_len` that processes exactly the supplied command length using a caller-supplied split state and flags.

**Traceability:** `wordsplit_len`

#### FR-3: Shared pipeline execution
The module shall execute a common word-splitting pipeline over command input, with behavior equivalent to `wordsplit_run`, including use of input text, input length, split state, flags, and internal processing level.

**Traceability:** `wordsplit_run`

#### FR-4: Delimiter skipping
The module shall support delimiter skipping behavior equivalent to `skip_delim_internal` and its two caller-facing variants, including distinct handling for normal delimiter skipping and delimiter-aware real-position handling.

**Traceability:** `skip_delim_internal`, `skip_delim`, `skip_delim_real`

#### FR-5: Quoted-region scanning
The module shall identify and scan quoted string regions from a given start position and report scan completion or error in a manner equivalent to `scan_qstring`.

**Traceability:** `scan_qstring`

#### FR-6: Word scanning
The module shall scan a word from a given start position while respecting delimiters and quoted regions, with support for a mode equivalent to `consume_all`.

**Traceability:** `scan_word`

#### FR-7: Quote-removing copy
The module shall copy source text into a destination buffer while removing quote syntax according to quoting context, with behavior equivalent to `wordsplit_string_unquote_copy`.

**Traceability:** `wordsplit_string_unquote_copy`

#### FR-8: Expansion-table matching
The module shall evaluate whether an expansion-table entry applies to the current split state, with behavior equivalent to `exptab_matches`.

**Traceability:** `exptab_matches`, `exptab`, `struct wordsplit`

#### FR-9: Expansion-list processing
The module shall process pending expansion-related items associated with the split state from a specified list position, applying only matching items, with behavior equivalent to `wordsplit_process_list`.

**Traceability:** `wordsplit_process_list`, `struct wordsplit`, `struct wordsplit_node`

#### FR-10: Tilde expansion stage
The module shall perform tilde expansion over eligible split content with behavior equivalent to `wordsplit_tildexpand`.

**Traceability:** `wordsplit_tildexpand`

#### FR-11: Pathname expansion stage
The module shall perform pathname expansion over eligible split content with behavior equivalent to `wordsplit_pathexpand`.

**Traceability:** `wordsplit_pathexpand`

#### FR-12: Word-result cleanup
The module shall release word storage owned by the split state with behavior equivalent to `wordsplit_free_words`.

**Traceability:** `wordsplit_free_words`, `struct wordsplit`

#### FR-13: Environment-buffer cleanup
The module shall release environment-related buffer storage owned by the split state with behavior equivalent to `wordsplit_free_envbuf`.

**Traceability:** `wordsplit_free_envbuf`, `struct wordsplit`

### Key Entities

#### `Wordsplit` state
A state object corresponding to `struct wordsplit`. It carries the command-processing context, flags, intermediate scan state, expansion-processing state, and owned output buffers. It is the central object passed to all major operations in this module.

**Relationships:**
- consumed by top-level split entry points,
- read and updated during scanning,
- used to determine expansion applicability,
- owns words storage and environment buffer storage,
- and associates with expansion list items and nodes.

**Traceability:** `struct wordsplit`, all listed functions

#### `WordsplitNode`
A node object corresponding to `struct wordsplit_node`. It represents internal items used during staged processing of words or expansions.

**Relationships:**
- belongs to or is referenced by the `Wordsplit` state,
- participates in ordered list processing,
- and is acted upon during expansion-list processing.

**Traceability:** `struct wordsplit_node`, `wordsplit_process_list`

#### `Expansion table entry`
An object corresponding to the referenced `exptab` type. It describes a conditional expansion-processing rule or entry whose applicability is checked against the current split state.

**Relationships:**
- evaluated against `Wordsplit`,
- and used by expansion-list processing to decide whether work is applied.

**Traceability:** `exptab_matches`, `exptab`

#### Word results
A collection of output words owned by the split state after pipeline execution.

**Relationships:**
- produced by scanning and expansion stages,
- rewritten by tilde or pathname expansion where applicable,
- and freed by word-result cleanup.

**Traceability:** `wordsplit_run`, `wordsplit_tildexpand`, `wordsplit_pathexpand`, `wordsplit_free_words`

#### Environment buffer storage
An owned buffer or buffer collection attached to the split state for environment-related processing results.

**Relationships:**
- associated with the split state during processing,
- and freed independently of word results.

**Traceability:** `wordsplit_free_envbuf`, `struct wordsplit`

## Success Criteria

### SC-1: Entry-point equivalence
For representative valid inputs, the Rust implementation accepts both full-string and explicit-length invocation and completes processing with outcomes equivalent to the C module for the same input, state, and flags.

**Traceability:** `wordsplit`, `wordsplit_len`, `wordsplit_run`

### SC-2: Length-bound compliance
For bounded-input tests, no word content derived by the Rust implementation depends on bytes beyond the provided input length.

**Traceability:** `wordsplit_len`, `wordsplit_run`

### SC-3: Delimiter behavior preservation
For tests with leading, trailing, and repeated delimiters, the Rust implementation preserves the delimiter-skipping behavior distinctions evidenced by the normal and real delimiter-skip functions.

**Traceability:** `skip_delim_internal`, `skip_delim`, `skip_delim_real`

### SC-4: Quote-aware scanning correctness
For inputs containing quoted substrings, the Rust implementation scans words without splitting on delimiters contained inside recognized quoted regions and reports scan success or failure consistently with source behavior.

**Traceability:** `scan_qstring`, `scan_word`

### SC-5: Unquote copy correctness
For inputs requiring quote removal, the Rust implementation produces destination text equivalent to the C module’s quote-removing copy behavior for the same source text, length, and quoting context.

**Traceability:** `wordsplit_string_unquote_copy`

### SC-6: Expansion-list correctness
For states containing multiple pending expansion items, the Rust implementation processes matching items in order from the requested start position and does not apply non-matching items.

**Traceability:** `exptab_matches`, `wordsplit_process_list`

### SC-7: Tilde expansion preservation
For applicable tilde-prefixed inputs, the Rust implementation performs tilde expansion with results equivalent to the C module; for non-applicable inputs, it preserves non-expansion behavior.

**Traceability:** `wordsplit_tildexpand`

### SC-8: Pathname expansion preservation
For applicable pathname-pattern inputs, the Rust implementation performs pathname expansion with results equivalent to the C module’s stage behavior.

**Traceability:** `wordsplit_pathexpand`

### SC-9: Cleanup correctness
After processing, invoking word cleanup releases owned word-result storage, and invoking environment-buffer cleanup releases owned environment buffer storage, with both operations remaining independently callable on the split state.

**Traceability:** `wordsplit_free_words`, `wordsplit_free_envbuf`

### SC-10: No unsupported feature growth
The Rust rewrite exposes no additional functional surface beyond the behavior evidenced by this module’s source functions and state.

**Traceability:** all listed functions and types

## Acceptance Notes

Conformance should be established by behavior-comparison tests against the C module for representative inputs covering:

- plain words,
- repeated delimiters,
- quoted words,
- bounded-length parsing,
- tilde-expansion cases,
- pathname-expansion cases,
- mixed expansion-list processing,
- and post-run cleanup behavior.