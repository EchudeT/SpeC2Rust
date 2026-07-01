# Implementation Plan: module_src_wordsplit_wordsplit_node_06

## Summary

This module migration covers the node-level helper logic in `src/wordsplit/wordsplit.c`, specifically the functions `wsnode_len`, `wsnode_free`, and `wsnode_tail`. The Rust implementation should preserve the existing linked-node traversal and destruction behavior without introducing broader parser or tokenizer redesign.

The technical approach is to port the relevant node representation and these helper functions into a focused Rust module that uses standard-library ownership and borrowing to replace manual memory management. Traversal operations should be implemented with safe references where possible, while node destruction should rely on Rust drop semantics and explicit ownership-consuming helpers instead of C-style recursive/manual free patterns.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve linear-time behavior for list traversal operations.
  - Avoid unnecessary allocation during `len` and `tail` operations.
  - Ensure node cleanup is ownership-driven and does not add extra passes beyond what existing destruction semantics require.

## Module Mapping

- **C source file**: `src/wordsplit/wordsplit.c`
- **Rust target module**: `src/wordsplit.rs` or `src/wordsplit/mod.rs`

Recommended restrained mapping:
- Port the node representation used by `wsnode_len`, `wsnode_free`, and `wsnode_tail` into the existing or newly created Rust `wordsplit` module.
- Implement direct Rust equivalents for:
  - `wsnode_len`
  - `wsnode_tail`
  - `wsnode_free`

If the Rust branch already contains a `wordsplit` module, place the migrated logic there rather than creating extra submodules solely for this node subset.

## Data Model

Because the analysis identifies only anonymous C structures, the Rust plan should infer and isolate only the structure shape required by the three functions.

### C-to-Rust structure mapping

| C concept | Rust mapping | Notes |
|---|---|---|
| anonymous node struct used by `wsnode_len` / `wsnode_tail` / `wsnode_free` | `struct WsNode` | Represent the linked list node directly. |
| `next` pointer chain | `Option<Box<WsNode>>` | Standard ownership model for singly linked lists. |
| nullable node pointer input | `Option<&WsNode>` / `Option<&mut WsNode>` / `Option<Box<WsNode>>` depending on function | Match each function’s access pattern. |
| manually freed heap nodes | ownership-consuming Rust value | Freeing becomes drop-driven once ownership is consumed. |

### Proposed Rust shape

The exact fields should be limited to those needed by the surrounding migrated code. At minimum, the node must expose its link field:

```rust
struct WsNode {
    next: Option<Box<WsNode>>,
    // other migrated fields from the original node struct, only as required
}
```

### Function-level ownership mapping

- **`wsnode_len`**
  C behavior likely accepts a nullable head pointer and counts linked nodes.
  Rust mapping:
  - Input: `Option<&WsNode>`
  - Output: `usize`

- **`wsnode_tail`**
  C behavior likely walks to the final node in the chain.
  Rust mapping:
  - Input: `Option<&WsNode>` or `Option<&mut WsNode>` based on caller needs
  - Output: `Option<&WsNode>` or `Option<&mut WsNode>`
  - Prefer separate immutable/mutable internal helpers only if current call sites require both; otherwise keep one minimal variant.

- **`wsnode_free`**
  C behavior frees an entire node chain.
  Rust mapping:
  - Input: `Option<Box<WsNode>>`
  - Output: none
  - Implementation should consume ownership and let recursive/iterative drop release the chain.
  - If deep lists make recursive drop undesirable in the migrated code path, implement an iterative detach loop during free; only do this if the original list depth or surrounding code suggests stack-sensitive behavior.

## Implementation Phases

## Phase 1: Recover the node shape and place the Rust module

- Inspect `src/wordsplit/wordsplit.c` for the exact anonymous struct definition actually used by these functions.
- Identify the minimum field set needed for this node subset, especially the next-link field and any data fields whose ownership affects destruction.
- Add or update the Rust `wordsplit` module in the branch’s standard project layout.
- Define `WsNode` with only the fields required by already-migrated or immediately adjacent code.
- Decide whether any payload fields need owned Rust types (`String`, `Vec<u8>`, enums) versus borrowed forms, based strictly on current C usage in this module area.

## Phase 2: Port traversal helpers

- Implement `wsnode_len` as a simple iterative traversal over the `next` chain.
- Implement `wsnode_tail` as a traversal returning the last reachable node.
- Keep nullability behavior aligned with C by using `Option` inputs and outputs.
- Match existing semantics for empty-list handling exactly.
- Avoid cloning nodes or reallocating list contents during traversal.

## Phase 3: Port destruction semantics

- Implement `wsnode_free` as an ownership-consuming function for the whole chain.
- Replace manual `free` logic with Rust drop semantics for payload fields and `Box` links.
- If payload cleanup in C performs more than plain deallocation, encode that behavior in `Drop` only when strictly necessary; otherwise rely on derived destruction.
- Verify that freeing a partial or full chain does not require extra sentinel handling beyond the original code.

## Phase 4: Validate behavior and integrate call sites

- Update any direct call sites in the Rust port to use the new function signatures and ownership model.
- Add unit tests covering:
  - empty list length
  - single-node length and tail
  - multi-node length and tail
  - free/ownership consumption of empty and non-empty chains
- Run `cargo test` and confirm no semantic drift in node traversal and cleanup behavior.
- Perform a final review to ensure no extra abstractions or support layers were introduced beyond this module migration.

## Notes on Memory Management and Error Handling

- Prefer total functions over raw-pointer assumptions by expressing absent nodes with `Option`.
- These functions do not appear to require fallible APIs; avoid introducing `Result` unless a migrated payload conversion makes failure explicit.
- Replace C manual free discipline with Rust ownership transfer:
  - traversal helpers borrow
  - free helper consumes
- Do not introduce shared ownership (`Rc`, `Arc`) or interior mutability unless the existing Rust branch already requires it for this exact node structure.