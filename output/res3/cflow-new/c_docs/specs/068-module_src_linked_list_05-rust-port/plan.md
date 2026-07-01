# Implementation Plan: module_src_linked_list_05

## Summary

This module ports the linked-list and symbol-append behavior from `src/linked-list.c` and `src/symbol.c` into Rust with a narrow migration scope focused on preserving current behavior and call flow. The Rust implementation should replace manual pointer-based list management with standard-library ownership constructs while keeping the original responsibilities grouped around list creation, traversal/dereference, and symbol insertion.

The technical approach is to:
- translate the existing C list node/container patterns into explicit Rust structs,
- model nullable pointer relationships with `Option`,
- use heap allocation only where needed via `Box`,
- keep function boundaries aligned with the original C functions (`deref_linked_list`, `linked_list_create`, `append_symbol`),
- avoid introducing broader abstractions beyond what is required to migrate the existing files and functions.

Because the source analysis only identifies anonymous C data structures, the Rust design should be driven by the actual field layouts found in `src/linked-list.c` and `src/symbol.c`, with conservative one-to-one mappings during implementation.

## Technical Context

### Language/Version
- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies
- Rust standard library only

No third-party crate is recommended at planning time because the provided inputs do not justify external dependencies.

### Testing
- `cargo test`

Tests should focus on migrated behavior:
- list creation produces an empty or initialized list state matching the C behavior,
- dereference/traversal logic returns the expected node/value behavior,
- symbol append preserves ordering and linkage semantics.

### Performance Goals
- Preserve the asymptotic behavior of the C implementation.
- Avoid unnecessary cloning of symbol/list payloads.
- Keep append and traversal costs consistent with the original design discovered in the C code.
- Replace manual memory handling with ownership-based Rust code without introducing extra allocation layers beyond required `Box`/`Option` usage.

## Module Mapping

### C to Rust File Mapping
- `src/linked-list.c` -> `src/linked_list.rs`
- `src/symbol.c` -> `src/symbol.rs`

### Function Mapping
- `deref_linked_list` -> `linked_list::deref_linked_list`
- `linked_list_create` -> `linked_list::linked_list_create`
- `append_symbol` -> `symbol::append_symbol`

### Rust Module Integration
- Declare the migrated modules from the crate root using standard Rust module declarations only as needed:
  - `mod linked_list;`
  - `mod symbol;`

If the original C code shares internal list types across both files, define the shared type in the module that most directly owns it and expose only the minimal visibility required for `append_symbol` to use it.

## Data Model

The analysis reports only anonymous C structures, so the Rust data model must be finalized after inspecting the concrete field definitions in the two source files. The migration should follow these mapping rules.

### C Struct to Rust Struct/Enum Mapping Rules
- Anonymous C struct used as linked-list node -> named Rust `struct` such as `LinkedListNode<T>` or a concrete non-generic node type if the C implementation is type-specific.
- Anonymous C struct used as linked-list container/head -> named Rust `struct` such as `LinkedList` with explicit head/tail and any size/count fields if present in C.
- Anonymous C struct used for symbol payload -> named Rust `struct Symbol` mirroring only the fields used by `append_symbol`.
- Nullable pointer fields -> `Option<Box<T>>` for owned next-node links, or `Option<NonNull<T>>` only if the C logic truly requires non-owning internal back-references.
- C string pointers:
  - borrowed/static string semantics discovered in C -> `String` or `Option<String>` for owned migrated storage,
  - avoid raw pointer string storage unless required by unresolved surrounding APIs.
- C integral flags/counters -> matching Rust integer types (`usize`, `u32`, `i32`, etc.) based on observed usage rather than guessed width.

### Expected Ownership Strategy
- List ownership should reside in the list/container struct.
- Node chaining should use `Box` for forward links.
- `append_symbol` should mutate an existing list/symbol collection through `&mut` references instead of raw pointers.
- `deref_linked_list` should return either:
  - `Option<&T>` / `Option<&Node>` for read-only dereference behavior, or
  - `Option<&mut T>` / `Option<&mut Node>` if the original function mutates through the dereferenced pointer.

### Error Handling Mapping
- C null-pointer or allocation-failure paths -> `Option` or `Result`, chosen to match whether callers need failure detail.
- Functions that cannot fail after ownership conversion should return plain values instead of C-style sentinel states.
- Do not introduce custom error hierarchies unless the existing C control flow clearly requires structured errors.

## Implementation Phases

### Phase 1: Inspect and Name the C Data Structures
- Review `src/linked-list.c` and `src/symbol.c` to identify the anonymous struct layouts actually used by:
  - `deref_linked_list`
  - `linked_list_create`
  - `append_symbol`
- Assign stable Rust names to each required struct based on role, not guessed domain expansion.
- Determine whether the list implementation is:
  - generic in practice,
  - symbol-specific,
  - singly or doubly linked,
  - head-only or head/tail managed.
- Identify all nullability and ownership expectations in current pointer usage.
- Record exact field mappings before writing logic.

### Phase 2: Port Core Linked-List Types and Creation Logic
- Create `src/linked_list.rs`.
- Implement the Rust struct equivalents for the list container and node types required by `linked_list_create`.
- Port `linked_list_create` first, establishing the canonical initialized Rust state.
- Replace C allocation patterns with direct Rust construction.
- Encode empty-list state with `Option` rather than null pointers.
- Add unit tests for:
  - initial list state,
  - field defaults matching the C initialization behavior.

### Phase 3: Port Dereference/Traversal Behavior
- Implement `deref_linked_list` in `src/linked_list.rs`.
- Translate pointer-dereference behavior into safe borrowing where possible.
- If the original logic depends on index-like traversal or current-node access, model that explicitly with references and `Option`.
- Minimize use of `unsafe`; use it only if required by surrounding unmigrated interfaces, and isolate it to the smallest possible block.
- Add tests covering:
  - empty-list dereference behavior,
  - valid-node dereference behavior,
  - boundary/null-equivalent cases from the C implementation.

### Phase 4: Port Symbol Append Logic and Connect Modules
- Create or update `src/symbol.rs`.
- Port only the symbol structures and helper state required by `append_symbol`.
- Rework `append_symbol` to mutate Rust-owned list/symbol state via `&mut` references, preserving insertion order and linkage semantics from the C code.
- Reuse the linked-list structures from `linked_list.rs` if the original C code shares the same node/container concepts; otherwise keep the symbol-specific structure local to `symbol.rs`.
- Add tests for:
  - appending into an empty collection,
  - appending multiple symbols in order,
  - correct link updates after each append.

### Phase 5: Final Alignment and Cleanup
- Verify that all three migrated functions preserve observable behavior relative to the C implementation.
- Reduce any temporary visibility added during migration to the minimum needed across `linked_list` and `symbol`.
- Remove residual C-style patterns that are no longer needed after ownership conversion, while avoiding broader refactors.
- Run `cargo test` and fix mismatches in initialization, traversal, or append semantics.
- Confirm that memory management is fully represented by Rust ownership with no leaked or dangling list nodes.