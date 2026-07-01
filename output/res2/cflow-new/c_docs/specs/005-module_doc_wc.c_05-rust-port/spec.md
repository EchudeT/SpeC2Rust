# Functional Specification: `module_doc_wc.c_05`

- **Project**: `cflow-new`
- **Module**: `module_doc_wc.c_05`
- **Category**: `module_cluster`
- **Source file**: `doc/wc.c`
- **Rust target branch**: `005-module_doc_wc.c_05-rust-port`
- **Generation date**: `2026-06-17`

## 1. Overview

This module provides word-count style document counting for a single input stream or named file. Its functional scope is limited to:

- counting lines, words, and characters from input text,
- printing count reports in a fixed report format,
- printing formatted error messages, optionally including system error text.

The Rust rewrite must preserve this behavior as the module-level functionality evidenced by `doc/wc.c`, especially the interaction between counting, reporting, and error handling.

## 2. Feature Specification

### 2.1 Document counting

The module shall support counting text content and producing three totals:

- character count,
- word count,
- line count.

The counting behavior is evidenced by `counter` and `getword`.

A word is determined by the module’s internal word classification routine. The Rust version must preserve the same functional boundary: word counting is based on scanning input and recognizing word/non-word transitions rather than splitting on lines or pre-tokenized input.

### 2.2 Word extraction behavior

The module shall scan an input stream incrementally and identify the next word occurrence until end of file.

This behavior is evidenced by `getword` and `isword`.

The Rust version must implement equivalent stream-based word detection semantics:

- consume input from a file-like source,
- skip non-word content,
- recognize a word when word-class characters begin,
- stop at end of file.

The specification does not require exposing word contents because the analyzed module evidence shows counting behavior, not externally consumed token storage.

### 2.3 Reporting

The module shall output a report line containing the computed counts associated with a file label.

This behavior is evidenced by `report`.

The Rust version must preserve:

- reporting for a given file label,
- inclusion of character, word, and line totals in the report output,
- output ordering consistent enough for callers/tests to validate that the report corresponds to the supplied file and counts.

### 2.4 Error reporting

The module shall support two forms of formatted error output:

- formatted error messages without system error text,
- formatted error messages that include the active system error description.

This behavior is evidenced by `errf`, `perrf`, and their shared formatter `error_print`.

The Rust version must preserve the distinction between:

- plain formatted errors, and
- errors augmented with underlying OS or I/O error information.

### 2.5 File-oriented counting entry point

The module shall provide a file-oriented counting operation that accepts a file identifier/name, opens or reads that target, performs counting, and reports the result or emits an error.

This behavior is evidenced by `counter`.

The Rust version must preserve:

- acceptance of a file label/path input,
- attempted processing of that input as a source to count,
- successful report emission on readable input,
- error emission on failure.

## 3. User Scenarios & Testing

### 3.1 Count a readable text file

**Scenario**: A caller requests counting for a valid readable text file.

**Expected behavior**:
- the file is processed to completion,
- characters, words, and lines are counted,
- one report line is emitted for that file.

**Test guidance**:
- use a fixture file with known counts,
- verify the reported file label matches the input,
- verify all three totals match expected values.

### 3.2 Count an empty file

**Scenario**: A caller requests counting for an empty file.

**Expected behavior**:
- processing succeeds,
- all reported totals are zero,
- the file still receives a report line.

**Test guidance**:
- create a zero-length file,
- verify report totals are `0` for characters, words, and lines.

### 3.3 Count text containing separators and multiple words

**Scenario**: A caller provides a file containing words separated by spaces, punctuation, or other non-word characters.

**Expected behavior**:
- non-word regions do not count as words,
- each detected word contributes once to the word total,
- character and line totals reflect the full file contents.

**Test guidance**:
- include mixed separators,
- verify word count is based on word/non-word scanning rather than simple line count.

### 3.4 Count text spanning multiple lines

**Scenario**: A caller provides a file with multiple newline-terminated or partially terminated lines.

**Expected behavior**:
- line totals reflect newline handling used by the original module,
- word counting continues across line boundaries correctly,
- character totals include line-ending characters present in the file.

**Test guidance**:
- test files with several lines,
- include a case with and without a trailing newline.

### 3.5 Handle unreadable or missing file input

**Scenario**: A caller requests counting for a file that cannot be opened or read.

**Expected behavior**:
- no success report is emitted for that file,
- an error message is printed,
- when system error context exists, the error output includes that context.

**Test guidance**:
- use a non-existent path or a path without read permission,
- verify error output references the requested file,
- verify failure path uses OS error text when applicable.

### 3.6 Format a direct report from known counts

**Scenario**: A caller already has counts and invokes report formatting directly.

**Expected behavior**:
- output includes the provided file label and the three provided totals,
- output does not alter the supplied values.

**Test guidance**:
- call the reporting function with fixed count values,
- compare output against expected formatted structure.

## 4. Requirements

### 4.1 Functional Requirements

**FR-1. Counting operation**
The module shall provide a counting operation for a named file input that computes character, word, and line totals and then reports them on success.
**Traceability**: `counter`, `report` in `doc/wc.c`.

**FR-2. Stream-based word detection**
The module shall detect words by scanning input bytes/characters and distinguishing word characters from non-word characters.
**Traceability**: `getword`, `isword` in `doc/wc.c`.

**FR-3. Repeated word retrieval until EOF**
The module shall support repeated word-scanning progression over a stream until no further words are available because input is exhausted.
**Traceability**: `getword` in `doc/wc.c`.

**FR-4. Character counting**
The module shall count characters from the processed input and include that total in the final report.
**Traceability**: `counter`, `report` in `doc/wc.c`.

**FR-5. Line counting**
The module shall count lines from the processed input and include that total in the final report.
**Traceability**: `counter`, `report` in `doc/wc.c`.

**FR-6. Word counting**
The module shall count detected words from the processed input and include that total in the final report.
**Traceability**: `counter`, `getword`, `report` in `doc/wc.c`.

**FR-7. Report formatting**
The module shall provide report output for a supplied file label and supplied count values.
**Traceability**: `report` in `doc/wc.c`.

**FR-8. Plain formatted error output**
The module shall provide formatted error output that does not require inclusion of system error text.
**Traceability**: `errf`, `error_print` in `doc/wc.c`.

**FR-9. System-error formatted output**
The module shall provide formatted error output that includes current system error information when reporting failures tied to OS or I/O operations.
**Traceability**: `perrf`, `error_print` in `doc/wc.c`.

**FR-10. Failure handling during file processing**
The module shall emit an error instead of a success report when file processing cannot be completed because the target cannot be accessed or read as required by the counting operation.
**Traceability**: `counter`, `perrf` in `doc/wc.c`.

### 4.2 Key Entities

**File label / file input identifier**
A caller-supplied string identifying the file to process or to name in output. It is used by the counting entry point and by report/error output.
**Traceability**: `counter (char *file)`, `report (char *file, ...)`.

**Input stream**
The readable stream from which text is scanned for words and counted for aggregate totals.
**Traceability**: `getword (FILE *fp)`.

**Count values**
Three numeric totals are central to the module’s behavior:
- character count,
- word count,
- line count.

These are computed during counting and consumed by the reporting function.
**Traceability**: `report (char *file, count_t ccount, count_t wcount, count_t lcount)`, `counter`.

**Word-character classification**
An internal classification rule that determines whether an input unit participates in a word. This rule governs word boundary detection.
**Traceability**: `isword`, `getword`.

**Formatted error message**
A formatted output entity built from a format string and arguments, with an optional attached system error description.
**Traceability**: `error_print`, `errf`, `perrf`.

## 5. Success Criteria

**SC-1. Correct totals for known fixtures**
For fixture files with known character, word, and line totals, the Rust module produces matching reported totals.
**Traceability**: `counter`, `getword`, `report`.

**SC-2. Empty-file correctness**
For an empty file, the Rust module reports zero characters, zero words, and zero lines.
**Traceability**: `counter`, `report`.

**SC-3. Word-boundary correctness**
For test inputs containing mixtures of word and non-word characters, the Rust module’s word total matches the original module’s word-detection behavior.
**Traceability**: `isword`, `getword`, `counter`.

**SC-4. Multi-line correctness**
For multi-line inputs, the Rust module reports line totals consistent with the original module and includes line-ending characters in character totals as applicable.
**Traceability**: `counter`, `report`.

**SC-5. Report output presence and association**
On successful processing, the Rust module emits exactly one report associated with the requested file label for each successful counting invocation.
**Traceability**: `counter`, `report`.

**SC-6. Error-path behavior**
When given an unreadable or missing file, the Rust module emits an error message and does not emit a success report for that failed invocation.
**Traceability**: `counter`, `perrf`, `error_print`.

**SC-7. Distinct plain vs system error formatting**
The Rust module preserves separate behaviors for plain formatted errors and system-error-formatted errors.
**Traceability**: `errf`, `perrf`, `error_print`.