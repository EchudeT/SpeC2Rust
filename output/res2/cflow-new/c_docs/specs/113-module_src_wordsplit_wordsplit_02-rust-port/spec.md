# spec.md

## Title

Functional Specification: `module_src_wordsplit_wordsplit_02` Rust Port

## Status

Draft

## Scope

This specification covers the functional behavior of the `src/wordsplit/wordsplit.c` module area centered on node/segment management, segment coalescing, quote-removal stage handling, final word assembly, prefix-based node splitting, and environment/parameter lookup and assignment support used by the `wordsplit` processing state.

It applies to the Rust rewrite on branch `113-module_src_wordsplit_wordsplit_02-rust-port`.

## Overview

This module is responsible for managing intermediate token segments inside a `wordsplit` operation and converting those intermediate nodes into finalized output words. It also provides lookup and assignment helpers for variable and parameter expansion support that the broader `wordsplit` engine depends on.

The Rust version must preserve the observed functional boundaries:

- maintain an ordered collection of split-result nodes/segments;
- add, insert, split, and free segment nodes within a `wordsplit` session;
- remove quote-marking effects at the node level when the processing stage requires it;
- coalesce adjacent or related nodes into finalized textual segments;
- complete the finalization step that turns internal node state into finished word output;
- support environment-style variable lookup and retrieval through the `wordsplit` state;
- support assigning variables and positional/special parameters into the `wordsplit` state when requested by expansion logic.

## Source Traceability

Primary source file:

- `src/wordsplit/wordsplit.c`

Primary traced functions:

- `wsnode_insert`
- `wordsplit_add_segm`
- `wordsplit_free_nodes`
- `wordsplit_dump_nodes`
- `coalesce_segment`
- `wsnode_quoteremoval`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`
- `wordsplit_finish`
- `node_split_prefix`
- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`
- `wsplt_assign_var`
- `wsplt_assign_param`

Primary traced entities:

- `struct wordsplit`
- `struct wordsplit_node`

## Feature Specification

### 1. Ordered intermediate segment management

The module must support a `wordsplit` session maintaining an ordered set of intermediate nodes that represent portions of parsed input or derived expansion output.

Required behavior:

- A node can be inserted relative to an existing anchor position, preserving the intended ordering of segments.
- A new segment can be added from a source range identified by beginning and ending offsets plus node flags describing how that segment should be treated later.
- Node ordering must remain suitable for later coalescing and final word production.

Traceability:

- `wsnode_insert`
- `wordsplit_add_segm`
- `struct wordsplit`
- `struct wordsplit_node`

### 2. Node lifecycle cleanup

The module must support releasing all intermediate nodes associated with a `wordsplit` session when processing ends or aborts.

Required behavior:

- All nodes created for the session can be traversed and freed.
- Cleanup must leave the session without stale intermediate node ownership.

Traceability:

- `wordsplit_free_nodes`
- `struct wordsplit`
- `struct wordsplit_node`

### 3. Debug-oriented node inspection support

The module includes support for dumping current node state for inspection.

Required behavior:

- The Rust port must preserve the ability for the module logic to enumerate and expose node ordering/content for diagnostic purposes when such diagnostics are enabled by the surrounding implementation.
- This requirement is limited to behavior needed by current module diagnostics; it does not require a new public API format.

Traceability:

- `wordsplit_dump_nodes`

### 4. Segment coalescing and text assembly

The module must merge node content into larger textual units as processing advances toward finalized words.

Required behavior:

- A specific node or node segment can be coalesced into its resulting text representation.
- Coalescing must honor node boundaries and flags as interpreted by the current `wordsplit` state.
- Tail-oriented coalescing must be supported where the current processing position starts from a known node.
- Whole-list coalescing must be supported as a module stage.

Traceability:

- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`
- `struct wordsplit`
- `struct wordsplit_node`

### 5. Quote-removal stage handling

The module must support a quote-removal pass over intermediate nodes before final assembly where required by the `wordsplit` workflow.

Required behavior:

- Quote-removal processing operates on the maintained node sequence.
- The effect is to remove quote-marking semantics from nodes so subsequent coalescing/finalization reflects dequoted content according to current processing rules.
- Processing failures must be reportable to the caller through the module’s status return path.

Traceability:

- `wsnode_quoteremoval`
- `struct wordsplit`
- `struct wordsplit_node`

### 6. Finalization into completed words

The module must support a finishing stage that converts internal node state into the final output words retained by the `wordsplit` session.

Required behavior:

- Finalization consumes the prepared node sequence after prior transformations.
- The resulting word output must reflect the segment ordering and transformations already applied.
- Finalization must report success or failure.
- Finalization must be able to work with sessions containing multiple nodes and multiple resulting words.

Traceability:

- `wordsplit_finish`
- `struct wordsplit`
- `struct wordsplit_node`

### 7. Prefix-based node splitting

The module must support splitting an existing node so that a prefix portion becomes separately represented.

Required behavior:

- Given a node and a specified prefix range/length, the module can split that prefix into a distinct node segment.
- The resulting node sequence must preserve the original textual order.
- The operation must support updating the current tail/reference position used by callers that continue editing the node list.
- The new split portion must carry caller-supplied flags.

Traceability:

- `node_split_prefix`
- `struct wordsplit`
- `struct wordsplit_node`

### 8. Environment-style variable discovery and retrieval

The module must support locating variable values through the `wordsplit` state.

Required behavior:

- A variable can be searched by name and explicit name length.
- A lookup can distinguish between finding a value and failing to resolve one.
- A retrieval helper can return variable text suitable for expansion use.
- Variable lookup behavior must be driven by the `wordsplit` session configuration and data sources, not by hardcoded external assumptions in the Rust rewrite.

Traceability:

- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`
- `struct wordsplit`

### 9. Variable assignment support within wordsplit processing

The module must support assigning a named variable value through the `wordsplit` state.

Required behavior:

- A variable name and value can be provided for assignment.
- Assignment outcome must be reported to the caller.
- Assignment behavior must be mediated by the `wordsplit` session rules and storage/handler model represented in the original module.

Traceability:

- `wsplt_assign_var`
- `struct wordsplit`

### 10. Positional or indexed parameter assignment support

The module must support assigning a value to a parameter slot identified by index.

Required behavior:

- The caller can specify a parameter index and string value.
- Assignment outcome must be reported to the caller.
- Parameter assignment must integrate with the same `wordsplit` session state used for expansion-related processing.

Traceability:

- `wsplt_assign_param`
- `struct wordsplit`

## User Scenarios & Testing

### Scenario 1: Build intermediate segments from parsed input

A caller processing input text identifies subranges that belong to separate intermediate segments and adds them in order to a `wordsplit` session. The module stores them as nodes so later stages can transform them without losing source order.

The Rust version must support tests that verify:

- adding one segment creates one intermediate node;
- adding multiple segments preserves insertion order;
- segment metadata/flags remain attached to the corresponding node through later processing stages.

Traceability:

- `wordsplit_add_segm`
- `wsnode_insert`

### Scenario 2: Insert a derived node around existing content

During expansion or parsing, the caller needs to place a newly created node before or after an existing node. The module updates the node sequence so subsequent traversals see the derived content in the correct place.

The Rust version must support tests that verify:

- insertion before an anchor places the node immediately before it;
- insertion after an anchor places the node immediately after it;
- list consistency is preserved after repeated insertions.

Traceability:

- `wsnode_insert`

### Scenario 3: Split a prefix out of a node for special handling

A caller discovers that the first part of an existing node must be treated separately, such as when a prefix has different flags or processing semantics. The module splits the node and keeps both pieces in correct textual order.

The Rust version must support tests that verify:

- a node can be split at a specified prefix length;
- the prefix and remainder appear in order after the split;
- caller-specified flags apply to the split-out prefix node;
- the tail/reference pointer semantics needed by the caller are preserved functionally.

Traceability:

- `node_split_prefix`

### Scenario 4: Remove quote effects before producing final words

After parsing has marked quoted regions in nodes, the quote-removal stage is run. The module removes quote semantics so the resulting words contain dequoted text rather than quoting syntax markers.

The Rust version must support tests that verify:

- quote-removal can be invoked on a populated node list;
- resulting finalized output reflects quote removal where the original module would do so;
- errors during quote-removal are propagated as failure status.

Traceability:

- `wsnode_quoteremoval`
- `wordsplit_finish`

### Scenario 5: Coalesce adjacent node content into final text

A caller has a sequence of nodes representing one or more portions of a word after expansion. The module coalesces them into the appropriate textual units before final output is produced.

The Rust version must support tests that verify:

- coalescing a node sequence produces the expected assembled text for that sequence;
- whole-list coalescing and tail-started coalescing both produce consistent results where applicable;
- coalescing failures are reported.

Traceability:

- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

### Scenario 6: Finish a wordsplit session into output words

Once all parsing and expansion stages are complete, the caller invokes finishing logic. The module converts internal nodes into the final word array/state consumed by the broader program.

The Rust version must support tests that verify:

- finalization succeeds for valid node sequences;
- finalized words preserve source ordering after all prior edits;
- sessions with multiple resulting words finalize correctly;
- failure paths return an error status without claiming success.

Traceability:

- `wordsplit_finish`

### Scenario 7: Lookup a variable for expansion

During expansion, the caller requests a variable by name. The module resolves it through the `wordsplit` session and returns the current value if available.

The Rust version must support tests that verify:

- exact-name lookup succeeds when a variable exists;
- lookup using explicit name length does not require null-terminated matching beyond that length;
- unresolved variables are reported as unresolved rather than as successful empty lookups unless that is the represented session behavior;
- retrieval helpers return expansion-ready string content.

Traceability:

- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

### Scenario 8: Assign variables and parameters during processing

The caller needs to update named variables or indexed parameters as part of expansion-aware processing. The module performs the assignment through the `wordsplit` session and reports whether it succeeded.

The Rust version must support tests that verify:

- named variable assignment reports success when accepted by the session;
- parameter assignment by index reports success when accepted by the session;
- failure status is returned when assignment cannot be completed under session rules.

Traceability:

- `wsplt_assign_var`
- `wsplt_assign_param`

### Scenario 9: Abort processing and release all intermediate nodes

If parsing or expansion fails after nodes were created, the caller cleans up the session. The module releases all intermediate node resources.

The Rust version must support tests that verify:

- cleanup can be called after partial construction;
- cleanup after prior finalization or failure does not leave accessible intermediate nodes in the session state;
- cleanup supports sessions with multiple nodes.

Traceability:

- `wordsplit_free_nodes`

## Requirements

### Functional Requirements

#### FR-1: Maintain ordered node sequences
The module shall maintain intermediate `wordsplit_node` items in a deterministic sequence within a `wordsplit` session so that later transformations and finalization preserve intended textual order.

Traceability:

- `wsnode_insert`
- `wordsplit_add_segm`
- `struct wordsplit`
- `struct wordsplit_node`

#### FR-2: Support segment creation from source ranges
The module shall create intermediate segments from caller-provided begin/end offsets and associated flags.

Traceability:

- `wordsplit_add_segm`

#### FR-3: Support relative node insertion
The module shall support inserting a node before or after a specified anchor node within the current sequence.

Traceability:

- `wsnode_insert`

#### FR-4: Support node-sequence cleanup
The module shall provide cleanup that releases all intermediate nodes associated with a `wordsplit` session.

Traceability:

- `wordsplit_free_nodes`

#### FR-5: Support diagnostic traversal of nodes
The module shall support diagnostic inspection of the current node sequence sufficient to preserve existing dump behavior used by this module area.

Traceability:

- `wordsplit_dump_nodes`

#### FR-6: Support node and sequence coalescing
The module shall support combining node content into assembled text both for an individual target segment and for broader node-list processing stages.

Traceability:

- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

#### FR-7: Support quote-removal transformation
The module shall support a quote-removal pass over the intermediate node sequence before final output assembly where required by the session workflow.

Traceability:

- `wsnode_quoteremoval`

#### FR-8: Support final output assembly
The module shall transform the prepared node sequence into finalized words retained by the `wordsplit` session and report success or failure.

Traceability:

- `wordsplit_finish`

#### FR-9: Support prefix extraction from a node
The module shall support splitting a prefix portion out of an existing node into a separately represented node while preserving overall textual order.

Traceability:

- `node_split_prefix`

#### FR-10: Support variable discovery by exact name span
The module shall support resolving variable names supplied as `(name pointer, length)` pairs against data accessible from the `wordsplit` session.

Traceability:

- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

#### FR-11: Support variable value retrieval for expansion
The module shall return variable values in a form usable by the surrounding expansion logic and shall report unresolved lookups via status returns.

Traceability:

- `wsplt_env_lookup`
- `wsplt_env_getvar`

#### FR-12: Support named variable assignment
The module shall support assigning a value to a named variable through the `wordsplit` session and report assignment status.

Traceability:

- `wsplt_assign_var`

#### FR-13: Support indexed parameter assignment
The module shall support assigning a value to a parameter identified by index through the `wordsplit` session and report assignment status.

Traceability:

- `wsplt_assign_param`

#### FR-14: Preserve stage-to-stage consistency
The module shall preserve enough node metadata and ordering across insertion, splitting, quote-removal, coalescing, and finalization so that each later stage can operate on results from earlier stages without reinterpreting unrelated state.

Traceability:

- `wsnode_insert`
- `wordsplit_add_segm`
- `wsnode_quoteremoval`
- `coalesce_segment`
- `wordsplit_finish`
- `node_split_prefix`
- `struct wordsplit`
- `struct wordsplit_node`

### Key Entities

#### `wordsplit`
Session state for a single word-splitting operation.

Functional role evidenced by traced functions:

- owns or references the current intermediate node sequence;
- provides the context used during quote removal, coalescing, and finishing;
- provides access to variable lookup and assignment facilities;
- receives final output produced by finishing.

Relationships:

- contains or governs multiple `wordsplit_node` instances;
- is the state carrier passed to every traced operation in this module area.

Traceability:

- all traced functions
- `struct wordsplit`

#### `wordsplit_node`
Intermediate representation of a segment or partial word within a `wordsplit` session.

Functional role evidenced by traced functions:

- participates in ordered node-list insertion;
- represents a segment added from source ranges or created by splitting/coalescing operations;
- carries flags/metadata that affect quote removal, coalescing, and final assembly.

Relationships:

- belongs to one `wordsplit` session at a time;
- may be split into additional nodes;
- may be merged/coalesced into final textual output.

Traceability:

- `wsnode_insert`
- `wordsplit_add_segm`
- `coalesce_segment`
- `wsnode_quoteremoval`
- `wsnode_tail_coalesce`
- `node_split_prefix`
- `struct wordsplit_node`

## Success Criteria

### SC-1: Ordered insertion correctness
For controlled test sessions with existing node anchors, insertion before and after an anchor produces the expected node order in all cases covered by the original module behavior.

Traceability:

- `wsnode_insert`

### SC-2: Segment creation correctness
For test inputs using begin/end ranges and flags, created segments appear in the session node sequence with preserved order and retain their associated processing flags through later stages.

Traceability:

- `wordsplit_add_segm`

### SC-3: Prefix split correctness
For representative nodes, splitting out a prefix yields two ordered logical parts whose concatenation matches the original content span and whose split-out node uses the requested flags.

Traceability:

- `node_split_prefix`

### SC-4: Quote-removal stage correctness
For test cases containing quoted segments represented in node form, invoking quote-removal followed by finalization produces dequoted output matching the C module’s behavior for the same cases.

Traceability:

- `wsnode_quoteremoval`
- `wordsplit_finish`

### SC-5: Coalescing correctness
For representative multi-node inputs, node coalescing produces the same assembled text and word boundaries as the C module for the same session state.

Traceability:

- `coalesce_segment`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

### SC-6: Finalization correctness
For successful sessions, finishing produces final words equivalent in content and ordering to those produced by the original C module for the same inputs and intermediate node setup.

Traceability:

- `wordsplit_finish`

### SC-7: Lookup correctness
For existing and missing variable names, environment lookup and retrieval return values and resolution status equivalent to the C module when exercised through the same `wordsplit` configuration.

Traceability:

- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

### SC-8: Assignment correctness
For accepted and rejected named-variable and indexed-parameter assignments, the Rust port returns success/failure results equivalent to the C module and updates the `wordsplit` session state accordingly where successful.

Traceability:

- `wsplt_assign_var`
- `wsplt_assign_param`

### SC-9: Cleanup correctness
After cleanup is invoked on sessions with allocated intermediate nodes, no intermediate nodes remain owned by the session, and repeated test execution shows no retained logical node state from prior runs.

Traceability:

- `wordsplit_free_nodes`

### SC-10: End-to-end stage compatibility
For representative end-to-end cases that combine segment addition, node insertion or splitting, quote-removal, coalescing, variable lookup/assignment where applicable, and finalization, the Rust module produces the same observable final words and status outcomes as the C module.

Traceability:

- `wordsplit_add_segm`
- `wsnode_insert`
- `node_split_prefix`
- `wsnode_quoteremoval`
- `wsnode_coalesce`
- `wordsplit_finish`
- `wsplt_env_lookup`
- `wsplt_assign_var`
- `wsplt_assign_param`