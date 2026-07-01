# Implementation Plan: module_src_wordsplit_wordsplit_node_06

## Summary

This module covers migration of the node-oriented helper logic in `src/wordsplit/wordsplit.c` for:

- `wsnode_len`
- `wsnode_free`
- `wsnode_tail`

The Rust implementation should preserve the existing list/node behavior and ownership model while replacing manual C memory handling with Rust ownership and borrowing. The scope is intentionally narrow: port only the functionality required for node traversal, node length calculation, tail lookup, and node destruction as represented by these functions.

The technical approach is to isolate the corresponding node representation inside the Rust wordsplit module and implement direct equivalents of the three C functions as internal Rust functions or methods. Linked-structure traversal should remain explicit and simple, using `Option<Box<Node>>` or borrowed links as appropriate. Memory release should be expressed through ownership drop semantics rather than custom deallocation logic, while still preserving any recursive or iterative destruction ordering needed by the original code.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the asymptotic behavior of the C implementation for all three functions.
  - Keep `wsnode_len` and `wsnode_tail` as linear traversals over the node chain.
  - Ensure `wsnode_free` performs deterministic destruction without additional retained allocations.
  - Avoid unnecessary cloning or intermediate collections during traversal and teardown.

## Module Mapping

### C to Rust File Mapping

- **C source**: `src/wordsplit/wordsplit.c`
- **Rust target**: `src/wordsplit.rs` or `src/wordsplit/mod.rs`

If the project already places wordsplit logic in a dedicated Rust module, these functions should be migrated into that same file rather than split into new helper modules. The node helpers should stay adjacent to the Rust representation of the node structure they operate on.

### Function Mapping

- `wsnode_len`
  - Rust mapping: internal function or `impl` method returning `usize`
  - Responsibility: count nodes in a linked sequence without mutation

- `wsnode_tail`
  - Rust mapping: internal function or `impl` method returning a reference, mutable reference, or link handle to the last node, depending on surrounding ownership design
  - Responsibility: traverse to the terminal node of the chain

- `wsnode_free`
  - Rust mapping: ownership-consuming function or implicit drop path
  - Responsibility: release the full node chain and any node-owned payloads through Rust drop semantics

## Data Model

The C analysis lists only anonymous data structures, so the exact struct names are not available from the input. The migration plan should therefore derive the Rust types from the actual node definitions used by these functions inside `wordsplit.c`, but keep the translation minimal and local.

### Expected Structural Mapping

Because these functions operate on node chains, the relevant C structure is expected to be a self-linked node record. Map it as follows:

- **C anonymous node-like struct**
  - **Rust**: named private struct such as `WsNode`
  - **Fields**:
    - payload/value fields: mapped to direct Rust field types based on the original C field types
    - next pointer: `Option<Box<WsNode>>` for owned forward links, or another minimal ownership-preserving representation if the surrounding port already established one

### Pointer and Ownership Mapping

- `NULL` next pointers -> `Option::None`
- owned heap node pointers -> `Box<T>`
- borrowed traversal pointers -> `&T` / `&mut T`
- integer length/count results -> `usize`
- C manual free logic -> Rust `Drop`-driven release through ownership consumption

### Memory Management Notes

- Prefer making the node chain singly owned so dropping the head drops the full list automatically.
- If `wsnode_free` in C performs cleanup of nested heap fields before freeing each node, those fields must become owned Rust fields so cleanup remains automatic.
- If recursive drop depth could become a concern due to long chains, implement `wsnode_free` as an explicit iterative unlink loop; otherwise rely on standard ownership drop if consistent with the broader module design.
- Do not introduce reference counting or shared ownership unless the existing translated structure already requires it.

### Error Handling Notes

These functions appear to be structural helpers rather than fallible operations.

- `wsnode_len`: should be infallible
- `wsnode_tail`: should return `Option<_>` when the input chain may be empty
- `wsnode_free`: should be infallible and consume ownership

Avoid adding custom error types unless required by the existing Rust port around `wordsplit`.

## Implementation Phases

### Phase 1: Recover and Define the Node Representation

- Inspect `src/wordsplit/wordsplit.c` to identify the exact anonymous struct used by `wsnode_len`, `wsnode_free`, and `wsnode_tail`.
- Define the corresponding Rust node struct in the existing wordsplit module.
- Map only the fields required by these functions and their direct cleanup responsibilities.
- Establish the minimal ownership model for the `next` link and any node-owned payload data.
- Confirm whether empty-list inputs are valid for each function and reflect that in function signatures.

### Phase 2: Port Traversal Helpers

- Implement the Rust equivalent of `wsnode_len`.
- Implement the Rust equivalent of `wsnode_tail`.
- Keep traversal logic close to the C control flow to simplify validation.
- Use borrowing for read-only traversal and mutable borrowing only if required by the selected tail-return form.
- Verify that signatures align with how the surrounding Rust port will call these helpers.

### Phase 3: Port Destruction Logic

- Implement the Rust equivalent of `wsnode_free`.
- Prefer ownership consumption so node release is automatic.
- If the C function manually frees nested allocations per node, encode those allocations as owned Rust fields and let drop semantics handle them.
- If explicit iterative destruction is needed to match safety or stack-usage expectations, implement a loop that repeatedly detaches `next` and advances.
- Ensure no unsafe code is introduced unless the surrounding port already mandates it.

### Phase 4: Validation and Cleanup

- Add focused unit tests covering:
  - empty chain behavior
  - single-node chain behavior
  - multi-node length counting
  - tail selection on multi-node chains
  - full-chain destruction through ownership consumption
- Run `cargo test`.
- Remove any temporary compatibility code introduced during the port if it is no longer needed.
- Confirm the final Rust implementation remains limited to the migrated C functionality and does not add new node utilities.

## Acceptance Notes

- The Rust module contains direct equivalents for `wsnode_len`, `wsnode_tail`, and `wsnode_free`.
- The node chain representation is local to the existing wordsplit module and does not introduce unnecessary abstraction layers.
- Ownership and drop semantics replace manual C deallocation without changing the intended behavior of the node helpers.
- Tests validate the migrated behavior using standard `cargo test`.