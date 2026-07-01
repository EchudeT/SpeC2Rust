# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_05`

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_05`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `116-module_src_wordsplit_wordsplit_05-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines the cleanup, result access, and error-reporting behavior for a `wordsplit` processing context.

The Rust rewrite must preserve the observable behavior evidenced by the analyzed functions:

- release of internally owned parameter-buffer state
- clearing of stored error state
- full release of a `wordsplit` context
- retrieval of split-word results from a completed context
- conversion of the current error state to a message string
- printing of the current error state in a diagnostic form

The module scope is limited to lifecycle and result/error handling for an existing `wordsplit` object. The specification does not require the Rust version to add capabilities beyond those behaviors.

## Feature Specification

### Summary

The Rust version must provide functional equivalents for the analyzed module behaviors around a `wordsplit` state object:

1. **Parameter-buffer cleanup**
   - A context may hold parameter-buffer related allocations or temporary state.
   - The module must support releasing that state without requiring destruction of the entire context.

2. **Error-state reset**
   - A context stores error information.
   - The module must support clearing the stored error so later status inspection no longer reports the previous error condition.

3. **Full context cleanup**
   - A context owns memory and state associated with wordsplitting results and internal processing.
   - The module must support freeing or dropping the full context and its owned resources.

4. **Word-result access**
   - A context may contain produced words.
   - The module must support returning the current word count together with access to the current word vector, subject to the context state.

5. **Error-message access**
   - A context exposes its current error state as a human-readable message string.

6. **Diagnostic error output**
   - A context can emit its current error description in a perror-style form for diagnostics.

### Functional Boundary

This module is responsible for post-processing interactions with a `wordsplit` context after or around splitting work has occurred elsewhere in the source file. It is not specified here as the module that performs the actual parsing or tokenization algorithm.

## User Scenarios & Testing

### Scenario 1: Retrieve generated words from a valid context

A caller has a populated `wordsplit` context after prior processing and needs access to the produced words.

**Expected behavior**
- The module returns the number of words currently stored in the context.
- The module returns access to the corresponding word vector.
- The returned count and vector are consistent with the context contents.

**Test focus**
- Verify that a context with results exposes both count and word list.
- Verify that the returned word count matches the accessible number of entries.

### Scenario 2: Clear a previous error before reusing or re-inspecting a context

A caller has a context with an error recorded and wants to clear the error state.

**Expected behavior**
- After clearing, subsequent error inspection no longer reports the previous error condition.
- Human-readable error reporting reflects the cleared state according to module-defined behavior.

**Test focus**
- Set or simulate a context error state.
- Clear the error.
- Verify that later calls to error-to-string and diagnostic reporting no longer reflect the old error.

### Scenario 3: Release temporary parameter-buffer resources without destroying the whole context

A caller needs to remove parameter-buffer related allocations while preserving the outer context object.

**Expected behavior**
- Parameter-buffer owned resources are released.
- The operation does not require immediate destruction of the entire context.

**Test focus**
- Create a context with parameter-buffer state present.
- Invoke parameter-buffer cleanup.
- Verify that cleanup completes and the context remains destructible afterward.

### Scenario 4: Print a diagnostic message for the current error

A caller needs to emit an error report for a context in a user-visible or log-visible way.

**Expected behavior**
- The module prints a diagnostic corresponding to the current error state of the context.
- The printed information is based on the same underlying error state used by string-based error retrieval.

**Test focus**
- Put the context into a known error state.
- Capture diagnostic output from perror-style reporting.
- Verify that output reflects the current error condition.

### Scenario 5: Destroy a context and all remaining owned resources

A caller is finished with a `wordsplit` context and must release all associated resources.

**Expected behavior**
- Full cleanup releases resources owned by the context, including data relevant to this module’s result and temporary-state handling.
- Destruction is the terminal lifecycle operation for the context.

**Test focus**
- Create a context containing words, error state, and parameter-buffer state where applicable.
- Destroy the context.
- Verify no further use is required and cleanup completes without partial-release leaks in Rust ownership terms.

## Requirements

### Functional Requirements

#### FR-1: Parameter-buffer cleanup
The Rust module shall provide behavior equivalent to `wordsplit_free_parambuf`, releasing parameter-buffer related resources owned by a `wordsplit` context.

**Traceability**
- `wordsplit_free_parambuf` in `src/wordsplit/wordsplit.c:2785-2800`
- `struct wordsplit`

#### FR-2: Error-state clearing
The Rust module shall provide behavior equivalent to `wordsplit_clearerr`, clearing the current stored error state of a `wordsplit` context.

**Traceability**
- `wordsplit_clearerr` in `src/wordsplit/wordsplit.c:2802-2813`
- `struct wordsplit`

#### FR-3: Full context cleanup
The Rust module shall provide behavior equivalent to `wordsplit_free`, releasing resources owned by a `wordsplit` context, including state relevant to this module’s managed results and temporary buffers.

**Traceability**
- `wordsplit_free` in `src/wordsplit/wordsplit.c:2815-2829`
- `struct wordsplit`

#### FR-4: Word-result retrieval
The Rust module shall provide behavior equivalent to `wordsplit_get_words`, allowing callers to obtain the current word count and current word-vector contents from a `wordsplit` context.

**Traceability**
- `wordsplit_get_words` in `src/wordsplit/wordsplit.c:2831-2846`
- `struct wordsplit`

#### FR-5: Error string retrieval
The Rust module shall provide behavior equivalent to `wordsplit_strerror`, returning a human-readable string describing the current error state of a `wordsplit` context.

**Traceability**
- `wordsplit_strerror` in `src/wordsplit/wordsplit.c:2864-2872`
- `struct wordsplit`

#### FR-6: Diagnostic error reporting
The Rust module shall provide behavior equivalent to `wordsplit_perror`, emitting a diagnostic message derived from the current error state of a `wordsplit` context.

**Traceability**
- `wordsplit_perror` in `src/wordsplit/wordsplit.c:2874-2891`
- `struct wordsplit`

#### FR-7: Shared error-state consistency
The Rust module shall ensure that error clearing, error string retrieval, and diagnostic error output operate on the same underlying error state within a `wordsplit` context.

**Traceability**
- `wordsplit_clearerr` in `src/wordsplit/wordsplit.c:2802-2813`
- `wordsplit_strerror` in `src/wordsplit/wordsplit.c:2864-2872`
- `wordsplit_perror` in `src/wordsplit/wordsplit.c:2874-2891`
- `struct wordsplit`

#### FR-8: Shared result-state consistency
The Rust module shall ensure that returned word count and returned word-vector access represent the same current result state of the `wordsplit` context.

**Traceability**
- `wordsplit_get_words` in `src/wordsplit/wordsplit.c:2831-2846`
- `struct wordsplit`

### Key Entities

#### `Wordsplit` context
Primary state object corresponding to `struct wordsplit`.

**Role**
- Owns current wordsplit-related state handled by this module.
- Holds word-result state exposed to callers.
- Holds error state used for string conversion and diagnostic reporting.
- Holds parameter-buffer or related temporary resources that can be freed separately or during full cleanup.

**Relationships**
- Is the sole input object for all analyzed module functions.
- Owns or references word-vector data returned by word retrieval.
- Owns error information consumed by error-reporting operations.

#### `WordsplitNode`
Supporting internal node structure corresponding to `struct wordsplit_node`.

**Role**
- Participates in the internal representation associated with the broader `wordsplit` context.

**Relationships**
- Associated with `Wordsplit` as internal state.
- Not required to be directly exposed by the Rust public functional surface for this module, but any owned state relevant to cleanup must be handled during context destruction.

## Success Criteria

### SC-1: Parameter-buffer cleanup behavior
Given a `Wordsplit` context with parameter-buffer state, invoking the Rust equivalent of parameter-buffer cleanup releases that state and does not prevent subsequent full context cleanup.

**Traceability**
- FR-1
- `wordsplit_free_parambuf`

### SC-2: Error clearing behavior
Given a `Wordsplit` context with a known error state, invoking the Rust equivalent of error clearing causes subsequent error inspection to stop reporting that previous error.

**Traceability**
- FR-2
- FR-7
- `wordsplit_clearerr`
- `wordsplit_strerror`

### SC-3: Word retrieval behavior
Given a `Wordsplit` context containing split results, invoking the Rust equivalent of word retrieval returns a word count and word collection view that match the context’s current results.

**Traceability**
- FR-4
- FR-8
- `wordsplit_get_words`

### SC-4: Error string behavior
Given distinct observable error states in a `Wordsplit` context, invoking the Rust equivalent of error-string retrieval returns a human-readable message corresponding to the current state.

**Traceability**
- FR-5
- FR-7
- `wordsplit_strerror`

### SC-5: Diagnostic output behavior
Given a `Wordsplit` context with an error state, invoking the Rust equivalent of diagnostic reporting emits output derived from the same current error state used by error-string retrieval.

**Traceability**
- FR-6
- FR-7
- `wordsplit_perror`
- `wordsplit_strerror`

### SC-6: Full cleanup behavior
Given a `Wordsplit` context containing any combination of word results, error state, parameter-buffer state, and internal supporting state, final cleanup of the Rust context releases owned resources without requiring additional manual cleanup steps beyond the module-defined lifecycle operations.

**Traceability**
- FR-1
- FR-3
- `wordsplit_free`
- `wordsplit_free_parambuf`
- `struct wordsplit`
- `struct wordsplit_node`