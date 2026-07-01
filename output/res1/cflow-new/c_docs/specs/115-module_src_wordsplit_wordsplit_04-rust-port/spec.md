# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_04`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_src_wordsplit_wordsplit_04`
- **Category**: `module_cluster`
- **Source file basis**: `src/wordsplit/wordsplit.c`
- **Rust branch target**: `115-module_src_wordsplit_wordsplit_04-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module is the late-stage word splitting and expansion portion of the `wordsplit` subsystem. Based on the analyzed functions in `src/wordsplit/wordsplit.c`, it is responsible for:

- scanning command text into words while honoring delimiters and quoting,
- unquoting copied string content,
- applying tilde expansion,
- applying pathname expansion,
- processing pending expansion entries,
- running the end-to-end split operation over a command string,
- reporting split length,
- and releasing word and environment-buffer storage owned by a `wordsplit` state object.

The Rust rewrite must preserve the observable behavior of this module as exercised through the analyzed entry points and helper behavior. The specification is limited to functionality evidenced by the listed functions and referenced data structures.

## Feature Specification

### 1. Command word scanning and token boundary detection

The module shall parse an input command string stored in a `wordsplit` processing context into word units.

Observed behavior boundaries from:
- `skip_delim_internal`
- `skip_delim`
- `skip_delim_real`
- `scan_qstring`
- `scan_word`
- `wordsplit_run`
- `wordsplit_len`
- `wordsplit`

Required behavior:
- Identify and skip delimiters according to the current `wordsplit` configuration.
- Support two delimiter-skipping modes:
  - skipping delimiters as separators,
  - and returning/counting delimiters when requested by internal processing.
- Detect quoted substrings and scan through them as part of a word rather than treating internal delimiters as separators.
- Detect word extents starting from a given input position.
- Support a mode in which scanning consumes the rest of the input as one unit when requested by the caller.
- Produce a final list/count of words through the top-level run functions.

### 2. Quoted string handling and unquoting

The module shall recognize quoted text during scanning and provide string-copy behavior that removes or interprets quoting syntax as required by the existing subsystem contract.

Observed behavior boundaries from:
- `scan_qstring`
- `wordsplit_string_unquote_copy`

Required behavior:
- Recognize quoted string regions beginning at a given offset.
- Determine the matching end position for valid quoted input.
- Report failure for malformed quoted input rather than silently accepting it.
- Copy string content from source to destination while applying the module’s unquoting rules.
- Support unquoting behavior that depends on whether copying occurs in a quoted context.

### 3. Tilde expansion

The module shall expand tilde-prefixed path forms found in words handled by the `wordsplit` state.

Observed behavior boundaries from:
- `wordsplit_tildexpand`
- referenced type `passwd`

Required behavior:
- Detect tilde-based forms eligible for expansion within the current word list/state.
- Expand home-directory references according to the existing C module behavior.
- Support expansion cases that require lookup of user account information, as evidenced by the `passwd` reference.
- Return status indicating success or expansion failure in the same class of control flow used by the source module.

This specification does not require any tilde syntax beyond what is evidenced by the existing function’s purpose and account lookup dependency.

### 4. Pathname expansion

The module shall expand path patterns over the current words in the `wordsplit` state.

Observed behavior boundaries from:
- `wordsplit_pathexpand`

Required behavior:
- Detect words eligible for pathname expansion.
- Replace or augment current word entries with pathname expansion results according to the existing module behavior.
- Preserve non-expandable input as required by the source behavior when no pathname matches are produced, if that is how the existing module behaves.
- Return status suitable for integration into the main processing pipeline.

The Rust rewrite must preserve the source module’s externally observable treatment of words before and after pathname expansion.

### 5. Expansion-table driven processing

The module shall process a list/table of pending expansions against the current `wordsplit` state.

Observed behavior boundaries from:
- `exptab_matches`
- `wordsplit_process_list`
- referenced type `exptab`

Required behavior:
- Evaluate whether a given expansion-table entry applies to the current processing state.
- Traverse/process pending expansion entries starting at a specified position.
- Apply matching expansion steps in the same stage ordering expected by the source module.
- Return status when processing succeeds or when any expansion step fails.

### 6. Top-level run and convenience entry points

The module shall expose the same top-level functional behavior currently provided by the analyzed C entry points.

Observed behavior boundaries from:
- `wordsplit_run`
- `wordsplit_len`
- `wordsplit`

Required behavior:
- Accept a command string and an explicit length for bounded processing.
- Support a convenience entry point that derives length from a NUL-terminated command string.
- Execute the module’s scanning and expansion pipeline against a supplied `wordsplit` state object and flags.
- Populate the `wordsplit` state with the resulting words and related processing state expected by downstream code.
- Return an integer status indicating success or failure.

### 7. Resource release for owned output buffers

The module shall release storage associated with words and environment-related buffers owned by a `wordsplit` state.

Observed behavior boundaries from:
- `wordsplit_free_words`
- `wordsplit_free_envbuf`

Required behavior:
- Free and clear word storage associated with a completed or partially completed split operation.
- Free and clear environment-buffer storage associated with the `wordsplit` state.
- Be safe to call as part of normal cleanup after top-level processing.

## User Scenarios & Testing

### Scenario 1: Split a command string into words

A caller provides a command string and a `wordsplit` state object, then invokes the top-level split entry point.

Expected support:
- The command is scanned into words.
- Delimiters separate words unless inside recognized quoted regions.
- The resulting state contains the final word list or count.
- An integer status reports success or failure.

Test focus:
- empty input,
- single word input,
- multiple words separated by delimiters,
- explicit-length processing through `wordsplit_len`,
- NUL-terminated processing through `wordsplit`.

### Scenario 2: Quoted substrings remain part of the same word

A caller processes input containing quoted text with embedded delimiter characters.

Expected support:
- The scan treats delimiters inside a quoted region as content, not separators.
- A malformed quoted region causes an error status.
- Unquoted output content is copied correctly when the module performs unquoting.

Test focus:
- balanced quoted input,
- adjacent quoted and unquoted text within one word,
- unterminated quote handling,
- unquote copy behavior for quoted and non-quoted modes.

### Scenario 3: Tilde-prefixed words are expanded

A caller processes words that begin with tilde forms eligible for home-directory expansion.

Expected support:
- Eligible words are transformed according to the module’s tilde expansion behavior.
- Expansion failures are reported through the module status path.
- The resulting word list reflects the expanded path text.

Test focus:
- current-user home expansion cases,
- account-based expansion cases evidenced by `passwd` dependency,
- non-eligible words left unchanged.

### Scenario 4: Path patterns are expanded

A caller processes words containing pathname patterns that are eligible for path expansion.

Expected support:
- Matching filesystem pathnames are reflected in the final word set.
- The module integrates pathname expansion into the existing processing pipeline.
- Status handling matches source behavior on success and failure.

Test focus:
- words with patterns that match paths,
- words with no matches,
- multiple expandable words in one command.

### Scenario 5: Mixed processing pipeline

A caller uses the main split operation on input containing delimiters, quotes, tilde forms, and pathname patterns.

Expected support:
- The module performs word scanning first and applies relevant expansion stages in the source-consistent order.
- Final output reflects all applicable transformations.
- Intermediate expansion-list processing is not externally visible except through final results and status.

Test focus:
- one command string combining quoting and expansions,
- ordering-sensitive cases where scanning must happen before expansion,
- failure propagation when one stage fails.

### Scenario 6: Cleanup after processing

A caller cleans up a `wordsplit` state after success or failure.

Expected support:
- Word storage can be released using `wordsplit_free_words`.
- Environment-buffer storage can be released using `wordsplit_free_envbuf`.
- Cleanup leaves no stale owned allocations in the managed state.

Test focus:
- cleanup after successful split,
- cleanup after failed split,
- repeated cleanup calls if source behavior permits idempotent clearing.

## Requirements

### Functional Requirements

#### FR-1: Delimiter skipping
The Rust module shall provide the same delimiter-skipping behavior used by the source module’s scanning logic, including the internal distinction between ordinary delimiter skipping and delimiter-returning behavior.

Traceability:
- `skip_delim_internal`
- `skip_delim`
- `skip_delim_real`

#### FR-2: Quoted string scanning
The Rust module shall detect and scan quoted string regions from a specified input position and determine the end of the quoted region or report an error for malformed input.

Traceability:
- `scan_qstring`

#### FR-3: Word scanning
The Rust module shall scan a word from a specified input position, respecting quoting rules and supporting the source module’s consume-all mode.

Traceability:
- `scan_word`

#### FR-4: Unquoted copy transformation
The Rust module shall copy string content from source to destination while applying the unquoting behavior used by the source module, including behavior dependent on whether the source is considered in-quote.

Traceability:
- `wordsplit_string_unquote_copy`

#### FR-5: Tilde expansion processing
The Rust module shall process tilde expansion over the current `wordsplit` state and update resulting words according to the source module’s behavior.

Traceability:
- `wordsplit_tildexpand`
- referenced type `passwd`

#### FR-6: Pathname expansion processing
The Rust module shall process pathname expansion over the current `wordsplit` state and update resulting words according to the source module’s behavior.

Traceability:
- `wordsplit_pathexpand`

#### FR-7: Expansion entry matching
The Rust module shall determine whether an expansion-table entry applies to the current `wordsplit` state.

Traceability:
- `exptab_matches`
- referenced type `exptab`

#### FR-8: Expansion list processing
The Rust module shall process pending expansion-list entries starting from a supplied position and integrate the results into the active `wordsplit` state.

Traceability:
- `wordsplit_process_list`
- `exptab_matches`

#### FR-9: Bounded top-level processing
The Rust module shall accept a command string plus explicit length and execute the module’s split-and-expand pipeline into a provided `wordsplit` state.

Traceability:
- `wordsplit_run`
- `wordsplit_len`

#### FR-10: NUL-terminated convenience processing
The Rust module shall provide top-level processing for a NUL-terminated command string by deriving length and invoking the same underlying behavior as bounded processing.

Traceability:
- `wordsplit`

#### FR-11: Result and error status propagation
The Rust module shall return status codes from scanning and expansion stages through the top-level processing path so callers can distinguish successful completion from failure.

Traceability:
- `wordsplit_tildexpand`
- `wordsplit_pathexpand`
- `scan_qstring`
- `scan_word`
- `wordsplit_process_list`
- `wordsplit_run`
- `wordsplit_len`
- `wordsplit`

#### FR-12: Word storage cleanup
The Rust module shall release and clear word storage owned by the `wordsplit` state.

Traceability:
- `wordsplit_free_words`

#### FR-13: Environment-buffer cleanup
The Rust module shall release and clear environment-related buffer storage owned by the `wordsplit` state.

Traceability:
- `wordsplit_free_envbuf`

### Key Entities

#### `wordsplit`
Primary processing state for this module.

Role evidenced by:
- all analyzed functions accept or modify `struct wordsplit *`

Required conceptual responsibilities:
- holds the input command context,
- carries delimiter and scanning state used by delimiter-skipping and word-scanning helpers,
- owns or references the current word collection,
- carries flags controlling processing behavior,
- stores intermediate and final data needed by expansion stages,
- owns cleanup-relevant buffers such as word storage and environment-related storage.

Relationships:
- is consumed and mutated by scanning helpers,
- is read and updated by tilde and pathname expansion,
- is used when matching and processing expansion-table entries,
- is the object cleaned by `wordsplit_free_words` and `wordsplit_free_envbuf`.

#### `wordsplit_node`
Internal node structure used by the `wordsplit` subsystem.

Role evidenced by:
- referenced local definition region and type usage in `src/wordsplit/wordsplit.c`

Required conceptual responsibility:
- represents internal linked or staged elements used by the wordsplit pipeline.

Relationship:
- associated with `wordsplit` as internal processing/storage structure.

This specification does not require exposing `wordsplit_node` publicly unless needed by the existing module boundary.

#### `exptab`
Expansion-table entry type used to decide which expansion operations apply.

Role evidenced by:
- `exptab_matches`
- `wordsplit_process_list`

Relationship:
- evaluated against a `wordsplit` state,
- used to drive staged expansion processing.

#### `passwd`
Account record type used during tilde expansion.

Role evidenced by:
- `wordsplit_tildexpand`

Relationship:
- consulted to resolve account-based home-directory expansion behavior.

## Success Criteria

1. **Correct word boundary handling**
   For representative inputs with delimiters and quotes, the Rust module produces the same word segmentation outcomes as the C module’s scanning path.

   Traceability:
   - `skip_delim_internal`
   - `skip_delim`
   - `skip_delim_real`
   - `scan_qstring`
   - `scan_word`
   - `wordsplit_run`

2. **Correct quoted-input failure behavior**
   For malformed quoted input, the Rust module reports failure through the same top-level status path rather than silently producing completed output.

   Traceability:
   - `wordsplit_len`
   - `wordsplit`

3. **Correct unquoting transformation**
   For inputs copied through the unquote path, the Rust module produces output text equivalent to the C module for both quoted-context and non-quoted-context cases.

   Traceability:
   - `wordsplit_string_unquote_copy`

4. **Correct tilde expansion behavior**
   For tilde-eligible inputs, the Rust module produces expanded results equivalent to the C module, including cases requiring account lookup.

   Traceability:
   - `wordsplit_tildexpand`
   - `passwd`

5. **Correct pathname expansion behavior**
   For pathname-pattern inputs, the Rust module produces final word results equivalent to the C module’s pathname expansion stage.

   Traceability:
   - `wordsplit_pathexpand`

6. **Correct staged expansion processing**
   For inputs that require multiple expansion steps, the Rust module processes applicable expansion entries and yields final results consistent with the C module’s expansion-list processing.

   Traceability:
   - `exptab_matches`
   - `wordsplit_process_list`

7. **Consistent top-level entry behavior**
   `wordsplit_len` and `wordsplit` in Rust must drive the same processing pipeline and produce equivalent results for the same logical input.

   Traceability:

8. **Cleanup correctness**
   After calling the Rust equivalents of word and environment-buffer cleanup, the `wordsplit` state no longer retains those owned allocations.

   Traceability:
   - `wordsplit_free_words`
   - `wordsplit_free_envbuf`

9. **No required functionality omitted from this source slice**
   The Rust rewrite includes all behavior evidenced by the analyzed functions in this module slice: scanning, unquoting, tilde expansion, pathname expansion, expansion-list processing, top-level execution, and cleanup.

   Traceability: