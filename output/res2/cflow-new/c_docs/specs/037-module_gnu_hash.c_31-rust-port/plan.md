# Implementation Plan

## Summary

Port `gnu/hash.c` into a single Rust module that preserves the existing hash-table behavior and lifecycle operations without adding new capabilities. The Rust implementation should focus on migrating the current entry points:

- `hash_print_statistics`
- `hash_reset_tuning`
- `raw_hasher`
- `raw_comparator`
- `check_tuning`
- `hash_initialize`
- `hash_rehash`
- `hash_insert`
- `hash_delete`

The technical approach is to replace C-managed table storage, callback handling, and resize logic with a Rust-owned table representation using standard-library allocation and explicit function-pointer based hooks where the C code expects caller-supplied hashing or comparison behavior. The port should keep the original operational model: initialize table state, validate tuning, insert/delete entries, rehash when needed, and expose statistics/reset helpers. Memory ownership rules must be made explicit in Rust so that table buckets and entries are dropped automatically, while preserving any non-owning semantics for stored payload references if the original code treats element data as externally owned.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve expected constant-time average behavior for insert, delete, and lookup-related internal operations.
  - Keep rehash behavior close to the C implementation’s tuning thresholds and growth strategy.
  - Avoid unnecessary heap allocations beyond table/bucket resizing required by the original design.
  - Maintain low-overhead callback dispatch for hashing and comparison.

## Module Mapping

- **C source file**: `gnu/hash.c`
- **Rust module file**: `src/module_gnu_hash.rs`

Recommended crate-local exposure:

- `src/module_gnu_hash.rs`
  - table state definition
  - tuning definition and validation
  - raw/default hash and compare helpers
  - initialize/rehash/insert/delete/statistics functions

If the crate already uses `mod.rs` organization, expose it with the minimal existing project convention only; do not split this port into extra submodules unless required by current repository structure.

## Data Model

The input only identifies anonymous C data structures, so the Rust design should derive a minimal set of named types directly from actual usage in `gnu/hash.c`.

### C-to-Rust structure mapping

| C shape | Rust mapping | Notes |
|---|---|---|
| anonymous hash table state struct | `struct HashTable<T>` | Owns bucket storage, tuning state, counters, and callback hooks. |
| anonymous tuning/config struct | `struct HashTuning` | Stores resize thresholds, growth/shrink factors, and policy flags if present in C. |
| anonymous bucket/entry record | `enum Bucket<T>` or `struct HashEntry<T>` | Final choice depends on whether the C implementation uses empty/deleted markers or chained entries. |
| anonymous statistics-related state | fields on `HashTable<T>` or dedicated `struct HashStats` | Keep local unless the C code exposes a separate aggregate. |
| function-pointer hash callback | `type HashFn<T> = fn(&T) -> usize` | Use plain function pointers if C behavior is fixed-function and non-capturing. |
| function-pointer comparator callback | `type CompareFn<T> = fn(&T, &T) -> bool` | Mirrors C equality callback semantics. |
| opaque item payload pointer | generic `T` or pointer-like `NonNull<c_void>` wrapper | Choose based on how stored elements are used in the source. If the table stores external pointers without ownership, represent that explicitly. |

### Ownership and memory decisions

- Bucket arrays and table metadata should be fully owned by Rust containers such as `Vec`.
- If the C table stores arbitrary client pointers rather than owned values, the Rust port should avoid inventing ownership and should model entries as non-owning pointer values or references with clear lifetime boundaries.
- Rehashing should move bucket/entry state through owned Rust memory without manual free logic.
- Deletion behavior must preserve any sentinel semantics required by the probing strategy if open addressing is used.

### Error handling mapping

- C initialization/allocation failure paths should become `Result<_, HashError>` or a narrow internal error enum.
- Operations that are total in C through preconditions should use internal validation plus documented assumptions rather than panic-driven control flow.
- Tuning validation from `check_tuning` should return explicit success/failure values rather than relying on implicit C conventions.

## Implementation Phases

### Phase 1: Port core types and table initialization

Scope:
- Create `src/module_gnu_hash.rs`.
- Define Rust equivalents for the table state, tuning state, callback types, and any bucket/entry representation required by `gnu/hash.c`.
- Port:
  - `raw_hasher`
  - `raw_comparator`
  - `check_tuning`
  - `hash_reset_tuning`
  - `hash_initialize`

Technical decisions:
- Translate anonymous C structs into named Rust structs based on field use in the source.
- Keep callback configuration close to the C API shape using function pointers rather than trait objects.
- Implement tuning defaults and validation logic first so initialization and future rehash decisions use the same constraints.
- Convert allocation/setup failure cases into `Result`.

Exit criteria:
- A table can be constructed with validated tuning and initialized bucket storage.
- Default/raw hash and compare helpers compile and are covered by unit tests.
- Tuning validation behavior matches the original acceptance/rejection boundaries.

### Phase 2: Port mutation and resizing behavior

Scope:
- Port:
  - `hash_rehash`
  - `hash_insert`
  - `hash_delete`

Technical decisions:
- Reproduce the original collision-resolution strategy exactly as found in `gnu/hash.c` rather than replacing it with `std::collections`.
- Implement resize triggers from the tuning data and preserve load-factor accounting.
- Ensure insertion and deletion update all counters needed by future rehash/statistics logic.
- Preserve any “entry already present” semantics from the C implementation instead of normalizing behavior.

Memory and correctness focus:
- Rehash must rebuild bucket placement safely without aliasing old and new storage.
- Delete must preserve probe-chain correctness if tombstones are used.
- Avoid hidden clones/copies of payload data unless the original ownership model requires them.

Exit criteria:
- Insert/delete/rehash behavior works through unit tests covering:
  - empty table insertion
  - duplicate/equivalent key handling
  - deletion of present and absent entries
  - growth-triggered rehash
  - state consistency after repeated mutations

### Phase 3: Port statistics output and finalize behavioral parity

Scope:
- Port:
  - `hash_print_statistics`
- Finalize remaining counters/auxiliary fields used by diagnostics.
- Reconcile function signatures and visibility with the rest of the Rust crate.

Technical decisions:
- Keep statistics generation narrow and faithful to current stored counters and computed values.
- If the C function prints directly, use a Rust writer-oriented implementation internally where practical, but do not broaden the public API beyond what integration requires.
- Match any integer-width-sensitive calculations using `usize`, `u64`, or other fixed-width types according to the original field roles.

Testing focus:
- Add unit tests for counter progression and statistic formatting where deterministic.
- Add regression tests for edge cases discovered during the port, especially around tuning thresholds and rehash boundaries.

Exit criteria:
- All listed functions are implemented in Rust.
- `cargo test` passes for the module port.
- The module is integrated under the project branch `037-module_gnu_hash.c_31-rust-port` with no planned expansion beyond the migrated file.