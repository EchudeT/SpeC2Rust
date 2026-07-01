# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_01`

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_01`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Target branch: `112-module_src_wordsplit_wordsplit_01-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides the initialization and early-state management logic for a word-splitting engine centered on a `wordsplit` context. The analyzed functions show responsibility for:

- establishing a clean `wordsplit` state,
- initializing processing for an input string with flags,
- allocating and growing internal storage,
- creating and maintaining internal split nodes,
- performing nested or subsidiary splitting using a subordinate `wordsplit` context,
- classifying name characters during parsing-related decisions,
- and recording, propagating, and contextualizing errors, including out-of-memory conditions.

The Rust rewrite must preserve this functional behavior and module boundary. It must implement the same observable responsibilities for context setup, internal node management, subsplitting support, and error reporting as evidenced by the source module.

## Feature Specification

### Feature: `wordsplit` context initialization

The module must support creation of a clean processing context and initialization of that context for a specific input buffer, input length, and flag set.

Observed behavior indicates two levels of initialization:

- a zero or baseline initializer that clears the `wordsplit` state to defaults,
- and a higher-level initializer that binds the context to an input string and prepares internal storage and parsing state according to supplied flags.

The Rust version must preserve the distinction between baseline reset and full initialization, because later operations such as allocation, node management, and nested splitting depend on a valid initialized context.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`

### Feature: internal storage reservation and growth

The module must be able to reserve or grow internal storage space used by the split process. This includes failure signaling when memory cannot be acquired.

The Rust version must preserve the functional result that a caller can request room for additional internal items and receive success or a module-level error result if capacity cannot be ensured.

Traceability:
- `alloc_space`
- `_wsplt_nomem`
- `_wsplt_alloc_die`

### Feature: word-split node lifecycle management

The module manages internal `wordsplit_node` instances representing split fragments or parse nodes. Supported behavior includes:

- creating a new node,
- appending a node into the active node collection,
- removing a node from that collection,
- and resolving a node back to a character pointer/view into associated text storage.

The Rust version must preserve these operations as module-internal functionality supporting the split engine.

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`
- `wordsplit_node`

### Feature: nested or subsidiary split execution

The module can perform a split operation using a subordinate `wordsplit` context on a specified substring, with its own flags and a finalize control. This allows the parent split process to delegate processing of a subrange while retaining responsibility for final error handling and state integration.

The Rust version must preserve the ability to run such a subsidiary split and return its success or failure to the parent logic.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

### Feature: module error recording and propagation

The module records error state in the `wordsplit` context and supports several error-related behaviors:

- setting an error code on the active context,
- handling out-of-memory as a distinct error path,
- storing a text fragment as error context,
- setting an error together with error-context text,
- and propagating an error from a subordinate split context into the parent context.

The Rust version must preserve these behaviors so that callers and parent logic can distinguish success from failure and inspect context-associated error information.

Traceability:
- `_wsplt_seterr`
- `_wsplt_nomem`
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`
- `_wsplt_seterr_sub`
- `_wsplt_alloc_die`

### Feature: name-character classification support

The module provides a character classification helper used to decide whether a character is valid as a name character under the active `wordsplit` configuration.

The Rust version must preserve equivalent classification behavior as required by this module’s parsing logic.

Traceability:
- `is_name_char`
- `wordsplit`

## User Scenarios & Testing

### Scenario 1: Initialize a split context for new input

A caller creates a `wordsplit` context and initializes it for a given input buffer, explicit input length, and flag set. The operation succeeds and leaves the context ready for later splitting work and internal node activity.

The Rust version must support tests that verify:

- baseline initialization produces a known empty/default state,
- full initialization accepts input and flags,
- the initialized context is accepted by subsequent internal operations.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`

### Scenario 2: Fail initialization or growth when storage cannot be obtained

A caller or internal path requires additional storage. If storage cannot be reserved, the context records an allocation-related failure and returns an error result.

The Rust version must support tests that verify:

- failed storage growth returns failure,
- the context error state reflects allocation failure,
- any context error information expected for allocation failure is present.

Traceability:
- `alloc_space`
- `_wsplt_nomem`
- `_wsplt_alloc_die`
- `_wsplt_seterr`

### Scenario 3: Create, append, inspect, and remove internal nodes

During split preparation or parsing, the module creates a new node, links it into the active node sequence, resolves its text position/view, and later removes it.

The Rust version must support tests that verify:

- node creation succeeds when capacity is available,
- appending updates the active collection membership,
- pointer/view resolution returns the expected text location for the node,
- removing a node detaches it cleanly from the active collection.

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_ptr`
- `wsnode_remove`

### Scenario 4: Perform a subordinate split on a substring

The parent split logic delegates processing of a substring to a subordinate `wordsplit` context, with explicit flags and a finalize mode. On success, the subordinate operation returns normally. On failure, parent-visible error state is updated from the subordinate context.

The Rust version must support tests that verify:

- a subordinate split can be invoked with substring bounds and flags,
- success is returned when the subordinate operation succeeds,
- parent error state is updated when the subordinate operation fails.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

### Scenario 5: Record contextual error text

When an error occurs at or near a particular input fragment, the module stores the relevant text context and associates it with the error state.

The Rust version must support tests that verify:

- error code setting works independently,
- error code plus context text can be set together,
- copied/stored error context matches the supplied fragment length and content,
- subordinate errors can transfer their context into the parent-visible error state.

Traceability:
- `_wsplt_seterr`
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`
- `_wsplt_seterr_sub`

### Scenario 6: Use name-character classification during parsing decisions

Parsing logic asks whether a character qualifies as a name character under the active context configuration.

The Rust version must support tests that verify:

- classification returns consistent results for representative accepted and rejected characters,
- behavior is tied to the active `wordsplit` context rather than a completely context-free rule.

Traceability:
- `is_name_char`

## Requirements

### Functional Requirements

#### FR-1: Context baseline initialization

The module shall provide a way to reset or initialize a `wordsplit` context to its baseline default state before full processing begins.

Traceability:
- `wordsplit_init0`

#### FR-2: Full context initialization from input

The module shall initialize a `wordsplit` context from an input character buffer, explicit input length, and flag set, returning success or failure.

Traceability:
- `wordsplit_init`

#### FR-3: Internal space allocation

The module shall reserve or expand internal processing space for a requested count of items and signal failure when the request cannot be satisfied.

Traceability:
- `alloc_space`

#### FR-4: Allocation failure handling

When an allocation-related operation fails, the module shall convert that failure into the module’s error state for the active `wordsplit` context.

Traceability:
- `_wsplt_nomem`
- `_wsplt_alloc_die`
- `_wsplt_seterr`

#### FR-5: Error code storage

The module shall store an error code in the `wordsplit` context and return an error result suitable for caller propagation.

Traceability:
- `_wsplt_seterr`

#### FR-6: Error context storage

The module shall be able to store a text fragment and fragment length as contextual information associated with an error in the `wordsplit` context.

Traceability:
- `_wsplt_store_errctx`

#### FR-7: Combined error-and-context reporting

The module shall support setting an error code together with associated error-context text in one operation.

Traceability:
- `_wsplt_setctxerr`

#### FR-8: Subordinate split execution

The module shall support invoking a subordinate split operation using a secondary `wordsplit` context over a specified string region, with caller-supplied flags and finalize behavior.

Traceability:
- `_wsplt_subsplit`

#### FR-9: Subordinate error propagation

When a subordinate split fails, the module shall propagate the subordinate error state and relevant context into the parent `wordsplit` context.

Traceability:
- `_wsplt_seterr_sub`

#### FR-10: Node creation

The module shall support creation of a new internal `wordsplit_node` associated with the active `wordsplit` context.

Traceability:
- `wsnode_new`

#### FR-11: Node collection insertion

The module shall support appending an internal node into the active node collection maintained by the `wordsplit` context.

Traceability:
- `wsnode_append`

#### FR-12: Node collection removal

The module shall support removing an internal node from the active node collection maintained by the `wordsplit` context.

Traceability:
- `wsnode_remove`

#### FR-13: Node text resolution

The module shall support resolving a `wordsplit_node` to its corresponding character pointer or text view within the owning context’s underlying text storage.

Traceability:
- `wsnode_ptr`

#### FR-14: Name-character classification

The module shall support classifying whether a character is a valid name character under the active `wordsplit` context.

Traceability:
- `is_name_char`

### Key Entities

#### `wordsplit`

The primary processing context for this module. It owns or references:

- the input text being processed,
- flags controlling split behavior,
- error state and error-context text,
- internal storage used during processing,
- and the active collection of internal nodes.

It is the central entity passed to all major module operations.

Traceability:
- `wordsplit`
- `wordsplit_init0`
- `wordsplit_init`
- `alloc_space`
- `_wsplt_seterr`
- `_wsplt_store_errctx`
- `_wsplt_subsplit`

#### `wordsplit_node`

An internal node representing a split-related item associated with a `wordsplit` context. Nodes can be created, linked into the context-managed node collection, removed, and resolved back to a location in text storage.

Traceability:
- `wordsplit_node`
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`

#### Parent/subordinate `wordsplit` relationship

The module supports a relationship in which one `wordsplit` context acts as a parent controller and another acts as a subordinate context for nested splitting of a substring. Error state can move from subordinate to parent when nested processing fails.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

## Success Criteria

1. A Rust `wordsplit` context can be baseline-initialized and then fully initialized from input, length, and flags, with success/failure status preserved.
   - Traceability: `wordsplit_init0`, `wordsplit_init`

2. Requests for internal space growth succeed when capacity is available and produce module error state when capacity acquisition fails.
   - Traceability: `alloc_space`, `_wsplt_nomem`, `_wsplt_alloc_die`

3. The Rust module records error codes and, when provided, stores associated error-context text accurately by content and length.
   - Traceability: `_wsplt_seterr`, `_wsplt_store_errctx`, `_wsplt_setctxerr`

4. A subordinate split can be executed over a caller-specified substring with flags and finalize control, and failure in the subordinate context is reflected in the parent context.
   - Traceability: `_wsplt_subsplit`, `_wsplt_seterr_sub`

5. Internal node lifecycle operations support creation, append, removal, and text-resolution behavior consistent with the owning `wordsplit` context.
   - Traceability: `wsnode_new`, `wsnode_append`, `wsnode_remove`, `wsnode_ptr`

6. Name-character classification produces stable accept/reject results for characters under the active context configuration.
   - Traceability: `is_name_char`

7. All functionality specified above is implemented without introducing unevidenced capabilities beyond this module’s observed boundary.
   - Traceability: all listed functions and types in `src/wordsplit/wordsplit.c`