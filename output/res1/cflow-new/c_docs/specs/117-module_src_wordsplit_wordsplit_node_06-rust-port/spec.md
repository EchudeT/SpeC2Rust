# spec.md

## Title

Functional Specification: `module_src_wordsplit_wordsplit_node_06`

## Overview

This module defines list-node utilities used by the `wordsplit` subsystem in `src/wordsplit/wordsplit.c`. The evidenced scope is limited to operations on `struct wordsplit_node` instances:

- computing the total text length represented by a linked node chain,
- freeing a linked node chain and any owned node text,
- locating the tail node of a linked node chain.

The Rust rewrite must preserve this node-level behavior as used by the surrounding `wordsplit` logic. The specification does not require any broader parsing, tokenization, or shell-expansion behavior beyond what is directly evidenced by these node utility functions and the referenced `wordsplit` / `wordsplit_node` data types.

## Feature Specification

### Feature: Linked word-split node utility operations

The module provides support operations for a linked sequence of `wordsplit_node` values associated with the `wordsplit` subsystem.

A Rust implementation must support the following functional behavior:

1. **Aggregate node-chain length**
   - Given the head of a node chain, determine the total length of the text represented by that chain.
   - The result must be returned as a non-negative size/count value.
   - The operation is based on traversing the chain of `wordsplit_node` elements.

2. **Node-chain destruction**
   - Given the head of a node chain, release the chain and any node-owned text storage represented by that chain.
   - The operation must safely consume the entire chain reachable from the provided head.
   - After destruction, no nodes from that chain remain retained by the module.

3. **Tail lookup**
   - Given the head of a node chain, return the final node in that chain.
   - This operation must follow node links until the last reachable element is found.

### In-scope boundaries

This specification covers only the above node utility behavior evidenced by:

- `wsnode_len`
- `wsnode_free`
- `wsnode_tail`
- `struct wordsplit_node`
- surrounding ownership context implied by `struct wordsplit`

### Out-of-scope boundaries

The following are not part of this module specification unless required only as surrounding context:

- word parsing rules,
- quote handling,
- expansion semantics,
- public CLI behavior,
- persistence, serialization, or inter-process exchange,
- concurrency guarantees,
- any new public API surface not required to preserve the evidenced behavior.

## User Scenarios & Testing

### Scenario 1: Measure the total text represented by a node list

A caller in the `wordsplit` subsystem builds or receives a linked chain of `wordsplit_node` items, each carrying text content. Before allocating or copying a combined representation, it needs the total number of characters represented by the chain.

**Expected behavior**
- The module accepts the chain head.
- It traverses the chain.
- It returns the sum of represented text lengths for all reachable nodes.

**Testing focus**
- Single-node chain returns that node's text length.
- Multi-node chain returns the sum of all node text lengths in order-independent accumulation.
- Empty or absent chain handling must match the C behavior expected by callers; the Rust port must preserve this observable behavior.

### Scenario 2: Release an entire temporary node chain

A caller finishes using an intermediate `wordsplit_node` chain created during word processing and needs to dispose of it.

**Expected behavior**
- The module releases each reachable node exactly once.
- Any text storage owned by each node is also released.
- The full chain is consumed from head through tail.

**Testing focus**
- Single-node chain is fully released.
- Multi-node chain is fully released.
- No reachable node is skipped.
- The Rust version must use ownership-safe destruction while preserving the same effective cleanup scope as the C code.

### Scenario 3: Append or inspect by first finding the tail

A caller has the head of an existing node chain and needs the last node so it can inspect the end of the sequence or connect additional nodes in surrounding logic.

**Expected behavior**
- The module traverses from head to the final reachable node.
- It returns that final node.

**Testing focus**
- Single-node chain returns the head as tail.
- Multi-node chain returns the last linked node.
- Returned tail must be the same logical node that terminates the chain.

## Requirements

### Functional Requirements

#### FR-1: Node-chain length calculation
The module shall provide behavior equivalent to `wsnode_len` for `struct wordsplit_node` chains in `src/wordsplit/wordsplit.c:479-488`.

**Required outcome**
- For a provided node-chain head, compute the total represented text length across all nodes in the chain.

#### FR-2: Full node-chain cleanup
The module shall provide behavior equivalent to `wsnode_free` for `struct wordsplit_node` chains in `src/wordsplit/wordsplit.c:500-506`.

**Required outcome**
- For a provided node-chain head, release all nodes reachable from that head and any node-owned text storage covered by the original C behavior.

#### FR-3: Tail-node retrieval
The module shall provide behavior equivalent to `wsnode_tail` for `struct wordsplit_node` chains in `src/wordsplit/wordsplit.c:544-550`.

**Required outcome**
- For a provided node-chain head, identify and return the last reachable node in the chain.

#### FR-4: Preserve `wordsplit` subsystem compatibility
The Rust rewrite shall preserve the node utility behavior expected by the surrounding `wordsplit` subsystem context evidenced by references to `struct wordsplit` and `struct wordsplit_node` in `src/wordsplit/wordsplit.c`.

**Required outcome**
- The Rust module integrates with the same logical relationships between word-splitting state and node chains, without expanding the module’s responsibilities beyond the evidenced node utilities.

### Key Entities

#### `wordsplit_node`
A linked-list node type used by the `wordsplit` subsystem.

**Role**
- Represents one element in a chain of word-splitting text fragments or segments.
- Participates in traversal for aggregate length calculation.
- Participates in ownership-based cleanup of the full chain.
- Participates in tail discovery through next-node linkage.

**Relationships**
- Nodes are linked into a linear chain.
- A chain is addressed by its head node.
- The tail is the last node with no further successor.
- Node data includes text content or text-associated storage whose length contributes to aggregate length and whose storage is released during cleanup.

#### `wordsplit`
The broader subsystem state structure referenced around these node operations.

**Role**
- Provides surrounding ownership and processing context in which node chains are created, traversed, and destroyed.
- Establishes that node utilities are internal support behavior for the word-splitting system rather than standalone general-purpose collections.

## Success Criteria

### SC-1: Correct aggregate length
For representative `wordsplit_node` chains used in tests, the Rust implementation returns the same total text length as the C behavior defined by `wsnode_len`.

### SC-2: Correct full-chain destruction
For representative `wordsplit_node` chains used in tests, destroying a chain in the Rust implementation releases the full chain scope covered by `wsnode_free`, with no retained reachable nodes from the destroyed head.

### SC-3: Correct tail identification
For representative single-node and multi-node chains used in tests, the Rust implementation returns the same logical tail node as the C behavior defined by `wsnode_tail`.

### SC-4: No scope expansion beyond evidenced behavior
The Rust rewrite implements the node-chain utility behavior evidenced in `src/wordsplit/wordsplit.c` and does not require unrelated new features or APIs to satisfy this module specification.