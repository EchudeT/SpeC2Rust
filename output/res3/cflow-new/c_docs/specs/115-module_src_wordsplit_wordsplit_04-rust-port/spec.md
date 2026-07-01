# spec.md

## Title

Rust Port Functional Specification: `module_src_wordsplit_wordsplit_04`

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_04`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Target Rust branch: `115-module_src_wordsplit_wordsplit_04-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is the command-string splitting stage responsible for scanning input text into words, handling quoting-aware token boundaries, and applying post-scan expansion passes that are evidenced in this slice of the source: tilde expansion and pathname expansion. It also includes the run entrypoints for processing a full input string, plus cleanup helpers for word storage and environment-related temporary buffers.

The Rust rewrite must preserve the observable behavior of this module within the wordsplitting pipeline:

- accept an input command string and length-aware variant,
- identify word boundaries while honoring delimiter and quote rules,
- unquote copied string content where required,
- process expansion lists through the module’s expansion dispatch logic,
- perform tilde and pathname expansion where enabled by the surrounding `wordsplit` state and flags,
- expose result length / result words through the `wordsplit` state object,
- free words and environment buffers through explicit cleanup operations.

## Scope

### In Scope

The Rust version must implement the behavior evidenced by the following functions in this module slice:

- input execution entrypoints: `wordsplit_run`, `wordsplit_len`, `wordsplit`
- delimiter scanning helpers: `skip_delim_internal`, `skip_delim`, `skip_delim_real`
- quoted and general word scanning: `scan_qstring`, `scan_word`
- string unquoting copy: `wordsplit_string_unquote_copy`
- expansion dispatch support: `exptab_matches`, `wordsplit_process_list`
- tilde expansion: `wordsplit_tildexpand`
- pathname expansion: `wordsplit_pathexpand`
- cleanup helpers: `wordsplit_free_words`, `wordsplit_free_envbuf`

### Out of Scope

The Rust specification does not require any capabilities not evidenced by this module slice, including:

- new public APIs beyond the C-visible functional surface represented here,
- concurrency guarantees,
- serialization or persistence,
- FFI design commitments,
- performance targets beyond functional equivalence,
- expansion categories not evidenced in this module slice.

## Feature Specification

### 1. Command string processing entrypoints

The module provides a primary wordsplitting operation for a NUL-terminated command string and a length-bounded variant. Both execute the module’s scan-and-process pipeline against a mutable `wordsplit` state object.

The Rust version must:

- support processing an entire command string via a convenience entrypoint equivalent to `wordsplit`,
- support processing a string with explicit byte length via an entrypoint equivalent to `wordsplit_len`,
- route both through a shared internal run routine equivalent to `wordsplit_run`,
- update the provided `wordsplit` state with produced words and related temporary processing state,
- return status codes consistent with success/failure signaling used by the module.

### 2. Delimiter-aware scanning

The module contains helper behavior for skipping delimiters, with two visible modes evidenced by `skip_delim` and `skip_delim_real`, both backed by `skip_delim_internal`.

The Rust version must preserve the distinction between:

- skipping delimiters for normal word boundary processing, and
- skipping delimiters under the alternate “real delimiter” mode represented by the helper split.

The exact delimiter set is controlled by the `wordsplit` state and broader parser configuration; the Rust port must preserve behavior as defined by that state rather than hard-code unrelated semantics.

### 3. Quote-aware word recognition

The module recognizes quoted strings and broader words using `scan_qstring` and `scan_word`.

The Rust version must:

- detect and scan quoted substrings beginning at a given start position,
- determine the end of a quoted segment or report failure when scanning cannot complete successfully,
- scan words from a start position while respecting delimiter rules and quote handling,
- support the `consume_all` mode evidenced by `scan_word`,
- integrate quoted segments into word recognition rather than splitting on delimiters inside active quoted ranges.

### 4. Unquoting during string copy

The module exposes `wordsplit_string_unquote_copy`, which copies a span of string data while removing quote syntax according to parser state.

The Rust version must:

- copy source text into destination/result storage while applying the module’s quote-removal behavior,
- honor whether the source is considered to be in a quoted context (`inquote` input),
- preserve non-quote character content,
- avoid retaining quote markers that the C module treats as syntax rather than payload.

### 5. Expansion list processing

The module includes expansion-table matching (`exptab_matches`) and list processing (`wordsplit_process_list`). This indicates an internal phase that walks pending word nodes and applies enabled expansion handlers.

The Rust version must:

- evaluate whether a configured expansion entry applies to the current `wordsplit` state,
- iterate pending words or nodes from a given starting position,
- apply the module’s enabled expansion stages in the same pipeline role as the C implementation,
- preserve ordered processing so later module output reflects expansion results generated by earlier steps.

### 6. Tilde expansion

The module performs tilde expansion through `wordsplit_tildexpand`.

The Rust version must:

- recognize word content that is eligible for tilde expansion,
- expand leading tilde forms using the same `wordsplit` configuration rules represented by the source,
- support home-directory style substitution behavior evidenced by the source’s use of `passwd`,
- leave non-eligible words unchanged,
- return a status indicating success or failure of the expansion pass.

This requirement is limited to the tilde expansion behavior present in this module and does not imply unrelated shell compatibility beyond what the original code performs.

### 7. Pathname expansion

The module performs pathname expansion through `wordsplit_pathexpand`.

The Rust version must:

- recognize words eligible for pathname pattern expansion,
- expand matching path patterns into one or more result words when enabled,
- preserve words unchanged when they are not eligible or do not match expansion conditions as defined by the source behavior,
- integrate pathname expansion into the module’s word list processing flow,
- return a status indicating success or failure of the expansion pass.

### 8. Explicit cleanup of produced words

The module provides `wordsplit_free_words` for releasing word result storage associated with a `wordsplit` state.

The Rust version must provide equivalent cleanup behavior at the module boundary level, even if implemented with Rust ownership internally, so that the externally observable lifecycle of words associated with the state remains correct.

This includes:

- releasing all stored output words associated with the current split result,
- resetting the associated word-storage fields in the state so they are no longer treated as populated.

### 9. Explicit cleanup of environment buffers

The module provides `wordsplit_free_envbuf` for releasing environment-related temporary buffers associated with a `wordsplit` state.

The Rust version must provide equivalent cleanup behavior for that state-managed temporary storage, including:

- releasing all environment buffer entries tracked by the state,
- resetting the corresponding state so subsequent processing does not observe stale buffer contents.

## User Scenarios & Testing

### Scenario 1: Split a command string into words

A caller provides a command string and an initialized `wordsplit` state, then invokes the main wordsplitting entrypoint.

Expected support:

- the module scans the string,
- skips delimiters at word boundaries,
- identifies one or more words,
- stores results in the `wordsplit` state,
- returns a success status when parsing completes normally.

### Scenario 2: Process only a fixed-length prefix

A caller needs to split a string buffer using an explicit length rather than relying on full-string termination.

Expected support:

- only the supplied length is processed,
- scanning and splitting operate on that bounded range,
- produced words reflect only the specified prefix.

### Scenario 3: Preserve delimiter behavior outside quotes

A caller provides input containing delimiters between words.

Expected support:

- delimiters separate words,
- leading delimiter runs are skipped,
- repeated delimiters do not produce unintended payload characters in output words.

### Scenario 4: Preserve literal content inside quotes

A caller provides input where delimiter characters appear inside quoted text.

Expected support:

- delimiters inside a quoted segment do not terminate the current word,
- the quoted segment is scanned as part of one logical word,
- output content is unquoted according to module rules.

Example class of input:

- a word containing spaces within quotes should remain one result word after splitting.

### Scenario 5: Detect malformed quoted input

A caller provides input with an unterminated quoted segment.

Expected support:

- quoted-string scanning reports failure,
- overall processing returns an error status rather than silently accepting malformed input.

### Scenario 6: Apply tilde expansion to eligible words

A caller enables or reaches the expansion phase with words beginning with tilde syntax.

Expected support:

- eligible leading-tilde words are expanded,
- non-eligible words are left unchanged,
- expansion output is reflected in the final word list.

### Scenario 7: Apply pathname expansion to eligible words

A caller provides words containing pathname pattern syntax that is eligible for expansion.

Expected support:

- matching filesystem pathnames are expanded into result words according to module behavior,
- words not eligible for pathname expansion remain unchanged,
- the resulting list is available through the `wordsplit` state.

### Scenario 8: Run expansion processing over pending words

A caller reaches the post-scan processing stage with a list of parsed word nodes.

Expected support:

- the module evaluates enabled expansion handlers,
- applies matching handlers in the processing sequence,
- updates later-visible results based on expansion output.

### Scenario 9: Reuse the state after cleanup

A caller finishes using a split result and invokes cleanup helpers.

Expected support:

- word storage cleanup removes prior result words,
- environment-buffer cleanup removes prior temporary buffers,
- the state no longer exposes stale prior output through those storage areas.

## Requirements

### Functional Requirements

#### FR-1: Entry-point processing
The module shall provide wordsplitting operations equivalent to `wordsplit` and `wordsplit_len`, both executed through shared run behavior equivalent to `wordsplit_run`.
Traceability: `wordsplit`, `wordsplit_len`, `wordsplit_run`.

#### FR-2: Length-bounded processing
The length-aware operation shall process only the provided command prefix length.
Traceability: `wordsplit_len`, `wordsplit_run`.

#### FR-3: Delimiter skipping
The module shall skip delimiter runs before scanning the next word, with behavior variants corresponding to `skip_delim` and `skip_delim_real`.
Traceability: `skip_delim_internal`, `skip_delim`, `skip_delim_real`, `scan_word`.

#### FR-4: Quoted-string scanning
The module shall scan quoted substrings and determine their valid end position or signal failure if the quoted content is malformed.
Traceability: `scan_qstring`.

#### FR-5: Word scanning
The module shall scan a word from a start position while respecting delimiters, quote handling, and the `consume_all` control mode.
Traceability: `scan_word`.

#### FR-6: Quote-removing copy
The module shall copy text into result storage while removing quote syntax according to the parser’s quote rules and the `inquote` context indicator.
Traceability: `wordsplit_string_unquote_copy`.

#### FR-7: Expansion applicability checks
The module shall determine whether a configured expansion entry applies to the current processing state before invoking that expansion stage.
Traceability: `exptab_matches`, `wordsplit_process_list`, `exptab`.

#### FR-8: Expansion list processing
The module shall process pending word/node entries from a specified position through the module’s enabled expansion stages.
Traceability: `wordsplit_process_list`, `wordsplit_node`, `wordsplit`.

#### FR-9: Tilde expansion
The module shall perform tilde expansion on eligible words during expansion processing and preserve non-eligible words unchanged.
Traceability: `wordsplit_tildexpand`, `passwd`, `wordsplit_process_list`.

#### FR-10: Pathname expansion
The module shall perform pathname expansion on eligible words during expansion processing and preserve non-eligible words unchanged when expansion does not apply.
Traceability: `wordsplit_pathexpand`, `wordsplit_process_list`.

#### FR-11: Result-state population
Successful processing shall populate the `wordsplit` state with the resulting words and associated count/collection state used by the module.
Traceability: `wordsplit_run`, `wordsplit_len`, `wordsplit`, `wordsplit`.

#### FR-12: Word cleanup
The module shall release and clear word-result storage associated with a `wordsplit` state when cleanup is requested.
Traceability: `wordsplit_free_words`, `wordsplit`.

#### FR-13: Environment buffer cleanup
The module shall release and clear environment-related temporary buffer storage associated with a `wordsplit` state when cleanup is requested.
Traceability: `wordsplit_free_envbuf`, `wordsplit`.

### Key Entities

#### `wordsplit`
Primary mutable processing state for the module.

Role evidenced by the source slice:

- receives input and flags for run execution,
- controls delimiter and quote-sensitive scanning behavior,
- stores produced words,
- stores temporary state used by expansion processing,
- owns cleanup-managed word storage and environment-buffer storage.

Relationships:

- consumed and mutated by all major entrypoints and scan/expand functions,
- associated with zero or more `wordsplit_node` items during internal processing,
- evaluated against `exptab` entries during expansion dispatch.

Traceability: `wordsplit_run`, `scan_qstring`, `scan_word`, `wordsplit_tildexpand`, `wordsplit_pathexpand`, `wordsplit_process_list`, `wordsplit_free_words`, `wordsplit_free_envbuf`.

#### `wordsplit_node`
Internal node/list element representing a parsed or intermediate word item during processing.

Role evidenced by the source slice:

- participates in list-based processing,
- serves as the unit operated on by expansion phases.

Relationships:

- belongs to processing structures owned by `wordsplit`,
- traversed or transformed during `wordsplit_process_list`.

Traceability: `wordsplit_node`, `wordsplit_process_list`.

#### `exptab`
Expansion-table entry type used to decide whether a particular expansion stage applies.

Role evidenced by the source slice:

- matched against current `wordsplit` state before expansion processing.

Relationships:

- consulted by `exptab_matches`,
- used by `wordsplit_process_list` to dispatch enabled expansion behavior.

Traceability: `exptab_matches`, `wordsplit_process_list`, `exptab`.

#### `passwd`
External account-information type referenced by tilde expansion behavior.

Role evidenced by the source slice:

- supports resolving home-directory style substitutions used by tilde expansion.

Relationships:

- used only as part of `wordsplit_tildexpand` behavior.

Traceability: `wordsplit_tildexpand`, `passwd`.

## Success Criteria

### Functional Equivalence Criteria

1. Invoking the Rust equivalent of `wordsplit` on valid input shall return success and populate the output words in the provided processing state.
   Traceability: `wordsplit`, `wordsplit_run`.

2. Invoking the Rust equivalent of `wordsplit_len` with a bounded length shall ignore content beyond that length when forming words.
   Traceability: `wordsplit_len`, `wordsplit_run`.

3. Inputs containing delimiter runs between unquoted words shall be split into separate words, with delimiters not retained as word payload unless the original module’s delimiter-returning mode requires otherwise internally.
   Traceability: `skip_delim_internal`, `skip_delim`, `skip_delim_real`, `scan_word`.

4. Inputs containing delimiters inside quoted segments shall produce a single logical word segment spanning that quoted content.
   Traceability: `scan_qstring`, `scan_word`.

5. Inputs with malformed or unterminated quoted content shall produce an error result rather than a normal successful split.
   Traceability: `scan_qstring`, `scan_word`, `wordsplit_run`.

6. Quote-removing copy behavior shall omit quote syntax from copied output while preserving non-quote character content.
   Traceability: `wordsplit_string_unquote_copy`.

7. Eligible leading-tilde words shall be transformed by the tilde expansion stage, while non-eligible words remain unchanged by that stage.
   Traceability: `wordsplit_tildexpand`.

8. Eligible pathname-pattern words shall be processed by the pathname expansion stage and reflected in the final result list according to module behavior.
   Traceability: `wordsplit_pathexpand`.

9. Expansion processing shall apply only expansion handlers whose applicability matches the current processing state.
   Traceability: `exptab_matches`, `wordsplit_process_list`.

10. Calling the Rust equivalents of `wordsplit_free_words` and `wordsplit_free_envbuf` shall leave their corresponding state-managed storage cleared and no longer observable as populated.
    Traceability: `wordsplit_free_words`, `wordsplit_free_envbuf`.

### Testability Expectations

The Rust port shall be considered complete for this module when automated tests cover at least:

- successful splitting of multiple unquoted words,
- bounded-length splitting,
- quoted-word scanning with embedded delimiters,
- failure on unterminated quotes,
- unquote-copy behavior,
- tilde expansion on eligible input,
- pathname expansion on eligible input,
- cleanup of words,
- cleanup of environment buffers.