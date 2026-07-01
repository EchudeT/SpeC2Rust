# Implementation Plan

## Summary

Port the `src/wordsplit/wordsplit.c` node-helper portion covering `wsnode_len`, `wsnode_free`, and `wsnode_tail` into a focused Rust module that preserves the existing linked-node behavior and ownership semantics without adding new capabilities.

The Rust implementation should model the original word-split node chain explicitly, using safe ownership where practical and tightly scoped internal mutation where needed for list traversal and teardown. The main technical approach is:

- represent the C word-split node structure as a Rust node type with an optional link to the next node;
- implement direct equivalents of the three functions:
  - length/count traversal,
  - tail lookup traversal,
  - list destruction via Rust ownership drop behavior or explicit iterative consumption;
- keep behavior aligned with the existing C logic, especially around null/empty-list handling and full-chain cleanup.

This migration should remain narrowly scoped to the existing file and functions, avoiding broader parser or tokenizer redesign.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time traversal characteristics for `wsnode_len` and `wsnode_tail`
  - Ensure node cleanup remains linear in the number of nodes
  - Avoid unnecessary allocations beyond those already inherent in node ownership
  - Keep implementation close to the C execution model, with no added abstraction layers that materially change runtime behavior

## Module Mapping

### C to Rust File Mapping

- `src/wordsplit/wordsplit.c`
  - migrate the relevant node-list logic into a Rust source file under standard crate layout, preferably:
    - `src/wordsplit.rs` if the project already keeps wordsplit logic in a single module, or
    - `src/wordsplit/mod.rs` if the crate already uses directory-based module organization

### Function Mapping

- `wsnode_len`
  - map to a Rust function/method that takes a node-chain reference and returns the node count
- `wsnode_tail`
  - map to a Rust function/method that traverses the chain and returns access to the last node
- `wsnode_free`
  - map to Rust-owned destruction logic, preferably an explicit helper that consumes the head of the chain; rely on normal drop behavior for recursive ownership only if it matches list size expectations and remains simple

### Visibility Guidance

Keep these items crate-private unless existing Rust call sites require wider exposure. Do not introduce public API surface beyond what the migrated code needs.

## Data Model

The analysis only identifies anonymous C structures, so the plan should treat the relevant node type as the migration target inferred from the three functions.

### Core Mapping

- C linked-list node used by `wsnode_len`, `wsnode_free`, `wsnode_tail`
  - Rust: `struct WsNode`
  - expected shape:
    - payload fields migrated only if required by these functions or nearby compile dependencies
    - `next` pointer mapped as `Option<Box<WsNode>>`

Example target form:

```rust
struct WsNode {
    // migrated payload fields only as needed
    next: Option<Box<WsNode>>,
}
```

### Pointer and Nullability Mapping

- `struct wsnode *`
  - Rust: `Option<Box<WsNode>>` for owned heads
  - Rust: `Option<&WsNode>` or `Option<&mut WsNode>` for traversal-only access
- null pointer
  - Rust: `None`

### Memory Management Mapping

- C manual free of an entire chain
  - Rust ownership-driven destruction of `Box<WsNode>` chains
- explicit free helper
  - Rust helper should consume the list head, making destruction timing obvious and matching the intent of `wsnode_free`

### Anonymous C Structures

The input lists multiple anonymous structures but does not tie them directly to these functions. Do not invent Rust counterparts for unrelated anonymous structs in this module plan. Only migrate the concrete node structure and any immediately required embedded fields referenced by the three target functions.

## Implementation Phases

### Phase 1: Establish Rust Node Representation

- Identify the exact C node struct used by `wsnode_len`, `wsnode_free`, and `wsnode_tail`
- Define the minimal Rust `WsNode` struct needed to compile these functions
- Map `next` linkage to `Option<Box<WsNode>>`
- Port any directly referenced scalar or payload fields only if required by local compilation
- Keep layout and naming close to the C source to reduce migration risk

### Phase 2: Port Traversal Functions

- Implement Rust equivalent of `wsnode_len`
  - iterate from head to tail
  - return zero for empty input
- Implement Rust equivalent of `wsnode_tail`
  - traverse until `next` is absent
  - return `None` for empty input
- Choose signatures that reflect actual usage:
  - immutable traversal for length
  - mutable or owned access for tail only if later code needs mutation
- Preserve C edge-case behavior for single-node and empty chains

### Phase 3: Port Free Logic

- Implement Rust equivalent of `wsnode_free`
  - consume the owned head and drop the full chain
- Prefer an explicit iterative consumption pattern if it keeps destruction behavior obvious and avoids deep recursive drop concerns for long lists
- Ensure no aliasing assumptions from the C code survive into Rust unsafe patterns
- Avoid unsafe code unless the surrounding ported representation makes it unavoidable

### Phase 4: Integrate and Validate

- Replace or wire the original call paths in the Rust module to use the migrated functions
- Add unit tests covering:
  - empty list
  - single-node list
  - multi-node list
  - tail selection on varying lengths
  - free/consumption behavior compiling and executing without leaks or double-drop patterns
- Run `cargo test` and adjust signatures only as needed to fit existing Rust-side call sites

## Notes and Constraints

- Keep the migration limited to the logic already present in `src/wordsplit/wordsplit.c` for these functions.
- Do not introduce new container abstractions, intrusive list utilities, or generalized memory-management helpers beyond what is required for the direct port.
- Prefer safe Rust. If surrounding code later forces raw-pointer interoperation, isolate it locally and document the ownership boundary, but do not plan such work unless it is necessary for these specific functions.
- Preserve the original semantics of empty-input handling and whole-list cleanup timing as closely as possible.