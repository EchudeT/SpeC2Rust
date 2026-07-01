# Implementation Plan: module_src_wordsplit_wordsplit_02

## Summary

This module ports the node and segment handling portion of `src/wordsplit/wordsplit.c` into Rust, with focus on preserving the existing parsing and environment-substitution behavior of the listed functions without adding new capabilities.

The Rust implementation should keep the original operational model:

- maintain a mutable sequence of word-split nodes/segments,
- support insertion, coalescing, prefix splitting, quote removal, and finalization,
- perform environment lookup and assignment through explicit helper functions,
- replace manual allocation/free logic with ownership-based Rust containers.

Technical approach:

- migrate the C node list logic into a Rust-owned linked representation or index-based sequence internal to a single module,
- convert C string handling into `String`/`Vec<u8>`-backed processing as needed by exact behavior,
- encode nullable pointers and flags with `Option`, enums, and explicit state fields,
- keep function boundaries close to the C functions to reduce migration risk and simplify behavior matching,
- use `Result` only where the C code reports failure; otherwise preserve in-place mutation patterns.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required based on current evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - preserve linear-time behavior for traversal-heavy operations where the C code walks node chains,
  - avoid unnecessary string cloning during segment insertion/coalescing,
  - keep finalization and quote-removal passes proportional to node/segment count,
  - match existing module-scale performance characteristics rather than introduce new abstractions with hidden overhead.

## Module Mapping

### Source File Mapping

- **C source**
  - `src/wordsplit/wordsplit.c`
- **Rust target**
  - `src/wordsplit/wordsplit.rs`

If the Rust project already exposes a `wordsplit` module, this port should remain inside that existing module tree and not be split further unless required by the current crate layout.

### Function Mapping

The Rust port should keep function scope and naming close to the C implementation, with idiomatic signature adjustments only where ownership or borrowing requires it.

| C Function | Rust Target | Notes |
|---|---|---|
| `wsnode_insert` | `fn wsnode_insert(...)` | Mutates node sequence; replace pointer rewiring with owned/indexed mutation. |
| `wordsplit_add_segm` | `fn wordsplit_add_segm(...)` | Adds a segment/node into the current sequence. |
| `wordsplit_free_nodes` | `fn wordsplit_free_nodes(...)` or removed in favor of `Drop` | Explicit free logic should collapse into owned cleanup; keep helper only if call sites depend on reset semantics. |
| `wordsplit_dump_nodes` | `fn wordsplit_dump_nodes(...)` | Debug/helper output retained only as internal diagnostic behavior if required by existing tests. |
| `coalesce_segment` | `fn coalesce_segment(...)` | Local helper for merging adjacent text segments. |
| `wsnode_quoteremoval` | `fn wsnode_quoteremoval(...)` | In-place quote removal over node contents. |
| `wsnode_coalesce` | `fn wsnode_coalesce(...)` | Merge compatible nodes. |
| `wsnode_tail_coalesce` | `fn wsnode_tail_coalesce(...)` | Tail-focused merge helper. |
| `wordsplit_finish` | `fn wordsplit_finish(...) -> Result<..., ...>` | Final pass over node sequence; exact return shape depends on surrounding crate API. |
| `node_split_prefix` | `fn node_split_prefix(...)` | Splits one node into prefix/remainder representation. |
| `wsplt_env_find` | `fn wsplt_env_find(...)` | Internal search helper over environment storage. |
| `wsplt_env_lookup` | `fn wsplt_env_lookup(...)` | Lookup API preserving current semantics. |
| `wsplt_env_getvar` | `fn wsplt_env_getvar(...)` | Reads variable value from configured source. |
| `wsplt_assign_var` | `fn wsplt_assign_var(...)` | Assignment helper for variable table/environment state. |
| `wsplt_assign_param` | `fn wsplt_assign_param(...)` | Assignment helper for positional or parameter-like state. |

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust data model should be derived directly from the fields actually used by the listed functions, rather than introducing speculative types. The mapping below defines the expected Rust representation strategy.

### Core Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Raw owning pointer to node | `Box<Node>` or element in `Vec<Node>` | Prefer the simplest form that matches existing traversal/update needs. |
| Nullable pointer | `Option<T>` / `Option<usize>` | Use indices if using a vector-backed sequence. |
| Mutable linked list links | explicit `next`/`prev` fields or ordered `Vec` | Choose based on how much insertion/splitting logic depends on O(1) local rewiring. |
| C string pointer | `String` | Use for owned UTF-8 text when valid text semantics are already assumed. |
| Mutable byte buffer | `Vec<u8>` | Use if exact byte-preserving shell-like operations are required. |
| Flag fields / bit masks | `u32` newtype or `bitflags`-like plain constants | Prefer plain constants in std-only implementation. |
| Integer status/error code | `Result<T, WordSplitError>` or small status enum | Keep close to existing error branches. |
| External environment array/table | `Vec<EnvEntry>` / `HashMap<String, String>` only if ordering is irrelevant | Prefer ordered representation first if C code searches sequentially. |

### Planned Rust Types

These are implementation-facing types intended to replace the anonymous C structs used by the listed functions.

#### `WordSplitNode`

Represents one node/segment in the working sequence.

Planned fields:

- node kind/category
- owned text payload
- flags copied from C behavior
- link or position bookkeeping needed for insertion/coalescing

Candidate shape:

```rust
struct WordSplitNode {
    kind: NodeKind,
    text: Vec<u8>,
    flags: u32,
    next: Option<usize>,
    prev: Option<usize>,
}
```

If a `Vec`-ordered model is simpler after examining neighboring code, `next`/`prev` should be removed and adjacency inferred by index.

#### `NodeKind`

Replaces integer/discriminator usage in C.

```rust
enum NodeKind {
    Text,
    Segment,
    Delimiter,
    // only variants proven by the source should be introduced
}
```

Only variants evidenced by the original fields and branch conditions should be added during migration.

#### `WordSplitState`

Represents the mutable parser/splitter state touched by node and environment functions.

Planned contents:

- node storage/head/tail
- environment/parameter storage
- configuration flags
- accumulated output or completion state used by `wordsplit_finish`

Candidate shape:

```rust
struct WordSplitState {
    nodes: Vec<WordSplitNode>,
    head: Option<usize>,
    tail: Option<usize>,
    env: Vec<EnvEntry>,
    params: Vec<ParamEntry>,
    flags: u32,
}
```

This should be aligned to the actual C state object already present in the wider port and not duplicated if such a struct already exists.

#### `EnvEntry`

Replaces anonymous environment binding records.

```rust
struct EnvEntry {
    name: String,
    value: String,
}
```

Use sequential storage first if `wsplt_env_find` behavior depends on first-match order.

#### `ParamEntry`

Represents assignable parameter-style storage if distinct from normal variables.

```rust
struct ParamEntry {
    key: String,
    value: String,
}
```

If the C code uses the same storage form for both variable and parameter assignment, these should be unified instead of kept separate.

#### `WordSplitError`

Replaces integer error/status propagation.

```rust
enum WordSplitError {
    InvalidState,
    AllocationLikeFailure,
    LookupFailure,
}
```

Only concrete variants justified by existing C return paths should be implemented.

### Memory Management Notes

- `wordsplit_free_nodes` should become state clearing logic rather than manual deallocation.
- Ownership of node text and environment values should be explicit and singular.
- Borrowing rules will likely require short-lived mutable operations when splitting or coalescing nodes; helper functions should take indices or narrow mutable references to avoid aliasing conflicts.
- Any C logic depending on freed-node invalidation should be replaced by immediate removal from the container or by marking and compacting in a controlled pass.

### Error Handling Notes

- C functions returning status codes should map to `Result<(), WordSplitError>` or `Option<_>` depending on whether the C branch distinguishes error from absence.
- Internal helpers that cannot fail after invariant checks may remain private and panic-free by returning `Option` for structural lookup.
- Environment lookup should distinguish “not found” from actual failure if the C code does.

## Implementation Phases

## Phase 1: Establish Rust State and Node Representations

Scope:

- create the Rust module file for this port target,
- define the state, node, flag, and environment data structures required by the listed functions,
- map C ownership and list relationships into Rust-owned storage,
- introduce minimal error/status types required by the migrated functions.

Functions covered in this phase:

- structural support for all listed functions
- `wordsplit_free_nodes`
- `wsplt_env_find`
- `wsplt_env_lookup`
- `wsplt_env_getvar`

Technical decisions:

- inspect the existing C fields used by these functions and define only those fields in Rust,
- prefer a single internal node storage strategy across the module,
- keep environment representation simple and compatible with sequential lookup semantics.

Exit criteria:

- Rust types compile,
- node cleanup/reset behavior is implemented,
- environment search and retrieval helpers behave equivalently to the C logic.

## Phase 2: Port Node Mutation and Segment Operations

Scope:

- port list/sequence mutation operations,
- preserve insertion, splitting, and merge behavior,
- ensure string/byte ownership remains local and safe during in-place transformation.

Functions covered in this phase:

- `wsnode_insert`
- `wordsplit_add_segm`
- `coalesce_segment`
- `node_split_prefix`
- `wsnode_coalesce`
- `wsnode_tail_coalesce`

Technical decisions:

- keep helper boundaries close to the C code so mutation order stays recognizable,
- use indices or controlled mutable borrowing to avoid aliasing when adjacent nodes are updated,
- defer any nonessential refactoring until after behavior parity is confirmed.

Exit criteria:

- segment insertion and local rewrite operations compile,
- adjacent-node coalescing matches C ordering and flag propagation,
- prefix-split behavior preserves text boundaries and subsequent traversal validity.

## Phase 3: Port Quote Removal, Assignment, and Finish Pass

Scope:

- migrate node content normalization and final completion logic,
- port variable/parameter assignment helpers,
- connect environment and node processing paths into the final module behavior.

Functions covered in this phase:

- `wsnode_quoteremoval`
- `wsplt_assign_var`
- `wsplt_assign_param`
- `wordsplit_finish`
- `wordsplit_dump_nodes` if still required for existing diagnostics/tests

Technical decisions:

- implement quote removal as explicit byte/text transformation matching existing semantics,
- keep assignment side effects limited to the existing state object,
- make `wordsplit_finish` the point where remaining node normalization/coalescing assumptions are enforced.

Exit criteria:

- finish pass produces the same observable output/state transitions as the C implementation,
- assignment helpers integrate with lookup logic,
- optional dump/debug helper reflects current Rust state accurately.

## Phase 4: Verification and Cleanup of Migration Boundaries

Scope:

- add focused tests for migrated behavior,
- remove obsolete manual-memory assumptions,
- tighten signatures and invariants only where needed to support safe Rust.

Testing targets:

- node insertion and ordering,
- segment coalescing edge cases,
- prefix splitting,
- quote removal behavior,
- environment lookup and assignment precedence,
- finish-pass state cleanup/final output behavior.

Technical decisions:

- use table-driven unit tests in the same module or its corresponding test module,
- prefer direct fixture construction over introducing new test infrastructure,
- retain public/private visibility consistent with the rest of the crate.

Exit criteria:

- `cargo test` passes,
- no manual-free patterns remain in the ported logic,
- migrated functions are integrated without adding extra module surface.