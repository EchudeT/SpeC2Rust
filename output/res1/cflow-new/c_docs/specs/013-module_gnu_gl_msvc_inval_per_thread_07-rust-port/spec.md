# Specification: module_gnu_gl_msvc_inval_per_thread_07

- **Project**: `cflow-new`
- **Module**: `module_gnu_gl_msvc_inval_per_thread_07`
- **Category**: `module_cluster`
- **Source basis**: `gnu/msvc-inval.c`
- **Rust port branch**: `013-module_gnu_gl_msvc_inval_per_thread_07-rust-port`
- **Generation date**: `2026-06-11`

## 1. Feature Specification

This module provides per-thread state access for handling Microsoft C runtime invalid-parameter events.

The analyzed source shows two functional elements:

1. A per-thread state object named `gl_msvc_inval_per_thread`.
2. Logic that retrieves the current thread's instance of that state through `gl_msvc_inval_current()`.
3. An internal invalid-parameter handler named `gl_msvc_invalid_parameter_handler(...)` that operates on the current thread's state.

From the available evidence, the Rust rewrite must preserve the module's functional boundary as follows:

- Maintain a per-thread invalid-parameter tracking/state object corresponding to `gl_msvc_inval_per_thread`.
- Provide functionality equivalent to `gl_msvc_inval_current()` that returns access to the current thread's module state.
- Support use of that state by the module's invalid-parameter handler so that handler activity is associated with the calling thread's own state rather than shared process-global state.

No additional externally visible capabilities are evidenced by the input and therefore none are required by this specification.

## 2. User Scenarios & Testing

### Scenario 1: First access on a thread
A caller in one thread needs the module's invalid-parameter state for that thread.

**Expected behavior**
- Accessing the current-thread state returns a valid `gl_msvc_inval_per_thread` instance for that thread.
- The state is usable by the module's invalid-parameter handling logic.

**Testing focus**
- Call the Rust equivalent of `gl_msvc_inval_current()`.
- Verify that a non-null/non-absent per-thread state is returned.

### Scenario 2: Repeated access on the same thread
A caller accesses the current-thread invalid-parameter state more than once on the same thread.

**Expected behavior**
- Each access refers to the same logical per-thread state for that thread.
- Repeated retrieval does not switch to another thread's state.

**Testing focus**
- Retrieve the state multiple times on one thread.
- Verify identity or persistent logical association for that thread.

### Scenario 3: Independent access from different threads
Separate threads each trigger retrieval of the module state.

**Expected behavior**
- Each thread receives its own per-thread state object.
- One thread's handler-related state does not alias another thread's state.

**Testing focus**
- Retrieve state from at least two threads.
- Verify that the thread-local instances are distinct.

### Scenario 4: Invalid-parameter handler uses current-thread state
The module's internal invalid-parameter handler is invoked in a thread where per-thread state exists.

**Expected behavior**
- The handler consults and/or updates the state associated with the current thread.
- The handler behavior is tied to the same per-thread object returned by current-state retrieval.

**Testing focus**
- Exercise the handler path in a controlled way supported by the Rust port.
- Verify that the affected state belongs to the invoking thread.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Per-thread invalid-parameter state
The Rust module shall represent the per-thread invalid-parameter state corresponding to `struct gl_msvc_inval_per_thread` from `gnu/msvc-inval.c`.

**Traceability**
- Type references: `gl_msvc_inval_per_thread`
- Source basis: `gnu/msvc-inval.c`

#### FR-2: Current-thread state retrieval
The Rust module shall implement functionality equivalent to `gl_msvc_inval_current(void)` that retrieves the invalid-parameter state for the calling thread.

**Traceability**
- Function: `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)

#### FR-3: Stable logical association within a thread
For repeated retrievals on the same thread, the Rust module shall preserve a stable logical association with that thread's `gl_msvc_inval_per_thread` state.

**Traceability**
- Function: `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)
- Type references: `gl_msvc_inval_per_thread`

#### FR-4: Separation between threads
The Rust module shall preserve separation of `gl_msvc_inval_per_thread` state between different threads.

**Traceability**
- Function: `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)
- Type references: `gl_msvc_inval_per_thread`

#### FR-5: Handler operates on current-thread state
The Rust module shall preserve the relationship between the internal invalid-parameter handler and the calling thread's `gl_msvc_inval_per_thread` state.

**Traceability**
- Function: `gl_msvc_invalid_parameter_handler` (`gnu/msvc-inval.c:33-40`, `50-58`, `99-113`)
- Function: `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)
- Type references: `gl_msvc_inval_per_thread`

### 3.2 Key Entities

#### `gl_msvc_inval_per_thread`
The module's core state entity. It represents invalid-parameter-related state scoped to one thread.

**Relationships**
- Retrieved by the module's current-thread access function.
- Used by the module's internal invalid-parameter handler.
- One logical instance is associated with each participating thread.

#### Current-thread state access
The behavior represented by `gl_msvc_inval_current()`.

**Relationships**
- Produces access to the current thread's `gl_msvc_inval_per_thread`.
- Serves as the link between thread execution context and handler state.

#### Invalid-parameter handler
The behavior represented by `gl_msvc_invalid_parameter_handler(...)`.

**Relationships**
- Uses the current thread's `gl_msvc_inval_per_thread`.
- Must not be specified as an independent feature beyond its state-linked handling role, because no broader behavior is evidenced in the input.

## 4. Success Criteria

### SC-1: Current-thread state is obtainable
The Rust port provides a working equivalent of `gl_msvc_inval_current()` such that a caller can obtain the current thread's invalid-parameter state during normal module use.

**Traceability**
- `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)

### SC-2: Same-thread retrieval remains consistent
In tests that retrieve current-thread state multiple times on one thread, the results refer to the same logical per-thread state.

**Traceability**
- `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)
- `gl_msvc_inval_per_thread`

### SC-3: Cross-thread state is distinct
In tests using at least two threads, state retrieved on one thread is distinct from state retrieved on another thread.

**Traceability**
- `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)
- `gl_msvc_inval_per_thread`

### SC-4: Handler-state association is preserved
In tests that exercise the module's invalid-parameter handler path, the handler uses the state associated with the invoking thread.

**Traceability**
- `gl_msvc_invalid_parameter_handler` (`gnu/msvc-inval.c:33-40`, `50-58`, `99-113`)
- `gl_msvc_inval_current` (`gnu/msvc-inval.c:69-97`)

### SC-5: No unevidenced functionality is introduced
The Rust port limits itself to the evidenced module boundary: per-thread invalid-parameter state retrieval and handler association. It does not require additional capabilities not supported by the analyzed source.

**Traceability**
- Entire analyzed module boundary in `gnu/msvc-inval.c`