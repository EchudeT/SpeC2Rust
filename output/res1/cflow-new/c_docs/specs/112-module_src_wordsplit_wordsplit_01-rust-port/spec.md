# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_01`

## Summary

This module provides core state management, error handling, subordinate split invocation, storage allocation, and internal node-list operations for the `wordsplit` subsystem implemented in `src/wordsplit/wordsplit.c`.

The Rust rewrite must preserve the functional behavior evidenced by the analyzed functions:

- character classification for name parsing
- initialization of a `wordsplit` state object
- allocation growth for result storage
- error code and error-context recording
- out-of-memory handling with fatal/non-fatal behavior
- creation of subordinate split contexts and propagation of subordinate errors
- creation, append, removal, and pointer resolution for internal split nodes

The specification is limited to behavior evidenced by this module analysis and does not define new external capabilities.

## Scope

### In Scope

The Rust module must implement the following functional areas evidenced in the source module:

- initialization of a split state to defaults
- initialization of a split state from input text, length, and flags
- classification of valid name characters with dependence on split state
- allocation or growth of internal storage for split results
- setting and storing error codes and optional error context
- handling allocation failure according to module policy
- invoking a subordinate split operation using a nested `wordsplit` state
- transferring subordinate error state into the parent state
- managing internal `wordsplit_node` instances in a linked structure
- resolving a node to the corresponding character position in the tracked input/storage domain

### Out of Scope

The following are not specified here because they are not evidenced as module responsibilities in the provided analysis:

- full shell-like word splitting semantics beyond the behaviors directly implied by analyzed functions
- new public APIs beyond those needed to preserve this module’s role
- thread-safety guarantees
- persistence, serialization, or deserialization
- foreign-function interfaces
- performance guarantees beyond correct functional behavior

## Feature Specification

### Feature 1: `wordsplit` State Initialization

The module must support two levels of state initialization:

1. a zero/default initialization step that places a `wordsplit` instance into a known empty state
2. a configured initialization step that accepts input text, input length, and flags and prepares the instance for later splitting work

The configured initialization must establish a consistent state for later storage allocation, node handling, and error reporting. If initialization cannot complete because of memory exhaustion or another initialization error path evidenced by the module, the state must reflect the corresponding error.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`

### Feature 2: Name Character Classification

The module must provide internal character classification for whether a character is valid as a name character, using the active `wordsplit` state as part of the decision context.

The Rust rewrite must preserve the same role: a state-sensitive predicate used by the module’s parsing logic.

Traceability:
- `is_name_char`

### Feature 3: Error Recording and Error Context

The module must record an error code in the active `wordsplit` state and support attaching contextual source text associated with an error.

The Rust rewrite must preserve these behaviors:

- set a primary error code on the split state
- record contextual text and length when provided
- support a convenience path that both stores context and sets the error
- support a dedicated out-of-memory path
- support propagation of error state from a subordinate split into a parent split

Error propagation from a subordinate split must preserve the subordinate failure in the parent context rather than silently discarding it.

Traceability:
- `_wsplt_seterr`
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`
- `_wsplt_nomem`
- `_wsplt_seterr_sub`

### Feature 4: Allocation Failure Policy

The module must distinguish between ordinary out-of-memory reporting and fatal allocation handling. When the module’s fatal allocation path is selected by the active state/policy, allocation failure must terminate processing in the same semantic sense as the C module’s `_wsplt_alloc_die`; otherwise, the failure must be reflected as module error state.

The Rust rewrite must preserve this policy distinction without inventing new recovery semantics.

Traceability:
- `_wsplt_alloc_die`
- `_wsplt_nomem`

### Feature 5: Subordinate Split Execution

The module must support creating and using a subordinate `wordsplit` context to process a substring or related input segment under supplied flags, with optional finalization control.

The subordinate split facility must:

- accept a parent state and a subordinate state
- operate on a provided string segment and length
- accept flags controlling the subordinate operation
- accept a finalize control
- return status indicating success or failure
- allow subordinate error state to be transferred to the parent state

This behavior is part of hierarchical or nested split processing.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

### Feature 6: Storage Space Management

The module must provide internal allocation/growth of storage sufficient to hold a requested count of split-related elements or entries.

The Rust rewrite must preserve the behavior that requesting additional capacity either succeeds and makes the storage available or fails with the module’s normal error handling.

Traceability:
- `alloc_space`

### Feature 7: Internal Node Lifecycle Management

The module must manage internal `wordsplit_node` objects representing portions or structural units of split processing.

The Rust rewrite must preserve these node lifecycle operations:

- create a new node
- append a node into the owning `wordsplit` node sequence
- remove a node from that sequence
- resolve a node to the corresponding character pointer/location represented by the node

The relationships among nodes and the owning `wordsplit` state must remain consistent across append and removal operations.

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`

## User Scenarios & Testing

### Scenario 1: Initialize a Fresh Split State

A caller creates a fresh `wordsplit` state for a new input buffer. The module initializes the state to defaults, then initializes it with the provided input, input length, and flags. After success, later module operations can rely on a consistent starting state.

The Rust version must support tests that verify:

- default initialization produces an empty/non-error baseline state
- configured initialization accepts input and length
- configured initialization stores enough state for later node and allocation operations

Traceability:
- `wordsplit_init0`
- `wordsplit_init`

### Scenario 2: Allocation Failure During Setup or Growth

A split operation requires internal storage expansion. If expansion succeeds, processing can continue. If expansion fails, the module records an out-of-memory condition; if fatal allocation policy applies, processing terminates according to module policy.

The Rust version must support tests that verify:

- storage requests succeed when memory is available
- storage requests report the module’s memory error on failure
- fatal allocation policy follows the fatal path rather than ordinary continuation

Traceability:
- `alloc_space`
- `_wsplt_nomem`
- `_wsplt_alloc_die`

### Scenario 3: Error With Source Context

During processing, the module detects an error associated with a particular source fragment. It stores the error code and remembers the relevant text context and length so later consumers of the state can inspect or report it.

The Rust version must support tests that verify:

- setting an error code updates the state
- storing error context preserves the intended substring boundaries
- combined context-plus-error setting updates both consistently

Traceability:
- `_wsplt_seterr`
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`

### Scenario 4: Nested/Subordinate Split Fails

A parent split operation delegates part of its work to a subordinate split context over a substring. The subordinate operation fails. The parent then adopts the subordinate error state so the failure is visible at the parent level.

The Rust version must support tests that verify:

- subordinate split can be invoked with input segment, length, flags, and finalize control
- subordinate failure is observable through the parent after propagation
- propagated error information is not silently lost

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

### Scenario 5: Internal Node List Maintenance

During internal processing, the module creates nodes, appends them to the active `wordsplit` structure, resolves a node back to the represented text position, and removes nodes when no longer needed.

The Rust version must support tests that verify:

- a new node can be created successfully
- appended nodes become part of the owning sequence
- removing a node restores sequence consistency
- node-to-input pointer/location resolution is correct for the represented node

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`

### Scenario 6: Name Character Decisions Depend on Split Context

Parsing logic needs to decide whether a character qualifies as part of a name. The decision is made through the module’s state-aware predicate.

The Rust version must support tests that verify:

- characters accepted by the original predicate remain accepted
- characters rejected by the original predicate remain rejected
- any decision dependence on `wordsplit` state is preserved

Traceability:
- `is_name_char`

## Requirements

### Functional Requirements

#### FR-1: Default State Initialization

The module shall provide a way to place a `wordsplit` state into a default, empty, internally consistent baseline state before configured use.

Traceability:
- `wordsplit_init0`

#### FR-2: Configured State Initialization

The module shall initialize a `wordsplit` state from input text, input length, and flags, preparing it for subsequent split-related operations.

Traceability:
- `wordsplit_init`

#### FR-3: State-Aware Name Character Classification

The module shall provide a state-aware predicate for determining whether a character is a valid name character for parsing purposes.

Traceability:
- `is_name_char`

#### FR-4: Error Code Storage

The module shall record an error code in the active `wordsplit` state and return a status consistent with the recorded error path.

Traceability:
- `_wsplt_seterr`

#### FR-5: Error Context Storage

The module shall store error-associated source context consisting of a character range identified by pointer/reference and length.

Traceability:
- `_wsplt_store_errctx`

#### FR-6: Combined Error-and-Context Update

The module shall support a single operation that stores error context and sets the corresponding error code together.

Traceability:
- `_wsplt_setctxerr`

#### FR-7: Out-of-Memory Handling

The module shall convert allocation failure into module error state, using the module’s fatal-allocation policy when applicable.

Traceability:
- `_wsplt_nomem`
- `_wsplt_alloc_die`

#### FR-8: Subordinate Split Invocation

The module shall support invoking split processing on a subordinate `wordsplit` context using a provided string segment, length, flags, and finalize control.

Traceability:
- `_wsplt_subsplit`

#### FR-9: Subordinate Error Propagation

When a subordinate split fails, the module shall transfer the subordinate error state into the parent `wordsplit` state.

Traceability:
- `_wsplt_seterr_sub`

#### FR-10: Internal Storage Growth

The module shall allocate or extend internal storage sufficient for a requested count, and shall report failure using module error handling.

Traceability:
- `alloc_space`

#### FR-11: Node Creation

The module shall create new `wordsplit_node` instances associated with a `wordsplit` owner.

Traceability:
- `wsnode_new`

#### FR-12: Node Append

The module shall insert a node into the owning `wordsplit` node sequence.

Traceability:
- `wsnode_append`

#### FR-13: Node Removal

The module shall remove a node from the owning `wordsplit` node sequence while preserving sequence consistency.

Traceability:
- `wsnode_remove`

#### FR-14: Node Location Resolution

The module shall resolve a `wordsplit_node` to the character location it represents within the owning `wordsplit` text/storage domain.

Traceability:
- `wsnode_ptr`

### Key Entities

#### Entity: `wordsplit`

Primary state holder for split processing.

Observed responsibilities from this module include:

- input-associated initialization state
- flags controlling behavior
- error code storage
- error context storage
- allocation/storage management
- ownership of internal node sequences
- participation in parent/subordinate split relationships

Traceability:
- used across all analyzed functions
- specifically initialized by `wordsplit_init0`, `wordsplit_init`
- mutated by error and allocation functions
- owns nodes manipulated by `wsnode_*`

#### Entity: `wordsplit_node`

Internal node representing a structural or positional unit used during split processing.

Observed responsibilities from this module include:

- membership in a node sequence owned by `wordsplit`
- sufficient positional information to resolve back to a character location
- support for creation, insertion, and removal

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`

#### Relationship: Parent and Subordinate `wordsplit` States

The module supports use of one `wordsplit` state as a subordinate worker for another.

Observed relationship rules include:

- subordinate processing is invoked with explicit input segment and control flags
- subordinate failure can be reflected back into the parent state
- parent-level error state may be derived from subordinate state

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

## Success Criteria

### SC-1: Initialization Correctness

Given a fresh `wordsplit` instance, the Rust module can initialize it to default state and then configured state without leaving uninitialized or contradictory observable module state.

Traceability:
- `wordsplit_init0`
- `wordsplit_init`

### SC-2: Error State Preservation

When an error code is set, the Rust module records the error in the active `wordsplit` state; when error context is provided, the stored context corresponds to the supplied text range.

Traceability:
- `_wsplt_seterr`
- `_wsplt_store_errctx`
- `_wsplt_setctxerr`

### SC-3: Memory Failure Semantics Match Module Policy

Under simulated allocation failure, the Rust module produces the same class of outcome as the C module: ordinary memory error state in non-fatal mode and fatal termination behavior in fatal-allocation mode.

Traceability:
- `_wsplt_nomem`
- `_wsplt_alloc_die`
- `alloc_space`

### SC-4: Subordinate Failure Is Observable at Parent Level

When a subordinate split operation fails, propagating subordinate state to the parent results in parent-visible failure information rather than successful completion or silent loss of the subordinate error.

Traceability:
- `_wsplt_subsplit`
- `_wsplt_seterr_sub`

### SC-5: Node Sequence Operations Remain Consistent

Across create, append, pointer-resolution, and remove operations, the Rust module maintains a consistent relationship between `wordsplit`, its node sequence, and represented text positions.

Traceability:
- `wsnode_new`
- `wsnode_append`
- `wsnode_remove`
- `wsnode_ptr`

### SC-6: Character Classification Compatibility

For inputs and state conditions exercised by regression tests derived from the C module, the Rust module’s name-character predicate returns the same accept/reject results as the original module behavior.

Traceability:
- `is_name_char`

### SC-7: Storage Requests Are Correctly Satisfied or Rejected

For requested internal storage growth, the Rust module either makes sufficient space available for subsequent processing or reports failure through the module’s defined error path.

Traceability:
- `alloc_space`