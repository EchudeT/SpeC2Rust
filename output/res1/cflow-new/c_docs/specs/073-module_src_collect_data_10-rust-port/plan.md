# Implementation Plan: module_src_collect_data_10

## Summary

This module ports the symbol collection logic from `src/symbol.c` into Rust, covering the existing `collect_list_entry` and `collect_symbols` functions without adding new behavior. The Rust implementation should preserve the current traversal and collection flow, convert C pointer-based data access into explicit borrowed or owned Rust data structures, and replace implicit memory management with standard-library containers and scoped ownership.

The implementation approach is a direct migration of the current file-level responsibilities into a Rust module that:
- keeps the collection pipeline local to one Rust source file or one tightly scoped module,
- models C records and list/link relationships with Rust structs/enums,
- rewrites null-pointer and sentinel handling into `Option`-based control flow where appropriate,
- returns explicit `Result` values only where the original logic can fail in a meaningful way; otherwise keeps total functions simple,
- preserves output ordering and filtering rules encoded in the current C functions.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates recommended based on the available input

### Testing
- `cargo test`

### Performance Goals
- Preserve the asymptotic behavior of the C implementation for symbol traversal and list collection
- Avoid unnecessary cloning of symbol names or intermediate records
- Use contiguous standard collections (`Vec`) for accumulated results unless the original logic requires linked insertion behavior
- Keep per-entry processing overhead low and avoid heap allocation beyond what is required to represent collected entries

## Module Mapping

### C to Rust File Mapping
- `src/symbol.c` -> `src/symbol.rs`

### Function Mapping
- `collect_list_entry` -> `collect_list_entry`
- `collect_symbols` -> `collect_symbols`

### Scope Notes
- Keep the Rust port focused on the code now residing in `src/symbol.c`
- Do not split behavior into additional service/helper modules unless required by Rust borrow-checking or visibility constraints
- Any helper functions introduced during the port should remain private to `src/symbol.rs`

## Data Model

The analysis input exposes only anonymous C data structures, so the Rust plan should use migration-by-role rather than renaming based on unavailable type names. The first implementation step is to identify each anonymous struct/union usage in `src/symbol.c` and assign a local Rust type name based on its function in the file.

### Mapping Strategy
- C anonymous struct used as a list node -> Rust `struct` with named fields
- C anonymous struct used as a symbol record -> Rust `struct`
- C anonymous union or tag-discriminated state -> Rust `enum` where the variants are semantically distinct
- C nullable pointer fields -> Rust `Option<T>` or `Option<Box<T>>` depending on ownership
- C borrowed references into external state -> Rust references with explicit lifetimes where feasible, otherwise indices/owned snapshots if lifetime coupling becomes too restrictive
- C integer flags -> Rust `u32`/`usize`/`i32` initially, with a later conversion to small enums or bitflag-like constants only if directly justified by the C code
- C linked-list next pointers -> Rust `Option<Box<Node>>` only if preserving in-memory list shape is necessary; otherwise collect directly into `Vec<T>`

### Planned Rust Type Categories
Because the exact C type names are unavailable, map the anonymous structures into a restrained set of file-local Rust types such as:
- `SymbolRecord`
- `CollectedEntry`
- `SymbolListNode`
- `CollectionState`
- `SymbolKind` or similar enum if the C logic distinguishes cases
- `SourceLocation` or equivalent only if the C file stores line/file metadata directly in these functions

### Ownership and Memory Management
- Replace manual allocation/free patterns with Rust ownership
- Prefer `Vec<CollectedEntry>` for accumulated output
- Use borrowed string slices (`&str`) only if the source data lifetime is clear and stable; otherwise store `String`
- Eliminate null checks by representing absent data as `Option`
- Avoid self-referential structures; if the C logic walks global or shared registries, represent those registries as borrowed slices or vectors passed into `collect_symbols`

### Error Handling
- If the original functions do not report recoverable errors, keep Rust signatures non-fallible
- If input validation or conversion from raw parsed state can fail, use `Result<_, CollectError>`
- Define a minimal module-local error type only if needed by actual control paths in the C file
- Do not introduce broad error abstraction layers

## Implementation Phases

### Phase 1: Inventory and Type Reconstruction
- Inspect `src/symbol.c` and identify all anonymous data structures touched by `collect_list_entry` and `collect_symbols`
- Assign stable Rust type names based on field usage and role in the current file
- Document exact field mappings:
  - scalar integers,
  - string/name fields,
  - next/link pointers,
  - references to enclosing symbol/context objects
- Determine whether the C functions mutate shared global state, append into lists, or return collected data
- Define Rust struct/enum skeletons in `src/symbol.rs` matching current storage needs only

**Exit criteria**
- All data touched by the two target functions has an explicit Rust representation
- Pointer ownership/borrowing decisions are documented per field
- No extra capabilities or generalized abstractions added

### Phase 2: Port `collect_list_entry`
- Translate `collect_list_entry` first as the smallest unit of collection behavior
- Preserve current filtering, field extraction, and insertion behavior
- Replace pointer/null branching with `Option`-based logic
- If the C version appends into a linked list, decide whether to:
  - keep a list-like Rust representation for fidelity, or
  - translate directly into `Vec` insertion when behavior remains equivalent
- Add focused unit tests for:
  - valid entry collection,
  - skipped/empty entries,
  - ordering behavior,
  - missing optional fields

**Exit criteria**
- `collect_list_entry` is implemented and covered by unit tests
- Entry creation and insertion semantics match the C behavior
- No unsafe Rust is used unless a direct representation need is proven unavoidable

### Phase 3: Port `collect_symbols`
- Translate the top-level traversal performed by `collect_symbols`
- Recreate iteration order over source symbol records exactly as in C
- Integrate calls to the Rust `collect_list_entry`
- Replace any C global-state dependence with explicit function parameters or module-local state only if that state already exists in the migrated file design
- Ensure the produced collection matches C behavior for:
  - traversal coverage,
  - inclusion/exclusion conditions,
  - final ordering

**Exit criteria**
- `collect_symbols` is fully implemented
- Function interactions match the original call flow
- Tests verify representative multi-entry collection cases

### Phase 4: Validation and Cleanup
- Compare Rust behavior against the C implementation using targeted fixture-style tests derived from observed C cases
- Remove temporary translation artifacts such as placeholder field names introduced during the initial pass
- Tighten types where safe:
  - convert raw integer categories into enums only where already implied by the C logic,
  - reduce unnecessary cloning,
  - simplify signatures after borrow-check validation
- Run `cargo test` and resolve any ownership or lifetime issues without changing behavior

**Exit criteria**
- Rust module is behaviorally aligned with the original functions
- Memory management is fully owned/borrowed through safe Rust constructs
- The module remains limited to the migrated file responsibilities

## Notes and Constraints

- The plan assumes a direct port from `src/symbol.c` and does not introduce broader project refactors
- Anonymous C data structures should be named in Rust according to actual use discovered during implementation, not invented beyond current needs
- Standard library collections and ownership semantics are the default replacement for C allocation and pointer choreography
- The migration should prioritize preserving semantics and traversal order over stylistic redesign