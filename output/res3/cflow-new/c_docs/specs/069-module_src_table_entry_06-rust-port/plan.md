# Implementation Plan: module_src_table_entry_06

## Summary

This module ports the symbol-table entry handling logic from `src/symbol.c` into Rust, preserving the existing responsibilities around hashing, comparison, lookup, insertion, unlinking, targeted cleanup, and parameter-deletion traversal.

The Rust implementation should keep the module narrowly aligned with the current C behavior:

- represent symbol records and table ownership explicitly,
- replace manual allocation/free patterns with ownership-based memory management,
- translate hash/compare callbacks into Rust methods or helper functions tied to the table key type,
- preserve mutation order for install/unlink/delete flows,
- keep traversal-based cleanup logic local to the module rather than introducing new abstractions.

The technical approach is to migrate the C file into a single Rust module that uses standard-library collections where they can preserve intended semantics, with explicit handling for cases where removal order or pointer-linked behavior must be mirrored. The design should prefer `HashMap` plus owned symbol records unless the original code depends on stable node relationships, in which case a `Vec`/linked-index style internal representation can be used inside the same module without expanding scope.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library (`std::collections`, `std::mem`, `std::hash`, `std::rc`/`std::cell` only if required by the original mutation pattern)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve expected constant-time average lookup/insert behavior for symbol access,
  - avoid unnecessary string cloning beyond ownership boundaries required by Rust,
  - keep deletion and cleanup passes linear in the number of affected entries,
  - match the original module’s practical runtime profile without introducing heavier synchronization or indirection.

## Module Mapping

### Source Mapping

- **C source**: `src/symbol.c`
- **Rust target**: `src/symbol.rs`

### Function Mapping

| C Function | Rust Mapping | Notes |
|---|---|---|
| `hash_symbol_hasher` | `fn hash_symbol_hasher(...) -> u64` or internal key-hash helper | Keep local to module; only expose if needed by neighboring migrated code. |
| `hash_symbol_compare` | `fn hash_symbol_compare(...) -> bool` or `Ord/Eq` implementation support | Prefer idiomatic equality on key fields instead of callback-style comparison. |
| `lookup` | `pub(crate) fn lookup(...) -> Option<&Symbol>` / `Option<SymbolId>` | Return borrowed entry or internal handle instead of raw pointer. |
| `install` | `pub(crate) fn install(...) -> Result<..., ...>` or direct insertion method | Preserve overwrite/non-overwrite behavior from C. |
| `unlink_symbol` | `pub(crate) fn unlink_symbol(...) -> Option<Symbol>` | Represent removal explicitly and let ownership drive drop behavior. |
| `static_free` | `fn static_free(...)` or eliminated into `Drop`/owned cleanup | Keep only if there is a distinct partial cleanup path to preserve. |
| `collect_processor` | `fn collect_processor(...)` | Keep as internal traversal helper for selective collection/deletion. |
| `delete_parms_itr` | `fn delete_parms_itr(...)` | Internal iteration-based deletion helper; preserve traversal semantics. |

### Visibility Plan

- Expose only the operations that are used across module boundaries in the current project.
- Keep hash, compare, traversal, and cleanup helpers private unless required by existing call sites.
- Do not split this migration into extra submodules unless forced by existing Rust project layout.

## Data Model

Because the analysis only identifies anonymous C data structures, the Rust mapping should be driven directly from the fields and usage patterns in `src/symbol.c` during implementation. The plan is to name structures by role rather than preserve anonymous forms.

### Planned Structure Mapping

| C Form | Rust Form | Mapping Rule |
|---|---|---|
| anonymous symbol record struct | `struct Symbol` | Own all string/data fields; replace nullable pointers with `Option<T>`. |
| anonymous hash-table entry/node | `struct SymbolEntry` or folded into `Symbol` storage | Keep separate only if bucket/link metadata is required. |
| anonymous symbol table container | `struct SymbolTable` | Own collection storage and provide lookup/install/unlink methods. |
| anonymous iterator/traversal state | local iterator variables or `struct DeleteParmsIter` only if state must persist | Prefer local iteration state over introducing public types. |
| anonymous processor callback payload | private helper parameter struct if needed | Use explicit typed parameters rather than `void *`-style payloads. |
| anonymous free/cleanup context | private helper context struct or direct method args | Eliminate standalone free context when ownership makes it unnecessary. |

### C-to-Rust Representation Rules

- **C strings / symbol names** -> `String` for owned text, `&str` for lookup inputs.
- **Raw pointers to owned records** -> owned values in collections; use indices or keys instead of address identity where possible.
- **Nullable links / optional related objects** -> `Option<T>` / `Option<usize>`.
- **Manual free functions** -> normal `Drop` through ownership; keep explicit cleanup helpers only for selective unlinking behavior.
- **Callback-style hash/compare logic** -> trait-based hashing/equality or internal helper functions.
- **Out-of-band error signaling** -> `Option` for not-found cases, `Result` for insertion or invariant failures.

### Ownership and Memory Management

- The table owns all symbol entries.
- Lookup returns references or stable internal identifiers, not transferable ownership unless the original unlink/delete path removes the entry.
- Unlink/delete operations return removed owned values when callers need post-removal processing.
- Any C logic that freed nested allocations in stages should be converted into nested owned fields so Rust drops them automatically in the same effective order.
- If cyclic relationships appear in the original code, prefer redesigning them as key/index references rather than `Rc` cycles.

## Implementation Phases

## Phase 1: Inventory and Type Translation

- Inspect `src/symbol.c` and identify each anonymous struct by usage role:
  - symbol payload,
  - table/bucket container,
  - traversal/delete helper state,
  - any nested parameter records.
- Create `src/symbol.rs` with Rust type definitions matching the exact field set and nullability semantics.
- Define the key type and equality/hash behavior needed by:
  - `hash_symbol_hasher`
  - `hash_symbol_compare`
- Document any places where the C code depends on pointer identity, insertion order, or bucket-local links, because these choices determine the internal collection layout.
- Add minimal compile-only tests or module tests to validate basic construction if needed during migration.

## Phase 2: Core Table Operations

- Implement the table storage using the simplest structure that preserves current semantics:
  - `HashMap` if key-based lookup/removal is sufficient,
  - internal bucket/node representation only if the C code requires link-preserving unlink behavior beyond plain map removal.
- Port:
  - `lookup`
  - `install`
  - `unlink_symbol`
- Replace raw-pointer returns with:
  - `Option<&Symbol>` / `Option<&mut Symbol>` for borrowed access, or
  - internal entry handles if mutation patterns require non-borrowing references across steps.
- Encode insertion/update behavior explicitly:
  - whether duplicate installs replace,
  - reject,
  - or return existing entries.
- Add unit tests for:
  - successful lookup after install,
  - duplicate-key behavior,
  - unlink followed by not-found lookup.

## Phase 3: Cleanup and Traversal Migration

- Port:
  - `static_free`
  - `collect_processor`
  - `delete_parms_itr`
- Remove manual deallocation logic where ownership already guarantees cleanup.
- Preserve explicit selective-deletion behavior where the C code traverses symbols and removes only entries matching parameter-related criteria.
- Keep traversal helpers private to the module and avoid introducing generic visitor frameworks.
- Verify that deletion during iteration is implemented safely:
  - collect keys/ids first if direct mutable iteration would violate Rust borrowing rules,
  - then remove entries in a second pass.
- Add tests for:
  - selective collection behavior,
  - parameter-related deletion,
  - no leaked or double-removed entries after cleanup paths.

## Phase 4: Integration Validation and Semantics Check

- Reconcile function signatures with the current Rust branch’s calling code and adjust visibility only where needed.
- Confirm that all call sites formerly expecting null pointers or ownership transfer are updated to use `Option`/`Result` and owned removal values.
- Review edge cases from the C implementation:
  - empty table behavior,
  - repeated unlink attempts,
  - lookup of absent symbols,
  - cleanup of partially populated entries.
- Run `cargo test` and ensure the module compiles cleanly without introducing extra support layers or unrelated refactors.
- Keep the final Rust module limited to the migrated scope of `src/symbol.c`.