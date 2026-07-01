# spec.md

## Title

Functional Specification: `module_gnu_gl_msvc_inval_per_thread_07`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_gl_msvc_inval_per_thread_07`
- Category: `module_cluster`
- Source file: `gnu/msvc-inval.c`
- Rust branch target: `013-module_gnu_gl_msvc_inval_per_thread_07-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides per-thread state used when handling MSVC invalid-parameter events. Its observable role is:

- to expose access to the current thread’s invalid-parameter state object, and
- to let the module’s invalid-parameter handler record that an invalid-parameter event occurred for the current thread.

The Rust rewrite must preserve this behavior at the functional level reflected by:

- `gl_msvc_inval_current`
- `gl_msvc_invalid_parameter_handler`
- `struct gl_msvc_inval_per_thread`

This specification is limited to functionality evidenced by `gnu/msvc-inval.c`.

## Feature Specification

### Feature Summary

The module maintains a thread-associated invalid-parameter tracking object and updates that object when the module’s invalid-parameter handler is invoked.

### Functional Behavior

1. **Current per-thread state access**
   - The module shall provide a way to obtain the current thread’s `gl_msvc_inval_per_thread` state object.
   - Repeated access within the same thread shall refer to that thread’s current state object.
   - Access from different threads shall resolve to thread-associated state, not a single process-global state object.

2. **Lazy availability of thread state**
   - The current thread state shall be obtainable on demand when requested through the module’s state-access function.
   - If the current thread has no existing state object yet, the module shall make one available before returning success from the access function.

3. **Invalid-parameter event marking**
   - When `gl_msvc_invalid_parameter_handler` is called, it shall update the current thread’s `gl_msvc_inval_per_thread` state to reflect that an invalid-parameter event occurred.
   - The handler’s observable effect is per-thread event marking; the input diagnostic parameters (`expression`, `function`, `file`, `line`, `dummy`) are not evidenced as producing additional required behavior in this module.

4. **No required diagnostic payload handling**
   - The Rust rewrite is not required to preserve or expose the diagnostic string or line inputs from the handler, because no such behavior is evidenced in the analyzed module results.

## User Scenarios & Testing

### Scenario 1: Retrieve current thread tracking state

A caller needs access to the current thread’s MSVC invalid-parameter tracking state.

**Expected behavior**
- Calling the current-state access function returns a usable per-thread state object reference or pointer.
- A second call from the same thread returns access to the same thread’s state context rather than a different thread’s context.

**Test focus**
- Same-thread repeated access resolves consistently to the same logical per-thread state.

### Scenario 2: First use on a thread

A thread accesses invalid-parameter tracking state before any prior state exists for that thread.

**Expected behavior**
- The access function makes the thread state available and returns it.
- After this first access, the thread has a current state object that the handler can update.

**Test focus**
- Initial access succeeds in establishing per-thread state.

### Scenario 3: Handler marks invalid-parameter occurrence

An MSVC invalid-parameter condition causes the module handler to be invoked.

**Expected behavior**
- The handler marks the current thread’s invalid-parameter state as having observed an invalid-parameter event.

**Test focus**
- State before handler call indicates no recorded event.
- State after handler call indicates an event was recorded for that same thread.

### Scenario 4: Per-thread isolation

Two threads each interact with the module.

**Expected behavior**
- Each thread obtains its own current state object.
- An invalid-parameter event recorded in one thread affects that thread’s state and does not by itself mark the other thread’s state.

**Test focus**
- Thread A handler activity changes thread A state.
- Thread B state remains independently tracked.

## Requirements

### Functional Requirements

#### FR-1: Provide current-thread invalid-parameter state access
The module shall provide functionality equivalent to `gl_msvc_inval_current` for obtaining the current thread’s `gl_msvc_inval_per_thread` state.

**Traceability:** `gnu/msvc-inval.c:69-97`, `gl_msvc_inval_current`

#### FR-2: Ensure state is associated with the calling thread
The state returned by current-state access shall be associated with the calling thread rather than treated as shared state for all callers.

**Traceability:** `gnu/msvc-inval.c:69-97`, `gl_msvc_inval_current`; `struct gl_msvc_inval_per_thread`

#### FR-3: Make thread state available on first access
If no current `gl_msvc_inval_per_thread` state is yet available for the calling thread, the module shall create or otherwise establish one before returning successful access.

**Traceability:** `gnu/msvc-inval.c:69-97`, `gl_msvc_inval_current`

#### FR-4: Record invalid-parameter occurrence in current thread state
The module shall provide invalid-parameter handler behavior equivalent to `gl_msvc_invalid_parameter_handler`, updating the current thread’s state to indicate that an invalid-parameter event occurred.

**Traceability:** `gnu/msvc-inval.c:33-40`, `50-58`, `99-113`, `gl_msvc_invalid_parameter_handler`; `struct gl_msvc_inval_per_thread`

#### FR-5: Ignore handler diagnostic inputs for required external behavior
The Rust version shall not require externally visible behavior based on the handler’s diagnostic input parameters beyond recording the invalid-parameter occurrence, because no additional required behavior is evidenced.

**Traceability:** `gnu/msvc-inval.c:33-40`, `50-58`, `99-113`, `gl_msvc_invalid_parameter_handler`

### Key Entities

#### `gl_msvc_inval_per_thread`
The per-thread state entity representing invalid-parameter tracking for one thread.

**Role**
- Holds the thread-local status that is returned by current-state access.
- Is the state object modified by the invalid-parameter handler.

**Traceability:** `gnu/msvc-inval.c:67-89`, `99-113`

#### Current-thread state relationship
There is a one-thread-to-one-current-state relationship at the point of access: the module resolves operations against the calling thread’s `gl_msvc_inval_per_thread` object.

**Traceability:** `gnu/msvc-inval.c:69-97`, `99-113`

#### Invalid-parameter handler to thread-state relationship
The invalid-parameter handler does not operate on an arbitrary passed-in state object; instead, it resolves the current thread state and marks it.

**Traceability:** `gnu/msvc-inval.c:99-113`; also reflected by handler presence at `33-40` and `50-58`

## Success Criteria

1. **Current-state availability**
   - A test invoking the Rust equivalent of `gl_msvc_inval_current` from a thread with no prior state receives successful access to a `gl_msvc_inval_per_thread` state object.

   **Traceability:** FR-1, FR-3

2. **Same-thread consistency**
   - A test making repeated current-state access calls from one thread observes the same logical thread-associated state across calls.

   **Traceability:** FR-1, FR-2

3. **Handler state marking**
   - A test that inspects current thread state before and after invoking the Rust equivalent of `gl_msvc_invalid_parameter_handler` observes that the post-call state is marked as having seen an invalid-parameter event.

   **Traceability:** FR-4

4. **Per-thread isolation**
   - A test using at least two threads shows that recording an invalid-parameter event in one thread changes only that thread’s state and not the other thread’s state.

   **Traceability:** FR-2, FR-4

5. **No dependency on diagnostic argument content**
   - A test invoking the handler with differing diagnostic argument values observes the same required functional outcome: current-thread invalid-parameter occurrence is recorded.

   **Traceability:** FR-5