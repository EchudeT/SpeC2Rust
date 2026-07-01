# spec.md

## Title

Functional Specification: `module_src_wordsplit_wordsplit_node_06` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_node_06`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `117-module_src_wordsplit_wordsplit_node_06-rust-port`
- Generation date: `2026-06-17`

## Overview

This module covers internal list-node support used by the word-splitting subsystem. The analyzed functionality is limited to operations on `struct wordsplit_node`, specifically:

- determining the character length associated with a node,
- releasing node-owned resources,
- locating the last node in a linked chain.

The Rust rewrite must preserve the observable behavior of these node utilities as used by the surrounding `wordsplit` logic in `src/wordsplit/wordsplit.c`.

## Feature Specification

### Summary

The module provides basic functional support for node-based word-splitting state. A `wordsplit_node` participates in a linked structure managed by the broader `wordsplit` subsystem. The supported behavior is restricted to:

1. computing the effective length represented by a node,
2. freeing a node and any node-owned dynamic content,
3. traversing a node chain to obtain its tail.

### In-Scope Functionality

The Rust version must implement behavior equivalent to the following source-backed operations:

- `wsnode_len`: return the length represented by a single `wordsplit_node`.
- `wsnode_free`: release a single `wordsplit_node`.
- `wsnode_tail`: return the final node reachable from a given node in the chain.

### Behavioral Boundaries

This specification does not require functionality beyond the evidenced module scope. In particular, it does not require introducing new public APIs, alternate traversal modes, persistence, concurrency behavior, or recovery features not shown in the analyzed source.

## User Scenarios & Testing

### Scenario 1: Length query for a node

A caller in the word-splitting subsystem has a node that represents part of a parsed token and needs to know how many characters that node contributes.

**Expected behavior**
- The Rust module returns the node’s represented length as a non-negative size value.
- The result is based on the node’s current content/state.
- Repeated calls on an unchanged node produce the same result.

**Testing guidance**
- Construct nodes with representative content states used by the surrounding subsystem.
- Verify that the returned length matches the node’s stored or implied content length.
- Verify stable results across repeated calls without mutation.

### Scenario 2: Freeing an allocated node

A caller has finished using a node and needs to release resources associated with that node.

**Expected behavior**
- The Rust module releases any resources owned by the node.
- The free operation applies to one node, matching the source function scope.
- After freeing, the node must no longer be used by the caller.

**Testing guidance**
- Create nodes containing owned dynamic content.
- Invoke node-freeing behavior and verify resource release through Rust ownership/drop behavior.
- Verify that freeing one node does not imply freeing unrelated nodes unless ownership relationships explicitly require it.

### Scenario 3: Finding the tail of a node chain

A caller has the head or middle of a linked node chain and needs the last node in that chain.

**Expected behavior**
- The Rust module follows next-link relationships until the final reachable node.
- The last node in the chain is returned.
- For a single-node chain, that same node is returned.

**Testing guidance**
- Test chains of length 1, 2, and multiple nodes.
- Verify that the returned node has no successor.
- Verify that starting from an interior node returns the tail reachable from that starting point.

## Requirements

### Functional Requirements

#### FR-1: Node length evaluation
The module shall provide node-level length evaluation equivalent to `wsnode_len` in `src/wordsplit/wordsplit.c:479-488`.

**Traceability**
- Function: `wsnode_len`
- Type: `struct wordsplit_node`

#### FR-2: Node resource release
The module shall provide node-level resource release equivalent to `wsnode_free` in `src/wordsplit/wordsplit.c:500-506`.

**Traceability**
- Function: `wsnode_free`
- Type: `struct wordsplit_node`

#### FR-3: Tail-node lookup
The module shall provide linked-chain tail lookup equivalent to `wsnode_tail` in `src/wordsplit/wordsplit.c:544-550`.

**Traceability**
- Function: `wsnode_tail`
- Type: `struct wordsplit_node`

#### FR-4: Compatibility with surrounding wordsplit state
The module shall operate on node instances that are part of the broader `struct wordsplit`-managed subsystem without requiring behavior outside the analyzed source scope.

**Traceability**
- Source file: `src/wordsplit/wordsplit.c`
- Types: `struct wordsplit`, `struct wordsplit_node`

### Key Entities

#### `wordsplit_node`
A node entity used by the word-splitting subsystem. Based on the analyzed functions, it has two essential behavioral roles:

- it represents some amount of character content whose length can be queried;
- it participates in a linked chain that can be traversed to the last node.

It may also own dynamically managed resources that must be released when the node is freed.

**Traceability**
- Definition area: `src/wordsplit/wordsplit.c:416-430`
- Functions using it: `wsnode_len`, `wsnode_free`, `wsnode_tail`

#### `wordsplit`
The broader subsystem state within which node instances are created and used. This module does not define its full behavior, but the Rust port must remain compatible with node usage inside that subsystem.

**Traceability**
- Referenced throughout `src/wordsplit/wordsplit.c`
- Associated with node-related usage near lines 469, 491, 509

#### Node chain relationship
`wordsplit_node` instances are linked so that one node can lead to another, enabling traversal from a starting node to a tail node.

**Traceability**
- Function: `wsnode_tail`
- Type: `struct wordsplit_node`

## Success Criteria

### SC-1: Length behavior parity
For representative node states derived from the C module behavior, the Rust implementation returns the same node length results as the source logic of `wsnode_len`.

### SC-2: Correct tail resolution
For linked chains of one or more nodes, the Rust implementation returns the final reachable node from the given starting node, matching the behavior of `wsnode_tail`.

### SC-3: Resource release coverage
The Rust implementation releases node-owned resources with behavior equivalent in scope to `wsnode_free`, with no retained ownership of freed node content after disposal.

### SC-4: Module-scope conformance
The Rust port implements only the evidenced node support behavior from `src/wordsplit/wordsplit.c` for this module scope and does not require unsupported capabilities outside node length evaluation, node release, and tail lookup.

## Acceptance Notes

- Conformance should be validated against the semantics of the analyzed C functions, not against newly invented interfaces.
- If Rust design changes internal representation, observable behavior for the specified scenarios must remain equivalent.
- Any nullability or ownership distinctions introduced by Rust must preserve the original module’s functional outcomes for valid calling patterns evidenced by the source.