# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit_02`

## Metadata

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_02`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `113-module_src_wordsplit_wordsplit_02-rust-port`
- Generation date: `2026-06-11`

## Overview

This module covers the node and segment management portion of the word-splitting engine, together with the finalization steps that turn parsed segments into finished words and the variable lookup/assignment helpers used during expansion.

The Rust rewrite must preserve the observed behavior of this module as evidenced by the analyzed functions and data structures in `src/wordsplit/wordsplit.c`. In scope are:

- maintaining an ordered collection of word-split nodes and inserting segments into it,
- freeing and optionally dumping node state,
- quote-removal and segment coalescing operations,
- finalizing the accumulated node stream into completed output words,
- splitting an existing node to isolate a prefix segment,
- environment and parameter lookup helpers used by expansion,
- assignment of variables and positional parameters into the `wordsplit` processing state.

Out of scope for this spec are capabilities not evidenced by this module slice, including new public APIs, unrelated parsing stages, concurrency guarantees, serialization, or FFI behavior.

## Feature Specification

### 1. Node-based segment management

The module shall manage word-split content as an ordered sequence of nodes associated with a `wordsplit` processing context.

Observed behaviors from `wsnode_insert`, `wordsplit_add_segm`, `wordsplit_free_nodes`, and `node_split_prefix` show that the module must support:

- inserting a node relative to an anchor node, either before or after it,
- appending or otherwise adding a segment described by source bounds and flags,
- splitting an existing node so that a prefix range becomes its own node with supplied flags,
- releasing all managed nodes when processing is complete or aborted.

The Rust version must preserve segment ordering semantics and must keep node relationships consistent after insertion, splitting, coalescing, and cleanup.

### 2. Segment normalization before final output

The module performs normalization passes over the node list before final output is emitted.

Observed behaviors from `coalesce_segment`, `wsnode_quoteremoval`, `wsnode_coalesce`, and `wsnode_tail_coalesce` show that the Rust version must implement:

- quote-removal processing on nodes that require it,
- coalescing of compatible adjacent segments into a single logical segment,
- tail coalescing behavior for a supplied tail node or tail position,
- repeated or staged normalization sufficient for `wordsplit_finish` to consume a stable node stream.

The exact internal representation may differ in Rust, but externally visible behavior must remain equivalent: segments that should merge become a single output word fragment, and segments subject to quote removal are emitted without the removed quoting characters affecting the final token text.

### 3. Finalization into completed words

The function `wordsplit_finish` indicates that this module is responsible for converting the internal node list into the finished state expected by the wider wordsplit engine.

The Rust implementation must provide equivalent finalization behavior, including:

- consuming the accumulated node sequence,
- applying the required normalization passes before producing final words,
- constructing final word text from segment content in order,
- respecting segment flags and boundaries when deciding how output words are formed,
- reporting success or failure through module-appropriate result signaling.

This spec does not require any new public interface; it requires behavioral equivalence for the finalization responsibilities evidenced in `wordsplit_finish`.

### 4. Debug-oriented node inspection

The function `wordsplit_dump_nodes` demonstrates that the module has a node-dump capability used for inspection of the internal node state.

The Rust rewrite must preserve the availability of an equivalent internal/debug facility sufficient to inspect current node ordering and segment state for development or diagnostic parity. This requirement is limited to behavior evidenced by the source and does not require a new stable external debugging API.

### 5. Variable lookup for expansion support

The functions `wsplt_env_find`, `wsplt_env_lookup`, and `wsplt_env_getvar` show that this module consults the processing context for named variable values.

The Rust version must support equivalent lookup behavior for a variable name provided as a pointer-plus-length slice in C terms, meaning behavior must be based on explicit name length rather than requiring null-terminated ownership assumptions.

Required behavior evidenced by these functions includes:

- locating a variable value by name within the `wordsplit` context,
- returning lookup success/failure distinctly from the value itself,
- supporting a higher-level “get variable” operation layered on the lower-level lookup behavior.

This spec only requires the lookup semantics evidenced here and does not assume any broader shell-compatible expansion behavior beyond this module.

### 6. Variable and parameter assignment into processing state

The functions `wsplt_assign_var` and `wsplt_assign_param` show that this module can update the processing context with assigned values.

The Rust version must support equivalent assignment behavior for:

- a named variable, given its name and explicit name length plus a value string,
- a positional or indexed parameter, given a parameter index and value.

The module must update the `wordsplit` state so that subsequent lookups and later processing observe the assigned values consistently.

## User Scenarios & Testing

### Scenario 1: Build a segmented word and finish it

A caller accumulates several segments in a `wordsplit` context, each with source bounds and flags. The module adds them to the node list, runs finalization, and produces the completed word text in the correct order.

Tests should verify:

- segments added in order appear in final output in that same logical order,
- finalization succeeds for a valid non-empty segment list,
- resulting output reflects all participating segments.

Traceability: `wordsplit_add_segm`, `wsnode_insert`, `wordsplit_finish`, `wordsplit_node`.

### Scenario 2: Remove quoting and merge compatible adjacent segments

A caller has adjacent nodes that belong to the same final word, some of which contain quoting that should not remain in output. The normalization passes remove the quoting effect and coalesce segments so the final word is emitted as one contiguous value.

Tests should verify:

- quote-removal changes final emitted text as expected,
- compatible adjacent nodes are merged before or during finalization,
- no extra word boundary is introduced between coalesced segments.

Traceability: `wsnode_quoteremoval`, `coalesce_segment`, `wsnode_coalesce`, `wsnode_tail_coalesce`, `wordsplit_finish`.

### Scenario 3: Split a node prefix and continue processing

A caller needs to isolate the prefix portion of an existing node into its own node with specific flags, while preserving the remainder for later processing. After splitting, finalization still produces correctly ordered output.

Tests should verify:

- the prefix becomes a separate logical segment,
- the remainder is preserved,
- the total combined content after finalization matches the original content except for intended flag-driven behavior,
- node ordering remains valid after the split.

Traceability: `node_split_prefix`, `wsnode_insert`, `wordsplit_finish`.

### Scenario 4: Lookup an assigned variable by explicit name length

A caller assigns a named variable into the `wordsplit` context and later requests its value using a name plus length. The module returns the assigned value.

Tests should verify:

- exact-length name matching is honored,
- the assigned value is returned after assignment,
- lookup failure is distinguishable when the name is absent.

Traceability: `wsplt_env_find`, `wsplt_env_lookup`, `wsplt_env_getvar`, `wsplt_assign_var`, `wordsplit`.

### Scenario 5: Assign and retrieve a positional parameter

A caller assigns a parameter value by numeric index and then performs processing that relies on parameter storage in the `wordsplit` context.

Tests should verify:

- assignment to a valid parameter index updates the context,
- later access paths that depend on parameter storage observe the assigned value,
- existing unrelated variables/parameters remain unaffected.

Traceability: `wsplt_assign_param`, `wordsplit`.

### Scenario 6: Cleanup after processing

After completion or failure, the node list is released. No stale node state remains attached to the processing context.

Tests should verify:

- cleanup on a populated context succeeds,
- cleanup on an already empty context is safe,
- after cleanup, final node references in the context are cleared or otherwise unusable by design.

Traceability: `wordsplit_free_nodes`, `wordsplit`, `wordsplit_node`.

### Scenario 7: Internal dump reflects current node state

During debugging, a developer invokes node dumping after inserting or splitting nodes. The dump reflects current node ordering and segment characteristics closely enough to diagnose processing behavior.

Tests should verify:

- dumping on an empty and non-empty node list does not corrupt processing state,
- dump output changes consistently with node insertion/splitting/coalescing operations.

Traceability: `wordsplit_dump_nodes`, `wsnode_insert`, `node_split_prefix`, `wsnode_coalesce`.

## Requirements

### Functional Requirements

#### FR-1: Ordered node insertion
The module shall support insertion of a node into the `wordsplit` node sequence relative to an anchor position, before or after that anchor.

Traceability: `wsnode_insert`, `wordsplit_node`, `wordsplit`.

#### FR-2: Segment addition by source range
The module shall support creation/addition of a segment defined by beginning offset, ending offset, and flags into the current `wordsplit` state.

Traceability: `wordsplit_add_segm`, `wordsplit`.

#### FR-3: Node sequence cleanup
The module shall release all nodes associated with a `wordsplit` context and leave the context in a cleaned-up node state.

Traceability: `wordsplit_free_nodes`, `wordsplit`, `wordsplit_node`.

#### FR-4: Node-state inspection
The module shall provide an internal/debug means to inspect the current node sequence without changing functional results of subsequent processing.

Traceability: `wordsplit_dump_nodes`.

#### FR-5: Segment coalescing
The module shall merge compatible node segments when normalization requires them to become one logical segment.

Traceability: `coalesce_segment`, `wsnode_coalesce`, `wsnode_tail_coalesce`.

#### FR-6: Quote-removal processing
The module shall perform quote-removal on nodes during normalization so that quoting syntax handled by this stage does not remain in the final emitted text.

Traceability: `wsnode_quoteremoval`, `wordsplit_finish`.

#### FR-7: Finalization of node stream into output words
The module shall finalize the current node sequence into the completed word-splitting result, applying the normalization stages required by this module before output is considered complete.

Traceability: `wordsplit_finish`.

#### FR-8: Prefix isolation from an existing node
The module shall support splitting an existing node so that a specified prefix range becomes a separate node with caller-supplied flags, while preserving the remaining content.

Traceability: `node_split_prefix`, `wordsplit_node`, `wordsplit`.

#### FR-9: Named variable lookup
The module shall support lookup of a variable value by explicit name and name length from the `wordsplit` processing context.

Traceability: `wsplt_env_find`, `wsplt_env_lookup`, `wsplt_env_getvar`, `wordsplit`.

#### FR-10: Named variable assignment
The module shall support assignment of a value to a named variable in the `wordsplit` processing context using explicit name length.

Traceability: `wsplt_assign_var`, `wordsplit`.

#### FR-11: Positional/indexed parameter assignment
The module shall support assignment of a value to a positional or indexed parameter stored in the `wordsplit` processing context.

Traceability: `wsplt_assign_param`, `wordsplit`.

#### FR-12: Lookup consistency after assignment
Values assigned through named-variable or parameter assignment shall be reflected consistently in the same `wordsplit` context for subsequent processing steps that use that stored state.

Traceability: `wsplt_assign_var`, `wsplt_assign_param`, `wsplt_env_lookup`, `wsplt_env_getvar`, `wordsplit_finish`.

### Key Entities

#### `wordsplit`
The central processing context for this module. It owns or references:

- the current node sequence being built and normalized,
- variable/parameter storage consulted by lookup helpers and updated by assignment helpers,
- finalization state used to produce completed output words.

Traceability: `struct wordsplit` references throughout the listed functions.

#### `wordsplit_node`
A node representing a segment or fragment of a word under construction. A node participates in an ordered sequence and carries enough state for:

- insertion relative to other nodes,
- source range tracking,
- flags controlling processing behavior,
- quote removal and coalescing,
- splitting into prefix and remainder segments.

Traceability: `struct wordsplit_node`, `wsnode_insert`, `wordsplit_add_segm`, `coalesce_segment`, `node_split_prefix`.

#### Variable and parameter entries within `wordsplit`
Internal state associated with the `wordsplit` context for named variables and indexed parameters. This state is used by lookup helpers and modified by assignment helpers.

Traceability: `wsplt_env_find`, `wsplt_env_lookup`, `wsplt_env_getvar`, `wsplt_assign_var`, `wsplt_assign_param`.

## Success Criteria

1. The Rust module can add multiple segments to a processing context and finalize them into completed output with preserved logical ordering.
   - Traceability: `wordsplit_add_segm`, `wordsplit_finish`.

2. When adjacent segments are compatible for merging, finalization produces the same word content as if those segments had been treated as one continuous fragment.
   - Traceability: `coalesce_segment`, `wsnode_coalesce`, `wsnode_tail_coalesce`, `wordsplit_finish`.

3. Quote-removal processing affects final output text so that quoting syntax handled by this module is not retained in emitted words.
   - Traceability: `wsnode_quoteremoval`, `wordsplit_finish`.

4. Splitting a node prefix yields a valid node sequence whose final combined output remains ordered and content-consistent with the original segment stream, subject to intended flag effects.
   - Traceability: `node_split_prefix`, `wsnode_insert`, `wordsplit_finish`.

5. Looking up a named variable by explicit name length after assignment returns the assigned value, and looking up an absent name reports absence without returning a false match.
   - Traceability: `wsplt_env_lookup`, `wsplt_env_getvar`, `wsplt_assign_var`.

6. Assigning a positional/indexed parameter updates the `wordsplit` context so later processing observes that parameter value.
   - Traceability: `wsplt_assign_param`, `wordsplit`.

7. Releasing nodes after processing leaves the context free of active node content and does not fail when invoked on an empty node set.
   - Traceability: `wordsplit_free_nodes`, `wordsplit_node`, `wordsplit`.

8. Internal/debug dumping of nodes can be performed on current state without changing subsequent functional processing results.
   - Traceability: `wordsplit_dump_nodes`.

9. All required behaviors above are implemented without introducing unsupported capabilities not evidenced by `src/wordsplit/wordsplit.c`.
   - Traceability: all listed module functions and types.