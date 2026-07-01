# Functional Specification: `module_doc_wc.c_05`

- **Project**: `cflow-new`
- **Module**: `module_doc_wc.c_05`
- **Category**: `module_cluster`
- **Source file**: `doc/wc.c`
- **Rust port target branch**: `005-module_doc_wc.c_05-rust-port`
- **Generation date**: `2026-06-17`

## 1. Overview

This module provides word-count style processing for a single input stream or file path. It counts characters, words, and lines from file content, reports the resulting totals in a fixed textual format, and emits formatted error messages for failures encountered while opening or processing files.

The Rust rewrite must preserve the module’s observed functional boundaries:

- classify input bytes into word and non-word content,
- scan input and compute character, word, and line counts,
- print one result line per processed file,
- report errors in formatted text, with and without system error context.

This module is function-oriented and does not define persistent custom structs in the analyzed source.

## 2. Feature Specification

### 2.1 Counting and reporting file content

The module accepts a file identifier and processes the corresponding input content to produce three counts:

- character count,
- word count,
- line count.

After counting completes successfully, the module outputs a single report line containing the three counts and the file name. The Rust version must preserve this behavior as the module’s primary successful outcome.

### 2.2 Word detection behavior

The module distinguishes word characters from separators through a dedicated classification rule. A word is recognized as a contiguous run of characters accepted by the module’s word-character predicate. The Rust version must preserve the same functional role:

- determine whether a byte belongs to a word,
- use this classification to advance word counting during stream scanning.

Because the source exposes a dedicated `isword` helper and a `getword` scanner, the Rust rewrite must keep equivalent observable behavior for word boundary detection and word counting.

### 2.3 Stream scanning

The module reads from an input stream and scans content incrementally. It detects words, counts characters, and counts line terminators. The Rust version must support equivalent stream-based counting behavior for file content.

### 2.4 Error reporting

The module emits formatted diagnostics in two modes:

- generic formatted errors,
- formatted errors augmented by system error context.

The Rust version must preserve both behaviors as distinct outcomes:

- an error path for formatted reporting without OS error text,
- an error path for formatted reporting with OS error text when file operations fail.

## 3. User Scenarios & Testing

### Scenario 1: Count a normal text file
A caller requests processing of a readable text file containing multiple words across one or more lines.

**Expected behavior**
- The module opens and scans the file.
- It counts characters, words, and lines.
- It prints exactly one result line for that file.

**Test focus**
- Verify reported counts match file content.
- Verify the file name appears in the report line.

### Scenario 2: Count an empty file
A caller requests processing of a readable file with no content.

**Expected behavior**
- The module completes without error.
- It reports zero characters, zero words, and zero lines.

**Test focus**
- Verify all reported counts are zero.
- Verify output formatting remains valid.

### Scenario 3: Count a file with multiple separators
A caller processes a file where words are separated by spaces, tabs, newlines, or punctuation/non-word content.

**Expected behavior**
- The module counts words according to its word-character classification.
- Contiguous separators do not create extra words.
- Line count reflects newline occurrences.

**Test focus**
- Verify words are counted only for valid word runs.
- Verify line count corresponds to newline characters in input.

### Scenario 4: Process a missing or unreadable file
A caller requests processing of a file that cannot be opened.

**Expected behavior**
- The module does not produce a normal count report for that file.
- It emits a formatted error message that includes system error context.

**Test focus**
- Verify an error is reported.
- Verify the error path is distinguishable from a successful report.

### Scenario 5: Report counts directly
A caller already has computed count values and requests output formatting through the reporting function.

**Expected behavior**
- The module prints a single line containing the provided counts and file name.

**Test focus**
- Verify formatting is stable for representative count values.
- Verify the output includes all three counts and the file identifier.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: File-based counting
The module shall provide functionality to process a file identified by name, read its contents, and derive character, word, and line totals.

**Traceability**: `counter` in `doc/wc.c`

#### FR-2: Report generation
The module shall provide functionality to emit a formatted report line containing a file identifier and its character, word, and line counts.

**Traceability**: `report` in `doc/wc.c`

#### FR-3: Word classification
The module shall provide functionality to classify whether an input byte is part of a word for use in counting logic.

**Traceability**: `isword` in `doc/wc.c`

#### FR-4: Word scanning support
The module shall provide functionality to scan an input stream for the next word occurrence using the module’s word-classification rule.

**Traceability**: `getword` in `doc/wc.c`

#### FR-5: Character counting
The module shall count input characters while processing file content.

**Traceability**: `counter` in `doc/wc.c`

#### FR-6: Word counting
The module shall count words found in file content according to the module’s word-detection behavior.

**Traceability**: `getword`, `counter` in `doc/wc.c`

#### FR-7: Line counting
The module shall count lines from file content during processing.

**Traceability**: `counter` in `doc/wc.c`

#### FR-8: Generic formatted error output
The module shall provide formatted error reporting that emits a caller-supplied message without requiring system error text.

**Traceability**: `error_print`, `errf` in `doc/wc.c`

#### FR-9: System-context error output
The module shall provide formatted error reporting that emits a caller-supplied message together with system error context for file operation failures.

**Traceability**: `error_print`, `perrf`, `counter` in `doc/wc.c`

### 4.2 Key Entities

#### File identifier
A file name or file-designating string supplied to counting and reporting operations.

**Relationships**
- Used by file-processing logic to open input.
- Included in success reports.
- Included in error messages associated with file failures.

**Traceability**: `counter`, `report`

#### Input stream
A readable stream used as the source for word scanning and content counting.

**Relationships**
- Consumed by the word-scanning function.
- Opened from a file identifier by file-processing logic.

**Traceability**: `getword`, `counter`

#### Count values
Three numeric totals representing characters, words, and lines.

**Relationships**
- Produced by file-processing logic.
- Passed to reporting logic for output.

**Traceability**: `report`, `counter`

#### Error message
A formatted diagnostic string, optionally accompanied by system error information.

**Relationships**
- Produced by generic or system-context error functions.
- Used when file processing cannot proceed normally.

**Traceability**: `error_print`, `errf`, `perrf`

## 5. Success Criteria

### SC-1: Correct successful reporting
For a readable input file, the Rust module produces one report line containing the file identifier and the computed character, word, and line counts.

**Traceability**: `counter`, `report`

### SC-2: Correct zero-count behavior
For an empty readable file, the Rust module reports zero characters, zero words, and zero lines.

**Traceability**: `counter`, `report`

### SC-3: Word counting follows module classification
For test inputs containing mixed word characters and separators, the Rust module’s reported word count matches the count implied by the module’s word-classification and stream-scanning behavior.

**Traceability**: `isword`, `getword`, `counter`

### SC-4: Line counting reflects input newlines
For test inputs with known newline counts, the Rust module’s reported line count matches the number of line terminators counted by the original module behavior.

**Traceability**: `counter`

### SC-5: File-open failures are reported as errors
When asked to process a missing or unreadable file, the Rust module emits an error message through the system-context error path and does not emit a normal success report for that file.

**Traceability**: `perrf`, `counter`

### SC-6: Direct report formatting is preserved
When provided explicit count values and a file identifier, the Rust module emits a single formatted report line containing all supplied values.

**Traceability**: `report`

## 6. Out of Scope

The Rust rewrite specification does not require any capability not evidenced in the analyzed module, including:

- aggregation across multiple files beyond per-file processing,
- new public APIs beyond the module’s observed functional roles,
- concurrency or thread-safety guarantees,
- structured error types exposed as a new external contract,
- serialization, persistence, networking, or FFI behavior.