# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_02`

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_02`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Target Rust branch: `113-module_src_wordsplit_wordsplit_02-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is the node-management and finalization portion of the project’s word-splitting pipeline. It operates on an in-progress `wordsplit` state and its linked list of `wordsplit_node` segments.

The Rust rewrite must preserve the functional behavior evidenced by this portion of `wordsplit.c`, namely:

- building and inserting segment nodes into the current split result,
- freeing and debugging/dumping node sequences,
- coalescing adjacent segments into final textual units,
- performing quote-removal over node content,
- finalizing the node list into completed words,
- splitting an existing node around a detected prefix region,
- looking up variables and parameters from the `wordsplit` context,
- assigning variable and positional-parameter values back into the `wordsplit` context.

This specification covers only behavior evidenced by the listed functions and data structures. It does not define new external capabilities.

## Scope

### In Scope

The Rust module must implement the functional responsibilities evidenced by these behaviors:

- ordered maintenance of a mutable node sequence,
- creation of segment nodes from source ranges and flags,
- destruction/reset of node storage,
- optional debug dumping of node state,
- transformation passes over nodes for quote removal and coalescing,
- final assembly of node content into completed split words,
- splitting one node into multiple nodes around a prefix span,
- environment and variable retrieval through the `wordsplit` context,
- variable and parameter assignment through the `wordsplit` context.

### Out of Scope

The following are not specified here because they are not evidenced by this module slice alone:

- design of unrelated parsing stages before nodes are created,
- public CLI behavior,
- thread safety,
- serialization,
- FFI interfaces,
- recovery or persistence features,
- any new expansion language beyond the lookups and assignments evidenced here.

## Feature Specification

### 1. Segment Node Sequence Management

The module manages a sequence of `wordsplit_node` items associated with a `wordsplit` instance.

Required behavior:

- A node can be inserted relative to an anchor node, both before and after it.
- The insertion operation must preserve list integrity for subsequent traversal and transformation.
- The module can create and append or otherwise place a segment node representing a source span `[beg, end)` plus associated flags.
- Segment creation failure must be reportable to the caller as an error result.
- The module must support complete release of all nodes belonging to a `wordsplit` instance and leave the node collection in a cleared state.

Traceability:
- `wsnode_insert`
- `wordsplit_add_segm`
- `wordsplit_free_nodes`
- `wordsplit_node`
- `wordsplit`

### 2. Node Inspection Support

The module includes internal support for dumping the current node sequence for inspection.

Required behavior:

- The Rust rewrite must preserve a way to produce node-sequence diagnostic output or equivalent inspectable state for debugging-oriented parity with the C module.
- This capability is internal/supporting behavior and must reflect the current node ordering and segment properties relevant to the transformation pipeline.

Traceability:
- `wordsplit_dump_nodes`

### 3. Segment Coalescing

The module merges node content into larger logical segments when permitted by node state.

Required behavior:

- A coalescing step can be applied to a specific node and may merge it with adjacent compatible content.
- A full coalescing pass can be applied across the current node sequence.
- A tail-oriented coalescing step can continue merging from a given node onward.
- Coalescing must preserve final textual ordering.
- Coalescing failures, including allocation-related failures implied by content merging, must be surfaced as errors.

Traceability:
- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

### 4. Quote Removal Pass

The module applies quote-removal logic over the node sequence before final word completion.

Required behavior:

- Quote-removal must operate over the existing node sequence held by `wordsplit`.
- The pass must remove quoting syntax from node content where this stage requires it while preserving the resulting literal text.
- The pass must integrate with later coalescing and finishing stages so that quote-removal affects final output words.
- Errors during quote-removal must be reportable.

Traceability:
- `wsnode_quoteremoval`

### 5. Finalization of Split Results

The module converts the processed node sequence into finalized word-splitting results.

Required behavior:

- Finalization must consume the current transformed node sequence in order.
- The final result must reflect prior segment creation, quote removal, node splitting, environment substitution results already stored in nodes/context, and coalescing decisions.
- Finalization must detect and report failure conditions rather than silently producing partial success.
- Finalization must leave the `wordsplit` state consistent for the caller to inspect final outputs or handle errors according to the broader subsystem contract.

Traceability:
- `wordsplit_finish`
- `wordsplit`
- `wordsplit_node`

### 6. Prefix-Based Node Splitting

The module can split an existing node around a designated prefix region.

Required behavior:

- Given a node and a `(beg, len)` region, the module can divide that node into multiple nodes representing unaffected and affected portions.
- The newly created pieces must remain correctly connected in sequence order.
- The operation must support flagging the produced prefix-related segment with provided flags.
- The operation may update a tail/output node pointer used by subsequent processing.
- Errors during splitting or node creation must be reported.

Traceability:
- `node_split_prefix`

### 7. Variable Lookup from Context

The module retrieves variable values from the active `wordsplit` context.

Required behavior:

- A lookup can search for a variable by name and explicit name length.
- The module distinguishes between finding a raw value source and producing a returned string result.
- A higher-level “get variable” behavior must be supported through the same context.
- Lookup behavior must be bounded by the supplied name length rather than requiring NUL-terminated full-string matching only.
- Failure or absence must be represented through return status rather than invented fallback behavior.

Traceability:
- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`
- `wordsplit`

### 8. Assignment into Context

The module assigns values back into the `wordsplit` context for named variables and positional parameters.

Required behavior:

- A named variable can be assigned using `(name, name length, value)`.
- A positional parameter can be assigned using a parameter index and value.
- Assignment must update the `wordsplit` context so that subsequent module operations can observe the new values.
- Assignment failures must be reported.
- The Rust rewrite must preserve the distinction between named variable assignment and positional-parameter assignment.

Traceability:
- `wsplt_assign_var`
- `wsplt_assign_param`
- `wordsplit`

## User Scenarios & Testing

### Scenario 1: Build a segmented word from source spans

A caller processing input text creates a `wordsplit` state and adds several segments referencing ranges of the source input. The node sequence is maintained in input order. Finalization later produces words reflecting those segments.

The Rust version must support tests that verify:

- segments can be added for multiple source ranges,
- resulting order is preserved,
- finalization consumes the added segments without reordering them.

Traceability:
- `wordsplit_add_segm`
- `wordsplit_finish`

### Scenario 2: Insert a node around an existing anchor

A transformation stage discovers that one logical segment must be inserted before or after an already existing node. The node list is updated and subsequent traversal still sees a valid sequence.

The Rust version must support tests that verify:

- insertion before an anchor produces the expected order,
- insertion after an anchor produces the expected order,
- neighboring links or equivalent sequence structure remain valid for later passes.

Traceability:
- `wsnode_insert`

### Scenario 3: Remove quotes and preserve literal text

Input processing produces nodes containing quoted material. The quote-removal pass removes quote syntax, and the resulting content is later coalesced and finalized into words that preserve the intended literal characters.

The Rust version must support tests that verify:

- quote-removal changes node content as required,
- removed quotes do not appear in final output,
- non-quote literal content is preserved.

Traceability:
- `wsnode_quoteremoval`
- `wordsplit_finish`

### Scenario 4: Coalesce adjacent compatible segments

After transformations, adjacent nodes represent parts of the same final word. A coalescing pass merges them so finalization yields a single combined textual unit.

The Rust version must support tests that verify:

- a targeted node can be coalesced with compatible adjacent content,
- a full-pass coalescer merges eligible sequences,
- textual concatenation order matches original node order.

Traceability:
- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

### Scenario 5: Split a node around a prefix match

During processing, one node contains a prefix region that must be isolated from surrounding text. The node is split into separate pieces while preserving order and marking the relevant piece.

The Rust version must support tests that verify:

- splitting one node can produce preceding, target, and trailing pieces as applicable,
- the target piece receives the requested flags,
- the resulting sequence remains traversable in order.

Traceability:
- `node_split_prefix`

### Scenario 6: Lookup a context variable by bounded name

A caller requests a variable using a character pointer plus explicit length semantics. The value is searched in the current `wordsplit` context and returned if present.

The Rust version must support tests that verify:

- lookup by exact bounded name returns the expected value,
- absent names are reported as absent/failure according to status,
- lookup does not require the caller to pass an independently terminated substring.

Traceability:
- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

### Scenario 7: Assign variables and positional parameters

A processing stage updates a named variable or positional parameter in the current split context before later expansion or finishing steps.

The Rust version must support tests that verify:

- assigning a named variable updates later lookup results,
- assigning a positional parameter stores the value under the requested index,
- assignment errors are observable to the caller.

Traceability:
- `wsplt_assign_var`
- `wsplt_assign_param`
- `wsplt_env_lookup`

### Scenario 8: Clean up node storage after processing

After successful or failed processing, the caller resets node state. The module frees all nodes and leaves the `wordsplit` instance ready for safe post-cleanup inspection or reuse according to broader subsystem behavior.

The Rust version must support tests that verify:

- all nodes are removed by cleanup,
- repeated cleanup on an already cleared state does not corrupt state,
- no stale sequence remains visible after cleanup.

Traceability:
- `wordsplit_free_nodes`

## Requirements

### Functional Requirements

#### FR-1: Maintain an ordered mutable node sequence
The module shall maintain an ordered sequence of `wordsplit_node` entries within a `wordsplit` processing context and support insertion relative to an existing node.

Traceability:
- `wsnode_insert`
- `wordsplit`
- `wordsplit_node`

#### FR-2: Create segment nodes from source spans
The module shall create segment nodes representing source span boundaries and associated flags, and attach them to the active `wordsplit` node sequence.

Traceability:
- `wordsplit_add_segm`

#### FR-3: Release all nodes owned by a split context
The module shall provide cleanup that releases all nodes associated with a `wordsplit` instance and clears the module-managed node sequence.

Traceability:
- `wordsplit_free_nodes`

#### FR-4: Expose node-sequence diagnostics
The module shall preserve diagnostic visibility into node ordering and content/state sufficient to mirror the C module’s dump behavior for debugging or parity testing.

Traceability:
- `wordsplit_dump_nodes`

#### FR-5: Merge compatible node segments
The module shall support coalescing compatible node content at single-node, full-list, and tail-forward granularities.

Traceability:
- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

#### FR-6: Remove quote syntax from node content
The module shall apply a quote-removal pass to the active node sequence so that final produced words omit removed quoting characters while preserving the intended literal text.

Traceability:
- `wsnode_quoteremoval`

#### FR-7: Finalize transformed nodes into completed words
The module shall finalize the current node sequence into completed word-splitting results consistent with prior transformation passes and node ordering.

Traceability:
- `wordsplit_finish`

#### FR-8: Split an existing node around a marked prefix region
The module shall support splitting an existing node into multiple ordered pieces around a specified subrange and assigning flags to the produced prefix-related piece.

Traceability:
- `node_split_prefix`

#### FR-9: Lookup variables by name and explicit length
The module shall support variable lookup from the `wordsplit` context using a name pointer/string plus explicit length semantics, with status indicating success or absence/failure.

Traceability:
- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

#### FR-10: Assign named variables into the split context
The module shall support assignment of named variables into the `wordsplit` context using explicit name length and value input.

Traceability:
- `wsplt_assign_var`

#### FR-11: Assign positional parameters into the split context
The module shall support assignment of positional parameter values by parameter index within the `wordsplit` context.

Traceability:
- `wsplt_assign_param`

#### FR-12: Propagate operation failures
The module shall surface failures from segment creation, node splitting, quote removal, coalescing, finishing, lookup, and assignment through explicit status results rather than masking them.

Traceability:
- `wordsplit_add_segm`
- `coalesce_segment`
- `wsnode_quoteremoval`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`
- `wordsplit_finish`
- `node_split_prefix`
- `wsplt_env_lookup`
- `wsplt_env_getvar`
- `wsplt_assign_var`
- `wsplt_assign_param`

### Key Entities

#### `wordsplit`
Processing context for one word-splitting operation.

Observed responsibilities from this module:

- owns or references the current node sequence,
- supplies environment/variable lookup context,
- receives variable and positional-parameter assignments,
- serves as the shared state consumed by quote-removal, coalescing, and finalization.

Traceability:
- `wordsplit`
- all listed functions taking `struct wordsplit *`

#### `wordsplit_node`
A mutable segment unit within the in-progress split result.

Observed responsibilities from this module:

- participates in an ordered sequence,
- represents a segment derived from source text or transformed content,
- carries flags/state used by splitting and coalescing,
- is inserted, split, merged, dumped, and freed by this module.

Traceability:
- `wordsplit_node`
- `wsnode_insert`
- `coalesce_segment`
- `node_split_prefix`

#### Relationship: `wordsplit` owns/manages `wordsplit_node` sequence
This module evidences a parent-child relationship where `wordsplit` manages the lifecycle and transformation of a linked or equivalently ordered collection of `wordsplit_node` items.

Traceability:
- `wordsplit_add_segm`
- `wordsplit_free_nodes`
- `wsnode_coalesce`
- `wordsplit_finish`

## Success Criteria

### SC-1: Ordered insertion parity
Given an existing sequence and an anchor node, insertion before and after the anchor shall produce the expected node order in all tested cases.

Traceability:
- `wsnode_insert`

### SC-2: Segment creation correctness
Given valid source span inputs, segment creation shall add nodes representing those spans and preserve their order through finalization.

Traceability:
- `wordsplit_add_segm`
- `wordsplit_finish`

### SC-3: Cleanup completeness
After cleanup, the `wordsplit` node sequence shall contain no remaining nodes, and repeated cleanup on the cleared state shall not corrupt module-managed state.

Traceability:
- `wordsplit_free_nodes`

### SC-4: Quote-removal correctness
For test cases containing quoted node content, final outputs after quote-removal and finishing shall exclude removed quote syntax while preserving intended literal text.

Traceability:
- `wsnode_quoteremoval`
- `wordsplit_finish`

### SC-5: Coalescing correctness
For eligible adjacent segments, coalescing shall produce combined text in original order and reduce the number of separate output segments accordingly.

Traceability:
- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

### SC-6: Finalization consistency
Finalization shall transform the current processed node sequence into completed words without reordering content and shall return an explicit error status on failure cases exercised by tests.

Traceability:
- `wordsplit_finish`

### SC-7: Prefix splitting correctness
Given a node and a designated subrange, node splitting shall produce correctly ordered pieces, assign requested flags to the target piece, and preserve traversability of the resulting sequence.

Traceability:
- `node_split_prefix`

### SC-8: Variable lookup fidelity
Context lookup by `(name, length)` shall return the correct value for present variables and a distinct non-success status for absent variables in all tested bounded-name cases.

Traceability:
- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

### SC-9: Assignment visibility
After assigning a named variable or positional parameter, subsequent context-dependent operations and direct lookup tests shall observe the assigned value.

Traceability:
- `wsplt_assign_var`
- `wsplt_assign_param`
- `wsplt_env_lookup`

### SC-10: Error propagation parity
Operations that can fail in the C module shall return explicit failure status in the Rust rewrite under equivalent induced failure or invalid-state tests.

Traceability:
- `wordsplit_add_segm`
- `coalesce_segment`
- `wsnode_quoteremoval`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`
- `wordsplit_finish`
- `node_split_prefix`
- `wsplt_env_lookup`
- `wsplt_env_getvar`
- `wsplt_assign_var`
- `wsplt_assign_param`

## Acceptance Notes

- The Rust rewrite may use Rust-native ownership and collection models, but it must preserve the functional behaviors defined above.
- Internal representations may differ from C as long as externally observable module behavior and transformation results remain equivalent within the covered scope.
- Any behavior not evidenced by the cited functions and entities is intentionally left unspecified here.