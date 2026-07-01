# Implementation Plan

## Summary

Port the GNU-hash entry management logic from `gnu/hash.c` into a focused Rust module that preserves the existing responsibilities of:

- conditional insertion via `hash_insert_if_absent`
- removal via `hash_remove`
- textual output via `hash_print`

The Rust implementation should translate the current hash-table behavior into safe ownership-based code using the Rust standard library where possible, while keeping the module boundary narrow and aligned to the original C file. The main technical approach is to migrate the current table, entry, and callback-oriented logic into explicit Rust structs and methods, replacing manual allocation, pointer traversal, and null checks with `Option`, owned storage, and borrow-checked mutation.

The port should prioritize behavioral parity over redesign. If the original implementation relies on custom bucket chains or entry nodes, that structure should be mirrored directly in Rust rather than replaced with broader abstractions beyond what is required for the listed functions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from current evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve expected constant-time average-case hash insert/remove behavior
  - Avoid unnecessary cloning during insertion, removal, and printing
  - Keep allocation patterns close to the original implementation’s effective behavior
  - Maintain predictable traversal cost for bucket-chain operations used by the three migrated functions

## Module Mapping

### C to Rust File Mapping

- `gnu/hash.c` -> `src/gnu/hash.rs`

### Rust Module Placement

Use a direct Rust module layout that mirrors the original source location as closely as practical:

- `src/gnu/mod.rs`
- `src/gnu/hash.rs`

If the crate already exposes this namespace differently, integrate `hash.rs` into the existing module tree without introducing new architectural layers.

### Function Mapping

Map the C functions into Rust methods or free functions according to the data ownership implied by the original signatures:

- `hash_insert_if_absent`
  - Rust target: mutable operation on the hash-table type, likely `fn insert_if_absent(...)`
- `hash_remove`
  - Rust target: mutable operation on the hash-table type, likely `fn remove(...)`
- `hash_print`
  - Rust target: read-only operation on the hash-table type, likely `fn print(...)` or formatter helper

The exact public/private split should follow existing crate conventions, but no extra API should be introduced beyond what is needed to host the migrated behavior.

## Data Model

The analysis only reports multiple anonymous C data structures. The Rust mapping should therefore be driven by actual usage inside `gnu/hash.c`, not by inventing new model layers.

### Expected Mapping Strategy

Anonymous C structs used for GNU-hash table state and node storage should be converted into named Rust types with file-local visibility unless required elsewhere.

Typical mapping pattern:

- anonymous table state struct -> `struct HashTable`
- anonymous entry/node struct -> `struct HashEntry`
- anonymous bucket/link struct -> `struct Bucket` or folded into `Vec<Option<...>>`
- anonymous print/helper state structs -> private helper structs only if needed after code translation

### C-to-Rust Type Mapping Rules

- raw owning pointers -> owned Rust fields (`Box<T>`, `Vec<T>`, or direct struct ownership)
- nullable pointers -> `Option<T>` / `Option<Box<T>>` / `Option<usize>` depending on representation
- linked-list next pointers -> `Option<Box<Node>>` or index-based links if closer to the original layout
- size/count fields -> `usize`
- boolean-like integer flags -> `bool`
- function pointers used for comparison, hashing, or printing:
  - map to explicit generic parameters or stored function traits only if required by the existing implementation
  - prefer plain function pointer types from the standard library over trait-object indirection when behavior is fixed by the C design
- output stream handling for print logic:
  - if C prints to a stream/file handle, map to `&mut dyn std::io::Write` or implement formatting through `fmt::Write` only if the original usage supports it

### Memory Management Decisions

- Replace manual allocation/free paths with ownership-based drop semantics.
- Removal must return or dispose of entries in a way that clearly preserves the original ownership contract.
- Avoid interior aliasing patterns that would force `unsafe` unless the original structure cannot be expressed safely with standard ownership.
- If bucket-chain mutation requires node relinking, prefer safe iterative extraction/reinsertion techniques over raw-pointer translation.

### Error Handling Decisions

- Convert null/failed-operation return patterns into:
  - `Option<T>` for absent entries / no-op removal cases
  - `Result<T, E>` only where the C code has meaningful failure modes beyond “not found”
- For print behavior, propagate I/O failures with `std::io::Result<()>` if writing to a Rust writer is required.
- Do not introduce custom error hierarchies unless the original C logic clearly requires multiple distinct failure categories.

## Implementation Phases

## Phase 1: Analyze and Define Rust Data Structures

- Inspect `gnu/hash.c` and enumerate the actual anonymous structs participating in:
  - table ownership
  - bucket storage
  - entry chaining
  - key/value retention
  - printing support
- Name the minimal set of Rust structs needed to represent those layouts in `src/gnu/hash.rs`.
- Decide whether the table is best represented as:
  - `Vec<Option<Box<HashEntry>>>` for direct bucket-chain parity, or
  - another equally narrow standard-library structure only if it matches the C logic more closely
- Translate callback and comparison contracts into explicit Rust types.
- Establish method signatures for:
  - `insert_if_absent`
  - `remove`
  - `print`

## Phase 2: Port Mutation Logic

- Implement the Rust equivalent of `hash_insert_if_absent`.
- Implement the Rust equivalent of `hash_remove`.
- Preserve original lookup, equality, and collision-chain traversal order.
- Ensure insertion does not duplicate existing entries when the key is already present.
- Ensure removal updates bucket links correctly without leaks or dangling references.
- Keep behavior aligned with the C code’s return semantics by using `Option` or `Result` only where justified by the original function contract.

## Phase 3: Port Print Logic and Finalize API Shape

- Implement the Rust equivalent of `hash_print`.
- Translate C-side print traversal into deterministic Rust iteration over buckets and chained entries.
- Preserve output ordering and formatting rules as closely as practical from the original file.
- If printing depends on callback-based entry formatting, keep that contract narrow and local to this module.
- Finalize visibility of structs and functions so only the necessary module surface is exposed.

## Phase 4: Verification and Cleanup

- Add `cargo test` coverage for:
  - insert into empty table
  - insert of existing key
  - remove existing key
  - remove missing key
  - collision-chain insert/remove behavior
  - print traversal over empty and populated tables
- Compare edge behavior against the C implementation, especially:
  - absent-key handling
  - chain relinking after removal
  - ownership of removed entries or values
  - formatting stability
- Remove any temporary translation scaffolding and keep the final module limited to the migrated file scope.