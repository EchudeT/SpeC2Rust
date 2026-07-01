# Implementation Plan

## Summary

Port the GNU-hash entry handling logic from `gnu/hash.c` into a Rust module that preserves the current behavior of:

- conditional insertion via `hash_insert_if_absent`
- entry removal via `hash_remove`
- state/output traversal via `hash_print`

The Rust implementation should stay narrowly aligned with the existing C module rather than redesigning the subsystem. The technical approach is to translate the current hash-table data and helper logic into owned Rust data structures using the standard library, with explicit handling for:

- bucket and entry ownership
- collision-chain traversal
- insert-if-missing semantics
- safe removal without manual deallocation
- print/output behavior through Rust formatting traits or writer-based functions

Because the C analysis only identifies anonymous data structures, the Rust plan should begin by reconstructing the concrete shapes used by these functions directly from `gnu/hash.c`, then define minimal Rust equivalents in the same module scope needed to support the three migrated functions.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended at this stage, since the input provides no evidence that external hashing, printing, or memory-management crates are required
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve expected constant-time average behavior for lookup, insert-if-absent, and removal
  - Avoid unnecessary cloning of keys or payload data beyond what is required by ownership rules
  - Keep traversal and print behavior linear in the number of stored entries
  - Maintain memory usage in line with the original bucket/chain-based representation, without introducing extra indexing layers

## Module Mapping

### Source Mapping

| C Source File | Rust Target |
|---|---|
| `gnu/hash.c` | `src/gnu/hash.rs` |

### Function Mapping

| C Function | Rust Function |
|---|---|
| `hash_insert_if_absent` | `hash_insert_if_absent` |
| `hash_remove` | `hash_remove` |
| `hash_print` | `hash_print` |

### Rust Module Placement

Follow standard Rust project layout with a direct module translation:

- `src/gnu/mod.rs`
- `src/gnu/hash.rs`

If the current crate already has a `gnu` namespace, add `hash.rs` to that existing module tree rather than introducing a new architectural layer.

## Data Model

The C analysis reports only anonymous structures, so the first implementation task is to recover the actual struct boundaries and field roles from `gnu/hash.c`. The Rust data model should be a direct structural translation of only the types used by the three target functions.

### Mapping Strategy

| C Pattern | Rust Mapping |
|---|---|
| Anonymous struct representing hash table state | Named `struct HashTable` |
| Anonymous struct representing a node/entry in a collision chain | Named `struct HashEntry` |
| Raw pointers linking entries | `Option<Box<HashEntry>>` for owned next-link chains, or index-based links if the C layout makes that simpler to preserve |
| Bucket array managed manually | `Vec<Option<Box<HashEntry>>>` or equivalent contiguous bucket storage |
| Function-local pointer traversal variables | Mutable references / `Option` traversal via `as_mut`, `take`, and pattern matching |
| C string pointers or borrowed key references | `String`, `Vec<u8>`, or borrowed slices, chosen strictly based on the actual field usage in `gnu/hash.c` |
| C integer status returns | `bool`, `Option<T>`, or `Result<T, E>` depending on whether the original function distinguishes success, absence, and error separately |

### Expected Core Rust Structures

The exact fields must be confirmed from the source, but the implementation should converge on minimal shapes such as:

```rust
struct HashTable {
    buckets: Vec<Option<Box<HashEntry>>>,
    len: usize,
    // additional migrated fields only if present in gnu/hash.c
}

struct HashEntry {
    // key/data fields translated directly from the C struct
    next: Option<Box<HashEntry>>,
}
```

### Memory Management Decisions

- Replace manual allocation/free in C with Rust ownership through `Box` and `Vec`.
- Use `Option::take` and link replacement for safe removal from singly linked bucket chains.
- Do not preserve raw-pointer lifetime behavior unless the original API requires external aliasing; prefer internal ownership contained by the table.
- If `hash_print` only reads state, expose immutable borrowing rather than copying entry contents.

### Error Handling Decisions

- If insertion can fail only due to allocation, Rust can rely on standard allocation behavior without introducing custom recovery paths.
- If the C function reports “already present” distinctly from “inserted”, model that with a return type such as `bool` or a small enum.
- If `hash_remove` reports whether an item was found, return a presence indicator rather than silently ignoring misses.
- Avoid introducing new error categories not evidenced by the C implementation.

## Implementation Phases

## Phase 1: Recover C Shapes and Establish Rust Module Skeleton

- Inspect `gnu/hash.c` and identify:
  - the concrete anonymous structs used by the hash module
  - the table/entry field layout
  - the key comparison and hash usage patterns
  - the exact return-value semantics of the three target functions
- Create `src/gnu/hash.rs` with:
  - named Rust structs replacing the anonymous C structures
  - placeholder signatures for:
    - `hash_insert_if_absent`
    - `hash_remove`
    - `hash_print`
- Add `src/gnu/mod.rs` wiring only as needed for this file migration.
- Keep names close to the C source to simplify review and cross-reference.

## Phase 2: Port Core Storage and Mutation Logic

- Implement the table and collision-chain representation in Rust using `Vec` plus owned entry links.
- Port `hash_insert_if_absent` first:
  - reproduce bucket selection and chain traversal order from the C code
  - preserve duplicate-detection behavior
  - insert new entries at the same logical position used by the C implementation
- Port `hash_remove` second:
  - preserve matching semantics
  - unlink entries safely using `Option` manipulation instead of manual pointer reassignment/free
  - update table metadata consistently if the C code tracks count or occupancy
- Keep helper logic local to `src/gnu/hash.rs`; do not split into additional files unless the existing Rust crate already requires it for consistency.

## Phase 3: Port Print Logic and Stabilize Public API

- Implement `hash_print` using Rust formatting/output primitives while preserving:
  - traversal order
  - formatting structure
  - omission/inclusion rules visible in the C code
- Prefer accepting a generic writer if the C version prints to a stream-like target; otherwise use the narrowest equivalent that matches existing call sites.
- Verify that the Rust function signatures expose no unnecessary ownership transfer for print-only operations.
- Align naming and visibility with the current crate conventions, but avoid introducing broader abstractions.

## Phase 4: Validation and Behavior Lock-In

- Add `cargo test` coverage for the migrated behavior:
  - insert into empty table
  - insert duplicate entry and verify “absent” semantics
  - remove existing entry from:
    - single-entry bucket
    - head of collision chain
    - middle/tail of collision chain
  - remove missing entry
  - print output for empty and populated tables
- Where possible, derive tests directly from observable behavior in `gnu/hash.c` rather than inventing new semantics.
- Review for:
  - absence of unsafe code unless forced by an exact layout dependency
  - correct ownership and no detached chain segments
  - no extra module growth beyond the migrated file scope