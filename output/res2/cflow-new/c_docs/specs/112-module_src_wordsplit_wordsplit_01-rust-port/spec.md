# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_01`

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_01`
- Category: `module_cluster`
- Source basis: `src/wordsplit/wordsplit.c`
- Target branch: `112-module_src_wordsplit_wordsplit_01-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides foundational state management for a word-splitting engine. Within the analyzed source range, its responsibilities are limited to:

- initializing and resetting `wordsplit` processing state,
- validating name characters under module-controlled rules,
- allocating and growing internal storage,
- creating, linking, and removing internal parse nodes,
- invoking nested or subordinate word-splitting work on a substring,
- propagating and contextualizing errors, including out-of-memory handling.

The Rust rewrite must preserve these observed behaviors and boundaries. It must not add unrelated capabilities beyond the evidenced responsibilities in `src/wordsplit/wordsplit.c`.

## Feature Specification

### 1. Processing State Initialization

The module must support initialization of a `wordsplit` state object into a known baseline state and preparation of that state for processing an input string plus flags.

Observed behaviors indicate two levels of setup:

- a zero/base initialization step that clears or normalizes internal state,
- a full initialization step that prepares the state for a specific input buffer, input length, and flag set.

The Rust version must implement both behaviors as module functionality, whether exposed directly or composed internally.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`

### 2. Character Classification for Name Parsing

The module must classify whether a character is acceptable as a “name character” under the active `wordsplit` state.

This classification is state-dependent rather than globally fixed, because the classifier receives the current `wordsplit` object.

Traceability:
- `is_name_char`

### 3. Internal Storage Reservation and Growth

The module must reserve or expand internal storage used during splitting and node management. When growth cannot be satisfied, the module must transition into a defined allocation-failure error path.

The Rust rewrite must preserve:

- request-driven allocation for a specified amount of additional space,
- failure signaling through module error state,
- dedicated no-memory handling behavior.

Traceability:
- `alloc_space`
- `_wsplt_alloc_die`
- `_wsplt_nomem`

### 4. Error Recording and Context Capture

The module must support recording an error code in the active `wordsplit` state and, when applicable, storing a slice of source text as error context.

The Rust version must preserve these functional distinctions:

- setting an error code,
- setting a no-memory error through the dedicated path,
- storing an error context substring,
- setting an error together with context in one operation,
- copying subordinate split errors into the parent split state.

Traceability:
- `_wsplt_seterr`
- `_wsplt_nomem`
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`
- `_wsplt_seterr_sub`

### 5. Nested/Subordinate Splitting

The module must be able to run a subordinate split operation against a substring using a secondary `wordsplit` state, under caller-provided flags and a finalize control.

The Rust rewrite must preserve the functional role of this operation:

- receive parent and child split states,
- operate on a specific source substring and length,
- honor subordinate flags,
- support a finalize/non-finalize mode,
- propagate subordinate errors back to the parent state.

This specification does not require exposing subordinate splitting as a public API if it is internal in the Rust design, but the behavior must exist to support equivalent module operation.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

### 6. Internal Parse Node Lifecycle Management

The module must support creation and list management of internal `wordsplit_node` records associated with the active `wordsplit` state.

Required lifecycle behaviors evidenced by the source:

- compute the source pointer represented by a node,
- allocate a new node,
- append a node to the module-managed node sequence,
- remove a node from that sequence.

The Rust rewrite must preserve correct association between nodes and the owning split state.

Traceability:
- `wsnode_ptr`
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`

## User Scenarios & Testing

### Scenario 1: Initialize a split state for new input

A caller prepares a fresh `wordsplit` state and initializes it for a given input string and flag set before any parsing work begins.

Expected behavior:

- the state starts from a known baseline,
- the state becomes associated with the supplied input and length,
- initialization reports success or an error status,
- allocation failure during setup is recorded through the module’s error path.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`
- `_wsplt_seterr`
- `_wsplt_nomem`

Suggested tests:

- initialize with a valid non-empty input,
- initialize with empty input,
- initialize repeatedly on distinct states and verify deterministic baseline state,
- force allocation failure and verify error state is set.

### Scenario 2: Reject invalid allocation growth

During processing, the module needs more internal storage and requests additional capacity.

Expected behavior:

- successful requests increase usable internal capacity,
- failed requests do not silently continue,
- failure is surfaced through the module’s error-setting path,
- the owning `wordsplit` state reflects the failure.

Traceability:
- `alloc_space`
- `_wsplt_alloc_die`
- `_wsplt_nomem`

Suggested tests:

- request a small additional allocation and verify success,
- request multiple growth steps in sequence,
- inject allocator failure and verify no-memory error handling is used.

### Scenario 3: Run subordinate splitting on a substring

A parent split operation needs a substring to be split using a secondary `wordsplit` state.

Expected behavior:

- the child split operates only on the provided substring and length,
- caller-supplied flags are honored for the child operation,
- finalize mode affects whether the child split is completed in that call,
- child errors can be copied back into the parent state with preserved context/error meaning.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

Suggested tests:

- subordinate split on a valid substring,
- subordinate split with finalize enabled and disabled,
- child split failure propagating to parent state,
- child error context copied into parent state.

### Scenario 4: Store contextual error information

When an error is tied to a specific region of source text, the module captures that substring as context.

Expected behavior:

- the module can store a source fragment and its length as error context,
- setting an error with context updates both the error code and stored context,
- later error propagation from a child state can preserve meaningful context in the parent.

Traceability:
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`
- `_wsplt_seterr_sub`

Suggested tests:

- set an error with a short context slice,
- set an error with zero-length context if allowed by input path,
- verify parent state receives child context after propagated failure.

### Scenario 5: Manage internal nodes during parse construction

While building or adjusting parse results, the module creates nodes, appends them to the maintained sequence, queries their source position, and may remove nodes.

Expected behavior:

- a new node can be allocated and associated with the current split state,
- appended nodes become part of the maintained node sequence,
- removing a node detaches it cleanly from that sequence,
- querying a node pointer yields the corresponding source position within the owning input.

Traceability:
- `wsnode_ptr`
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`

Suggested tests:

- create and append a single node,
- append multiple nodes and verify sequence order,
- remove head, middle, and tail nodes,
- verify pointer lookup corresponds to expected input location.

### Scenario 6: Classify name characters under current split rules

During parsing, the module checks whether characters belong to a valid name.

Expected behavior:

- classification is available from the active split state,
- allowed and disallowed characters produce stable results for the same state.

Traceability:
- `is_name_char`

Suggested tests:

- verify alphabetic characters,
- verify digits and underscore if supported by observed rules,
- verify punctuation and whitespace are rejected when not valid under the current state.

## Requirements

### Functional Requirements

#### FR-1: Base State Reset
The module shall provide functionality to place a `wordsplit` state into a defined baseline state before use.

Traceability:
- `wordsplit_init0`

#### FR-2: Input-Bound Initialization
The module shall initialize a `wordsplit` state for a supplied input string, explicit input length, and flag set, returning success or failure status.

Traceability:
- `wordsplit_init`

#### FR-3: State-Dependent Name Character Classification
The module shall determine whether a character is a valid name character using the active `wordsplit` state.

Traceability:
- `is_name_char`

#### FR-4: Internal Capacity Allocation
The module shall allocate or extend internal working space for the active `wordsplit` state based on a requested count.

Traceability:
- `alloc_space`

#### FR-5: Allocation Failure Handling
The module shall provide a dedicated allocation-failure path that marks the `wordsplit` state with an appropriate failure result.

Traceability:
- `_wsplt_alloc_die`
- `_wsplt_nomem`
- `_wsplt_seterr`

#### FR-6: Error Code Storage
The module shall record an error code in the active `wordsplit` state and return a failure indicator suitable for caller propagation.

Traceability:
- `_wsplt_seterr`

#### FR-7: Error Context Storage
The module shall store source-text context for an error using a string pointer and explicit length.

Traceability:
- `_wsplt_store_errctx`

#### FR-8: Combined Error-and-Context Update
The module shall support setting an error code and its associated source context as one logical operation.

Traceability:
- `_wsplt_setctxerr`

#### FR-9: Subordinate Split Execution
The module shall execute a subordinate split using a child `wordsplit` state over a supplied substring, substring length, flags, and finalize control.

Traceability:
- `_wsplt_subsplit`

#### FR-10: Subordinate Error Propagation
The module shall copy or translate error state from a child `wordsplit` state into a parent `wordsplit` state after subordinate split failure.

Traceability:
- `_wsplt_seterr_sub`

#### FR-11: Node Allocation
The module shall allocate a new internal `wordsplit_node` for the active `wordsplit` state and report success or failure.

Traceability:
- `wsnode_new`

#### FR-12: Node Sequence Append
The module shall append an internal node to the node sequence maintained by the active `wordsplit` state.

Traceability:
- `wsnode_append`

#### FR-13: Node Sequence Removal
The module shall remove an internal node from the node sequence maintained by the active `wordsplit` state.

Traceability:
- `wsnode_remove`

#### FR-14: Node-to-Source Resolution
The module shall resolve a `wordsplit_node` to the corresponding source-text pointer within the owning `wordsplit` input.

Traceability:
- `wsnode_ptr`

### Key Entities

#### `wordsplit`
Core processing state for a single word-splitting operation.

Observed roles:

- owns the active input and associated length information used during initialization and node pointer resolution,
- stores status/error information,
- stores contextual error text,
- owns internal working storage,
- owns or anchors the internal node sequence,
- serves as parent or child state during subordinate splitting.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`
- `_wsplt_seterr`
- `_wsplt_store_errctx`
- `_wsplt_subsplit`
- `alloc_space`
- `wsnode_ptr`
- `wsnode_append`
- `wsnode_remove`

#### `wordsplit_node`
Internal node used to represent parse or token construction state associated with a `wordsplit`.

Observed roles:

- can be allocated,
- can be linked into or removed from a node sequence,
- can be mapped back to a source-text position through the owning `wordsplit`.

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`

#### Parent/Child `wordsplit` Relationship
A `wordsplit` instance may invoke processing through a secondary `wordsplit` instance over a substring and later absorb that child’s error state.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

## Success Criteria

1. A Rust `wordsplit` state can be brought to a deterministic baseline state and then initialized for specific input, length, and flags with explicit success/failure reporting.
   - Traceability: `wordsplit_init0`, `wordsplit_init`

2. Character classification for name parsing produces stable results from the active split state and is callable wherever parsing requires it.
   - Traceability: `is_name_char`

3. Internal storage growth requests succeed when capacity is available and produce defined module error state when allocation fails.
   - Traceability: `alloc_space`, `_wsplt_alloc_die`, `_wsplt_nomem`

4. Error codes can be recorded independently, and source-context substrings can be stored independently or together with an error code.
   - Traceability: `_wsplt_seterr`, `_wsplt_store_errctx`, `_wsplt_setctxerr`

5. A subordinate split can be executed over a caller-specified substring using a child state, with child failure propagated back into the parent state.
   - Traceability: `_wsplt_subsplit`, `_wsplt_seterr_sub`

6. Internal nodes can be allocated, appended, removed, and resolved back to source positions without breaking ownership association with the active split state.
   - Traceability: `wsnode_new`, `wsnode_append`, `wsnode_remove`, `wsnode_ptr`

7. Test coverage includes the usage scenarios in this document and demonstrates parity of the Rust module’s observed functional behavior with the analyzed C module boundaries.
   - Traceability: all functions listed in this specification