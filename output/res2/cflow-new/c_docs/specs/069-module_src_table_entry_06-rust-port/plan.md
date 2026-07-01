# Implementation Plan: module_src_table_entry_06

## Summary

This module centers on symbol-table entry management currently implemented in `src/symbol.c`. The Rust port should migrate the existing hashing, comparison, lookup, insertion, unlinking, cleanup, and iteration/deletion logic without widening scope beyond the current file responsibilities.

The Rust implementation should translate the C hash-table and symbol-entry handling into ownership-based Rust data structures using the standard library. The preferred approach is to replace manual allocation/free paths with `HashMap`-backed storage and explicit entry/state types, while preserving current operational behavior for:

- symbol hashing and equality
- lookup and install workflows
- symbol unlink/removal behavior
- cleanup of static/global symbol storage
- iteration-based collection/deletion helpers

Where the original C code relies on pointer-linked structures or callback-style iteration, the Rust port should keep the same execution order and mutation boundaries, but express them through safe containers and borrowing rules. Any C-specific lifetime coupling should be made explicit in Rust through owned `String`, `Vec`, `Option`, and map entry APIs.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::collections::HashMap`, `Vec`, `Option`, iterator APIs)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve expected near-constant-time symbol lookup and install behavior.
  - Avoid unnecessary string cloning during lookup paths where borrowing is sufficient.
  - Replace manual free/unlink operations with ownership-driven drops while keeping deletion cost proportional to the affected entries only.
  - Maintain behavior comparable to the C implementation for table traversal and entry removal.

## Module Mapping

### Source File Mapping

- **C source**: `src/symbol.c`
- **Rust target**: `src/symbol.rs`

If this module is currently exposed through a broader crate root, wire it through the existing `mod`/`pub(crate)` declarations only as needed for parity with current call sites, without introducing extra façade modules.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `hash_symbol_hasher` | internal hash helper or `Hash` impl support | Prefer standard `HashMap` hashing unless exact legacy bucketing behavior is required by surrounding logic. |
| `hash_symbol_compare` | `PartialEq`/`Eq` on key or entry type | Prefer direct key comparison via Rust map keys. |
| `lookup` | `lookup(...)` in `src/symbol.rs` | Return borrowed or mutable access depending on call-site needs. |
| `install` | `install(...)` | Use `HashMap::entry` to preserve lookup-or-insert semantics. |
| `unlink_symbol` | `unlink_symbol(...)` | Remove symbol from owning container; return removed item if needed by current flow. |
| `static_free` | `static_free(...)` or `clear()`-backed helper | Replace manual recursive/free traversal with container clearing and drop. |
| `collect_processor` | internal iterator-processing helper | Keep local to module unless used externally. |
| `delete_parms_itr` | deletion iterator/helper | Implement as focused mutable traversal over stored parameter entries. |

## Data Model

The analysis only identifies anonymous C data structures, so the Rust plan should derive concrete types from actual field usage in `src/symbol.c` rather than inventing broader abstractions.

### Data-Structure Mapping Strategy

| C Shape | Rust Mapping | Migration Notes |
|---|---|---|
| anonymous struct used as symbol entry | `struct SymbolEntry` | Consolidate fields actually used by `lookup`, `install`, `unlink_symbol`, and cleanup paths. |
| anonymous struct used as hash table / bucket state | `struct SymbolTable` | Prefer `HashMap<Key, SymbolEntry>` unless the C code exposes bucket-level behavior that must be preserved. |
| anonymous linked-list node(s) | `Vec<T>` or `Linked ownership via Option<Box<T>>` only if order/link semantics are required | Default to `Vec` for owned collections; use linked representation only when unlink logic depends on node chaining. |
| anonymous parameter/deletion iteration record | focused helper struct or tuple state | Keep internal and minimal. |
| anonymous callback/processor context | internal context struct | Only if required to carry mutable state across traversal. |

### Core Rust Type Decisions

- **Symbol names / textual keys**: use `String` for owned storage and `&str` for lookup parameters.
- **Optional linked references**: use `Option<T>`; if recursive ownership is truly required, use `Option<Box<Node>>`.
- **Table ownership**: centralize ownership in a `SymbolTable` struct rather than scattered globals where feasible; if the C module relies on file-static state, migrate that state into a module-local static owner only if existing architecture requires it.
- **Deletion semantics**: represent removed state by actual container removal instead of detached raw pointers.
- **Error handling**:
  - Use `Option` for “not found” outcomes.
  - Use `Result` only where insertion or state validation can actually fail in Rust-visible ways.
  - Do not model C allocation-failure branches unless the surrounding Rust project already does so.

### Memory Management Notes

- Replace `malloc`/`free` and explicit cleanup chains with Rust ownership and `Drop`.
- Convert any borrowed C string assumptions into explicit owned strings at insertion boundaries.
- Review all unlink/delete helpers for aliasing hazards; Rust mutable-borrow rules may require collecting keys first, then deleting in a second pass.
- If the C code stores interior cross-references between entries, prefer stable keys/indices rather than self-referential borrowing.

## Implementation Phases

## Phase 1: Inventory and Type Extraction

- Read `src/symbol.c` and identify the concrete anonymous structs and typedef-equivalents used by:
  - `lookup`
  - `install`
  - `unlink_symbol`
  - `static_free`
  - `collect_processor`
  - `delete_parms_itr`
- Extract the minimal field set required for symbol identity, stored payload, link/removal behavior, and iteration/deletion.
- Define Rust structs/enums in `src/symbol.rs` that match current storage roles.
- Decide whether the existing C table behavior maps cleanly to:
  - `HashMap<String, SymbolEntry>`, or
  - a `HashMap` plus auxiliary vectors/lists for ordering/deletion semantics.
- Capture any global/static state from the C file and map it to the narrowest Rust module-local ownership model compatible with current call sites.

## Phase 2: Core Table Operations Migration

- Implement the Rust equivalents of:
  - `lookup`
  - `install`
  - `unlink_symbol`
- Replace custom hasher/compare logic with standard Rust key hashing/equality unless exact legacy behavior is required for non-key fields.
- If legacy comparison is more than string equality, encode it in key normalization or explicit entry matching.
- Use `HashMap::get`, `get_mut`, `entry`, and `remove` APIs to preserve current semantics.
- Keep signatures and visibility aligned with actual project usage rather than introducing generic interfaces.
- Validate that insert-vs-update behavior matches the C code, especially around duplicate installs and existing-entry reuse.

## Phase 3: Cleanup and Iteration Helpers

- Implement `static_free` as deterministic container clearing/drop logic.
- Port `collect_processor` as an internal iteration helper preserving current filtering and processing order.
- Port `delete_parms_itr` using safe mutable traversal:
  - either collect target keys/indices first and remove afterward,
  - or use `retain`/drain-style operations if behavior matches the C code.
- Resolve any borrow-checker conflicts by separating read and mutation passes instead of adding unsafe code unless the data layout makes this unavoidable.
- Ensure no manual deallocation logic remains beyond normal Rust ownership boundaries.

## Phase 4: Verification and Integration

- Add focused unit tests in the existing Rust test layout covering:
  - lookup hit/miss
  - install new/existing entry behavior
  - unlink/removal behavior
  - cleanup clearing all stored state
  - deletion/iteration helpers affecting only intended entries
- Add regression-style tests for any edge cases discovered in `src/symbol.c`, especially around duplicate names, null-like optional fields, or delete-while-iterating behavior.
- Integrate `src/symbol.rs` into the crate in place of the C-backed logic for this module branch, keeping module boundaries unchanged.
- Run `cargo test` and fix semantic mismatches before any broader refactoring.