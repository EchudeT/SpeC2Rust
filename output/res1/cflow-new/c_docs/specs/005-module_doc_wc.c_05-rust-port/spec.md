# spec.md

## Title

Rust Functional Specification for `module_doc_wc.c_05`

## Metadata

- Project: `cflow-new`
- Source module: `doc/wc.c`
- Module category: `module_cluster`
- Target Rust branch: `005-module_doc_wc.c_05-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides word-count style processing for input files. Its behavior is centered on reading a file stream, identifying words, counting words and lines, and reporting per-file totals. It also provides formatted error reporting for ordinary errors and system-call-related errors.

The Rust rewrite must preserve the module’s observable behavior as evidenced by `doc/wc.c`, including:

- counting words from file content using the module’s word classification rules,
- counting lines from file content,
- reporting counts for a named file,
- reporting failures encountered while opening or processing a file.

## Scope

In scope for this module:

- file-oriented counting initiated for a named file,
- extraction of words from a byte stream,
- per-file reporting of character, word, and line totals,
- formatted error output for non-`errno` and `errno`-based failures.

Out of scope:

- aggregation across multiple files,
- command-line parsing,
- output formatting beyond the reporting behavior evidenced in this module,
- any new API surface not required to preserve the source module behavior.

## Source Traceability

| Source function | Required behavior covered in this spec |
|---|---|
| `error_print` | common formatted error emission, optionally including system error text |
| `errf` | formatted error emission without system error text |
| `perrf` | formatted error emission with system error text |
| `report` | output of counts associated with a file name |
| `isword` | classification of whether a byte belongs to a word |
| `getword` | scanning the next word from a file stream |
| `counter` | end-to-end counting flow for a named file, including open/read/report/error handling |

## Feature Specification

### Feature 1: File-based counting workflow

The module accepts a file name, opens that file for reading, scans its contents, computes counts, and emits a single report line for that file when processing succeeds.

The Rust version must implement the same workflow boundary:

1. take a file identifier/path equivalent to the C module’s `char *file`,
2. attempt to open it for reading,
3. if opening fails, emit an error through the module’s error-reporting behavior,
4. if opening succeeds, scan the file and compute the counts required by the module,
5. emit the resulting counts together with the file name through the module’s report behavior.

### Feature 2: Word detection and extraction

The module defines a word classifier and a stream scanner that advances through a file until it finds and consumes the next word. The Rust version must preserve the module’s notion of a word as determined by `isword` and used by `getword`.

The required behavior is:

- bytes are tested individually for word membership,
- scanning skips non-word content until a word begins or end-of-file is reached,
- once a word begins, scanning continues through contiguous word bytes,
- the scanner returns a result that allows the caller to distinguish “a word was found” from end-of-file.

This feature is limited to the evidenced byte-based classification and scanning behavior; no additional tokenization rules may be introduced.

### Feature 3: Line counting

The module computes line totals while processing file content. The Rust version must preserve line counting for the same input stream processed by the module.

A line count must reflect newline-delimited lines as evidenced by the source module’s counting behavior during stream traversal.

### Feature 4: Count reporting

The module emits a report consisting of the file name and three counts: character count, word count, and line count.

The Rust version must provide equivalent reporting behavior for successful processing, with the same count categories and association to the processed file.

### Feature 5: Error reporting

The module supports two observable classes of error output:

- formatted errors without appending system error text,
- formatted errors that include current system error information.

The Rust version must preserve these two behaviors for failures that arise in this module’s scope, especially file open or read failures evidenced by `counter` and the error helpers.

## User Scenarios & Testing

### Scenario 1: Count a readable text file

A caller asks the module to process a valid readable file containing ordinary text.

Expected behavior:

- the file is opened,
- the full file is scanned,
- words are counted according to the module’s word-classification rules,
- lines are counted from the file content,
- a report is emitted for that file with character, word, and line counts.

Test focus:

- processing completes without error output,
- exactly one report is produced for the file,
- reported counts match the source module for the same input.

### Scenario 2: Process a file containing punctuation and separators

A caller provides a file whose content mixes letters, digits, punctuation, whitespace, and line breaks.

Expected behavior:

- non-word bytes are skipped between words,
- contiguous word bytes are treated as one word,
- punctuation and other separators do not themselves produce words unless classified as word bytes by the source rules,
- final word and line counts match the source module.

Test focus:

- compare Rust and C outputs on fixtures with mixed separators,
- verify that adjacent separators do not inflate word count,
- verify word boundaries match the source module’s `isword`/`getword` behavior.

### Scenario 3: Process an empty file

A caller provides a readable file with zero bytes.

Expected behavior:

- the file opens successfully,
- scanning reaches end-of-file immediately,
- the module emits a report for the file,
- word count is zero,
- line count reflects the source module behavior for empty input,
- character count reflects the source module behavior for empty input.

Test focus:

- no error output,
- report is still emitted,
- counts match the source module exactly.

### Scenario 4: Process a nonexistent or unreadable file

A caller provides a file path that cannot be opened.

Expected behavior:

- no success report is emitted for that file,
- an error is emitted through the system-error-reporting path,
- the error message includes the caller-supplied file context as supported by the source formatting behavior.

Test focus:

- failure path is observable,
- system error text is included,
- output is traceable to the attempted file.

### Scenario 5: Detect end-of-file after the last word

A caller processes a file where the final token is a word and the file may or may not end with a separator.

Expected behavior:

- the last word is counted exactly once,
- scanning then terminates cleanly at end-of-file,
- no extra word is produced after end-of-file.

Test focus:

- compare behavior on files with and without trailing newline or punctuation,
- ensure the Rust scanner’s final-token handling matches `getword`.

## Requirements

### Functional Requirements

#### FR-1: Open and process a named file
Traceability: `counter`

The module shall accept a file name/path input and attempt to open that file for reading in order to perform counting.

#### FR-2: Emit an error on file open or processing failure
Traceability: `counter`, `perrf`, `error_print`

If the file cannot be opened, or if an `errno`-style failure arises within this module’s processing scope, the module shall emit an error message using the system-error-reporting behavior.

#### FR-3: Classify bytes as word or non-word
Traceability: `isword`

The module shall apply a single byte-classification rule to determine whether an input byte belongs to a word.

#### FR-4: Scan the next word from a stream
Traceability: `getword`, `isword`

The module shall traverse the input stream by skipping non-word bytes, consuming one contiguous sequence of word bytes as one word, and returning a result that distinguishes a found word from end-of-file.

#### FR-5: Count words in the processed file
Traceability: `counter`, `getword`

The module shall increment the word total once for each word found by the stream scanner.

#### FR-6: Count lines in the processed file
Traceability: `counter`

The module shall compute a line total from the file content during processing.

#### FR-7: Maintain a character count for reporting
Traceability: `report`, `counter`

The module shall produce a character-count value for each successfully processed file and include it in the report output.

#### FR-8: Report per-file totals on success
Traceability: `report`, `counter`

For each file successfully processed, the module shall emit a report that associates the file name with its character, word, and line counts.

#### FR-9: Support formatted non-system error messages
Traceability: `errf`, `error_print`

The module shall support emitting formatted error messages that do not append system error text.

#### FR-10: Support formatted system error messages
Traceability: `perrf`, `error_print`

The module shall support emitting formatted error messages that include current system error text.

### Key Entities

#### Entity: File input target
Traceability: `counter`

A named file supplied to the module for processing. It is the unit of work for open, scan, count, and report operations.

Relationship:
- one file input target produces either one error outcome or one report outcome.

#### Entity: Input stream
Traceability: `getword`, `counter`

The readable stream derived from the file input target. It is traversed to detect words and derive counts.

Relationship:
- the input stream is opened from the file input target,
- the scanner reads this stream to identify words,
- counting logic derives totals from this stream.

#### Entity: Word token
Traceability: `isword`, `getword`

A contiguous sequence of bytes that satisfy the module’s word-classification rule.

Relationship:
- word tokens are extracted from the input stream,
- each extracted word token contributes exactly one increment to the word count.

#### Entity: Count set
Traceability: `report`, `counter`

The per-file totals consisting of character count, word count, and line count.

Relationship:
- the count set is computed from the input stream,
- the count set is emitted by the reporting function together with the file name.

#### Entity: Error message
Traceability: `error_print`, `errf`, `perrf`

A formatted diagnostic emitted by the module, optionally augmented with system error text.

Relationship:
- errors may be produced during file open or processing failure,
- non-system and system-error variants share common formatting behavior.

## Success Criteria

### SC-1: Behavioral parity for successful file processing
Traceability: `counter`, `report`, `getword`, `isword`

For a corpus of representative input files, including empty files, single-line text, multi-line text, and mixed-separator text, the Rust module shall produce the same per-file character, word, and line counts as the source C module.

### SC-2: Behavioral parity for word boundary detection
Traceability: `isword`, `getword`

For fixture inputs designed to exercise leading separators, repeated separators, trailing separators, and final-word-at-EOF cases, the Rust module shall return the same word count results as the source C module.

### SC-3: Correct failure reporting for unreadable files
Traceability: `counter`, `perrf`, `error_print`

When given a nonexistent or unreadable file, the Rust module shall emit an error outcome instead of a success report, and the error output shall include system error information analogous to the source module behavior.

### SC-4: One report per successfully processed file
Traceability: `counter`, `report`

For each file that opens and processes successfully, the Rust module shall emit exactly one report associated with that file.

### SC-5: No spurious words at end-of-file
Traceability: `getword`

For inputs whose final content ends immediately after a word or after trailing separators, the Rust module shall count the final word exactly once and shall stop scanning at end-of-file without producing extra words.

## Constraints

- The Rust rewrite must preserve only the functionality evidenced by `doc/wc.c`.
- No additional public capabilities shall be introduced beyond the module’s existing functional boundary.
- This specification defines observable behavior and required outcomes, not a mandated Rust implementation strategy.