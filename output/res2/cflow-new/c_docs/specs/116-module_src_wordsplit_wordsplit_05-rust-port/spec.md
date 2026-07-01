# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_05`

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_05`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Target Rust branch: `116-module_src_wordsplit_wordsplit_05-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines the cleanup, error-state, and result-access behavior for a `wordsplit` parsing context. The Rust rewrite must preserve the observable behavior of the C module section represented by the following operations:

- releasing parameter-buffer resources associated with a `wordsplit` instance,
- clearing recorded error state on a `wordsplit` instance,
- freeing the full `wordsplit` instance state,
- exposing the produced word list and word count,
- returning a human-readable error description for the current `wordsplit` state,
- printing that error information through the module’s error-reporting path.

The module does not define the word-splitting algorithm itself in this analyzed slice; it defines lifecycle and retrieval behavior around an already-populated `wordsplit` state.

## Feature Specification

### Feature: `wordsplit` lifecycle finalization and result access

The Rust module must provide behavior equivalent to the C module for managing a `wordsplit` context after parsing or during teardown.

Supported functional boundaries:

1. **Parameter buffer cleanup**
   - The module must support releasing any parameter-buffer-related storage held by a `wordsplit` instance without requiring full destruction of the whole instance.
   - After this cleanup, the parameter-buffer state must no longer remain owned by the instance.

2. **Error state clearing**
   - The module must support resetting the stored error indication of a `wordsplit` instance.
   - After clearing, subsequent error reporting for the instance must reflect the cleared state rather than a previous failure condition.

3. **Full context cleanup**
   - The module must support freeing all resources owned by a `wordsplit` instance, including dynamically managed internal state associated with split results and related buffers.
   - Destruction must be safe for a context that has previously undergone partial cleanup such as parameter-buffer release or error clearing.

4. **Word result retrieval**
   - The module must support returning the current split result as:
     - a word count, and
     - a word vector/list reference corresponding to the current `wordsplit` state.
   - Retrieval must reflect the state already stored in the instance; this operation is an accessor, not a new split action.

5. **Error message retrieval**
   - The module must support obtaining a textual description of the current error condition from a `wordsplit` instance.
   - The returned text must be suitable for user-facing diagnostics.

6. **Error printing**
   - The module must support emitting an error report for a `wordsplit` instance using the current stored error information.
   - The emitted output must be based on the same error state exposed by the error-string retrieval behavior.

## User Scenarios & Testing

### Scenario 1: Caller retrieves split results after successful processing
A caller holds a populated `wordsplit` context and needs access to the generated words.

Expected behavior:
- The caller can obtain the number of words.
- The caller can obtain the corresponding word sequence.
- The retrieved values match the current state stored in the context.

Test focus:
- Accessor returns the stored count and word list.
- No new parsing or mutation is required to retrieve results.

### Scenario 2: Caller clears a previously recorded error
A caller has a `wordsplit` context with an error recorded and wants to reset it before reuse or before suppressing diagnostics.

Expected behavior:
- Error state can be cleared explicitly.
- After clearing, querying the error string reflects the cleared condition rather than the old error.

Test focus:
- Clear operation changes later error reporting outcome.
- Repeated clearing does not reintroduce prior error information.

### Scenario 3: Caller prints diagnostic information for a failed context
A caller wants the module to emit an error message for the current `wordsplit` state.

Expected behavior:
- The module produces output derived from the context’s stored error.
- The printed diagnostic is consistent with the string returned by the error-string accessor.

Test focus:
- Printed message corresponds to current error state.
- After `clearerr`, printed output reflects the cleared/non-previous-error state.

### Scenario 4: Caller releases parameter-buffer resources separately
A caller no longer needs parameter-expansion buffer data but still retains the `wordsplit` context for other state inspection or staged cleanup.

Expected behavior:
- Parameter-buffer storage can be released independently.
- Subsequent full cleanup remains valid.

Test focus:
- Separate parameter-buffer cleanup succeeds.
- Full destruction after partial cleanup succeeds without requiring the buffer to still exist.

### Scenario 5: Caller destroys the `wordsplit` context at end of use
A caller finishes with the split context and releases all owned resources.

Expected behavior:
- All resources owned by the context are released by the full cleanup path.
- Cleanup remains valid whether or not error state was set, results were retrieved, or parameter buffers were separately freed first.

Test focus:
- Destruction covers successful and failed contexts.
- Destruction after result retrieval and after partial cleanup is supported.

## Requirements

### Functional Requirements

#### FR-1: Parameter buffer release
The Rust module shall provide a functionally equivalent operation to release parameter-buffer resources associated with a `wordsplit` instance.
**Traceability:** `wordsplit_free_parambuf`, `struct wordsplit`.

#### FR-2: Error-state reset
The Rust module shall provide a functionally equivalent operation to clear the current error state stored in a `wordsplit` instance.
**Traceability:** `wordsplit_clearerr`, `struct wordsplit`.

#### FR-3: Full `wordsplit` resource release
The Rust module shall provide a functionally equivalent operation to release all resources owned by a `wordsplit` instance.
**Traceability:** `wordsplit_free`, `struct wordsplit`, `struct wordsplit_node`.

#### FR-4: Retrieval of stored split results
The Rust module shall provide a functionally equivalent operation to expose the current stored word count and word collection from a `wordsplit` instance.
**Traceability:** `wordsplit_get_words`, `struct wordsplit`.

#### FR-5: Retrieval of current error text
The Rust module shall provide a functionally equivalent operation to return a human-readable error string representing the current error state of a `wordsplit` instance.
**Traceability:** `wordsplit_strerror`, `struct wordsplit`.

#### FR-6: Emission of current error report
The Rust module shall provide a functionally equivalent operation to print or otherwise emit the current error report for a `wordsplit` instance using the stored diagnostic state.
**Traceability:** `wordsplit_perror`, `struct wordsplit`.

#### FR-7: Consistent error reporting across accessors
The Rust module shall ensure that the error text returned by the error-string accessor and the error report emitted by the print/report operation are derived from the same current `wordsplit` error state.
**Traceability:** `wordsplit_strerror`, `wordsplit_perror`, `wordsplit_clearerr`, `struct wordsplit`.

#### FR-8: Cleanup compatibility across operation orderings
The Rust module shall preserve valid lifecycle behavior when parameter-buffer cleanup, error clearing, result retrieval, and final cleanup are invoked in the combinations evidenced by this module’s API boundaries.
**Traceability:** `wordsplit_free_parambuf`, `wordsplit_clearerr`, `wordsplit_free`, `wordsplit_get_words`, `struct wordsplit`.

### Key Entities

#### `wordsplit`
The primary module state object. It owns or references:
- the current split result, including word count and word vector/list exposure,
- the current error status and related diagnostic information,
- parameter-buffer-related storage,
- internal dynamically managed resources that must be released during final cleanup.

**Traceability:** `struct wordsplit`, all listed functions.

#### `wordsplit_node`
An internal node-based resource associated with `wordsplit`-owned state and relevant to full cleanup behavior.

**Traceability:** `struct wordsplit_node`, `wordsplit_free`.

## Success Criteria

1. **Result access correctness**
   - Given a populated `wordsplit` instance, the Rust module returns the same stored word count and corresponding word collection state as the C module boundary exposes.
   **Traceability:** `wordsplit_get_words`.

2. **Error reset correctness**
   - After clearing error state, the Rust module no longer reports the prior error through its error-string behavior.
   **Traceability:** `wordsplit_clearerr`, `wordsplit_strerror`.

3. **Diagnostic consistency**
   - For a given `wordsplit` state, the Rust module’s printed/emitted diagnostic corresponds to the same error condition represented by its error-string accessor.
   **Traceability:** `wordsplit_strerror`, `wordsplit_perror`.

4. **Partial then full cleanup validity**
   - A `wordsplit` instance remains safely destructible after parameter-buffer cleanup has already been performed.
   **Traceability:** `wordsplit_free_parambuf`, `wordsplit_free`.

5. **Lifecycle coverage**
   - The Rust module supports successful completion of the scenarios in this document: result retrieval, error clearing, error reporting, parameter-buffer cleanup, and final cleanup.
   **Traceability:** all listed functions, `struct wordsplit`.

6. **No required behavior regression within analyzed scope**
   - The Rust rewrite preserves the functional boundaries of cleanup, error handling, result access, and error reporting evidenced in `src/wordsplit/wordsplit.c` for this module slice.
   **Traceability:** `wordsplit_free_parambuf`, `wordsplit_clearerr`, `wordsplit_free`, `wordsplit_get_words`, `wordsplit_strerror`, `wordsplit_perror`.