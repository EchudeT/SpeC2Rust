# spec.md

## Title

Functional Specification: `module_src_wordsplit_wordsplit_05` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_05`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `116-module_src_wordsplit_wordsplit_05-rust-port`
- Generation date: `2026-06-11`

## Overview

This module defines the cleanup, error-state, and result-access portion of the `wordsplit` facility. The observed functionality centers on managing a `wordsplit` instance after word-splitting work has already populated its internal state.

The Rust rewrite must preserve the module behavior evidenced by the source-level interface:

- releasing parameter-buffer resources owned by a `wordsplit` instance,
- clearing the module-maintained error state,
- freeing all resources associated with a `wordsplit` instance,
- returning the current split-word result set to callers,
- returning a human-readable error string for the current error state,
- printing an error report for a `wordsplit` instance.

This specification covers only those behaviors directly evidenced by the identified functions and the `wordsplit` / `wordsplit_node` data structures.

## Feature Specification

### Feature Summary

The module provides lifecycle-finalization and inspection behavior for `wordsplit` objects:

1. **Owned temporary-buffer cleanup**
   - The module can release internal parameter-buffer state associated with a `wordsplit` object without necessarily destroying the entire object.

2. **Error-state reset**
   - The module can clear a previously stored error condition on a `wordsplit` object.

3. **Full object cleanup**
   - The module can free the resources held by a `wordsplit` object.

4. **Access to produced words**
   - The module can expose the current word count and word vector stored in a `wordsplit` object.

5. **Error text retrieval**
   - The module can provide a textual description of the current error state for a `wordsplit` object.

6. **Error report output**
   - The module can emit an error message for a `wordsplit` object in a perror-style usage pattern.

### In-Scope Behavior

The Rust version must implement behavior corresponding to these functions:

- `wordsplit_free_parambuf`
- `wordsplit_clearerr`
- `wordsplit_free`
- `wordsplit_get_words`
- `wordsplit_strerror`
- `wordsplit_perror`

### Out-of-Scope Behavior

The following are not specified here unless required to support the observed functions:

- parsing or splitting input text,
- defining new public configuration capabilities,
- concurrency guarantees,
- serialization or persistence,
- FFI surface changes,
- enhanced diagnostics beyond the observed error-text and perror behavior.

## User Scenarios & Testing

### Scenario 1: Caller retrieves split results from an existing `wordsplit` object

A caller has a `wordsplit` instance whose internal processing has already produced words. The caller requests the current word count and the current word vector.

Expected behavior:

- the module returns the result information through caller-provided outputs,
- the returned values reflect the state currently stored in the `wordsplit` object,
- the operation reports success or failure through its return value.

Tests:

- given a `wordsplit` object with stored words, `wordsplit_get_words` returns the expected count and vector references,
- given valid output pointers, the outputs are updated consistently,
- failure behavior is preserved if the source implementation reports failure for invalid or unusable state.

### Scenario 2: Caller clears a stale error before reusing or inspecting the object

A caller has a `wordsplit` instance that contains an error state from an earlier operation and wants the object error state reset.

Expected behavior:

- the module removes the stored error indication maintained by the object,
- subsequent error-text lookup reflects the cleared state.

Tests:

- after setting up an object with an error state, `wordsplit_clearerr` resets the state,
- `wordsplit_strerror` after clear no longer reports the previous error text.

### Scenario 3: Caller needs a human-readable explanation of the current error

A caller has a `wordsplit` object in an error state and needs a string description.

Expected behavior:

- the module returns a stable textual description associated with the object's current error condition,
- the function returns a string pointer/reference rather than writing into caller memory.

Tests:

- for a known error state, `wordsplit_strerror` returns a non-empty description,
- changing or clearing the error state changes or removes the prior error description as appropriate.

### Scenario 4: Caller prints an error report for diagnostics

A caller has a `wordsplit` object and wants the module to print an error report directly.

Expected behavior:

- the module emits an error message derived from the object state in perror-style usage,
- the operation does not require the caller to manually format the message text.

Tests:

- with an object in error state, `wordsplit_perror` produces output,
- the output includes the error meaning associated with the current state.

### Scenario 5: Caller frees only parameter-buffer resources

A caller wants to release parameter-buffer resources owned by the object while keeping the overall object lifecycle under separate control.

Expected behavior:

- parameter-buffer-associated resources are released,
- repeated use of the parameter buffer after cleanup is not required to remain valid until reinitialized by other module logic.

Tests:

- on an object with parameter-buffer state, `wordsplit_free_parambuf` releases that state without requiring full object destruction,
- invoking the full-object cleanup after parameter-buffer cleanup remains safe if the C behavior permits this sequence.

### Scenario 6: Caller destroys the `wordsplit` object after use

A caller has finished with the object and wants all module-owned resources released.

Expected behavior:

- the module releases resources associated with the `wordsplit` instance, including internally owned dynamic state,
- cleanup includes resources linked through the object, such as internal node-based allocations where applicable.

Tests:

- `wordsplit_free` on an initialized object releases all owned resources without leaving result buffers accessible through the object,
- object cleanup works whether or not parameter-buffer cleanup was invoked earlier,
- object cleanup works after error-state inspection and result retrieval.

## Requirements

### Functional Requirements

#### FR-1: Parameter-buffer cleanup
The Rust module shall provide behavior equivalent to `wordsplit_free_parambuf(struct wordsplit *ws)` for releasing parameter-buffer resources owned by a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:2785-2800`
- `struct wordsplit`

#### FR-2: Error-state clearing
The Rust module shall provide behavior equivalent to `wordsplit_clearerr(struct wordsplit *ws)` for clearing the current error state stored in a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:2802-2813`
- `struct wordsplit`

#### FR-3: Full object cleanup
The Rust module shall provide behavior equivalent to `wordsplit_free(struct wordsplit *ws)` for releasing resources owned by a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:2815-2829`
- `struct wordsplit`
- `struct wordsplit_node`

#### FR-4: Word-result access
The Rust module shall provide behavior equivalent to `wordsplit_get_words(struct wordsplit *ws, size_t *wordc, char ***wordv)` for exposing the current word count and word-vector result stored in a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:2831-2846`
- `struct wordsplit`

#### FR-5: Error-string access
The Rust module shall provide behavior equivalent to `wordsplit_strerror(struct wordsplit *ws)` for returning a textual description of the current error condition of a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:2864-2872`
- `struct wordsplit`

#### FR-6: Printed error reporting
The Rust module shall provide behavior equivalent to `wordsplit_perror(struct wordsplit *wsp)` for emitting an error report derived from the current state of a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:2874-2891`
- `struct wordsplit`

#### FR-7: Consistent post-clear error inspection
After error-state clearing, error inspection functions in this module shall reflect the cleared state rather than the previous one.

Traceability:
- `wordsplit_clearerr`
- `wordsplit_strerror`
- `wordsplit_perror`
- `src/wordsplit/wordsplit.c:2802-2813`
- `src/wordsplit/wordsplit.c:2864-2891`

#### FR-8: Cleanup must cover internally linked allocations owned by the object
Full cleanup shall release object-owned dynamic state reachable through the `wordsplit` object, including node-based internal state evidenced by the presence of `struct wordsplit_node`.

Traceability:
- `wordsplit_free`
- `struct wordsplit`
- `struct wordsplit_node`
- `src/wordsplit/wordsplit.c:416-430`
- `src/wordsplit/wordsplit.c:2815-2829`

### Key Entities

#### `wordsplit`
Primary module state object.

Observed roles supported by this specification:

- stores the module error state,
- stores or references the produced word count and word vector,
- owns parameter-buffer resources,
- owns additional dynamic resources released by full cleanup,
- is the input to all public behaviors covered by this module.

Relationships:

- may own or reference one or more internal `wordsplit_node` structures,
- is the authoritative source for result access and error reporting.

Traceability:
- `struct wordsplit` references throughout `src/wordsplit/wordsplit.c`
- public functions at lines `2785-2891`

#### `wordsplit_node`
Internal node-based allocation linked to a `wordsplit` object.

Observed role supported by this specification:

- participates in internal dynamically allocated state that must be released during full cleanup when owned by the parent `wordsplit`.

Relationships:

- belongs to or is linked from a `wordsplit` instance.

Traceability:
- `src/wordsplit/wordsplit.c:416-430`
- `src/wordsplit/wordsplit.c:469-509`
- `wordsplit_free`

## Success Criteria

1. **Result-access parity**
   - For representative `wordsplit` instances with populated results, the Rust port returns the same word count and word-vector contents as the C module through the module’s result-access behavior.
   - Traceability: `wordsplit_get_words`

2. **Error-clear parity**
   - After invoking the Rust equivalent of `wordsplit_clearerr`, previously observable error text from the same object is no longer reported as the active error.
   - Traceability: `wordsplit_clearerr`, `wordsplit_strerror`, `wordsplit_perror`

3. **Error-text parity**
   - For representative error states supported by the source module, the Rust port returns the same or behaviorally equivalent human-readable error descriptions.
   - Traceability: `wordsplit_strerror`

4. **Printed-error parity**
   - For representative error states, the Rust port emits an error report consistent with the C module’s perror-style behavior.
   - Traceability: `wordsplit_perror`

5. **Partial-cleanup parity**
   - Invoking the Rust equivalent of parameter-buffer cleanup releases that portion of object-owned state without requiring full object destruction.
   - Traceability: `wordsplit_free_parambuf`

6. **Full-cleanup parity**
   - Invoking the Rust equivalent of full object cleanup releases all resources owned by the `wordsplit` instance, including internal node-linked allocations evidenced by the module types.
   - Traceability: `wordsplit_free`, `wordsplit_node`

7. **Scenario support**
   - All user scenarios in this document are implementable and pass tests against the Rust port.
   - Traceability: all functions covered by this specification

## Notes for Porting Boundaries

- Preserve module-visible behavior, especially around cleanup ordering, result retrieval, and error reporting.
- Do not add new public features beyond the behavior evidenced by the source functions and data structures listed in this document.
- Rust ownership and destructor patterns may replace C memory-management mechanics internally, but externally observable module behavior must remain equivalent to the specified functions.