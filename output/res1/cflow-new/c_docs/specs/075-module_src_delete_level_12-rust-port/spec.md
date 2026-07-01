# spec.md

## Title

Rust Port Functional Specification: `module_src_delete_level_12`

## Summary

This module defines level-based symbol deletion behavior within `src/symbol.c`. The analyzed functionality is limited to two deletion callbacks:

- `delete_level_autos`
- `delete_level_statics`

Their role is to identify table entries that belong to a specified level and symbol class, and to mark those entries for deletion through the callback result expected by the surrounding table/list traversal logic.

The Rust rewrite must preserve this behavior exactly at the functional level: when invoked during collection or filtering over symbol table entries, it must distinguish automatic-scope entries from static-scope entries, compare entry level against the requested target level, and return a decision consistent with deleting matching entries and retaining non-matching entries.

## Scope

In scope for this specification:

- Functional behavior of level-based deletion predicates for symbol table entries.
- Use of caller-provided context indicating the level to remove.
- Correct distinction between automatic and static entries.
- Compatibility with traversal/collection logic that consumes callback return values.

Out of scope for this specification:

- Redesign of the larger symbol table subsystem.
- New public APIs beyond what is required to preserve the analyzed behavior.
- Concurrency, persistence, serialization, recovery, or FFI behavior.
- Performance guarantees not evidenced by the analyzed module.

## Feature Specification

### Feature: Delete symbol-table entries at a selected level by storage class

The module provides callback-style filtering functions used during symbol table traversal. Each callback examines a symbol table entry together with caller-supplied deletion context and determines whether that entry should be deleted.

The Rust version must implement the same functional behavior for the two supported deletion modes:

1. **Automatic entry deletion by level**
   - Match entries that represent automatic storage for the requested level.
   - Signal deletion only for entries that satisfy both conditions.

2. **Static entry deletion by level**
   - Match entries that represent static storage for the requested level.

Entries that do not match the requested level, do not belong to the relevant storage class, or are otherwise outside the callback’s selection rule must be retained.

### Behavioral boundaries

The available evidence supports these callbacks as narrow predicates over existing symbol table entries. Therefore, the Rust port must preserve these boundaries:

- The callbacks operate on an existing table-entry object.
- The callbacks consume caller-provided context containing at least a target level.
- The callbacks return a traversal-compatible integer decision value.
- The callbacks do not define broader symbol-table mutation semantics on their own; they only determine whether an entry qualifies for deletion.

## User Scenarios & Testing

### Scenario 1: Remove automatic symbols created in a nested scope

A traversal over symbol table entries is performed when leaving a nested scope. The caller supplies the scope level being exited and uses the automatic-deletion callback.

Expected behavior:
- Entries classified as automatic and created at the exited level are selected for deletion.
- Automatic entries from other levels are not selected.
- Static entries at the same level are not selected by this callback.

Test coverage:
- Provide entries spanning multiple levels and storage classes.
- Verify that only automatic entries at the requested level produce the deletion decision.

### Scenario 2: Remove static symbols associated with a selected level

A traversal is performed for cleanup of static-level symbols tied to a specific level. The caller supplies that level and uses the static-deletion callback.

Expected behavior:
- Entries classified as static and belonging to the target level are selected for deletion.
- Static entries at other levels are retained.
- Automatic entries at the same level are retained by this callback.

Test coverage:
- Provide mixed entries and verify only static entries at the target level are selected.

### Scenario 3: Mixed symbol table contents during deletion pass

The symbol table contains entries with varying levels and categories. A deletion pass is run with one of the module callbacks.

Expected behavior:
- Each entry is judged independently from its own attributes and the supplied level context.
- Non-matching entries remain unaffected by the callback decision.
- The callback result is stable and deterministic for the same entry and same level context.

Test coverage:
- Re-run the same callback with identical inputs and verify identical results.
- Verify no cross-entry effects in callback decisions.

### Scenario 4: No entries match the requested level

A deletion pass is invoked for a level that has no entries of the requested storage class.

Expected behavior:
- No entries are selected for deletion.
- All callback results indicate retention for the examined entries.

Test coverage:
- Use entries from different levels or different storage classes only.
- Verify zero deletion selections.

## Requirements

### Functional Requirements

#### FR-1: Automatic-level deletion predicate
The Rust module shall provide behavior equivalent to `delete_level_autos`, evaluating a symbol table entry against caller-supplied deletion context and selecting the entry for deletion only when the entry is an automatic entry at the requested level.

Traceability:
- `src/symbol.c`
- `delete_level_autos` at lines 274-284
- `struct table_entry`

#### FR-2: Static-level deletion predicate
The Rust module shall provide behavior equivalent to `delete_level_statics`, evaluating a symbol table entry against caller-supplied deletion context and selecting the entry for deletion only when the entry is a static entry at the requested level.

Traceability:
- `src/symbol.c`
- `delete_level_statics` at lines 286-296
- `struct table_entry`

#### FR-3: Level-driven selection
Both deletion predicates shall use caller-provided level context to determine whether an entry belongs to the deletion target.

Traceability:
- `delete_level_autos` at lines 274-284
- `delete_level_statics` at lines 286-296

#### FR-4: Storage-class separation
The automatic-deletion predicate shall not select static entries, and the static-deletion predicate shall not select automatic entries.

Traceability:
- Distinct functions `delete_level_autos` and `delete_level_statics`
- `struct table_entry`

#### FR-5: Traversal-compatible decision result
Each predicate shall return an integer-style decision value compatible with surrounding traversal/filtering logic, such that matching entries are signaled for deletion and non-matching entries are signaled for retention.

Traceability:
- Function signatures returning `int`
- `delete_level_autos`
- `delete_level_statics`

### Key Entities

#### `table_entry`
Represents a symbol table record examined by the deletion predicates. The available evidence shows that deletion decisions are based on properties of a table entry, including at minimum:
- a level association
- a classification sufficient to distinguish automatic versus static entries

Relationship:
- Each callback evaluates one `table_entry` at a time.

Traceability:
- Multiple `struct table_entry` references in `src/symbol.c`
- Used by `delete_level_autos` and `delete_level_statics`

#### Deletion context (`call_data`)
Represents caller-supplied context passed into the deletion callbacks. The evidence supports that this context includes the target level used for matching.

Relationship:
- One deletion context is supplied to a traversal pass.
- Each visited `table_entry` is compared against that context.

Traceability:
- `delete_level_autos(void *data, void *call_data)`
- `delete_level_statics(void *data, void *call_data)`

#### Linked-list / traversal infrastructure
The module exists within a symbol-table system backed by linked-list-based structures and callback-driven iteration. The deletion predicates are consumed by that surrounding traversal machinery rather than acting as standalone deletion routines.

Relationship:
- Traversal infrastructure supplies entries and consumes callback results.
- The callbacks participate in selection, not independent table ownership.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`
- callback signatures in `src/symbol.c`

## Success Criteria

### SC-1: Correct automatic-entry matching
Given a set of symbol table entries with mixed levels and storage classes, the Rust implementation selects for deletion exactly those entries that are automatic and whose level equals the requested level.

Traceability:
- `delete_level_autos`

### SC-2: Correct static-entry matching
Given a set of symbol table entries with mixed levels and storage classes, the Rust implementation selects for deletion exactly those entries that are static and whose level equals the requested level.

Traceability:
- `delete_level_statics`

### SC-3: No cross-class deletion
For the same target level, the automatic-deletion behavior never selects entries that only satisfy the static condition, and the static-deletion behavior never selects entries that only satisfy the automatic condition.

Traceability:
- `delete_level_autos`
- `delete_level_statics`

### SC-4: Non-matching levels are retained
For either predicate, entries whose level differs from the requested level are not selected for deletion.

Traceability:
- `delete_level_autos`
- `delete_level_statics`

### SC-5: Deterministic callback outcome
For repeated invocations with the same entry data and the same level context, each predicate returns the same decision result.

Traceability:
- Callback form and pure selection role of `delete_level_autos` and `delete_level_statics`