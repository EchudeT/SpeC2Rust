# spec.md

## Title

Functional Specification for `module_src_wordsplit_wordsplit_node_06` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_wordsplit_wordsplit_node_06`
- **Category**: `module_cluster`
- **Source file**: `src/wordsplit/wordsplit.c`
- **Rust branch**: `117-module_src_wordsplit_wordsplit_node_06-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module covers the node-level list support used by the word-splitting subsystem. The analyzed functionality is limited to operations on `struct wordsplit_node` instances that are linked together and associated with a `struct wordsplit` context.

The Rust rewrite must preserve the observed behavior of this node utility layer:

- determine the effective length represented by a node,
- release a node and any node-owned dynamic content,
- walk a linked node chain to obtain its last element.

This specification is intentionally limited to behavior evidenced by the analyzed C module content and does not define unrelated parser, tokenizer, or public API capabilities.

## Feature Specification

### Feature: Node length evaluation

The module must provide behavior equivalent to `wsnode_len` for a `wordsplit_node`.

Observed purpose: compute the length contribution of a single node for later word assembly or accounting.

Required behavior:

- Accept a node instance.
- Return the node’s effective content length as a non-negative size value.
- Support the node variants represented in `struct wordsplit_node`, including the variant that stores string-backed content and the variant that stores a length-bearing segment/range representation.
- Produce results consistent with the node’s own stored content rather than with any external traversal state.

Traceability:

- `wsnode_len` in `src/wordsplit/wordsplit.c:479-488`
- `struct wordsplit_node` in `src/wordsplit/wordsplit.c:416-430`

### Feature: Node destruction

The module must provide behavior equivalent to `wsnode_free` for a `wordsplit_node`.

Observed purpose: dispose of a single node and release any dynamic memory owned by that node.

Required behavior:

- Accept a node instance.
- Release any node-owned dynamically allocated string content when present.
- Release the node itself.
- Not require callers to manually free node-owned content before invoking node destruction.

This feature is scoped to destruction of one node at a time, because only single-node destruction is evidenced in the analyzed functions.

Traceability:

- `wsnode_free` in `src/wordsplit/wordsplit.c:500-506`
- `struct wordsplit_node` in `src/wordsplit/wordsplit.c:416-430`

### Feature: Tail discovery in a node chain

The module must provide behavior equivalent to `wsnode_tail` for a linked chain of `wordsplit_node` instances.

Observed purpose: locate the final node in a chain connected by `next` links.

Required behavior:

- Accept a starting node in a linked chain.
- Follow node-to-node linkage until the last reachable node is found.
- Return that final node.
- Preserve the chain structure; the operation is read-only with respect to list topology.

This feature is only defined for traversal from a provided node and does not imply any additional insertion, deletion, or mutation behavior beyond what is evidenced.

Traceability:

- `wsnode_tail` in `src/wordsplit/wordsplit.c:544-550`
- `struct wordsplit_node` in `src/wordsplit/wordsplit.c:416-430`

## User Scenarios & Testing

### Scenario 1: Measure the length contribution of a prepared node

A caller in the word-splitting subsystem has already created a node representing a piece of output. Before combining pieces, it needs the node’s contribution to output length.

The Rust module must support:

- obtaining the correct length for a node containing direct string content,
- obtaining the correct length for a node containing an alternate length-defined representation,
- returning a size value suitable for later aggregation by higher-level wordsplit logic.

Suggested tests:

- Create a node with owned string content and verify the returned length matches that string length.
- Create a node with a range/segment-style representation and verify the returned length matches the represented span length.
- Verify repeated length queries on the same node return the same value while the node is unchanged.

Traceability:

- `wsnode_len`
- `struct wordsplit_node`

### Scenario 2: Free a node after it is no longer needed

A caller has finished using a node and must dispose of it without leaking any node-owned content.

The Rust module must support:

- destruction of a node with owned dynamic string content,
- destruction of a node without such content,
- one-step cleanup through the node destruction operation.

Suggested tests:

- Allocate a node with owned string content, destroy it, and verify no retained ownership remains in safe Rust design.
- Allocate a node whose active representation does not own a string and destroy it successfully.
- Verify callers do not need a separate string-release step before node destruction.

Traceability:

- `wsnode_free`
- `struct wordsplit_node`

### Scenario 3: Find the last node in a linked chain

A caller has the head or an interior pointer of a linked node chain and needs the chain tail before appending or inspecting the final piece.

The Rust module must support:

- returning the same node when the input is already the last node,
- traversing multiple `next` links to return the final node,
- leaving all links unchanged.

Suggested tests:

- Single-node chain returns that node.
- Multi-node chain returns the last node.
- Starting from a middle node returns the last reachable node from that point.

Traceability:

- `wsnode_tail`
- `struct wordsplit_node`

## Requirements

### Functional Requirements

#### FR-1: Single-node length calculation

The Rust port shall calculate the effective length represented by one `wordsplit_node`.

The calculation shall reflect the node’s active content form as evidenced by the node structure and `wsnode_len`.

Traceability:

- `wsnode_len`
- `struct wordsplit_node`

#### FR-2: Support for node content variants used in length calculation

The Rust port shall support the content representations of `wordsplit_node` that participate in length determination, including string-backed content and non-string length-defined content present in the node structure.

Traceability:

- `wsnode_len`
- `struct wordsplit_node`

#### FR-3: Single-node destruction

The Rust port shall support destruction/cleanup of one `wordsplit_node`, including cleanup of node-owned dynamic content when applicable.

Traceability:

- `wsnode_free`
- `struct wordsplit_node`

#### FR-4: Tail lookup from a starting node

The Rust port shall support traversal from a provided `wordsplit_node` through its linked `next` chain to identify the final reachable node.

Traceability:

- `wsnode_tail`
- `struct wordsplit_node`

#### FR-5: Non-mutating tail traversal

The Rust port shall perform tail lookup without altering node content or link relationships.

Traceability:

- `wsnode_tail`
- `struct wordsplit_node`

### Key Entities

#### `wordsplit_node`

Core node entity used by this module.

Observed role:

- represents one piece of wordsplit intermediate/output data,
- participates in a singly linked chain,
- carries content in more than one form, including a direct string form and an alternate structured form used by length calculation,
- may own dynamic content that must be released during destruction.

Relationships:

- linked to another `wordsplit_node` through a `next`-style relationship,
- associated with the broader `wordsplit` subsystem as internal state used by that context.

Traceability:

- `struct wordsplit_node` in `src/wordsplit/wordsplit.c:416-430`

#### `wordsplit`

Owning or surrounding subsystem context in which node chains are used.

Observed role in this module:

- provides the larger processing context in which nodes exist,
- is related to node management, though the analyzed functions are node-focused rather than context-focused.

Relationships:

- contains or references node structures elsewhere in the source file,
- uses node utilities as internal support behavior.

Traceability:

- `struct wordsplit` references in `src/wordsplit/wordsplit.c`
- node/context relationship evidenced near `src/wordsplit/wordsplit.c:469, 491, 509`

## Success Criteria

### SC-1: Length behavior matches source semantics

For each `wordsplit_node` content form evidenced in the analyzed structure and used by `wsnode_len`, the Rust implementation returns the same effective length that the C module would return for equivalent node state.

Traceability:

- `wsnode_len`
- `struct wordsplit_node`

### SC-2: Tail lookup matches source semantics

Given equivalent linked node chains, the Rust implementation returns the final reachable node from the supplied starting node, matching the behavior of `wsnode_tail`.

Traceability:

- `wsnode_tail`
- `struct wordsplit_node`

### SC-3: Node cleanup covers node-owned content

The Rust implementation’s node cleanup behavior releases all node-owned content covered by `wsnode_free` semantics and does not require extra caller cleanup for such owned content.

Traceability:

- `wsnode_free`
- `struct wordsplit_node`

### SC-4: Supported scenarios pass module-level tests

The Rust port passes tests covering:

- length calculation for each evidenced node content representation,
- cleanup of nodes with and without owned string content,
- tail lookup for single-node and multi-node chains.

Traceability:

- `wsnode_len`
- `wsnode_free`
- `wsnode_tail`
- `struct wordsplit_node`